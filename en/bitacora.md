# Logbook

Here it matters **more** than in the Go course, for a concrete reason: **the borrow checker is learned by
fighting it.** What looks hostile to you today will look obvious in three weeks, but you will only
notice that change if you wrote it down.

---

## My fights with the borrow checker

_Every time the compiler blocks you and you don't understand why: paste the error, and then how you solved it._

| # | Date | The error (EXXXX code) | What I was trying to do | How I solved it |
|---|---|---|---|---|
| 1 | | | | |
| 2 | | | | |
| 3 | | | | |
| 4 | | | | |
| 5 | | | | |

Warning: **reread this table when you reach week 5.** If the first three already look trivial to you, the
mental model has been built. That is the only indicator that matters.

---

## Week 0 — Setup

_date:_ · Doubts:

## Week 1 — Fundamentals

_date:_ · Did the extra semicolon bite me?

## Week 2 — Ownership

_date:_

- How many times did I want to write `.clone()` to silence an error?
- What I found hardest to understand:
- The moment it clicked (if it has happened yet):

## Week 3 — Structs, Enums, and match

_date:_

- When I added a variant, how many `match` expressions did the compiler point out to me?
- What did I think of `Option` compared with Go's `nil`?

## Week 4 — Collections and Errors

_date:_

- Lines of error handling in Go: ____ · in Rust: ____
- Which do I prefer, and why?

## Week 5 — Traits, Generics, and Lifetimes

_date:_

- Were lifetimes as bad as I expected?
- Rereading my fights from week 2: do they already look obvious to me?

## Week 6 — Modules, Tests, and Cargo

_date:_

- The three `clippy` suggestions that taught me the most:
  1.
  2.
  3.

## Week 7 — Concurrency and async

_date:_

- Times: threads ____ · async ____
- The error the compiler kept me from making:
- Do I understand why Rust's Mutex wraps the data?

## Week 8 — The Finished Program

_date:_

### The table that closes both courses

| | Go | Rust |
|---|---|---|
| Lines of code | | |
| External dependencies | | |
| Cold build | | |
| Binary size | | |
| Memory at run time | | |
| Time it took me | | |
| Fights with the compiler | | |

**Which one for a tool that is needed by Friday?**

**Which one for production, three years untouched?**

**Which one helped me understand better what I was doing?**

---

## What I want to build now

-
