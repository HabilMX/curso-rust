# Lección 5 — Traits, genéricos y lifetimes

**Tiempo:** 2 × 45 min.

**Qué construyes:** el trait `Revisor` y una función genérica que lo usa, en programas aparte (el `revisor` real no declara ningún trait).

**Qué aprendes:** traits y métodos por omisión, genéricos con restricciones, lifetimes y el `'a` que asusta.

## Al terminar vas a poder

- Definir un trait, implementar su contrato para dos tipos y usar un método por omisión.
- Distinguir una implementación inherente (`impl Tipo`) de una implementación de trait (`impl Trait for Tipo`).
- Escribir una función genérica con restricciones y explicar cuándo Rust genera código especializado.
- Elegir entre un parámetro genérico y `dyn Trait` según si necesitas decidir el tipo en compilación o ejecución.
- Leer una firma con `'a` y explicar qué referencias quedan relacionadas por ese lifetime.
- Reconocer un error de lifetime o de trait bound, ubicar su causa y corregir el diseño sin usar copias innecesarias.

## El porqué antes del cómo

Hasta la lección 4, el `revisor` ya tiene un modelo útil. Puede representar un `Servicio`, guardar una lista en un `Vec`, distinguir resultados con `Estado`, cargar configuración y reportar errores. Sin embargo, todavía hay una pregunta de diseño que aparece cada vez que el programa crece: ¿cómo separas lo que el programa necesita hacer de la forma concreta en que se hace?

El `revisor` necesita obtener un `Estado` para cada `Servicio`. Hoy la implementación real hace una consulta HTTP con `reqwest::Client`; mañana podrías querer una implementación que lea un archivo, consulte una base de datos, mida un proceso local o simule respuestas para una prueba. El resto del programa no debería tener que conocer todos esos detalles. Solo necesita poder pedir: “revisa este servicio y devuelve su estado”.

En Go, ese contrato se expresa con una interfaz. Un tipo satisface una interfaz de manera implícita: si tiene los métodos requeridos, ya cumple. Esa decisión hace muy fácil adaptar tipos existentes, pero también puede ocultar relaciones importantes. Un tipo puede terminar cumpliendo una interfaz por accidente, y al leer su definición no siempre sabes qué contratos participa en otros paquetes.

Rust usa traits para resolver la misma clase de problema, pero exige declarar la relación de forma explícita. Un trait describe capacidades; después, `impl Revisor for RevisorHttp` declara que ese tipo cumple esa capacidad. Es una línea más, pero es una línea que documenta arquitectura. Al leerla sabes que `RevisorHttp` no solo tiene un método llamado `revisar`: se comprometió con el contrato `Revisor`.

Los traits no reemplazan a los structs ni a los enums. Cada herramienta responde una pregunta distinta. Un `struct` dice qué datos forman una cosa; un `enum` dice cuáles alternativas válidas existen; un trait dice qué operaciones puede ofrecer un tipo. El `Servicio` de la lección 3 sigue siendo un struct porque modela datos. `Estado` sigue siendo un enum porque un servicio puede estar sano, lento, fallar o no haberse consultado. `Revisor` es un trait porque describe la operación que produce un estado.

Los genéricos hacen posible escribir una función que trabaja con una familia de tipos sin perder información sobre cuál tipo concreto recibió. La función `revisar_todos` de esta lección puede aceptar cualquier `R` que implemente `Revisor`. No necesita un `if` por cada implementación ni convertir todo a texto. El compilador conoce el tipo concreto de `R` al compilar cada llamada y puede verificar que existe el método correcto.

Los lifetimes completan este modelo cuando trabajas con referencias. Ownership ya estableció que cada valor tiene un dueño y que una referencia es un préstamo. Un lifetime no crea otra forma de propiedad ni prolonga un valor. Es una anotación que ayuda al compilador a demostrar que un préstamo seguirá siendo válido durante todo uso posible. Aparece sobre todo cuando una función recibe referencias y devuelve una referencia, o cuando un struct guarda referencias.

