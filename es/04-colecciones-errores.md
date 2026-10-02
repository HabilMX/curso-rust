# Lección 4 — Colecciones y errores

**Tiempo:** 2 × 45 min.

**Qué construyes:** la lista de servicios y el reporte, con errores de verdad

**Qué aprendes:** `Vec`, `HashMap`, `String` contra `&str`, `Result` y `?`, `panic!` contra `Result`, `anyhow` y `thiserror`

**The Rust Book, capítulos 8 y 9.** Rustlings: `vecs`, `hashmaps`, `strings`, `error_handling`.

## Al terminar vas a poder

- Guardar una lista de `Servicio` en un `Vec<Servicio>` y recorrerla sin pelearte con ownership.
- Elegir entre indexar una colección y usar métodos que devuelven `Option`.
- Guardar estados por nombre en un `HashMap<String, Estado>` y producir un reporte ordenado.
- Explicar por qué una función normalmente recibe `&str`, mientras que un struct normalmente guarda `String`.
- Propagar errores de archivos y de validación con `Result` y `?`.
- Distinguir un error que quien usa el programa puede corregir de un invariante roto que justifica `panic!`.
- Agregar contexto a un error de aplicación con `anyhow` y reconocer cuándo una biblioteca necesita `thiserror`.

## El porqué antes del cómo

En la lección 3 modelaste un servicio y los distintos resultados de revisarlo. Ese modelo todavía necesita vivir en algún lado. El programa recibe muchos servicios, no uno solo; debe conservarlos mientras consulta cada URL, asociar cada resultado con su servicio y después imprimir un reporte. En un programa pequeño puedes escribir dos o tres variables manualmente. En el `revisor` real, esa estrategia deja de servir desde el momento en que el archivo YAML trae una cantidad variable de servicios.

Las colecciones resuelven la parte de “cuántos valores hay”, pero no resuelven por sí solas qué significa que falte un valor o qué debe hacer el programa cuando algo externo falla. Un archivo puede no existir, una línea puede tener formato inválido, el nombre de un servicio puede repetirse y una URL puede no traer esquema. Ninguno de esos casos es raro ni imposible: todos ocurren porque el programa recibe datos del exterior. La diferencia importante es que Rust te pide representar esa posibilidad en el tipo de retorno.

Un `Vec<Servicio>` representa una lista ordenada de servicios. Un `HashMap<String, Estado>` representa una asociación por llave: dado el nombre `"catalogo"`, busca su estado. Un `String` es un texto que posee su memoria; un `&str` es una vista prestada de texto que alguien más posee. Y un `Result<T, E>` representa una operación que puede terminar con un valor `T` o con un error `E`. No son cuatro temas inconexos: son las piezas que hacen que el estado del programa tenga forma explícita.

El curso de Go construye el mismo revisor. En Go, leer una llave inexistente de un `map` devuelve el valor cero y obliga a recordar la forma de dos resultados para distinguir “no existe” de “existe y vale cero”. Rust elige otro contrato: `HashMap::get` devuelve `Option<&V>`. La ausencia aparece en el tipo y no puede confundirse con un estado real. El costo es que tienes que decidir qué hacer con `None`; la ganancia es que esa decisión no se puede olvidar sin que el código lo haga visible.

Con los errores ocurre algo parecido. Go usa la convención `if err != nil` después de cada operación que puede fallar. Rust usa `Result` y permite escribir la propagación con `?`. No hay una respuesta universal sobre cuál estilo es más legible: Go repite una estructura muy explícita; Rust concentra la misma decisión en un operador. Lo importante es que ambos obligan a atender el error. Rust no convierte un archivo ausente en una cadena vacía ni deja que una conversión fallida siga como si fuera válida.

Esta lección no busca que uses `unwrap()` para hacer que el compilador se calle. Busca que leas la firma de cada función como contrato. Si una función devuelve `Option`, debes pensar qué significa la ausencia. Si devuelve `Result`, debes decidir si el error se resuelve ahí, se transforma o se propaga. Si recibe `&str`, solo necesita leer texto; si recibe `String`, probablemente pretende quedarse con él. Las firmas describen el flujo de datos y el flujo de fallas antes de que el programa se ejecute.

## Los conceptos

### `Vec<T>`: una lista dueño de valores del mismo tipo

`Vec<T>` es el vector de Rust: una colección de tamaño variable que posee sus elementos. El parámetro `T` dice qué tipo de valores puede guardar. Un `Vec<Servicio>` solo guarda servicios; un `Vec<Estado>` solo guarda estados. Esta restricción no es una incomodidad accidental. Le permite al compilador saber cómo debe administrar cada elemento, qué métodos son válidos y qué operaciones podrían mover o prestar valores.

