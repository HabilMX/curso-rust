# Lesson 7 — Concurrency and async

**Time:** 2 × 45 min.

**What you build:** the concurrent `revisor`: one that checks everything at once

**What you learn:** OS threads, `Arc` and `Mutex`, channels, `async/await` with `tokio`, the honest comparison with Go's goroutines

## By the end you will be able to

- Launch OS threads with `thread::spawn`, hand them the right ownership, and collect their result with `join`.
- Explain why data shared between threads needs `Arc`, when it also needs a `Mutex`, and what the `MutexGuard` protects.
- Use a standard library channel to deliver results without sharing a mutable collection.
- Read the `E0277` error when a value doesn't satisfy `Send` and recognize that the compiler is protecting a boundary between threads.
- Explain the difference between concurrency and parallelism, and between an OS thread, an async task, and a Go goroutine.
- Follow in the `revisor` the path of an async query, from `#[tokio::main]` to `join_all`, the semaphore, and the status that ends up tied to each service.

## The why before the how

So far, the `revisor` can receive a list of services, query one, and classify the response. If it queries ten services in series and each takes a second to respond, the report takes about ten seconds. It doesn't matter that the computer has several cores or that the program is fast: during almost all that time the CPU isn't computing anything; it is waiting for a network response.

Waiting for a network response is different from calculating a big sum. In an intensive calculation, more cores can allow truly parallel work. In an HTTP request, on the other hand, most of the time is outside the process: the operating system waits for packets, the remote server decides what to do, and the network carries the response. Meanwhile, the program could start other requests. That is concurrency: organizing several tasks that advance in interleaved periods. It can become parallelism if several tasks execute code at the same time on different cores, but they aren't synonyms.

The practical goal of this lesson is to change the total time. If five services take about one second each and you query them one by one, the report takes about five seconds. If you start all five requests and wait for their responses at once, the time approaches that of the slowest service, not the sum of all. That doesn't make the remote services respond faster; it avoids wasting the time the program spent waiting for the first one before starting the second.

The cost is that several parts of the program can be alive at the same time. Questions appear that sequential code didn't have: who owns the data? When does a task end? What order do the results have? Can two tasks modify the same value? What happens if one fails? How do you keep from opening thousands of simultaneous connections? Rust doesn't answer these questions by hiding shared memory. It makes ownership, borrows, and traits like `Send` and `Sync` keep mattering when there are several threads or tasks.

This connects directly with lesson 2. Ownership looked like a local rule: a value has an owner and borrows must respect its lifetime. In concurrency, those rules become a guarantee between tasks. A thread can't keep a reference to a variable in `main` that may already be gone. Nor can it receive a type that Rust knows isn't safe to move to another thread. What felt like compiler friction at first becomes, here, a barrier against dangling references and data races in safe code.

Rust offers two main tools for the problem. The standard library brings OS threads, mutexes, atomic reference counters, and channels. They are appropriate for CPU work, small programs, or integration with blocking APIs. For many network operations that spend time waiting, the `revisor` uses `async/await` and Tokio. Async doesn't mean "faster by definition": it means a moderate number of threads can advance many operations that wait on I/O without reserving a blocked thread for each one.

Go makes another decision. A goroutine is started with `go f()` and the runtime comes built into the language and the distribution. Rust makes you distinguish a thread from an async task and, for async, choose a runtime. That demands more vocabulary and more decisions, but it lets the data type and the ownership boundary be explicit. Neither approach removes the need to design limits, handle errors, and measure. The useful comparison is not which language "wins", but what cost each one pays and what guarantee it offers in return.

Read chapters 16 and 17 of The Rust Book first. Then do the Rustlings exercises on threads and channels before adapting the `revisor`. The order matters: async becomes much less mysterious when you already understand what it means for a closure to move to another thread, what `Send` means, and why sharing mutability requires visible synchronization.

## The concepts

### OS threads, `move`, and `join`

`std::thread::spawn` takes a closure and starts an operating system thread to run it. The value it returns is a `JoinHandle<T>`: a concrete promise that the thread can finish with a value of type `T`. Calling `join()` waits for that thread to finish and returns `Result<T, Box<dyn Any + Send>>`; the `Err` represents that the thread did `panic!`.

