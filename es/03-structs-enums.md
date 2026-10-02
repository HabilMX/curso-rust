# Lección 3 — Structs, enums y match

**Tiempo:** 2 × 45 min

**Qué construyes:** el modelo del `revisor`: `Servicio` y `Estado`

**Qué aprendes:** structs e `impl`, enums que llevan datos, `match` exhaustivo, `Option` en lugar de `nil`

**The Rust Book, capítulos 5 y 6.** Rustlings: `structs`, `enums`, `options`.

## Al terminar vas a poder

- Modelar un servicio con un `struct` cuyos campos tengan nombre y tipo.
- Escribir métodos dentro de un bloque `impl` y decidir si reciben `&self`, `&mut self` o `self`.
- Representar resultados incompatibles entre sí con un `enum` que lleve datos.
- Escribir un `match` exhaustivo que extraiga datos de cada variante.
- Explicar por qué agregar una variante nueva obliga a revisar decisiones existentes.
- Usar `Option<T>` cuando un valor puede faltar, sin recurrir a `nil`.
- Elegir entre `match`, `if let` y métodos como `unwrap_or` según la intención del código.

## El porqué antes del cómo

Hasta aquí el curso ha usado valores simples: números para tiempos, cadenas para nombres y condiciones para clasificar una respuesta. Eso alcanza para practicar variables, funciones, tipos y ownership, pero no alcanza para describir el dominio real del `revisor`. Un servicio no es solo un nombre, una URL y un tiempo límite que casualmente aparecen juntos en tres variables. Son tres datos que describen una sola cosa y que deben viajar, validarse y consultarse como una unidad.

Guardar esos datos por separado produce errores silenciosos. Imagina que tienes `nombre_catalogo`, `url_catalogo`, `timeout_catalogo`, después agregas los mismos tres valores para pagos y reportes, y al construir el reporte unes el nombre de pagos con la URL de catálogo. El compilador no puede detectar el problema: las tres piezas tienen tipos válidos, pero su relación se perdió. Un `struct` permite declarar esa relación una vez y convertirla en parte del tipo.

El segundo problema aparece después de consultar un servicio. Una respuesta sana trae un código HTTP y una duración. Una respuesta lenta también trae ambos datos, pero requiere una etiqueta distinta. Una falla puede traer un mensaje y una duración, pero no necesariamente un código HTTP. Y un servicio que aún no se consulta no tiene ni código, ni duración, ni mensaje de falla. Si intentaras guardar todo eso en un solo `struct` con campos “a veces válidos”, tendrías combinaciones absurdas: un estado de falla con código `200`, un servicio no intentado con duración de `0 ms` que nadie sabe interpretar, o un mensaje de error vacío que significa cosas distintas según otro campo booleano.

Rust resuelve ese modelado con `enum`. A diferencia de un enum tradicional de otros lenguajes, que suele ser una lista de números o constantes con nombre, una variante de Rust puede llevar datos. `Estado::Ok` lleva código y milisegundos; `Estado::Falla` lleva un motivo; `Estado::NoIntentado` no lleva nada porque no hay datos honestos que guardar. El tipo expresa que un valor está en exactamente uno de esos estados, nunca en varios a la vez.

La tercera pieza es `match`. Cuando recibes un `Estado`, no basta saber que pertenece al enum: necesitas decidir qué hacer con cada posibilidad. Rust exige que esa decisión cubra todas las variantes. No es una recomendación de estilo ni una regla de un linter; forma parte de la compilación. Si mañana agregas `Estado::Rechazado`, cada `match` que antes parecía terminado se convierte en un lugar que el compilador te señala para revisar. Esa obligación es una red de seguridad para refactorizaciones.

El curso de Go construye el mismo `revisor`, pero aquí aparece una diferencia importante entre ambos lenguajes. En Go, un resultado se suele modelar con un `struct`, campos de valor cero, punteros y convenciones sobre cuáles campos están presentes. En Rust, el tipo puede representar directamente alternativas incompatibles. No elimina la necesidad de pensar en el dominio, pero hace que las decisiones correctas sean más fáciles de expresar y las inconsistencias más difíciles de compilar.

La última parte del modelo es la ausencia. En muchos lenguajes una referencia puede ser `null` o `nil` aunque su tipo no lo diga de forma visible. El programa llega a una línea que esperaba un objeto, recibe ausencia y falla durante la ejecución. Rust no tiene `nil`. Cuando algo puede faltar, su tipo lo declara mediante `Option<T>`. Eso obliga a tomar una decisión antes de usar el contenido: manejar `Some(valor)`, manejar `None` o proporcionar una alternativa explícita. La ausencia deja de ser un accidente escondido y se vuelve parte del contrato de la función.

