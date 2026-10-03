# Lesson 4 — Collections and Errors

**Time:** 2 × 45 min.

**What you build:** the list of services and the report, with real errors

**What you learn:** `Vec`, `HashMap`, `String` versus `&str`, `Result` and `?`, `panic!` versus `Result`, `anyhow` and `thiserror`

**The Rust Book, chapters 8 and 9.** Rustlings: `vecs`, `hashmaps`, `strings`, `error_handling`.

## By the end you will be able to

- Store a list of `Servicio` in a `Vec<Servicio>` and walk through it without fighting ownership.
- Choose between indexing a collection and using methods that return `Option`.
- Store statuses by name in a `HashMap<String, Estado>` and produce a sorted report.
- Explain why a function normally receives `&str`, while a struct normally stores `String`.
- Propagate file and validation errors with `Result` and `?`.
- Tell apart an error that the user of the program can fix from a broken invariant that justifies `panic!`.
- Add context to an application error with `anyhow` and recognize when a library needs `thiserror`.

## The why before the how

In lesson 3 you modeled a service and the different results of checking it. That model still needs to live somewhere. The program receives many services, not just one; it must keep them while it queries each URL, associate each result with its service, and then print a report. In a small program you can write two or three variables by hand. In the real `revisor`, that strategy stops working from the moment the YAML file brings a variable number of services.

Collections solve the "how many values are there" part, but they don't by themselves solve what it means for a value to be missing or what the program should do when something external fails. A file may not exist, a line may have an invalid format, a service name may be repeated, and a URL may lack a scheme. None of those cases is rare or impossible: they all happen because the program receives data from the outside. The important difference is that Rust asks you to represent that possibility in the return type.

A `Vec<Servicio>` represents an ordered list of services. A `HashMap<String, Estado>` represents an association by key: given the name `"catalogo"`, look up its status. A `String` is text that owns its memory; a `&str` is a borrowed view of text that someone else owns. And a `Result<T, E>` represents an operation that can end with a value `T` or with an error `E`. They are not four unrelated topics: they are the pieces that give the program's state an explicit shape.

The Go course builds the same revisor. In Go, reading a nonexistent key from a `map` returns the zero value and forces you to remember the two-result form to tell "doesn't exist" from "exists and is zero". Rust chooses another contract: `HashMap::get` returns `Option<&V>`. Absence appears in the type and can't be confused with a real state. The cost is that you have to decide what to do with `None`; the gain is that this decision can't be forgotten without the code making it visible.

Something similar happens with errors. Go uses the `if err != nil` convention after every operation that can fail. Rust uses `Result` and lets you write propagation with `?`. There is no universal answer about which style is more readable: Go repeats a very explicit structure; Rust concentrates the same decision in an operator. What matters is that both force you to attend to the error. Rust doesn't turn a missing file into an empty string or let a failed conversion continue as if it were valid.

This lesson doesn't aim for you to use `unwrap()` to make the compiler be quiet. It aims for you to read each function's signature as a contract. If a function returns `Option`, you must think about what absence means. If it returns `Result`, you must decide whether the error is resolved there, transformed, or propagated. If it receives `&str`, it only needs to read text; if it receives `String`, it probably intends to keep it. Signatures describe the flow of data and the flow of failures before the program runs.

## The concepts

### `Vec<T>`: a list that owns values of the same type

`Vec<T>` is Rust's vector: a variable-size collection that owns its elements. The parameter `T` says what type of values it can store. A `Vec<Servicio>` only stores services; a `Vec<Estado>` only stores statuses. This restriction is not an accidental inconvenience. It lets the compiler know how it must manage each element, which methods are valid, and which operations could move or lend values.

An empty vector needs a type annotation if Rust can't infer it. That is why the figure writes `let mut v: Vec<Servicio> = Vec::new();`. The compiler hasn't seen any element yet and can't guess what will be inside. If you create the vector with `vec![...]`, or if the context already determines the type, you normally don't need to write it. The word `mut` is necessary because `push` changes the collection: it adds an element and may make the vector reserve more space.

The vector owns each `Servicio` it receives. In `v.push(s)`, the variable `s` is moved into the vector. This applies the ownership rules of lesson 2: after moving a `Servicio`, you can't keep using the previous variable as if it still owned it. It is not an implicit copy. If you need to keep another independent version, you must design the operation to borrow, or clone deliberately when the cost and the semantics justify it.

