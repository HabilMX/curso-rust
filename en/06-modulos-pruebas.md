# Lesson 6 — Modules, Tests, and Cargo

**Time:** 2 × 45 min.

**What you build:** the `revisor` project, organized and tested.

**What you learn:** modules and visibility, unit and integration tests, `cargo test`, dependencies and versions.

**The Rust Book, chapters 7, 11, and 14.** Rustlings: `modules`, `tests`.

## By the end you will be able to

- Split a Rust program into modules with clear responsibilities and navigate their paths with `crate`, `self`, and `super`.
- Explain why everything is private by default and choose between `pub`, `pub(crate)`, and a private API.
- Tell a unit test from an integration test and know which kind of problem each one detects.
- Write tests with `#[test]`, `assert!`, `assert_eq!`, `matches!`, and `#[should_panic]`.
- Run, filter, and diagnose tests with `cargo test`.
- Read `Cargo.toml` and `Cargo.lock`, add a dependency with a reasonable version, and inspect its transitive tree.
- Keep the `revisor` verifiable with `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`.

## The why before the how

So far the `revisor` has been able to fit in a few files because the course was presenting the pieces of the language one by one. You already know the types that model a service, the collections that hold the list, the errors that describe external failures, and the traits that express contracts. The next problem is not writing another function: it is keeping those functions from turning into a single mass that is hard to read, test, and change.

A huge file doesn't automatically stop working. The problem appears when a seemingly local change forces you to understand too many things at once. If the code that reads YAML, validates services, makes HTTP requests, produces JSON, and parses arguments lives mixed together, a formatting test can end up requiring the network; a configuration change can affect the binary; and a private function can end up being used from anywhere just because nobody defined a boundary.

Modules are those boundaries. They aren't folders meant to make the project "look tidy"; they are names for responsibilities and, in Rust, also an explicit part of access control. A module can say: "this is where we define what a service is", "this is where we load the configuration", or "this is where a status becomes a table". Whoever uses a module knows its public interface; they don't need to, and shouldn't, depend on the internal details of how it is implemented.

The important word is interface. In Go, a folder defines a package and a capital initial letter decides whether a name can cross the package boundary. Rust is more detailed. A folder can help organize files, but visibility depends on modules and on `pub`. A name without `pub` is private, even if it is in another file of the same project. This seems strict at first, but it keeps a helper function from turning by accident into a promise to the rest of the program.

The `revisor` applies this idea with two products inside the same Cargo package. `src/lib.rs` declares a library: that's where the reusable, testable logic lives. `src/main.rs` declares the binary: it receives arguments, calls the library, prints the result, and decides the exit code. Separating the two lets you test the logic without having to invoke the command line in every case. It also lets the integration tests use the `revisor` the way another application would: by importing only its public API.

This is the same principle you used in Go when splitting packages by responsibility, but Rust makes the contract more visible. In Go, a lowercase function can't be imported from another package; in Rust, a function, a struct, a field, or a module needs declared visibility. Both decisions aim to limit dependencies. Rust gives you more levels to express that intent: public for anyone who imports the library, public only within the current package, or public for a parent module.

Tests turn those boundaries into something verifiable. A unit test lives close to the function it tests and can examine private details. It is useful for small rules: if the YAML doesn't declare `timeout_ms`, is the default value applied? Is an empty list rejected? Does the table keep its alignment? An integration test lives under `tests/`, is compiled as another crate, and can only use `pub`. It is useful for checking that the interface really suffices: if someone builds a `Servicio`, calls `revisar`, and receives an `Estado`, does the public contract work without depending on internal details?

Don't confuse many tests with good coverage of decisions. A suite can have a hundred tests that repeat the same healthy case and none that covers an invalid URL, a missing file, or a service that takes too long. Don't turn the coverage percentage into an isolated goal either. The useful question is: "what important behavior could break without any test turning red?" The `revisor` tests healthy statuses, HTTP 500, timeouts, invalid configuration, JSON format, and exit codes because those are behaviors that matter to whoever uses the program.

