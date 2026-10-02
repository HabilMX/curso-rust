# Lección 1 — Fundamentos

**Tiempo:** 2 × 45 min.

**Qué construyes:** las funciones y los tipos base del `revisor`

**Qué aprendes:** variables y `mut`, sombreado, tipos escalares y compuestos, funciones, «todo es una expresión», `if`, `loop`, `while` y `for`

## Al terminar vas a poder

- Declarar variables inmutables, mutables y constantes, y explicar cuándo corresponde cada una.
- Elegir tipos numéricos, booleanos, caracteres, tuplas, arreglos, vectores y cadenas para datos sencillos del `revisor`.
- Escribir funciones con parámetros y valores de retorno sin depender de conversiones implícitas.
- Explicar por qué un bloque, un `if` y un `loop` pueden producir valores.
- Usar `if`, `loop`, `while` y `for` para clasificar y recorrer datos de manera legible.
- Leer y corregir dos variantes comunes del error `E0308`.
- Resolver los ejercicios `variables`, `functions`, `if` y `primitive_types` de Rustlings.

## El porqué antes del cómo

El `revisor` que construirás durante el curso recibe una lista de servicios, los consulta y reporta qué ocurrió. Aunque al final tendrá HTTP, archivos YAML, concurrencia y salida JSON, su núcleo empieza con operaciones mucho más pequeñas: guardar un tiempo de respuesta, compararlo contra un límite, recorrer una lista y decidir qué texto mostrar. Antes de modelar un servicio con un `struct` o una falla con un `enum`, necesitas poder expresar esas operaciones con precisión.

Piensa en una regla inicial del programa: una respuesta de hasta mil milisegundos se considera normal; una más lenta se reporta como lenta. La regla parece sencilla, pero contiene varias decisiones que Rust quiere que declares: el tiempo no puede ser texto, debe ser un número; el umbral debe tener un tipo compatible; la comparación debe producir una condición booleana; y la función debe entregar siempre una clasificación. Rust no deja esas decisiones escondidas en conversiones automáticas o valores ambiguos. El compilador te pide que el programa diga qué representa cada dato.

Esa insistencia puede sentirse pesada si vienes de Go, Python o JavaScript. En Go también existen tipos estáticos y las conversiones entre números son explícitas, pero Rust extiende esa precisión a otras partes de la sintaxis. Una variable es inmutable por omisión. Un bloque puede devolver un valor. Un `if` debe producir valores del mismo tipo en ambas ramas cuando se usa como expresión. Un `for` distingue entre recorrer una colección, prestarla o consumirla. Al principio son más decisiones visibles; después son información que evita que alguien interprete mal tu intención al mantener el programa.

El modelo mental útil no es “Rust pone obstáculos antes de correr”. Es “Rust convierte decisiones de diseño en cosas comprobables”. Si nombras una medida como `u64`, el compilador sabe que no debe ser negativa. Si haces mutable una variable, el lector sabe que cambiará. Si una función devuelve `&'static str`, queda claro que devuelve una de unas etiquetas fijas y no texto recién construido. Si el resultado de un `if` se guarda en una variable, todas sus ramas deben describir el mismo tipo de resultado. La mayor parte de lo que sigue en el curso se apoya en esa misma idea.

Esta lección trabaja los capítulos 2 y 3 de *The Rust Programming Language*. El capítulo 2 presenta `let`, funciones y el uso de `mut` dentro de un programa pequeño; el capítulo 3 organiza los fundamentos: variables, tipos de datos, funciones, comentarios y flujo de control. No intentes memorizar todos los tipos disponibles en una sentada. Lo importante es aprender a leer una firma, elegir una representación razonable y dejar que el compilador te señale las contradicciones.

El programa real ya contiene estos fundamentos. En `programas/revisor/src/modelo.rs` hay límites expresados como constantes; en `src/config.rs` hay un `for` que valida cada servicio; en `src/revisar.rs` hay condiciones que clasifican respuestas; y en `src/reporte.rs` hay variables mutables para construir una salida. Esta lección no modifica ese proyecto: lo usa como mapa de hacia dónde van las piezas pequeñas que practicarás aquí.

## Los conceptos

### Variables, `mut`, constantes y sombreado

Una variable se declara con `let`. Por omisión, el enlace entre el nombre y su valor es inmutable: después de escribir `let x = 5;`, no puedes asignar otro valor a `x`. Esta elección es deliberada. Cuando lees una función larga, cada nombre que no lleve `mut` te da una garantía local: ese nombre seguirá representando el mismo valor durante el resto de su ámbito.

La inmutabilidad no significa que Rust prohíba cambiar datos. Significa que debes declararlo. Si una variable representa un contador, una salida que construyes poco a poco o un índice que decrece, usa `let mut`. La palabra `mut` va junto al nombre porque describe al enlace, no a toda la función. Evita poner `mut` por costumbre: una variable mutable que nunca cambia provoca un aviso, y compilar los ejemplos con `-D warnings` convierte ese aviso en un error. Es una señal pequeña, pero útil: el código dice que algo variará y en realidad no ocurre.