There are two ways to read an element. `&v[0]` produces a reference and assumes the index exists. If it doesn't, the program ends in a `panic!`. `v.get(0)` returns `Option<&Servicio>`: `Some(reference)` if it exists and `None` if it is out of range. The second form is appropriate when the index comes from a file, an argument, a request, or any data you don't fully control. The first is reasonable when breaking the program reveals a programming error that should already have been prevented by an earlier validation.

**Fig. 4.1** | Collections and their safe access.

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

Don't use indexes to walk through a vector out of habit. When all you need is to visit each element, `for servicio in &servicios` expresses the intent better and avoids index calculations. When you need the position number, use `enumerate()`: `for (i, servicio) in servicios.iter().enumerate()`. The value `i` stays associated with the right element and there is no risk of accidentally writing `i + 1` when reading.

It also matters that a reference to an element of the vector is a borrow of the whole vector. Adding elements may require moving all the storage to another area of memory. That is why Rust doesn't let you keep `let primero = &v[0]`, then call `v.push(...)`, and then use `primero` again. The restriction prevents dangling references: addresses that used to point to a valid element and would now point to freed memory.

The `revisor` keeps the list of services as a vector because the file declares a sequence and the report needs to keep that conceptual order. The report functions receive borrowed slices, `&[Servicio]` and `&[Estado]`, instead of taking the vectors. A slice gives access to a sequence without transferring ownership of the collection. That way `main` can print the report and then inspect the statuses to choose the exit code.

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

The value of `filas` is a `Vec<Fila<'a>>`: a new list of pairs of references. It doesn't clone the services or the statuses in order to sort them; it creates references to both. That difference matters in a program that may handle large lists. Owning new data costs memory and copying work; borrowing existing data keeps a single source of truth and makes it visible that the report is only observing.

### `HashMap<K, V>`: looking up by key without inventing absences

A `HashMap<K, V>` stores associations between a key and a value. For the revisor, a natural key would be the service name and the value would be its status: `"catalogo" -> Estado::Ok { ... }`. It is a useful collection when you know what you want to look up but don't know in which position of a list it is. Searching linearly in a `Vec` means checking elements until you find one; looking up by key in a map directly expresses the question.

The `insert` operation takes ownership of the key and the value. That is why the figure builds `"catalogo".to_string()`: the map needs to own a `String` that survives after the call ends. `get`, on the other hand, borrows. The type of `m.get("catalogo")` is `Option<&Estado>`, not `Estado`, because the key may not exist and because the map keeps ownership of the status.

This is an important difference from Go. An access like `m["pagos"]` in a Go `map[string]Estado` hands back the zero value if the key doesn't exist. If `Estado` contains numbers, that zero can look like a real answer. In Rust, `None` communicates an absence that you must handle. You can use `match`, `if let Some(estado) = ...`, or methods like `unwrap_or` when a default value is truly correct for the domain.

The pattern `entry(...).or_insert(0)` avoids doing two lookups when you want to update a counter. `entry` represents the position of a key that may be occupied or vacant. `or_insert(0)` leaves the existing value or inserts zero and returns a mutable reference to the counter. The `*` dereferences that mutable reference so that `+= 1` can be applied. It is not decorative syntax: Rust precisely separates the stored value from the borrow that allows modifying it.

A map promises no iteration order. The internal order depends on how the keys are distributed and can change when inserting, deleting, running again, or using another version of the library. Never build a public output by walking through a `HashMap` and expecting it to come out alphabetical by chance. For a reproducible report, extract the keys, sort them, and walk through them in that order, or use an ordered structure when that is the central operation.

The current project doesn't use a `HashMap` for its report. It uses two parallel vectors: services and statuses, both in the same order. That lets a service keep its position from the YAML to the result of the query. At printing time, the `ordenadas` function forms borrowed pairs and sorts them by `nombre`. The concept it shares with a `HashMap` is decisive: storage can have the order that is convenient for working, but the public output must impose its own order explicitly.

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

The lifetime `'a` will appear in depth in lesson 5. For now it is enough to read it as a guarantee: each pair of `Fila` contains references that can't outlive the input slices. The vector `filas` owns the pairs, but it doesn't own the services or the statuses. When `tabla` ends, the temporary references disappear; the original vectors are still owned by `main`.

### `String` and `&str`: owning text versus reading a view