`cargo` brings these decisions together. It doesn't just compile: it knows which files make up the package, which dependencies it needs, which Rust edition it uses, which tests exist, and which artifacts it must build. In the course figures you keep calling `rustc` directly to see an isolated example. In the real project you use `cargo` because there is no longer a reasonable manual invocation that remembers all the modules, crates, features, and test targets.

This lesson's discipline is simple: organize by responsibility, open the smallest public surface necessary, and test each boundary from the right side. If a unit test needs the network, you probably mixed a pure rule with infrastructure. If an integration test needs to import a private detail, your public API probably doesn't express what another consumer needs. If `cargo test` says it found no tests, don't congratulate yourself yet: check the count.

## The concepts

### Modules: names, paths, and responsibilities

A module groups related names. It can be declared inside a file with `mod name { ... }`, or it can live in another file. In a modern Cargo package, `src/lib.rs` and `src/main.rs` are separate crate roots. From either of them, `crate` means "the root of this crate"; `self` means the current module; and `super` means the parent module.

A common mistake is to think that file and module are synonyms. A file can contain several modules, and a module can be opened in another file. The file structure helps a person find code; the module structure determines how Rust resolves paths and applies visibility. Don't design an empty folder tree first. Start with responsibilities that have a stable reason to change separately.

**Fig. 6.1** | A module offers a public function and keeps its detail private.

```rust
// fig06_01.rs
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn linea(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::linea("catalogo", true));
    println!("{}", reporte::linea("pagos", false));
}
```

```bash
$ rustc --edition 2024 fig06_01.rs && ./fig06_01
catalogo: OK
pagos: FALLA
```

`reporte::linea` is accessible from `main` because it has `pub`. The `etiqueta` function doesn't have `pub`, so it can only be used inside `reporte`. That decision doesn't hide information out of mystery: it expresses that other parts of the program need a finished line, not to know the internal rule that translates a boolean into text. If you later change `"FALLA"` to `"NO DISPONIBLE"`, only the owning module needs to change.

In the `revisor`, the library root lists its public responsibilities. There is no module called `utilidades`, because that name doesn't explain which responsibility it owns. `config` loads and validates configuration; `modelo` defines the vocabulary; `reporte` translates statuses to text or JSON; `revisar` queries services.

<!-- verificar:extracto:src/lib.rs -->
```rust
//! El `revisor` del curso de Rust: recibe una lista de servicios, los consulta
//! todos a la vez y produce un reporte.
//!
//! La lógica vive aquí, en la biblioteca, y `main.rs` solo lee los argumentos y
//! llama (lección 6): así todo lo de abajo se puede probar desde fuera.
//!
//! - [`modelo`]: el vocabulario (`Servicio`, `Estado`, `EstadoJson`).
//! - [`config`]: lee y valida el archivo YAML de servicios.
//! - [`revisar`]: consulta un servicio por HTTP, o todos a la vez con un límite.
//! - [`reporte`]: convierte los estados en tabla o en JSON.

pub mod config;
pub mod modelo;
pub mod reporte;
pub mod revisar;
```

The word `pub` in front of each `mod` makes those modules form the public entry point of the library. That doesn't make all their contents public. Each module in turn decides which structs, functions, and constants it exposes. This composition is an advantage: publishing `reporte` lets you call `revisor::reporte::tabla`, but it doesn't force you to publish the helper functions that sort rows or compute labels.

The `revisor`'s module tree doesn't claim to be a universal hierarchy. In a small project, four flat modules are more readable than a long chain of folders. When a responsibility grows enough, it can be split into submodules. The question is not "how many files should a professional project have?" but "can I describe in one sentence what belongs here and what doesn't?".

### Visibility: private by default as a design

Rust starts closed. An item without `pub` is visible in its module and its descendants, but not to sibling modules or to the parent. This rule is more restrictive than many programmers expect after JavaScript, Python, or Go, where a file-level function is usually accessible within the package. The intent is to force you to design the interface before depending on a detail.

`pub` opens an item to whoever can reach the module that contains it. `pub(crate)` opens the item to the whole current crate, but not to someone who imports the library from another package. `pub(super)` opens the item only to the parent module. There is also `pub(in path)`, useful when a precise module boundary expresses a real rule, although it is less common in small projects.

