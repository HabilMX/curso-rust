# Lesson 3 — Structs, Enums, and match

**Time:** 2 × 45 min.

**What you build:** the `revisor` model: `Servicio` and `Estado`

**What you learn:** structs and `impl`, enums that carry data, exhaustive `match`, `Option` instead of `nil`

**The Rust Book, chapters 5 and 6.** Rustlings: `structs`, `enums`, `options`.

## By the end you will be able to

- Model a service with a `struct` whose fields have a name and a type.
- Write methods inside an `impl` block and decide whether they receive `&self`, `&mut self`, or `self`.
- Represent mutually incompatible results with an `enum` that carries data.
- Write an exhaustive `match` that extracts data from each variant.
- Explain why adding a new variant forces you to review existing decisions.
- Use `Option<T>` when a value can be missing, without resorting to `nil`.
- Choose between `match`, `if let`, and methods like `unwrap_or` depending on the intent of the code.

## The why before the how

So far the course has used simple values: numbers for times, strings for names, and conditions to classify a response. That is enough to practice variables, functions, types, and ownership, but it is not enough to describe the real domain of the `revisor`. A service is not just a name, a URL, and a time limit that happen to appear together in three variables. They are three pieces of data that describe a single thing and that must travel, be validated, and be consulted as a unit.

Storing that data separately produces silent errors. Imagine you have `nombre_catalogo`, `url_catalogo`, `timeout_catalogo`, then you add the same three values for payments and reports, and when building the report you join the payments name with the catalog URL. The compiler can't detect the problem: the three pieces have valid types, but their relationship was lost. A `struct` lets you declare that relationship once and make it part of the type.

The second problem appears after querying a service. A healthy response brings an HTTP code and a duration. A slow response also brings both pieces of data, but requires a different label. A failure may bring a message and a duration, but not necessarily an HTTP code. And a service that hasn't been queried yet has no code, no duration, and no failure message. If you tried to store all of that in a single `struct` with "sometimes valid" fields, you would have absurd combinations: a failure state with code `200`, a not-attempted service with a duration of `0 ms` that nobody knows how to interpret, or an empty error message that means different things depending on another boolean field.

Rust solves that modeling with `enum`. Unlike a traditional enum in other languages, which is usually a list of numbers or named constants, a Rust variant can carry data. `Estado::Ok` carries a code and milliseconds; `Estado::Falla` carries a reason; `Estado::NoIntentado` carries nothing because there is no honest data to store. The type expresses that a value is in exactly one of those states, never in several at once.

The third piece is `match`. When you receive an `Estado`, it isn't enough to know that it belongs to the enum: you need to decide what to do with each possibility. Rust demands that this decision cover all the variants. It is not a style recommendation or a linter rule; it is part of compilation. If tomorrow you add `Estado::Rechazado`, every `match` that used to look finished becomes a place the compiler points out for you to review. That obligation is a safety net for refactoring.

The Go course builds the same `revisor`, but here an important difference between the two languages appears. In Go, a result is usually modeled with a `struct`, zero-value fields, pointers, and conventions about which fields are present. In Rust, the type can directly represent incompatible alternatives. It doesn't remove the need to think about the domain, but it makes the right decisions easier to express and the inconsistencies harder to compile.

The last part of the model is absence. In many languages a reference can be `null` or `nil` even though its type doesn't visibly say so. The program reaches a line that expected an object, receives absence, and fails at run time. Rust has no `nil`. When something can be missing, its type declares it through `Option<T>`. That forces you to make a decision before using the content: handle `Some(valor)`, handle `None`, or provide an explicit alternative. Absence stops being a hidden accident and becomes part of the function's contract.

This lesson is not about memorizing all of the pattern syntax. It is about learning to ask yourself which real states exist, which data belongs to each state, and which decisions must change when the model changes. Those questions come back in lesson 4 with `Result`, in lesson 5 with traits, in lesson 6 with tests, and in lesson 8 when the `revisor` generates JSON.

## The concepts

### Structs: a name for data that belongs together

A `struct` defines a compound type with named fields. The important word is "type": after defining `Servicio`, Rust stops seeing an informal collection of three pieces of data and starts seeing a value that represents a service. Each field keeps its own type, so the compiler still tells text from numbers, but now it also knows that those values form a single entity.