Las constantes se escriben con `const`, llevan tipo explícito y se evalúan antes de ejecutar el programa. Úsalas para reglas cuyo nombre debe aparecer en todo el código: un límite de tiempo, una capacidad o un máximo de intentos. Una constante no es una variable inmutable con otro nombre. No ocupa un lugar de memoria único que puedas prestar o modificar; se sustituye donde se usa. En esta etapa basta con recordar la regla práctica: `let` para valores locales y `const` para una regla estable y nombrada.

**Fig. 1.1** | Variables, mutabilidad y constantes.

```rust
// fig01_01.rs
fn main() {
    let x: i32 = 5;             // tipo explícito (casi nunca hace falta: lo infiere)
    let mut y = 10;             // mutable
    const MAX: u32 = 100_000;   // constante, siempre con tipo

    y += x;
    println!("x = {x}, y = {y}, MAX = {MAX}");
}
```

```bash
$ rustc --edition 2024 fig01_01.rs && ./fig01_01
x = 5, y = 15, MAX = 100000
```

El tipo de `x` está escrito como `i32`, pero Rust podría inferirlo aquí porque `y += x` y el literal `10` dan suficiente contexto. Anotar tipos ayuda cuando una firma forma parte de una API, cuando el compilador no puede inferirlos o cuando quieres comunicar una restricción importante. No los anotes mecánicamente en cada `let`: la inferencia bien usada reduce ruido sin perder seguridad.

El sombreado es distinto de la mutabilidad. Con sombreado declaras una nueva variable con el mismo nombre; la anterior deja de ser accesible desde ese punto. Es útil cuando una idea pasa por etapas y quieres conservar un nombre honesto. Por ejemplo, un texto con espacios y la cantidad de espacios son dos valores distintos, pero ambos pueden llamarse `espacios` porque la primera versión ya no hace falta. A diferencia de `mut`, el sombreado permite que cambie el tipo.

**Fig. 1.7** | Sombreado, tuplas y arreglos.

```rust
// fig01_07.rs
fn main() {
    let espacios = "   ";
    let espacios = espacios.len();

    let medicion: (u16, u64, bool) = (200, 750, true);
    let (codigo, ms, saludable) = medicion;
    let nombres = ["catalogo", "pagos"];

    println!("espacios = {espacios}");
    println!("codigo = {codigo}, ms = {ms}, saludable = {saludable}");
    println!("primer servicio = {}", nombres[0]);
}
```

```bash
$ rustc --edition 2024 fig01_07.rs && ./fig01_07
espacios = 3
codigo = 200, ms = 750, saludable = true
primer servicio = catalogo
```

Aquí el primer `espacios` es `&str`, una vista de texto; el segundo es `usize`, una cantidad. No es que una variable haya mutado de texto a número: son dos enlaces distintos, con ámbitos superpuestos. Esta diferencia importa más adelante con ownership. `let mut nombre` conserva el mismo valor y permite modificarlo; `let nombre = ...` vuelve a enlazar el nombre y puede transformar el valor sin conservar la versión anterior.

En el `revisor`, una variable mutable aparece al construir la tabla que se imprimirá. El nombre `salida` no representa una regla fija: es un acumulador al que se agregan líneas, así que `mut` comunica exactamente la intención.

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
}
```

Todavía no necesitas entender referencias, iteradores ni `format!` para reconocer la decisión fundamental: `filas` y `ancho` no cambian; `salida` sí. En la lección 2 estudiarás por qué `&[Servicio]` y `&[Estado]` son préstamos, y en la lección 4 verás cómo `String` permite construir texto dinámico. Por ahora, identifica el patrón: declara inmutable hasta que una modificación sea parte real del trabajo.

Las reglas de clasificación del `revisor` también son constantes. El valor no está repetido como un número anónimo en cada comparación; tiene nombre, tipo y comentario. Cuando cambie la política de “lento”, habrá un lugar obvio que revisar.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;
```

`TIMEOUT_POR_OMISION_MS` es privado al módulo porque solo se usa para crear servicios por omisión. `UMBRAL_LENTO_MS` lleva `pub` porque otro módulo, `revisar.rs`, necesita consultarlo. La visibilidad de módulos se estudia formalmente en la lección 6; lo importante hoy es que ambos valores tienen tipos explícitos y nombres que expresan unidades. Un `1000` sin nombre deja preguntas: ¿mil segundos, mil bytes, mil milisegundos? `UMBRAL_LENTO_MS` las responde.

### Tipos escalares y compuestos

