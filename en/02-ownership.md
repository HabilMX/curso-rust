# Lesson 2 — Ownership

**Time:** 2 × 45 min.

**What you build:** programs that collide with the compiler on purpose

**What you learn:** the three rules of ownership, move versus copy, `&` and `&mut` borrows, the two rules of borrowing, why there is no garbage collector

## By the end you will be able to

- Explain the three rules of ownership and use them to anticipate when Rust drops a value.
- Tell an implicit copy from a move, and justify when to use `clone()`.
- Choose whether a function should receive a value, a `&T` reference, or a `&mut T` mutable reference.
- Apply the "many reads or one write" rule and fix the `E0382` and `E0502` errors.
- Explain why Rust needs neither a garbage collector nor manual calls to `free`.
- Write a function that receives `&str` and returns a borrowed portion of the text.
- Solve the Rustlings exercises `move_semantics` and `primitive_types`.

## The why before the how

Lesson 2 is the point where Rust stops looking like simply a compiled language with a syntax different from Go's. The variables, functions, types, and control flow of the previous lesson are recognizable. Ownership changes a question that many languages hide: when a program creates a piece of data in memory, who must destroy it, and when?

The `revisor` will store service names, URLs, failure messages, lists, and reports. All of that data can grow at run time. The name of a service read from YAML doesn't have a known size when you compile; it needs dynamic memory. The final table of the report is also built up little by little. In C or C++, whoever writes the program would have to allocate and free that memory manually. If you free twice, the program can be corrupted. If you don't free it, it leaks memory. If you keep a pointer after freeing, you can read an area that already belongs to something else.

Go makes another decision. The program can create data and forget to free memory because the garbage collector watches which objects are still reachable and reclaims the rest. That makes writing programs more direct, but adds a runtime component that manages memory, decides when to work, and consumes resources to find objects that are no longer useful. In most Go programs that decision is excellent: it reduces complexity and avoids serious errors.

Rust looks for another combination: safe memory without a garbage collector and without manual freeing. Its proposal is to check, before generating the executable, who owns each value and who can access it. If the compiler can prove that a value will no longer be used, it inserts the proper release when it leaves its scope. If it can't prove that a reference will remain valid, it rejects the program. If it detects two incompatible accesses to the same piece of data, it also rejects it.

This doesn't mean Rust "guesses" what you wanted to do. On the contrary: it requires your intent to be visible in signatures and assignments. A function that receives `String` takes the value; one that receives `&str` only consults it; one that receives `&mut String` can modify it during an exclusive borrow. The information that in Go sometimes stays in a convention, a comment, or a code review is, in Rust, part of the type.

The price is real. At the start you will write code that looks reasonable but doesn't compile. The normal reaction is to try adding `.clone()` until the error disappears. Sometimes a copy is the right decision; many times it is a sign that the function asked for more ownership than it needed. Learning ownership consists of ceasing to treat those errors as obstacles and starting to read them as design questions: who should keep this value? how long does it need to live? who can modify it?

Chapter 4 of *The Rust Programming Language* explains ownership, references, borrowing, and slices. Read it in full during this lesson. Don't try to memorize every compiler message. The goal is to build a simple mental model: every value has an owner; moving hands over that responsibility; borrowing lets you use a value without handing over the responsibility; and the borrowing rules prevent a read from seeing data while someone is changing it.

The real project already uses this idea, even though you haven't written all its pieces yet. `Servicio` owns its `String` fields because the revisor must keep a name and a URL beyond the function that read them. In contrast, the functions that print a report receive references to the services and their statuses: they only need to consult them, not take them over. Later, in lessons 3, 4, and 5, these same decisions will appear in structs, collections, errors, and lifetimes.

## The concepts

### Ownership, scope, and deterministic release

Ownership boils down to three rules.

1. Every value in Rust has an owner.
2. There can only be one owner of a value at a time.
3. When the owner goes out of scope, Rust drops the value.

