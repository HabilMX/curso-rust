# Lección 2 — Ownership

**Tiempo:** 2 × 45 min.

**Qué construyes:** los programas que chocan a propósito con el compilador

**Qué aprendes:** las tres reglas de la propiedad, mover contra copiar, préstamos `&` y `&mut`, las dos reglas de los préstamos, por qué no hay recolector de basura

## Al terminar vas a poder

- Explicar las tres reglas de ownership y usarlas para anticipar cuándo Rust libera un valor.
- Distinguir una copia implícita de un movimiento, y justificar cuándo usar `clone()`.
- Elegir si una función debe recibir un valor, una referencia `&T` o una referencia mutable `&mut T`.
- Aplicar la regla de “muchas lecturas o una escritura” y corregir los errores `E0382` y `E0502`.
- Explicar por qué Rust no necesita ni un recolector de basura ni llamadas manuales a `free`.
- Escribir una función que reciba `&str` y devuelva una porción prestada del texto.
- Resolver los ejercicios `move_semantics` y `primitive_types` de Rustlings.

## El porqué antes del cómo

La lección 2 es el punto en que Rust deja de parecer simplemente un lenguaje compilado con sintaxis distinta de Go. Las variables, las funciones, los tipos y el flujo de control de la lección anterior son reconocibles. Ownership cambia una pregunta que muchos lenguajes esconden: cuando un programa crea un dato en memoria, ¿quién debe destruirlo y cuándo?

El `revisor` guardará nombres de servicios, URLs, mensajes de falla, listas y reportes. Todos esos datos pueden crecer en ejecución. El nombre de un servicio leído desde YAML no tiene un tamaño conocido cuando compilas; necesita memoria dinámica. La tabla final del reporte también se construye poco a poco. En C o C++, quien escribe el programa tendría que reservar y liberar esa memoria manualmente. Si libera dos veces, el programa puede corromperse. Si no la libera, pierde memoria. Si conserva un puntero después de liberarla, puede leer una zona que ya pertenece a otra cosa.

Go toma otra decisión. El programa puede crear datos y olvidarse de liberar memoria porque el recolector de basura observa qué objetos siguen siendo alcanzables y recupera los demás. Eso hace que escribir programas sea más directo, pero agrega un componente de ejecución que administra memoria, decide cuándo trabajar y consume recursos para encontrar objetos que ya no sirven. En la mayoría de los programas de Go esa decisión es excelente: reduce complejidad y evita errores graves.

Rust busca otra combinación: memoria segura sin recolector de basura y sin liberación manual. Su propuesta es comprobar, antes de generar el ejecutable, quién posee cada valor y quién puede acceder a él. Si el compilador puede demostrar que un valor ya no se usará, inserta la liberación adecuada al salir de su ámbito. Si no puede demostrar que una referencia seguirá siendo válida, rechaza el programa. Si detecta dos accesos incompatibles a un mismo dato, también lo rechaza.

Esto no significa que Rust “adivine” lo que querías hacer. Al contrario: te exige que tu intención sea visible en las firmas y en las asignaciones. Una función que recibe `String` toma el valor; una que recibe `&str` solo lo consulta; una que recibe `&mut String` puede modificarlo durante un préstamo exclusivo. La información que en Go a veces queda en una convención, un comentario o una revisión de código, en Rust forma parte del tipo.

El precio es real. Al inicio vas a escribir código que parece razonable, pero no compila. La reacción normal es intentar agregar `.clone()` hasta que desaparezca el error. A veces una copia es la decisión correcta; muchas veces es una señal de que la función pidió más propiedad de la que necesitaba. Aprender ownership consiste en dejar de tratar esos errores como obstáculos y empezar a leerlos como preguntas de diseño: ¿quién debe conservar este valor?, ¿cuánto tiempo necesita vivir?, ¿quién puede modificarlo?

El capítulo 4 de *The Rust Programming Language* explica ownership, referencias, préstamos y slices. Léelo completo durante esta lección. No intentes memorizar todos los mensajes del compilador. El objetivo es construir un modelo mental sencillo: cada valor tiene una dueña; mover entrega esa responsabilidad; prestar permite usar un valor sin entregar la responsabilidad; y las reglas de préstamos evitan que una lectura vea un dato mientras alguien lo está cambiando.

