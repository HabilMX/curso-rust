# Lesson 1 — Fundamentals

**Time:** 2 × 45 min.

**What you build:** the functions and base types of the `revisor`

**What you learn:** variables and `mut`, shadowing, scalar and compound types, functions, "everything is an expression", `if`, `loop`, `while`, and `for`

## By the end you will be able to

- Declare immutable variables, mutable variables, and constants, and explain when each one is appropriate.
- Choose numeric types, booleans, characters, tuples, arrays, vectors, and strings for simple `revisor` data.
- Write functions with parameters and return values without relying on implicit conversions.
- Explain why a block, an `if`, and a `loop` can produce values.
- Use `if`, `loop`, `while`, and `for` to classify and walk through data in a readable way.
- Read and fix two common variants of the `E0308` error.
- Solve the Rustlings exercises `variables`, `functions`, `if`, and `primitive_types`.

## The why before the how

The `revisor` you will build during the course receives a list of services, queries them, and reports what happened. Although in the end it will have HTTP, YAML files, concurrency, and JSON output, its core starts with much smaller operations: storing a response time, comparing it against a limit, walking through a list, and deciding which text to show. Before modeling a service with a `struct` or a failure with an `enum`, you need to be able to express those operations precisely.

Think of an initial rule of the program: a response of up to a thousand milliseconds is considered normal; a slower one is reported as slow. The rule looks simple, but it contains several decisions that Rust wants you to declare: the time can't be text, it must be a number; the threshold must have a compatible type; the comparison must produce a boolean condition; and the function must always deliver a classification. Rust doesn't let those decisions hide in automatic conversions or ambiguous values. The compiler asks that the program say what each piece of data represents.

That insistence can feel heavy if you come from Go, Python, or JavaScript. Go also has static types and conversions between numbers are explicit, but Rust extends that precision to other parts of the syntax. A variable is immutable by default. A block can return a value. An `if` must produce values of the same type in both branches when it is used as an expression. A `for` distinguishes between walking through a collection, borrowing it, or consuming it. At first these are more visible decisions; later they are information that keeps someone from misreading your intent when maintaining the program.

