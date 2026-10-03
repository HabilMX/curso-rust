# Lesson 5 — Traits, Generics, and Lifetimes

**Time:** 2 × 45 min.

**What you build:** the `Revisor` trait and a generic function that uses it, in separate programs (the real `revisor` declares no trait at all).

**What you learn:** traits and default methods, generics with bounds, lifetimes, and the `'a` that scares people.

## By the end you will be able to

- Define a trait, implement its contract for two types, and use a default method.
- Tell an inherent implementation (`impl Type`) from a trait implementation (`impl Trait for Type`).
- Write a generic function with bounds and explain when Rust generates specialized code.
- Choose between a generic parameter and `dyn Trait` depending on whether you need to decide the type at compile time or at run time.
- Read a signature with `'a` and explain which references that lifetime ties together.
- Recognize a lifetime or trait bound error, locate its cause, and fix the design without unnecessary copies.

## The why before the how

By lesson 4, the `revisor` already has a useful model. It can represent a `Servicio`, keep a list in a `Vec`, tell results apart with `Estado`, load configuration, and report errors. Still, there is a design question that comes up every time the program grows: how do you separate what the program needs to do from the concrete way it gets done?

The `revisor` needs to obtain an `Estado` for each `Servicio`. Today the real implementation makes an HTTP request with `reqwest::Client`; tomorrow you might want an implementation that reads a file, queries a database, measures a local process, or simulates responses for a test. The rest of the program shouldn't have to know all those details. It only needs to be able to ask: "check this service and return its status".

In Go, that contract is expressed with an interface. A type satisfies an interface implicitly: if it has the required methods, it already satisfies the interface. That decision makes it very easy to adapt existing types, but it can also hide important relationships. A type can end up satisfying an interface by accident, and when you read its definition you don't always know which contracts it takes part in across other packages.

Rust uses traits to solve the same kind of problem, but it requires you to declare the relationship explicitly. A trait describes capabilities; then `impl Revisor for RevisorHttp` declares that this type provides that capability. It is one more line, but it is a line that documents architecture. When you read it, you know that `RevisorHttp` doesn't merely have a method called `revisar`: it committed to the `Revisor` contract.

Traits don't replace structs or enums. Each tool answers a different question. A `struct` says what data makes up a thing; an `enum` says which valid alternatives exist; a trait says which operations a type can offer. The `Servicio` from lesson 3 is still a struct because it models data. `Estado` is still an enum because a service can be healthy, slow, failing, or not yet checked. `Revisor` is a trait because it describes the operation that produces a status.

Generics make it possible to write a function that works with a family of types without losing track of which concrete type it received. This lesson's `revisar_todos` function can accept any `R` that implements `Revisor`. It doesn't need an `if` for each implementation or convert everything to text. The compiler knows the concrete type of `R` when it compiles each call and can verify that the right method exists.

Lifetimes complete this model when you work with references. Ownership already established that every value has an owner and that a reference is a borrow. A lifetime doesn't create another form of ownership and doesn't extend a value's life. It is an annotation that helps the compiler prove that a borrow will remain valid for every possible use. It shows up mainly when a function receives references and returns a reference, or when a struct stores references.

The `'a` notation is intimidating because it looks like a mysterious variable, but it reads better as a label. If a function receives two references marked `'a` and returns another one marked `'a`, it is declaring: "the output reference depends on these inputs and can't be used after the shorter reference stops being valid". It doesn't say how long `'a` lasts; that depends on each call. It doesn't reserve memory or perform garbage collection either.

This lesson corresponds to chapter 10 of The Rust Book. Before continuing, read its sections on generics, traits, and validating references, and do the Rustlings exercises `generics`, `traits`, and `lifetimes`. The goal is not to memorize every possible syntax for bounds and lifetimes. It is to learn to recognize three questions: what behavior the program needs, which types can provide it, and where the references that outlive a function come from.

## The concepts

### Traits: explicit contracts and default methods

A trait gathers method signatures that represent a capability. The signature says what the method receives and what it returns, without deciding how it will do the work. Each type that wants to satisfy the trait writes its own implementation. That's why a trait resembles a Go interface, but its relationship with the type is explicit.

The figure defines `Revisor` with two methods. `revisar` has no body: every implementation must decide how to check a service. `nombre` does have a body and returns `"revisor"`. That is a default method. An implementation can accept it as is, like `RevisorHttp`, or replace it, like `RevisorFalso`.

**Fig. 5.1** | A trait with a default method, and two types that satisfy it.

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

`impl Revisor for RevisorHttp` reads left to right: "implement the `Revisor` trait for the `RevisorHttp` type". Inside that block, Rust requires you to implement every method without a body that the trait demands. If you omit `revisar`, the program doesn't compile. If you omit `nombre`, it does compile because the trait already provided a default implementation.

The method receives `&self`, just like the struct methods from lesson 3. It doesn't consume the checker or modify it; it only borrows it to inspect its data. `revisar` also receives `&Servicio`, because checking a service shouldn't consume it. The result, on the other hand, is returned by value: each check creates a new `Estado` and the caller receives ownership of it.

The `Display` trait in the figure comes from the standard library. `impl fmt::Display for Estado` lets you use `{}` inside `println!`. The implementation decides on a human-oriented representation: `OK en 1ms` or `FALLA: ...`. This is different from `Debug`, which is normally obtained with `#[derive(Debug)]` and printed with `{:?}` for diagnostics. If the report is part of the program's interface, defining `Display` forces you to think about which stable text deserves to be seen by whoever runs it.