A scope is the part of the program where a name exists. Braces delimit scopes, just as in lesson 1. The difference now is that leaving a scope doesn't just make a variable inaccessible: it also determines when the associated value is destroyed. For types that hold resources, Rust calls `drop` automatically. `String`, for example, frees the block of memory where it stores its characters.

**Fig. 2.1** | The scope of a variable.

```rust
// fig02_01.rs
fn main() {
    {
        let s = String::from("hola");     // s es el dueño
        println!("{s}");
    }                                     // aquí termina el ámbito: se libera. Sin free(), sin GC
}
```

```bash
$ rustc --edition 2024 fig02_01.rs && ./fig02_01
hola
```

`String::from("hola")` creates a `String` that owns dynamic memory. While `s` is in the inner scope, it can be used to print the text. When it reaches the closing brace, `s` ceases to exist and Rust frees its memory. You didn't write `free`, you didn't calculate sizes, and you didn't wait for a collector to decide to pass by. The compiler inserts the necessary work because it knows the reach of `s`.

The word "ownership" doesn't describe the physical location of a piece of data; it describes responsibility. The value may be on the stack, on the heap, or contain references to other values. What matters is that Rust can identify an owner responsible for cleaning up the resource. Many simple types, like `u64`, `bool`, or `char`, fit entirely on the stack and don't require freeing anything special. A `String`, a `Vec<T>`, or a `HashMap<K, V>` manage dynamic memory and do need an orderly end.

This release is called deterministic because it happens at a point you can reason about when reading the program: at the end of the scope, unless the value was moved earlier. It is important to tell it apart from manual management. You don't choose when to call `drop` for each value, nor should you in normal conditions. Rust knows the type and generates the right release. If a type contains other values, its destructor also frees whatever corresponds inside it.

This guarantee doesn't mean Rust forbids every imaginable memory leak. For example, it is possible to keep data alive with cycles of reference-counted pointers or to deliberately use mechanisms that avoid release. The central guarantee is another one: safe code can't later use a value that Rust has already freed, nor free the same memory twice. For a program like the revisor, that eliminates a whole class of errors without adding a garbage collector at run time.

In Go, a local variable also stops being useful when it leaves its block, but the memory that has become inaccessible is reclaimed later, when the collector decides. In Rust, the end of the scope is a direct part of the resource model. That difference doesn't automatically make one language better than the other. Go simplifies many applications; Rust lets you know more precisely when memory, files, sockets, or locks are released.

The revisor owns the text it needs to keep. A service can't depend on a temporary variable of the YAML parser still being alive: that is why its fields are `String`, not references to temporary text.

<!-- verificar:extracto:src/modelo.rs -->
```rust
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}
```

`nombre` and `url` are owned by each `Servicio`. When the vector of services is destroyed, its elements will be destroyed; when each element is destroyed, its `String`s will be destroyed; and each `String` will free its memory. There is no manual list of resources to clean up. The structure of the values also describes the structure of responsibility.

### Move, copy, and clone

The second rule says a value has only one owner at a time. That is why an assignment doesn't always mean copy. With types that own resources, Rust usually moves the value: the new variable becomes the owner and the previous name can no longer be used.

This is surprising if you come from Go. In Go, assigning a `string` to another variable copies its immutable header and both variables can be read. Assigning a struct copies its fields; if it contains a slice or a map, both copies can keep pointing to shared data. In Rust, the compiler demands that this relationship be explicit, because a shallow copy of an owning type can leave two values trying to free the same resource.

**Fig. 2.2** | The two ways out: copy or borrow.

```rust
// fig02_02.rs
fn main() {
    let a = String::from("hola");
    let b = a.clone();          // copia explícita: pagas la copia y lo dices
    let c = &a;                 // PRESTAR en vez de mover ← esto es lo normal
    println!("{a} {b} {c}");
}
```

```bash
$ rustc --edition 2024 fig02_02.rs && ./fig02_02
hola hola hola
```