El proyecto real ya usa esta idea, aunque todavía no hayas escrito todas sus piezas. `Servicio` posee sus campos `String` porque el revisor debe guardar un nombre y una URL más allá de la función que los leyó. En cambio, las funciones que imprimen un reporte reciben referencias a los servicios y a sus estados: solo necesitan consultarlos, no adueñarse de ellos. Más adelante, en las lecciones 3, 4 y 5, estas mismas decisiones aparecerán en structs, colecciones, errores y lifetimes.

## Los conceptos

### Propiedad, ámbito y liberación determinista

Ownership se resume en tres reglas.

1. Cada valor en Rust tiene una dueña.
2. Solo puede haber una dueña de un valor a la vez.
3. Cuando la dueña sale de ámbito, Rust libera el valor.

Un ámbito es la parte del programa donde un nombre existe. Las llaves delimitan ámbitos, igual que en la lección 1. La diferencia ahora es que salir de un ámbito no solo vuelve inaccesible una variable: también determina cuándo se destruye el valor asociado. Para tipos que reservan recursos, Rust llama a `drop` automáticamente. `String`, por ejemplo, libera el bloque de memoria donde guarda sus caracteres.

**Fig. 2.1** | El ámbito de una variable.

```rust
// fig02_01.rs
fn main() {
    {
        let s = String::from("hola");     // s es la dueña
        println!("{s}");
    }                                     // aquí termina el ámbito: se libera. Sin free(), sin GC
}
```

```bash
$ rustc --edition 2024 fig02_01.rs && ./fig02_01
hola
```

`String::from("hola")` crea un `String` que posee memoria dinámica. Mientras `s` está en el ámbito interno, puede usarse para imprimir el texto. Al llegar a la llave de cierre, `s` deja de existir y Rust libera su memoria. No escribiste `free`, no calculaste tamaños y no esperaste a que un recolector decidiera pasar. El compilador inserta el trabajo necesario porque conoce el alcance de `s`.

La palabra “propiedad” no describe la ubicación física de un dato; describe responsabilidad. El valor puede estar en la pila, en el heap o contener referencias a otros valores. Lo importante es que Rust puede identificar una dueña responsable de limpiar el recurso. Muchos tipos simples, como `u64`, `bool` o `char`, caben completamente en la pila y no requieren liberar nada especial. Un `String`, un `Vec<T>` o un `HashMap<K, V>` administran memoria dinámica y sí necesitan un final ordenado.

Esta liberación se llama determinista porque ocurre en un punto que puedes razonar al leer el programa: al final del ámbito, salvo que el valor se haya movido antes. Es importante distinguirla de la administración manual. No eliges cuándo llamar `drop` para cada valor ni debes hacerlo en condiciones normales. Rust conoce el tipo y genera la liberación correcta. Si un tipo contiene otros valores, su destructor libera también lo que corresponda dentro de él.

Esta garantía no significa que Rust prohíba toda fuga de memoria imaginable. Por ejemplo, es posible conservar datos con ciclos de referencias contadas o usar deliberadamente mecanismos que eviten la liberación. La garantía central es otra: el código seguro no puede usar después un valor que Rust ya liberó, ni liberar dos veces la misma memoria. Para un programa como el revisor, eso elimina una clase completa de errores sin agregar un recolector de basura en tiempo de ejecución.

En Go, una variable local también deja de ser útil cuando sale de su bloque, pero la memoria que haya quedado inaccesible se recupera después, cuando el recolector lo determine. En Rust, el fin del ámbito es parte directa del modelo de recursos. Esa diferencia no vuelve automáticamente mejor a uno u otro lenguaje. Go simplifica muchas aplicaciones; Rust permite saber con más precisión cuándo se liberan memoria, archivos, sockets o candados.

El revisor posee el texto que necesita conservar. Un servicio no puede depender de que siga viva una variable temporal del parser de YAML: por eso sus campos son `String`, no referencias a texto temporal.

<!-- verificar:extracto:src/modelo.rs -->
```rust
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}
```

`nombre` y `url` son propiedad de cada `Servicio`. Cuando se destruya el vector de servicios, se destruirán sus elementos; cuando se destruya cada elemento, se destruirán sus `String`; y cada `String` liberará su memoria. No hay una lista manual de recursos por limpiar. La estructura de valores describe también la estructura de responsabilidad.

### Mover, copiar y clonar

La segunda regla dice que un valor solo tiene una dueña a la vez. Por eso una asignación no siempre significa copiar. Con tipos que poseen recursos, Rust suele mover el valor: la nueva variable se convierte en la dueña y el nombre anterior deja de poder usarse.

