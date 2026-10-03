# The Rust Course — the Second One, After Go

**By Dorian Chávez, founder of Hábil and integration architect.**

**Who it is for:** someone who has already finished the **[Go course](https://github.com/HabilMX/curso-go)** and wants to understand the other
end of the spectrum. You don't learn Rust *instead of* Go: you learn it **after**, and that way you understand
what decision each one made.

**Do the Go one first.** It isn't a whim: this course takes the base concepts for granted —variables,
functions, structs, lists, errors— and spends its time on what Rust does **differently**. Starting here would be
learning two hard things at once.

**What you need:** a computer with Linux Mint and having finished the Go course. [Lesson 0](00-preparacion.md)
installs Rust from scratch.

Warning: **Rust ships a new version every six weeks.** Before each session: `rustup update stable`. And if a
tutorial doesn't say which version it was written for, be wary: in Rust nine months are **six versions** of
difference, and you do notice it.

## You will write the SAME program

The `revisor` again: it receives a list of services, queries **all of them at once**, and produces a report.

**Writing the same program twice is the method.** Reading "Go vs Rust" comparisons teaches nothing;
fighting with the same problem in both languages does. And you will discover that what took you an
afternoon in Go takes three in Rust — until you understand *why*, and then you understand both things.

## What nobody tells you, and what defines this course

**Rust has a different curve, not a longer one: a different one.** The syntax is easy. What is hard is **the
`borrow checker`**, the compiler component that verifies who owns each piece of data. What
people who teach and learn Rust usually report, as an order of magnitude and without claiming it is a measurement:

- **The borrow checker usually becomes intuitive after several weeks of practice.** Not before. It isn't that you are slow: that
  mental model is built by bumping into it.
- Many people spend **several weeks** on the basics before their first real project.
- **The Rust compiler is the best teacher there is.** Its errors explain the problem, point to the
  line, and **suggest the fix**. In Rust you learn by reading errors, not by avoiding them.

Warning: **and here is the most important difference with the Go course:** in Go the standard library is enough
for almost everything. In Rust it **isn't**: async, HTTP, and serialization live in external *crates* (`tokio`, `reqwest`,
`serde`). That isn't a lack — it is the decision to keep the standard library minimal and stable. But it means
that here you will use dependencies early on.

## The nine lessons

**The method is the one that works, measured: read a chapter of The Book, do its Rustlings exercises,
and only then move on.** Reading straight through retains much less, even if it takes the same time.

| | Lesson | The Book | What you build |
|---|---|---|---|
| 0 | [Setup](00-preparacion.md) | ch 1 | `rustup`, `cargo`, and Rustlings |
| 1 | [Fundamentals](01-fundamentos.md) | ch 2-3 | types, `mut`, control flow |
| 2 | [**Ownership**](02-ownership.md) | **ch 4** | **the lesson that decides everything** |
| 3 | [Structs, Enums, and match](03-structs-enums.md) | ch 5-6 | the `revisor` model, with `Option` |
| 4 | [Collections and Errors](04-colecciones-errores.md) | ch 8-9 | `Vec`, `HashMap`, `Result`, `?` |
| 5 | [Traits, Generics, and Lifetimes](05-traits-genericos.md) | ch 10 | the `Revisor` trait and generics, and the `'a` that scares people |
| 6 | [Modules, Tests, and Cargo](06-modulos-pruebas.md) | ch 7, 11, 14 | the real project |
| 7 | [Concurrency and async](07-concurrencia-async.md) | ch 16-17 | **one that checks everything at once** |
| 8 | [The Finished Program](08-el-programa.md) | ch 12 | `reqwest`, `serde`, `clap`, the binary |

**Ownership goes in lesson 2 and can't be moved.** It is chapter 4 of The Book for a reason: everything
else —structs, collections, errors, concurrency— is explained in terms of who owns what. If
you skip it, the following six weeks are memorizing rules that make no sense.

## The three sources, and how to combine them

1. **[The Book](https://doc.rust-lang.org/book/)** — official, 21 chapters. You have it **offline**:
   `rustup doc --book`. For this course: **chapters 1-12, 14, and 16-17**.
2. **[Rustlings](https://rustlings.rust-lang.org/)** — **95 interactive exercises**, many aimed
   specifically at the borrow checker. `cargo install rustlings`. **Order matters: chapter, then its
   exercises.**
3. **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** — as a quick reference.

**Afterward, once you are writing Rust that works:** *Rust for Rustaceans* (Jon Gjengset) is the book
that engineers who have already passed this stage turn to. Not before: it doesn't work as an introduction.

## How to use it

- **90 minutes per lesson at a minimum.** Rust asks for more seat time than Go; 60 isn't enough.
- **Read the compiler errors in full.** Not just the first line. Rust tells you the problem, the
  cause, and often the exact solution. Ignoring that is throwing away the best tool you have.
- **`cargo clippy` from day one.** It is the official linter and teaches idiomatic Rust while
  you work.
- **[`programas/`](../programas/)** — all the lesson programs, ready to compile and run, and the complete
  `revisor` with its tests. Each one is verified automatically on every change.
- **[`bitacora.md`](bitacora.md)** — here it matters more than in Go: write down **every fight with the borrow
  checker**. When you reread them in lesson 5, you will see that the first ones already look obvious to you. That is
  the moment when it was learned.
