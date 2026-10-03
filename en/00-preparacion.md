# Lesson 0 — Installing Rust on Your Linux Mint

**Time:** 90 min.

**What you build:** your environment and your first program with `cargo`.

**What you learn:** why not `apt install rustc`, `rustup`, `rustc` versus `cargo`, `cargo new/build/run/check`, the book and Rustlings offline, reading a compiler error.

## By the end you will be able to

- Install stable Rust with `rustup` and explain why relying on `apt install rustc` is a bad idea.
- Tell whether your terminal is using the tools managed by `rustup`.
- Tell apart the work of `rustc` from the work of `cargo`, and know which one to use in each case.
- Create a project with `cargo new`, build it, run it, and check it with `cargo check`.
- Open The Rust Book offline and install Rustlings to practice locally.
- Read a complete `rustc` diagnostic, find the line responsible, and try the suggested fix.

## The why before the how

This lesson doesn't look like a Rust lesson, because it doesn't teach ownership, types, or `match` yet. Even so, it decides an important part of how you will learn the language: from day one you will work with the same toolchain, the same conventions, and the same kind of errors that a real project uses. Installing something that "more or less compiles" is enough for an isolated exercise; installing the right environment is necessary to follow a course, read current documentation, and build the `revisor` without the tooling becoming an extra problem.

The course was written and checked with stable Rust 1.98.1 and the 2024 edition. This is a concrete reference so that the programs, messages, and examples mean the same thing for everyone; with a later stable release the programs should behave the same way, although the wording of some compiler message may change. Rust publishes a stable release roughly every six weeks. Linux Mint, on the other hand, inherits a good part of its packages from Ubuntu, and an LTS distribution puts system stability first: it freezes major versions and applies security patches. That is a reasonable decision for system software; it is not a good way to follow closely a language whose ecosystem, documentation, and tools change often.

That is why `apt install rustc` seems to work at first and can cause confusion later. It installs a compiler called `rustc`, but not necessarily the compiler that The Rust Book, recent examples, or the projects you find use. The problem doesn't always show up as "your version is old". Sometimes it appears as an unknown feature, an edition that doesn't exist, a different compiler suggestion, or a dependency that no longer accepts that version. It is the worst kind of setup failure: it happens later and looks like a bug in your program.

Rust solves this with `rustup`. It is not just an installer: it is the official Rust toolchain manager. A *toolchain* bundles a version of `rustc`, `cargo`, the standard library, documentation, and related components that must work together. `rustup` installs the stable channel and places its executables in a predictable location inside your user account. When it's time to update, you change that whole set with `rustup update stable`, without mixing in system packages or downloading files by hand.

The comparison with Go helps place the decision. In Go you installed the official distribution because the distribution's package could also be left frozen; Rust makes that problem more visible because its release pace is shorter and because `cargo` integrates building, dependencies, tests, formatting, and static analysis. In both courses you build the same `revisor`: a tool that reads a list of services, queries them, and reports their status. In Go, `go` covers many tasks. In Rust, `cargo` plays that role around `rustc`. The difference doesn't change the discipline: you work inside a project, you build with a repeatable tool, and you read the diagnostic before changing code at random.

The goal is not to memorize a list of commands. It is to build a simple mental model. `rustc` turns a Rust file into executable code and reports language errors. `cargo` understands a whole project: it knows its name, edition, dependencies, tests, build profiles, and file layout; then it calls `rustc` with the right arguments. At the start you will use `rustc` directly to see, without noise, what the compiler does. In day-to-day work you will use `cargo`, because a real program rarely consists of a single file with no dependencies.

The other important decision in this lesson is how to respond to an error. Rust doesn't try to guess what you meant, and it doesn't let doubtful code through only to fail later. The compiler stops the build, points to both the origin and the problematic use, and in many cases proposes a concrete change. This doesn't mean every message is easy from day one; it means it is worth reading them in full. In Rust, the compiler is part of the learning process. The next lessons will make you trigger ownership, borrowing, and type errors on purpose, because you will understand more by diagnosing a real failure than by memorizing an isolated rule.