A struct with named fields is a good choice when each position has its own meaning. A tuple like `(String, String, u64)` can store name, URL, and time limit, but it forces you to remember what `.0`, `.1`, and `.2` mean. With `Servicio`, the code says `servicio.timeout_ms`, which communicates both the data and its unit. Clarity is not an ornament: it reduces the chance of swapping similar values and makes it easier to read code you wrote weeks ago.

Creating a struct uses braces and `field: value` pairs. Access is also direct with a dot. Since the fields in the figure belong to `Servicio`, there is no temporary state in which a URL exists without a name or a time limit is accidentally associated with another service. It is still possible to create an incorrect value, for example a URL without a scheme; lesson 4 will teach how to validate that. What disappears is the mess of loose variables.

**Fig. 3.1** | A struct with its methods.

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

`#[derive(Debug, Clone)]` asks the compiler to implement well-known behaviors for the type. `Debug` lets you print a useful representation with `{:?}`. It is not a stable format for end users: it is a view for development and diagnosis. `Clone` lets you request an explicit copy with `.clone()`. In this example it is used only to show that the structure can be printed and then remain available; you shouldn't copy values out of habit to silence ownership errors.

The `revisor` uses the same model, but adds the attributes needed to read services from YAML. `pub` indicates that other modules of the crate can access those fields. `Deserialize` and `serde` will appear in depth in later lessons; for now observe that the core is still the same: name, URL, and time limit.

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

The attribute `#[serde(default = "timeout_por_omision")]` doesn't change what a service is. It describes an input rule: if the YAML doesn't declare `timeout_ms`, the program uses five seconds. It is important to distinguish modeling from validation. The struct declares which data form a service; the rules about whether a URL has a scheme, whether the time is greater than zero, or whether the name is repeated are checked later, when the program receives a list.

### `impl` and methods: behavior that belongs to the type

A `struct` stores data, but the type can also have operations that make sense for that data. Rust groups those operations in an `impl` block. The name means implementation: an implementation of behavior for a type. There is no special reserved word for constructors. By convention, an associated function called `new` creates a new value, but it is still a normal function inside `impl`.

The figure uses two ways of calling functions inside an `impl`. `Servicio::new(...)` uses `::` because `new` doesn't have an instance to work on yet. `s.etiqueta()` uses `.` because `etiqueta` receives a concrete service. Rust allows this method syntax when the first parameter is called `self`, `&self`, or `&mut self`.

`&self` means "lend this value for reading". The method can look at `nombre` and `url`, build a new string and return it, but it doesn't take ownership of `s` or modify its fields. That is why, after `s.etiqueta()`, you can still print `s`, read `s.timeout_ms`, or lend the service to another function. It is the direct application of lesson 2's borrows to a function that lives next to its type.

`&mut self` means "lend this value to modify it". A method like `fn cambiar_timeout(&mut self, ms: u64)` would require the caller to declare a mutable variable and would not allow other active references to the same value. The compiler applies the same rules you already saw with `&mut String`: only one mutable reference at a time, or several immutable references, but not both kinds simultaneously.

`self` without `&` consumes the value. It is a deliberate and less common decision. It is useful when the method transforms one value into another and the original should no longer exist, for example an operation that converts a temporary configuration into a validated structure. It is not a faster way of writing `&self`: it changes who owns the value. If you get a "value moved" error after calling a method, check its receiver first.

In the real project, `Servicio::new` concentrates the default value. That avoids every call having to repeat `5000` and reduces the risk that some services are created with a different rule by accident.

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

`Self` inside `impl Servicio` means `Servicio`. Using it avoids repeating the type name and preserves the intent if the type is renamed during a refactoring. The expression `Self { ... }` builds the value; `-> Self` declares the type the function returns. The constant `TIMEOUT_POR_OMISION_MS` is outside the excerpt and keeps the number five thousand from being scattered through the program as a magic value.

### Enums with data: valid alternatives, not ambiguous fields

An `enum` describes a value that can take one of several variants. The essential difference from a collection of constants is that each variant can have its own shape. `Estado::Ok` and `Estado::Lento` have named fields `codigo` and `ms`. `Estado::Falla` stores a string. `Estado::NoIntentado` carries no data because no query happened that would produce honest results.

This design avoids representing a failure as a special code, like `0`, `-1`, or an empty string. Those markers force you to remember rules outside the type: "if the code is zero, read the error; if the error is empty, maybe it succeeded; if the time is zero, maybe it wasn't attempted". An enum moves those rules to the compiler. If you have `Estado::Falla`, Rust knows there is a message; if you have `Estado::Ok`, Rust knows there is a code and a duration.