Don't mark everything with `pub` to silence visibility errors. Doing so has a cost: any consumer can start depending on those names, and afterward changing an internal function becomes an API break. For a private binary that cost stays inside the repository; for a published library, it can force you to keep an accidental decision for years. Start private and open only what another part needs.

The compiler tells apart a name that doesn't exist from a name that exists but is closed. In this case the function exists, but `main` tries to cross a private boundary.

**Fig. 6.2** | Accessing a private function produces `E0603`.

```rust
// fig06_02.rs
mod config {
    fn ruta_por_omision() -> &'static str {
        "servicios.yaml"
    }
}

fn main() {
    println!("{}", config::ruta_por_omision());
}
```

```bash
$ rustc --edition 2024 fig06_02.rs
error[E0603]: function `ruta_por_omision` is private
 --> fig06_02.rs:9:28
  |
9 |     println!("{}", config::ruta_por_omision());
  |                            ^^^^^^^^^^^^^^^^ private function
  |
note: the function `ruta_por_omision` is defined here
 --> fig06_02.rs:3:5
  |
3 |     fn ruta_por_omision() -> &'static str {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

The mechanical fix would be to write `pub fn ruta_por_omision`. Before doing it, ask whether the default path should be part of `config`'s contract. If another module truly needs to consult it, it may be a reasonable public function. If you only want `main` to load the normal file, it may be better for `config` to offer a higher-level public function and keep that string as a private detail.

The `revisor`'s `modelo` module shows a selected public API. `Servicio` is public because the configuration, the integration tests, and other modules need to build it. Its fields are public because the program needs to read and modify the declared data. The default timeout constant, on the other hand, stays private: whoever uses `Servicio::new` gets the rule without depending on how it is stored.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;

fn timeout_por_omision() -> u64 {
    TIMEOUT_POR_OMISION_MS
}

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

Notice the difference between exposing a constant and exposing a function. `UMBRAL_LENTO_MS` is a rule that other modules do need in order to classify responses. `timeout_por_omision` exists only so that `serde` can call the default value inside the model. Publishing it gives the consumer no useful capability and does enlarge the surface that would have to be maintained.

### Unit tests: one small property, close to the code

A unit test checks a unit of behavior in its own module. Rust normally writes them inside a `tests` module marked with `#[cfg(test)]`. That attribute indicates that the module is compiled only when building the tests. The production binary doesn't load those functions or their helpers.

`use super::*` imports into `tests` the names of the parent module. That lets you deliberately test private details. It isn't a trick against visibility: the test lives as a descendant of the same module and is checking the internal implementation. An integration test will have a different restriction, because it represents an external consumer.

The main assertions are `assert!`, for a boolean condition; `assert_eq!`, for comparing expected and obtained; and `assert_ne!`, for asserting that two values are not equal. All of them accept an additional formatted message. `matches!` is especially useful with enums: it lets you check the variant and, if needed, a condition on the data it carries.

**Fig. 6.3** | Unit tests, an enum assertion, and an expected panic.

```rust
// fig06_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Falla(String),
}

fn resumen(e: &Estado) -> String {
    match e {
        Estado::Ok { codigo, ms } => format!("OK {codigo} en {ms}ms"),
        Estado::Falla(msg) => format!("FALLA: {msg}"),
    }
}

fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("dividir por cero");
    }
    a / b
}

fn main() {
    println!("{}", resumen(&Estado::Ok { codigo: 200, ms: 100 }));
    println!("{}", dividir(10, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estado_ok_con_200() {
        let e = Estado::Ok { codigo: 200, ms: 100 };
        assert!(matches!(e, Estado::Ok { .. }));
    }

    #[test]
    fn falla_sin_codigo() {
        assert_eq!(resumen(&Estado::Falla("x".into())), "FALLA: x");
    }

    #[test]
    #[should_panic(expected = "dividir por cero")]
    fn panico_esperado() {
        dividir(1, 0);
    }
}
```

