# Lección 8 — El programa terminado

**Tiempo:** 2 × 45 min

**Qué construyes:** el `revisor` completo, en un binario

**Qué aprendes:** `reqwest`, `serde`, `clap`, el perfil de release, el binario final y su comparación con el de Go

## Al terminar vas a poder

- Explicar qué responsabilidad tiene `main.rs` y por qué la lógica reutilizable vive en la biblioteca `revisor`.
- Leer configuración YAML con `serde`, incluidos valores por omisión y validaciones que YAML no puede expresar.
- Usar `clap` para declarar opciones de línea de comandos con tipos, valores por omisión y ayuda automática.
- Explicar por qué una caída HTTP es un resultado del dominio del `revisor`, mientras que un archivo de configuración inválido impide arrancar.
- Compilar, probar y revisar el binario con `cargo test`, `cargo clippy` y `cargo build --release`.
- Comparar con argumentos concretos el binario final de Rust y el programa equivalente de Go.

## El porqué antes del cómo

Durante las lecciones anteriores construiste las piezas del `revisor`: el vocabulario del problema, la lectura de YAML, el reporte, las pruebas y las consultas HTTP concurrentes. Una pieza aislada puede estar bien escrita y, aun así, no ser una herramienta que otra persona pueda usar. Falta unirlas en una frontera clara: un ejecutable que recibe argumentos, lee un archivo, consulta servicios, imprime un resultado útil y termina con un código que otro programa pueda interpretar.

Eso parece una capa delgada, pero es donde se encuentran decisiones importantes. La línea de comandos es una interfaz pública: si hoy aceptas `--formato json`, alguien puede integrarla en un script y depender de ella mañana. El archivo YAML también es una interfaz: no es código Rust, así que puede contener nombres repetidos, URLs incompletas o un tiempo límite de cero. La red es otra frontera: una respuesta HTTP 500 no significa que el propio `revisor` esté roto; significa que el servicio revisado está en mal estado. En cambio, no poder leer el YAML sí impide comenzar a trabajar.

El programa terminado necesita distinguir esos casos sin esconderlos bajo un mismo `unwrap()`. Si todo está sano, termina con código 0. Si alcanzó a revisar y detectó una falla, imprime el reporte y termina con código 1. Si no pudo arrancar porque los argumentos o la configuración son inválidos, informa el problema en la salida de error y termina con código 2. Esa separación hace que el binario sea útil tanto para una persona que lo ejecuta en una terminal como para un sistema de automatización.

Esta lección continúa el mismo programa que hiciste en Go. La comparación importa porque ambos lenguajes llegan a un binario distribuible, pero toman caminos distintos. Go incluye HTTP, JSON, banderas y concurrencia en su biblioteca estándar. Rust deja que la biblioteca estándar sea pequeña y estable; para HTTP asíncrono, serialización YAML o una interfaz de línea de comandos declarativa usa crates del ecosistema. No es una ventaja automática de un lado ni una carencia automática del otro. Es una decisión de distribución del trabajo entre el lenguaje, el gestor de paquetes y las bibliotecas.

El proyecto `programas/revisor/` fija las versiones concretas con `Cargo.toml` y `Cargo.lock`. `cargo` resuelve el árbol completo de dependencias y el lockfile conserva la resolución exacta para que otra computadora construya el mismo conjunto de crates compatibles. No copies versiones de un tutorial antiguo a ciegas ni ejecutes `cargo update` como una reacción automática: primero entiende qué cambió, revisa el lockfile y corre las pruebas completas.

The Rust Book, capítulo 12, es la lectura central de esta lección: organiza un programa de línea de comandos alrededor de entrada, configuración, errores y separación de responsabilidades. Retoma también los ejercicios de Rustlings que te hayan costado sobre `Result`, pruebas e iteradores. El objetivo ya no es memorizar sintaxis; es ver cómo las decisiones de las lecciones anteriores sobreviven cuando el programa tiene usuarios, archivos y red reales.

## Los conceptos

### El binario es una frontera, no el lugar para toda la lógica