Rust distinguishes text that owns data from text that only lends a view. `String` is a UTF-8 string, mutable and of variable size; it normally lives on the heap and owns its bytes. `&str` is a reference to a UTF-8 sequence that already exists somewhere else. A literal like `"catalogo"` has type `&'static str`: it is a view of text stored inside the binary and available during the whole execution.

The practical rule is simple: receive `&str`, store `String`. A function that is only going to read a name doesn't need to receive ownership or force the caller to create a copy. A struct that must keep the name after the call ends does need to own a `String`. This rule is not absolute, but it avoids two common mistakes: accepting `String` by reflex and ending up moving values unnecessarily, or trying to store a reference to text whose owner will disappear.

**Fig. 4.2** | Receive `&str`, accept both.

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

The call `saludar(&propio)` works through coercion: a reference to `String` can be used where `&str` is expected. Don't copy the string or write `propio.to_string()` to "make it fit". The function only asks for reading, so borrowing is the right operation. Besides, `propio` is still available after the call.

<!-- verificar:fragmento -->
```rust
fn saludar(n: String) { }
```

That signature compiles, but it communicates something else: the caller must hand over ownership of a `String`. It doesn't accept a literal without conversion and doesn't allow continuing to use the original `String` after the call. An API like that could be right if the function is going to store, transform, or return the text as its owner, but it is a poor choice for a function that only prints or compares.

`String` doesn't support indexing with integers as if every character took one byte. Rust uses UTF-8; one visible letter can take several bytes. Allowing `nombre[3]` would be ambiguous: the fourth byte, the fourth Unicode value, or the fourth visible group? That is why you must decide which unit you need. `s.as_bytes()` works with bytes, `s.chars()` works with `char` values, and `s.get(range)` returns `Option<&str>` when the range may fall in the middle of an encoding. This restriction prevents splitting a UTF-8 string at an invalid point.

The `revisor` stores `nombre` as `String` because the value comes from the YAML and must live inside each `Servicio`. When it computes the width of a column, it doesn't count bytes: it uses `chars().count()`. It doesn't solve all the details of Unicode visual width, but it does avoid treating a multibyte character as several characters when counting.

<!-- verificar:extracto:src/reporte.rs -->
```rust
        .iter()
        .map(|(s, _)| s.nombre.chars().count())
        .max()
        .unwrap_or(0)
        .max("SERVICIO".len());
```

Don't convert everything to `String` "just in case". A conversion can allocate memory and, above all, hide who should own the text. Start with the signature: if the function only reads, `&str`; if the result must survive independently, `String`. If you must accept several types that can be viewed as text, you will meet `AsRef<str>` and generic traits in lesson 5, but don't use them before an API really needs it.

### `Result<T, E>` and `?`: making the error path visible

`Result<T, E>` is a standard library enum with two variants: `Ok(T)` and `Err(E)`. A function that returns `Result<String, std::io::Error>` promises one of two things: it will return text read correctly, or it will return the input/output error that prevented reading it. It doesn't return an empty text to signal failure and doesn't print an error inside a function that might be used from somewhere else.

<!-- verificar:fragmento -->
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `?` operator works on a `Result`. If it receives `Ok(valor)`, it extracts `valor` and execution continues. If it receives `Err(error)`, it ends the current function with that error, converting it to the declared error type when a valid conversion exists. It doesn't ignore the error and doesn't turn it into a panic. It is a compact way of writing a decision that is still mandatory.

**Fig. 4.3** | The `?` operator returns the error to the caller.

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

The function `leer_config` doesn't know whether a missing file is fatal for the whole application, recoverable through another path, or expected by a test. That is why it returns the error. `main` does decide how to communicate it: in this figure it prints it. In a production binary you would normally write it to the error output and exit with a nonzero code, so that a person and an automation can tell success from failure.

The comparison with Go is direct. These four lines of Rust:

<!-- verificar:fragmento -->
```rust
let a = paso1()?;
let b = paso2(a)?;
let c = paso3(b)?;
Ok(c)
```

express a chain of operations that in Go is usually written with an `if err != nil` check after each step. Rust reduces the repetition, but not the responsibility. Each `?` marks a place where the function can exit early. If later the program must clean up resources, transform an error, or take an alternative, you must decide it before or after that point.

The `revisor` uses `?` to read the file and to deserialize YAML. Both failures are part of starting the program: there is no valid list of services without a readable file or without valid YAML. The function returns `anyhow::Result<Vec<Servicio>>`, which lets you unify errors of different types without losing their messages.