Esta lección no trata de memorizar toda la sintaxis de patrones. Trata de aprender a preguntarte qué estados reales existen, qué datos pertenecen a cada estado y qué decisiones deben cambiar cuando el modelo cambia. Esas preguntas reaparecen en la lección 4 con `Result`, en la 5 con traits, en la 6 con pruebas y en la 8 cuando el `revisor` genera JSON.

## Los conceptos

### Structs: un nombre para datos que pertenecen juntos

Un `struct` define un tipo compuesto con campos nombrados. La palabra importante es “tipo”: después de definir `Servicio`, Rust deja de ver una colección informal de tres datos y empieza a ver un valor que representa un servicio. Cada campo conserva su propio tipo, así que el compilador sigue distinguiendo texto de números, pero ahora también conoce que esos valores forman una sola entidad.

Un struct con campos nombrados es una buena elección cuando cada posición tiene significado propio. Una tupla como `(String, String, u64)` puede almacenar nombre, URL y tiempo límite, pero obliga a recordar qué significa `.0`, `.1` y `.2`. Con `Servicio`, el código dice `servicio.timeout_ms`, que comunica tanto el dato como su unidad. La claridad no es un adorno: reduce la posibilidad de intercambiar valores parecidos y hace más fácil leer código que escribiste hace semanas.

La creación de un struct usa llaves y pares `campo: valor`. El acceso también es directo mediante punto. Como los campos de la figura pertenecen a `Servicio`, no hay un estado temporal en el que exista una URL sin nombre o un límite de tiempo asociado accidentalmente a otro servicio. Sigue siendo posible crear un valor incorrecto, por ejemplo una URL sin esquema; la lección 4 enseñará cómo validarlo. Lo que desaparece es el desorden de variables sueltas.

**Fig. 3.1** | Un struct con sus métodos.

```rust
// fig03_01.rs
#[derive(Debug, Clone)]          // el compilador te escribe esos comportamientos
struct Servicio {
    nombre: String,
    url: String,
    timeout_ms: u64,
}

impl Servicio {
    fn new(nombre: &str, url: &str) -> Self {     // no hay constructores: es convención
        Self { nombre: nombre.to_string(), url: url.to_string(), timeout_ms: 5000 }
    }
    fn etiqueta(&self) -> String {                // &self = presta, no consume
        format!("{} ({})", self.nombre, self.url)
    }
}

fn main() {
    let s = Servicio::new("catalogo", "http://localhost:8090/ok");
    println!("{}", s.etiqueta());
    println!("{:?}", s.clone());
    println!("timeout: {} ms", s.timeout_ms);
}
```

```bash
$ rustc --edition 2024 fig03_01.rs && ./fig03_01
catalogo (http://localhost:8090/ok)
Servicio { nombre: "catalogo", url: "http://localhost:8090/ok", timeout_ms: 5000 }
timeout: 5000 ms
```

`#[derive(Debug, Clone)]` pide al compilador implementar comportamientos conocidos para el tipo. `Debug` permite imprimir una representación útil con `{:?}`. No es un formato estable para usuarios finales: es una vista para desarrollo y diagnóstico. `Clone` permite solicitar una copia explícita con `.clone()`. En este ejemplo se usa solo para demostrar que la estructura puede imprimirse y después seguir disponible; no debes copiar valores por costumbre para apagar errores de ownership.

El `revisor` usa el mismo modelo, pero añade los atributos necesarios para leer servicios desde YAML. `pub` indica que otros módulos del crate pueden acceder a esos campos. `Deserialize` y `serde` aparecerán a fondo en las lecciones posteriores; por ahora observa que el núcleo sigue siendo el mismo: nombre, URL y límite de tiempo.

<!-- verificar:extracto:src/modelo.rs -->
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}
```

El atributo `#[serde(default = "timeout_por_omision")]` no cambia qué es un servicio. Describe una regla de entrada: si el YAML no declara `timeout_ms`, el programa usa cinco segundos. Es importante distinguir modelado de validación. El struct declara qué datos forman un servicio; las reglas sobre si una URL tiene esquema, si el tiempo es mayor que cero o si el nombre se repite se comprueban después, cuando el programa recibe una lista.

### `impl` y métodos: comportamiento que pertenece al tipo

Un `struct` guarda datos, pero el tipo también puede tener operaciones que hacen sentido para esos datos. Rust agrupa esas operaciones en un bloque `impl`. El nombre significa implementation: una implementación de comportamiento para un tipo. No hay una palabra reservada especial para constructores. Por convención, una función asociada llamada `new` crea un valor nuevo, pero sigue siendo una función normal dentro de `impl`.