Un binario tiene una tarea específica: traducir el exterior al interior del programa. Lee los argumentos del sistema operativo, decide qué se imprime, convierte el resultado general en un código de salida y delega el resto. Si mezclas ahí la carga de YAML, las solicitudes HTTP, el formato de tabla y el código de salida, las pruebas terminan dependiendo de la terminal y de `std::process::exit`. Eso hace que una prueba pequeña se vuelva lenta, frágil y difícil de diagnosticar.

La alternativa del `revisor` es tener una biblioteca con módulos públicos (`config`, `modelo`, `reporte` y `revisar`) y un `main.rs` corto. La biblioteca puede probarse desde sus pruebas unitarias y desde `tests/integracion.rs`. El binario conserva solo lo que necesariamente depende de la línea de comandos. Esta división no es una regla ceremonial: reduce el número de lugares donde un cambio de interfaz puede romper el comportamiento.

Antes de usar `clap`, conviene entender el trabajo que está automatizando. Un parser de argumentos debe llevar estado: cada bandera consume, en su caso, el valor que sigue; debe conservar valores por omisión; y debe rechazar una bandera desconocida en vez de interpretarla silenciosamente. El siguiente programa simula esa frontera sin depender de crates.

**Fig. 8.1** | Un parser mínimo conserva valores por omisión y convierte texto a tipos.

```rust
// fig08_01.rs
struct Args {
    archivo: String,
    formato: String,
    paralelo: usize,
}

fn leer(args: &[&str]) -> Result<Args, String> {
    let mut resultado = Args {
        archivo: "servicios.yaml".to_string(),
        formato: "tabla".to_string(),
        paralelo: 5,
    };
    let mut i = 0;

    while i < args.len() {
        match args[i] {
            "-a" | "--archivo" => {
                i += 1;
                resultado.archivo = args.get(i).ok_or("falta archivo")?.to_string();
            }
            "-f" | "--formato" => {
                i += 1;
                resultado.formato = args.get(i).ok_or("falta formato")?.to_string();
            }
            "-p" | "--paralelo" => {
                i += 1;
                resultado.paralelo = args
                    .get(i)
                    .ok_or("falta paralelo")?
                    .parse()
                    .map_err(|_| "paralelo no es un número")?;
            }
            otro => return Err(format!("argumento desconocido: {otro}")),
        }
        i += 1;
    }

    Ok(resultado)
}

fn main() {
    let a = leer(&["--archivo", "demo.yaml", "-f", "json", "-p", "2"]).unwrap();
    println!("{} | {} | {}", a.archivo, a.formato, a.paralelo);
}
```

```bash
$ rustc --edition 2024 fig08_01.rs && ./fig08_01
demo.yaml | json | 2
```

El programa ilustra por qué no conviene escribir este parser a mano para cada binario. No genera `--help`, no documenta sus opciones por sí mismo, no valida valores permitidos y su código crecería rápido. `clap` declara el contrato y genera gran parte de esa mecánica. En el `revisor`, `Args` es el contrato de entrada: `archivo`, `formato` y `paralelo` tienen nombre largo, nombre corto, tipo y valor por omisión.

<!-- verificar:extracto:src/main.rs -->
```rust
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}
```

El atributo `#[derive(Parser)]` genera una implementación del trait `Parser`. Por eso `Args::parse()` puede leer `std::env::args()` y construir un `Args` tipado. `default_value` recibe texto porque `clap` lo convierte al tipo del campo; `default_value_t` recibe directamente un valor Rust, por eso es apropiado para `usize`.

El `main` del proyecto no llama a `std::process::exit`. Regresa `ExitCode`, que es más fácil de probar cuando la lógica se mantiene en `ejecutar`. El código comprueba primero que el formato sea uno de los dos contratos prometidos. Después carga y valida la configuración, crea un cliente HTTP, pide los estados, decide si imprime tabla o JSON y convierte el resultado final en 0, 1 o 2. El orden importa: no hay razón para abrir conexiones de red si el archivo YAML ni siquiera es legible.