<!-- verificar:extracto:src/config.rs -->
```rust
use anyhow::{Context, Result};
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}
```

A clarification about the name: `yaml_serde::from_str` is the same `from_str` that `serde_yaml` offered, the earlier crate, which is no longer maintained. In lesson 6 you will see why the `revisor` uses the former.

`with_context` adds information the operating system doesn't know. The original error may say "No such file or directory", but the context clarifies which file the revisor was trying to read. It is the difference between a technically correct diagnostic and an actionable one. The `?` operator preserves that chain of causes when returning the error.

Also notice that not all failures of a query are `Err`. The project's `revisar` function returns `Estado`, even when a service doesn't respond. That is correct because "a service failed" is data that the report must show, not an impossibility to continue the program. `Result` represents that the program itself couldn't complete a necessary operation; `Estado::Falla` represents a normal result in the revisor's domain. Choosing between the two depends on who must decide what to do and on whether the program can still produce a useful result.

### `panic!`: a stop for bugs, not a substitute for `Result`

`panic!` ends the normal flow of execution because the program found a condition that its own assumptions declared impossible. Indexing a vector out of range causes a panic. Calling `unwrap()` on `None` or on `Err` also does. These mechanisms exist because there are invariants that, if broken, reveal a programming error and not a situation that a user should repair.

A missing file is not a broken invariant: it can be missing because of a mistyped path, permissions, an incomplete deployment, or a decision by whoever runs the binary. It must be a `Result`. An HTTP 500 response is not a reason to panic either: it is a state the revisor was built to report. An index computed from a file also shouldn't be used with `[]` without validating; use `get` and return an error that explains the problem.

**Fig. 4.4** | A caught panic, to show that it is not a `Result`.

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

The figure catches the panic only to isolate the demonstration. It is not the normal pattern of an application. `catch_unwind` doesn't turn panics into healthy control flow nor does it guarantee that a structure remains in a state fit for continued use. In everyday code, if you are thinking of recovering from a `panic!` caused by external data, you almost always should redesign the function so that it returns `Result`.

`.unwrap()` and `.expect("message")` are potential panics. `expect` is preferable when there is a concrete, invariant reason to believe it won't fail, because its message documents that reason. Don't write `expect("it should work")`: it explains nothing. A useful message names the assumption, like "the semaphore is never closed", and makes clear what would need to be investigated if it happens.

The project uses `expect` to acquire a permit from an internal semaphore. It is not an error caused by the YAML file or by a URL; it would be a contradiction in the coordination that the program itself built. That is why it is one of the few places where a panic makes sense.

<!-- verificar:extracto:src/revisar.rs -->
```rust
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
```

Don't copy this pattern for input/output operations. `File::open(ruta).expect(...)` turns a nonexistent path into the termination of the process and removes the chance for `main` to print the file, use another value, or select an appropriate exit code. First ask whether the case can occur with valid data from the outside. If the answer is yes, return `Result`.

### `anyhow` and `thiserror`: two different roles for your own errors

The standard library is enough for many small programs: you can return `Result<T, std::io::Error>` when every relevant failure is an input/output one. A real program usually combines several types: `std::io::Error`, a YAML error, an invalid URL, a command-line argument, or a validation rule. If each layer needs to know all those concrete types, the signatures become hard to maintain.

`anyhow` solves the edge of an application well. Its `Result<T>` is a shorthand for returning a dynamic error that can contain different causes and additional context. The revisor is a binary: it reads configuration, starts queries, and presents messages to a person. At that boundary, the priority is to explain which operation failed and to preserve the chain of causes. That is why `config::cargar` uses `anyhow::{Context, Result}`.

That doesn't mean `anyhow` is a license to erase meaning. If a function returns a state that another part of the program must distinguish in order to make decisions, an enum of its own may be better. For example, a library that needs to let the caller tell apart `NombreRepetido`, `UrlSinEsquema`, and `TimeoutCero` shouldn't deliver only a string. It should publish an error type with variants that represent those causes.

`thiserror` helps declare that kind of error type without manually writing repetitive implementations of `Display`, `Error`, and conversions from internal errors. It is used mostly in libraries, where the error type is part of the public API. The project's `Cargo.lock` contains `thiserror`, but the revisor's `Cargo.toml` doesn't declare it as a direct dependency and its current code doesn't expose an error enum of its own. That is why the `revisor` doesn't use it: its errors are text messages that `anyhow` accompanies with context.

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