La notación `'a` intimida porque parece una variable misteriosa, pero se lee mejor como una etiqueta. Si una función recibe dos referencias marcadas con `'a` y devuelve otra marcada con `'a`, está declarando: “la referencia de salida depende de estas entradas y no puede usarse después de que deje de ser válida la referencia más corta”. No dice cuánto dura `'a`; eso depende de cada llamada. Tampoco reserva memoria ni hace recolección de basura.

Esta lección corresponde al capítulo 10 de The Rust Book. Antes de seguir, lee sus secciones sobre genéricos, traits y validación de referencias, y realiza los ejercicios `generics`, `traits` y `lifetimes` de Rustlings. El objetivo no es memorizar todas las sintaxis posibles de bounds y lifetimes. Es aprender a reconocer tres preguntas: qué comportamiento necesita el programa, qué tipos pueden ofrecerlo y de dónde vienen las referencias que sobreviven a una función.

## Los conceptos

### Traits: contratos explícitos y métodos por omisión

Un trait reúne firmas de métodos que representan una capacidad. La firma dice qué recibe el método y qué devuelve, sin decidir cómo hará el trabajo. Cada tipo que quiera cumplir el trait escribe una implementación propia. Por eso un trait se parece a una interfaz de Go, pero su relación con el tipo es explícita.

La figura define `Revisor` con dos métodos. `revisar` no tiene cuerpo: toda implementación debe decidir cómo revisar un servicio. `nombre` sí tiene cuerpo y devuelve `"revisor"`. Ese es un método por omisión. Una implementación puede aceptarlo tal cual, como `RevisorHttp`, o reemplazarlo, como `RevisorFalso`.

**Fig. 5.1** | Un trait con método por omisión, y dos tipos que lo cumplen.

```rust
// fig05_01.rs
use std::fmt;

struct Servicio {
    nombre: String,
}

enum Estado {
    Ok { ms: u64 },
    Falla(String),
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Estado::Ok { ms } => write!(f, "OK en {ms}ms"),
            Estado::Falla(motivo) => write!(f, "FALLA: {motivo}"),
        }
    }
}

trait Revisor {
    fn revisar(&self, s: &Servicio) -> Estado;

    fn nombre(&self) -> String {
        "revisor".to_string()
    }
}

struct RevisorHttp { timeout_ms: u64 }

impl Revisor for RevisorHttp {
    fn revisar(&self, s: &Servicio) -> Estado {
        Estado::Falla(format!("{}: sin red en este ejemplo (límite {} ms)", s.nombre, self.timeout_ms))
    }
}

struct RevisorFalso;

impl Revisor for RevisorFalso {
    fn revisar(&self, _s: &Servicio) -> Estado {
        Estado::Ok { ms: 1 }
    }
    fn nombre(&self) -> String {
        "falso".to_string()
    }
}

fn main() {
    let s = Servicio { nombre: "catalogo".to_string() };
    let http = RevisorHttp { timeout_ms: 2000 };
    println!("{} -> {}", http.nombre(), http.revisar(&s));
    println!("{} -> {}", RevisorFalso.nombre(), RevisorFalso.revisar(&s));
}
```

```bash
$ rustc --edition 2024 fig05_01.rs && ./fig05_01
revisor -> FALLA: catalogo: sin red en este ejemplo (límite 2000 ms)
falso -> OK en 1ms
```

`impl Revisor for RevisorHttp` se lee de izquierda a derecha: “implementa el trait `Revisor` para el tipo `RevisorHttp`”. Dentro de ese bloque, Rust exige implementar cada método sin cuerpo que el trait requiera. Si omites `revisar`, el programa no compila. Si omites `nombre`, sí compila porque el trait ya proporcionó una implementación por omisión.

El método recibe `&self`, igual que los métodos de structs de la lección 3. No consume el revisor ni lo modifica; solo lo presta para consultar sus datos. `revisar` recibe también `&Servicio`, porque consultar un servicio no debe consumirlo. El resultado, en cambio, se devuelve por valor: cada revisión crea un `Estado` nuevo y quien llama recibe su propiedad.

