# Lesson 8 — The Finished Program

**Time:** 2 × 45 min.

**What you build:** the complete `revisor`, in a single binary

**What you learn:** `reqwest`, `serde`, `clap`, the release profile, the final binary and its comparison with the Go one

## By the end you will be able to

- Explain what responsibility `main.rs` has and why the reusable logic lives in the `revisor` library.
- Read YAML configuration with `serde`, including default values and validations that YAML can't express.
- Use `clap` to declare command-line options with types, default values, and automatic help.
- Explain why an HTTP failure is a result in the `revisor`'s domain, while an invalid configuration file prevents startup.
- Compile, test, and review the binary with `cargo test`, `cargo clippy`, and `cargo build --release`.
- Compare, with concrete arguments, the final Rust binary and the equivalent Go program.

## The why before the how

During the previous lessons you built the pieces of the `revisor`: the vocabulary of the problem, the reading of YAML, the report, the tests, and the concurrent HTTP queries. An isolated piece can be well written and still not be a tool that someone else can use. What's missing is joining them at a clear boundary: an executable that receives arguments, reads a file, queries services, prints a useful result, and ends with a code that another program can interpret.

That looks like a thin layer, but it is where important decisions meet. The command line is a public interface: if today you accept `--formato json`, someone can integrate it into a script and depend on it tomorrow. The YAML file is also an interface: it isn't Rust code, so it can contain repeated names, incomplete URLs, or a timeout of zero. The network is another boundary: an HTTP 500 response doesn't mean the `revisor` itself is broken; it means the service being checked is in a bad state. On the other hand, not being able to read the YAML does prevent the work from starting.

The finished program needs to tell those cases apart without hiding them under a single `unwrap()`. If everything is healthy, it ends with code 0. If it managed to check and detected a failure, it prints the report and ends with code 1. If it couldn't start because the arguments or the configuration are invalid, it reports the problem on the error output and ends with code 2. That separation makes the binary useful both for a person running it in a terminal and for an automation system.

This lesson continues the same program you made in Go. The comparison matters because both languages arrive at a distributable binary, but they take different paths. Go includes HTTP, JSON, flags, and concurrency in its standard library. Rust keeps the standard library small and stable; for asynchronous HTTP, YAML serialization, or a declarative command-line interface it uses crates from the ecosystem. It isn't an automatic advantage on one side or an automatic lack on the other. It is a decision about how to distribute the work among the language, the package manager, and the libraries.

The `programas/revisor/` project pins the concrete versions with `Cargo.toml` and `Cargo.lock`. `cargo` resolves the whole dependency tree and the lockfile keeps the exact resolution so that another computer builds the same set of compatible crates. Don't blindly copy versions from an old tutorial or run `cargo update` as an automatic reaction: first understand what changed, review the lockfile, and run the full tests.

The Rust Book, chapter 12, is this lesson's central reading: it organizes a command-line program around input, configuration, errors, and separation of responsibilities. Also revisit the Rustlings exercises that gave you the most trouble on `Result`, tests, and iterators. The goal is no longer to memorize syntax; it is to see how the decisions from the earlier lessons survive when the program has real users, files, and network.

## The concepts

### The binary is a boundary, not the place for all the logic

A binary has a specific job: to translate the outside into the inside of the program. It reads the operating system's arguments, decides what gets printed, converts the general result into an exit code, and delegates the rest. If you mix YAML loading, HTTP requests, table formatting, and the exit code in there, the tests end up depending on the terminal and on `std::process::exit`. That makes a small test slow, fragile, and hard to diagnose.

The `revisor`'s alternative is to have a library with public modules (`config`, `modelo`, `reporte`, and `revisar`) and a short `main.rs`. The library can be tested from its unit tests and from `tests/integracion.rs`. The binary keeps only what necessarily depends on the command line. This division isn't a ceremonial rule: it reduces the number of places where an interface change can break behavior.