A trait is not a base class. It stores no fields, builds no objects, and inherits no implementation from a parent. It can provide default behavior, but each type keeps its own data. `RevisorHttp` has `timeout_ms`; `RevisorFalso` doesn't need any field. Both satisfy the same contract because both can respond to `revisar(&Servicio)`.

Rust applies the coherence rule, also called the orphan rule. You can implement your own trait for someone else's type, for example `impl Revisor for String` if it made sense. You can also implement someone else's trait for your own type, like `impl Display for Estado`. What you can't do is implement someone else's trait for someone else's type: you can't decide from your crate how `Display` should be implemented for a `Vec<String>`. The rule prevents two different dependencies from defining incompatible implementations of the same contract.

The real `revisor` declares no trait for its HTTP requests. Its `revisar` function receives a concrete `reqwest::Client`, and its integration tests use a local HTTP server instead of a test double. It is a conscious decision: a trait that would have a single real implementation doesn't solve any problem yet. This lesson's `Revisor` trait lives in separate programs so that you can practice the form; the project would need it the day two different ways of checking a service exist. In the meantime, the program does use `impl` to group methods that belong to the domain types:

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

This block is an inherent implementation: `impl Estado`, without `for`, adds a method that belongs directly to `Estado`. It doesn't implement a trait. Telling the two forms apart avoids a common confusion: every trait implementation uses `impl`, but not every `impl` implements a trait.

A trait would be useful in the `revisor` if the application needed to swap the source of the checks within the same design. For example, a unit test could use a fake checker with no network. You shouldn't create a trait just because Rust offers it. Abstraction has a reading cost: it adds a contract, implementations, and decisions about how to inject them. The current project tests HTTP through a local server precisely because it wants to verify the real behavior of the HTTP layer.

### Generics and bounds: reuse without erasing the type

A generic parameter is a type variable. In `fn revisar_todos<R: Revisor>(...)`, `R` doesn't mean "any value, no rules"; it means "any type that implements `Revisor`". The part after the colon is a bound, also called a trait bound. Thanks to it, the function body can call `r.revisar(s)`: the compiler has the guarantee that any accepted `R` provides that method.

**Fig. 5.2** | A generic function with bounds.

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

The function receives `&R`, not `R`. That keeps ownership of the checker with the caller and lets it be used for all the services. `servicios: &[Servicio]` is a borrowed slice, just like the slices seen when walking through collections: the function can read the services, but it doesn't consume them or need a copy of the `Vec`.

`map` receives each `&Servicio`, calls `r.revisar(s)`, and produces an iterator of statuses. `collect()` gathers those statuses into a `Vec<Estado>` because the return type asks for it. The function is generic over the checker, but not over the status: the `Revisor` contract fixes that every implementation returns `Estado`. That choice is right when the domain needs a single coherent representation of the results.

The form `T: Display + Clone` shows several bounds joined with `+`. However, `imprimir` only uses `Display`; it doesn't call `clone`. The `Clone` bound is there to teach the syntax, not because it is needed. In production code you should ask only for the capabilities the body needs. An extra bound excludes valid types and makes the API look more demanding than it really is.

When the bounds grow, Rust lets you write them with `where`. For example, a long signature can end with `where R: Revisor, E: std::error::Error`. It doesn't change the behavior or the verification; it only places the bounds where they read better. Start with the short form and use `where` when the signature stops being clear.

Rust generics are normally resolved through monomorphization. If you call `revisar_todos` with `RevisorFalso` and then with another type `RevisorArchivo`, the compiler generates specialized versions for those concrete types. At run time it doesn't need to look up the method in a table for those calls. This is called static dispatch. The benefit is predictable performance and more precise checks; the cost is that each combination of types can increase the compiled code.