The `move` keyword is important. A closure without `move` may try to capture a reference to a variable from the outer context. A thread can keep running after that context has ended, so Rust doesn't allow handing the thread a reference that may stop being valid. `move` makes the closure capture by value. In the figure, each `Servicio` stops belonging to the vector and starts belonging to the closure of its own thread.

**Fig. 7.1** | One thread per service.

```rust
// fig07_01.rs
use std::thread;

struct Servicio {
    nombre: String,
}

type Estado = String;     // en el curso es el enum de la lección 3; aquí basta un texto

fn revisar(s: &Servicio) -> Estado {
    format!("{}: OK", s.nombre)
}

fn main() {
    let servicios = vec![
        Servicio { nombre: "catalogo".to_string() },
        Servicio { nombre: "pagos".to_string() },
        Servicio { nombre: "reportes".to_string() },
    ];

    let handles: Vec<_> = servicios.into_iter().map(|s| {
        thread::spawn(move || revisar(&s))      // `move` entrega la propiedad al hilo
    }).collect();

    for h in handles {
        let estado = h.join().unwrap();          // espera y recoge el resultado
        println!("{estado}");
    }
}
```

```bash
$ rustc --edition 2024 fig07_01.rs && ./fig07_01
catalogo: OK
pagos: OK
reportes: OK
```

The order of the `println!` calls is deterministic even though the threads may finish in another order. The `JoinHandle`s are stored in the same order that `servicios.into_iter()` produces, and the second `for` calls `join()` in that order. If the `pagos` thread finishes first, its value is ready, but the program first waits for and prints the `catalogo` one. This difference matters: concurrency doesn't force the output to be non-deterministic. You can design a boundary where the observable order stays stable.

`join` is enough when each task has one result and the number of tasks is small and known. You don't need a channel just to recover a value; the handle already delivers it. In Go you would normally combine a goroutine with a `WaitGroup` to wait and a channel or a protected collection to recover results. Rust makes the result part of the handle, although that doesn't remove the usefulness of channels for progressive communication.

An OS thread isn't free. It has operating system resources, a stack, and a scheduling cost higher than an async task. There is no universal figure for memory per thread: it depends on the operating system, the architecture, and the configuration. The design rule is more useful than a fixed number: don't open an OS thread per connection if the main work consists of waiting on the network. For a few CPU tasks or a blocking API, a thread may be exactly right. For many HTTP services, the `revisor` uses async.

The `revisor` doesn't create a thread per service. Its network work is expressed as async futures; the runtime decides which threads execute those futures. Still, the same idea of ownership appears: a task must own what it keeps during its execution, or receive borrows that remain valid until it finishes. That's why it's important to understand `move` first, even though the final code uses Tokio.

### `Arc`, `Mutex`, and the data that lives inside the lock

An `Rc<T>` lets several owners within a single thread share a value. Its reference counter isn't atomic, so it can't be shared between threads. `Arc<T>` stands for *atomic reference counted*: it does the same general job, but it updates the reference counter safely across threads. Cloning an `Arc` doesn't clone `T`; it only creates another owner of the same allocation.

Having several owners isn't the same as having permission to modify. If `T` is mutable and several tasks can access it, coordination is needed. `Mutex<T>` holds the data and lets only one task at a time receive mutable access through `lock()`. The result of `lock()` is a `MutexGuard<T>`. While that guard exists, the lock stays taken; when it goes out of scope, its `Drop` releases the lock. You don't need to write a separate call to `unlock`, and that reduces the risk of forgetting to release the resource on a return path.

**Fig. 7.2** | Data shared between threads.

```rust
// fig07_02.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let estado = "OK".to_string();

    let estados = Arc::new(Mutex::new(HashMap::new()));
    let copia = Arc::clone(&estados);
    let h = thread::spawn(move || {
        copia.lock().unwrap().insert("x".to_string(), estado);
    });

    h.join().unwrap();
    println!("{:?}", estados.lock().unwrap());
}
```