It also avoids the impossible combination of optional fields. A struct like `Resultado { codigo: Option<u16>, error: Option<String>, ms: u64 }` admits by construction both `codigo: Some(200), error: Some("no responde")` and `codigo: None, error: None`. There may be legitimate cases for a shape like that, especially when serializing external data, but it is not a good internal representation of mutually exclusive alternatives. For the state of a query, the enum expresses reality better.

<!-- verificar:fragmento -->
```rust
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}
```

The syntax has three forms worth recognizing. Variants with braces are similar to small structs and let you name fields when creating and destructuring. Variants with parentheses are similar to tuples and are useful when the data has a clear main meaning, like the message of a failure in this fragment. Variants without data represent a possibility that needs no additional information.

**Fig. 3.2** | An enum with data and its `match`.

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

The annotation `#[allow(dead_code)]` belongs to the example; it is not a recipe for hiding warnings in real projects. The `Lento` variant keeps `codigo` because a slow response may have been HTTP 200, although this program only uses `ms`. Without the annotation, Rust would warn that the `codigo` field of that variant is not read in this file. The real project does use the data where it belongs and is compiled with warnings treated as errors.

The `revisor` improves on the initial fragment with two domain decisions. First, a failure carries both `motivo` and `ms`, because knowing that a connection timed out after a certain duration is useful information for the report. Second, the enum gets `derive(Debug, Clone, PartialEq)`. `PartialEq` lets you compare statuses in tests with `assert_eq!`, something you will use in lesson 6.

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

The enum doesn't replace every use of booleans or every use of structs. A boolean is still right for a question with two simple answers, like "is the service healthy?". A struct is still right for data that exists together at the same time, like name, URL, and limit. An enum is suitable when the possibilities have different shapes and the program must treat them differently.

### `match`: deciding on each state without leaving gaps

`match` compares a value against patterns and produces a result. In the figure, each arm has the form `pattern => expression`. The pattern identifies a variant and can extract its data. In `Estado::Ok { codigo, ms }`, the names inside the braces create local variables called `codigo` and `ms`. In `Estado::Falla(msg)`, `msg` receives the string the variant carries.

The pattern `..` means "ignore the other fields". In the `Lento` arm, the program needs the duration to print it, but it doesn't need the code. It is better than inventing a name like `_codigo` when you aren't going to use it: it communicates that the data exists and that this decision doesn't depend on it. If you don't need any field of a variant with data, you can write `Estado::Ok { .. }`.

A `match` is an expression. That is why the figure can do `let texto = match estado { ... };`. Each arm returns a `String`: three use `format!` and the last one builds one with `"sin revisar".to_string()`. Rust demands that all branches produce compatible types. That rule prevents one path from returning text and another, by accident, from returning nothing.

The most valuable property is exhaustiveness. The compiler knows all the variants of `Estado` because they are declared in the same type. If a `match` doesn't cover one of them, it doesn't compile. This is safer than a `switch` that allows falling through without action, and it is also more explicit than a chain of `if`s that leaves a case as an implicit possibility.

In some cases you will use a wildcard pattern, `_ => ...`, to group possibilities that really must receive the same treatment. It is valid, but it has a cost: if you add a new variant, that arm will already accept it without making you think about whether the right behavior is the same. For a central enum like `Estado`, it is better to prefer explicit arms in the report. That way a new variant becomes a visible decision, not accidental behavior.

**Fig. 3.3** | If you forget a variant, it doesn't compile.

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

In the `revisor`, `match` appears in the report module to convert a technical state into a label a person can read. Notice that each variant is named explicitly. The code doesn't assume that "everything that isn't OK" is a failure: `Lento` and `NoIntentado` have their own meaning.

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

The parameter is `&Estado`, an immutable reference. The report must read the state several times to get label, duration, and detail, so it isn't a good idea to consume it. Rust allows `match` on a reference: the patterns read the fields they need without moving the original `Estado`. This combination of borrows and patterns will be very frequent in Rust code.

There is also `matches!`, a useful macro when you only want a boolean answer. The project's `esta_bien` method doesn't need to produce text or extract codes; it asks whether the value is one of two healthy variants. Grouping patterns with `|` expresses that rule without repeating logic.

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