The useful mental model is not "Rust puts up obstacles before running". It is "Rust turns design decisions into things that can be checked". If you name a measurement `u64`, the compiler knows it can't be negative. If you make a variable mutable, the reader knows it will change. If a function returns `&'static str`, it is clear that it returns one of a few fixed labels and not freshly built text. If the result of an `if` is stored in a variable, all its branches must describe the same kind of result. Most of what follows in the course rests on that same idea.

This lesson works through chapters 2 and 3 of *The Rust Programming Language*. Chapter 2 presents `let`, functions, and the use of `mut` inside a small program; chapter 3 organizes the fundamentals: variables, data types, functions, comments, and control flow. Don't try to memorize every available type in one sitting. What matters is learning to read a signature, choose a reasonable representation, and let the compiler point out the contradictions.

The real program already contains these fundamentals. In `programas/revisor/src/modelo.rs` there are limits expressed as constants; in `src/config.rs` there is a `for` that validates each service; in `src/revisar.rs` there are conditions that classify responses; and in `src/reporte.rs` there are mutable variables for building an output. This lesson doesn't modify that project: it uses it as a map of where the small pieces you will practice here are headed.

## The concepts

### Variables, `mut`, constants, and shadowing

A variable is declared with `let`. By default, the binding between the name and its value is immutable: after writing `let x = 5;`, you can't assign another value to `x`. This choice is deliberate. When you read a long function, every name that doesn't carry `mut` gives you a local guarantee: that name will keep representing the same value for the rest of its scope.

Immutability doesn't mean Rust forbids changing data. It means you have to declare it. If a variable represents a counter, an output you build up little by little, or an index that decreases, use `let mut`. The word `mut` goes next to the name because it describes the binding, not the whole function. Avoid putting `mut` out of habit: a mutable variable that never changes produces a warning, and compiling the examples with `-D warnings` turns that warning into an error. It is a small signal, but a useful one: the code says something will vary and in reality it doesn't.

Constants are written with `const`, carry an explicit type, and are evaluated before the program runs. Use them for rules whose name should appear throughout the code: a time limit, a capacity, or a maximum number of attempts. A constant is not an immutable variable with another name. It doesn't occupy a single memory location that you can borrow or modify; it is substituted where it is used. At this stage it is enough to remember the practical rule: `let` for local values and `const` for a stable, named rule.

**Fig. 1.1** | Variables, mutability, and constants.

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

The type of `x` is written as `i32`, but Rust could infer it here because `y += x` and the literal `10` give enough context. Annotating types helps when a signature is part of an API, when the compiler can't infer them, or when you want to communicate an important constraint. Don't annotate them mechanically in every `let`: well-used inference reduces noise without losing safety.

Shadowing is different from mutability. With shadowing you declare a new variable with the same name; the previous one is no longer accessible from that point. It is useful when an idea goes through stages and you want to keep an honest name. For example, a text with spaces and the number of spaces are two different values, but both can be called `espacios` because the first version is no longer needed. Unlike `mut`, shadowing allows the type to change.

**Fig. 1.2** | Shadowing, tuples, and arrays.

```rust
// fig01_02.rs
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
$ rustc --edition 2024 fig01_02.rs && ./fig01_02
espacios = 3
codigo = 200, ms = 750, saludable = true
primer servicio = catalogo
```

Here the first `espacios` is `&str`, a view of text; the second is `usize`, a count. It is not that one variable mutated from text to number: they are two different bindings, with overlapping scopes. This difference matters later with ownership. `let mut nombre` keeps the same value and lets you modify it; `let nombre = ...` binds the name again and can transform the value without keeping the previous version.

In the `revisor`, a mutable variable appears when building the table that will be printed. The name `salida` doesn't represent a fixed rule: it is an accumulator to which lines are added, so `mut` communicates exactly the intent.

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

You don't need to understand references, iterators, or `format!` yet to recognize the fundamental decision: `filas` and `ancho` don't change; `salida` does. In lesson 2 you will study why `&[Servicio]` and `&[Estado]` are borrows, and in lesson 4 you will see how `String` lets you build dynamic text. For now, identify the pattern: declare it immutable until a modification is a real part of the work.

The `revisor`'s classification rules are also constants. The value is not repeated as an anonymous number in every comparison; it has a name, a type, and a comment. When the "slow" policy changes, there will be an obvious place to review.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;
```

`TIMEOUT_POR_OMISION_MS` is private to the module because it is only used to create services by default. `UMBRAL_LENTO_MS` carries `pub` because another module, `revisar.rs`, needs to consult it. Module visibility is studied formally in lesson 6; what matters today is that both values have explicit types and names that express units. A bare `1000` leaves questions: a thousand seconds, a thousand bytes, a thousand milliseconds? `UMBRAL_LENTO_MS` answers them.

### Scalar and compound types

Rust is a statically typed language: before running, the compiler knows the type of every value. Sometimes it infers it and sometimes you must annotate it, but it never treats a number as text or mixes two integer sizes because they "more or less look compatible". That rigor lets many mistakes be caught before a binary is produced.

Scalar types hold a single value. The signed integers are `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`; the unsigned ones are `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`. The number says how many bits the value takes. `isize` and `usize` change with the architecture and are used mainly for sizes, lengths, and indexes. For the `revisor`'s millisecond quantities, `u64` is an explicit decision: there are no negative times and the range is wide. For an HTTP code, `u16` is enough. Choosing a type doesn't mean looking for the smallest possible number; it means expressing the domain of the data sensibly.