```bash
$ rustc --edition 2024 fig07_02.rs && ./fig07_02
{"x": "OK"}
```

The form `Arc<Mutex<HashMap<_, _>>>` reads from the inside out. The `HashMap` is the data. The `Mutex` is the only door for mutating it. The `Arc` lets different threads own that door. To insert, the thread takes the lock and receives the guard; to print, `main` takes the lock again. A mutable reference to the map never appears without the guard existing.

In Go it is common to declare a `sync.Mutex` next to the map and follow the convention of calling `Lock` before touching it. That convention can be encapsulated well, but the language doesn't force the map to be physically inside the mutex. In Rust, by wrapping the data in `Mutex<T>`, the normal API doesn't let you obtain `&mut T` without a `MutexGuard`. This doesn't make all concurrency errors impossible: you can still cause a deadlock by taking locks in inconsistent orders, or hold a guard for too long. What it does make impossible in safe code is a data race caused by simultaneously lending the same mutable value without synchronization.

Don't use `lock().unwrap()` as a formula you don't think about. `lock()` can return an error if another thread did `panic!` while holding the lock; that is called mutex poisoning. In a teaching example, `unwrap()` makes that case visible with a `panic!`. In a real service you must decide whether that state invalidates the program, whether you can recover the data with `into_inner`, or whether it is better to redesign so that shared state isn't needed.

The `revisor` avoids a `Mutex<HashMap<...>>` because it doesn't have to fill a shared map as each response arrives. Each future produces its own `Estado`, and `join_all` gathers them. The resource it does share is a limit on turns: several futures need to ask for permission to start a request, not to modify a common collection. That's why the project uses `Arc<Semaphore>` and not `Arc<Mutex<Vec<Estado>>>`.

### Channels: delivering values instead of sharing a collection

A channel splits communication into two ends: a sender, `Sender<T>`, and a receiver, `Receiver<T>`. The sender delivers values with `send`; the receiver takes them with `recv` or by iterating over it. Instead of several threads writing to a shared structure, each producer delivers a value that becomes the receiver's property. This architecture reduces the shared zone and clarifies who assembles the final result.

The channel created with `std::sync::mpsc::channel()` is multiple-producer, single-consumer. It is "multiple" because you can clone the sender before moving a copy into each thread. The receiver isn't cloned: a single place decides what to do with each message. When all the senders disappear, iterating over the receiver ends. That closing of the channel is part of the protocol, not an incidental detail.

**Fig. 7.3** | A channel delivers statuses to the thread that prints the report.

```rust
// fig07_03.rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (emisor, receptor) = mpsc::channel();

    let hilo = thread::spawn(move || {
        for estado in ["catalogo: OK", "pagos: OK"] {
            emisor.send(estado).unwrap();
        }
    });

    hilo.join().unwrap();

    for estado in receptor {
        println!("{estado}");
    }
}
```

```bash
$ rustc --edition 2024 fig07_03.rs && ./fig07_03
catalogo: OK
pagos: OK
```

The program waits for the thread before walking through the receiver in order to keep the output deterministic. In a program that really processes results as they arrive, you would normally start receiving while the producers are still working. The order would then be the order of arrival, not necessarily that of the input services. That can be a good decision for an interface that reports progress, but not for a report that must align each status with the original service.

Channels don't automatically replace mutexes. If several tasks need to read and update the same account, a `Mutex` may be the natural model. If one task produces values and another decides how to store or display them, a channel usually represents the responsibility better. The common mistake is to choose by fashion: "mutexes are bad" or "channels are complicated". The useful question is who should own each piece of data at each moment.

In the `revisor`, `join_all` serves a function similar to receiving all the results, but with an additional contract: it preserves the order of the input futures, so that `estados[i]` is always the result of `servicios[i]`. That is what the report needs: to know which service each status belongs to. Watch out for what it does *not* promise: the report isn't printed in the order of the YAML list, because `reporte::tabla` and `reporte::json` sort the rows by service name before writing them (you will see this in lesson 8). What `join_all` guarantees is the correspondence between each service and its status, and with a channel, where results arrive in the order they finish, you would have to rebuild it by hand. If the product needed to print "pagos finished" as soon as the response arrives, a channel or a stream (a sequence of values that arrive over time) would be a reasonable option. Don't add it just because it exists; the current choice is intentional and keeps the report reproducible.