Esto sorprende si vienes de Go. En Go, asignar un `string` a otra variable copia su encabezado inmutable y las dos variables pueden leerse. Asignar un struct copia sus campos; si contiene un slice o un map, ambas copias pueden seguir apuntando a datos compartidos. En Rust, el compilador exige que esa relación sea explícita porque una copia superficial de un tipo dueño puede dejar dos valores intentando liberar el mismo recurso.

**Fig. 2.3** | Las dos salidas: copiar o prestar.

```rust
// fig02_03.rs
fn main() {
    let a = String::from("hola");
    let b = a.clone();          // copia explícita: pagas la copia y lo dices
    let c = &a;                 // PRESTAR en vez de mover ← esto es lo normal
    println!("{a} {b} {c}");
}
```

```bash
$ rustc --edition 2024 fig02_03.rs && ./fig02_03
hola hola hola
```

`a.clone()` crea un segundo `String`, con su propia memoria y sus propios caracteres. Por eso `a` y `b` pueden vivir de forma independiente. La copia tiene un costo proporcional al tamaño del texto: copiar “hola” es pequeño; copiar una respuesta HTTP grande o una lista de miles de servicios puede no serlo. Rust hace visible ese costo con el método `clone()`.

No debes interpretar esto como una prohibición de clonar. Una copia es correcta cuando el programa de verdad necesita dos valores independientes: guardar un nombre para el reporte y otro para enviarlo a una tarea, conservar una configuración original antes de transformarla o separar datos que vivirán tiempos distintos. El problema aparece cuando `clone()` se usa mecánicamente para silenciar un error sin responder quién necesita poseer el dato.

El tercer nombre, `c`, es una referencia. `&a` no copia los caracteres ni entrega la propiedad. Crea un préstamo de solo lectura. Por eso se pueden imprimir `a`, `b` y `c`: `a` sigue siendo la dueña; `b` es dueña de otra copia; `c` solo apunta temporalmente a `a`.

Los tipos que implementan el trait `Copy` se comportan distinto. Enteros, booleanos, caracteres y tuplas compuestas exclusivamente por valores `Copy` se copian implícitamente porque duplicarlos es barato y no requieren liberar memoria. Si asignas `let b = a` cuando `a` es un `u64`, puedes usar ambos nombres. No es que ownership desaparezca: cada variable recibe su propia copia del valor.

`String` no implementa `Copy` porque copiar implícitamente sus tres datos internos —puntero, longitud y capacidad— produciría dos administradoras para el mismo bloque del heap. Rust podría copiar también los caracteres, pero entonces cada asignación potencialmente ocultaría trabajo costoso. Por eso diferencia movimiento de clonación.

El revisor clona solamente cuando necesita construir una salida que debe poseer texto propio. El reporte JSON no puede conservar referencias a servicios locales dentro de una función que ya terminó. Por eso convierte el nombre prestado del servicio en un `String` independiente.

<!-- verificar:extracto:src/reporte.rs -->
```rust
    let lineas: Vec<EstadoJson> = ordenadas(servicios, estados)
        .into_iter()
        .map(|(s, e)| EstadoJson {
            servicio: s.nombre.clone(),
            codigo: match e {
                Estado::Ok { codigo, .. } | Estado::Lento { codigo, .. } => Some(*codigo),
                _ => None,
            },
            ms: match e {
                Estado::Ok { ms, .. } | Estado::Lento { ms, .. } | Estado::Falla { ms, .. } => *ms,
                Estado::NoIntentado => 0,
            },
            error: match e {
                Estado::Falla { motivo, .. } => Some(motivo.clone()),
                Estado::NoIntentado => Some("sin revisar".to_string()),
                _ => None,
            },
        })
        .collect();
```

Aquí `servicios` y `estados` se prestan a la función de reporte. `s.nombre.clone()` y `motivo.clone()` son decisiones necesarias: `EstadoJson` debe sobrevivir como elemento de `lineas` y luego convertirse a JSON. El código no clona por miedo al compilador; clona porque el resultado tiene dueña propia.

### Referencias inmutables: prestar para leer

Una referencia es una forma de permitir acceso a un valor sin transferir su propiedad. Se escribe `&T`: “una referencia a un `T`”. Si tienes un `String` y una función solo necesita conocer su longitud, pasar `&String` evita crear una copia y evita que la función consuma el texto.

**Fig. 2.4** | Prestar para leer.