`impl Revisor` in a parameter is a short way to write an input generic. A signature like `fn ejecutar(r: impl Revisor)` is equivalent, for that simple case, to `fn ejecutar<R: Revisor>(r: R)`. The `<R: Revisor>` form is preferable when you must use the same generic type more than once in the signature, return it, or add relationships between several parameters.

When the type decision must be made at run time, the trait object appears: `Box<dyn Revisor>`. A `Vec<Box<dyn Revisor>>` can store a `RevisorHttp`, a `RevisorFalso`, and other checkers of different sizes in the same collection. In exchange, each call goes through an indirection and the value usually lives behind a pointer such as `Box`, `&`, or `Arc`. It is the closest equivalent to a Go interface at run time.

There is no universally better option. Use generics when the concrete type is known where the call is compiled and you want to keep that information. Use `dyn Trait` when the program needs to choose or combine implementations during execution. In Go, interfaces usually carry dynamic dispatch by their usual design; in Rust you explicitly choose between the two models.

The real project also uses generic types from the standard library, even though it doesn't declare a function of its own with `<T>`. `Option<T>` expresses that there may or may not be a value of any type, and here it is specialized as `Option<u16>` for an HTTP code:

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

`Option<u16>` and `Option<String>` are different uses of the same generic type. The first lets you represent that a failure had no HTTP code; the second lets you omit the error field when a service responded well. Generics aren't just a technique for sophisticated libraries: `Vec<T>`, `Option<T>`, `Result<T, E>`, and `HashMap<K, V>` are part of daily work in Rust.

### Lifetimes: describing borrows that are related

A lifetime is a region of validity for a reference. Rust almost always infers it, just as it infers many local types. You need to write an annotation when the signature could allow several relationships between references and the compiler can't know which one the design guarantees.

The figure returns one of two references. Without an annotation, the signature can't communicate whether the result comes from `a`, from `b`, or from somewhere else. By marking all three references with `'a`, you declare that the result will be valid for a period that can't exceed that of any chosen input.

**Fig. 5.3** | A lifetime that ties the output to both inputs.

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

`'a` doesn't mean "lives forever" or "lives exactly as long as both inputs". It is a name for a relationship. In a concrete call, Rust computes a lifetime that fits within the valid borrows. If `a` lasts ten lines and `b` lasts three, the result can only be used for the three lines during which both borrows remain valid. The signature prevents someone from keeping a reference to the result and then destroying the value it came from.

The function doesn't choose which string lasts longer. That depends on the caller's scopes, not on the number of letters or on a value stored in memory. `mas_largo` compares lengths to choose content, but the lifetime speaks about the validity of references. They are two different matters that happen to appear in the same function.

Rust has elision rules that make many lifetimes invisible. For example, `fn nombre(s: &str) -> &str` compiles without writing `'a` because there is a single input reference and Rust can tie the output to it. It also usually infers lifetimes for methods that receive `&self`. When there are two possible inputs, as in `mas_largo`, guessing is no longer safe and you must describe the relationship.

Don't add `'a` to everything that looks complicated. An annotation doesn't fix an invalid reference; it only declares a relationship that the compiler will check. If you try to return a reference to a local `String`, no lifetime exists that can make that borrow valid. The `String` is destroyed when the function ends. The solution is to return the `String` by value, receive a reference that belongs to the caller, or redesign who owns the data.

The special `'static` lifetime deserves care. A `&'static str` reference usually points to literal text included in the binary, like `"OK"` or `"FALLA"`. It doesn't mean "use `'static` to make errors go away". Forcing `'static` onto data that really lives for a short time doesn't make it last longer; the compiler will reject it. Use `'static` only when the value really lives for the whole execution.

The real `revisor` uses lifetimes where they are actually needed: in a temporary row that borrows a `Servicio` and its corresponding `Estado` in order to sort the report. It doesn't copy those values just to sort them. It builds references, stores them in a local vector, and lets the compiler check that the vector doesn't outlive its sources.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

`Fila<'a>` is an alias for a tuple of two references. It doesn't own a `Servicio` or an `Estado`; it only borrows them. `ordenadas` receives two slices with the same annotated lifetime and returns rows that carry that lifetime too. Therefore, nobody can keep the rows after the original vectors disappear. The vector of rows can change order because it owns the vector, but it can't modify the services or the statuses because it only borrows them.