Un vector vacío necesita una anotación de tipo si Rust no puede inferirlo. Por eso la figura escribe `let mut v: Vec<Servicio> = Vec::new();`. El compilador aún no ha visto ningún elemento y no puede adivinar qué habrá dentro. Si creas el vector con `vec![...]`, o si el contexto ya determina el tipo, normalmente no necesitas escribirlo. La palabra `mut` es necesaria porque `push` cambia la colección: agrega un elemento y puede hacer que el vector reserve más espacio.

El vector es dueño de cada `Servicio` que recibe. En `v.push(s)`, la variable `s` se mueve al vector. Esto aplica las reglas de ownership de la lección 2: después de mover un `Servicio`, no puedes seguir usando la variable anterior como si todavía lo poseyera. No es una copia implícita. Si necesitas conservar otra versión independiente, debes diseñar la operación para prestar, o clonar de manera deliberada cuando el costo y la semántica lo justifiquen.

Hay dos formas de leer un elemento. `&v[0]` produce una referencia y supone que el índice existe. Si no existe, el programa entra en `panic!`. `v.get(0)` devuelve `Option<&Servicio>`: `Some(referencia)` si existe y `None` si está fuera del rango. La segunda forma es adecuada cuando el índice viene de un archivo, un argumento, una petición o cualquier dato que no controlas por completo. La primera es razonable cuando romper el programa revela un error de programación que ya debió evitarse mediante una validación anterior.

**Fig. 4.1** | Las colecciones y su acceso seguro.

```rust
// fig04_01.rs
use std::collections::HashMap;

struct Servicio {
    nombre: String,
}

#[derive(Debug)]
enum Estado {
    Ok,
    Falla,
}

fn main() {
    let s = Servicio { nombre: "catalogo".to_string() };
    let mut v: Vec<Servicio> = Vec::new();
    v.push(s);
    let primero = &v[0];                    // si no existe: panic
    println!("{}", primero.nombre);
    let primero = v.get(0);                 // devuelve Option<&Servicio>
    println!("{}", primero.is_some());

    let mut m: HashMap<String, Estado> = HashMap::new();
    m.insert("catalogo".to_string(), Estado::Ok);
    println!("{:?}", m.get("catalogo"));
    println!("{:?}", m.get("pagos"));       // Option<&Estado>: no hay valor cero silencioso
    println!("{:?}", Estado::Falla);

    let mut conteo: HashMap<String, u32> = HashMap::new();
    *conteo.entry("reportes".into()).or_insert(0) += 1;   // el patrón para contar
    *conteo.entry("reportes".into()).or_insert(0) += 1;
    println!("{:?}", conteo.get("reportes"));
}
```

```bash
$ rustc --edition 2024 fig04_01.rs && ./fig04_01
catalogo
true
Some(Ok)
None
Falla
Some(2)
```

No uses índices para recorrer un vector por costumbre. Cuando lo único que necesitas es visitar cada elemento, `for servicio in &servicios` expresa mejor la intención y evita cálculos de índice. Cuando necesitas el número de posición, usa `enumerate()`: `for (i, servicio) in servicios.iter().enumerate()`. El valor `i` queda asociado al elemento correcto y no existe el riesgo de escribir accidentalmente `i + 1` al leer.

También importa que una referencia a un elemento del vector es un préstamo del vector completo. Agregar elementos puede requerir mover todo el almacenamiento a otra zona de memoria. Por eso Rust no permite conservar `let primero = &v[0]`, llamar después a `v.push(...)` y volver a usar `primero`. La restricción evita referencias colgantes: direcciones que antes apuntaban a un elemento válido y ahora apuntarían a memoria liberada.

El `revisor` mantiene la lista de servicios como vector porque el archivo declara una secuencia y el reporte necesita conservar ese orden conceptual. Las funciones de reporte reciben slices prestados, `&[Servicio]` y `&[Estado]`, en vez de tomar los vectores. Un slice da acceso a una secuencia sin transferir la propiedad de la colección. Así `main` puede imprimir el reporte y luego inspeccionar los estados para elegir el código de salida.

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

El valor de `filas` es un `Vec<Fila<'a>>`: una lista nueva de pares de referencias. No clona los servicios ni los estados para poder ordenarlos; crea referencias a ambos. Esa diferencia importa en un programa que puede manejar listas grandes. Poseer datos nuevos cuesta memoria y trabajo de copia; prestar datos existentes conserva una fuente de verdad y hace visible que el reporte solo está observando.

### `HashMap<K, V>`: buscar por llave sin inventar ausencias

Un `HashMap<K, V>` guarda asociaciones entre una llave y un valor. Para el revisor, una llave natural sería el nombre del servicio y el valor sería su estado: `"catalogo" -> Estado::Ok { ... }`. Es una colección útil cuando sabes qué quieres buscar, pero no sabes en qué posición de una lista está. Buscar linealmente en un `Vec` implica revisar elementos hasta encontrar uno; buscar por llave en un mapa expresa directamente la pregunta.