Rust es un lenguaje de tipos estáticos: antes de ejecutar, el compilador conoce el tipo de cada valor. A veces lo infiere y a veces debes anotarlo, pero nunca trata un número como texto o mezcla dos tamaños enteros porque “más o menos parecen compatibles”. Ese rigor permite que muchas equivocaciones se detecten antes de producir un binario.

Los tipos escalares guardan un solo valor. Los enteros con signo son `i8`, `i16`, `i32`, `i64`, `i128` e `isize`; los sin signo son `u8`, `u16`, `u32`, `u64`, `u128` y `usize`. El número dice cuántos bits ocupa el valor. `isize` y `usize` cambian según la arquitectura y se usan principalmente para tamaños, longitudes e índices. Para cantidades de milisegundos del `revisor`, `u64` es una decisión explícita: no hay tiempos negativos y el rango es amplio. Para un código HTTP, `u16` es suficiente. Elegir un tipo no consiste en buscar el número más pequeño posible; consiste en expresar el dominio del dato de manera sensata.

`i32` es el tipo entero predeterminado cuando el compilador no recibe más contexto. Es una buena elección general para cálculos enteros locales. No asumas que todos los enteros son `i32`: un `usize` que viene de `len()` no se puede sumar directamente a un `u64`, y un `u16` de un código HTTP no se vuelve `i32` por estar en la misma operación. La ventaja es que ves el cruce de dominios en el punto exacto donde sucede.

Rust no hace conversiones numéricas implícitas. No es una rareza aislada: evita que una asignación aparentemente inocente cambie tamaño, signo o rango sin que quien escribió el código lo haya considerado. En Go las conversiones entre tipos numéricos también se piden explícitamente; Rust conserva esa disciplina y la vuelve especialmente importante porque sus tipos enteros se usan con frecuencia para representar capacidades, longitudes y datos de red.

**Fig. 1.2** | No hay conversión implícita, ni entre números.