### `Send`, `Sync`, and the error the compiler stops

`Send` and `Sync` are marker traits. They don't usually require methods of their own; they describe safety properties that Rust can derive from a type's fields. A `Send` type can be transferred by value to another thread. A `Sync` type can be shared by reference between threads: if `T` is `Sync`, then `&T` is `Send`. Many common structures implement them automatically when their components are also safe, but `Rc<T>` is neither `Send` nor `Sync` because its counter can't be updated from several threads.

This rule isn't a list to memorize. It is a question that Rust answers by composition. If you make a struct that contains `Rc<RefCell<_>>`, it inherits the restrictions of those pieces. If you switch to `Arc<Mutex<_>>`, you change the representation and also the available guarantees. The compiler follows the value all the way to the closure that is sent to `thread::spawn` and demands that the boundary be safe.

In Go, a data race can compile and require `go test -race` to be detected during an execution that happens to hit the problematic interleaving. The detector is valuable and you should use it, but it depends on the test running the conflicting path. Rust prevents data races in safe code before running the program. That doesn't prove the logic is correct, nor does it automatically detect deadlocks, starvation (a task that never gets a turn because others hog the resource), or badly designed protocols. There is also `unsafe`, where the programmer takes on additional responsibilities. The precise statement is: Rust prevents data races through its type and borrow rules in safe code; it doesn't promise that every concurrent program is correct.

Inside the `revisor`, `Arc<Semaphore>` is valid because Tokio's semaphore is designed to be shared between tasks. Each future receives its own `Arc`, asks for a permit, and keeps that permit during the request. The permit's type and lifecycle express that the turn can't be returned before the query finishes. There is no shared `usize` counter that each future manually increments and decrements.

### `async`, futures, and the Tokio runtime

A function marked `async fn` doesn't immediately execute its whole body when you call it. It produces a future: a value that represents pending work. That future advances when an executor polls it. If it reaches an operation that isn't ready yet, such as waiting for a network response, it hands control back to the executor. Later, when the operation can continue, the executor polls it again.

`.await` is the point where an async function waits for the result of another future. By itself it doesn't create a new task or a new thread. This distinction corrects two frequent misunderstandings. First: writing `let futuro = revisar(...);` doesn't necessarily start the request; it only builds the future. Second: calling `.await` one after another in the same block can make the operations serial. To start several operations concurrently, you build several futures and drive them together with a combinator like `join_all`, or you turn them into tasks with `tokio::spawn` when you really need independence.

Rust defines the syntax and traits of async, but it doesn't include a complete async executor in the standard library. Tokio is the runtime chosen by this project. It provides an executor, timers, async synchronization, and I/O adapters. Some async libraries are runtime-independent, but Tokio's resources, such as its timers and several synchronization types, require running inside a Tokio context. Read each crate's documentation before assuming that any future works the same with any runtime.

<!-- verificar:extracto:src/main.rs -->
```rust
#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    ejecutar(&args).await
}

async fn ejecutar(args: &Args) -> ExitCode {
```

The `#[tokio::main]` attribute builds the runtime and runs the async `main` function. The program's `main` still returns an `ExitCode`, as you saw in lesson 4; what changes is that it can now wait on async operations before deciding the exit code. The binary keeps the responsibility of parsing arguments, printing, and exiting; the library keeps the logic of querying services.

Don't block a runtime thread with `std::thread::sleep`, heavy file reads, or long computation inside an async function. A blocked thread can't poll the other futures assigned to it. For blocking work there is `tokio::task::spawn_blocking`; for network I/O, use async APIs like `reqwest`. The `revisor` uses `reqwest::Client` and awaits its `send().await`, so while a response is pending the runtime can advance queries to other services.