```bash
$ rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1
running 3 tests
test tests::estado_ok_con_200 ... ok
test tests::falla_sin_codigo ... ok
test tests::panico_esperado - should panic ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`#[should_panic]` doesn't mean that panics are a normal way to handle invalid data. In the `revisor`, a wrong URL must end as a `Result` and a message for whoever invoked the program, not as a panic. The panic test is useful when a contract deliberately stops execution because of a broken invariant. The `expected` argument matters: it confirms that the panic came from the expected reason and not from some other accidental failure.

A test should describe behavior, not incidental implementation. The name `falla_sin_codigo` communicates a domain rule: a failure is shown without an HTTP code. A name like `prueba_resumen_2` only says that somebody wrote a test. When it fails months from now, the name will be the first clue to understand which decision of the program changed.

In `config.rs`, the unit tests don't make HTTP requests or run the binary. They build small pieces of data and call `validar`, which is the unit responsible for verifying the list. This keeps the suite fast and makes each failure point to a concrete rule.

<!-- verificar:fragmento -->
```rust
#[test]
fn validar_rechaza_nombre_repetido() {
    let v = vec![
        Servicio::new("a", concat!("http", "://x")),
        Servicio::new("a", concat!("http", "://y")),
    ];
    let err = validar(&v).unwrap_err().to_string();
    assert!(err.contains("repetido"), "mensaje: {err}");
}

#[test]
fn validar_rechaza_url_sin_esquema() {
    let v = vec![Servicio::new("a", "localhost:80")];
    assert!(validar(&v).is_err());
}

#[test]
fn validar_rechaza_timeout_cero() {
    let mut s = Servicio::new("a", concat!("http", "://x"));
    s.timeout_ms = 0;
    assert!(validar(&[s]).is_err());
}
```

The first test inspects part of the message because here the text is part of the experience of whoever fixes the YAML. The others only verify that an error exists. Not every test has to compare entire strings. Compare the exact detail when it is a public contract; for internal errors, checking the type, a condition, or the existence of the error usually produces less fragile tests.

### Integration tests: the library seen from outside

Cargo recognizes `tests/` as the place for integration tests. Each Rust file directly inside that folder is compiled as a separate crate. That's why it can't use private functions, import the library's internal `tests` module, or assume details about files. It can only use what the library exports with `pub`.

This limitation is useful. An API can have excellent unit tests and still be awkward or insufficient for someone trying to use it from outside. Integration tests find that problem because they cross the same boundary another binary would cross. If you need to break encapsulation to write them, first check whether a reasonable public operation is missing; don't make everything `pub` as an automatic reaction.

The `revisor` has a `tests/integracion.rs` file. It imports the types and functions that a public consumer needs: `Estado`, `Servicio`, `revisar`, and `revisar_todos`. It doesn't import private functions that build requests or `reqwest` details.

<!-- verificar:fragmento -->
```rust
use revisor::modelo::{Estado, Servicio};
use revisor::revisar::{revisar, revisar_todos};

fn servicio(nombre: &str, direccion: &str, ruta: &str, timeout_ms: u64) -> Servicio {
    Servicio {
        nombre: nombre.to_string(),
        url: ["http:", "//", direccion, ruta].concat(),
        timeout_ms,
    }
}
```

The `servicio` helper belongs to the test, not to the library, because it exists only to make the test cases readable. It's a healthy distinction: don't promote a function to production just because two tests repeat it. The library should contain capabilities of the program; the suite can contain small tools for setting up scenarios.

The next test starts a local HTTP server defined in `tests/comun/mod.rs`, calls the public API, and checks the resulting variant. It doesn't depend on a real service on the internet, an account, or a specific time of day. That keeps a network failure from turning a deterministic test into a false alarm.

<!-- verificar:extracto:tests/integracion.rs -->
```rust
#[tokio::test]
async fn un_500_es_falla_con_su_codigo() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let e = revisar(&cliente, &servicio("mal", &d, "/error", 2000)).await;
    assert!(
        matches!(&e, Estado::Falla { motivo, .. } if motivo == "codigo 500"),
        "estado: {e:?}"
    );
}
```