El trait `Display` de la figura viene de la biblioteca estándar. `impl fmt::Display for Estado` permite usar `{}` dentro de `println!`. La implementación decide una representación dirigida a una persona: `OK en 1ms` o `FALLA: ...`. Esto es distinto de `Debug`, que normalmente se obtiene con `#[derive(Debug)]` y se imprime con `{:?}` para diagnóstico. Si el reporte es parte de la interfaz del programa, definir `Display` obliga a pensar qué texto estable merece ver quien lo ejecuta.

Un trait no es una clase base. No guarda campos, no construye objetos y no hereda implementación de un padre. Puede proporcionar comportamiento por omisión, pero cada tipo conserva sus propios datos. `RevisorHttp` tiene `timeout_ms`; `RevisorFalso` no necesita ningún campo. Ambos cumplen el mismo contrato porque ambos pueden responder a `revisar(&Servicio)`.

Rust aplica la regla de coherencia, también llamada regla huérfana. Puedes implementar un trait tuyo para un tipo ajeno, por ejemplo `impl Revisor for String` si tuviera sentido. También puedes implementar un trait ajeno para un tipo tuyo, como `impl Display for Estado`. Lo que no puedes hacer es implementar un trait ajeno para un tipo ajeno: no puedes decidir desde tu crate cómo debe implementar `Display` un `Vec<String>`. La regla evita que dos dependencias distintas definan implementaciones incompatibles del mismo contrato.

El `revisor` real no declara ningún trait para sus consultas HTTP. Su función `revisar` recibe un `reqwest::Client` concreto, y sus pruebas de integración usan un servidor HTTP local en lugar de un doble de pruebas. Es una decisión consciente: un trait que tendría una sola implementación real todavía no resuelve ningún problema. El trait `Revisor` de esta lección vive en programas aparte, para que practiques la forma; el proyecto lo necesitaría el día que existan dos maneras distintas de revisar un servicio. Mientras tanto, el programa sí usa `impl` para agrupar métodos propios de los tipos del dominio:

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

Este bloque es una implementación inherente: `impl Estado`, sin `for`, agrega un método que pertenece directamente a `Estado`. No implementa un trait. Distinguir ambas formas evita una confusión común: toda implementación de trait usa `impl`, pero no todo `impl` implementa un trait.

Un trait sería útil en el `revisor` si la aplicación necesitara intercambiar la fuente de las comprobaciones dentro del mismo diseño. Por ejemplo, una prueba unitaria podría usar un revisor falso sin red. No debes crear un trait solo porque Rust lo ofrece. La abstracción tiene costo de lectura: añade un contrato, implementaciones y decisiones sobre cómo inyectarlas. El proyecto actual prueba HTTP mediante un servidor local precisamente porque quiere verificar el comportamiento real de la capa HTTP.

### Genéricos y restricciones: reutilizar sin borrar el tipo

Un parámetro genérico es una variable de tipo. En `fn revisar_todos<R: Revisor>(...)`, `R` no significa “cualquier valor sin reglas”; significa “cualquier tipo que implemente `Revisor`”. La parte después de los dos puntos es una restricción, también llamada trait bound. Gracias a ella, el cuerpo de la función puede llamar `r.revisar(s)`: el compilador tiene la garantía de que cualquier `R` admitido proporciona ese método.

**Fig. 5.2** | Una función genérica con restricciones.

```rust
// fig05_02.rs
use std::fmt;

struct Servicio {
    nombre: String,
}

enum Estado {
    Ok { ms: u64 },
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Estado::Ok { ms } => write!(f, "OK en {ms}ms"),
        }
    }
}

trait Revisor {
    fn revisar(&self, s: &Servicio) -> Estado;
}

struct RevisorFalso;

impl Revisor for RevisorFalso {
    fn revisar(&self, s: &Servicio) -> Estado {
        Estado::Ok { ms: s.nombre.len() as u64 }
    }
}

fn revisar_todos<R: Revisor>(r: &R, servicios: &[Servicio]) -> Vec<Estado> {
    servicios.iter().map(|s| r.revisar(s)).collect()
}

fn imprimir<T: std::fmt::Display + Clone>(x: T) { println!("{x}"); }

fn main() {
    let servicios = vec![
        Servicio { nombre: "catalogo".to_string() },
        Servicio { nombre: "pagos".to_string() },
    ];
    for estado in revisar_todos(&RevisorFalso, &servicios) {
        imprimir(estado.to_string());
    }
}
```