La figura usa dos formas de llamar funciones dentro de un `impl`. `Servicio::new(...)` usa `::` porque `new` todavía no tiene una instancia sobre la cual trabajar. `s.etiqueta()` usa `.` porque `etiqueta` recibe un servicio concreto. Rust permite esta sintaxis de método cuando el primer parámetro se llama `self`, `&self` o `&mut self`.

`&self` significa “presta este valor para lectura”. El método puede mirar `nombre` y `url`, construir una cadena nueva y devolverla, pero no toma la propiedad de `s` ni modifica sus campos. Por eso, después de `s.etiqueta()`, todavía puedes imprimir `s`, leer `s.timeout_ms` o prestar el servicio a otra función. Es la aplicación directa de los préstamos de la lección 2 a una función que vive junto a su tipo.

`&mut self` significa “presta este valor para modificarlo”. Un método como `fn cambiar_timeout(&mut self, ms: u64)` necesitaría que quien llama declare una variable mutable y no permitiría otras referencias activas al mismo valor. El compilador aplica las mismas reglas que ya viste con `&mut String`: una sola referencia mutable a la vez, o varias referencias inmutables, pero no ambas clases simultáneamente.

`self` sin `&` consume el valor. Es una decisión deliberada y menos común. Resulta útil cuando el método transforma un valor en otro y el original ya no debe existir, por ejemplo una operación que convierta una configuración temporal en una estructura validada. No es una forma más rápida de escribir `&self`: cambia quién posee el valor. Si recibes un error de “valor movido” después de llamar a un método, revisa primero su receptor.

En el proyecto real, `Servicio::new` concentra el valor por omisión. Eso evita que cada llamada tenga que repetir `5000` y reduce el riesgo de que algunos servicios se creen con una regla distinta sin querer.

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Servicio {
    /// Un servicio con el tiempo límite por omisión (5 segundos).
    pub fn new(nombre: &str, url: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            url: url.to_string(),
            timeout_ms: TIMEOUT_POR_OMISION_MS,
        }
    }
}
```

`Self` dentro de `impl Servicio` significa `Servicio`. Usarlo evita repetir el nombre del tipo y conserva la intención si el tipo cambia de nombre durante una refactorización. La expresión `Self { ... }` construye el valor; `-> Self` declara el tipo que devuelve la función. La constante `TIMEOUT_POR_OMISION_MS` está fuera del extracto y evita que el número cinco mil quede repartido por el programa como un valor mágico.

### Enums con datos: alternativas válidas, no campos ambiguos

Un `enum` describe un valor que puede tomar una de varias variantes. La diferencia esencial con una colección de constantes es que cada variante puede tener su propia forma. `Estado::Ok` y `Estado::Lento` tienen campos nombrados `codigo` y `ms`. `Estado::Falla` guarda una cadena. `Estado::NoIntentado` no carga datos porque no ocurrió una consulta que produzca resultados honestos.

Este diseño evita representar una falla como un código especial, como `0`, `-1` o una cadena vacía. Esos marcadores obligan a recordar reglas fuera del tipo: “si código es cero, lee error; si error está vacío, quizá fue exitoso; si tiempo es cero, quizá no se intentó”. Un enum mueve esas reglas al compilador. Si tienes `Estado::Falla`, Rust sabe que hay un mensaje; si tienes `Estado::Ok`, Rust sabe que hay código y duración.

También evita la combinación imposible de campos opcionales. Un struct como `Resultado { codigo: Option<u16>, error: Option<String>, ms: u64 }` admite por construcción tanto `codigo: Some(200), error: Some("no responde")` como `codigo: None, error: None`. Puede haber casos legítimos para una forma así, especialmente al serializar datos externos, pero no es una buena representación interna de alternativas mutuamente excluyentes. Para el estado de una consulta, el enum expresa mejor la realidad.

<!-- verificar:fragmento -->
```rust
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}
```

La sintaxis tiene tres formas que conviene reconocer. Las variantes con llaves son parecidas a structs pequeños y permiten nombrar campos al crear y desestructurar. Las variantes con paréntesis son parecidas a tuplas y sirven cuando el dato tiene un significado principal claro, como el mensaje de una falla en este fragmento. Las variantes sin datos representan una posibilidad que no necesita información adicional.

**Fig. 3.2** | Un enum con datos y su `match`.

```rust
// fig03_02.rs
#[allow(dead_code)]              // este ejemplo no lee el código de los lentos
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),                      // lleva el mensaje dentro
    NoIntentado,
}