La operación `insert` toma posesión de la llave y del valor. Por eso la figura construye `"catalogo".to_string()`: el mapa necesita poseer una `String` que sobreviva después de terminar la llamada. `get`, en cambio, presta. El tipo de `m.get("catalogo")` es `Option<&Estado>`, no `Estado`, porque la llave puede no existir y porque el mapa conserva la propiedad del estado.

Esta es una diferencia importante con Go. Un acceso como `m["pagos"]` en un `map[string]Estado` de Go entrega el valor cero si no existe la llave. Si `Estado` contiene números, ese cero puede parecer una respuesta real. En Rust, `None` comunica una ausencia que debes manejar. Puedes usar `match`, `if let Some(estado) = ...`, o métodos como `unwrap_or` cuando un valor por omisión sea verdaderamente correcto para el dominio.

El patrón `entry(...).or_insert(0)` evita hacer dos búsquedas cuando quieres actualizar un contador. `entry` representa la posición de una llave que puede estar ocupada o vacante. `or_insert(0)` deja el valor existente o inserta cero y devuelve una referencia mutable al contador. El `*` desreferencia esa referencia mutable para poder aplicar `+= 1`. No es sintaxis decorativa: Rust separa con precisión el valor guardado del préstamo que permite modificarlo.

Un mapa no promete un orden de recorrido. El orden interno depende de cómo se distribuyen las llaves y puede cambiar al insertar, borrar, cambiar de ejecución o usar otra versión de la biblioteca. Nunca construyas una salida pública recorriendo un `HashMap` y esperando que salga alfabética por casualidad. Para un reporte reproducible, extrae las llaves, ordénalas y recórrelas en ese orden, o usa una estructura ordenada cuando ésa sea la operación central.

El proyecto actual no usa un `HashMap` para su reporte. Usa dos vectores paralelos: servicios y estados, ambos en el mismo orden. Eso permite que un servicio conserve su posición desde el YAML hasta el resultado de la consulta. Al momento de imprimir, la función `ordenadas` forma pares prestados y los ordena por `nombre`. El concepto que comparte con un `HashMap` es decisivo: el almacenamiento puede tener el orden conveniente para trabajar, pero la salida pública debe imponer su propio orden de manera explícita.

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

El lifetime `'a` aparecerá a fondo en la lección 5. Por ahora basta leerlo como una garantía: cada par de `Fila` contiene referencias que no pueden vivir más que los slices de entrada. El vector `filas` es dueño de los pares, pero no es dueño de los servicios ni de los estados. Cuando termina `tabla`, desaparecen las referencias temporales; los vectores originales siguen siendo propiedad de `main`.

### `String` y `&str`: poseer texto contra leer una vista

Rust distingue el texto que posee datos del texto que solo presta una vista. `String` es una cadena UTF-8, mutable y con tamaño variable; normalmente vive en el heap y es dueño de sus bytes. `&str` es una referencia a una secuencia UTF-8 que ya existe en otro lugar. Un literal como `"catalogo"` tiene tipo `&'static str`: es una vista de texto almacenado dentro del binario y disponible durante toda la ejecución.

La regla práctica es sencilla: recibe `&str`, guarda `String`. Una función que solo va a leer un nombre no necesita recibir la propiedad ni obligar a la persona que llama a crear una copia. Un struct que debe conservar el nombre después de que termine la llamada sí necesita ser dueño de un `String`. Esta regla no es absoluta, pero evita dos errores comunes: aceptar `String` por reflejo y terminar moviendo valores innecesariamente, o intentar guardar una referencia a texto cuyo dueño desaparecerá.

**Fig. 4.2** | Recibe `&str`, acepta los dos.

```rust
// fig04_02.rs
fn saludar(n: &str) { println!("hola, {n}"); }

fn main() {
    let propio = String::from("catalogo");
    saludar("pagos");
    saludar(&propio);
}
```

```bash
$ rustc --edition 2024 fig04_02.rs && ./fig04_02
hola, pagos
hola, catalogo
```

La llamada `saludar(&propio)` funciona mediante coerción: una referencia a `String` puede utilizarse donde se espera `&str`. No copies la cadena ni escribas `propio.to_string()` para “hacer que cuadre”. La función solo pide lectura, así que prestar es la operación correcta. Además, `propio` continúa disponible después de la llamada.

<!-- verificar:fragmento -->
```rust
fn saludar(n: String) { }
```

Esa firma compila, pero comunica otra cosa: quien llama debe entregar la propiedad de un `String`. No acepta un literal sin conversión y no permite seguir usando el `String` original después de la llamada. Una API así podría ser correcta si la función va a guardar, transformar o devolver el texto como propietaria, pero es una mala elección para una función que solo imprime o compara.