```bash
$ rustc --edition 2024 fig05_02.rs && ./fig05_02
OK en 8ms
OK en 5ms
```

La función recibe `&R`, no `R`. Eso mantiene la propiedad del revisor en quien llama y permite usarlo para todos los servicios. `servicios: &[Servicio]` es un slice prestado, igual que los slices vistos al recorrer colecciones: la función puede leer los servicios, pero no los consume ni necesita una copia del `Vec`.

`map` recibe cada `&Servicio`, llama a `r.revisar(s)` y produce un iterador de estados. `collect()` reúne esos estados en `Vec<Estado>` porque el tipo de retorno lo pide. La función es genérica en el revisor, pero no en el estado: el contrato `Revisor` fija que toda implementación devuelve `Estado`. Esa elección es correcta cuando el dominio necesita una sola representación coherente de los resultados.

La forma `T: Display + Clone` muestra varias restricciones unidas con `+`. Sin embargo, `imprimir` solo usa `Display`; no llama `clone`. La restricción `Clone` está ahí para enseñar la sintaxis, no porque sea necesaria. En código de producción debes pedir únicamente las capacidades que el cuerpo necesita. Un bound de más excluye tipos válidos y hace que la API parezca más exigente de lo que realmente es.

Cuando los bounds crecen, Rust permite escribirlos con `where`. Por ejemplo, una firma larga puede terminar con `where R: Revisor, E: std::error::Error`. No cambia el comportamiento ni la verificación; solo coloca las restricciones donde se leen mejor. Empieza con la forma corta y usa `where` cuando la firma deje de ser clara.

Los genéricos de Rust se resuelven normalmente mediante monomorfización. Si llamas `revisar_todos` con `RevisorFalso` y después con otro tipo `RevisorArchivo`, el compilador genera versiones especializadas para esos tipos concretos. En tiempo de ejecución no necesita buscar el método en una tabla para esas llamadas. A esto se le llama despacho estático. El beneficio es rendimiento predecible y verificaciones más precisas; el costo es que cada combinación de tipos puede aumentar el código compilado.

`impl Revisor` en un parámetro es una forma breve de escribir un genérico de entrada. Una firma como `fn ejecutar(r: impl Revisor)` equivale, para ese caso simple, a `fn ejecutar<R: Revisor>(r: R)`. La forma con `<R: Revisor>` es preferible cuando debes usar el mismo tipo genérico más de una vez en la firma, devolverlo o añadir relaciones entre varios parámetros.

Cuando la decisión del tipo debe tomarse en ejecución, aparece el trait object: `Box<dyn Revisor>`. Un `Vec<Box<dyn Revisor>>` puede guardar en la misma colección un `RevisorHttp`, un `RevisorFalso` y otros revisores de tamaños distintos. A cambio, cada llamada pasa por una indirección y el valor suele vivir detrás de un puntero como `Box`, `&` o `Arc`. Es el equivalente más cercano a una interfaz de Go en tiempo de ejecución.

No hay una opción universalmente mejor. Usa genéricos cuando el tipo concreto se conoce donde se compila la llamada y quieres conservar esa información. Usa `dyn Trait` cuando el programa necesita elegir o combinar implementaciones durante la ejecución. En Go, las interfaces suelen llevar despacho dinámico por su diseño habitual; en Rust eliges explícitamente entre ambos modelos.

El proyecto real también usa tipos genéricos de la biblioteca estándar, aunque no declare una función propia con `<T>`. `Option<T>` expresa que puede haber o no un valor de cualquier tipo, y aquí se especializa como `Option<u16>` para un código HTTP:

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

`Option<u16>` y `Option<String>` son usos distintos del mismo tipo genérico. El primero permite representar que una falla no tuvo código HTTP; el segundo permite omitir el campo de error cuando un servicio respondió bien. Los genéricos no son solo una técnica para bibliotecas sofisticadas: `Vec<T>`, `Option<T>`, `Result<T, E>` y `HashMap<K, V>` son parte del trabajo diario en Rust.

### Lifetimes: describir préstamos que se relacionan