`#[tokio::test]` appears because the `revisar` function is asynchronous. Lesson 7 goes deeper into what it means to await a future and how the runtime works. What matters here is recognizing the boundary: the integration test uses the same asynchronous API that the binary will use, but replaces the internet with a controlled local server.

Besides integration tests, the project has binary tests. These run the compiled `revisor`, pass it a temporary YAML file, and check `stdout`, `stderr`, and the exit code. They are slower and broader than a unit test, so they don't replace the others; they check the last boundary, where arguments, configuration, reports, and process output meet.

### `cargo test`: building, selecting, and reading results

`cargo test` discovers the unit tests, the integration tests, the binary tests, and the documentation tests; it compiles the necessary targets and runs each set. It is more than an abbreviation for `rustc --test`: Cargo knows the dependencies and builds each crate with the right paths.

The commands you will use most often are these:

```bash
cargo test
cargo test validar_rechaza_nombre_repetido
cargo test --test integracion
cargo test -- --nocapture
cargo test --release
```

The first command runs everything. The second filters by part of the test name; it is useful for working on a single rule without waiting for the entire suite. The third specifically selects the integration file named `integracion`. The `--` separates Cargo's options from the test runner's options: `--nocapture` lets you see the `println!` output of a passing test, which is useful for temporary diagnosis, not as a substitute for an assertion. `--release` compiles with optimizations; use it when behavior really depends on the profile or when you are measuring performance, not as a daily mode.

Tests can run in parallel. That is correct if each one creates its own data and doesn't depend on execution order. If you are diagnosing output or a test shares a resource you can't isolate yet, use:

```bash
cargo test -- --test-threads=1
```

Don't turn that flag into a habit. A suite that only works serially can hide global state or temporary files with colliding names. In the `revisor`, the test servers ask the system for a free port and each case uses its own data; that lets you run tests without depending on a particular order.

The simplest trap is that `cargo test` can finish successfully without running a relevant test. A badly written filter can produce output with filtered-out tests; a crate may have no `#[test]` at all; and a file placed outside `tests/` may not be an integration test. Always read the `running N tests` and `test result` lines. An exit code of zero means the runner found no failure, not that your intent was verified.

The project keeps the logic in the library and the startup in the binary. The binary imports the public API like any other internal consumer. This separation is the reason why the integration tests can import `revisor` under the same name.

<!-- verificar:extracto:src/main.rs -->
```rust
use std::process::ExitCode;

use revisor::{config, reporte, revisar};

use clap::Parser;
```

The path `revisor::{config, reporte, revisar}` doesn't use `crate::` because `main.rs` is another crate inside the same package. From the binary's perspective, `revisor` is the library declared by `src/lib.rs`. It's a small syntax difference with an important design consequence: the binary has no privileges to reach private details of the library.

### Dependencies, versions, and the work of `cargo`

`Cargo.toml` is the package's declarative manifest. It says what the package is called, which edition it uses, which direct dependencies it needs, and which build profiles exist. `Cargo.lock` records the concrete resolution: the exact versions of direct and transitive dependencies that Cargo chose when it built the project.

The `revisor` doesn't depend only on the standard library. That's deliberate: YAML, asynchronous HTTP, JSON, and a complete command line live in specialized crates. The declared dependencies are the ones the project actually uses.

<!-- verificar:extracto:Cargo.toml -->
```toml
[dependencies]
anyhow = "1.0.104"
clap = { version = "4.6.7", features = ["derive"] }
futures = "0.3.34"
reqwest = { version = "0.13.5", features = ["json"] }
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
yaml_serde = "0.10.7"
tokio = { version = "1.53.1", features = ["full"] }
```

A note about one of those lines. Until 2024, the most widely used crate for reading YAML with `serde` was `serde_yaml`. Its author, David Tolnay, stopped maintaining it: its last version is `0.9.34+deprecated`, from March 2024, and crates.io marks it as deprecated. It still compiles and works, but it no longer receives fixes or improvements, so it is not a good idea to start a new project with it. The `revisor` uses `yaml_serde`, a continuation published by the YAML organization on GitHub: its repository presents it as the maintained fork of `serde_yaml` and promises the same interface. That's why the change barely touches the code: what you know about `serde_yaml::from_str` works the same with `yaml_serde::from_str`. Other forks and alternatives exist; before choosing one, look at the date of its latest release and whether its repository still receives changes. (Data checked on crates.io on October 2, 2026.)