## The concepts

### `rustup` installs and manages the toolchain

The recommended installation on Linux Mint is the one published by the Rust project itself:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Before running a command that downloads and runs a script, it's worth understanding it. `curl` downloads the installer; `--proto '=https'` restricts the download to HTTPS; `--tlsv1.2` demands a modern TLS connection; `-sSf` makes the command fail if the server returns an error; and `| sh` passes the downloaded content to the shell interpreter. It is the official method, but being official doesn't remove your responsibility to review what you run.

If you prefer to inspect it first, download the file, read it, and run it only after reviewing it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
less rustup.sh
sh rustup.sh
```

The installer offers a default installation. Choose option `1`, which installs the stable channel for your platform. When it finishes, open a new terminal. If you want to use Rust in the current terminal without closing it, load the environment file that the installer set up:

```bash
source "$HOME/.cargo/env"
```

The important location is `~/.cargo/bin`. That is where `rustup` puts the programs you invoke: `rustup`, `rustc`, `cargo`, `clippy-driver`, and others. The terminal only finds commands that are in its `PATH` variable; that is why a correct installation can exist on disk and you can still get `command not found`. The installer tries to update your startup configuration, but each shell and each terminal may read different files.

Check the installation with these commands:

```bash
rustup show active-toolchain
rustc --version
cargo --version
type -a rustc
type -a cargo
```

The first one shows the active toolchain. The next two should show Rust 1.98.1 or a later stable version if you have already updated your environment. `type -a` is especially useful if you ever installed Rust with `apt`: it lists all the matches the shell finds and lets you discover that you are running an old binary ahead of the one in `~/.cargo/bin`.

Don't confuse a Linux Mint update with a Rust update. To update the stable toolchain, use:

```bash
rustup update stable
```

You don't need to run it before every command; it is a good idea to do it periodically and before starting a session after several weeks. If you are already up to date, `rustup` will say so. The point is not to chase version numbers for sport: it is to keep the compiler, documentation, examples, and installed components aligned.

In the `revisor`, this decision shows from its root. The project declares the 2024 edition and Cargo builds all its modules with the active toolchain. There is no "Rust version" hidden in each file: the package configuration fixes the language the project speaks, and the lockfile fixes the concrete versions of its dependencies so that a repeated build resolves the same set.

### The `PATH` and the compiler you actually run

When you type `rustc`, the shell doesn't search the whole disk. It walks the folders listed in `PATH`, in order, and runs the first match. That rule explains two common diagnoses. If there is no `rustc` at all in those folders, `bash` usually shows:

```bash
rustc: command not found
```

If there is one, but it comes from an earlier `apt` installation, the command may work and show an unexpected version. This one is more deceptive: it doesn't look like an installation problem, but the course would be built with a tool other than the expected one.

First confirm which shell you use and how it was started:

```bash
echo "$SHELL"
echo "$0"
printf '%s\n' "$PATH"
```

In a usual Bash installation, `~/.bashrc` is loaded for interactive shells and `~/.profile` for a login session. In Zsh, the equivalent file for interactive sessions is usually `~/.zshrc`. The file that `rustup` creates, `~/.cargo/env`, adds `~/.cargo/bin` to the `PATH`. Running `source "$HOME/.cargo/env"` in the current terminal is a direct check: if `rustc --version` works after that, the problem was the shell environment, not the compiler.

Don't add duplicate paths over and over without checking what happened. First use `type -a rustc`. If a path under `~/.cargo/bin` appears, `rustup` is available in the `PATH`. If a path like `/usr/bin/rustc` appears before it, a system installation is taking priority. In that case, first identify which packages are installed and which binary the shell is using; don't try to fix it by copying executables or editing symbolic links by hand.

The `revisor` doesn't depend on a fixed compiler path. This is an advantage of working with `cargo`: the tool invokes the `rustc` of the active toolchain and keeps the build results inside `target/`. That is why a Cargo project can be built the same way on another computer with a correct installation, without the code containing personal paths or commands specific to your machine.

### `rustc`: the compiler and the first program

`rustc` is the Rust compiler. It receives source code, checks that it respects the rules of the language and, if everything is fine, produces an executable. For a small file it is useful to invoke it directly because it lets you see the exact relationship between source, compilation, and resulting program. Every figure in this course is compiled this way so that the documented output matches the code you are reading.

**Fig. 0.1** | The first program.

```rust
// fig00_01.rs
fn main() {
    println!("hola, ya tengo Rust");
}
```

```bash
$ rustc --edition 2024 fig00_01.rs && ./fig00_01
hola, ya tengo Rust
```

`fn main()` declares the function where an executable program starts. Rust looks for precisely a function called `main` to begin. Inside it, `println!` prints text and adds a newline. The `!` sign is not decorative: `println!` is a macro. Macros generate or transform code during compilation; for now it is enough to recognize the convention that names ending in `!` are not ordinary functions. The Rust Book returns to that topic in chapter 20.

The `--edition 2024` flag selects the current edition of the language. An edition doesn't mean your code is automatically converted to a different language each year; it is a way for Rust to improve rules and syntax without silently breaking existing projects. The `revisor` project declares that same edition in its manifest. The course examples state it explicitly so that a figure's command doesn't depend on the default edition of a particular installation.

This direct use of `rustc` is deliberately small. If you had two files, a library, tests, external dependencies, optimization options, and different target platforms, writing the right compiler invocation by hand would become fragile. That is where `cargo` comes in. The relationship is not a competition between two programs: Cargo organizes, Rustc compiles. When you use `cargo build`, Cargo ends up calling `rustc` for you with the paths, edition, and flags the project needs.

The same point exists in the `revisor`: the binary has a `main` function, but it is not built by isolating that file with `rustc src/main.rs`. It imports modules from the local library and external crates; it needs the manifest and the structure that Cargo knows. The figures in this lesson teach the compilation mechanism. Later lessons apply that mechanism to a composite program.

### `cargo`: the project before the file

Create your first project in a working folder of your own:

```bash
mkdir -p "$HOME/w/curso-rust"
cd "$HOME/w/curso-rust"
cargo new hola
cd hola
```

`cargo new hola` creates a folder called `hola`, a `Cargo.toml` manifest, and the `src/main.rs` file. If Git is installed and Cargo can initialize a repository, it also sets up Git and a `.gitignore`; if Git is not available, the project is still valid. The minimal result has this structure:

```text
hola/
├── Cargo.toml
├── .gitignore
└── src/
    └── main.rs