Don't use `matches!` to replace a `match` that must transform data. Its result is always `bool`; it is a question, not a complete decision. When the program needs to build a report, get a duration, or choose a specific detail, `match` is still the right tool.

### `Option<T>`: absence declared in the type

Rust has no `null` or `nil`. A value of type `String` is always a valid string; a value of type `&Servicio` is always a valid reference as long as the borrow is valid. When a piece of data can be missing, its type must declare it. The standard form is `Option<T>`.

<!-- verificar:fragmento -->
```rust
enum Option<T> {
    Some(T),
    None,
}
```

The real definition belongs to the standard library and has more internal attributes, but this fragment shows its central idea. `Option<u16>` means "there may be a `u16` code, or there may not". `Option<Servicio>` means "a search can return a service or find none". The type doesn't decide what to do about absence; it forces whoever consumes the value to do so explicitly.

Absence is not always an error. Searching for a service by name may not find it because the name isn't configured. An HTTP header can be optional. A network failure may produce no HTTP code. In those cases, `Option` communicates that the lack of a value is a possibility foreseen by the contract, not a secret value like `0`, `""`, or a null pointer that could blow up later.

**Fig. 3.4** | Consuming an `Option`.

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

The first consumption uses `match` because both cases matter: there is an output for `Some` and another for `None`. The second uses `if let` because there is only work to do when a code exists; if it doesn't, the program doesn't need to do anything. `if let Some(c) = quizas` is a short way of writing a `match` whose other arm would be `_ => {}`.

`unwrap_or(0)` returns the content when it exists and the default value when it doesn't. The decision to use `0` is only correct if the caller understands that zero represents "no response" in that context. In a public HTTP report, it may be clearer to keep `Option<u16>` until the point where the data is presented, so as not to confuse an absence with a real HTTP code.

Don't confuse `unwrap_or` with `unwrap`. `unwrap()` says: "I know there is a value here; if there isn't, end the program with a `panic!`". It can be reasonable in a test where the absence shows that the setup of the case failed, but in application code it usually hides a pending decision. `clippy`, with its default configuration, doesn't warn about a plain `unwrap`; there is an optional lint (`clippy::unwrap_used`) that whoever maintains a project can enable to forbid it. Lesson 6 will show how to run `clippy`. Before writing it, ask yourself whether `None` can occur in production. If it can, you need to handle it.

The project uses `Option` for the JSON form of the report. A failure doesn't have an invented HTTP code, which is why `codigo` is `Option<u16>`. The `error` field is also optional: it appears in a failure or in a not-attempted service, but it is omitted for a healthy response. This struct represents a serializable output; it doesn't replace the internal `Estado` enum; both types have different responsibilities.

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

This separation is useful. `Estado` models exclusive alternatives so that the internal logic is safe. `EstadoJson` models the shape an external tool expects to read, where some fields can be `null` or omitted. Rust doesn't prevent an external API from having optional values; it prevents your internal logic from treating them as if they always existed.

## The error you will see

### `E0004`: a `match` doesn't cover all the patterns

Figure 3.3 produces `E0004`, "non-exhaustive patterns". The compiler saw that `estado` is of type `Estado`, read the definition of the enum, and checked that the variant `NoIntentado` exists. Then it went through the arms of the `match` and found no pattern that covered it.

The arrow under `estado` indicates the value on which the decision is being made. The following note shows where `Estado` was defined and underlines the missing variant. The help proposes two fixes: add an explicit arm for `Estado::NoIntentado` or add a wildcard pattern. For this case, the right fix is the explicit one because "sin revisar" deserves a visible output:

<!-- verificar:fragmento -->
```rust
Estado::NoIntentado => "sin revisar".to_string(),
```

Don't copy `todo!()` from the suggestion as the final solution. Rust proposes it because it completes the pattern and leaves a visible marker for you to decide what to do; if that branch runs, `todo!()` ends the program with a `panic!`. It is useful during a brief refactoring, not as behavior of the `revisor`.

This error also appears when you add a new variant. That is precisely one of its advantages. Instead of depending on a manual search through the repository, let the type and the compiler list the places that must decide how to respond to the new state. Fix each one with a business rule, not with `_ =>` by reflex.

### Reading the error as a guide to change