Before seeing how the `revisor` uses Tokio, it helps to see Tokio on its own. The following program is the smallest one that shows what matters: three "checks" that, instead of querying the network, simply wait, each for a different time. It doesn't fit in a bare `rustc` invocation, because it depends on the `tokio` and `futures` crates; that's why it lives in `programas/revisor/examples/` and runs with Cargo, which downloads and compiles those dependencies.

**Cargo example with `tokio`** | Three waits driven at once with `join_all`: they finish in one order and are delivered in another.

<!-- verificar:ejemplo:ejemplo_tokio -->
```rust
// ejemplo_tokio.rs
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::time::sleep;

async fn revisar(nombre: &str, espera_ms: u64) -> String {
    sleep(Duration::from_millis(espera_ms)).await;
    println!("terminó {nombre}");
    format!("{nombre}: respondió tras {espera_ms} ms")
}

#[tokio::main]
async fn main() {
    let servicios = [("catalogo", 600), ("pagos", 200), ("usuarios", 400)];
    let inicio = Instant::now();

    let futuros = servicios.iter().map(|(nombre, ms)| revisar(nombre, *ms));
    let resultados = join_all(futuros).await;

    println!("--- en el orden de la lista ---");
    for resultado in &resultados {
        println!("{resultado}");
    }
    // Esperarlos uno tras otro habría tardado 1200 ms; a la vez tardan lo del más lento.
    let a_la_vez = inicio.elapsed() < Duration::from_millis(1100);
    println!("tardó menos que la suma de las esperas: {a_la_vez}");
}
```

```bash
$ cargo run --example ejemplo_tokio
terminó pagos
terminó usuarios
terminó catalogo
--- en el orden de la lista ---
catalogo: respondió tras 600 ms
pagos: respondió tras 200 ms
usuarios: respondió tras 400 ms
tardó menos que la suma de las esperas: true
```

Run it from `programas/revisor/` (on the first run Cargo takes a while to compile the dependencies).

`#[tokio::main]` turns `main` into an async function: it builds the runtime and hands it the future that `main` describes. `tokio::time::sleep` is Tokio's wait, and it resembles `std::thread::sleep` in what it does but not in how: with `.await`, the task yields control to the runtime while it waits, and the runtime takes the opportunity to advance the others. With `std::thread::sleep` the entire thread would fall asleep and nothing else would advance on it.

Read the output in two parts. The `terminó ...` messages come out in the order in which each wait completes: `pagos` (200 ms), `usuarios` (400 ms), and `catalogo` (600 ms), even though the list declares them in another order. Then `join_all` delivers the results in the order of the list —`catalogo`, `pagos`, `usuarios`—, because it returns a `Vec` where each position corresponds to its input future. The last line checks that it was concurrent: waiting for the three checks one after another would have taken 1200 ms, and at once it takes as long as the slowest one, about 600 ms.

If you replace `sleep` with a network call using `.send().await`, you have the shape of the `revisor`: many futures that wait, a single `join_all` that drives them, and a `Vec` of results aligned with the list of services.

### `join_all`, semaphores, and the `revisor`'s parallelism limit

Launching every possible request at the same time isn't always an improvement. A file with thousands of services could open too many connections, saturate the local network, exhaust file descriptors, or overload the very server you are trying to check. Concurrency needs a limit. The `revisor`'s `--paralelo` argument expresses how many queries can be active at once.

A semaphore holds permits. To start a query, a future acquires one; if none are left, it waits. When the permit goes out of scope, it is released automatically and another future can continue. It is the same idea of RAII (the resource is released when the value that represents it goes out of scope) that you saw with `MutexGuard`: the resource is released when the guard is destroyed, even if the function exits through a normal path. Here the resource isn't an exclusive lock but a limited capacity.

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