`i32` is the default integer type when the compiler receives no more context. It is a good general choice for local integer calculations. Don't assume all integers are `i32`: a `usize` that comes from `len()` can't be added directly to a `u64`, and a `u16` HTTP code doesn't become `i32` just because it is in the same operation. The advantage is that you see the crossing of domains at the exact point where it happens.

Rust does no implicit numeric conversions. This is not an isolated quirk: it prevents a seemingly innocent assignment from changing size, sign, or range without the person who wrote the code having considered it. In Go, conversions between numeric types are also requested explicitly; Rust keeps that discipline and makes it especially important because its integer types are often used to represent capacities, lengths, and network data.

**Fig. 1.3** | There is no implicit conversion, not even between numbers.

```rust
// fig01_03.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a;             // ← no compila
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_03.rs
error[E0308]: mismatched types
 --> fig01_03.rs:4:18
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

For a simple, known conversion you can use `as`. The conversion from `i32` to `i64` is safe in this case because every `i32` fits in an `i64`. However, `as` also allows conversions that can truncate, reinterpret the sign, or lose precision. Don't use it as a way to "make the compiler be quiet". When a conversion can fail or lose information, later you will meet `TryFrom`, `TryInto`, and `Result`.

**Fig. 1.4** | The conversion is requested with `as`.

```rust
// fig01_04.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a as i64;      // así
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_04.rs && ./fig01_04
5
```

Besides integers, the scalars include `f32` and `f64` for floating-point numbers, `bool` for `true` or `false`, and `char` for a Unicode character. For the `revisor`, avoid floating point if an integer expresses the unit better. Storing `750` milliseconds as `u64` is clearer than storing `0.75` seconds as `f64`, and it avoids questions about rounding when you display, compare, or serialize the value.

Compound types group several values. A tuple can hold elements of different types and has a fixed size. In figure 1.2, `(u16, u64, bool)` represents three results that belong to one measurement: code, duration, and health status. The destructuring `let (codigo, ms, saludable) = medicion;` extracts those values with useful names. Tuples are suitable for small, local results; when the meaning of the fields is central to the program, as it will be for a service, a struct with named fields will be better. That comes in lesson 3.

An array like `["catalogo", "pagos"]` contains values of the same type and has a fixed length known at compile time. A vector, `Vec<T>`, also contains values of the same type, but it can grow or shrink at run time. Figure 1.7 uses `vec!` because the list of services is a collection that conceptually can change size. In the real project, the configuration is loaded from YAML and produces a `Vec<Servicio>` for the same reason.

Strings also require precision. A literal like `"catalogo"` is usually `&str`, a borrowed view of text that already exists. A `String` is text that owns memory and can grow. In this lesson you will see `&str` as the output value of fixed labels; in lesson 2 you will study why not all texts can be copied and why the two types are distinguished. For now, keep this rule: fixed text written in the code is usually `&str`; text that is read, built, or stored usually ends up as `String`.

The `revisor` makes its numeric vocabulary explicit. The time limit is stored as `u64` and the HTTP code as `Option<u16>`. You don't need to master `Option` yet; lesson 3 will explain why it replaces `nil`. Today it is enough to observe that the type describes a constraint of reality: there may be no HTTP code if no response arrived.

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

The type is not decorative documentation. `ms: u64` prevents assigning a string to it; `codigo: Option<u16>` prevents treating the absence of a response as if it were automatically a `200`; `servicio: String` indicates that the name is text the program owns. Rust will use that information throughout compilation.

### Functions, parameters, and return values

A function is declared with `fn`, has a name, parameters in parentheses, and a body in braces. Parameters always carry a type: `fn doble(x: i32)` says both the name of the data and what the function can receive. If it returns something other than `()`, that is indicated after an arrow: `-> i32`. This signature is a short, verifiable contract. Whoever calls the function knows what to provide and what they will get; the compiler checks both ends.

Unlike Go, Rust writes the type after the parameter name, not before. In Go you would write `func doble(x int) int`; in Rust, `fn doble(x: i32) -> i32`. The visual difference stops mattering after a few functions. What matters is that in both languages the signature is part of the design: it is not a comment or an informal convention.

A small function shouldn't take on work that isn't its job. `doble` receives a number and returns another; it doesn't print, doesn't read files, and doesn't modify external state. That separation seems basic, but it prepares the ground for the `revisor`: a function that classifies milliseconds can be tested with three numbers without starting an HTTP client or opening a configuration. As the program grows, splitting the logic into functions with clear inputs and outputs will be a way to keep it understandable.

**Fig. 1.5** | Everything is an expression.

```rust
// fig01_05.rs
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
$ rustc --edition 2024 fig01_05.rs && ./fig01_05
grande 49 14
```

`main` is also a function. In an executable program it starts with no parameters and doesn't need to declare a return if it simply finishes. In contrast, `doble` promises an `i32`, so the last value of its body must be compatible with `i32`. You can use `return x * 2;`, but it is not the usual form for a function's last value. Rust favors the final expression because it keeps visible what result the body produces.

Parameters are passed in different ways depending on the type and the intent. Scalar types like `i32`, `u64`, and `bool` are copied cheaply; receiving `ms: u64` doesn't stop the caller from continuing to use its measurement. With `String`, vectors, and more complex structures, the move and borrow rules of lesson 2 will appear. Don't get ahead of them by solving everything with copies. For now use scalar parameters to practice clean signatures, and recognize that the `&str` of a fixed label has a different life from a `String` that is built.

The real project has a small function that converts a textual configuration into program data. Although it uses libraries you will study later, its signature shows the essential pattern: it receives an input, returns a result, and its body ends with an `Ok(...)` expression.

<!-- verificar:extracto:src/config.rs -->
```rust
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}
```

You don't need to take apart `Result`, `?`, or `yaml_serde` yet; they will arrive in lesson 4. What you can already read is the shape: `ruta` comes in as a view of text, the function promises to return a list of services or an error, `txt` is an immutable local value, and `Ok(...)` is the final result. Signatures let you understand a function's boundary even before knowing all of its internal details.

### Expressions, statements, and the semicolon

In Rust, many constructs produce a value. An arithmetic operation like `x * 2` produces a number; a block in braces can produce the last value it contains; an `if` can produce one of two values; and a `loop` can end with a value sent by `break`. These constructs are called expressions.

A statement performs an action but doesn't produce a useful value. A declaration `let x = 7;` is a statement. So is an expression to which you add a semicolon. The value of a statement is `()`, called the unit type. You can think of `()` as "there is no result to deliver". It is not an error or a null value: it is a real type that appears when an operation is used only for its effect.

The semicolon determines that difference in important places. In figure 1.5, the block assigned to `cuadrado` ends with `t` without a semicolon, so the block produces the value of `t`. The function `doble` ends with `x * 2` without a semicolon, so it returns that `i32`. If you add `;`, the operation runs and its result is discarded. Then the function body produces `()`, but the signature demands `i32`.

**Fig. 1.6** | The extra semicolon.

```rust
// fig01_06.rs
fn doble(x: i32) -> i32 {
    x * 2;                     // ← el punto y coma de más
}