`String` no admite indexación con enteros como si cada carácter ocupara un byte. Rust usa UTF-8; una letra visible puede ocupar varios bytes. Permitir `nombre[3]` sería ambiguo: ¿el cuarto byte, el cuarto valor Unicode o el cuarto grupo visible? Por eso debes decidir la unidad que necesitas. `s.as_bytes()` trabaja con bytes, `s.chars()` trabaja con valores `char`, y `s.get(rango)` devuelve `Option<&str>` cuando el rango puede caer a media codificación. Esta restricción evita partir una cadena UTF-8 en un punto inválido.

El `revisor` guarda `nombre` como `String` porque el valor viene del YAML y debe vivir dentro de cada `Servicio`. Cuando calcula el ancho de una columna, no cuenta bytes: usa `chars().count()`. No resuelve todos los detalles de ancho visual de Unicode, pero sí evita tratar un carácter multibyte como varios caracteres al contar.

<!-- verificar:extracto:src/reporte.rs -->
```rust
        .iter()
        .map(|(s, _)| s.nombre.chars().count())
        .max()
        .unwrap_or(0)
        .max("SERVICIO".len());
```

No conviertas todo a `String` “por si acaso”. Una conversión puede asignar memoria y, sobre todo, esconder quién debe poseer el texto. Empieza por la firma: si la función solo lee, `&str`; si el resultado debe sobrevivir de forma independiente, `String`. Si debes aceptar varios tipos que pueden verse como texto, conocerás `AsRef<str>` y traits genéricos en la lección 5, pero no los uses antes de que una API realmente lo necesite.

### `Result<T, E>` y `?`: hacer visible el camino de error

`Result<T, E>` es un enum de la biblioteca estándar con dos variantes: `Ok(T)` y `Err(E)`. Una función que devuelve `Result<String, std::io::Error>` promete una de dos cosas: devolverá texto leído correctamente, o devolverá el error de entrada/salida que impidió leerlo. No devuelve un texto vacío para señalar fracaso y no imprime un error dentro de una función que quizá se use desde otro lugar.

<!-- verificar:fragmento -->
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

El operador `?` opera sobre un `Result`. Si recibe `Ok(valor)`, extrae `valor` y la ejecución continúa. Si recibe `Err(error)`, termina la función actual con ese error, convirtiéndolo al tipo de error declarado cuando existe una conversión válida. No ignora el error y no lo vuelve un pánico. Es una forma compacta de escribir una decisión que sigue siendo obligatoria.

**Fig. 4.3** | El operador `?` devuelve el error al llamador.

```rust
// fig04_03.rs
fn leer_config(ruta: &str) -> Result<String, std::io::Error> {
    let contenido = std::fs::read_to_string(ruta)?;   // si falla, retorna el error
    Ok(contenido)
}

fn main() {
    match leer_config("servicios-que-no-existe.txt") {
        Ok(texto) => println!("{texto}"),
        Err(e) => println!("error: {e}"),
    }
}
```

```bash
$ rustc --edition 2024 fig04_03.rs && ./fig04_03
error: No such file or directory (os error 2)
```

La función `leer_config` no sabe si un archivo ausente es fatal para toda la aplicación, recuperable mediante otra ruta o esperado por una prueba. Por eso devuelve el error. `main` sí decide cómo comunicarlo: en esta figura lo imprime. En un binario de producción normalmente lo escribirías en la salida de error y terminarías con un código distinto de cero, para que una persona y una automatización puedan distinguir éxito de fracaso.

La comparación con Go es directa. Estas cuatro líneas de Rust:

<!-- verificar:fragmento -->
```rust
let a = paso1()?;
let b = paso2(a)?;
let c = paso3(b)?;
Ok(c)
```

expresan una cadena de operaciones que en Go suele escribirse con una comprobación `if err != nil` después de cada paso. Rust reduce la repetición, pero no reduce la responsabilidad. Cada `?` marca un lugar donde la función puede salir antes. Si más adelante el programa debe limpiar recursos, transformar un error o tomar una alternativa, debes decidirlo antes o después de ese punto.

El `revisor` usa `?` para leer el archivo y para deserializar YAML. Ambos fallos son parte de iniciar el programa: no hay lista válida de servicios sin archivo legible ni sin YAML válido. La función devuelve `anyhow::Result<Vec<Servicio>>`, que permite unificar errores de tipos distintos sin perder sus mensajes.

<!-- verificar:extracto:src/config.rs -->
```rust
use anyhow::{Context, Result};
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}
```