fn main() {
    let estados = [
        Estado::Ok { codigo: 200, ms: 120 },
        Estado::Lento { codigo: 200, ms: 1800 },
        Estado::Falla("no responde".to_string()),
        Estado::NoIntentado,
    ];

    for estado in estados {
        let texto = match estado {
            Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
            Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
            Estado::Falla(msg)           => format!("FALLA: {msg}"),
            Estado::NoIntentado          => "sin revisar".to_string(),
        };
        println!("{texto}");
    }
}
```

```bash
$ rustc --edition 2024 fig03_02.rs && ./fig03_02
OK 200 en 120ms
LENTO 1800ms
FALLA: no responde
sin revisar
```

La anotación `#[allow(dead_code)]` pertenece al ejemplo, no es una receta para ocultar avisos en proyectos reales. La variante `Lento` conserva `codigo` porque una respuesta lenta puede haber sido HTTP 200, aunque este programa solo usa `ms`. Sin la anotación, Rust advertiría que el campo `codigo` de esa variante no se lee en este archivo. El proyecto real sí utiliza los datos donde corresponden y se compila con avisos tratados como errores.

El `revisor` mejora el fragmento inicial con dos decisiones de dominio. Primero, una falla lleva tanto `motivo` como `ms`, porque saber que una conexión agotó el tiempo después de cierta duración es información útil para el reporte. Segundo, las variantes reciben `derive(Debug, Clone, PartialEq)`. `PartialEq` permite comparar estados en pruebas con `assert_eq!`, algo que usarás en la lección 6.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Lo que se supo de un servicio después de consultarlo.
///
/// Cada variante lleva sus propios datos: así `Falla` no tiene código HTTP que
/// alguien pueda leer por error, y `Ok` no tiene mensaje de error.
#[derive(Debug, Clone, PartialEq)]
pub enum Estado {
    /// Contestó con 2xx a tiempo.
    Ok { codigo: u16, ms: u64 },
    /// Contestó con 2xx, pero tardó más de [`UMBRAL_LENTO_MS`].
    Lento { codigo: u16, ms: u64 },
    /// No contestó, contestó con error, o se acabó el tiempo.
    Falla { motivo: String, ms: u64 },
    /// Nunca se llegó a consultar.
    NoIntentado,
}
```

El enum no sustituye todo uso de booleanos ni todo uso de structs. Un booleano sigue siendo correcto para una pregunta con dos respuestas simples, como “¿el servicio está sano?”. Un struct sigue siendo correcto para datos que existen juntos al mismo tiempo, como nombre, URL y límite. Un enum es adecuado cuando las posibilidades tienen formas distintas y el programa debe tratarlas de manera distinta.

### `match`: decidir sobre cada estado sin dejar huecos

`match` compara un valor contra patrones y produce un resultado. En la figura, cada brazo tiene la forma `patrón => expresión`. El patrón identifica una variante y puede extraer sus datos. En `Estado::Ok { codigo, ms }`, los nombres dentro de las llaves crean variables locales llamadas `codigo` y `ms`. En `Estado::Falla(msg)`, `msg` recibe la cadena que lleva la variante.

El patrón `..` significa “ignora los demás campos”. En el brazo de `Lento`, el programa necesita la duración para imprimirla, pero no necesita el código. Es preferible a inventar un nombre como `_codigo` cuando no vas a usarlo: comunica que el dato existe y que esta decisión no depende de él. Si no necesitas ningún campo de una variante con datos, puedes escribir `Estado::Ok { .. }`.

Un `match` es una expresión. Por eso la figura puede hacer `let texto = match estado { ... };`. Cada brazo devuelve un `String`: tres usan `format!` y el último construye uno con `"sin revisar".to_string()`. Rust exige que todas las ramas produzcan tipos compatibles. Esa regla evita que una ruta devuelva texto y otra, accidentalmente, no devuelva nada.

La propiedad más valiosa es la exhaustividad. El compilador conoce todas las variantes de `Estado` porque están declaradas en el mismo tipo. Si un `match` no cubre una de ellas, no compila. Esto es más seguro que un `switch` que permite caer sin acción, y también es más explícito que una cadena de `if` que deja un caso como posibilidad implícita.

En algunos casos usarás un patrón comodín, `_ => ...`, para agrupar posibilidades que realmente deben recibir el mismo tratamiento. Es válido, pero tiene un costo: si agregas una variante nueva, ese brazo ya la aceptará sin obligarte a pensar si el comportamiento correcto es el mismo. Para un enum central como `Estado`, conviene preferir brazos explícitos en el reporte. Así una nueva variante se convierte en una decisión visible, no en un comportamiento accidental.

**Fig. 3.3** | Si olvidas una variante, no compila.

```rust
// fig03_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}