This fragment illustrates a library API; it is not part of the current revisor. `#[from]` lets a `std::io::Error` be converted automatically into `ErrorConfiguracion`, so that `?` stays useful. The other variants keep data that the caller can inspect through `match`. In a single-layer application, converting those errors to `anyhow::Error` at the end can be convenient; in a library, hiding them too early takes options away from whoever uses it.

The practical boundary is this: `anyhow` to run an application and explain a complete failure; `thiserror` to offer an error contract that other programs must handle by variant. You can combine both, but don't add them out of fashion. Start with the type that lets the next layer make the right decision.

## The error you will see

### E0277: using `?` in a function that can't return an error

The most frequent error when you start using `?` appears when the function declares a simple return, like `String`, but inside tries to propagate a `Result`. Rust can't invent where to store the error or how to communicate it to the caller. The following real run, with `rustc 1.98.1`, reads the program from standard input, which is why the compiler names the file `<anon>`; with a file on disk you will see its name instead of `<anon>`.

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

`E0277` says that `?` needs a function able to return an error residual. `E0308` is the consequence: `Ok(texto)` is a `Result`, but the signature promised a `String`. The fix is not to remove `?` and use `unwrap()`. You must correct the contract so that it describes the real possibility of failure.

<!-- verificar:fragmento -->
```rust
fn leer() -> Result<String, std::io::Error> {
    let texto = std::fs::read_to_string("faltante.txt")?;
    Ok(texto)
}
```

If you are in `main`, you can also return `Result` when the error must end the program. However, the current revisor needs to control what is printed and which exit code it returns, which is why `main` transforms the result of `config::cargar` into a visible output and `ExitCode::from(2)`. The message goes to `stderr`; the successful table or JSON remains available on `stdout`.

### A panic from an out-of-range index

The access `v[indice]` is not a compile error if `indice` is a variable. Rust can't know at compile time which number will arrive. If the number falls out of range, the program panics during execution. The diagnostic mentions the requested index and the real length of the vector. For example, asking for position `3` of a three-element list fails because the valid positions are `0`, `1`, and `2`.

The fix depends on where the index comes from. If it is a constant written next to a fixed list and the index is wrong, fix the program. If it comes from a file, a request, or a user option, use `get(indice)` and convert `None` into a `Result` that explains what the valid range was. Don't let a recoverable input end the process with a backtrace.

### Diagnose before fixing

First read the function's signature and the concrete type of the expression that failed. If it says `Option`, decide what absence means. If it says `Result`, read the error variant and decide whether it should be propagated, given context, or handled there. If `panic!` appears, ask which assumption of the program was broken. Copying `.clone()`, `.unwrap()`, or `as` until it compiles usually erases useful information and moves the problem to run time.

`rustc --explain E0277` expands the general meaning of the code, but the local diagnostic is still the main source. Its arrows point to the expected type, the found type, and the line where the contract stopped matching. Learning to follow those three clues is worth more than memorizing a list of codes.

## What goes wrong

- **Using `v[i]` for indexes that come from outside.** The direct index asserts that the position exists. If that assertion depends on a file or on human input, use `get`; `None` is data you must turn into a useful explanation.

- **Walking through a `HashMap` and publishing its accidental order.** A report that changes order is hard to read, test, and compare. Extract keys or rows, sort them, and only then print. The revisor's output must be reproducible even if the internal storage changes.

- **Using `String` in every parameter.** It forces transferring ownership or creating unnecessary allocations. If a function only reads, declare `&str`; reserve `String` for structures and results that must own text.

- **Indexing a `String` by byte or assuming `len()` counts visible letters.** Rust stores UTF-8 text. Use `chars`, `bytes`, or ranges with `get` depending on the unit you actually need. A name that today only has ASCII may tomorrow contain valid characters of more than one byte.

- **Turning all errors into `unwrap()` or `expect()`.** It makes the program look short while removing recovery paths and context. `unwrap` is acceptable in a test when the failure invalidates the test itself; it is not the normal way to read files or process arguments.

- **Using `panic!` for invalid configuration data.** An incorrect URL, a missing file, or a repeated name are failures a person can fix. Return `Result` with the missing data and an understandable cause.

- **Losing the original cause when creating a new message.** A text like `"could not load"` doesn't say which path failed or why. Use `with_context` to add the operation and local data without discarding the cause that the system or the parser returned.