`servicios.iter()` keeps borrows of the list; it doesn't consume the services. Each async closure owns its clone of `Arc<Semaphore>`, but it borrows `cliente` and `s` during the call to `revisar`. This works because `join_all(futuros).await` finishes before `revisar_todos` can return, so those borrows are still alive. If you used `tokio::spawn`, the task could outlive the function that created it and would normally need data with a `'static` lifetime; there you would have to move or clone more data.

`join_all` returns a `Vec<Estado>` in the order of the input futures, not in the order in which the requests finish. It is a useful decision for the report: status zero corresponds to service zero. The semaphore limits when each future can enter `revisar`, but it doesn't change that final relationship. That way, the program has real concurrency without turning the report into a source of random order.

The `revisar` function doesn't return `Result<Estado>`. That decision deserves attention. An HTTP 500, a timeout, or a refused connection isn't an internal error that keeps the program from continuing: it is precisely the information the `revisor` must report about a service. That's why those cases are turned into `Estado::Falla` variants. The `acquire` error is treated differently because the semaphore is never closed in this design; if it happened, it would be a violation of an internal assumption.

The project's timeout is configured on the `reqwest` request, with each `Servicio`'s `timeout_ms`. Don't confuse that limit with the semaphore. The timeout limits how long an individual query can wait; the semaphore limits how many queries can be waiting or communicating at the same time. You need both: without a timeout, a turn can stay occupied for too long; without a semaphore, many requests with timeouts can all start together and overload resources.

### An honest comparison with Go

Go makes it very easy to start a concurrent unit: `go revisar(s)`. The runtime schedules goroutines over OS threads, grows their stacks, and manages network work. Rust explicitly separates the decision: `thread::spawn` creates an OS thread; a runtime like Tokio executes async futures; `tokio::spawn` creates a Tokio task. This means Go usually has less ceremony at the start and Rust forces you to know which of the three abstractions you are using.

Rust has no magic performance advantage from writing `async`. An async task doesn't speed up an individual HTTP request; it improves thread utilization while several requests wait. For a CPU load, async can be worse if it blocks the executor. In that case use threads, a worker pool, or `spawn_blocking`. Go doesn't automatically make a CPU-intensive calculation faster either: several goroutines can compete for the same cores. Measuring the kind of work matters more than applying a buzzword.

Rust's central guarantee appears before running: a mutable piece of data can't be lent in an incompatible way, and values sent to another thread must satisfy the right traits. Go favors a small syntax and runtime tools like the race detector. Go can properly encapsulate mutexes and channels; Rust can have deadlocks and logic errors. The difference isn't "Go allows errors and Rust doesn't". It is where each language places the burden of checking and which errors it can reject before running.

For the `revisor`, the decision is justified by the problem. There are many HTTP waits, each result must stay tied to its service, and a configurable cap on requests is needed. Tokio, `join_all`, and `Semaphore` express those three needs. A design with one thread per service would work for a small list, but it would scale worse and brings no advantage for the use case. A design with a mutex and a shared map could also work, but it would make more complex a relationship that `join_all` already preserves.

## The error you will see

The most instructive error in this lesson is `E0277`. It doesn't mean "Rust doesn't want to use threads"; it means the type you are trying to move doesn't satisfy the contract that `thread::spawn` requires. `Rc<i32>` is useful for sharing ownership within one thread, but its reference counter isn't atomic. Moving it into a closure that may run on another thread would be unsafe.

**Fig. 7.4** | `Rc<T>` can't be sent to a thread.

```rust
// fig07_04.rs
use std::rc::Rc;
use std::thread;

fn main() {
    let conteo = Rc::new(0);
    thread::spawn(move || println!("{conteo}"));
}
```

```bash
$ rustc --edition 2024 fig07_04.rs
error[E0277]: `Rc<i32>` cannot be sent between threads safely
 --> fig07_04.rs:7:19
  |
7 |     thread::spawn(move || println!("{conteo}"));
  |     ------------- -------^^^^^^^^^^^^^^^^^^^^^
  |     |             |
  |     |             `Rc<i32>` cannot be sent between threads safely
  |     |             within this `{closure@fig07_04.rs:7:19: 7:26}`
  |     required by a bound introduced by this call
  |
  = help: within `{closure@fig07_04.rs:7:19: 7:26}`, the trait `Send` is not implemented for `Rc<i32>`
note: required because it's used within this closure
 --> fig07_04.rs:7:19
  |