Un lifetime es una región de validez de una referencia. Casi siempre Rust lo infiere, igual que infiere muchos tipos locales. Necesitas escribir una anotación cuando la firma podría permitir varias relaciones entre referencias y el compilador no puede saber cuál garantiza el diseño.

La figura devuelve una de dos referencias. Sin una anotación, la firma no puede comunicar si el resultado viene de `a`, de `b` o de otro lugar. Al marcar las tres referencias con `'a`, declaras que el resultado será válido durante un periodo que no puede superar al de ninguna entrada elegida.

**Fig. 5.3** | Un lifetime que une la salida con las dos entradas.

```rust
// fig05_03.rs
fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", mas_largo("catalogo", "pagos"));
}
```

```bash
$ rustc --edition 2024 fig05_03.rs && ./fig05_03
catalogo
```

`'a` no significa “vive para siempre” ni “vive exactamente lo mismo que ambas entradas”. Es un nombre para una relación. En una llamada concreta, Rust calcula un lifetime que cabe dentro de los préstamos válidos. Si `a` dura diez líneas y `b` dura tres, el resultado solo podrá usarse durante las tres líneas compatibles. La firma evita que alguien guarde una referencia al resultado y después destruya el valor del que provenía.

La función no elige cuál cadena dura más. Eso depende de los scopes de quien llama, no de la cantidad de letras ni de un valor guardado en memoria. `mas_largo` compara longitudes para elegir contenido, pero el lifetime habla de validez de referencias. Son dos asuntos diferentes que casualmente aparecen en la misma función.

Rust tiene reglas de elisión que hacen invisibles muchos lifetimes. Por ejemplo, `fn nombre(s: &str) -> &str` compila sin escribir `'a` porque hay una sola referencia de entrada y Rust puede asociar la salida a ella. También suele inferir lifetimes de métodos que reciben `&self`. Cuando hay dos entradas posibles, como en `mas_largo`, ya no es seguro adivinar y debes describir la relación.

No agregues `'a` a todo lo que parezca complicado. Una anotación no arregla una referencia inválida; solo declara una relación que el compilador comprobará. Si intentas devolver una referencia a un `String` local, no existe un lifetime que pueda volver válido ese préstamo. El `String` se destruye al terminar la función. La solución es devolver el `String` por valor, recibir una referencia que pertenezca a quien llama o rediseñar quién posee el dato.

El lifetime especial `'static` merece cuidado. Una referencia `&'static str` suele apuntar a texto literal incluido en el binario, como `"OK"` o `"FALLA"`. No significa “usa `'static` para quitar errores”. Forzar `'static` sobre un dato que en realidad vive poco tiempo no lo hace durar más; el compilador lo rechazará. Usa `'static` solo cuando el valor realmente vive durante toda la ejecución.

El `revisor` real usa lifetimes donde sí hacen falta: en una fila temporal que presta un `Servicio` y su `Estado` correspondiente para ordenar el reporte. No copia esos valores solo para ordenarlos. Construye referencias, las guarda en un vector local y deja que el compilador compruebe que el vector no sobrevive a sus fuentes.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

`Fila<'a>` es un alias para una tupla de dos referencias. No posee un `Servicio` ni un `Estado`; solo los presta. `ordenadas` recibe dos slices con el mismo lifetime anotado y devuelve filas que también llevan ese lifetime. Por tanto, nadie puede conservar las filas después de que desaparezcan los vectores originales. El vector de filas puede cambiar de orden porque es dueño del vector, pero no puede modificar los servicios ni los estados porque solo los presta.

La misma función muestra una razón práctica para preferir referencias: evita clonar información solo para mostrarla ordenada. Clonar sería válido si necesitaras una colección independiente que sobreviviera al reporte, pero no es necesario aquí. El reporte termina de usar `filas` antes de que terminen `servicios` y `estados`, así que los préstamos expresan exactamente el modelo de datos.

Cuando un lifetime aparece en un struct o alias, no debes leerlo como sintaxis ceremonial. Pregunta: “¿este tipo guarda una referencia?” Si la respuesta es sí, la anotación vincula el tipo con la duración del valor prestado. Si la respuesta es no, probablemente el tipo debería poseer un `String`, `Vec<T>` u otro valor y no necesita lifetime explícito.