```rust
// fig02_04.rs
fn largo(s: &String) -> usize { s.len() }      // presta, no toma posesión

fn main() {
    let s = String::from("hola");
    let n = largo(&s);
    println!("{s} mide {n}");                   // sigue siendo mía ✓
}
```

```bash
$ rustc --edition 2024 fig02_04.rs && ./fig02_04
hola mide 4
```

La función `largo` recibe una referencia. Dentro de ella, `s.len()` consulta la longitud, pero no puede quedarse con el `String` ni modificarlo. Cuando la llamada termina, el préstamo termina y la dueña original sigue siendo la variable `s` de `main`. Eso explica por qué la última línea puede imprimir tanto el texto como su longitud.

El ejemplo conserva `&String` porque muestra directamente el contraste entre un `String` dueño y una referencia a él. En una API general conviene recibir `&str` cuando solo necesitas leer texto. `&str` es una vista de una secuencia UTF-8; acepta tanto un literal como una referencia a `String`. La lección 4 profundizará esa distinción, pero desde ahora puedes usar una regla práctica: guarda texto que posees como `String`; recibe texto de solo lectura como `&str`.

Una referencia no es una copia del valor. Tiene una vida útil limitada por el valor al que apunta. Rust no permite devolver una referencia a una variable local que desaparecerá al salir de una función, ni conservar una referencia cuando su dueña ya se movió. Esta parte del análisis se conoce como comprobación de préstamos o *borrow checking*.

La referencia hace visible el contrato de una función. Una firma que recibe `String` comunica “necesito tomar este texto”. Una que recibe `&str` comunica “solo necesito leerlo”. En Go, pasar un `string` es barato porque su representación se copia; pasar una estructura grande por valor o por puntero requiere leer la documentación y conocer su implementación. Rust vuelve esa diferencia parte de la firma.