A version like `"1.0.104"` doesn't by itself pin every digit forever. In Cargo, that specification uses semantic compatibility with an implicit caret operator: it allows compatible updates within the same major version. `Cargo.lock` is what makes the binary's concrete build repeatable. That's why the `revisor`'s lockfile must go into the repository: someone who clones the application should resolve the same known versions, not a new combination that looks compatible today.

For a published library, the answer is less clear-cut. `cargo new` adds the `Cargo.lock` to the repository by default, and Cargo's frequently asked questions (the [Cargo FAQ](https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control)) say that whether to commit it depends on what your package needs. Committing it gives repeatable builds: it helps you find with `git bisect` which change introduced a bug, makes continuous integration fail only because of new commits and not because of a dependency that changed outside, and lets you verify with known versions things like the minimum Rust version or the exact text of error messages. But that file doesn't protect whoever uses your library: consumers resolve dependencies with what your `Cargo.toml` declares and with their own `Cargo.lock`, and `cargo install` by default ignores the package's `Cargo.lock` and picks the most recent compatible versions, unless you pass `--locked`. In short: an application like the `revisor` should always be committed, because it is the final product you want to reproduce; for a library, decide according to what you want to guarantee, and if you don't commit it, test from time to time with the newest dependencies.

Add a dependency with Cargo instead of hand-writing a line you don't understand:

```bash
cargo add serde --features derive
cargo add tokio --features full
cargo tree
cargo update
```

`cargo add` updates the manifest and resolves the lockfile. Features activate optional parts of a crate. `serde` needs `derive` so that `#[derive(Serialize, Deserialize)]` exists; `tokio` needs runtime, network, and macro capabilities for the current program. Don't enable `full` by reflex in a new project if you only need a small part; here it is a conscious decision of the course so that the revisor uses the capabilities it teaches.

`cargo tree` shows the full tree. It is how you discover transitive dependencies: crates you didn't add directly, but that arrived because another dependency needs them. It is not necessarily a sign of a problem. It is a tool for answering "who brings in this version?", "why is so much code being compiled?", or "why are there two versions of this crate?".

`cargo update` updates within the constraints you wrote in `Cargo.toml`. It isn't equivalent to "install the latest version of everything" without limits. Before updating a stable project, review what changed in the lockfile, run the tests, and read the release notes when a core dependency changes. The declared version defines the acceptable range; the lockfile records the decision made.

During development, `cargo check` is usually faster than `cargo build` because it verifies types and borrows without generating the final executable. It doesn't replace tests, but it reduces feedback time while you edit a function. To maintain quality across the whole project, use this sequence:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo fmt --check` confirms formatting without modifying files. `cargo clippy --all-targets -- -D warnings` reviews the library, the binary, and the tests, and turns its warnings into failures so the project doesn't accumulate known debt. `cargo test` verifies behavior. They are three distinct signals: consistent formatting, idiomatic usage, and expected behavior.

## The error you will see

### `E0603`: the name exists, but it isn't part of the API

`E0603` appears when Rust found the item you named, but the path tries to cross a private boundary. The diagnostic in figure 6.2 gives three clues: it points at the illegal use, says the function is private, and points to where it was declared. That is different from a typo error like "this function was not found"; here Rust does know which function you wanted to use.

The fix depends on the design. If the function should be part of the interface, declare it `pub`. If it only needs to serve a sibling module, consider moving the operation to a more appropriate owning module or exposing a higher-level public function. If you need to limit it to the crate, `pub(crate)` better expresses that nobody outside the package should depend on it.

In the `revisor`, the helpers in `reporte.rs`, such as `etiqueta`, `tiempo`, and `detalle`, remain private. The public API is `tabla` and `json`, because those are the operations that the binary and any consumer can ask for. The choice keeps integration tests or future code from depending on an internal label and freezing an accidental formatting decision.

### A red test is not a compile error