Rust errors usually have four parts: the stable code like `E0004`, the location, notes with context, and a help. Start with the code and the main sentence; in this case they are enough to know that a variant is missing. Then read the note to confirm the type and the help to learn a valid syntactic way to fix it.

The compiler's help doesn't know your domain. It can tell you how to complete a `match`, but it can't decide whether a new state should count as healthy, failed, slow, or not checked. That decision is still yours. The advantage is that Rust separates both problems: it guarantees that you didn't forget to handle the case and lets you define the right treatment.

You can ask for an extended explanation with `rustc --explain E0004`. Do it when you run into an error code you don't understand. You don't need to memorize codes; what matters is learning to recognize that they are searchable identifiers and that the message contains concrete evidence about the type and the line involved.

## What goes wrong

### Modeling alternatives with a struct full of optional fields

A struct like `Resultado { codigo: Option<u16>, error: Option<String>, ms: Option<u64> }` looks flexible, but it accepts too many incoherent states. It can hold both a success code and a failure message at once, or hold neither. If those cases are invalid, the type should make them hard or impossible to build.

Use an enum when the possibilities exclude each other and carry different data. Keep structs with `Option` for external boundaries, like JSON, forms, or partial configurations, where you really need to represent fields that can be missing independently.

### Using `_ =>` to silence exhaustiveness

The wildcard pattern is right when all the remaining variants receive exactly the same treatment. The problem appears when it is used only to make the compiler stop complaining. In a business enum, `_` can turn a new variant into a generic failure or, worse, into a healthy response by accident.

In `Estado`, write the four arms explicitly. If you add a variant, accept that the compiler makes you review the report and the tests. The small immediate work avoids unreviewed behavior later.

### Writing `unwrap()` on a normal path of the program

`unwrap()` doesn't resolve absence: it turns it into a `panic!`. If a service may not be found, a response may carry no code, or a file may not exist, absence is part of the program's reality. It must become a `match`, an `if let`, a justified default value, or, in lesson 4, a `Result`.

In tests, `unwrap()` can be useful to state that a case must be set up correctly. In production logic, use it only when you have shown that `None` is impossible and the failure represents a programming error, not an expectable condition.

### Copying with `clone()` to avoid thinking about ownership

`Clone` is not an automatic way out of a move error. Copying a `Servicio` just to lend it to a function duplicates its `String`s and can hide that the function should receive `&Servicio`. In figure 3.1, `clone()` exists to demonstrate the trait and make the structure visible; it is not the recommended way to pass services around the program.

Prefer borrows for reading (`&Servicio`), mutable borrows when there is a real modification (`&mut Servicio`), and moves when the function must take ownership of the value. Copy only when the program truly needs two independent values.

### Using magic numbers or strings to represent states

Representing a failure with `codigo == 0`, a pending query with `ms == 0`, or an error with `mensaje == ""` forces you to remember conventions the type doesn't express. It also makes it harder to answer simple questions: can a real response take zero milliseconds? is an empty message a failure or the absence of failure?

Name the state with a variant. `Estado::NoIntentado` communicates more than a special number and lets `match` force you to handle it. When the state has data, put it inside the variant that makes it valid.

### Confusing `Option` with `Result`

`Option<T>` answers "is there a value or not?". `Result<T, E>` answers "was there a value or was there an error I need to know about?". A search that doesn't find a name can return `Option<Servicio>`; reading a file that doesn't exist should normally return `Result<String, Error>`, because the caller needs to know what went wrong. Lesson 4 goes deeper into `Result` and `?`.

Don't invent error messages inside `Option` or use `None` to hide a failure the user needs to diagnose. Choose the type according to the operation's contract.

## Exercises

### Exercise 1 — Describe a service

Create a `struct ServicioLocal` with `nombre: String`, `url: String`, and `timeout_ms: u64`. Write an associated function `new(nombre: &str, url: &str) -> Self` that assigns `3000` as the default time. Add a method `etiqueta(&self) -> String` that returns `nombre (url)`.

In `main`, create a service called `pagos`, print the label, and then print the time limit. Check that you don't need `mut` or `clone()` for these operations.

### Exercise 2 — Summarize all the states

Declare an enum `EstadoLocal` with the variants `Ok { codigo: u16, ms: u64 }`, `Lento { codigo: u16, ms: u64 }`, `Falla(String)`, and `NoIntentado`. Write `fn resumen(estado: &EstadoLocal) -> String` using an exhaustive `match`.