Las funciones del reporte reciben slices prestados. No consumen el vector de servicios ni el vector de estados porque `main` todavía los necesita para decidir el código de salida. La firma expresa esta intención sin comentarios adicionales.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub fn tabla(servicios: &[Servicio], estados: &[Estado]) -> String {
    let filas = ordenadas(servicios, estados);
    let ancho = filas
        .iter()
        .map(|(s, _)| s.nombre.chars().count())
        .max()
        .unwrap_or(0)
        .max("SERVICIO".len());
```

`&[Servicio]` significa “slice prestado de servicios”. Un slice presta una parte contigua de una colección y sabe cuántos elementos contiene. La función puede recorrerla, consultar nombres y calcular el ancho de la tabla, pero no puede vaciar el vector, agregar servicios ni quedarse con ellos. La misma decisión vale para `&[Estado]`.

### Referencias mutables: prestar para modificar

Una referencia mutable se escribe `&mut T`. Sirve cuando una función debe modificar un valor cuya propiedad sigue siendo de quien llama. Para crearla, necesitas dos cosas: la dueña debe declararse con `mut`, y el préstamo debe escribirse como `&mut`.

**Fig. 2.6** | Prestar exclusivamente para modificar.

```rust
// fig02_06.rs
fn agregar_puerto(etiqueta: &mut String) {
    etiqueta.push_str(":443");
}

fn main() {
    let mut servicio = String::from("catalogo");
    agregar_puerto(&mut servicio);
    println!("{servicio}");
}
```

```bash
$ rustc --edition 2024 fig02_06.rs && ./fig02_06
catalogo:443
```

`servicio` es mutable porque su contenido cambiará. `agregar_puerto` no recibe el `String` por valor: recibe un préstamo exclusivo y agrega caracteres al mismo texto. Al terminar la llamada, el préstamo termina y `main` vuelve a usar a la dueña para imprimirla.

La exclusividad es la condición importante. Durante un préstamo `&mut`, nadie más puede leer o modificar el mismo dato a través de otra referencia. No es una limitación arbitraria: si una parte del programa cambia una cadena mientras otra asume que la está leyendo establemente, el resultado puede depender del orden de ejecución. En programas concurrentes, esa situación es una carrera de datos.

Las dos reglas de préstamos son:

1. Puedes tener cualquier número de referencias inmutables a un valor.
2. Puedes tener exactamente una referencia mutable a un valor, o referencias inmutables, pero no ambas al mismo tiempo.

La forma breve de recordarlas es: muchas lecturas o una escritura. Una lectura no cambia el dato y puede compartirse. Una escritura necesita exclusividad porque podría cambiar cualquier parte del valor.

Rust también analiza el último uso real de una referencia. No necesitas esperar necesariamente hasta la llave final del bloque para pedir un préstamo mutable. Si una referencia inmutable ya no se volverá a usar, Rust puede considerar que su préstamo terminó. Esto se conoce como préstamos no léxicos. No debes depender de ello para escribir código confuso, pero explica por qué separar una lectura y una modificación en pasos claros suele compilar.

En el revisor, la tabla se construye con una variable mutable. La propiedad de `salida` sigue en `tabla`, pero `push_str` necesita un préstamo mutable temporal para agregar cada línea. Al salir de la función, `tabla` devuelve el `String` completo y la propiedad pasa a quien llamó.

<!-- verificar:extracto:src/reporte.rs -->
```rust
    let mut salida = format!(
        "{:<ancho$}  {:<6}  {:>8}  DETALLE\n",
        "SERVICIO", "ESTADO", "TIEMPO"
    );
    for (s, e) in filas {
        salida.push_str(&format!(
            "{:<ancho$}  {:<6}  {:>8}  {}\n",
            s.nombre,
            etiqueta(e),
            tiempo(e),
            detalle(e)
        ));
    }
    salida
```

`push_str` modifica a `salida`; por eso la variable se declara con `mut`. En cambio, `s` y `e` son referencias de lectura a los datos de cada fila. El compilador permite ambas cosas porque la función modifica el reporte nuevo, no los servicios ni los estados que está consultando.

### Slices, `&str` y referencias que devuelven referencias

Ownership no obliga a copiar cuando quieres obtener una parte de un valor. Una función puede recibir una referencia y devolver otra referencia a una parte de la misma información, siempre que Rust pueda comprobar que la salida no vivirá más que la entrada. Ese patrón aparece con slices de arreglos, slices de vectores y `&str`.

**Fig. 2.7** | Devolver una vista prestada de texto.

```rust
// fig02_07.rs
fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let texto = String::from("revisor listo");
    let palabra = primera_palabra(&texto);
    println!("primera: {palabra}");
}
```

```bash
$ rustc --edition 2024 fig02_07.rs && ./fig02_07
primera: revisor
```

`primera_palabra` no construye un `String` nuevo. Devuelve una vista hacia una parte de `s`. Por eso es eficiente: no copia los caracteres. También tiene una limitación saludable: `palabra` no puede sobrevivir a `texto`, porque apunta dentro de él. Si `texto` se modifica de una manera que cambie su almacenamiento, una referencia vieja podría dejar de ser válida; Rust evita que uses ambas cosas de manera incompatible.

La firma `fn primera_palabra(s: &str) -> &str` usa una regla de inferencia de lifetimes. El compilador entiende que la referencia de salida está vinculada a la referencia de entrada. En la lección 5 verás los casos donde debes escribir una anotación como `'a`; por ahora quédate con la idea importante: la función no posee la palabra devuelta, así que no puede prometer que existirá por más tiempo que el texto prestado.

El revisor usa lifetimes explícitos cuando arma filas que solo prestan datos de dos slices. No duplica cada servicio y cada estado antes de ordenarlos; conserva referencias válidas mientras los vectores originales sigan vivos.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

La anotación `'a` dice que las referencias dentro de `Fila` no pueden vivir más que los slices prestados a `ordenadas`. `filas` es dueño del vector de referencias, pero no de los servicios ni de los estados. Esta distinción es la base de muchos programas Rust eficientes: poseer la colección no implica poseer todos los datos a los que apunta.

## El error que vas a ver

### E0382: usar un valor después de moverlo

El siguiente programa no compila a propósito. La asignación `let b = a` mueve el `String` de `a` a `b`. La última línea intenta pedir prestado `a` para imprimirlo, pero `a` ya no es dueña ni puede prestarse.

**Fig. 2.2** | Mover, no copiar.

```rust
// fig02_02.rs
fn main() {
    let a = String::from("hola");
    let b = a;                  // NO copia: MUEVE. Ahora b es la dueña
    println!("{a}");            // ← error: valor movido
}
```

```bash
$ rustc --edition 2024 fig02_02.rs
error[E0382]: borrow of moved value: `a`
 --> fig02_02.rs:5:16
  |
3 |     let a = String::from("hola");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es la dueña
  |             - value moved here
5 |     println!("{a}");            // ← error: valor movido
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let b = a.clone();                  // NO copia: MUEVE. Ahora b es la dueña
  |              ++++++++

warning: unused variable: `b`
 --> fig02_02.rs:4:9
  |
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es la dueña
  |         ^ help: if this is intentional, prefix it with an underscore: `_b`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
```