7 |     thread::spawn(move || println!("{conteo}"));
  |                   ^^^^^^^
note: required by a bound in `spawn`
 --> /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library/std/src/thread/functions.rs:125:0

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

The message contains the answer. The closure captures `Rc<i32>`, `thread::spawn` demands that what is captured be `Send`, and `Rc<i32>` doesn't implement `Send`. Don't fix this error by adding traits manually with `unsafe impl Send`; you would be promising the compiler a safety that `Rc` doesn't offer. If several threads only need to read a piece of data, use `Arc<T>`. If they must also modify it, consider `Arc<Mutex<T>>` or redesign the flow to send values through channels.

Also recognize the design error that the compiler sometimes doesn't reject: holding a `MutexGuard` across an `.await` operation. If you launch the task with `tokio::spawn` and the guard comes from a `std::sync::Mutex`, `rustc` does reject it (`future cannot be sent between threads safely`, because that guard isn't `Send`); but if the future is awaited on the same thread, as `join_all` does in the `revisor`, the program compiles. For that case `clippy` ships by default the `await_holding_lock` warning. A `std::sync::Mutex` guard blocks a thread; an async mutex guard keeps the lock while the task may yield the executor. In both cases, waiting on the network while you hold the lock usually blocks work unnecessarily and can produce deadlocks. Extract or update the data under the lock, release the guard, and only then wait.

## What goes wrong

- Creating a thread for every service with no limit. It works with three examples and fails as a strategy when the list grows. OS threads consume operating system resources and a large number of them makes it harder to plan, measure, and debug. For network I/O, use async with a parallelism limit; for CPU, use a number of threads proportional to the work and the available cores.

- Using `Arc<Mutex<_>>` as an automatic answer to any ownership error. This combination is correct when there is genuinely shared mutable state, but it can hide a design where several tasks do too much. If each task can return a value and a single part gathers them, a channel, `join_all`, or a later reduction is usually clearer and reduces contention.

- Holding a lock during a network request or an `.await`. The guard exists to protect a small critical section. If you keep it while you wait, you turn concurrent tasks into a queue and increase the chance of deadlock. Limit the scope with braces or a temporary variable so that the guard is destroyed before the wait.

- Calling blocking functions inside Tokio code. `std::thread::sleep` blocks the thread, not just a task. A large read, a blocking query, or heavy computation has the same problem. Use async APIs for I/O, `tokio::time` for timers, or `spawn_blocking` for work that truly must block.

- Confusing `async` with parallelism. A future can advance concurrently with others and still run on a single thread. If you need to speed up CPU computation, you must decide how to distribute it among cores. If you are waiting on the network, async improves the utilization of the existing threads. Before optimizing, measure what the program is waiting for.

- Using `tokio::spawn` just to "make it concurrent". In the `revisor`, `join_all` can drive futures that borrow `cliente` and `servicios`, preserves the order, and avoids requiring `'static` ownership. `tokio::spawn` is useful for independent tasks that must live beyond the current block, but it implies another contract of lifetimes and types.

- Treating the output that arrives first as if it were the status of the service in that position. In a progress interface it can be useful to report by arrival. In a report, if another service's result sneaks into a row, each line lies about its service. The `revisor` avoids that risk with `join_all`, which preserves the correspondence between `servicios[i]` and `estados[i]`, and its integration tests check that property; the order in which the rows are printed is another decision, which the report makes by sorting them by name.

## Exercises

### Exercise 1 — Three checks with `join`

Write a program with three text-based `Servicio` values. Use `thread::spawn(move || ...)` to produce one status for each, keep the handles in a vector, and use `join` to print the results in the same input order. Don't use `sleep` to "give the threads time".

### Exercise 2 — Protected counter

Create an `Arc<Mutex<u32>>` with an initial value of zero. Launch four threads; each must increment the counter once. Wait for all the handles and print `total: 4`. Then deliberately change `Arc` to `Rc` and confirm that `E0277` appears.

### Exercise 3 — Results through a channel

Create a channel and three producer threads. Each producer must send the name of a service and a status. The main thread must receive exactly three messages and sort them by name before printing them. Explain in a comment why you must not depend on the order of arrival.