The summary must use exactly these forms: `OK 200 en 80ms`, `LENTO 1200ms`, `FALLA: sin conexión`, and `sin revisar`. Test it with one instance of each variant.

### Exercise 3 — Add a variant and let Rust find the work

Add `Rechazado { codigo: u16, ms: u64 }` to `EstadoLocal`. Compile without modifying `resumen` and observe `E0004`. Then add the arm that produces `RECHAZADO 403 en 15ms`.

Don't use `_ =>`. The goal is to verify that the compiler points to a pending business decision. Explain in one sentence why `Rechazado` should not be classified automatically as `Falla`: the server did respond, but the response was not accepted.

### Exercise 4 — Search without using `nil`

Create an array or vector of two `ServicioLocal`: `catalogo` and `pagos`. Write a function that receives a slice of services and a name, and returns `Option<&ServicioLocal>`. Search first for `pagos` and then for `reportes`.

Consume the first result with `if let` to print its URL. Consume the second with `match` to print `no existe reportes`. Don't use indexes with a sentinel value, null references, or `unwrap()`.

## Solutions

### Solution 1

`ServicioLocal` must be a struct with three named fields. The `new` function must use `Self` and convert `nombre` and `url` from `&str` to `String`; the `etiqueta` method must receive `&self`, since it only reads the fields. A correct output contains:

```text
pagos (http://localhost:8091/ok)
timeout: 3000 ms
```

If you need to declare `let mut servicio`, review the exercise: no requested operation modifies the value. If you need `clone()`, you probably changed a signature to receive `self` when it should have received `&self`.

### Solution 2

`resumen` must receive `&EstadoLocal` to read the state without consuming it. It must have four explicit arms. The `Ok` arm extracts `codigo` and `ms`; the `Lento` one can use `ms` and ignore the code with `..`; the `Falla` one extracts the message; the `NoIntentado` one returns the fixed text.

The four calls must produce these lines:

```text
OK 200 en 80ms
LENTO 1200ms
FALLA: sin conexión
sin revisar
```

If one branch returns `&str` and the others return `String`, make them all produce the same type. `format!` returns `String`; for a fixed label you can use `.to_string()`.

### Solution 3

When you add `Rechazado`, compilation must fail with `E0004` until you add an explicit arm. The correct arm extracts the two fields and produces:

```text
RECHAZADO 403 en 15ms
```

The solution is not to change the last arm to `_ => "FALLA"`. That form would make the program compile, but it would lose the difference between a network that is down and a server that responded with an authorization policy. The enum offers a new possibility; the `match` must turn it into an explicit decision.

### Solution 4

The search function must return `Option<&ServicioLocal>`, not `Option<ServicioLocal>`. The reference lets you lend the service found from the list without copying its strings or moving it out of the vector. A search can walk through the services and return the first one whose `nombre` matches; if it finds none, it returns `None`.

For `pagos`, `if let Some(servicio)` must print its URL. For `reportes`, a `match` must include both arms and produce:

```text
no existe reportes
```

If the compiler complains about lifetimes, check the signature: the output reference must come from the input slice. In most of these cases, Rust can infer the right lifetime without your writing it. Lesson 5 will explain the cases where you must declare it.

## How do I know I got it

- [ ] I compile figure 3.1 with `rustc --edition 2024 fig03_01.rs && ./fig03_01` and get the three documented lines.
- [ ] I compile figure 3.2 and can explain why each variant of `Estado` carries different data.
- [ ] I compile figure 3.3, see `error[E0004]`, and make it compile by adding an explicit arm for `NoIntentado`.
- [ ] I can add a variant to my enum and locate every pending decision through the `match` errors.
- [ ] My exercise 4 returns `Option<&ServicioLocal>` and handles both `Some` and `None` without `unwrap()`.
- [ ] I can explain why the `revisor` uses an enum for `Estado` and a struct with `Option` for `EstadoJson`.
- [ ] I completed the Rustlings exercises `structs`, `enums`, and `options`.

## Further reading

- [The Rust Programming Language, chapter 5: Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html) — accessed October 2, 2026.
- [The Rust Programming Language, chapter 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) — accessed October 2, 2026.
- [Official documentation of `std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html) — accessed October 2, 2026.
- [Rustlings: structs, enums, and options exercises](https://github.com/rust-lang/rustlings/tree/main/exercises) — accessed October 2, 2026.