`a.clone()` creates a second `String`, with its own memory and its own characters. That is why `a` and `b` can live independently. The copy has a cost proportional to the size of the text: copying "hola" is small; copying a large HTTP response or a list of thousands of services may not be. Rust makes that cost visible with the `clone()` method.

You shouldn't interpret this as a ban on cloning. A copy is correct when the program really needs two independent values: keeping one name for the report and another to send to a task, keeping an original configuration before transforming it, or separating data that will live different lengths of time. The problem appears when `clone()` is used mechanically to silence an error without answering who needs to own the data.

The third name, `c`, is a reference. `&a` doesn't copy the characters or hand over ownership. It creates a read-only borrow. That is why `a`, `b`, and `c` can be printed: `a` is still the owner; `b` owns another copy; `c` only points temporarily to `a`.

Types that implement the `Copy` trait behave differently. Integers, booleans, characters, and tuples made up exclusively of `Copy` values are copied implicitly because duplicating them is cheap and they don't require freeing memory. If you assign `let b = a` when `a` is a `u64`, you can use both names. It is not that ownership disappears: each variable receives its own copy of the value.

`String` doesn't implement `Copy` because implicitly copying its three internal fields — pointer, length, and capacity — would produce two managers for the same block of the heap. Rust could also copy the characters, but then every assignment would potentially hide costly work. That is why it distinguishes move from clone.

The revisor clones only when it needs to build an output that must own its own text. The JSON report can't keep references to local services inside a function that has already finished. That is why it converts the borrowed name of the service into an independent `String`.

<!-- verificar:extracto:src/reporte.rs -->
```rust
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
```

Here `servicios` and `estados` are lent to the report function. `s.nombre.clone()` and `motivo.clone()` are necessary decisions: `EstadoJson` must survive as an element of `lineas` and later be converted to JSON. The code doesn't clone out of fear of the compiler; it clones because the result has its own owner.

### Immutable references: borrowing to read

A reference is a way to allow access to a value without transferring its ownership. It is written `&T`: "a reference to a `T`". If you have a `String` and a function only needs to know its length, passing `&String` avoids creating a copy and keeps the function from consuming the text.

**Fig. 2.3** | Borrowing to read.

```rust
// fig02_03.rs
fn largo(s: &String) -> usize { s.len() }      // presta, no toma posesión

fn main() {
    let s = String::from("hola");
    let n = largo(&s);
    println!("{s} mide {n}");                   // sigue siendo mía ✓
}
```

```bash
$ rustc --edition 2024 fig02_03.rs && ./fig02_03
hola mide 4
```

The function `largo` receives a reference. Inside it, `s.len()` consults the length, but it can't keep the `String` or modify it. When the call ends, the borrow ends and the original owner is still the variable `s` in `main`. That explains why the last line can print both the text and its length.

The example keeps `&String` because it directly shows the contrast between an owning `String` and a reference to it. In a general API it is better to receive `&str` when you only need to read text. `&str` is a view of a UTF-8 sequence; it accepts both a literal and a reference to a `String`. Lesson 4 will go deeper into that distinction, but from now on you can use a practical rule: store text you own as `String`; receive read-only text as `&str`.

A reference is not a copy of the value. Its lifetime is limited by the value it points to. Rust doesn't allow returning a reference to a local variable that will disappear when a function exits, nor keeping a reference once its owner has moved. This part of the analysis is known as borrow checking.

The reference makes a function's contract visible. A signature that receives `String` communicates "I need to take this text". One that receives `&str` communicates "I only need to read it". In Go, passing a `string` is cheap because its representation is copied; passing a large struct by value or by pointer requires reading the documentation and knowing its implementation. Rust makes that difference part of the signature.

The report functions receive borrowed slices. They don't consume the vector of services or the vector of statuses because `main` still needs them to decide the exit code. The signature expresses this intent without additional comments.

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