fn main() {
    println!("{}", doble(4));
}
```

```bash
$ rustc --edition 2024 fig01_06.rs
error[E0308]: mismatched types
 --> fig01_06.rs:2:21
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

This error is baffling once and very useful afterward. The compiler is not saying the multiplication is invalid; it says the function promised `i32` and ended up producing `()`. Read the two parts of the message: `expected i32, found ()` identifies the contradiction, and the help proposes removing the semicolon. Don't add a `return` without understanding why; in this case the problem is that you discarded the correct value.

The expression style makes small transformations compact and clear. You can compute an intermediate value in a block, keep the local variables inside that block, and deliver only the result. This reduces unnecessary scopes and avoids temporary names that stay alive when they no longer mean anything. Don't turn every line into a complicated expression: readability is still the criterion. A block with two or three well-named steps is usually clearer than a clever line.

In `revisar.rs`, the classification of a response uses conditions inside a `match`; the result of each branch is an `Estado`. Although `match` is studied in depth in lesson 3, the pattern is already familiar: each path produces the value the function promised.

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

The part that belongs to this lesson is the `if` conditions and the idea that a control construct ends in a value. The new part is `match`, `Result`, and the variants of `Estado`. You don't need to copy them yet; just recognize that the syntax you practice with numbers will end up making real decisions about services.