## El error que vas a ver

### E0515: devolver una referencia a un valor local

Este error aparece cuando una función intenta prestar algo que deja de existir al regresar. La siguiente figura falla a propósito. El lifetime `'a` de la firma no puede salvar a `nombre`: ese `String` es propiedad de `devolver` y se destruye al cerrar la función.

**Fig. 5.4** | Un préstamo que intenta escapar del valor que lo posee.

```rust
// fig05_04.rs
fn devolver<'a>() -> &'a str {
    let nombre = String::from("catalogo");
    &nombre
}

fn main() {
    println!("{}", devolver());
}
```

```bash
$ rustc --edition 2024 fig05_04.rs
error[E0515]: cannot return reference to local variable `nombre`
 --> fig05_04.rs:4:5
  |
4 |     &nombre
  |     ^^^^^^^ returns a reference to data owned by the current function

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0515`.
```

El mensaje señala exactamente la referencia que pretende escapar. No agregues otro lifetime ni uses `&'static str`: ninguno cambia quién posee `nombre`. Si la función debe crear el texto, la corrección es devolver `String`. Si debe devolver una vista de texto que ya existía, recibe `&str` como parámetro y relaciona el lifetime de salida con esa entrada.

### E0277: el tipo no cumple la restricción pedida

Un trait bound también es un contrato verificable. En esta figura, `imprimir` pide un tipo que implemente `Display`, pero `Vec<&str>` no tiene esa implementación. Rust no lo convierte a texto de manera implícita porque no existe una única representación correcta para todas las colecciones.

**Fig. 5.5** | Un argumento que no cumple el trait bound.

```rust
// fig05_05.rs
use std::fmt::Display;

fn imprimir<T: Display>(valor: T) {
    println!("{valor}");
}

fn main() {
    imprimir(vec!["catalogo"]);
}
```

```bash
$ rustc --edition 2024 fig05_05.rs
error[E0277]: `Vec<&str>` doesn't implement `std::fmt::Display`
 --> fig05_05.rs:9:14
  |
9 |     imprimir(vec!["catalogo"]);
  |     -------- ^^^^^^^^^^^^^^^^ the trait `std::fmt::Display` is not implemented for `Vec<&str>`
  |     |
  |     required by a bound introduced by this call
  |
note: required by a bound in `imprimir`
 --> fig05_05.rs:4:16
  |
4 | fn imprimir<T: Display>(valor: T) {
  |                ^^^^^^^ required by this bound in `imprimir`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

`E0277` dice que no se cumplió una restricción de trait. La nota te lleva a la firma que introdujo la exigencia. La corrección depende de la intención: puedes imprimir con `{:?}` si quieres una representación de depuración y el tipo implementa `Debug`; puedes recorrer el vector e imprimir cada elemento; o puedes convertirlo explícitamente a un `String` con el formato que necesita tu reporte. No implementes `Display` para un tipo ajeno solo para apagar el error: la regla huérfana lo impedirá y, además, sería una decisión global difícil de justificar.

## Lo que se hace mal

### Crear un trait para cada struct

Un trait debe representar una capacidad compartida, no repetir el nombre de un tipo. Si solo existe una implementación y no hay una razón concreta para intercambiarla, un método inherente suele ser más claro. `impl Estado { fn esta_bien(...) }` expresa que la operación pertenece naturalmente a `Estado`. Crear un trait `EstadoConsultable` para una sola función solo añade nombres y archivos sin separar una dependencia real.

Empieza con structs, enums y funciones directas. Extrae un trait cuando varias implementaciones deban cumplir el mismo contrato, cuando necesites recibir una capacidad en vez de un tipo concreto o cuando una frontera de pruebas realmente lo justifique.

### Agregar bounds “por si acaso”

Es común copiar una firma como `T: Clone + Debug + Display` y conservar todos los bounds aunque el cuerpo solo use `Display`. Cada bound limita los tipos que pueden llamar la función. Además, cada capacidad prometida por una firma se vuelve parte de la API que otros deben entender.

Pide `Clone` solo si el cuerpo llama `clone`, `Ord` solo si ordena y `Send` solo si mueve datos a otro hilo. Un bound pequeño es una abstracción más flexible y describe mejor la necesidad real. La figura 5.2 conserva `Clone` para mostrar que se pueden combinar restricciones, pero no es un modelo para copiar literalmente.

### Usar `Box<dyn Trait>` por costumbre

Un trait object resuelve un problema real: almacenar o elegir implementaciones diferentes en tiempo de ejecución. No es la forma obligatoria de usar traits. Si el tipo se conoce al compilar, un parámetro genérico normalmente es más simple, evita asignaciones en heap innecesarias y permite despacho estático.

La pregunta útil no es “¿traits o genéricos?”. Un trait describe una capacidad; después eliges si la recibes con genéricos, como `impl Trait`, mediante una referencia `&dyn Trait` o detrás de `Box<dyn Trait>`. La elección depende de propiedad, tamaño y momento en que conoces el tipo.

### Clonar para silenciar errores de borrow checker

Si una referencia no vive lo suficiente, copiar un `String` con `.clone()` puede hacer que el programa compile, pero no siempre resuelve el diseño correcto. A veces solo oculta que una función debería devolver una referencia, que un tipo debería poseer sus datos o que un préstamo dura más de lo necesario.

Haz primero el diagnóstico: identifica el dueño, identifica quién necesita usar el dato después y decide si necesita una vista o una copia independiente. Clona cuando dos dueños legítimos necesitan conservar valores separados. El vector de filas del `revisor` no clona servicios ni estados porque solo necesita ordenarlos mientras sus dueños siguen vivos.

### Leer `'a` como una duración concreta