<!-- verificar:extracto:src/main.rs -->
```rust
#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    ejecutar(&args).await
}

async fn ejecutar(args: &Args) -> ExitCode {
    if args.formato != "tabla" && args.formato != "json" {
        eprintln!(
            "revisor: --formato debe ser «tabla» o «json», no «{}»",
            args.formato
        );
        return ExitCode::from(2);
    }

    let servicios =
        match config::cargar(&args.archivo).and_then(|s| config::validar(&s).map(|()| s)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("revisor: {e:#}");
                return ExitCode::from(2);
            }
        };

    let cliente = reqwest::Client::new();
    let estados = revisar::revisar_todos(&cliente, &servicios, args.paralelo).await;

    if args.formato == "json" {
        match reporte::json(&servicios, &estados) {
            Ok(j) => println!("{j}"),
            Err(e) => {
                eprintln!("revisor: escribiendo el reporte: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        print!("{}", reporte::tabla(&servicios, &estados));
    }

    if estados.iter().all(|e| e.esta_bien()) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
```

La anotación `#[tokio::main]` crea y arranca el runtime necesario para poder usar `.await` en la función principal. No hace que todo el programa sea más rápido por sí solo. Su propósito es ejecutar futuros mientras las solicitudes HTTP esperan datos de red, sin bloquear un hilo por cada espera.

### `serde` hace explícito el contrato de datos

Un archivo YAML llega al programa como texto. El texto no sabe qué es un nombre, qué campo es obligatorio ni qué valor debe usarse cuando falta `timeout_ms`. Convertirlo a un `Vec<Servicio>` es pasar de datos que vienen del exterior a valores que el compilador puede revisar. Esa conversión es un límite de confianza: después de deserializar todavía debes validar las reglas de negocio que el formato no conoce.

`serde` separa dos direcciones. `Deserialize` construye valores Rust desde YAML, JSON, TOML u otro formato que tenga un adaptador compatible. `Serialize` convierte valores Rust en un formato de salida. La estructura del dominio se conserva; cambia el formato que la rodea. Por eso el mismo `EstadoJson` se puede generar con `serde_json`, mientras `Servicio` entra con `serde_yaml`.

El siguiente programa muestra una decisión del dominio que también aparece en el JSON del `revisor`: un estado sano tiene código HTTP y no lleva error; una falla no inventa un código y sí incluye un motivo. El programa arma JSON manualmente para que se vea la diferencia. En el proyecto real no debes hacer esto a mano: `serde_json` se encarga de escapar texto y preservar un JSON válido.

**Fig. 8.2** | La forma de salida depende de la variante del estado.

```rust
// fig08_02.rs
enum Estado {
    Ok(u16),
    Falla(&'static str),
}

fn json(nombre: &str, estado: &Estado, ms: u64) -> String {
    match estado {
        Estado::Ok(codigo) => {
            format!("{{\"servicio\":\"{nombre}\",\"codigo\":{codigo},\"ms\":{ms}}}")
        }
        Estado::Falla(motivo) => {
            format!(
                "{{\"servicio\":\"{nombre}\",\"codigo\":null,\"ms\":{ms},\"error\":\"{motivo}\"}}"
            )
        }
    }
}

fn main() {
    println!("{}", json("catalogo", &Estado::Ok(200), 7));
    println!("{}", json("pagos", &Estado::Falla("codigo 500"), 12));
}
```

```bash
$ rustc --edition 2024 fig08_02.rs && ./fig08_02
{"servicio":"catalogo","codigo":200,"ms":7}
{"servicio":"pagos","codigo":null,"ms":12,"error":"codigo 500"}
```

En el `revisor`, el derive declara exactamente qué debe leer o escribir. `Servicio` deriva `Deserialize` porque se crea desde YAML. `EstadoJson` deriva `Serialize` porque se crea desde los resultados internos para producir JSON. `#[serde(default = "timeout_por_omision")]` no equivale a que el campo sea opcional en Rust: el campo final sigue siendo un `u64`; solo obtiene un valor cuando el YAML no lo declara.

<!-- verificar:extracto:src/modelo.rs -->
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}