When an assertion fails, Cargo compiles correctly and then the runner marks the test as failed. There will be no `EXXXX` code, because it isn't a violation of the compiler's static rules. You will see the name of the test, the left and right values if you used `assert_eq!`, and any extra message you added.

This is an important difference in diagnosis. `rustc` errors tell you that the program can't be built under the rules of the language. A red test tells you that the program was built, but it broke the behavior you declared. Don't fix a red test by removing the assertion or changing the expected value without reviewing which contract it was supposed to uphold.

Cause a failure on purpose once. In the `falla_sin_codigo` test, temporarily change the expected text to `"OK: x"` and run the corresponding filter. You should see the test turn red. Then restore the correct behavior. A test you have never seen fail may be covering a different branch than you think, or may assert something too weak to detect a regression.

## What goes wrong

### Making everything `pub`

Opening every struct, field, and function often starts as a quick way to beat `E0603`. The result is a library without boundaries: any module can lean on internal details, and each change requires reviewing many more calls than necessary. Publish operations that represent domain capabilities, not every auxiliary step you use to implement them.

The practical alternative is not to guess the perfect API from day one. Keep private what doesn't yet have a clear consumer. When another module needs an operation, open the minimal interface and let that real use guide the design.

### Organizing under vague names like `utils` or `helpers`

A folder called `utils` doesn't describe a responsibility; it describes that someone didn't know where to put something. Over time it accumulates text conversion, file access, formatting, HTTP, and functions nobody dares to move. Finding code gets slower and the dependency between modules becomes arbitrary.

In the `revisor`, a YAML rule lives in `config`, a status translation lives in `reporte`, and the domain vocabulary lives in `modelo`. If a function doesn't fit in any module, first ask whether a concept with its own name is missing. Often the new name reveals a responsibility that was mixed in.

### Testing only the healthy path

A test that checks `200 OK` is necessary, but it isn't enough for a service checker. An HTTP 500, an invalid URL, a missing file, a timeout, an empty list, and an unknown format must also be covered. Errors aren't improbable exceptions in this domain: they are part of what the program exists to report.

Don't turn every external failure into a real network test. The `revisor` uses a fake local server to reproduce known responses. That way it tests its own behavior, not the availability of someone else's service.

### Using `unwrap()` to write shorter tests

`unwrap()` is reasonable for preparing data that the test itself controls, like literal YAML that must be valid. If that YAML fails, the test is badly built and stopping is correct. Don't use it on the result you are trying to test. If you want to show that `validar` rejects an input, use `is_err`, `unwrap_err`, or `matches!` depending on the contract.

The rule is to distinguish setup from verification. In setup, an `expect("el YAML de la prueba es válido")` gives useful context. In verification, an assertion expresses exactly the property you want to uphold.

### Trusting the exit code of `cargo test` without reading the count

A filter with no matches can return success because no tests failed. A new crate can compile without tests. A misplaced integration test may never be discovered. Read `running N tests`, the names that appear, and the final summary. The useful result is not just "it exited with zero"; it is "the test I expected ran and passed".

### Updating dependencies without reviewing the lockfile

`cargo update` can change several transitive dependencies even if you asked for only one update. That doesn't make it dangerous in itself, but it does call for review. Look at the change in `Cargo.lock`, understand which crates were updated, and run the full suite. A version that is compatible in theory can reveal a fragile assumption or change compile times significantly.

## Exercises

### Exercise 1 — Split a report without opening too much

Create a program with a `reporte` module. It must expose a public function `resumen(nombre, sano)` that returns a `String` with the name and the label `OK` or `FALLA`. The function that decides the label must stay private. From `main`, print two lines: a healthy one and a failing one.

### Exercise 2 — Test every status variant

Write a function `es_sano(&Estado) -> bool` for the four variants of the revisor's `Estado`: `Ok`, `Lento`, `Falla`, and `NoIntentado`. Add one test per variant. Use `assert!` or `assert!(!...)` and name each test after the rule it checks.

### Exercise 3 — An integration test that doesn't know internal details