Una aclaración sobre el nombre: `yaml_serde::from_str` es el mismo `from_str` que ofrecía `serde_yaml`, el crate anterior, que ya no se mantiene. En la lección 6 verás por qué el `revisor` usa el primero.

`with_context` agrega información que el sistema operativo no conoce. El error original puede decir “No such file or directory”, pero el contexto aclara cuál archivo estaba intentando leer el revisor. Es una diferencia entre un diagnóstico técnicamente correcto y un diagnóstico accionable. El operador `?` conserva esa cadena de causas al devolver el error.

También observa que no todos los fracasos de una consulta son `Err`. La función `revisar` del proyecto devuelve `Estado`, incluso cuando un servicio no responde. Eso es correcto porque “un servicio falló” es un dato que el reporte debe mostrar, no una imposibilidad de continuar el programa. `Result` representa que el propio programa no pudo completar una operación necesaria; `Estado::Falla` representa un resultado normal del dominio del revisor. Elegir entre ambos depende de quién debe decidir qué hacer y de si el programa todavía puede producir un resultado útil.

### `panic!`: un alto para bugs, no un sustituto de `Result`

`panic!` termina el flujo normal de ejecución porque el programa encontró una condición que sus propios supuestos declaraban imposible. Indexar un vector fuera de rango provoca un pánico. Llamar `unwrap()` sobre `None` o sobre `Err` también. Estos mecanismos existen porque hay invariantes que, si se rompen, revelan un error de programación y no una situación que una persona usuaria deba reparar.

Un archivo ausente no es un invariante roto: puede faltar por una ruta mal escrita, permisos, un despliegue incompleto o una decisión de quien ejecuta el binario. Debe ser `Result`. Una respuesta HTTP 500 tampoco es una razón para entrar en pánico: es un estado que el revisor fue construido para reportar. Un índice calculado desde un archivo tampoco debe usarse con `[]` sin validar; usa `get` y devuelve un error que explique el problema.

**Fig. 4.4** | Un pánico atrapado para demostrar que no es un `Result`.

```rust
// fig04_04.rs
fn main() {
    let previo = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let resultado = std::panic::catch_unwind(|| {
        panic!("la lista validada no puede estar vacia");
    });

    std::panic::set_hook(previo);
    println!("hubo panico: {}", resultado.is_err());
}
```

```bash
$ rustc --edition 2024 fig04_04.rs && ./fig04_04
hubo panico: true
```

La figura atrapa el pánico solo para aislar la demostración. No es el patrón normal de una aplicación. `catch_unwind` no convierte los pánicos en control de flujo sano ni garantiza que una estructura quede en un estado apto para seguir usándose. En código cotidiano, si estás pensando en recuperar un `panic!` provocado por datos externos, casi siempre debes rediseñar la función para que devuelva `Result`.

`.unwrap()` y `.expect("mensaje")` son pánicos potenciales. `expect` es preferible cuando existe una razón concreta e invariante para creer que no fallará, porque su mensaje documenta esa razón. No escribas `expect("debe funcionar")`: no explica nada. Un mensaje útil nombra el supuesto, como “el semáforo nunca se cierra”, y deja claro qué habría que investigar si ocurre.

El proyecto usa `expect` para adquirir un permiso de un semáforo interno. No es un error provocado por el archivo YAML ni por una URL; sería una contradicción en la coordinación que el propio programa construyó. Por eso es uno de los pocos lugares donde el pánico tiene sentido.

<!-- verificar:extracto:src/revisar.rs -->
```rust
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
```

No copies este patrón para operaciones de entrada/salida. `File::open(ruta).expect(...)` convierte una ruta inexistente en la terminación del proceso y elimina la oportunidad de que `main` imprima el archivo, use otro valor o seleccione un código de salida apropiado. Primero pregunta si el caso puede ocurrir con datos válidos del exterior. Si la respuesta es sí, devuelve `Result`.

### `anyhow` y `thiserror`: dos papeles distintos para errores propios

La biblioteca estándar basta para muchos programas pequeños: puedes devolver `Result<T, std::io::Error>` cuando todo fallo relevante es de entrada/salida. Un programa real suele combinar varios tipos: `std::io::Error`, un error de YAML, una URL inválida, un argumento de línea de comandos o una regla de validación. Si cada capa necesita conocer todos esos tipos concretos, las firmas se vuelven difíciles de mantener.

`anyhow` resuelve bien el borde de una aplicación. Su `Result<T>` es una forma abreviada de devolver un error dinámico que puede contener diferentes causas y contexto adicional. El revisor es un binario: lee configuración, inicia consultas y presenta mensajes a una persona. En esa frontera, la prioridad es explicar qué operación falló y conservar la cadena de causas. Por eso `config::cargar` usa `anyhow::{Context, Result}`.