`E0382` significa que intentaste usar un valor después de haber transferido su propiedad. El mensaje señala tres lugares: dónde nació `a`, dónde ocurrió el movimiento y dónde intentaste usarlo otra vez. Esa secuencia es más útil que memorizar el código del error: sigue las flechas y pregunta quién es la dueña después de cada línea.

Hay tres arreglos posibles, y no son intercambiables. Si ya no necesitas `a`, imprime `b`. Si necesitas dos valores independientes, usa `a.clone()` y acepta el costo de la copia. Si la segunda parte solo necesita leer el valor, cambia el diseño para prestar `&a` en vez de moverlo. La tercera opción suele ser la mejor cuando escribes funciones auxiliares para el revisor.

El aviso sobre `b` aparece porque el programa no llega a usarlo. No es el error principal; es consecuencia de que el ejemplo usa `a` a propósito para provocar `E0382`. Los programas que sí deben compilar en el curso se verifican con `-D warnings`, así que una variable sin usar también se convierte en un problema que debes corregir.

### E0502: pedir escritura mientras existen lecturas

El siguiente error representa la segunda regla de préstamos. `r1` y `r2` son referencias inmutables vivas porque se usan en el `println!` final. Mientras esas lecturas existan, Rust no puede crear `r3`, una referencia mutable al mismo `String`.

**Fig. 2.5** | Lecturas y escritura a la vez.

```rust
// fig02_05.rs
fn main() {
    let mut s = String::from("hola");
    let r1 = &s;                  // lectura, ok
    let r2 = &s;                  // otra lectura, ok
    let r3 = &mut s;              // ← error: ya hay lecturas vivas
    println!("{r1} {r2} {r3}");
}
```