`'a` no significa un segundo, un scope fijo ni una variable creada al inicio del programa. Es una etiqueta que Rust sustituye por una región válida en cada llamada. Dos funciones pueden usar el nombre `'a` sin compartir absolutamente nada; el nombre solo tiene significado dentro de su propia firma.

También es un error pensar que más anotaciones son más seguras. Las anotaciones deben reflejar de dónde viene una referencia. Si no puedes explicar qué referencia de entrada respalda la salida, probablemente la función debe devolver un valor propio en vez de una referencia.

## Ejercicios

### Ejercicio 1 — Un revisor falso con nombre por omisión

Define un trait `Revisor` con `revisar(&self, servicio: &Servicio) -> Estado` y un método por omisión `nombre() -> String`. Crea `RevisorFalso` que devuelva `Estado::Ok { ms: 1 }` sin reemplazar `nombre`. Comprueba que imprime `revisor -> OK en 1ms`.

Después agrega `RevisorArchivo`, que reemplace `nombre` por `"archivo"` y devuelva una falla determinista. Explica en una frase por qué ambos tipos pueden usarse donde se espera un `Revisor`.

### Ejercicio 2 — Contar resultados sanos de manera genérica

Usa el trait de la figura 5.1 y escribe una función `contar_sanos<R: Revisor>`. Debe recibir un revisor y un slice de servicios, revisarlos y devolver cuántos estados son `Ok`. Prueba la función con tres servicios y `RevisorFalso`.

Antes de programar, decide qué debe poseer cada parte: la función no debe consumir el revisor ni el vector de servicios. Usa `&R` y `&[Servicio]`, no clones.

### Ejercicio 3 — Leer el lifetime del reporte

Abre `programas/revisor/src/reporte.rs` y localiza `Fila<'a>` y `ordenadas<'a>`. Escribe con tus palabras qué posee el `Vec<Fila<'a>>`, qué toma prestado y qué ocurriría si intentaras devolver esas filas después de destruir `servicios` o `estados`.

Luego escribe una función `primero<'a>` que reciba `&'a str` y devuelva `&'a str`. Compárala con `devolver` de la figura 5.4 y explica por qué una compila y la otra no.

## Soluciones

### Solución 1

La implementación que acepta el método por omisión no escribe `nombre`; el trait proporciona su cuerpo. La segunda implementación sí lo reemplaza porque necesita una etiqueta distinta.