No significa que `anyhow` sea una licencia para borrar significado. Si una función devuelve un estado que otra parte del programa debe distinguir para tomar decisiones, un enum propio puede ser mejor. Por ejemplo, una biblioteca que necesite permitir que quien llama diferencie `NombreRepetido`, `UrlSinEsquema` y `TimeoutCero` no debe entregar solamente una cadena. Debe publicar un tipo de error con variantes que representen esas causas.

`thiserror` ayuda a declarar ese tipo de error propio sin escribir manualmente implementaciones repetitivas de `Display`, `Error` y conversiones desde errores internos. Se usa sobre todo en bibliotecas, donde el tipo de error forma parte de la API pública. El `Cargo.lock` del proyecto contiene `thiserror`, pero el `Cargo.toml` del revisor no lo declara como dependencia directa y su código actual no expone un enum de errores propio. Por eso el `revisor` no lo usa: sus errores son mensajes de texto que `anyhow` acompaña con contexto.

<!-- verificar:fragmento -->
```rust
#[derive(Debug, thiserror::Error)]
enum ErrorConfiguracion {
    #[error("el nombre «{0}» está repetido")]
    NombreRepetido(String),
    #[error("la URL «{0}» no tiene esquema HTTP")]
    UrlSinEsquema(String),
    #[error("no se pudo leer la configuración")]
    Lectura(#[from] std::io::Error),
}
```

Este fragmento ilustra una API de biblioteca, no es parte del revisor actual. `#[from]` permite convertir automáticamente un `std::io::Error` en `ErrorConfiguracion`, de modo que `?` siga siendo útil. Las otras variantes conservan datos que la persona que llama puede inspeccionar mediante `match`. En una aplicación de una sola capa, convertir al final esos errores a `anyhow::Error` puede ser cómodo; en una biblioteca, ocultarlos demasiado pronto le quita opciones a quien la use.

La frontera práctica es ésta: `anyhow` para ejecutar una aplicación y explicar una falla completa; `thiserror` para ofrecer un contrato de errores que otros programas deban manejar por variante. Puedes combinar ambos, pero no los agregues por moda. Empieza con el tipo que permite a la siguiente capa tomar la decisión correcta.

## El error que vas a ver

### E0277: usar `?` en una función que no puede devolver un error

El error más frecuente al empezar a usar `?` aparece cuando la función declara un retorno simple, como `String`, pero dentro intenta propagar un `Result`. Rust no puede inventar dónde guardar el error ni cómo comunicarlo a quien llamó. La siguiente corrida real, con `rustc 1.98.1`, lee el programa desde la entrada estándar, por eso el compilador nombra el archivo como `<anon>`; con un archivo en disco verás su nombre en lugar de `<anon>`.

<!-- verificar:fragmento -->
```rust
fn leer() -> String {
    let texto = std::fs::read_to_string("faltante.txt")?;
    Ok(texto)
}

fn main() {}
```

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> <anon>:2:56
  |