```bash
$ rustc --edition 2024 fig02_05.rs
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> fig02_05.rs:6:14
  |
4 |     let r1 = &s;                  // lectura, ok
  |              -- immutable borrow occurs here
5 |     let r2 = &s;                  // otra lectura, ok
6 |     let r3 = &mut s;              // ← error: ya hay lecturas vivas
  |              ^^^^^^ mutable borrow occurs here
7 |     println!("{r1} {r2} {r3}");
  |                -- immutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

`E0502` significa que pediste un préstamo mutable mientras existe un préstamo inmutable activo. Rust no supone que “seguro no pasa nada”. Un préstamo mutable podría reemplazar, acortar, vaciar o realojar el contenido de `s`, de modo que las referencias de lectura ya no tendrían una vista coherente.

La corrección no es copiar `s` por reflejo. Primero decide si de verdad necesitas leer y modificar al mismo tiempo. Si no, termina las lecturas antes de pedir la escritura: imprime o calcula con `r1` y `r2`, deja de usarlas y después crea la referencia mutable. Si necesitas conservar información de lectura mientras modificas, guarda una copia pequeña del dato necesario, como una longitud o una bandera, no necesariamente una copia completa de la estructura.

Este error es una versión local de una garantía que será decisiva en la lección 7. En Go, dos goroutines que leen y escriben datos compartidos sin coordinación pueden tener una carrera de datos que solo aparece al ejecutar. Rust establece estas reglas antes de hablar de hilos; cuando llegues a `Arc`, `Mutex` y canales, el mismo modelo seguirá protegiendo los accesos compartidos.

## Lo que se hace mal

### Usar `clone()` para callar cada error

El compilador sugiere `clone()` en varios mensajes porque es una solución mecánica y segura: crea un valor independiente. Sin embargo, la sugerencia no conoce el diseño de tu programa ni el tamaño de tus datos. Si clonas un `String` pequeño una vez, probablemente no importa. Si clonas una lista de servicios en cada función o duplicas cuerpos HTTP grandes en un ciclo, agregas tiempo y memoria sin necesitarlo.

Antes de escribir `.clone()`, pregunta si la función solo necesita leer. Si la respuesta es sí, recibe `&T` o `&str`. Si debe modificar algo pero la dueña debe conservarlo, recibe `&mut T`. Clona cuando necesites dos propietarias reales, como el JSON del reporte y los datos originales que siguen vivos para otra operación.

### Recibir `String` por valor cuando solo vas a leer

Una firma que recibe `String` hace que quien llama entregue la propiedad. Eso puede ser correcto para una función que normaliza, consume o guarda el texto. Es innecesario para una función que solo imprime, mide o busca una palabra. El costo no siempre es una copia: a veces quien llama puede mover el valor. El problema es que la firma reduce las opciones de uso de la persona que llama.

Para funciones de consulta, prefiere `&str` si trabajas con texto y `&T` si trabajas con otro tipo. La API será más flexible: aceptará literales, `String` y porciones de texto sin obligar a crear propietarios nuevos. La lección 4 mostrará por qué `&str` es normalmente una mejor frontera pública que `&String`.

### Pensar que `mut` significa “puedo prestar mutablemente cuando quiera”

`let mut s` permite modificar a `s`, pero no elimina las reglas de préstamos. La mutabilidad pertenece a la dueña; la exclusividad pertenece a cada préstamo. Puedes declarar una cadena mutable y aun así recibir `E0502` si existen referencias de lectura activas. También puedes tener una variable inmutable que contenga una referencia mutable creada en otro contexto; los dos conceptos son distintos.

Usa `mut` solo cuando el nombre debe cambiar o cuando vas a pedir un préstamo mutable. Si una variable nunca cambia, quitar `mut` deja una intención más clara y evita avisos del compilador.

### Pelear contra el préstamo en vez de reducir su alcance

Una referencia vive hasta su último uso, no necesariamente hasta el final visual del bloque. Si el compilador no acepta un préstamo mutable, revisa dónde se usa por última vez la referencia anterior. Muchas correcciones consisten en reorganizar unas líneas: termina de leer, guarda el resultado que necesitas y luego modifica. Separar las fases de lectura y escritura mejora tanto la legibilidad como la compatibilidad con el borrow checker.

No escondas el problema detrás de una referencia larga, un `unsafe` o una estructura global. El revisor todavía es pequeño; si el modelo de propiedad se vuelve difícil de explicar, suele ser señal de que una función tiene demasiadas responsabilidades o de que un dato se está compartiendo más de lo necesario.

### Confundir `String` con `&str`

`String` posee texto y puede crecer; `&str` es una vista de texto que pertenece a otra cosa. Convertir un `&str` a `String` con `to_string()` o `String::from()` es correcto cuando vas a guardarlo. Hacerlo solo porque una función podría recibir una referencia es una copia evitable. Del otro lado, devolver `&str` cuando el texto se construyó dentro de la función no puede funcionar: el texto local desaparece al terminar la función.

La pregunta útil es siempre la misma: ¿quién debe poseer estos caracteres después de esta operación? Si la respuesta es “la estructura que los guarda”, usa `String`. Si la respuesta es “nadie nuevo; solo necesito observarlos ahora”, usa `&str`.

## Ejercicios

### Ejercicio 1 — Sigue a la dueña

Lee las siguientes situaciones y escribe, antes de compilar, cuál nombre puede usarse al final: una asignación de `u64`; una asignación de `String`; y una asignación de `String` seguida de `clone()`. Después crea tres archivos pequeños y comprueba tus respuestas con `rustc --edition 2024`.

Explica en una frase por qué el entero se copia, por qué el `String` se mueve y por qué el `clone()` produce dos dueñas. No uses `Copy` como una palabra mágica: relaciónalo con el costo y con la necesidad de liberar memoria.

### Ejercicio 2 — Una función que toma y otra que presta

Escribe dos funciones sobre un nombre de servicio. La primera debe recibir un `String` por valor y devolver su longitud. La segunda debe recibir `&str` y devolver la misma longitud. En `main`, demuestra que después de llamar a la primera función ya no puedes imprimir el `String`, y que después de llamar a la segunda sí puedes imprimirlo.

Primero deja activa la línea que provoca `E0382` y lee el diagnóstico completo. Luego comenta esa línea para que el programa compile. No arregles la primera función con `clone()`: el objetivo es observar la diferencia entre tomar propiedad y prestar.

### Ejercicio 3 — Actualiza un servicio sin cambiar de dueña

Escribe `fn agregar_puerto(etiqueta: &mut String)` para anexar `:443` a una etiqueta. Declara un `String` mutable en `main`, préstalo a la función y comprueba que `main` puede imprimir el resultado al final.

Después provoca `E0502`: crea una referencia de solo lectura al mismo texto, úsala después de pedir una referencia mutable y observa la línea señalada por el compilador. Reordena el programa para que la lectura termine antes de modificar.

### Ejercicio 4 — La primera palabra prestada

Implementa `fn primera_palabra(s: &str) -> &str`. Debe devolver la primera palabra de una frase o una cadena vacía si solo recibe espacios. Pruébala con un `String` llamado `texto`, imprime el resultado y después imprime también `texto`.

Haz los ejercicios `move_semantics` y `primitive_types` de Rustlings. En particular, no avances por ensayo y error con `clone()`: en cada solución identifica si Rust te pide mover, copiar o prestar.

## Soluciones

### Solución 1

Un `u64` implementa `Copy`, así que después de `let b = a` existen dos valores independientes y ambos nombres son utilizables. Un `String` no implementa `Copy`; la misma asignación mueve la propiedad a `b`, por lo que `a` deja de ser utilizable. Si escribes `let b = a.clone()`, `a` y `b` poseen dos bloques de texto distintos y ambos pueden usarse.

La prueba no consiste en recordar qué tipos implementan `Copy`, sino en hacer una predicción y comprobarla. Cuando tengas dudas sobre un tipo propio, el compilador te dirá si implementa `Copy`. En structs del revisor que contienen `String`, asume inicialmente que el valor se mueve.

### Solución 2

La función que recibe `String` consume el argumento. Su firma debe verse como `fn largo_tomando(s: String) -> usize`; después de la llamada, el `String` original ya no está disponible. La función que recibe `&str` debe verse como `fn largo_prestando(s: &str) -> usize`; llámala con `&nombre` y después imprime `nombre`.

La diferencia no está en el número que devuelven, sino en el contrato de entrada. Para una función que solo calcula la longitud, la segunda firma es la adecuada. La primera existe para que observes explícitamente el movimiento y para los casos reales donde una función sí necesita quedarse con el valor.

### Solución 3

La solución es la de la figura 2.6: la dueña se declara como `let mut servicio`, se llama a la función con `&mut servicio` y se imprime después de que la llamada termina. La función no devuelve el `String` porque nunca lo recibió como propiedad.

Para corregir el conflicto de préstamos, usa por completo la referencia de lectura antes de crear la referencia mutable. El punto importante no es poner ambas referencias en bloques artificiales, sino hacer visible que la fase de lectura terminó antes de la fase de escritura.

### Solución 4

La solución es la de la figura 2.7. `split_whitespace()` ignora espacios iniciales y separa las palabras; `next()` produce un `Option<&str>`; `unwrap_or("")` devuelve una cadena vacía si no había ninguna palabra. El resultado es una referencia tomada de la entrada, no un `String` nuevo.

La prueba correcta imprime primero la palabra y luego el `String` original. Eso demuestra que `primera_palabra` no tomó la propiedad de `texto`. Si intentaras devolver una referencia a un `String` creado dentro de la función, Rust lo rechazaría porque ese `String` se destruiría al terminar la llamada.

## Cómo sé que lo logré

- [ ] `rustc --edition 2024 fig02_01.rs && ./fig02_01` imprime `hola`.
- [ ] `rustc --edition 2024 fig02_03.rs && ./fig02_03` imprime tres veces `hola` y puedo explicar cuál valor se clonó y cuál se prestó.
- [ ] `rustc --edition 2024 fig02_02.rs` falla con `E0382`, y sé explicar en qué línea se movió la propiedad.
- [ ] `rustc --edition 2024 fig02_05.rs` falla con `E0502`, y sé corregirlo terminando primero las lecturas.
- [ ] `rustc --edition 2024 fig02_06.rs && ./fig02_06` imprime `catalogo:443`.
- [ ] `rustc --edition 2024 fig02_07.rs && ./fig02_07` imprime `primera: revisor`.
- [ ] Terminé `move_semantics` y `primitive_types` de Rustlings sin usar `clone()` como solución automática.
- [ ] Puedo explicar en una frase por qué Rust libera memoria al salir de ámbito sin requerir un recolector de basura.

## Para leer más

- [The Rust Programming Language, capítulo 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html), consultado el 2 de octubre de 2026.
- [The Rust Programming Language, referencias y préstamos](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html), consultado el 2 de octubre de 2026.
- [Documentación oficial de `String`](https://doc.rust-lang.org/std/string/struct.String.html), consultado el 2 de octubre de 2026.
- [Rustlings](https://rustlings.rust-lang.org/), ejercicios `move_semantics` y `primitive_types`, consultado el 2 de octubre de 2026.