`&[Servicio]` means "borrowed slice of services". A slice borrows a contiguous part of a collection and knows how many elements it contains. The function can walk through it, consult names, and compute the width of the table, but it can't empty the vector, add services, or keep them. The same decision applies to `&[Estado]`.

### Mutable references: borrowing to modify

A mutable reference is written `&mut T`. It is useful when a function must modify a value whose ownership remains with the caller. To create one, you need two things: the owner must be declared with `mut`, and the borrow must be written as `&mut`.

**Fig. 2.4** | Borrowing exclusively to modify.

```rust
// fig02_04.rs
fn agregar_puerto(etiqueta: &mut String) {
    etiqueta.push_str(":443");
}

fn main() {
    let mut servicio = String::from("catalogo");
    agregar_puerto(&mut servicio);
    println!("{servicio}");
}
```

```bash
$ rustc --edition 2024 fig02_04.rs && ./fig02_04
catalogo:443
```

`servicio` is mutable because its content will change. `agregar_puerto` doesn't receive the `String` by value: it receives an exclusive borrow and adds characters to the same text. When the call ends, the borrow ends and `main` uses the owner again to print it.

Exclusivity is the important condition. During a `&mut` borrow, nobody else can read or modify the same data through another reference. It is not an arbitrary limitation: if one part of the program changes a string while another assumes it is reading it stably, the result can depend on the order of execution. In concurrent programs, that situation is a data race.

The two borrowing rules are:

1. You can have any number of immutable references to a value.
2. You can have exactly one mutable reference to a value, or immutable references, but not both at the same time.

The short way to remember them is: many reads or one write. A read doesn't change the data and can be shared. A write needs exclusivity because it could change any part of the value.

Rust also analyzes the last real use of a reference. You don't necessarily have to wait until the closing brace of the block to ask for a mutable borrow. If an immutable reference will not be used again, Rust can consider its borrow finished. This is known as non-lexical lifetimes. You shouldn't depend on it to write confusing code, but it explains why separating a read and a modification into clear steps usually compiles.

In the revisor, the table is built with a mutable variable. Ownership of `salida` stays in `tabla`, but `push_str` needs a temporary mutable borrow to add each line. When leaving the function, `tabla` returns the complete `String` and ownership passes to the caller.

<!-- verificar:extracto:src/reporte.rs -->
```rust
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
```

`push_str` modifies `salida`; that is why the variable is declared with `mut`. In contrast, `s` and `e` are read references to the data of each row. The compiler allows both things because the function modifies the new report, not the services or statuses it is consulting.

### Slices, `&str`, and references that return references

Ownership doesn't force you to copy when you want to get a part of a value. A function can receive a reference and return another reference to a part of the same information, as long as Rust can check that the output won't outlive the input. That pattern appears with array slices, vector slices, and `&str`.

**Fig. 2.5** | Returning a borrowed view of text.

```rust
// fig02_05.rs
fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let texto = String::from("revisor listo");
    let palabra = primera_palabra(&texto);
    println!("primera: {palabra}");
}
```

```bash
$ rustc --edition 2024 fig02_05.rs && ./fig02_05
primera: revisor
```

`primera_palabra` doesn't build a new `String`. It returns a view into a part of `s`. That is why it is efficient: it doesn't copy the characters. It also has a healthy limitation: `palabra` can't outlive `texto`, because it points inside it. If `texto` is modified in a way that changes its storage, an old reference could stop being valid; Rust prevents you from using both things in an incompatible way.

The signature `fn primera_palabra(s: &str) -> &str` uses a lifetime inference rule. The compiler understands that the output reference is tied to the input reference. In lesson 5 you will see the cases where you must write an annotation like `'a`; for now keep the important idea: the function doesn't own the word it returns, so it can't promise that it will exist longer than the borrowed text.

The revisor uses explicit lifetimes when it assembles rows that only borrow data from two slices. It doesn't duplicate each service and each status before sorting them; it keeps references that are valid as long as the original vectors stay alive.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