1 | fn leer() -> String {
  | ------------------- this function should return `Result` or `Option` to accept `?`
2 |     let texto = std::fs::read_to_string("faltante.txt")?;
  |                                                        ^ cannot use the `?` operator in a function that returns `String`

error[E0308]: mismatched types
 --> <anon>:3:5
  |
1 | fn leer() -> String {
  |              ------ expected `String` because of return type
2 |     let texto = std::fs::read_to_string("faltante.txt")?;
3 |     Ok(texto)
  |     ^^^^^^^^^ expected `String`, found `Result<String, _>`
  |
  = note: expected struct `String`
               found enum `Result<String, _>`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

`E0277` dice que `?` necesita una función capaz de devolver un residuo de error. `E0308` es la consecuencia: `Ok(texto)` es un `Result`, pero la firma prometía un `String`. El arreglo no es quitar `?` y usar `unwrap()`. Debes corregir el contrato para que describa la posibilidad real de fallo.

<!-- verificar:fragmento -->
```rust
fn leer() -> Result<String, std::io::Error> {
    let texto = std::fs::read_to_string("faltante.txt")?;
    Ok(texto)
}
```

Si estás en `main`, también puedes devolver `Result` cuando el error debe terminar el programa. Sin embargo, el revisor actual necesita controlar qué se imprime y qué código de salida devuelve, por eso `main` transforma el resultado de `config::cargar` en una salida visible y `ExitCode::from(2)`. El mensaje va a `stderr`; la tabla o el JSON exitosos quedan disponibles en `stdout`.

### Un pánico por índice fuera de rango

El acceso `v[indice]` no es un error de compilación si `indice` es una variable. Rust no puede saber en tiempo de compilación qué número llegará. Si el número cae fuera del rango, el programa entra en pánico durante la ejecución. El diagnóstico menciona el índice pedido y la longitud real del vector. Por ejemplo, pedir la posición `3` de una lista de tres elementos falla porque las posiciones válidas son `0`, `1` y `2`.

La corrección depende del origen del índice. Si es una constante escrita junto a una lista fija y el índice está mal, corrige el programa. Si viene de un archivo, una petición o una opción de usuario, usa `get(indice)` y convierte `None` en un `Result` que explique cuál era el rango válido. No dejes que una entrada recuperable termine el proceso con un backtrace.

### Diagnosticar antes de arreglar

Lee primero la firma de la función y el tipo concreto de la expresión que falló. Si dice `Option`, decide qué significa la ausencia. Si dice `Result`, lee la variante de error y decide si debe propagarse, contextualizarse o manejarse ahí. Si aparece `panic!`, pregunta qué supuesto del programa se rompió. Copiar `.clone()`, `.unwrap()` o `as` hasta que compile suele borrar información útil y mueve el problema a la ejecución.

`rustc --explain E0277` amplía el significado general del código, pero el diagnóstico local sigue siendo la fuente principal. Sus flechas señalan el tipo esperado, el tipo encontrado y la línea donde el contrato dejó de coincidir. Aprender a seguir esas tres pistas vale más que memorizar una lista de códigos.

## Lo que se hace mal

- **Usar `v[i]` para índices que vienen de fuera.** El índice directo afirma que la posición existe. Si esa afirmación depende de un archivo o de una entrada humana, usa `get`; `None` es un dato que debes transformar en una explicación útil.

- **Recorrer un `HashMap` y publicar su orden accidental.** Un reporte que cambia de orden es difícil de leer, probar y comparar. Extrae llaves o filas, ordénalas y solo entonces imprime. La salida del revisor debe ser reproducible aunque el almacenamiento interno cambie.

- **Usar `String` en todos los parámetros.** Obliga a transferir propiedad o crear asignaciones innecesarias. Si una función solo lee, declara `&str`; reserva `String` para estructuras y resultados que deban poseer texto.

- **Indexar un `String` por byte o asumir que `len()` cuenta letras visibles.** Rust almacena texto UTF-8. Usa `chars`, `bytes` o rangos con `get` según la unidad que realmente necesitas. Un nombre que hoy solo tiene ASCII puede mañana contener caracteres válidos de más de un byte.

- **Convertir todos los errores en `unwrap()` o `expect()`.** Hace que el programa parezca corto mientras elimina rutas de recuperación y contexto. `unwrap` es aceptable en una prueba cuando el fallo vuelve inválida la propia prueba; no es la forma normal de leer archivos ni procesar argumentos.

- **Usar `panic!` para datos inválidos de configuración.** Una URL incorrecta, un archivo ausente o un nombre repetido son fallas que una persona puede corregir. Devuelve `Result` con el dato que falta y una causa comprensible.

- **Perder la causa original al crear un mensaje nuevo.** Un texto como `"no se pudo cargar"` no dice qué ruta falló ni por qué. Usa `with_context` para agregar operación y datos locales sin desechar la causa que devolvió el sistema o el parser.

- **Usar `anyhow` en una biblioteca que necesita errores distinguibles.** Si quien llama debe reaccionar distinto ante una URL inválida y un nombre repetido, publica un enum propio, normalmente con `thiserror`. La conveniencia de una cadena no debe borrar decisiones de dominio.

## Ejercicios

### Ejercicio 1 — Acceso honesto a una lista

Crea un `Vec<Servicio>` con dos servicios. Escribe una función `nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str>` que devuelva el nombre del servicio cuando existe y `None` cuando no. Pruébala con los índices `0`, `1` y `2`. No uses `[]` dentro de la función.

Explica por escrito por qué devolver `Option<&str>` es más honesto que devolver una cadena vacía. Piensa qué pasaría si un nombre vacío fuera un dato permitido.

### Ejercicio 2 — Estados por nombre y reporte determinista

Crea un `HashMap<String, Estado>` con tres nombres, incluyendo una falla. Escribe una función que produzca un `Vec<String>` con los nombres ordenados alfabéticamente. Después recorre esos nombres y genera líneas con formato `nombre: estado`.

Corre el programa varias veces. La salida debe conservar exactamente el mismo orden. No ordenes el `HashMap`: no se ordena. Ordena una colección separada de llaves o de filas.

### Ejercicio 3 — Cargar, validar y contextualizar

Escribe una función `cargar(ruta: &str) -> Result<Vec<Servicio>, ...>` que lea un archivo de texto con una línea por servicio. Cada línea debe contener nombre y URL separados por coma. Rechaza una línea sin dos campos, una URL sin `http://` o `https://`, y una lista vacía. Propaga los errores de lectura con `?`.

Después adapta la función para usar `anyhow::Context` y agregar la ruta al mensaje de lectura. Haz que `main` imprima los errores en la salida de error y termine con código `2`; si todos los datos son válidos, imprime la lista ordenada y termina con código `0`.

## Soluciones

### Solución 1

La función debe prestar el slice, no tomar el vector. `get` ya devuelve `Option<&Servicio>`, y `map` transforma el contenido de `Some` sin tocar `None`.

<!-- verificar:fragmento -->
```rust
fn nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str> {
    servicios.get(indice).map(|servicio| servicio.nombre.as_str())
}
```

`as_str()` convierte la referencia a `String` en una referencia a `str`; no asigna memoria ni clona texto. La vida del `&str` queda limitada por la vida del slice prestado, que es precisamente el contrato correcto. Una cadena vacía sería ambigua: podría significar “no encontré ese índice” o “sí encontré el servicio y su nombre es vacío”.

### Solución 2

La solución necesita separar la estructura útil para buscar de la estructura útil para presentar. El mapa conserva la asociación; el vector de llaves recibe el orden que requiere el reporte.

<!-- verificar:fragmento -->
```rust
let mut nombres: Vec<&str> = estados.keys().map(String::as_str).collect();
nombres.sort();

for nombre in nombres {
    let estado = &estados[nombre];
    println!("{nombre}: {estado:?}");
}
```

El acceso `estados[nombre]` es razonable aquí porque `nombre` proviene directamente de `estados.keys()`: el programa ya demostró que la llave existe. Si `nombre` viniera de un archivo o de un argumento, esta forma volvería a afirmar algo que no has validado y deberías usar `get`.

### Solución 3

La firma de carga debe dejar que los problemas externos suban como `Result`. Las reglas de formato también deben convertirse en errores, no en pánicos. Si usas `anyhow`, una implementación puede seguir esta forma:

<!-- verificar:fragmento -->
```rust
fn cargar(ruta: &str) -> anyhow::Result<Vec<Servicio>> {
    let texto = std::fs::read_to_string(ruta)
        .with_context(|| format!("leyendo {ruta}"))?;

    let mut servicios = Vec::new();
    for (numero, linea) in texto.lines().enumerate() {
        let (nombre, url) = linea
            .split_once(',')
            .with_context(|| format!("línea {} sin coma", numero + 1))?;

        anyhow::ensure!(
            url.starts_with("http://") || url.starts_with("https://"),
            "línea {}: URL sin esquema: {url}",
            numero + 1
        );

        servicios.push(Servicio::new(nombre, url));
    }

    anyhow::ensure!(!servicios.is_empty(), "el archivo no declara servicios");
    Ok(servicios)
}
```

La solución no usa `unwrap` porque cada fallo puede originarse en contenido externo. `split_once` devuelve `Option`; `with_context` lo convierte en un error explicativo. `ensure!` termina la función con `Err` si la condición no se cumple. El contexto incluye número de línea o ruta para que quien corrige el archivo no tenga que adivinar dónde empezar.

## Cómo sé que lo logré

- Ejecutas `rustc --edition 2024 fig04_01.rs && ./fig04_01` y obtienes exactamente seis líneas, incluida `None` para `pagos` y `Some(2)` para el contador.
- Ejecutas `rustc --edition 2024 fig04_02.rs && ./fig04_02` y puedes explicar por qué el mismo parámetro `&str` acepta un literal y una referencia a `String`.
- Ejecutas `rustc --edition 2024 fig04_03.rs && ./fig04_03` y obtienes un error de archivo ausente sin un pánico.
- Tu solución del ejercicio 1 devuelve `None` para el índice fuera de rango y no contiene acceso con `servicios[indice]`.
- Tu reporte del ejercicio 2 produce las mismas líneas, en el mismo orden, después de al menos diez ejecuciones.
- Tu solución del ejercicio 3 nombra la ruta cuando el archivo no existe, nombra la línea cuando el formato está mal y no usa `unwrap` en la ruta normal de ejecución.
- Desde `programas/revisor`, `cargo test`, `cargo clippy --all-targets -- -D warnings` y `cargo fmt --check` terminan correctamente.

## Para leer más

- [The Rust Programming Language, capítulo 8: colecciones](https://doc.rust-lang.org/book/ch08-00-common-collections.html), en particular vectores, cadenas y mapas hash. Consultado el 2 de octubre de 2026.

- [The Rust Programming Language, capítulo 9: manejo de errores](https://doc.rust-lang.org/book/ch09-00-error-handling.html). Consultado el 2 de octubre de 2026.

- [Documentación oficial de `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) y [de `HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html). Consultado el 2 de octubre de 2026.

- [Documentación de `anyhow`](https://docs.rs/anyhow/) y [documentación de `thiserror`](https://docs.rs/thiserror/). Consultado el 2 de octubre de 2026.