The same function shows a practical reason to prefer references: it avoids cloning information just to show it sorted. Cloning would be valid if you needed an independent collection that outlived the report, but it isn't necessary here. The report finishes using `filas` before `servicios` and `estados` end, so the borrows express the data model exactly.

When a lifetime appears in a struct or alias, don't read it as ceremonial syntax. Ask: "does this type store a reference?" If the answer is yes, the annotation links the type to the duration of the borrowed value. If the answer is no, the type should probably own a `String`, `Vec<T>`, or other value and doesn't need an explicit lifetime.

## The error you will see

### E0515: returning a reference to a local value

This error appears when a function tries to return a reference to something that stops existing when it returns. The following figure fails on purpose. The `'a` lifetime in the signature can't save `nombre`: that `String` is owned by `devolver` and is destroyed when the function closes.

**Fig. 5.4** | A borrow that tries to escape from the value that owns it.

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

The message points exactly at the reference that tries to escape. Don't add another lifetime or use `&'static str`: neither changes who owns `nombre`. If the function must create the text, the fix is to return `String`. If it must return a view of text that already existed, receive `&str` as a parameter and tie the output lifetime to that input.

### E0277: the type doesn't satisfy the requested bound

A trait bound is also a verifiable contract. In this figure, `imprimir` asks for a type that implements `Display`, but `Vec<&str>` doesn't have that implementation. Rust doesn't convert it to text implicitly because there is no single correct representation for all collections.

**Fig. 5.5** | An argument that doesn't satisfy the trait bound.

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

`E0277` says that a trait bound was not satisfied. The note takes you to the signature that introduced the requirement. The fix depends on your intent: you can print with `{:?}` if you want a debug representation and the type implements `Debug`; you can walk through the vector and print each element; or you can explicitly convert it to a `String` with the format your report needs. Don't implement `Display` for someone else's type just to silence the error: the orphan rule will prevent it and, besides, it would be a global decision that is hard to justify.

## What goes wrong

### Creating a trait for every struct

A trait should represent a shared capability, not repeat the name of a type. If there is only one implementation and no concrete reason to swap it, an inherent method is usually clearer. `impl Estado { fn esta_bien(...) }` expresses that the operation naturally belongs to `Estado`. Creating an `EstadoConsultable` trait for a single function only adds names and files without separating a real dependency.

Start with structs, enums, and direct functions. Extract a trait when several implementations must satisfy the same contract, when you need to receive a capability instead of a concrete type, or when a testing boundary really justifies it.

### Adding bounds "just in case"

It's common to copy a signature like `T: Clone + Debug + Display` and keep all the bounds even though the body only uses `Display`. Each bound limits the types that can call the function. In addition, each capability promised by a signature becomes part of the API that others must understand.

Ask for `Clone` only if the body calls `clone`, `Ord` only if it sorts, and `Send` only if it moves data to another thread. A small bound is a more flexible abstraction and describes the real need better. Figure 5.2 keeps `Clone` to show that bounds can be combined, but it isn't a model to copy literally.

### Using `Box<dyn Trait>` out of habit

A trait object solves a real problem: storing or choosing different implementations at run time. It isn't the mandatory way to use traits. If the type is known at compile time, a generic parameter is normally simpler, avoids unnecessary heap allocations, and allows static dispatch.

The useful question is not "traits or generics?". A trait describes a capability; then you choose whether to receive it with generics, such as `impl Trait`, through a `&dyn Trait` reference, or behind a `Box<dyn Trait>`. The choice depends on ownership, size, and when you know the type.

### Cloning to silence borrow checker errors

If a reference doesn't live long enough, copying a `String` with `.clone()` can make the program compile, but it doesn't always solve the right design. Sometimes it only hides that a function should return a reference, that a type should own its data, or that a borrow lasts longer than necessary.

Diagnose first: identify the owner, identify who needs to use the data afterward, and decide whether it needs a view or an independent copy. Clone when two legitimate owners need to keep separate values. The `revisor`'s vector of rows doesn't clone services or statuses because it only needs to sort them while their owners are still alive.

### Reading `'a` as a concrete duration