```

`Cargo.toml` is the package manifest. It writes down the project's identity, its edition, its dependencies, and some build decisions. The file is not an administrative detail: it is what lets another person run the same `cargo build` without rebuilding the list of compiler arguments by hand. When Cargo resolves dependencies, it also creates `Cargo.lock`; that file records the concrete resolution so that builds are repeatable.

The `revisor` is already a complete Cargo project. This is its real manifest:

<!-- verificar:extracto:Cargo.toml -->
```toml
[package]
name = "revisor"
version = "0.1.0"
edition = "2024"
description = "El revisor del curso de Rust: consulta una lista de servicios a la vez y reporta cuáles responden."

[dependencies]
anyhow = "1.0.104"
clap = { version = "4.6.7", features = ["derive"] }
futures = "0.3.34"
reqwest = { version = "0.13.5", features = ["json"] }
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
yaml_serde = "0.10.7"
tokio = { version = "1.53.1", features = ["full"] }

[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila

[dev-dependencies]
serde_json = "1.0.151"
```

You don't need to understand the dependencies or the release profile yet. What matters is recognizing the shape: `[package]` describes the package; `[dependencies]` lists what it needs to build; `[profile.release]` changes how the final binary will be built. In lesson 4 you will meet the error and serialization dependencies, in lesson 6 the order of modules and tests, and in lesson 8 the release profile. Today it is enough to understand why a project needs a manifest and why it is not a good idea to replace Cargo with a long manual `rustc` command.

Run the project you just created with:

```bash
cargo run
```

The first time, Cargo builds the package and then runs the binary. On later runs, it reuses artifacts that haven't changed. Unlike `rustc fig00_01.rs`, you don't have to type the file name or the executable name: Cargo knows the `src/main.rs` convention and knows that the `hola` package produces the `hola` binary.

This convention reduces repetitive decisions. A Rust program can be organized in several ways, but Cargo provides a common structure for the frequent cases. The project you created today has only `src/main.rs`, the binary. When you open the `revisor`, you will also recognize `src/lib.rs`, the package's library. That separation is not invented in lesson 6: Cargo recognizes it by convention, just as it recognizes `src/main.rs`.

### `cargo build`, `run`, `check`, and the work cycle

Cargo's commands are not synonyms. Each one answers a different question you ask yourself while working:

```bash
cargo run
cargo build
cargo build --release
cargo check
cargo test
cargo clippy
cargo fmt
```

`cargo run` answers "does my program compile, and what does it do?". It first builds what is needed and then runs the binary. It is the command you will use when you change an output, try out a branch of the program, or want to observe a behavior. For the `hola` project, it should print the message that is in `src/main.rs`.

`cargo build` answers "can I produce the binary?". It builds the package but doesn't run it. In development mode it leaves the artifacts in `target/debug/`; you don't need to learn that path by heart, but it helps to know that Cargo doesn't fill the project's root folder with executables and intermediate files. That keeps source code and build results separate.

`cargo build --release` produces the release profile, normally with more optimization and with results inside `target/release/`. The difference matters when you deliver the finished `revisor` or measure its performance. Don't compare run times of a development binary and draw conclusions about Rust's performance: the development profile prioritizes fast compiling and comfortable debugging; the release profile prioritizes the resulting program. The `revisor`'s `Cargo.toml` shows that this profile can even adjust size, optimization across modules, and behavior on `panic!`.

`cargo check` answers "does the compiler accept my code?". It checks types, borrows, modules, and much of the compilation work, but it doesn't finish generating an executable binary. In a growing project it can save time. Use it while you write, especially when you only want to know whether a change is valid; use `cargo run` when you also want to run the behavior. Neither command replaces the other: one verifies quickly and the other also verifies the result at run time.

`cargo test` builds and runs tests. You haven't written tests in this lesson yet, but you will start seeing them in lesson 6. `cargo clippy` runs the official linter and points out patterns that compile but tend to be confusing, inefficient, or unidiomatic. `cargo fmt` applies the standard Rust format. It is the same discipline as `gofmt` in Go: you don't spend time arguing about the alignment of each file, you let the tool give a uniform answer.

The `revisor` uses exactly that cycle. Before publishing a change, it is a good idea to run these commands from `programas/revisor/`:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

The first checks behavior; the second turns Clippy warnings into failures so they are not ignored; the third confirms that the code already has the expected format. You shouldn't run them yet to "understand" their output. Keep them as a reference for the routine you will arrive at when you build the program.

### Immutability by default and the first conversation with `rustc`

Rust considers a variable created with `let` immutable, unless you write `mut`. This is not an obstacle put there to make the code longer. It is a visible declaration of intent: if a value must change, the reader and the compiler should be able to see it at the place where the variable is defined.

**Fig. 0.2** | A program that doesn't compile.

```rust
// fig00_02.rs
fn main() {
    let x = 5;
    x = 6;
    println!("{x}");
}
```

```bash
$ rustc --edition 2024 fig00_02.rs
error[E0384]: cannot assign twice to immutable variable `x`
 --> fig00_02.rs:4:5
  |
3 |     let x = 5;
  |         - first assignment to `x`
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
3 |     let mut x = 5;
  |         +++

warning: value assigned to `x` is never read
 --> fig00_02.rs:3:13
  |
3 |     let x = 5;
  |             ^ this value is reassigned later and never used
4 |     x = 6;
  |     ----- `x` is overwritten here before the previous value is read
  |
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0384`.
```

The solution the compiler suggests is correct if you really want to change the value. The word `mut` is written in the declaration, not in the later assignment. That way, whoever reads the block knows from the start that `x` is part of that function's mutable state.

**Fig. 0.3** | The same variable, now mutable.

```rust
// fig00_03.rs
fn main() {
    let mut x = 5;      // mut = mutable
    println!("{x}");
    x = 6;              // ahora sí
    println!("{x}");
}
```

```bash
$ rustc --edition 2024 fig00_03.rs && ./fig00_03
5
6
```

Immutability by default helps reduce accidental changes and prepares the ground for ownership and borrowing. In other languages it is common to modify a variable because the language allows it and only afterward ask who depended on its previous value. Rust asks you to declare that possibility from the start. It doesn't eliminate all errors, but it turns an implicit assumption into a checkable property.

In the `revisor` there is mutability only where the operation really requires it. The function that sorts the rows creates a vector and then sorts it in place; that is why the variable is declared `mut`:

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

`mut` is not written out of habit. `filas.sort_by(...)` modifies the vector, so the declaration communicates it. In contrast, `servicios` and `estados` are references that this function only reads; they are not declared mutable. That difference, small in this function, becomes important when several parts of a program try to use the same data. Lesson 2 explains the rules Rust applies so that those uses are safe.

### The Rust Book and Rustlings as local practice

The Rust Book is the course's official reference text. `rustup` installs a local copy of the documentation, so you can open it offline once the installation is finished:

```bash
rustup doc --book
```

That command opens the first chapter in your default browser. If you have no graphical environment or you prefer to find other documents, `rustup doc --help` shows the available options. The local copy is not a substitute for updates: if you update the toolchain, the local documentation is updated with it. That is another reason to keep the compiler and the documentation together.

Read chapter 1 of The Rust Book in full: installation, "Hello, world!", and "Hello, Cargo!". Don't skip it because you already created a project. What you did here gives you context to read it faster; the chapter puts the concepts in order and explains the conventions that will come back throughout the course. Chapter 2, which includes the guessing game, corresponds to lesson 1 together with the fundamentals of chapter 3.

Rustlings complements the book with small exercises that you solve by editing local files. Its initial installation does need network access so that Cargo can download the program; after that, the exercise directory and its code live on your computer. Install and initialize it like this:

```bash
cargo install rustlings
rustlings init
cd rustlings
rustlings
```

The interactive command watches the exercises, tells you which one fails, and checks them again as you save changes. Don't use Rustlings as a collection of answers to tick off. The order of the course is intentional: first read the chapter of The Rust Book, then solve the linked exercises, and then apply the idea to the `revisor`. At this stage, install Rustlings and get familiar with its directory; in lesson 1 you will work through its `variables`, `functions`, `if`, and `primitive_types` sections.

The book and the exercises serve different functions. The Rust Book explains the model and names its parts; Rustlings forces you to touch the code and receive a concrete error. The `revisor` is the integration problem: not an isolated exercise, but the same program you already built in Go, now with Rust's decisions. The three sources support each other. If an explanation seems abstract, try an exercise; if an exercise feels mechanical, go back to the chapter; if both are already clear, locate the pattern inside the real project.

## The error you will see

The central error of this lesson is `E0384`, shown in full in Fig. 0.2. Don't read it as a wall of text. Read it in order. The first line names the diagnostic code, `E0384`, and summarizes the problem: you cannot assign twice to an immutable variable. That code is useful for asking the compiler for an extended explanation:

```bash
rustc --explain E0384
```

The line that starts with `--> fig00_02.rs:4:5` locates the assignment attempt: file, line, and column. The numbered lines show enough context that you don't have to search blindly. The `^^^^^` mark points to the exact part that causes the error. Before it appears the first assignment, on line 3, because Rust doesn't only report where it detected the problem: it also shows the origin of the condition that makes it invalid.

The `help:` section deserves special attention. In this case it proposes changing `let x = 5;` to `let mut x = 5;`, and the `+++` marks indicate what text to add. Don't apply every suggestion mechanically. First verify the intent: if `x` shouldn't change, the right fix is not to add `mut`, but to remove or rethink the later assignment. The compiler can offer a local fix; you decide whether that fix represents the right design.

The same diagnostic also includes a warning: the first value, `5`, is never read because it is overwritten immediately. The warning doesn't stop compilation by itself, but it gives useful information. If you make `x` mutable without looking at the warning, the program will still have an unnecessary assignment. The version in Fig. 0.3 prints `5` before changing it, so both values make sense and the program produces no warnings.

When you run the same error inside a project with `cargo run` or `cargo check`, Cargo will show the `rustc` diagnostic together with package context and the `src/main.rs` path. The rule doesn't change: start with the first error, read its notes and its help, fix one cause at a time, and compile again. An initial error can cause several later ones; trying to fix them all at once often hides the real cause.

There are other setup errors that are not Rust codes because they happen before the compiler can analyze your program. If `cargo` or `rustc` don't exist for the terminal, the problem is `PATH`; reload `~/.cargo/env` and check `type -a cargo`. If compilation gets as far as linking and a message like `error: linker 'cc' not found` appears, you are missing the C compiler that Rust uses for linking on Linux Mint. Install the distribution's build tools package and run the command again:

```bash
sudo apt install build-essential
```

Don't confuse that case with "Rust didn't install". `rustc --version` may work perfectly; the failure shows up later, when the compiler needs to turn compiled objects into a system executable. Separating the stage that fails avoids random fixes.

## What goes wrong

- Installing Rust with `apt install rustc` and taking for granted that the package name guarantees a current toolchain. The problem is not that the package is useless; it is that it follows the distribution's calendar, not Rust's. For this course use `rustup`, check `rustc --version`, and update the stable channel periodically.

- Mixing an `apt` installation with a `rustup` one without checking which one wins in `PATH`. Having two executables called `rustc` doesn't necessarily produce an immediate error. It may compile for days with the wrong version. Use `type -a rustc` and `type -a cargo` before editing startup files or removing packages.

- Using `rustc` for a whole project out of habit. For a single-file figure it is an excellent teaching tool. For the `revisor`, it would mean manually rebuilding dependencies, paths, edition, modules, and profiles. Use `cargo` from the project root; let it build the `rustc` invocation.

- Using `cargo run` every time you want to know whether the code compiles. It works, but it builds and runs even when you are only fixing types or borrows. During quick editing, `cargo check` gives more direct feedback. When you need to observe the behavior, use `cargo run`.

- Measuring performance with a development binary. `cargo build` and `cargo run` use the development profile by default. When the course gets to comparing size, speed, or delivery of the binary, use `cargo build --release`. Without that distinction, a measurement tells you more about the chosen profile than about the program.

- Ignoring a warning because "it doesn't stop compilation". Warnings usually point to unused values, dead code, or confusing constructs. This course compiles the correct figures with warnings treated as errors so that the output shown doesn't hide problems. Do the same in your routine: understand the warning or remove its cause.

- Reading only the first line of an error. The first line names the category; the following lines say where it happened, which earlier value explains it, which notes apply, and which alternative the compiler considers. Copying only "error E0384" to search for it loses a good part of the answer that is already in front of you.

- Installing Rustlings and solving exercises with copied answers. An exercise finished without understanding the diagnostic doesn't build the mental model you will need for ownership. Make small changes, run the checker, read the error, and explain in your own words why the solution compiles.

## Exercises

### Exercise 1 — Check your toolchain

Install Rust with `rustup` if you don't have it yet. Run `rustup show active-toolchain`, `rustc --version`, `cargo --version`, and `type -a rustc`. Write down which path the terminal is using for `rustc` and confirm that it corresponds to `~/.cargo/bin` when you use the installation managed by `rustup`.

### Exercise 2 — Create and walk through a Cargo project

In a working folder, run `cargo new saludo-rust` and enter the directory it created. Read `Cargo.toml` and `src/main.rs` before modifying them. Change the message to one of your own and run, in this order, `cargo check`, `cargo build`, and `cargo run`. Explain which question each command answered and which file or result you expected from each one.

### Exercise 3 — Trigger and explain `E0384`

Temporarily replace the contents of `src/main.rs` with the program from Fig. 0.2 and run `cargo check`. Don't fix anything until you have identified the file, the line, the first assignment, and the suggestion marked `help:`. Then change the declaration to `let mut x = 5;`, observe the remaining warning, and modify the program so that both values are read, as in Fig. 0.3.

### Exercise 4 — Prepare for reading and local practice

Open The Rust Book with `rustup doc --book` and read chapter 1 completely. Install Rustlings with `cargo install rustlings`, run it in its local directory, and find out how to reopen the exercises without depending on a web page. Write a short note that distinguishes what you get from the book, what you get from Rustlings, and what you will build next in the `revisor`.

## Solutions

### Solution 1

A correct installation shows an active stable toolchain and lets you run both `rustc --version` and `cargo --version`. The exact output may change when you update Rust, but both tools must belong to the same stable installation. `type -a rustc` should list `~/.cargo/bin/rustc` as the chosen path or, at least, let you explain why another path has priority. If it doesn't appear, run `source "$HOME/.cargo/env"` and check again.

### Solution 2

`cargo check` verifies the project without finishing producing an executable; `cargo build` builds the package and leaves development artifacts inside `target/debug/`; `cargo run` builds what is needed and runs the binary. All three commands should accept the `saludo-rust` project. The output of `cargo run` should be exactly the message you left in `src/main.rs`.

### Solution 3

`cargo check` shows `E0384` because `let x = 5;` creates an immutable binding and the next line tries to reassign it. Changing it to `let mut x = 5;` allows the reassignment, but at first it leaves a warning because `5` is overwritten without being used. Printing `x` before and after the assignment removes the warning and produces the two lines of Fig. 0.3. The lesson is not "always add `mut`"; it is to declare mutability only when the modification is part of the design.

### Solution 4

The Rust Book presents an ordered explanation of installation, the first program, and Cargo; its chapter 1 is available locally with `rustup doc --book`. Rustlings provides editable exercises and feedback on local code after you install it and initialize its directory. The `revisor` is where those pieces come together in an application: it doesn't replace the book or the exercises, but gives a continuous problem on which to apply the concepts of the following lessons.

## How do I know I got it

- [ ] `rustc --version` and `cargo --version` work and show a compatible stable toolchain.
- [ ] `type -a rustc` lets me identify which compiler my terminal is running.
- [ ] `rustup update stable` finishes without errors and I can explain what it updates.
- [ ] `cargo new saludo-rust` created a project with `Cargo.toml` and `src/main.rs`.
- [ ] `cargo check`, `cargo build`, and `cargo run` work inside that project, and I know what each one does.
- [ ] My program prints the message I wrote when I run `cargo run`.
- [ ] I can trigger `E0384`, point to its line of origin, read its help, and fix the program without leaving warnings.
- [ ] `rustup doc --book` opens the local Rust Book and Rustlings is initialized in a local folder.

## Further reading

- [The Rust Programming Language, chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html) — installation, first program, and Cargo. Accessed October 2, 2026.

- [Rust: Install](https://www.rust-lang.org/tools/install) — official installation with `rustup`, toolchain updates, and notes on `PATH`. Accessed October 2, 2026.

- [The Cargo Book: Why Cargo Exists](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html) — why Cargo manages packages, dependencies, and the calls to `rustc`. Accessed October 2, 2026.

- [Rustlings](https://rustlings.rust-lang.org/) — installation, initialization, and use of local exercises alongside The Rust Book. Accessed October 2, 2026.