In `programas/revisor`, read `tests/integracion.rs`. Add an integration test that uses only `revisor::modelo` and `revisor::revisar`. It must use the shared local server and check that `revisar_todos` returns the same number of statuses as services, even when one receives HTTP 500. Write it before reading the tests that `tests/integracion.rs` already includes, and then compare: what does yours check that the others don't?

### Exercise 4 — Make a test red and leave it green again

Choose an existing test in `config.rs` or `reporte.rs`. Temporarily change an expectation so that it fails, run only that test with `cargo test nombre_de_la_prueba`, read the diagnostic, and restore the correct behavior. Finally run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`.

## Solutions

### Solution 1

<!-- verificar:fragmento -->
```rust
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn resumen(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::resumen("catalogo", true));
    println!("{}", reporte::resumen("pagos", false));
}
```

`etiqueta` doesn't need `pub` because only `resumen` uses it. The public function delivers the result that `main` needs, not the intermediate detail.

### Solution 2

<!-- verificar:fragmento -->
```rust
#[derive(Debug)]
enum Estado {
    Ok,
    Lento,
    Falla,
    NoIntentado,
}

fn es_sano(estado: &Estado) -> bool {
    matches!(estado, Estado::Ok | Estado::Lento)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_es_sano() {
        assert!(es_sano(&Estado::Ok));
    }

    #[test]
    fn lento_es_sano() {
        assert!(es_sano(&Estado::Lento));
    }

    #[test]
    fn falla_no_es_sana() {
        assert!(!es_sano(&Estado::Falla));
    }

    #[test]
    fn no_intentado_no_es_sano() {
        assert!(!es_sano(&Estado::NoIntentado));
    }
}
```

The four tests aren't redundant. The function contains two groups of variants and each one expresses a domain decision. If someone changes `matches!` in an incomplete way, at least one test identifies which status lost its meaning.

### Solution 3

<!-- verificar:fragmento -->
```rust
#[tokio::test]
async fn revisar_todos_conserva_un_estado_por_servicio() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let servicios = vec![
        servicio("bien", &d, "/ok", 2000),
        servicio("mal", &d, "/error", 2000),
    ];

    let estados = revisar_todos(&cliente, &servicios, 2).await;

    assert_eq!(estados.len(), servicios.len());
    assert!(estados[0].esta_bien());
    assert!(!estados[1].esta_bien());
}
```

The test uses only public types and functions of the `revisor`. The local helper builds the services; the shared server controls the responses. It doesn't require opening any private function of the HTTP client.

### Solution 4

First run a specific test, for example:

```bash
cargo test validar_rechaza_timeout_cero
```

Temporarily change `assert!(validar(&[s]).is_err())` to `assert!(validar(&[s]).is_ok())`. The test must fail. Restore `is_err()` and finish with:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The solution is not to keep the change that makes the test pass; it is to check that the suite detects a modification that breaks the rule and then recover the correct rule.

## How do I know I got it

- `rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1` prints `running 3 tests` and ends with `3 passed; 0 failed`.
- `cargo test validar_rechaza_nombre_repetido` ends with the test `config::tests::validar_rechaza_nombre_repetido ... ok`.
- `cargo test --test integracion` runs the tests that import only the library's public API.
- `cargo fmt --check` finishes with no pending formatting changes.
- `cargo clippy --all-targets -- -D warnings` finishes with no warnings.
- `cargo test` finishes with `ok` results for the library, the binary, and the integration tests.
- You can explain why `main.rs` imports `revisor::{config, reporte, revisar}` and not private details of `src/lib.rs`.

## Further reading

- [The Rust Programming Language, chapter 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) — accessed October 2, 2026.
- [The Rust Programming Language, chapter 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html) — accessed October 2, 2026.
- [The Rust Programming Language, chapter 14: More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) — accessed October 2, 2026.
- [Official Cargo reference: specifying dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) — accessed October 2, 2026.
- [Tests that actually catch errors: beyond coverage and a green quality gate](https://www.habil.mx/en/blog/tests-that-catch-errors-coverage-quality-gate/) — article on why a green dashboard or high coverage is not enough and why test counts should be read from the report and not from the exit code; accessed on October 6, 2026.