### `if`, `loop`, `while`, and `for`

`if` evaluates a condition that must be `bool`. Rust doesn't consider `0`, an empty string, or a null reference to be false automatically. Write a comparison or use a boolean variable. This decision avoids accidental conditions and makes it evident which property you are asking about: `ms > UMBRAL_LENTO_MS` communicates a rule; `if ms` would have no meaning.

When `if` is used to choose a value, its branches must return the same type. You can't return `"rápido"` in one branch and `1000` in another, because the variable that receives the result must have a consistent representation. This restriction is exactly the kind of decision that becomes useful in a report: a classification will always be text, an exit code will always be a suitable integer, and a status will always be a variant of the same type.

`loop` starts an infinite cycle. It seems like an extreme tool, but it is appropriate when you don't know the number of iterations in advance and the natural exit is `break`. Unlike other languages, `break valor` can provide the result of a `loop`. This is useful when the cycle searches for or computes something; the value found comes out directly as the result of the expression.

`while condicion` repeats as long as the condition is true. Use it when progress depends on a state you control: decrementing a count, reading until a condition, or retrying under an explicit rule. Make sure the body can change the state that makes the condition false. A `while` whose counter is never updated is an infinite loop in disguise.

`for` is the normal option for walking through a collection or a range. Rust doesn't have the traditional `for initialization; condition; update` style of C, Java, or Go. Instead, it walks through something that knows how to hand out its elements one at a time (in Rust, something that implements `IntoIterator`): a range like `0..10`, a list, an array, or an iterator. This form eliminates a lot of index code and reduces boundary errors.

**Fig. 1.7** | The loops.

```rust
// fig01_07.rs
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
$ rustc --edition 2024 fig01_07.rs && ./fig01_07
0 1 2 3 4 5 6 7 8 9 
0 1 2 3 4 5 6 7 8 9 10 
catalogo
pagos
reportes
x = 0, r = 42
```

Ranges are a common source of boundary errors. `0..10` includes `0` and excludes `10`, so it has ten values: from zero to nine. `0..=10` includes both ends and has eleven values. To walk through the positions of an array of length ten, you almost always want `0..10` or, better still, to walk directly through the elements. Use the inclusive range only when the final limit is part of the rule and should appear.

The line `for s in &servicios` carries a reference to the list. That lets you read each element without handing over ownership of `servicios`. The full difference between `servicios`, `&servicios`, and `&mut servicios` is the topic of lesson 2, but you can adopt a provisional rule from today: if you only want to look at a collection and keep it, walk through it by reference. The compiler will prevent unsafe uses once you know the borrow rules.

The `revisor` validates a list with a `for`. The function doesn't need to know how many services arrived: it takes them one by one. `enumerate()` adds the index so that the current service can be compared with the previous ones. Although the whole expression looks advanced, its flow is the same as in figure 1.7: walk through, check a condition, and end with a result.

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

Here `s.timeout_ms > 0` is a boolean condition like the ones you have already used. The difference is that, instead of printing a label, `anyhow::ensure!` stops the validation with an error if the condition is false. Lesson 4 will explain `Result` and this kind of error handling. Lesson 2 will explain the references in `&[Servicio]`. You can already read the intent without knowing every detail: all services must have a URL with a scheme, a name that is not repeated, and a limit greater than zero.