- **Using `anyhow` in a library that needs distinguishable errors.** If the caller must react differently to an invalid URL and a repeated name, publish an enum of your own, normally with `thiserror`. The convenience of a string shouldn't erase domain decisions.

## Exercises

### Exercise 1 — Honest access to a list

Create a `Vec<Servicio>` with two services. Write a function `nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str>` that returns the service's name when it exists and `None` when it doesn't. Test it with the indexes `0`, `1`, and `2`. Don't use `[]` inside the function.

Explain in writing why returning `Option<&str>` is more honest than returning an empty string. Think about what would happen if an empty name were allowed data.

### Exercise 2 — Statuses by name and a deterministic report

Create a `HashMap<String, Estado>` with three names, including a failure. Write a function that produces a `Vec<String>` with the names sorted alphabetically. Then walk through those names and generate lines in the format `nombre: estado`.

Run the program several times. The output must keep exactly the same order. Don't sort the `HashMap`: it can't be sorted. Sort a separate collection of keys or rows.

### Exercise 3 — Load, validate, and add context

Write a function `cargar(ruta: &str) -> Result<Vec<Servicio>, ...>` that reads a text file with one line per service. Each line must contain a name and a URL separated by a comma. Reject a line without two fields, a URL without `http://` or `https://`, and an empty list. Propagate read errors with `?`.

Then adapt the function to use `anyhow::Context` and add the path to the read message. Make `main` print the errors to the error output and exit with code `2`; if all the data is valid, print the sorted list and exit with code `0`.

## Solutions

### Solution 1

The function must borrow the slice, not take the vector. `get` already returns `Option<&Servicio>`, and `map` transforms the content of `Some` without touching `None`.

<!-- verificar:fragmento -->
```rust
fn nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str> {
    servicios.get(indice).map(|servicio| servicio.nombre.as_str())
}
```

`as_str()` converts the reference to `String` into a reference to `str`; it doesn't allocate memory or clone text. The lifetime of the `&str` is limited by the lifetime of the borrowed slice, which is precisely the right contract. An empty string would be ambiguous: it could mean "I didn't find that index" or "I did find the service and its name is empty".

### Solution 2

The solution needs to separate the structure that is useful for looking things up from the structure that is useful for presenting them. The map keeps the association; the vector of keys gets the order that the report requires.

<!-- verificar:fragmento -->
```rust
let mut nombres: Vec<&str> = estados.keys().map(String::as_str).collect();
nombres.sort();

for nombre in nombres {
    let estado = &estados[nombre];
    println!("{nombre}: {estado:?}");
}
```

The access `estados[nombre]` is reasonable here because `nombre` comes directly from `estados.keys()`: the program has already shown that the key exists. If `nombre` came from a file or an argument, this form would again assert something you haven't validated and you should use `get`.

### Solution 3

The load signature must let external problems rise as `Result`. The format rules must also become errors, not panics. If you use `anyhow`, an implementation can follow this form:

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

The solution doesn't use `unwrap` because every failure can originate in external content. `split_once` returns `Option`; `with_context` turns it into an explanatory error. `ensure!` ends the function with `Err` if the condition isn't met. The context includes the line number or path so that whoever fixes the file doesn't have to guess where to start.

## How do I know I got it

- You run `rustc --edition 2024 fig04_01.rs && ./fig04_01` and get exactly six lines, including `None` for `pagos` and `Some(2)` for the counter.
- You run `rustc --edition 2024 fig04_02.rs && ./fig04_02` and can explain why the same `&str` parameter accepts a literal and a reference to `String`.
- You run `rustc --edition 2024 fig04_03.rs && ./fig04_03` and get a missing-file error without a panic.
- Your solution to exercise 1 returns `None` for the out-of-range index and contains no access with `servicios[indice]`.
- Your report from exercise 2 produces the same lines, in the same order, after at least ten runs.
- Your solution to exercise 3 names the path when the file doesn't exist, names the line when the format is wrong, and doesn't use `unwrap` on the normal execution path.
- From `programas/revisor`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --check` finish successfully.

## Further reading

- [The Rust Programming Language, chapter 8: collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html), in particular vectors, strings, and hash maps. Accessed October 2, 2026.

- [The Rust Programming Language, chapter 9: error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html). Accessed October 2, 2026.

- [Official documentation of `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) and [of `HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html). Accessed October 2, 2026.

- [Documentation of `anyhow`](https://docs.rs/anyhow/) and [documentation of `thiserror`](https://docs.rs/thiserror/). Accessed October 2, 2026.