The annotation `'a` says that the references inside `Fila` can't live longer than the slices lent to `ordenadas`. `filas` owns the vector of references, but not the services or the statuses. This distinction is the basis of many efficient Rust programs: owning the collection doesn't imply owning all the data it points to.

## The error you will see

### E0382: using a value after moving it

The following program doesn't compile on purpose. The assignment `let b = a` moves the `String` from `a` to `b`. The last line tries to borrow `a` in order to print it, but `a` is no longer the owner and can't be lent.

**Fig. 2.6** | Move, don't copy.

```rust
// fig02_06.rs
fn main() {
    let a = String::from("hola");
    let b = a;                  // NO copia: MUEVE. Ahora b es el dueño
    println!("{a}");            // ← error: valor movido
}
```

```bash
$ rustc --edition 2024 fig02_06.rs
error[E0382]: borrow of moved value: `a`
 --> fig02_06.rs:5:16
  |
3 |     let a = String::from("hola");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es el dueño
  |             - value moved here
5 |     println!("{a}");            // ← error: valor movido
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let b = a.clone();                  // NO copia: MUEVE. Ahora b es el dueño
  |              ++++++++

warning: unused variable: `b`
 --> fig02_06.rs:4:9
  |
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es el dueño
  |         ^ help: if this is intentional, prefix it with an underscore: `_b`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
```

`E0382` means you tried to use a value after transferring its ownership. The message points to three places: where `a` was born, where the move happened, and where you tried to use it again. That sequence is more useful than memorizing the error code: follow the arrows and ask who the owner is after each line.

There are three possible fixes, and they are not interchangeable. If you no longer need `a`, print `b`. If you need two independent values, use `a.clone()` and accept the cost of the copy. If the second part only needs to read the value, change the design to borrow `&a` instead of moving it. The third option is usually the best when you write helper functions for the revisor.

The warning about `b` appears because the program never gets to use it. It is not the main error; it is a consequence of the example using `a` on purpose to provoke `E0382`. The programs in the course that must compile are checked with `-D warnings`, so an unused variable also becomes a problem you must fix.

### E0502: asking for a write while reads exist

The following error represents the second borrowing rule. `r1` and `r2` are live immutable references because they are used in the final `println!`. While those reads exist, Rust can't create `r3`, a mutable reference to the same `String`.

**Fig. 2.7** | Reads and a write at the same time.

```rust
// fig02_07.rs
fn main() {
    let mut s = String::from("hola");
    let r1 = &s;                  // lectura, ok
    let r2 = &s;                  // otra lectura, ok
    let r3 = &mut s;              // ← error: ya hay lecturas vivas
    println!("{r1} {r2} {r3}");
}
```