#[derive(Serialize)]
pub struct EstadoJson {
    pub servicio: String,
    pub codigo: Option<u16>,
    pub ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

`Option<u16>` representa una diferencia importante: si no hubo respuesta HTTP, no existe código HTTP. Usar `0` o una cadena vacía inventaría un dato y obligaría a todo consumidor a recordar esa convención. En JSON, `Option::None` se convierte en `null` para `codigo`. Para `error`, el atributo `skip_serializing_if` elige otra política: cuando no hay error, ni siquiera aparece la llave. Ambas decisiones son válidas, pero deben ser intencionales y estar cubiertas por pruebas.

Deserializar bien no basta. YAML puede expresar una lista vacía, repetir un nombre, aceptar una cadena como URL o incluir un tiempo límite de cero. El parser no sabe que esas opciones vuelven inútil al `revisor`. `config::validar` es el segundo paso y expresa las reglas del programa, no las del formato.

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

`anyhow::ensure!` devuelve temprano un error cuando una condición no se cumple. Aquí es adecuado porque una configuración inválida no representa un servicio fallido: es una condición que impide iniciar la revisión. `with_context` en `cargar` agrega el nombre del archivo a un error de lectura; el formato `{e:#}` en `main` imprime esa cadena de contexto junto con la causa original. El resultado es más útil que un mensaje genérico de “no se pudo abrir”.

### `reqwest` convierte la red en estados del dominio

Un programa de supervisión no consulta HTTP para obtener un cuerpo y olvidarlo; consulta para clasificar el estado de un servicio. Por eso la función `revisar` no devuelve `Result<Estado>`. Si un servidor responde 500, rechaza la conexión o excede su tiempo límite, la operación del `revisor` sí terminó: descubrió una falla y debe incluirla en el reporte. Esos casos se convierten en `Estado::Falla`.

No confundas ese resultado con una falla de configuración o con un error de serialización al producir el reporte. Esos sí impiden que el programa cumpla su trabajo y hacen que termine con código 2. La distinción evita dos errores frecuentes: detener toda la revisión porque un servicio cayó, o continuar como si nada cuando no se pudo leer el archivo que define qué servicios existen.

La red además obliga a separar concurrencia de orden. El `revisor` puede iniciar varias consultas al mismo tiempo, pero la salida debe ser reproducible y debe relacionar cada estado con su servicio correcto. `join_all` conserva el orden de los futuros de entrada, aunque las respuestas lleguen en otro orden. El semáforo limita cuántas consultas entran a la sección activa; no determina el orden final del `Vec<Estado>`.

El ejemplo no hace HTTP real porque un programa de una sola figura debe ser determinista. En cambio, representa la política del semáforo: con límite 2, se entregan turnos de dos en dos. En el proyecto real cada turno vive hasta que termina la solicitud y el runtime despierta la tarea cuando la red responde.

**Fig. 8.3** | Un límite de concurrencia divide los pendientes en lotes sin cambiar su orden.

```rust
// fig08_03.rs
fn lote<T>(pendientes: &mut Vec<T>, limite: usize) -> Vec<T> {
    let n = limite.min(pendientes.len());
    pendientes.drain(..n).collect()
}

fn main() {
    let mut servicios = vec!["catalogo", "pagos", "usuarios", "correo", "facturas"];

    while !servicios.is_empty() {
        println!("arrancan: {}", lote(&mut servicios, 2).join(", "));
    }
}
```

```bash
$ rustc --edition 2024 fig08_03.rs && ./fig08_03
arrancan: catalogo, pagos
arrancan: usuarios, correo
arrancan: facturas
```

La función real crea un `Arc<Semaphore>` porque cada futuro necesita compartir el mismo contador de turnos. `Arc` permite propiedad compartida entre tareas; `Semaphore` entrega un permiso temporal; y la variable `_turno` conserva ese permiso hasta terminar `revisar`. El guion bajo inicial evita un aviso del compilador, pero el valor no se descarta: su destructor devuelve el permiso cuando sale del bloque.

<!-- verificar:extracto:src/revisar.rs -->
```rust
pub async fn revisar_todos(
    cliente: &reqwest::Client,
    servicios: &[Servicio],
    paralelo: usize,
) -> Vec<Estado> {
    let paralelo = if paralelo == 0 {
        PARALELO_POR_OMISION
    } else {
        paralelo
    };
    let turnos = Arc::new(Semaphore::new(paralelo));

    let futuros = servicios.iter().map(|s| {
        let turnos = Arc::clone(&turnos);
        async move {
            // pide turno; espera si ya hay `paralelo` corriendo, y lo devuelve al soltar `_turno`
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
            revisar(cliente, s).await
        }
    });
    join_all(futuros).await
}
```

El parámetro `paralelo` tiene un matiz: el programa acepta `0` como “usa el valor por omisión”. Es una decisión de interfaz; no significa que Tokio pueda ejecutar cero solicitudes. La constante `PARALELO_POR_OMISION` conserva esa regla cerca de la lógica que la utiliza.

La consulta individual crea un límite por servicio con `.timeout(Duration::from_millis(s.timeout_ms))`. Ese límite no reemplaza al semáforo. El timeout decide cuánto puede esperar una solicitud; el semáforo decide cuántas solicitudes pueden estar esperando a la vez. Sin timeout, un turno puede quedarse ocupado demasiado tiempo. Sin semáforo, una lista enorme puede abrir demasiadas conexiones al mismo tiempo. Ambos límites protegen recursos diferentes.

`reqwest::Client::new()` se crea una vez y se presta a todas las consultas. No construyas un cliente por servicio: un cliente puede conservar y reutilizar conexiones, mientras uno nuevo por llamada pierde esa ventaja y añade trabajo innecesario. El préstamo compartido `&reqwest::Client` es suficiente porque las solicitudes del cliente no requieren una referencia mutable exclusiva.

### El reporte es una interfaz estable para personas y programas

Una tabla sirve para leer en una terminal. JSON sirve para que otro programa consuma resultados sin tener que adivinar columnas, espacios o alineación. No conviene usar la tabla como un formato de integración: si mañana cambias su ancho, un script que la procese con `awk` puede romperse aunque la información sea la misma. Tampoco conviene imprimir diagnósticos humanos en la salida estándar cuando se pidió JSON, porque dejaría de ser JSON válido.

El `revisor` usa `stdout` para el reporte y `stderr` para los errores de arranque. Esa convención permite redirigir solo el reporte a un archivo:

```bash
cargo run -- --archivo servicios.yaml --formato json > estado.json
```

Si la configuración es válida, `estado.json` contiene únicamente JSON. Si algo impide arrancar, el mensaje aparece en la terminal mediante `stderr`; no se mezcla con un formato que otro proceso espera analizar.

La función `json` parte de servicios y estados, los ordena por nombre y construye una colección de `EstadoJson`. Ordenar antes de serializar no es obligatorio para que JSON sea válido, pero sí ayuda a que el resultado sea estable. Un resultado estable se compara mejor en pruebas, revisiones de cambios y automatizaciones.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub fn json(servicios: &[Servicio], estados: &[Estado]) -> serde_json::Result<String> {
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
    serde_json::to_string_pretty(&lineas)
}
```

Los `match` fuerzan a decidir qué representa cada variante. `NoIntentado` no tiene duración ni código real, así que se expresa como `ms: 0`, `codigo: null` y un error explícito. `Falla` conserva su tiempo transcurrido porque saber que una conexión agotó su límite en 5 000 ms es información útil. `Ok` y `Lento` tienen código; ambos son sanos para el código de salida, aunque el reporte los diferencie.

Esta parte revela una diferencia útil con Go. En Go puedes construir structs con etiquetas `json:"..."` usando solo `encoding/json`; para YAML normalmente agregas otra biblioteca con su propia convención de etiquetas. En Rust, `serde` centraliza la definición de serialización y permite que los adaptadores de formato trabajen sobre el mismo derive. A cambio, debes incorporar crates externos y entender qué partes del contrato están en los atributos y cuáles en la validación explícita.

### El perfil de release prepara un artefacto, no sustituye la verificación

`cargo run` y `cargo test` usan por omisión el perfil de desarrollo. Ese perfil privilegia que compilar durante la edición sea rápido y conserva información útil para depurar. Un binario distribuible se construye con `cargo build --release`; entonces Cargo aplica el perfil `release` definido por el proyecto.

El perfil no transforma un programa incorrecto en uno correcto. Primero deben pasar las pruebas, el formato y Clippy. Después el perfil decide el equilibrio entre tamaño de binario, tiempo de compilación, optimización y diagnóstico disponible si ocurre un pánico. En este proyecto se eligió tamaño: se quitan símbolos, se optimiza para tamaño, se activa la optimización entre módulos, se usa una sola unidad de generación y los pánicos abortan el proceso.

<!-- verificar:extracto:Cargo.toml -->
```toml
[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila
```

Cada opción tiene costo. `lto = true` y `codegen-units = 1` pueden aumentar el tiempo de compilación porque permiten optimizar con más información global. `panic = "abort"` reduce el binario y evita el desenrollado de pila, pero sacrifica la posibilidad de limpiar mediante desenrollado y ofrece menos contexto si un pánico llega a producción. Para esta herramienta de línea de comandos es una elección razonable; no es una receta universal para toda biblioteca.

No declares que Rust o Go “ganan” por el tamaño de un binario sin medir en tu máquina y con la misma funcionalidad. El binario de Rust puede crecer al incluir Tokio, Reqwest, TLS y sus dependencias transitivas; el de Go también incluye su runtime y depende de sus opciones de compilación. La comparación válida anota plataforma, arquitectura, compilación en frío o incremental, perfil usado y qué dependencias incluye cada programa.

La compilación cruzada deja otra diferencia práctica. Go suele permitir seleccionar plataforma con variables como `GOOS` y `GOARCH`. Rust requiere instalar el target correspondiente y, según el target, disponer también de un enlazador y bibliotecas compatibles. Para un Linux estático basado en musl, el primer paso sería:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Esto no significa que la compilación cruzada sea imposible en Rust; significa que debes modelar el target como parte del entorno de construcción. Verifica el binario resultante en el sistema donde se usará, especialmente si incluye TLS o si cambias entre libc de Linux.

## El error que vas a ver

El parser de argumentos debe convertir texto a valores tipados. Si intentas asignar directamente un texto a un `usize`, Rust no adivina que quieres interpretar el texto como número. El siguiente programa falla antes de ejecutarse.

**Fig. 8.4** | Un texto no es un número aunque represente una opción de línea de comandos.

```rust
// fig08_04.rs
fn main() {
    let paralelo: usize = "cinco";
    println!("{paralelo}");
}
```

```bash
$ rustc --edition 2024 fig08_04.rs
error[E0308]: mismatched types
 --> fig08_04.rs:3:27
  |
3 |     let paralelo: usize = "cinco";
  |                   -----   ^^^^^^^ expected `usize`, found `&str`
  |                   |
  |                   expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`E0308` significa que dos tipos no coinciden. La anotación `: usize` establece lo que debe producir la expresión de la derecha, pero `"cinco"` es un `&str`. El arreglo no es quitar el tipo para que compile con una cadena: `paralelo` debe seguir siendo un número porque controla un límite de concurrencia. Debes convertir con `.parse::<usize>()` y decidir qué hacer si la conversión falla.

En el programa mínimo esa decisión aparece como `Result<Args, String>`. En el proyecto real, `clap` hace la conversión de los campos declarados y muestra un diagnóstico de uso cuando no puede construir el tipo esperado. Después `main` todavía valida las reglas propias de la aplicación: que `formato` sea `tabla` o `json`, que el YAML tenga servicios y que los valores de cada servicio tengan sentido.

Otro error frecuente es confundir un HTTP no exitoso con un error que debe propagarse mediante `?`. En este programa, un HTTP 500 se convierte en `Estado::Falla`, se imprime y da código de salida 1. Propagarlo como error de arranque haría que un servicio caído escondiera el resultado de los demás. El operador `?` sigue siendo correcto para fallas que impiden continuar, como no poder leer el YAML o no poder serializar el reporte JSON.

## Lo que se hace mal

### Poner `unwrap()` en las fronteras del programa

`unwrap()` es aceptable en una prueba cuando preparas datos literales que tú controlas. No lo es para argumentos, archivos YAML ni red. Un argumento mal escrito no debe producir un pánico con un backtrace; debe dar un mensaje que diga qué opción es inválida y terminar con código 2. Un archivo ausente no es una sorpresa de programación: es una condición de uso que el binario debe informar con el nombre del archivo.

La regla útil es preguntar quién controla el dato. Si lo controla la prueba, `expect` puede hacer que una falla sea clara. Si lo controla una persona, un archivo, el sistema operativo o la red, conviértelo en un error contextual o en un estado del dominio.

### Crear un `reqwest::Client` por cada servicio

Crear el cliente dentro de `revisar` parece local y sencillo, pero pierde reutilización de conexiones y hace que la configuración del cliente se disperse. El proyecto crea un solo `reqwest::Client` después de que la configuración se validó y lo presta a todas las consultas. Así el ciclo de vida del cliente coincide con el ciclo de vida de una corrida del `revisor`.

No confundas compartir el cliente con compartir estado mutable sin control. El cliente se usa mediante referencias compartidas, y el semáforo protege el recurso que sí debe limitarse: cuántas solicitudes están activas.

### Usar JSON construido con `format!` en producción

La figura de JSON muestra la forma de los datos, no una técnica de serialización segura. Si el nombre de un servicio contiene comillas, una barra inversa o un salto de línea, una cadena construida a mano puede dejar de ser JSON válido o cambiar su significado. `serde_json` escapa esos datos correctamente y conserva la relación entre `Option`, `null` y campos omitidos.

Una buena regla es que los formatos estructurados se construyan con un serializador y se prueben leyéndolos de vuelta. En las pruebas del proyecto, el JSON se convierte a `serde_json::Value` antes de comprobar campos específicos.

### Tratar el código de salida como un detalle invisible

Una tabla con la palabra `FALLA` ayuda a una persona, pero un trabajo programado necesita saber si debe alertar. Si el `revisor` siempre termina con 0, un cron, un pipeline o un supervisor puede creer que todo está sano aunque haya servicios caídos. Si termina con 1 ante cualquier problema, tampoco distingue entre una caída vigilada y una mala configuración.

Mantén el contrato simple y documentado: 0 para una revisión sana, 1 para servicios fallidos y 2 para errores de arranque o de uso. Las pruebas del binario deben comprobar esos tres caminos.

### Medir el binario de Rust contra el de Go sin controlar el experimento

Comparar solo el tamaño de dos archivos es una forma pobre de sacar conclusiones. El resultado cambia si uno incluye TLS, si el otro usa una biblioteca HTTP externa, si se compiló uno en desarrollo y otro en release, si se eliminaron símbolos o si el sistema comprime ejecutables. El tiempo también cambia entre una compilación fría, una incremental y una reconstrucción después de tocar una línea.

Mide con el mismo caso de uso y registra las condiciones. A veces Go será la elección práctica porque la compilación cruzada y la biblioteca estándar reducen pasos. A veces Rust será preferible porque quieres expresar ciertas garantías antes de ejecutar y aceptar una compilación más costosa. El propósito de hacer los dos cursos no es repetir un eslogan; es tener evidencia propia.

## Ejercicios

### Ejercicio 1 — Declara una opción adicional

Agrega a un programa independiente una opción `--silencioso` que por omisión sea falsa. Cuando sea verdadera, el programa debe imprimir únicamente el código numérico de cada estado; cuando sea falsa, debe imprimir nombre, estado y código. Haz que una bandera desconocida produzca un `Err` con un mensaje claro.

Después explica qué declararías en `clap`: un campo `bool`, una opción con `#[arg(long)]` y el comportamiento por omisión que su tipo ya expresa.

### Ejercicio 2 — Separa datos inválidos de servicios fallidos

Escribe una función que reciba una lista de servicios mínimos con `nombre`, `url` y `timeout_ms`. Debe devolver un error si la lista está vacía, si un nombre se repite, si la URL no comienza con `http://` o `https://`, o si el tiempo límite es cero. Por separado, representa un HTTP 503 como una variante de estado, no como error de validación.

Usa las reglas de `config::validar` como referencia, pero escribe primero tus casos de prueba: una lista válida, una vacía, una con nombre duplicado y una con timeout cero.

### Ejercicio 3 — Prueba el contrato del binario

En `programas/revisor/`, lee `tests/binario.rs`. Agrega una prueba de punta a punta para el formato inválido `-f xml`. Debe comprobar que el código de salida sea 2, que no haya tabla en la salida estándar y que la salida de error nombre `--formato`.

Después corre solo esa prueba y luego toda la suite. No cambies el código del proyecto para que pase una prueba mal escrita: la prueba debe describir el contrato que ya expone `main.rs`.

### Ejercicio 4 — Compara las dos implementaciones

Construye en release el `revisor` de Rust y el programa equivalente de Go. En una bitácora anota, para ambos, líneas de código propias, dependencias directas, tiempo de compilación en frío, tamaño del binario, uso de memoria en una misma configuración y tiempo que te tomó completar el programa.

Responde con una frase fundamentada: cuál entregarías para una herramienta interna con una fecha cercana, cuál para una herramienta que debe mantenerse años y qué decisión concreta de cada lenguaje te llevó a esa conclusión.

## Soluciones

### Solución 1

<!-- verificar:fragmento -->
```rust
fn imprimir(nombre: &str, codigo: u16, silencioso: bool) {
    if silencioso {
        println!("{codigo}");
    } else {
        println!("{nombre} OK {codigo}");
    }
}
```

La opción no cambia el estado del dominio; cambia la presentación. Por eso debe aplicarse cerca de la frontera de salida, no dentro de `Estado` ni de la lógica HTTP. Con `clap`, un `bool` marcado con `#[arg(long)]` expresa que la ausencia de la bandera vale `false` y su presencia vale `true`.

### Solución 2

<!-- verificar:fragmento -->
```rust
fn validar_timeout(timeout_ms: u64) -> Result<(), String> {
    if timeout_ms == 0 {
        Err("el tiempo límite debe ser mayor que cero".to_string())
    } else {
        Ok(())
    }
}

enum Estado {
    Falla { codigo: u16 },
}
```

El timeout inválido es un problema de entrada: no se debe iniciar la revisión. Un código 503, en cambio, es información obtenida al revisar y pertenece al estado del servicio. La separación permite que el binario termine con 2 en el primer caso y con 1 en el segundo.

### Solución 3

<!-- verificar:fragmento -->
```rust
#[test]
fn un_formato_desconocido_sale_con_dos() {
    let r = revisor(&["-f", "xml"]);
    assert_eq!(r.status.code(), Some(2), "stderr: {}", texto(&r.stderr));
    assert!(texto(&r.stdout).is_empty(), "stdout: {}", texto(&r.stdout));
    assert!(
        texto(&r.stderr).contains("--formato"),
        "stderr: {}",
        texto(&r.stderr)
    );
}
```

La prueba ejecuta el binario real, por eso revisa sus tres salidas observables: código, salida estándar y salida de error. No necesita conocer funciones privadas de `main.rs`; solo conoce el contrato que verá quien ejecute `revisor -f xml`.

### Solución 4

La comparación debe comenzar con comandos reproducibles:

```bash
cd programas/revisor
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

Después repite el experimento con el programa de Go, en la misma computadora y con la misma configuración de servicios. No copies resultados ajenos como si fueran universales. El valor del ejercicio es identificar qué parte del tiempo fue compilación, qué parte fue aprender el ecosistema y qué parte fue resolver el mismo problema de diseño.

## Cómo sé que lo logré

- `cargo run -- --help` muestra las opciones `--archivo`, `--formato` y `--paralelo`.
- `cargo run -- --archivo archivo-que-no-existe.yaml` termina con código 2 y escribe el nombre del archivo en la salida de error.
- `cargo run -- --formato xml` termina con código 2 y explica que los formatos válidos son `tabla` y `json`.
- `cargo test` termina con resultados correctos para biblioteca, integración y binario.
- `cargo clippy --all-targets -- -D warnings` termina sin avisos.
- `cargo fmt --check` termina sin cambios pendientes.
- `cargo build --release` crea el binario en `target/release/revisor`.
- Puedes explicar por qué un HTTP 500 produce código de salida 1, mientras un YAML inválido produce código 2.

## Para leer más

- [The Rust Programming Language, capítulo 12: An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) — consultado el 2 de octubre de 2026.
- [Referencia oficial de Cargo: perfiles](https://doc.rust-lang.org/cargo/reference/profiles.html) — consultado el 2 de octubre de 2026.
- [Documentación de `clap`](https://docs.rs/clap/latest/clap/) — consultado el 2 de octubre de 2026.
- [Documentación de `reqwest`](https://docs.rs/reqwest/latest/reqwest/) — consultado el 2 de octubre de 2026.