Before using `clap`, it helps to understand the work it automates. An argument parser must keep state: each flag consumes, when applicable, the value that follows; it must keep default values; and it must reject an unknown flag instead of silently interpreting it. The following program simulates that boundary without depending on crates.

**Fig. 8.1** | A minimal parser keeps default values and converts text into types.

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

The program illustrates why it isn't a good idea to write this parser by hand for every binary. It generates no `--help`, doesn't document its options by itself, doesn't validate allowed values, and its code would grow quickly. `clap` declares the contract and generates much of that machinery. In the `revisor`, `Args` is the input contract: `archivo`, `formato`, and `paralelo` have a long name, a short name, a type, and a default value.

Figure 8.1 built the argument parser by hand. Now, the same contract with `clap`, the crate the `revisor` uses. Like the other examples with dependencies, it lives in `programas/revisor/examples/` and runs with Cargo from that folder.

**Cargo example with `clap`** | The same argument contract, declared instead of programmed.

<!-- verificar:ejemplo:ejemplo_clap -->
```rust
// ejemplo_clap.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}

fn main() {
    // `try_parse_from` recibe los argumentos como una lista, en vez de leer
    // los del sistema operativo: así el ejemplo corre igual en cualquier máquina.
    let a = Args::try_parse_from(["revisor", "--formato", "json", "-p", "2"])
        .expect("estos argumentos son válidos");
    println!("{a:?}");

    // Un valor que no es número: clap lo rechaza antes de que el programa arranque.
    if let Err(e) = Args::try_parse_from(["revisor", "--paralelo", "muchos"]) {
        println!("{:?}", e.kind());
        println!("{}", e.to_string().lines().next().unwrap_or(""));
    }

    // Una opción que no existe.
    if let Err(e) = Args::try_parse_from(["revisor", "--velocidad", "3"]) {
        println!("{:?}", e.kind());
    }
}
```

```bash
$ cargo run --example ejemplo_clap
Args { archivo: "servicios.yaml", formato: "json", paralelo: 2 }
ValueValidation
error: invalid value 'muchos' for '--paralelo <PARALELO>': invalid digit found in string
UnknownArgument
```

`#[derive(Parser)]` writes for you the code that converts the list of arguments into an `Args`. In the real program it is called `Args::parse()`, which reads the operating system's arguments; here `try_parse_from` is used, which receives the list as a parameter, so that the example always gives the same output and so you can see the errors without ending the program. The first line shows a complete `Args`: what was passed (`formato` and `paralelo`) and the default value of what was missing (`archivo`). The next ones show what happens with a value that isn't a number and with an option that doesn't exist: `clap` rejects them with a clear error kind (`ValueValidation` and `UnknownArgument`) before the program's logic starts. When you use `parse()` instead of `try_parse_from`, `clap` prints that message with the usage help and ends with code 2, the same code the `revisor` reserves for "I couldn't start". In addition, `--help` and `--version` come for free from the `#[command(version, about = ...)]` attribute.

This is what the complete declaration looks like inside the `revisor`:

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

The `#[derive(Parser)]` attribute generates an implementation of the `Parser` trait. That's why `Args::parse()` can read `std::env::args()` and build a typed `Args`. `default_value` receives text because `clap` converts it to the field's type; `default_value_t` directly receives a Rust value, which is why it is appropriate for `usize`.

The project's `main` doesn't call `std::process::exit`. It returns `ExitCode`, which is easier to test when the logic stays in `ejecutar`. The code first checks that the format is one of the two promised contracts. Then it loads and validates the configuration, creates an HTTP client, asks for the statuses, decides whether to print a table or JSON, and converts the final result into 0, 1, or 2. The order matters: there is no reason to open network connections if the YAML file isn't even readable.

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

The `#[tokio::main]` annotation creates and starts the runtime needed to be able to use `.await` in the main function. By itself it doesn't make the whole program faster. Its purpose is to run futures while the HTTP requests wait for network data, without blocking a thread for each wait.