```bash
$ rustc --edition 2024 fig02_07.rs
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> fig02_07.rs:6:14
  |
4 |     let r1 = &s;                  // lectura, ok
  |              -- immutable borrow occurs here
5 |     let r2 = &s;                  // otra lectura, ok
6 |     let r3 = &mut s;              // ← error: ya hay lecturas vivas
  |              ^^^^^^ mutable borrow occurs here
7 |     println!("{r1} {r2} {r3}");
  |                -- immutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

`E0502` means you asked for a mutable borrow while an immutable borrow is active. Rust doesn't assume that "surely nothing will happen". A mutable borrow could replace, shorten, empty, or reallocate the content of `s`, so the read references would no longer have a coherent view.

The fix is not to copy `s` by reflex. First decide whether you really need to read and modify at the same time. If not, finish the reads before asking for the write: print or compute with `r1` and `r2`, stop using them, and then create the mutable reference. If you need to keep read information while you modify, store a small copy of the necessary data, like a length or a flag, not necessarily a full copy of the structure.

This error is a local version of a guarantee that will be decisive in lesson 7. In Go, two goroutines that read and write shared data without coordination can have a data race that only shows up when running. Rust sets these rules before talking about threads; when you get to `Arc`, `Mutex`, and channels, the same model will keep protecting shared accesses.

## What goes wrong

### Using `clone()` to silence every error

The compiler suggests `clone()` in several messages because it is a mechanical and safe solution: it creates an independent value. However, the suggestion doesn't know your program's design or the size of your data. If you clone a small `String` once, it probably doesn't matter. If you clone a list of services in every function or duplicate large HTTP bodies in a loop, you add time and memory you don't need.

Before writing `.clone()`, ask whether the function only needs to read. If the answer is yes, receive `&T` or `&str`. If it must modify something but the owner must keep it, receive `&mut T`. Clone when you need two real owners, like the report's JSON and the original data that stays alive for another operation.

### Receiving `String` by value when you will only read

A signature that receives `String` makes the caller hand over ownership. That can be right for a function that normalizes, consumes, or stores the text. It is unnecessary for a function that only prints, measures, or searches for a word. The cost is not always a copy: sometimes the caller can move the value. The problem is that the signature reduces the usage options of the caller.

For query functions, prefer `&str` if you work with text and `&T` if you work with another type. The API will be more flexible: it will accept literals, `String`s, and portions of text without forcing the creation of new owners. Lesson 4 will show why `&str` is normally a better public boundary than `&String`.

### Thinking that `mut` means "I can borrow mutably whenever I want"

`let mut s` lets you modify `s`, but it doesn't remove the borrowing rules. Mutability belongs to the owner; exclusivity belongs to each borrow. You can declare a mutable string and still get `E0502` if active read references exist. You can also have an immutable variable that holds a mutable reference created in another context; the two concepts are distinct.

Use `mut` only when the name must change or when you will ask for a mutable borrow. If a variable never changes, removing `mut` leaves a clearer intent and avoids compiler warnings.

### Fighting the borrow instead of reducing its reach

A reference lives until its last use, not necessarily until the visual end of the block. If the compiler doesn't accept a mutable borrow, check where the previous reference is last used. Many fixes consist of rearranging a few lines: finish reading, store the result you need, and then modify. Separating the read and write phases improves both readability and compatibility with the borrow checker.

Don't hide the problem behind a long reference, an `unsafe`, or a global structure. The revisor is still small; if the ownership model becomes hard to explain, it is usually a sign that a function has too many responsibilities or that a piece of data is being shared more than necessary.

### Confusing `String` with `&str`

`String` owns text and can grow; `&str` is a view of text that belongs to something else. Converting a `&str` to `String` with `to_string()` or `String::from()` is correct when you are going to store it. Doing it only because a function could receive a reference is an avoidable copy. On the other side, returning `&str` when the text was built inside the function can't work: the local text disappears when the function ends.

The useful question is always the same: who should own these characters after this operation? If the answer is "the structure that stores them", use `String`. If the answer is "nobody new; I only need to observe them now", use `&str`.

## Exercises

### Exercise 1 — Follow the owner

Read the following situations and write, before compiling, which name can be used at the end: an assignment of a `u64`; an assignment of a `String`; and an assignment of a `String` followed by `clone()`. Then create three small files and check your answers with `rustc --edition 2024`.

Explain in one sentence why the integer is copied, why the `String` is moved, and why `clone()` produces two owners. Don't use `Copy` as a magic word: relate it to cost and to the need to free memory.

### Exercise 2 — One function that takes and one that borrows

Write two functions on a service name. The first must receive a `String` by value and return its length. The second must receive `&str` and return the same length. In `main`, show that after calling the first function you can no longer print the `String`, and that after calling the second one you can print it.

First leave active the line that provokes `E0382` and read the full diagnostic. Then comment out that line so the program compiles. Don't fix the first function with `clone()`: the goal is to observe the difference between taking ownership and borrowing.

### Exercise 3 — Update a service without changing owner

Write `fn agregar_puerto(etiqueta: &mut String)` to append `:443` to a label. Declare a mutable `String` in `main`, lend it to the function, and check that `main` can print the result at the end.

Then provoke `E0502`: create a read-only reference to the same text, use it after asking for a mutable reference, and observe the line the compiler points to. Reorder the program so that the read finishes before modifying.

### Exercise 4 — The borrowed first word

Implement `fn primera_palabra(s: &str) -> &str`. It must return the first word of a phrase, or an empty string if it only receives spaces. Test it with a `String` called `texto`, print the result, and then also print `texto`.

Do the Rustlings exercises `move_semantics` and `primitive_types`. In particular, don't advance by trial and error with `clone()`: in each solution, identify whether Rust is asking you to move, copy, or borrow.

## Solutions

### Solution 1

A `u64` implements `Copy`, so after `let b = a` there are two independent values and both names are usable. A `String` doesn't implement `Copy`; the same assignment moves ownership to `b`, so `a` is no longer usable. If you write `let b = a.clone()`, `a` and `b` own two distinct blocks of text and both can be used.

The test is not about remembering which types implement `Copy`, but about making a prediction and checking it. When you have doubts about your own type, the compiler will tell you whether it implements `Copy`. In the revisor's structs that contain `String`, initially assume the value is moved.

### Solution 2

The function that receives `String` consumes the argument. Its signature should look like `fn largo_tomando(s: String) -> usize`; after the call, the original `String` is no longer available. The function that receives `&str` should look like `fn largo_prestando(s: &str) -> usize`; call it with `&nombre` and then print `nombre`.

The difference is not in the number they return, but in the input contract. For a function that only computes the length, the second signature is the right one. The first exists so that you can explicitly observe the move and for the real cases where a function does need to keep the value.

### Solution 3

The solution is the one in figure 2.4: the owner is declared as `let mut servicio`, the function is called with `&mut servicio`, and it is printed after the call ends. The function doesn't return the `String` because it never received it as owned.

To fix the borrow conflict, completely use the read reference before creating the mutable reference. The important point is not to put both references in artificial blocks, but to make visible that the read phase ended before the write phase.

### Solution 4

The solution is the one in figure 2.5. `split_whitespace()` ignores leading spaces and splits the words; `next()` produces an `Option<&str>`; `unwrap_or("")` returns an empty string if there was no word. The result is a reference taken from the input, not a new `String`.

The right test prints the word first and then the original `String`. That shows that `primera_palabra` didn't take ownership of `texto`. If you tried to return a reference to a `String` created inside the function, Rust would reject it because that `String` would be destroyed when the call ends.

## How do I know I got it

- [ ] `rustc --edition 2024 fig02_01.rs && ./fig02_01` prints `hola`.
- [ ] `rustc --edition 2024 fig02_02.rs && ./fig02_02` prints `hola` three times and I can explain which value was cloned and which was borrowed.
- [ ] `rustc --edition 2024 fig02_06.rs` fails with `E0382`, and I can explain on which line ownership was moved.
- [ ] `rustc --edition 2024 fig02_07.rs` fails with `E0502`, and I know how to fix it by finishing the reads first.
- [ ] `rustc --edition 2024 fig02_04.rs && ./fig02_04` prints `catalogo:443`.
- [ ] `rustc --edition 2024 fig02_05.rs && ./fig02_05` prints `primera: revisor`.
- [ ] I finished `move_semantics` and `primitive_types` in Rustlings without using `clone()` as an automatic solution.
- [ ] I can explain in one sentence why Rust frees memory when leaving scope without requiring a garbage collector.

## Further reading

- [The Rust Programming Language, chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html), accessed October 2, 2026.
- [The Rust Programming Language, references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html), accessed October 2, 2026.
- [Official documentation of `String`](https://doc.rust-lang.org/std/string/struct.String.html), accessed October 2, 2026.
- [Rustlings](https://rustlings.rust-lang.org/), exercises `move_semantics` and `primitive_types`, accessed October 2, 2026.