<!-- verificar:fragmento -->
```rust
trait Revisor {
    fn revisar(&self, servicio: &Servicio) -> Estado;

    fn nombre(&self) -> String {
        "revisor".to_string()
    }
}

struct RevisorFalso;
struct RevisorArchivo;

impl Revisor for RevisorFalso {
    fn revisar(&self, _servicio: &Servicio) -> Estado {
        Estado::Ok { ms: 1 }
    }
}

impl Revisor for RevisorArchivo {
    fn revisar(&self, servicio: &Servicio) -> Estado {
        Estado::Falla(format!("{}: no existe el archivo", servicio.nombre))
    }

    fn nombre(&self) -> String {
        "archivo".to_string()
    }
}
```

Ambos tipos se pueden usar donde se espera un `Revisor` porque ambos escribieron `impl Revisor for ...` y proporcionan el método obligatorio `revisar`. La diferencia entre sus datos y su algoritmo queda encapsulada dentro de cada implementación.

### Solución 2

La función toma préstamos porque solo necesita consultar los datos. Cada resultado es temporal: se cuenta y se descarta. No hay razón para almacenar un `Vec<Estado>` ni para clonar el revisor o los servicios.

<!-- verificar:fragmento -->
```rust
fn contar_sanos<R: Revisor>(revisor: &R, servicios: &[Servicio]) -> usize {
    servicios
        .iter()
        .filter(|servicio| matches!(revisor.revisar(servicio), Estado::Ok { .. }))
        .count()
}
```

`iter()` produce `&Servicio`; el closure recibe cada préstamo y llama al trait mediante `&R`. `matches!` decide si el estado pertenece a la variante `Ok`, y `count()` devuelve el total. Si `Estado` tuviera también `Lento`, debes decidir explícitamente si cuenta como sano; el `revisor` real responde esa pregunta con `Estado::esta_bien()`.

### Solución 3

`Vec<Fila<'a>>` posee el vector y el orden de sus elementos, pero no posee los servicios ni los estados. Cada elemento contiene dos referencias. Por eso sus filas solo pueden vivir mientras sigan vivos los slices prestados a `ordenadas`. Intentar devolverlas para usarlas después de destruir las colecciones originales produciría un error de borrow checker: serían referencias colgantes.

La función correcta devuelve una referencia que pertenece a quien llama:

<!-- verificar:fragmento -->
```rust
fn primero<'a>(texto: &'a str) -> &'a str {
    texto
}
```

`primero` no crea el texto ni intenta prestarlo después de destruirlo. Solo devuelve el mismo préstamo que recibió. En cambio, `devolver` crea un `String` local, es su dueño y lo destruye al salir; por eso la referencia de la figura 5.4 no puede escapar.

## Cómo sé que lo logré

- `rustc --edition 2024 fig05_01.rs && ./fig05_01` imprime las dos líneas documentadas, incluida la etiqueta por omisión de `RevisorHttp`.
- `rustc --edition 2024 fig05_02.rs && ./fig05_02` imprime `OK en 8ms` y `OK en 5ms`.
- `rustc --edition 2024 fig05_03.rs && ./fig05_03` imprime `catalogo`.
- `rustc --edition 2024 fig05_04.rs` falla con `error[E0515]`; puedes explicar por qué cambiar `'a` no arregla el préstamo local.
- `rustc --edition 2024 fig05_05.rs` falla con `error[E0277]`; puedes localizar tanto la llamada incorrecta como el bound que la rechazó.
- Puedes señalar `Fila<'a>` en `programas/revisor/src/reporte.rs` y explicar que presta servicios y estados en vez de clonarlos.
- Terminaste los ejercicios `generics`, `traits` y `lifetimes` de Rustlings.

## Para leer más

- [The Rust Programming Language, capítulo 10.1: sintaxis de tipos genéricos](https://doc.rust-lang.org/book/ch10-01-syntax.html), consultado el 2 de octubre de 2026.
- [The Rust Programming Language, capítulo 10.2: traits](https://doc.rust-lang.org/book/ch10-02-traits.html), consultado el 2 de octubre de 2026.
- [The Rust Programming Language, capítulo 10.3: validación de referencias con lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), consultado el 2 de octubre de 2026.
- [Rustlings: ejercicios de genéricos, traits y lifetimes](https://rustlings.rust-lang.org/), consultado el 2 de octubre de 2026.