### Exercise 4 — Explain the `revisor`'s limit

Read `programas/revisor/src/revisar.rs`. Run the integration tests and locate the test `el_tope_de_paralelo_se_respeta`. In your logbook, answer: what resource does the `Semaphore` control, when is the permit acquired, when is it released, and why does `join_all` preserve the order of `servicios`.

## Solutions

### Solution 1

The correct solution moves each `Servicio` into the thread and collects the handles afterward. The decisive point isn't the `map`, but that the vector of handles keeps the obligation to wait for each job before `main` ends. If you print after each `join`, the observable order is the order of the handle vector.

One way to check that you didn't depend on `sleep` is to run the program several times: it must always print the three lines. The scheduler may change which thread finishes first, but it can't prevent `join` from waiting.

### Solution 2

Each thread must receive its own `Arc::clone(&contador)`. Inside the thread, take the guard, increment, and let the guard go out of scope. After waiting for the four handles, take one last guard to print the value. Don't try to keep a mutable borrow of the counter outside the mutex; that borrow can't coexist with the other threads.

When you change `Arc` to `Rc`, the expected result isn't a numeric output but `E0277`. The fix isn't to "silence" the compiler: `Rc` is for shared references within a single thread; `Arc` is the right type for the counter to have owners across several threads.

### Solution 3

Each producer receives a clone of the sender and sends a struct or tuple with the name and the status. The original sender must stop existing before you walk through the whole receiver, or you can call `recv` exactly three times because you know the number of producers. Store the received messages in a vector and sort it by name before printing.

The important part is that the receiver is the sole owner of the final vector. The producers have no mutable access to it. That's why you don't need a mutex to gather the results; ownership travels through the channel along with each message.

### Solution 4

The semaphore controls the number of HTTP queries that can be active at once, not the total number of services. Each future acquires a permit right before calling `revisar(cliente, s).await`. The permit lives in `_turno`; when that call finishes, `_turno` goes out of scope and returns the capacity to the semaphore.

`join_all` receives futures built by walking through `servicios` and produces the vector of statuses in that same sequence. That's why the results can finish at different times without getting out of alignment with the service that originated them. The test measures times to confirm that the limit changes the behavior and isn't just a decorative flag.

## How do I know I got it

Run this lesson's figures from their directories and compare the exact output:

```bash
cd programas/07-concurrencia-async
rustc --edition 2024 fig07_01.rs && ./fig07_01
rustc --edition 2024 fig07_02.rs && ./fig07_02
rustc --edition 2024 fig07_03.rs && ./fig07_03
rustc --edition 2024 fig07_04.rs
```

The first three compilations must finish without warnings and produce the documented outputs. The last one must fail with `E0277`; that failure is the correct result of the exercise.

Check that the blocks and their outputs are still verifiable from the course root:

```bash
herramientas/verificar-programas.sh es
herramientas/verificar-extractos.sh
herramientas/verificar-ejemplos.sh
```

The three commands must finish successfully. The first confirms that each figure compiles, runs, and matches its documented output, except for the figure designed to fail. The second confirms that the `revisor` excerpts are still exact copies of the real project. The third compiles and runs this lesson's `tokio` example and compares what it prints with what is documented.

Finally, verify the project's real concurrent behavior:

```bash
cd programas/revisor
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

`cargo test` must report successful results, including the `el_tope_de_paralelo_se_respeta` test. `cargo clippy --all-targets -- -D warnings` must finish without warnings, and `cargo fmt --check` must not propose changes. If you can explain why the semaphore limits queries, why `join_all` preserves order, and why `Rc` causes `E0277`, you have finished the lesson.

## Further reading

- [The Rust Programming Language, chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — accessed October 2, 2026.

- [The Rust Programming Language, chapter 17: Fundamentals of Asynchronous Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html) — accessed October 2, 2026.

- [Documentation of `std::thread`](https://doc.rust-lang.org/std/thread/) — accessed October 2, 2026.

- [Documentation of `tokio::sync::Semaphore`](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html) — accessed October 2, 2026.