`'a` doesn't mean one second, a fixed scope, or a variable created at the start of the program. It is a label that Rust replaces with a valid region in each call. Two functions can use the name `'a` without sharing absolutely anything; the name only has meaning inside its own signature.

It is also a mistake to think that more annotations are safer. Annotations should reflect where a reference comes from. If you can't explain which input reference backs the output, the function should probably return an owned value instead of a reference.

## Exercises

### Exercise 1 — A fake checker with a default name

Define a `Revisor` trait with `revisar(&self, servicio: &Servicio) -> Estado` and a default method `nombre() -> String`. Create `RevisorFalso`, which returns `Estado::Ok { ms: 1 }` without replacing `nombre`. Check that it prints `revisor -> OK en 1ms`.

Then add `RevisorArchivo`, which replaces `nombre` with `"archivo"` and returns a deterministic failure. Explain in one sentence why both types can be used where a `Revisor` is expected.

### Exercise 2 — Counting healthy results generically

Use the trait from figure 5.1 and write a function `contar_sanos<R: Revisor>`. It must receive a checker and a slice of services, check them, and return how many statuses are `Ok`. Test the function with three services and `RevisorFalso`.

Before coding, decide what each part must own: the function must not consume the checker or the vector of services. Use `&R` and `&[Servicio]`, don't clone.

### Exercise 3 — Reading the report's lifetime

Open `programas/revisor/src/reporte.rs` and locate `Fila<'a>` and `ordenadas<'a>`. Write in your own words what the `Vec<Fila<'a>>` owns, what it borrows, and what would happen if you tried to return those rows after destroying `servicios` or `estados`.

Then write a function `primero<'a>` that receives `&'a str` and returns `&'a str`. Compare it with `devolver` from figure 5.4 and explain why one compiles and the other doesn't.

## Solutions

### Solution 1

The implementation that accepts the default method doesn't write `nombre`; the trait provides its body. The second implementation does replace it because it needs a different label.

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

Both types can be used where a `Revisor` is expected because both wrote `impl Revisor for ...` and provide the required method `revisar`. The difference between their data and their algorithm is encapsulated inside each implementation.

### Solution 2

The function takes borrows because it only needs to consult the data. Each result is temporary: it is counted and discarded. There is no reason to store a `Vec<Estado>` or to clone the checker or the services.

<!-- verificar:fragmento -->
```rust
fn contar_sanos<R: Revisor>(revisor: &R, servicios: &[Servicio]) -> usize {
    servicios
        .iter()
        .filter(|servicio| matches!(revisor.revisar(servicio), Estado::Ok { .. }))
        .count()
}
```

`iter()` produces `&Servicio`; the closure receives each borrow and calls the trait through `&R`. `matches!` decides whether the status belongs to the `Ok` variant, and `count()` returns the total. If `Estado` also had `Lento`, you must explicitly decide whether it counts as healthy; the real `revisor` answers that question with `Estado::esta_bien()`.

### Solution 3

`Vec<Fila<'a>>` owns the vector and the order of its elements, but it doesn't own the services or the statuses. Each element contains two references. That's why its rows can only live while the slices borrowed by `ordenadas` are still alive. Trying to return them to use them after destroying the original collections would produce a borrow checker error: they would be dangling references.

The correct function returns a reference that belongs to the caller:

<!-- verificar:fragmento -->
```rust
fn primero<'a>(texto: &'a str) -> &'a str {
    texto
}
```

`primero` doesn't create the text or try to lend it after destroying it. It only returns the same borrow it received. `devolver`, on the other hand, creates a local `String`, owns it, and destroys it on exit; that's why the reference in figure 5.4 can't escape.

## How do I know I got it

- `rustc --edition 2024 fig05_01.rs && ./fig05_01` prints the two documented lines, including the default label of `RevisorHttp`.
- `rustc --edition 2024 fig05_02.rs && ./fig05_02` prints `OK en 8ms` and `OK en 5ms`.
- `rustc --edition 2024 fig05_03.rs && ./fig05_03` prints `catalogo`.
- `rustc --edition 2024 fig05_04.rs` fails with `error[E0515]`; you can explain why changing `'a` doesn't fix the local borrow.
- `rustc --edition 2024 fig05_05.rs` fails with `error[E0277]`; you can locate both the incorrect call and the bound that rejected it.
- You can point to `Fila<'a>` in `programas/revisor/src/reporte.rs` and explain that it borrows services and statuses instead of cloning them.
- You finished the Rustlings exercises `generics`, `traits`, and `lifetimes`.

## Further reading

- [The Rust Programming Language, chapter 10.1: generic data types syntax](https://doc.rust-lang.org/book/ch10-01-syntax.html), accessed October 2, 2026.
- [The Rust Programming Language, chapter 10.2: traits](https://doc.rust-lang.org/book/ch10-02-traits.html), accessed October 2, 2026.
- [The Rust Programming Language, chapter 10.3: validating references with lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), accessed October 2, 2026.
- [Rustlings: exercises on generics, traits, and lifetimes](https://rustlings.rust-lang.org/), accessed October 2, 2026.