```rust
// fig01_02.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a;             // ← no compila
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_02.rs
error[E0308]: mismatched types
 --> fig01_02.rs:4:18
  |
4 |     let b: i64 = a;             // ← no compila
  |            ---   ^ expected `i64`, found `i32`
  |            |
  |            expected due to this
  |
help: you can convert an `i32` to an `i64`
  |
4 |     let b: i64 = a.into();             // ← no compila
  |                   +++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Para una conversión simple y conocida puedes usar `as`. La conversión de `i32` a `i64` es segura para este caso porque todo `i32` cabe en un `i64`. Sin embargo, `as` también permite conversiones que pueden truncar, reinterpretar signo o perder precisión. No lo uses como una forma de “hacer que el compilador se calle”. Cuando una conversión pueda fallar o perder información, más adelante conocerás `TryFrom`, `TryInto` y `Result`.

**Fig. 1.3** | La conversión se pide con `as`.

```rust
// fig01_03.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a as i64;      // así
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_03.rs && ./fig01_03
5
```

Además de enteros, los escalares incluyen `f32` y `f64` para números de punto flotante, `bool` para `true` o `false`, y `char` para un carácter Unicode. Para el `revisor`, evita usar punto flotante si un entero expresa mejor la unidad. Guardar `750` milisegundos como `u64` es más claro que guardar `0.75` segundos como `f64`, y evita preguntas sobre redondeo cuando muestres, compares o serialices el valor.

Los tipos compuestos agrupan varios valores. Una tupla puede guardar elementos de tipos distintos y tiene tamaño fijo. En la figura 1.7, `(u16, u64, bool)` representa tres resultados que pertenecen a una misma medición: código, duración y estado de salud. La desestructuración `let (codigo, ms, saludable) = medicion;` extrae esos valores con nombres útiles. Las tuplas son adecuadas para resultados pequeños y locales; cuando el significado de los campos sea central al programa, como lo será un servicio, una estructura con campos con nombre será mejor. Eso llega en la lección 3.

Un arreglo como `["catalogo", "pagos"]` contiene valores del mismo tipo y tiene longitud fija conocida en compilación. Un vector, `Vec<T>`, también contiene valores del mismo tipo, pero puede crecer o encogerse en ejecución. La figura 1.6 usa `vec!` porque la lista de servicios es una colección que conceptualmente puede cambiar de tamaño. En el proyecto real, la configuración se carga desde YAML y produce un `Vec<Servicio>` por la misma razón.

Las cadenas también requieren precisión. Un literal como `"catalogo"` suele ser `&str`, una vista prestada de texto ya existente. Un `String` es texto que posee memoria y puede crecer. En esta lección verás `&str` como valor de salida de etiquetas fijas; en la lección 2 estudiarás por qué no todos los textos se pueden copiar y por qué se distinguen ambos tipos. De momento, conserva esta regla: un texto fijo escrito en el código suele ser `&str`; texto leído, construido o almacenado suele acabar como `String`.

El `revisor` hace explícito su vocabulario numérico. El tiempo límite se guarda como `u64` y el código HTTP como `Option<u16>`. No necesitas dominar `Option` todavía; la lección 3 explicará por qué sustituye a `nil`. Hoy basta observar que el tipo describe una restricción de la realidad: puede no haber código HTTP si no llegó respuesta.

<!-- verificar:extracto:src/modelo.rs -->
```rust
#[derive(Serialize)]
pub struct EstadoJson {
    pub servicio: String,
    pub codigo: Option<u16>,
    pub ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

El tipo no es documentación decorativa. `ms: u64` impide asignarle una cadena; `codigo: Option<u16>` impide tratar la ausencia de respuesta como si fuera automáticamente un `200`; `servicio: String` indica que el nombre es texto que el programa posee. Rust usará esa información durante toda la compilación.

### Funciones, parámetros y valores de retorno

Una función se declara con `fn`, tiene un nombre, parámetros entre paréntesis y un cuerpo entre llaves. Los parámetros siempre llevan tipo: `fn doble(x: i32)` dice tanto el nombre del dato como qué puede recibir la función. Si devuelve algo distinto de `()`, se indica después de una flecha: `-> i32`. Esta firma es un contrato corto y verificable. Quien llame a la función sabe qué debe entregar y qué obtendrá; el compilador comprueba ambos extremos.

A diferencia de Go, Rust escribe el tipo después del nombre del parámetro, no antes. En Go escribirías `func doble(x int) int`; en Rust, `fn doble(x: i32) -> i32`. La diferencia visual deja de importar tras unas cuantas funciones. Lo importante es que en ambos lenguajes la firma forma parte del diseño: no es un comentario ni una convención informal.

Una función pequeña no debe asumir trabajo que no le corresponde. `doble` recibe un número y devuelve otro; no imprime, no lee archivos y no modifica estado externo. Esa separación parece básica, pero prepara el terreno para el `revisor`: una función que clasifica milisegundos puede probarse con tres números sin iniciar un cliente HTTP ni abrir una configuración. Cuando el programa crezca, dividir la lógica en funciones con entradas y salidas claras será una forma de mantenerlo entendible.

**Fig. 1.4** | Todo es una expresión.

```rust
// fig01_04.rs
fn main() {
    let x = 7;
    let n = if x > 5 { "grande" } else { "chico" };      // el if DEVUELVE valor

    let cuadrado = {
        let t = x * x;
        t                          // 🔑 sin punto y coma = es el valor del bloque
    };

    println!("{n} {cuadrado} {}", doble(x));
}

fn doble(x: i32) -> i32 {
    x * 2                      // sin `return` y sin `;`
}
```

```bash
$ rustc --edition 2024 fig01_04.rs && ./fig01_04
grande 49 14
```

`main` también es una función. En un programa ejecutable empieza sin parámetros y no necesita declarar retorno si solo termina. En cambio, `doble` promete un `i32`, así que el último valor de su cuerpo debe ser compatible con `i32`. Puedes usar `return x * 2;`, pero no es la forma habitual para el último valor de una función. Rust favorece la expresión final porque deja visible qué resultado produce el cuerpo.

Los parámetros se pasan de distintas maneras según el tipo y la intención. Los tipos escalares como `i32`, `u64` y `bool` se copian de manera económica; recibir `ms: u64` no impide al llamador seguir usando su medida. Con `String`, vectores y estructuras más complejas aparecerán las reglas de movimiento y préstamo de la lección 2. No las adelantes resolviendo todo con copias. Por ahora usa parámetros escalares para practicar firmas limpias y reconoce que `&str` de una etiqueta fija tiene una vida diferente de un `String` que se construye.

El proyecto real tiene una función pequeña que convierte una configuración textual en datos del programa. Aunque usa bibliotecas que estudiarás después, su firma muestra el patrón esencial: recibe una entrada, devuelve un resultado y su cuerpo termina con una expresión `Ok(...)`.

<!-- verificar:extracto:src/config.rs -->
```rust
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(serde_yaml::from_str(&txt)?)
}
```

Aún no necesitas desmenuzar `Result`, `?` ni `serde_yaml`; llegarán en la lección 4. Lo que ya puedes leer es la forma: `ruta` entra como una vista de texto, la función promete devolver una lista de servicios o un error, `txt` es un valor local inmutable y `Ok(...)` es el resultado final. Las firmas te dejan entender la frontera de una función incluso antes de conocer todos sus detalles internos.

### Expresiones, sentencias y el punto y coma

En Rust, muchas construcciones producen un valor. Una operación aritmética como `x * 2` produce un número; un bloque entre llaves puede producir el último valor que contiene; un `if` puede producir uno de dos valores; y un `loop` puede terminar con un valor enviado por `break`. A estas construcciones se les llama expresiones.

Una sentencia realiza una acción pero no produce un valor útil. Una declaración `let x = 7;` es una sentencia. También lo es una expresión a la que agregas punto y coma. El valor de una sentencia es `()`, llamado tipo unidad. Puedes pensar en `()` como “no hay resultado que entregar”. No es un error ni un valor nulo: es un tipo real que aparece cuando una operación se usa solo por su efecto.

El punto y coma determina esa diferencia en lugares importantes. En la figura 1.4, el bloque asignado a `cuadrado` termina con `t` sin punto y coma, por lo que el bloque produce el valor de `t`. La función `doble` termina con `x * 2` sin punto y coma, por lo que devuelve ese `i32`. Si agregas `;`, la operación se ejecuta y su resultado se descarta. Entonces el cuerpo de la función produce `()`, pero la firma exige `i32`.

**Fig. 1.5** | El punto y coma de más.

```rust
// fig01_05.rs
fn doble(x: i32) -> i32 {
    x * 2;                     // ← el punto y coma de más
}

fn main() {
    println!("{}", doble(4));
}
```

```bash
$ rustc --edition 2024 fig01_05.rs
error[E0308]: mismatched types
 --> fig01_05.rs:2:21
  |
2 | fn doble(x: i32) -> i32 {
  |    -----            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
3 |     x * 2;                     // ← el punto y coma de más
  |          - help: remove this semicolon to return this value

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Este error es desconcertante una vez y muy útil después. El compilador no está diciendo que la multiplicación sea inválida; dice que la función prometió `i32` y acabó produciendo `()`. Lee las dos partes del mensaje: `expected i32, found ()` identifica la contradicción, y la ayuda propone quitar el punto y coma. No añadas un `return` sin entender por qué; en este caso el problema es que descartaste el valor correcto.

El estilo de expresión hace que transformaciones pequeñas queden compactas y claras. Puedes calcular un valor intermedio en un bloque, conservar las variables locales dentro de ese bloque y entregar solo el resultado. Esto reduce ámbitos innecesarios y evita nombres temporales que siguen vivos cuando ya no significan nada. No conviertas cada línea en una expresión complicada: la legibilidad sigue siendo el criterio. Un bloque con dos o tres pasos bien nombrados suele ser más claro que una línea ingeniosa.

En `revisor.rs`, la clasificación de una respuesta usa condiciones dentro de un `match`; el resultado de cada rama es un `Estado`. Aunque `match` se estudia a fondo en la lección 3, el patrón ya es familiar: cada camino produce el valor que la función prometió.

<!-- verificar:extracto:src/revisar.rs -->
```rust
    match respuesta {
        Ok(r) if r.status().is_success() && ms > UMBRAL_LENTO_MS => Estado::Lento {
            codigo: r.status().as_u16(),
            ms,
        },
        Ok(r) if r.status().is_success() => Estado::Ok {
            codigo: r.status().as_u16(),
            ms,
        },
        Ok(r) => Estado::Falla {
            motivo: format!("codigo {}", r.status().as_u16()),
            ms,
        },
        Err(e) if e.is_timeout() => Estado::Falla {
            motivo: "se acabo el tiempo de espera".to_string(),
            ms,
        },
        Err(_) => Estado::Falla {
            motivo: "no responde".to_string(),
            ms,
        },
    }
```

La parte que corresponde a esta lección son las condiciones `if` y la idea de que una construcción de control termina en un valor. La parte nueva son `match`, `Result` y las variantes de `Estado`. No necesitas copiarlos aún; solo reconoce que la sintaxis que practicas con números terminará tomando decisiones reales sobre servicios.

### `if`, `loop`, `while` y `for`

`if` evalúa una condición que debe ser `bool`. Rust no considera que `0`, una cadena vacía o una referencia nula sean falsos de forma automática. Escribe una comparación o usa una variable booleana. Esta decisión evita condiciones accidentales y hace evidente qué propiedad estás preguntando: `ms > UMBRAL_LENTO_MS` comunica una regla; `if ms` no tendría significado.

Cuando `if` se usa para elegir un valor, sus ramas deben devolver el mismo tipo. No puedes devolver `"rápido"` en una rama y `1000` en otra, porque la variable que recibe el resultado debe tener una representación consistente. Esta restricción es exactamente la clase de decisión que se vuelve útil en un reporte: una clasificación siempre será texto, un código de salida siempre será un entero adecuado y un estado siempre será una variante del mismo tipo.

`loop` inicia un ciclo infinito. Parece una herramienta extrema, pero es adecuada cuando no conoces de antemano el número de iteraciones y la salida natural es `break`. A diferencia de otros lenguajes, `break valor` puede dar el resultado de un `loop`. Esto sirve cuando el ciclo busca o calcula algo; el valor encontrado sale directamente como resultado de la expresión.

`while condicion` repite mientras la condición sea verdadera. Úsalo cuando el avance depende de un estado que controlas: decrementar una cuenta, leer hasta una condición o reintentar bajo una regla explícita. Asegúrate de que el cuerpo puede cambiar el estado que hace falsa la condición. Un `while` cuyo contador nunca se actualiza es un ciclo infinito disfrazado.

`for` es la opción normal para recorrer una colección o un rango. Rust no tiene el estilo tradicional `for inicialización; condición; actualización` de C, Java o Go. En vez de ello, recorre algo que implementa `IntoIterator`: un rango como `0..10`, una lista, un arreglo o un iterador. Esta forma elimina gran parte del código de índices y reduce errores de límites.

**Fig. 1.6** | Los bucles.

```rust
// fig01_06.rs
fn main() {
    let mut x = 3;
    let servicios = vec!["catalogo", "pagos", "reportes"];

    loop { break; }                          // infinito, con break
    while x > 0 { x -= 1; }
    for i in 0..10 { print!("{i} "); }       // rango: 0 a 9
    println!();
    for i in 0..=10 { print!("{i} "); }      // inclusivo: 0 a 10
    println!();
    for s in &servicios { println!("{s}"); } // sobre una referencia, para no consumir la lista

    let r = loop { break 42; };              // 🔑 loop devuelve valor con break
    println!("x = {x}, r = {r}");
}
```

```bash
$ rustc --edition 2024 fig01_06.rs && ./fig01_06
0 1 2 3 4 5 6 7 8 9 
0 1 2 3 4 5 6 7 8 9 10 
catalogo
pagos
reportes
x = 0, r = 42
```

Los rangos son una fuente común de errores de límite. `0..10` incluye `0` y excluye `10`, por lo que tiene diez valores: del cero al nueve. `0..=10` incluye ambos extremos y tiene once valores. Para recorrer posiciones de un arreglo de longitud diez, casi siempre quieres `0..10` o, mejor todavía, recorrer directamente los elementos. Usa el rango inclusivo solo cuando el límite final sea parte de la regla y deba aparecer.

La línea `for s in &servicios` lleva una referencia a la lista. Eso permite leer cada elemento sin entregar la propiedad de `servicios`. La diferencia completa entre `servicios`, `&servicios` y `&mut servicios` es el tema de la lección 2, pero puedes adoptar desde hoy una regla provisional: si solo quieres mirar una colección y conservarla, recórrela por referencia. El compilador evitará usos inseguros cuando conozcas las reglas de préstamo.

El `revisor` valida una lista con un `for`. La función no necesita saber cuántos servicios llegaron: los toma uno por uno. `enumerate()` agrega el índice para poder comparar el servicio actual con los anteriores. Aunque la expresión completa parece avanzada, su flujo es el mismo de la figura 1.6: recorrer, comprobar una condición y terminar con un resultado.

<!-- verificar:extracto:src/config.rs -->
```rust
pub fn validar(servicios: &[Servicio]) -> Result<()> {
    anyhow::ensure!(
        !servicios.is_empty(),
        "el archivo no declara ningún servicio"
    );
    for (i, s) in servicios.iter().enumerate() {
        anyhow::ensure!(
            !servicios[..i].iter().any(|antes| antes.nombre == s.nombre),
            "el nombre «{}» está repetido",
            s.nombre
        );
        anyhow::ensure!(
            s.url.starts_with("http://") || s.url.starts_with("https://"),
            "la URL «{}» de «{}» debe empezar con http:// o https://",
            s.url,
            s.nombre
        );
        anyhow::ensure!(
            s.timeout_ms > 0,
            "el tiempo límite de «{}» debe ser mayor que cero",
            s.nombre
        );
    }
    Ok(())
}
```

Aquí `s.timeout_ms > 0` es una condición booleana como las que ya usaste. La diferencia es que, en vez de imprimir una etiqueta, `anyhow::ensure!` detiene la validación con un error si la condición es falsa. La lección 4 explicará `Result` y este tipo de manejo de errores. La lección 2 explicará las referencias de `&[Servicio]`. Ya puedes leer la intención sin conocer cada detalle: todos los servicios deben tener una URL con esquema, un nombre no repetido y un límite mayor que cero.

## El error que vas a ver

### `E0308`: tipos que no coinciden

`E0308` significa que Rust esperaba un tipo en un punto del programa y encontró otro. No es un mensaje vago: léelo como una frase con dos partes. Primero identifica el sitio donde se fijó la expectativa; luego identifica el valor que contradice esa expectativa. En la figura 1.2, la anotación `let b: i64` fija que `b` será `i64`; la variable `a` es `i32`; por eso la asignación falla.

La corrección no siempre será `as`. Para convertir de `i32` a `i64`, el ensanchamiento es seguro y `as i64` comunica la intención. Para convertir de un número grande a uno pequeño, o de un texto a un entero, debes decidir qué hacer cuando el valor no cabe o no tiene formato válido. Esas conversiones se tratarán con resultados que pueden fallar. La buena práctica es resolver la discrepancia en el límite entre dominios, no convertir valores repetidamente dentro de cada función.

La figura 1.5 produce el mismo código `E0308`, pero por una causa distinta: la función declara `-> i32` y su último elemento es una sentencia cuyo valor es `()`. Esta diferencia ilustra por qué no debes resolver errores solo por el número. El código agrupa una familia de diagnósticos; las líneas señaladas y las palabras `expected` y `found` cuentan la historia concreta.

Cuando veas `expected i32, found ()`, haz estas preguntas: ¿la función prometió un retorno? ¿el último valor tiene punto y coma? ¿una rama de `if` no devuelve lo mismo que la otra? ¿puse `println!` como último elemento cuando necesitaba producir un valor? Con ese orden normalmente encuentras el problema sin buscar respuestas al azar.

### Leer una sugerencia sin obedecerla a ciegas

Rust suele ofrecer una sección `help:`. Es una propuesta contextual, no una orden. En la figura 1.2 sugiere `a.into()`, que también puede convertir el valor porque existe una conversión conocida entre ambos tipos. La figura 1.3 conserva `as i64` porque es la forma que se quiere enseñar para una conversión numérica explícita y simple. En otros casos, la sugerencia puede ser `clone()`, agregar una referencia o cambiar una firma. Antes de aceptarla, pregúntate qué costo, propiedad o comportamiento está introduciendo.

El mensaje también termina con `rustc --explain E0308`. Ese comando abre una explicación general del código de error instalado con tu compilador. Úsalo cuando el diagnóstico local no baste, pero empieza por el archivo, la línea y las columnas que el compilador ya te mostró. Casi siempre contienen más información específica sobre tu programa que una búsqueda general.

## Lo que se hace mal

### Declarar todo como `mut`

Declarar cada variable con `mut` para “tener libertad” borra información. Si un nombre no cambia, el lector no debe tener que rastrear toda la función para descubrirlo. Además, el compilador avisa cuando `mut` no hace falta. Declara mutable únicamente lo que el algoritmo modifica, como `x` en un conteo regresivo o `salida` al construir un reporte.

### Usar `as` para apagar errores de tipos

Una conversión con `as` puede ser correcta, pero no es una cura universal. Convertir un `u64` grande a `u16` puede perder datos; convertir un entero con signo a uno sin signo puede producir un valor sorprendente. Define qué representa cada número y convierte una vez, en el borde donde cambias de dominio. Si la conversión puede fallar, el programa debe expresarlo en lugar de esconderlo.

### Usar números sin unidades ni nombre

Un `if ms > 1000` funciona, pero obliga a recordar qué representa `1000`. ¿Son milisegundos, segundos o bytes? Usa una constante como `UMBRAL_LENTO_MS` cuando el valor sea una regla del negocio. Para valores locales obvios, un literal puede estar bien; para una política que se repetirá o cambiará, un nombre evita errores y mejora la lectura.

### Agregar punto y coma a la última expresión por reflejo

En muchos lenguajes cada línea termina con punto y coma o la convención invita a usarlo. En Rust, el último punto y coma de una función o bloque cambia su valor a `()`. No memorices una excepción; reconoce la regla: una expresión final sin punto y coma puede ser el resultado. Si un bloque existe para calcular algo, revisa qué deja como última expresión.

### Escribir `for i in 0..lista.len()` cuando solo necesitas elementos

Recorrer índices funciona, pero añade una forma innecesaria de equivocarse. Si solo necesitas cada servicio, escribe `for servicio in &servicios`. Usa `enumerate()` cuando el índice forme parte real de la lógica, como en la validación del `revisor`. Usa índices directos cuando debas acceder a posiciones concretas y puedas justificar los límites.

### Usar `loop` cuando el número de pasos ya es conocido

Un `loop` con varias condiciones de salida puede ser correcto, pero si tienes una colección o un rango conocido, `for` expresa mejor la intención. Si depende de una condición cambiante, `while` suele mostrar el criterio de terminación con más claridad. Reserva `loop` para procesos que realmente esperan una salida mediante `break`, como un lector de eventos o una búsqueda que termina al encontrar el dato.

## Ejercicios

### Ejercicio 1 — Clasifica una respuesta

Escribe `fn clasificar(ms: u64) -> &'static str`. Debe devolver `"rápido"` si el tiempo es menor o igual a `1000`, `"lento"` si es mayor que `1000` y menor o igual a `5000`, y `"timeout"` si es mayor. Usa un `if` como expresión: no uses `return`. Desde `main`, imprime la clasificación de `700`, `1500` y `6000`, una por línea.

Antes de ver la solución, verifica que las tres ramas devuelven el mismo tipo. La firma no necesita crear un `String`: las tres etiquetas son literales fijos y por eso pueden ser `&'static str`.

### Ejercicio 2 — Suma sin consumir la lista

Escribe `fn sumar(valores: &[i32]) -> i32` que use `for` para sumar un slice. Desde `main`, crea `let valores = vec![3, 5, 8];`, imprime el resultado y después imprime la longitud de `valores`. La segunda impresión debe compilar: demuestra que el recorrido no consumió el vector.

Haz que solo el acumulador sea mutable. No conviertas el vector en mutable: no estás agregando, quitando ni modificando sus elementos.

### Ejercicio 3 — Un reporte mínimo del revisor

Declara `const UMBRAL_LENTO_MS: u64 = 1000;` y escribe `fn etiqueta(ms: u64) -> &'static str` que devuelva `"OK"` hasta el umbral y `"LENTO"` por encima. En `main`, usa un arreglo con `[120_u64, 1000, 1500]` y un `for` para imprimir exactamente estas líneas:

```text
120ms: OK
1000ms: OK
1500ms: LENTO
```

Después cambia el tipo del arreglo a `i32` sin cambiar la firma de `etiqueta`. Lee `E0308`, corrígelo de forma explícita y explica con tus palabras por qué Rust no hizo la conversión por ti.

## Soluciones

### Solución 1

<!-- verificar:fragmento -->
```rust
fn clasificar(ms: u64) -> &'static str {
    if ms <= 1000 {
        "rápido"
    } else if ms <= 5000 {
        "lento"
    } else {
        "timeout"
    }
}
```

El `if` completo es la expresión final de la función. Cada rama devuelve un literal de tipo `&'static str`, así que la firma y el resultado coinciden. El orden importa: la segunda condición solo se evalúa si la primera fue falsa, por lo que no hace falta repetir `ms > 1000`.

### Solución 2

<!-- verificar:fragmento -->
```rust
fn sumar(valores: &[i32]) -> i32 {
    let mut total = 0;

    for valor in valores {
        total += valor;
    }

    total
}
```

`valores` recibe una referencia a un slice, por lo que la función observa los números sin quedarse con el vector del llamador. Dentro del `for`, `valor` es una referencia a cada `i32`; la suma funciona porque los enteros escalares se copian. La última expresión, `total`, entrega el resultado sin punto y coma.

### Solución 3

<!-- verificar:fragmento -->
```rust
const UMBRAL_LENTO_MS: u64 = 1000;

fn etiqueta(ms: u64) -> &'static str {
    if ms <= UMBRAL_LENTO_MS {
        "OK"
    } else {
        "LENTO"
    }
}

fn main() {
    let mediciones = [120_u64, 1000, 1500];

    for ms in mediciones {
        println!("{ms}ms: {}", etiqueta(ms));
    }
}
```

El sufijo `_u64` en el primer literal fija el tipo del arreglo. Los demás elementos deben ser del mismo tipo, así que Rust los interpreta también como `u64`. La constante expresa tanto el valor como la unidad de la regla. Si cambiaras el arreglo a `i32`, deberías convertir cada dato de manera explícita o cambiar el contrato de la función; ambas decisiones tienen significado y no deben ocurrir por accidente.

## Cómo sé que lo logré

- Desde `programas/01-fundamentos`, `rustc --edition 2024 -D warnings fig01_01.rs && ./fig01_01` imprime `x = 5, y = 15, MAX = 100000` sin avisos.
- `rustc --edition 2024 fig01_02.rs` falla con `error[E0308]`, y puedes señalar que `b` espera `i64` mientras `a` es `i32`.
- `rustc --edition 2024 fig01_05.rs` falla con `error[E0308]`, y puedes explicar que el punto y coma hizo que la función devolviera `()`.
- `rustc --edition 2024 -D warnings fig01_06.rs && ./fig01_06` imprime los dos rangos, los tres servicios y termina con `x = 0, r = 42`.
- `rustc --edition 2024 -D warnings fig01_07.rs && ./fig01_07` imprime los tres resultados documentados y no reporta avisos.
- Terminaste las secciones `variables`, `functions`, `if` y `primitive_types` de Rustlings, y puedes resolver los tres ejercicios sin copiar las soluciones.

## Para leer más

- [The Rust Programming Language, capítulo 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) — consulta: 2 de octubre de 2026.
- [The Rust Programming Language, capítulo 3: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) — consulta: 2 de octubre de 2026.
- [Documentación oficial de `i32` y de los tipos numéricos primitivos](https://doc.rust-lang.org/std/primitive.i32.html) — consulta: 2 de octubre de 2026.
- [Rustlings](https://rustlings.rust-lang.org/) — completa `variables`, `functions`, `if` y `primitive_types`; consulta: 2 de octubre de 2026.