## The error you will see

### `E0308`: mismatched types

`E0308` means Rust expected one type at a point in the program and found another. It is not a vague message: read it as a sentence with two parts. First identify the place where the expectation was set; then identify the value that contradicts that expectation. In figure 1.3, the annotation `let b: i64` sets that `b` will be `i64`; the variable `a` is `i32`; that is why the assignment fails.

The fix will not always be `as`. To convert from `i32` to `i64`, widening is safe and `as i64` communicates the intent. To convert from a large number to a small one, or from text to an integer, you must decide what to do when the value doesn't fit or doesn't have a valid format. Those conversions will be handled with results that can fail. The good practice is to resolve the mismatch at the boundary between domains, not to convert values repeatedly inside each function.

Figure 1.6 produces the same `E0308` code, but for a different cause: the function declares `-> i32` and its last element is a statement whose value is `()`. This difference illustrates why you shouldn't resolve errors by number alone. The code groups a family of diagnostics; the lines pointed to and the words `expected` and `found` tell the concrete story.

When you see `expected i32, found ()`, ask these questions: did the function promise a return? does the last value have a semicolon? does one branch of an `if` not return the same as the other? did I put `println!` as the last element when I needed to produce a value? In that order you will normally find the problem without searching for answers at random.

### Reading a suggestion without blindly obeying it

Rust usually offers a `help:` section. It is a contextual proposal, not an order. In figure 1.3 it suggests `a.into()`, which can also convert the value because there is a known conversion between the two types. Figure 1.4 keeps `as i64` because it is the form we want to teach for an explicit, simple numeric conversion. In other cases, the suggestion may be `clone()`, adding a reference, or changing a signature. Before accepting it, ask yourself what cost, ownership, or behavior it is introducing.

The message also ends with `rustc --explain E0308`. That command opens a general explanation of the error code installed with your compiler. Use it when the local diagnostic isn't enough, but start with the file, line, and columns that the compiler already showed you. They almost always contain more specific information about your program than a general search.

## What goes wrong

### Declaring everything as `mut`

Declaring every variable with `mut` to "have freedom" erases information. If a name doesn't change, the reader shouldn't have to trace the whole function to find out. Besides, the compiler warns when `mut` isn't needed. Declare mutable only what the algorithm modifies, like `x` in a countdown or `salida` when building a report.

### Using `as` to silence type errors

A conversion with `as` can be correct, but it is not a universal cure. Converting a large `u64` to `u16` can lose data; converting a signed integer to an unsigned one can produce a surprising value. Define what each number represents and convert once, at the edge where you change domains. If the conversion can fail, the program should express that instead of hiding it.

### Using numbers with no units or name

An `if ms > 1000` works, but it forces you to remember what `1000` represents. Is it milliseconds, seconds, or bytes? Use a constant like `UMBRAL_LENTO_MS` when the value is a business rule. For obvious local values, a literal can be fine; for a policy that will be repeated or will change, a name avoids errors and improves reading.

### Adding a semicolon to the last expression by reflex

In many languages every line ends with a semicolon, or the convention invites you to use it. In Rust, the last semicolon of a function or block changes its value to `()`. Don't memorize an exception; recognize the rule: a final expression without a semicolon can be the result. If a block exists to compute something, check what it leaves as its last expression.

### Writing `for i in 0..lista.len()` when you only need elements

Walking through indexes works, but it adds an unnecessary way to make a mistake. If you only need each service, write `for servicio in &servicios`. Use `enumerate()` when the index is a real part of the logic, as in the `revisor`'s validation. Use direct indexes when you must access specific positions and can justify the bounds.

### Using `loop` when the number of steps is already known

A `loop` with several exit conditions can be correct, but if you have a collection or a known range, `for` expresses the intent better. If it depends on a changing condition, `while` usually shows the termination criterion more clearly. Reserve `loop` for processes that really wait for an exit through `break`, like an event reader or a search that ends when it finds the data.