### `serde` makes the data contract explicit

A YAML file reaches the program as text. The text doesn't know what a name is, which field is required, or which value to use when `timeout_ms` is missing. Converting it to a `Vec<Servicio>` is going from data that comes from the outside to values the compiler can check. That conversion is a trust boundary: after deserializing you still must validate the business rules that the format doesn't know.

`serde` separates two directions. `Deserialize` builds Rust values from YAML, JSON, TOML, or another format that has a compatible adapter. `Serialize` converts Rust values into an output format. The domain structure is preserved; the format around it changes. That's why the same `EstadoJson` can be generated with `serde_json`, while `Servicio` comes in with `yaml_serde`.

The following program shows a domain decision that also appears in the `revisor`'s JSON: a healthy status has an HTTP code and carries no error; a failure doesn't invent a code and does include a reason. The program builds JSON manually so the difference is visible. In the real project you must not do this by hand: `serde_json` takes care of escaping text and preserving valid JSON.

**Fig. 8.2** | The shape of the output depends on the status variant.

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

In the `revisor`, the derive declares exactly what to read or write. `Servicio` derives `Deserialize` because it is created from YAML. `EstadoJson` derives `Serialize` because it is created from the internal results to produce JSON. `#[serde(default = "timeout_por_omision")]` is not the same as the field being optional in Rust: the final field is still a `u64`; it only gets a value when the YAML doesn't declare it.

The following program uses the `revisor`'s two data crates in miniature: `serde` to declare the contract, and `yaml_serde` and `serde_json` for the formats. The data comes in as YAML text written inside the program itself, so as not to depend on any file.

**Cargo example with `serde`** | The same `struct` reads YAML and writes JSON.

<!-- verificar:ejemplo:ejemplo_serde -->
```rust
// ejemplo_serde.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Servicio {
    nombre: String,
    url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    timeout_ms: u64,
}

fn timeout_por_omision() -> u64 {
    5000
}

fn main() -> Result<(), yaml_serde::Error> {
    let yaml = "\
- nombre: catalogo
  url: http://localhost:8080
- nombre: pagos
  url: http://localhost:8081
  timeout_ms: 250
";
    // YAML adentro: el mismo derive sirve para leer...
    let servicios: Vec<Servicio> = yaml_serde::from_str(yaml)?;
    for s in &servicios {
        println!("{} espera {} ms", s.nombre, s.timeout_ms);
    }

    // ...y para escribir JSON, que es otro formato con el mismo modelo.
    match serde_json::to_string(&servicios) {
        Ok(json) => println!("{json}"),
        Err(e) => println!("no se pudo escribir el JSON: {e}"),
    }

    // Un servicio sin `url` no es un Servicio: el error dice qué falta y dónde.
    let roto: Result<Vec<Servicio>, _> = yaml_serde::from_str("- nombre: sin-url\n");
    if let Err(e) = roto {
        println!("YAML inválido: {e}");
    }
    Ok(())
}
```

```bash
$ cargo run --example ejemplo_serde
catalogo espera 5000 ms
pagos espera 250 ms
[{"nombre":"catalogo","url":"http://localhost:8080","timeout_ms":5000},{"nombre":"pagos","url":"http://localhost:8081","timeout_ms":250}]
YAML inválido: .[0]: missing field `url` at line 1 column 3
```