fn main() {
    let estado = Estado::NoIntentado;
    let texto = match estado {
        Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
        Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
        Estado::Falla(msg)           => format!("FALLA: {msg}"),
    };
    println!("{texto}");
}
```

```bash
$ rustc --edition 2024 fig03_03.rs
error[E0004]: non-exhaustive patterns: `Estado::NoIntentado` not covered
  --> fig03_03.rs:11:23
   |
11 |     let texto = match estado {
   |                       ^^^^^^ pattern `Estado::NoIntentado` not covered
   |
note: `Estado` defined here
  --> fig03_03.rs:2:6
   |
 2 | enum Estado {
   |      ^^^^^^
...
 6 |     NoIntentado,
   |     ----------- not covered
   = note: the matched value is of type `Estado`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
14 ~         Estado::Falla(msg)           => format!("FALLA: {msg}"),
15 ~         Estado::NoIntentado => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

En el `revisor`, `match` aparece en el módulo de reportes para convertir un estado técnico en una etiqueta que una persona pueda leer. Observa que cada variante se nombra de forma explícita. El código no presupone que “todo lo que no sea OK” es una falla: `Lento` y `NoIntentado` tienen significado propio.

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn etiqueta(e: &Estado) -> &'static str {
    match e {
        Estado::Ok { .. } => "OK",
        Estado::Lento { .. } => "LENTO",
        Estado::Falla { .. } => "FALLA",
        Estado::NoIntentado => "NO",
    }
}
```

El parámetro es `&Estado`, una referencia inmutable. El reporte debe leer el estado varias veces para obtener etiqueta, duración y detalle, así que no conviene consumirlo. Rust permite hacer `match` sobre una referencia: los patrones leen los campos que necesitan sin mover el `Estado` original. Esta combinación de préstamos y patrones será muy frecuente en código Rust.

También existe `matches!`, una macro útil cuando solo quieres una respuesta booleana. El método `esta_bien` del proyecto no necesita producir un texto ni extraer códigos; pregunta si el valor es una de dos variantes sanas. Agrupar patrones con `|` expresa esa regla sin repetir lógica.

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

No uses `matches!` para sustituir un `match` que debe transformar datos. Su resultado siempre es `bool`; es una pregunta, no una decisión completa. Cuando el programa necesita construir un reporte, obtener duración o elegir un detalle específico, `match` sigue siendo la herramienta adecuada.

### `Option<T>`: la ausencia declarada en el tipo

Rust no tiene `null` ni `nil`. Un valor de tipo `String` siempre es una cadena válida; un valor de tipo `&Servicio` siempre es una referencia válida mientras el préstamo sea válido. Cuando un dato puede faltar, su tipo debe declararlo. La forma estándar es `Option<T>`.

<!-- verificar:fragmento -->
```rust
enum Option<T> {
    Some(T),
    None,
}
```

La definición real pertenece a la biblioteca estándar y tiene más atributos internos, pero este fragmento muestra su idea central. `Option<u16>` significa “puede haber un código `u16`, o puede no haberlo”. `Option<Servicio>` significa “una búsqueda puede devolver un servicio o no encontrar ninguno”. El tipo no decide qué hacer ante la ausencia; obliga a quien consume el valor a hacerlo de forma explícita.

La ausencia no siempre es un error. Buscar un servicio por nombre puede no encontrarlo porque el nombre no está configurado. Un encabezado HTTP puede ser opcional. Una falla de red puede no producir código HTTP. En esos casos, `Option` comunica que la falta de valor es una posibilidad prevista por el contrato, no un valor secreto como `0`, `""` o un puntero nulo que podría explotar más adelante.

**Fig. 3.4** | Consumir un `Option`.

```rust
// fig03_04.rs
fn main() {
    let quizas: Option<u16> = Some(200);

    match quizas {
        Some(c) => println!("código {c}"),
        None    => println!("sin respuesta"),
    }

    if let Some(c) = quizas { println!("código {c}"); }     // cuando solo importa un caso
    let c = quizas.unwrap_or(0);                            // valor por omisión
    println!("{c}");
}
```

```bash
$ rustc --edition 2024 fig03_04.rs && ./fig03_04
código 200
código 200
200
```

El primer consumo usa `match` porque los dos casos importan: hay una salida para `Some` y otra para `None`. El segundo usa `if let` porque solo hay trabajo que hacer cuando existe un código; si no existe, el programa no necesita hacer nada. `if let Some(c) = quizas` es una forma breve de escribir un `match` cuyo otro brazo sería `_ => {}`.

`unwrap_or(0)` devuelve el contenido cuando existe y el valor por omisión cuando no. La decisión de usar `0` solo es correcta si el llamador entiende que cero representa “sin respuesta” en ese contexto. En un reporte HTTP público, puede ser más claro conservar `Option<u16>` hasta el punto donde se presenta el dato, para no confundir una ausencia con un código HTTP real.

No confundas `unwrap_or` con `unwrap`. `unwrap()` dice: “sé que aquí hay un valor; si no lo hay, termina el programa con un `panic!`”. Puede ser razonable en una prueba donde la ausencia demuestra que falló la preparación del caso, pero en código de aplicación suele ocultar una decisión pendiente. `clippy` suele señalar usos cuestionables de `unwrap`; la lección 6 mostrará cómo ejecutar esas comprobaciones. Antes de escribirlo, pregúntate si `None` puede ocurrir en producción. Si puede, necesitas manejarlo.

El proyecto usa `Option` para la forma JSON del reporte. Una falla no tiene un código HTTP inventado, por eso `codigo` es `Option<u16>`. El campo `error` también es opcional: aparece en una falla o en un servicio no intentado, pero se omite para una respuesta sana. Este struct representa una salida serializable, no reemplaza el enum interno `Estado`; ambos tipos tienen responsabilidades distintas.

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

Esta separación es útil. `Estado` modela alternativas exclusivas para que la lógica interna sea segura. `EstadoJson` modela la forma que una herramienta externa espera leer, donde algunos campos pueden ser `null` u omitirse. Rust no evita que una API externa tenga valores opcionales; evita que tu lógica interna los trate como si siempre existieran.

## El error que vas a ver

### `E0004`: un `match` no cubre todos los patrones

La figura 3.3 produce `E0004`, “non-exhaustive patterns”. El compilador vio que `estado` es de tipo `Estado`, leyó la definición del enum y comprobó que existe la variante `NoIntentado`. Después recorrió los brazos del `match` y no encontró ningún patrón que la cubriera.

La flecha bajo `estado` indica el valor sobre el que se está haciendo la decisión. La nota posterior muestra dónde se definió `Estado` y subraya la variante que falta. La ayuda propone dos arreglos: agregar un brazo explícito para `Estado::NoIntentado` o agregar un patrón comodín. Para este caso, el arreglo correcto es explícito porque “sin revisar” merece una salida visible:

<!-- verificar:fragmento -->
```rust
Estado::NoIntentado => "sin revisar".to_string(),
```

No copies `todo!()` de la sugerencia como solución final. Rust lo propone porque completa el patrón y deja un marcador visible para que decidas qué hacer; si esa rama se ejecuta, `todo!()` termina el programa con un `panic!`. Es útil durante una refactorización breve, no como comportamiento del `revisor`.

Este error aparece también cuando agregas una variante nueva. Esa es precisamente una de sus ventajas. En vez de depender de una búsqueda manual por el repositorio, deja que el tipo y el compilador enumeren los lugares que deben decidir cómo responder al nuevo estado. Corrige cada uno con una regla de negocio, no con `_ =>` por reflejo.

### Leer el error como una guía de cambio

Los errores de Rust suelen traer cuatro partes: el código estable como `E0004`, la ubicación, notas con contexto y una ayuda. Empieza por el código y la frase principal; en este caso bastan para saber que falta una variante. Después lee la nota para confirmar el tipo y la ayuda para conocer una forma sintáctica válida de corregirlo.

La ayuda del compilador no conoce tu dominio. Puede decirte cómo completar un `match`, pero no puede decidir si un estado nuevo debe contar como sano, fallido, lento o no revisado. Esa decisión sigue siendo tuya. La ventaja es que Rust separa ambos problemas: te garantiza que no olvidaste tratar el caso y te deja definir el tratamiento correcto.

Puedes pedir una explicación ampliada con `rustc --explain E0004`. Hazlo al encontrar un código de error que no entiendas. No hace falta memorizar códigos; importa aprender a reconocer que son identificadores consultables y que el mensaje contiene evidencia concreta sobre el tipo y la línea involucrada.

## Lo que se hace mal

### Modelar alternativas con un struct lleno de campos opcionales

Un struct como `Resultado { codigo: Option<u16>, error: Option<String>, ms: Option<u64> }` parece flexible, pero acepta demasiados estados incoherentes. Puede contener a la vez código exitoso y mensaje de falla, o no contener ninguno de los dos. Si esos casos son inválidos, el tipo debería hacerlos difíciles o imposibles de construir.

Usa un enum cuando las posibilidades se excluyen entre sí y llevan datos diferentes. Conserva structs con `Option` para límites externos, como JSON, formularios o configuraciones parciales, donde realmente necesitas representar campos que pueden faltar de manera independiente.

### Usar `_ =>` para silenciar la exhaustividad

El patrón comodín es correcto cuando todas las variantes restantes reciben exactamente el mismo trato. El problema aparece cuando se usa solo para hacer que el compilador deje de protestar. En un enum de negocio, `_` puede convertir una variante nueva en una falla genérica o, peor, en una respuesta sana por accidente.

En `Estado`, escribe los cuatro brazos de forma explícita. Si agregas una variante, acepta que el compilador te obligue a revisar el reporte y las pruebas. El pequeño trabajo inmediato evita un comportamiento no revisado más adelante.

### Escribir `unwrap()` en una ruta normal del programa

`unwrap()` no resuelve la ausencia: la convierte en un `panic!`. Si un servicio puede no encontrarse, una respuesta puede no traer código o un archivo puede no existir, la ausencia forma parte de la realidad del programa. Debe convertirse en un `match`, un `if let`, un valor por omisión justificado o, en la lección 4, un `Result`.

En pruebas, `unwrap()` puede ser útil para declarar que un caso debe prepararse correctamente. En lógica de producción, úsalo solo cuando hayas demostrado que `None` es imposible y el fallo representa un error de programación, no una condición esperable.

### Copiar con `clone()` para evitar pensar en ownership

`Clone` no es una salida automática ante un error de movimiento. Copiar un `Servicio` solo para prestarlo a una función duplica sus `String` y puede ocultar que la función debería recibir `&Servicio`. En la figura 3.1, `clone()` existe para demostrar el trait y hacer visible la estructura; no es la forma recomendada de pasar servicios por el programa.

Prefiere préstamos para leer (`&Servicio`), préstamos mutables cuando haya una modificación real (`&mut Servicio`) y movimiento cuando la función debe tomar propiedad del valor. Copia únicamente cuando el programa necesita de verdad dos valores independientes.

### Usar números o cadenas mágicas para representar estados

Representar una falla con `codigo == 0`, una consulta pendiente con `ms == 0` o un error con `mensaje == ""` obliga a recordar convenciones que el tipo no expresa. También dificulta responder preguntas simples: ¿puede una respuesta real tardar cero milisegundos? ¿un mensaje vacío es una falla o la ausencia de falla?

Nombra el estado con una variante. `Estado::NoIntentado` comunica más que un número especial y permite que `match` obligue a manejarlo. Cuando el estado tiene datos, ponlos dentro de la variante que los hace válidos.

### Confundir `Option` con `Result`

`Option<T>` responde “¿hay valor o no?”. `Result<T, E>` responde “¿hubo valor o hubo un error que necesito conocer?”. Una búsqueda que no encuentra un nombre puede devolver `Option<Servicio>`; leer un archivo que no existe normalmente debe devolver `Result<String, Error>`, porque quien llama necesita saber qué salió mal. La lección 4 profundiza en `Result` y `?`.

No inventes mensajes de error dentro de `Option` ni uses `None` para esconder un fallo que el usuario necesita diagnosticar. Elige el tipo según el contrato de la operación.

## Ejercicios

### Ejercicio 1 — Describe un servicio

Crea un `struct ServicioLocal` con `nombre: String`, `url: String` y `timeout_ms: u64`. Escribe una función asociada `new(nombre: &str, url: &str) -> Self` que asigne `3000` como tiempo por omisión. Agrega un método `etiqueta(&self) -> String` que devuelva `nombre (url)`.

En `main`, crea un servicio llamado `pagos`, imprime la etiqueta y después imprime el tiempo límite. Comprueba que no necesitas `mut` ni `clone()` para estas operaciones.

### Ejercicio 2 — Resume todos los estados

Declara un enum `EstadoLocal` con las variantes `Ok { codigo: u16, ms: u64 }`, `Lento { codigo: u16, ms: u64 }`, `Falla(String)` y `NoIntentado`. Escribe `fn resumen(estado: &EstadoLocal) -> String` usando un `match` exhaustivo.

El resumen debe usar exactamente estas formas: `OK 200 en 80ms`, `LENTO 1200ms`, `FALLA: sin conexión` y `sin revisar`. Pruébalo con una instancia de cada variante.

### Ejercicio 3 — Agrega una variante y deja que Rust encuentre el trabajo

Añade `Rechazado { codigo: u16, ms: u64 }` a `EstadoLocal`. Compila sin modificar `resumen` y observa `E0004`. Después agrega el brazo que produzca `RECHAZADO 403 en 15ms`.

No uses `_ =>`. El objetivo es comprobar que el compilador señala una decisión de negocio pendiente. Explica con una frase por qué `Rechazado` no debe clasificarse automáticamente como `Falla`: el servidor sí respondió, pero la respuesta no fue aceptada.

### Ejercicio 4 — Busca sin usar `nil`

Crea un arreglo o vector de dos `ServicioLocal`: `catalogo` y `pagos`. Escribe una función que reciba una rebanada de servicios y un nombre, y devuelva `Option<&ServicioLocal>`. Busca primero `pagos` y luego `reportes`.

Consume el primer resultado con `if let` para imprimir su URL. Consume el segundo con `match` para imprimir `no existe reportes`. No uses índices con un valor centinela, referencias nulas ni `unwrap()`.

## Soluciones

### Solución 1

`ServicioLocal` debe ser un struct con tres campos nombrados. La función `new` debe usar `Self` y convertir `nombre` y `url` de `&str` a `String`; el método `etiqueta` debe recibir `&self`, pues solo lee los campos. Una salida correcta contiene:

```text
pagos (http://localhost:8091/ok)
timeout: 3000 ms
```

Si necesitas declarar `let mut servicio`, revisa el ejercicio: ninguna operación solicitada modifica el valor. Si necesitas `clone()`, probablemente cambiaste una firma para recibir `self` cuando debía recibir `&self`.

### Solución 2

`resumen` debe recibir `&EstadoLocal` para leer el estado sin consumirlo. Debe tener cuatro brazos explícitos. El brazo de `Ok` extrae `codigo` y `ms`; el de `Lento` puede usar `ms` e ignorar el código con `..`; el de `Falla` extrae el mensaje; el de `NoIntentado` devuelve el texto fijo.

Las cuatro llamadas deben producir estas líneas:

```text
OK 200 en 80ms
LENTO 1200ms
FALLA: sin conexión
sin revisar
```

Si una rama devuelve `&str` y las demás devuelven `String`, haz que todas produzcan el mismo tipo. `format!` devuelve `String`; para una etiqueta fija puedes usar `.to_string()`.

### Solución 3

Al añadir `Rechazado`, la compilación debe fallar con `E0004` hasta que agregues un brazo explícito. El brazo correcto extrae los dos campos y produce:

```text
RECHAZADO 403 en 15ms
```

La solución no consiste en cambiar el último brazo por `_ => "FALLA"`. Esa forma haría compilar el programa, pero perdería la diferencia entre una red caída y un servidor que respondió con una política de autorización. El enum ofrece una posibilidad nueva; el `match` debe convertirla en una decisión explícita.

### Solución 4

La función de búsqueda debe devolver `Option<&ServicioLocal>`, no `Option<ServicioLocal>`. La referencia permite prestar el servicio encontrado desde la lista sin copiar sus cadenas ni moverlo fuera del vector. Una búsqueda puede recorrer los servicios y devolver el primero cuyo `nombre` coincida; si no encuentra ninguno, devuelve `None`.

Para `pagos`, `if let Some(servicio)` debe imprimir su URL. Para `reportes`, un `match` debe incluir ambos brazos y producir:

```text
no existe reportes
```

Si el compilador se queja por lifetimes, revisa la firma: la referencia de salida debe provenir de la rebanada de entrada. En la mayoría de estos casos, Rust puede inferir el lifetime correcto sin que lo escribas. La lección 5 explicará los casos donde sí debes declararlo.

## Cómo sé que lo logré

- [ ] Compilo la figura 3.1 con `rustc --edition 2024 fig03_01.rs && ./fig03_01` y obtengo las tres líneas documentadas.
- [ ] Compilo la figura 3.2 y puedo explicar por qué cada variante de `Estado` lleva datos distintos.
- [ ] Compilo la figura 3.3, veo `error[E0004]` y la hago compilar agregando un brazo explícito para `NoIntentado`.
- [ ] Puedo agregar una variante a mi enum y localizar cada decisión pendiente mediante los errores de `match`.
- [ ] Mi ejercicio 4 devuelve `Option<&ServicioLocal>` y maneja tanto `Some` como `None` sin `unwrap()`.
- [ ] Puedo explicar por qué el `revisor` usa un enum para `Estado` y un struct con `Option` para `EstadoJson`.
- [ ] Completé los ejercicios de Rustlings `structs`, `enums` y `options`.

## Para leer más

- [The Rust Programming Language, capítulo 5: Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html) — consultado el 2 de octubre de 2026.
- [The Rust Programming Language, capítulo 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) — consultado el 2 de octubre de 2026.
- [Documentación oficial de `std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html) — consultado el 2 de octubre de 2026.
- [Rustlings: ejercicios de structs, enums y options](https://github.com/rust-lang/rustlings/tree/main/exercises) — consultado el 2 de octubre de 2026.