## Exercises

### Exercise 1 — Classify a response

Write `fn clasificar(ms: u64) -> &'static str`. It must return `"rápido"` if the time is less than or equal to `1000`, `"lento"` if it is greater than `1000` and less than or equal to `5000`, and `"timeout"` if it is greater. Use an `if` as an expression: don't use `return`. From `main`, print the classification of `700`, `1500`, and `6000`, one per line.

Before looking at the solution, verify that the three branches return the same type. The signature doesn't need to create a `String`: the three labels are fixed literals and so they can be `&'static str`.

### Exercise 2 — Sum without consuming the list

Write `fn sumar(valores: &[i32]) -> i32` that uses `for` to sum a slice. From `main`, create `let valores = vec![3, 5, 8];`, print the result, and then print the length of `valores`. The second print must compile: it shows that the walk didn't consume the vector.

Make only the accumulator mutable. Don't make the vector mutable: you are not adding, removing, or modifying its elements.

### Exercise 3 — A minimal revisor report

Declare `const UMBRAL_LENTO_MS: u64 = 1000;` and write `fn etiqueta(ms: u64) -> &'static str` that returns `"OK"` up to the threshold and `"LENTO"` above it. In `main`, use an array with `[120_u64, 1000, 1500]` and a `for` to print exactly these lines:

```text
120ms: OK
1000ms: OK
1500ms: LENTO
```

Then change the array's type to `i32` without changing the signature of `etiqueta`. Read `E0308`, fix it explicitly, and explain in your own words why Rust didn't do the conversion for you.

## Solutions

### Solution 1

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

The whole `if` is the function's final expression. Each branch returns a literal of type `&'static str`, so the signature and the result match. The order matters: the second condition is only evaluated if the first was false, so there is no need to repeat `ms > 1000`.

### Solution 2

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

`valores` receives a reference to a slice, so the function observes the numbers without keeping the caller's vector. Inside the `for`, `valor` is a reference to each `i32`; the sum works because `i32` implements addition with a reference to another `i32` (`total += valor`), without your having to write `*valor`. The last expression, `total`, delivers the result without a semicolon.

### Solution 3

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

The `_u64` suffix on the first literal fixes the type of the array. The other elements must be of the same type, so Rust interprets them as `u64` too. The constant expresses both the value and the unit of the rule. If you changed the array to `i32`, you would have to convert each item explicitly or change the function's contract; both decisions have meaning and shouldn't happen by accident.

## How do I know I got it

- From `programas/01-fundamentos`, `rustc --edition 2024 -D warnings fig01_01.rs && ./fig01_01` prints `x = 5, y = 15, MAX = 100000` with no warnings.
- `rustc --edition 2024 fig01_03.rs` fails with `error[E0308]`, and you can point out that `b` expects `i64` while `a` is `i32`.
- `rustc --edition 2024 fig01_06.rs` fails with `error[E0308]`, and you can explain that the semicolon made the function return `()`.
- `rustc --edition 2024 -D warnings fig01_07.rs && ./fig01_07` prints the two ranges, the three services, and ends with `x = 0, r = 42`.
- `rustc --edition 2024 -D warnings fig01_02.rs && ./fig01_02` prints the three documented results and reports no warnings.
- You finished the Rustlings sections `variables`, `functions`, `if`, and `primitive_types`, and you can solve the three exercises without copying the solutions.

## Further reading

- [The Rust Programming Language, chapter 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) — accessed October 2, 2026.
- [The Rust Programming Language, chapter 3: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) — accessed October 2, 2026.
- [Official documentation of `i32` and the primitive numeric types](https://doc.rust-lang.org/std/primitive.i32.html) — accessed October 2, 2026.
- [Rustlings](https://rustlings.rust-lang.org/) — complete `variables`, `functions`, `if`, and `primitive_types`; accessed October 2, 2026.