The same `struct Servicio` serves for reading and for writing because it derives both halves of `serde`: `Deserialize` to build it from YAML and `Serialize` to write it as JSON (the `revisor`'s `Servicio` only derives `Deserialize`, because it is never written back). Notice two details. The `catalogo` service doesn't declare `timeout_ms` in the YAML and still comes out with 5000: that's the effect of `#[serde(default = ...)]`. And the invalid YAML doesn't bring the program down with a panic: `from_str` returns an `Err` whose message says what is missing (``missing field `url` ``) and where (`.[0]` is the first element of the list; `line 1 column 3`, the position in the text). That message is the one the `revisor` shows to whoever is fixing their file. And since `yaml_serde` is the maintained continuation of `serde_yaml` (the original crate no longer receives changes, as you saw in lesson 6), everything done here with `from_str` works the same with either name.

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

`Option<u16>` represents an important difference: if there was no HTTP response, there is no HTTP code. Using `0` or an empty string would invent a datum and force every consumer to remember that convention. In JSON, `Option::None` becomes `null` for `codigo`. For `error`, the `skip_serializing_if` attribute picks another policy: when there is no error, the key doesn't even appear. Both decisions are valid, but they must be intentional and covered by tests.

Deserializing well isn't enough. YAML can express an empty list, repeat a name, accept any string as a URL, or include a timeout of zero. The parser doesn't know that those options make the `revisor` useless. `config::validar` is the second step and expresses the program's rules, not the format's.

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

`anyhow::ensure!` returns early with an error when a condition isn't met. It is appropriate here because an invalid configuration doesn't represent a failed service: it is a condition that prevents starting the check. `with_context` in `cargar` adds the file name to a read error; the `{e:#}` format in `main` prints that chain of context together with the original cause. The result is more useful than a generic "could not open" message.

### `reqwest` turns the network into domain statuses

A monitoring program doesn't query HTTP to obtain a body and forget it; it queries to classify the status of a service. That's why the `revisar` function doesn't return `Result<Estado>`. If a server responds 500, refuses the connection, or exceeds its time limit, the `revisor`'s operation did finish: it discovered a failure and must include it in the report. Those cases become `Estado::Falla`.

Don't confuse that result with a configuration failure or a serialization error when producing the report. Those do prevent the program from doing its job and make it end with code 2. The distinction avoids two frequent mistakes: stopping the whole check because one service went down, or carrying on as if nothing happened when the file that defines which services exist couldn't be read.

To see `reqwest` without depending on any real service, the following program starts a fake server on the same machine and queries it with a `reqwest` client. The server is built with `TcpListener`, from the standard library, and answers the minimal HTTP text by hand: it responds `200` on `/sano`, `500` on `/roto`, and takes half a second to answer `/lento`. In addition, the program tries to connect to a port where nobody is listening.

**Cargo example with `reqwest`** | Four different responses from the network, seen from the client.

<!-- verificar:ejemplo:ejemplo_reqwest -->
```rust
// ejemplo_reqwest.rs
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

/// Un servidor HTTP de mentira, en esta misma máquina: /sano responde 200,
/// /roto responde 500 y /lento tarda medio segundo en contestar.
fn servidor_de_mentira() -> String {
    let escucha = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = escucha.local_addr().expect("el servidor tiene dirección");
    thread::spawn(move || {
        for conexion in escucha.incoming().flatten() {
            thread::spawn(move || atender(conexion));
        }
    });
    format!("http://{direccion}")
}

fn atender(mut conexion: std::net::TcpStream) {
    let mut pedido = [0u8; 1024];
    let n = conexion.read(&mut pedido).unwrap_or(0);
    let texto = String::from_utf8_lossy(&pedido[..n]);
    let ruta = texto.split_whitespace().nth(1).unwrap_or("/");
    let estado = match ruta {
        "/sano" => "200 OK",
        "/roto" => "500 Internal Server Error",
        "/lento" => {
            thread::sleep(Duration::from_millis(500));
            "200 OK"
        }
        _ => "404 Not Found",
    };
    let _ = write!(
        conexion,
        "HTTP/1.1 {estado}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
}

#[tokio::main]
async fn main() {
    let base = servidor_de_mentira();
    let cliente = reqwest::Client::new();

    for ruta in ["sano", "roto", "lento"] {
        let respuesta = cliente
            .get(format!("{base}/{ruta}"))
            .timeout(Duration::from_millis(200))
            .send()
            .await;
        match respuesta {
            Ok(r) => println!("{ruta:<6} responde {}", r.status().as_u16()),
            Err(e) if e.is_timeout() => println!("{ruta:<6} se acabó el tiempo de espera"),
            Err(_) => println!("{ruta:<6} no responde"),
        }
    }

    // Un puerto donde nadie escucha: la conexión misma falla.
    let vacio = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = vacio.local_addr().expect("tiene dirección");
    drop(vacio);
    match cliente.get(format!("http://{direccion}/")).send().await {
        Ok(r) => println!("vacío  responde {}", r.status().as_u16()),
        Err(e) if e.is_connect() => println!("vacío  no responde (rechazó la conexión)"),
        Err(e) => println!("vacío  falló de otra forma: {e}"),
    }
}
```

```bash
$ cargo run --example ejemplo_reqwest
sano   responde 200
roto   responde 500
lento  se acabó el tiempo de espera
vacío  no responde (rechazó la conexión)
```

Each line of the output is one of the cases that the `revisor` converts into a domain status. A `500` isn't a `reqwest` error: the request was made and the server answered, so `send().await` returns `Ok` with the code `500`, and it is the program that decides what that means. A timed-out request (`.timeout(...)` of 200 ms against a server that takes 500) and a refused connection, on the other hand, do arrive as `Err`, and the error itself tells which of the two cases it is with `is_timeout()` and `is_connect()`. The client is created only once with `reqwest::Client::new()` and reused for every query; the time limit, on the other hand, is set per request. The real project's `revisar` function is this same idea, with the statuses `Ok`, `Lento`, and `Falla` in place of lines of text:

<!-- verificar:extracto:src/revisar.rs -->
```rust
pub async fn revisar(cliente: &reqwest::Client, s: &Servicio) -> Estado {
    let inicio = Instant::now();
    let respuesta = cliente
        .get(&s.url)
        .timeout(Duration::from_millis(s.timeout_ms)) // .await cede el control mientras espera
        .send()
        .await;
    let ms = inicio.elapsed().as_millis() as u64;

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
}
```

The network also forces you to separate concurrency from order. The `revisor` can start several queries at the same time, but the output must be reproducible and must relate each status to its correct service. `join_all` preserves the order of the input futures, even if the responses arrive in another order. The semaphore limits how many queries enter the active section; it doesn't determine the final order of the `Vec<Estado>`.

The next figure uses neither the network nor any crate, so that anyone can compile it with a bare `rustc` and always get the same output. Instead of HTTP, it represents the semaphore's policy: with a limit of 2, turns are handed out two at a time. In the real project each turn lives until the request finishes and the runtime wakes the task when the network responds.

**Fig. 8.3** | A concurrency limit splits the pending items into batches without changing their order.

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

The real function creates an `Arc<Semaphore>` because each future needs to share the same counter of turns. `Arc` allows shared ownership between tasks; `Semaphore` hands out a temporary permit; and the `_turno` variable keeps that permit until `revisar` finishes. The leading underscore avoids a compiler warning, but the value isn't discarded: its destructor returns the permit when it leaves the block.

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

The `paralelo` parameter has a nuance: the program accepts `0` as "use the default value". It is an interface decision; it doesn't mean Tokio can run zero requests. The `PARALELO_POR_OMISION` constant keeps that rule close to the logic that uses it.

The individual query creates a per-service limit with `.timeout(Duration::from_millis(s.timeout_ms))`. That limit doesn't replace the semaphore. The timeout decides how long a request can wait; the semaphore decides how many requests can be waiting at once. Without a timeout, a turn can stay occupied for too long. Without a semaphore, a huge list can open too many connections at the same time. Both limits protect different resources.

`reqwest::Client::new()` is created once and lent to all the queries. Don't build a client per service: a client can keep and reuse connections, while a new one per call loses that advantage and adds unnecessary work. The shared borrow `&reqwest::Client` is enough because the client's requests don't require an exclusive mutable reference.

### The report is a stable interface for people and programs

A table is for reading in a terminal. JSON is for another program to consume results without having to guess columns, spaces, or alignment. It isn't a good idea to use the table as an integration format: if tomorrow you change its width, a script that processes it with `awk` can break even though the information is the same. Nor is it a good idea to print human diagnostics on standard output when JSON was requested, because it would stop being valid JSON.

The `revisor` uses `stdout` for the report and `stderr` for startup errors. That convention lets you redirect only the report to a file:

```bash
cargo run -- --archivo servicios.yaml --formato json > estado.json
```

If the configuration is valid, `estado.json` contains only JSON. If something prevents startup, the message appears in the terminal through `stderr`; it isn't mixed into a format that another process expects to parse.

The `json` function starts from services and statuses, sorts them by name, and builds a collection of `EstadoJson`. Sorting before serializing isn't required for JSON to be valid, but it does help make the result stable. A stable result compares better in tests, code reviews, and automations.

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

The `match` expressions force you to decide what each variant represents. `NoIntentado` has no duration or real code, so it is expressed as `ms: 0`, `codigo: null`, and an explicit error. `Falla` keeps its elapsed time because knowing that a connection exhausted its limit at 5,000 ms is useful information. `Ok` and `Lento` have a code; both are healthy for the exit code, although the report differentiates them.

This part reveals a useful difference with Go. In Go you can build structs with `json:"..."` tags using only `encoding/json`; for YAML you normally add another library with its own tag convention. In Rust, `serde` centralizes the serialization definition and lets the format adapters work on the same derive. In exchange, you must bring in external crates and understand which parts of the contract live in the attributes and which in the explicit validation.

### The release profile prepares an artifact; it doesn't replace verification

`cargo run` and `cargo test` use the development profile by default. That profile favors fast compilation while editing and keeps information useful for debugging. A distributable binary is built with `cargo build --release`; then Cargo applies the `release` profile defined by the project.

The profile doesn't turn an incorrect program into a correct one. First the tests, the formatting, and Clippy must pass. Then the profile decides the balance among binary size, compile time, optimization, and the diagnostics available if a panic occurs. In this project size was chosen: symbols are stripped, the build is optimized for size, cross-module optimization is enabled, a single code generation unit is used, and panics abort the process.

<!-- verificar:extracto:Cargo.toml -->
```toml
[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila
```

Each option has a cost. `lto = true` and `codegen-units = 1` can increase compile time because they allow optimizing with more global information. `panic = "abort"` shrinks the binary and avoids stack unwinding, but it gives up the possibility of cleaning up through unwinding and offers less context if a panic reaches production. For this command-line tool it is a reasonable choice; it isn't a universal recipe for every library.

Don't declare that Rust or Go "wins" on binary size without measuring on your machine and with the same functionality. The Rust binary can grow by including Tokio, Reqwest, TLS, and their transitive dependencies; the Go one also includes its runtime and depends on its compile options. A valid comparison notes the platform, the architecture, cold or incremental build, the profile used, and which dependencies each program includes.

Cross-compilation leaves another practical difference. Go usually lets you select the platform with variables like `GOOS` and `GOARCH`. Rust requires installing the corresponding target and, depending on the target, also having a linker and compatible libraries available. For a static Linux binary based on musl, the first step would be:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

This doesn't mean cross-compilation is impossible in Rust; it means you must model the target as part of the build environment. Verify the resulting binary on the system where it will be used, especially if it includes TLS or if you switch between Linux libc implementations.

## The error you will see

The argument parser must convert text into typed values. If you try to assign text directly to a `usize`, Rust doesn't guess that you want to interpret the text as a number. The following program fails before running.

**Fig. 8.4** | Text isn't a number even if it represents a command-line option.

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

`E0308` means that two types don't match. The `: usize` annotation establishes what the expression on the right must produce, but `"cinco"` is a `&str`. The fix isn't to remove the type so that it compiles with a string: `paralelo` must remain a number because it controls a concurrency limit. You must convert with `.parse::<usize>()` and decide what to do if the conversion fails.

In the minimal program that decision appears as `Result<Args, String>`. In the real project, `clap` performs the conversion of the declared fields and shows a usage diagnostic when it can't build the expected type. Afterward `main` still validates the application's own rules: that `formato` is `tabla` or `json`, that the YAML has services, and that each service's values make sense.

Another frequent mistake is to confuse an unsuccessful HTTP response with an error that must be propagated through `?`. In this program, an HTTP 500 becomes `Estado::Falla`, is printed, and gives exit code 1. Propagating it as a startup error would make one downed service hide the results of the others. The `?` operator is still correct for failures that prevent continuing, like not being able to read the YAML or not being able to serialize the JSON report.

## What goes wrong

### Putting `unwrap()` on the program's boundaries

`unwrap()` is acceptable in a test when you prepare literal data that you control. It is not acceptable for arguments, YAML files, or the network. A badly written argument must not produce a panic with a backtrace; it must give a message saying which option is invalid and end with code 2. A missing file isn't a programming surprise: it is a usage condition that the binary must report with the file name.

The useful rule is to ask who controls the data. If the test controls it, `expect` can make a failure clear. If a person, a file, the operating system, or the network controls it, turn it into a contextual error or a domain status.

### Creating a `reqwest::Client` for each service

Creating the client inside `revisar` seems local and simple, but it loses connection reuse and makes the client's configuration scatter. The project creates a single `reqwest::Client` after the configuration has been validated and lends it to all the queries. That way the client's lifecycle matches the lifecycle of one run of the `revisor`.

Don't confuse sharing the client with sharing mutable state without control. The client is used through shared references, and the semaphore protects the resource that does need to be limited: how many requests are active.

### Using JSON built with `format!` in production

The JSON figure shows the shape of the data, not a safe serialization technique. If a service's name contains quotes, a backslash, or a newline, a hand-built string can stop being valid JSON or change its meaning. `serde_json` escapes that data correctly and preserves the relationship among `Option`, `null`, and omitted fields.

A good rule is that structured formats should be built with a serializer and tested by reading them back. In the project's tests, the JSON is converted to `serde_json::Value` before checking specific fields.

### Treating the exit code as an invisible detail

A table with the word `FALLA` helps a person, but a scheduled job needs to know whether it should raise an alert. If the `revisor` always ends with 0, a cron job, a pipeline, or a supervisor may believe everything is healthy even though there are downed services. If it ends with 1 for any problem, it doesn't distinguish between a monitored outage and a bad configuration either.

Keep the contract simple and documented: 0 for a healthy check, 1 for failed services, and 2 for startup or usage errors. The binary's tests must verify those three paths.

### Measuring the Rust binary against the Go one without controlling the experiment

Comparing only the size of two files is a poor way to draw conclusions. The result changes if one includes TLS, if the other uses an external HTTP library, if one was compiled in development and the other in release, if symbols were stripped, or if the system compresses executables. The time also changes between a cold build, an incremental build, and a rebuild after touching one line.

Measure with the same use case and record the conditions. Sometimes Go will be the practical choice because cross-compilation and the standard library cut steps. Sometimes Rust will be preferable because you want to express certain guarantees before running and accept a more expensive compilation. The purpose of doing both courses isn't to repeat a slogan; it is to have your own evidence.

## Exercises

### Exercise 1 — Declare an additional option

Add to a standalone program a `--silencioso` option that is false by default. When it is true, the program must print only the numeric code of each status; when it is false, it must print name, status, and code. Make an unknown flag produce an `Err` with a clear message.

Then explain what you would declare in `clap`: a `bool` field, an option with `#[arg(long)]`, and the default behavior that its type already expresses.

### Exercise 2 — Separate invalid data from failed services

Write a function that receives a list of minimal services with `nombre`, `url`, and `timeout_ms`. It must return an error if the list is empty, if a name is repeated, if the URL doesn't start with `http://` or `https://`, or if the time limit is zero. Separately, represent an HTTP 503 as a status variant, not as a validation error.

Use the rules of `config::validar` as a reference, but write your test cases first: a valid list, an empty one, one with a duplicate name, and one with a zero timeout.

### Exercise 3 — Test the binary's contract

Without opening `tests/binario.rs` yet, write an end-to-end test for the invalid format `-f xml` (on a copy of the project, or under another name if you work on the original). It must check that the exit code is 2, that there is no table on standard output, and that the error output names `--formato`.

Then run only that test and then the whole suite. Don't change the project's code to make a badly written test pass: the test must describe the contract that `main.rs` already exposes. When you finish, open `tests/binario.rs`: the project already includes a test for this contract and it is the reference solution; compare what each one verifies.

### Exercise 4 — Compare the two implementations

Build the Rust `revisor` and the equivalent Go program in release mode. In a logbook, note for both: lines of your own code, direct dependencies, cold build time, binary size, memory usage under the same configuration, and the time it took you to complete the program.

Answer with one well-grounded sentence: which one you would deliver for an internal tool with a close deadline, which for a tool that must be maintained for years, and which concrete decision from each language led you to that conclusion.

## Solutions

### Solution 1

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

The option doesn't change the domain status; it changes the presentation. That's why it must be applied close to the output boundary, not inside `Estado` or the HTTP logic. With `clap`, a `bool` marked with `#[arg(long)]` expresses that the absence of the flag is `false` and its presence is `true`.

### Solution 2

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

The invalid timeout is an input problem: the check must not start. A 503 code, on the other hand, is information obtained while checking and belongs to the service's status. The separation lets the binary end with 2 in the first case and with 1 in the second.

### Solution 3

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

The test runs the real binary, which is why it checks its three observable outputs: code, standard output, and error output. It doesn't need to know private functions in `main.rs`; it only knows the contract that whoever runs `revisor -f xml` will see.

### Solution 4

The comparison must begin with reproducible commands:

```bash
cd programas/revisor
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

Then repeat the experiment with the Go program, on the same computer and with the same configuration of services. Don't copy other people's results as if they were universal. The value of the exercise is to identify which part of the time was compilation, which part was learning the ecosystem, and which part was solving the same design problem.

## How do I know I got it

- `cargo run -- --help` shows the options `--archivo`, `--formato`, and `--paralelo`.
- `cargo run -- --archivo archivo-que-no-existe.yaml` ends with code 2 and writes the file name to the error output.
- `cargo run -- --formato xml` ends with code 2 and explains that the valid formats are `tabla` and `json`.
- `cargo run --example ejemplo_clap`, `ejemplo_serde`, and `ejemplo_reqwest` print the same thing this lesson documents, and `herramientas/verificar-ejemplos.sh` finishes without errors.
- `cargo test` finishes with correct results for the library, integration, and binary.
- `cargo clippy --all-targets -- -D warnings` finishes without warnings.
- `cargo fmt --check` finishes with no pending changes.
- `cargo build --release` creates the binary at `target/release/revisor`.
- You can explain why an HTTP 500 produces exit code 1, while an invalid YAML produces exit code 2.

## Further reading

- [The Rust Programming Language, chapter 12: An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) — accessed October 2, 2026.
- [Official Cargo reference: profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) — accessed October 2, 2026.
- [Documentation of `clap`](https://docs.rs/clap/latest/clap/) — accessed October 2, 2026.
- [Documentation of `reqwest`](https://docs.rs/reqwest/latest/reqwest/) — accessed October 2, 2026.
