# Semana 1 — Fundamentos

**The Book, capítulos 2 y 3.** Rustlings: `variables`, `functions`, `if`, `primitive_types`.

## Lo que se parece a Go y lo que no

**Fig. 1.1** | Variables, mutabilidad y constantes.

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

**Los tipos numéricos son explícitos:** `i8 i16 i32 i64 i128 isize` y sus `u`. En Go tienes `int` que
depende de la máquina; en Rust eliges. `i32` es el default razonable, `usize` para índices y tamaños.

⚠️ **Y una que sorprende:** en Rust **no hay conversión implícita**, ni entre números.

**Fig. 1.2** | No hay conversión implícita, ni entre números.

```rust
// fig01_02.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a;             // ← no compila
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_02.rs
error[E0308]: mismatched types
 --> fig01_02.rs:4:18
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

**Fig. 1.3** | La conversión se pide con `as`.

```rust
// fig01_03.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a as i64;      // así
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_03.rs && ./fig01_03
5
```

Molesta al principio y evita una clase entera de errores de desbordamiento.

## Todo es una expresión

**Fig. 1.4** | Todo es una expresión.

```rust
// fig01_04.rs
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
$ rustc --edition 2024 fig01_04.rs && ./fig01_04
grande 49 14
```

🔑 **El punto y coma decide si algo es una expresión o una sentencia.** `x * 2` devuelve; `x * 2;`
descarta. Es la fuente del error más desconcertante de la primera semana:

**Fig. 1.5** | El punto y coma de más.

```rust
// fig01_05.rs
fn doble(x: i32) -> i32 {
    x * 2;                     // ← el punto y coma de más
}

fn main() {
    println!("{}", doble(4));
}
```

```bash
$ rustc --edition 2024 fig01_05.rs
error[E0308]: mismatched types
 --> fig01_05.rs:2:21
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

`()` es el tipo «nada» — el equivalente de haber devuelto vacío por poner un `;` de más.

## Bucles

**Fig. 1.6** | Los bucles.

```rust
// fig01_06.rs
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
$ rustc --edition 2024 fig01_06.rs && ./fig01_06
0 1 2 3 4 5 6 7 8 9 
0 1 2 3 4 5 6 7 8 9 10 
catalogo
pagos
reportes
x = 0, r = 42
```

⚠️ **`for s in servicios` (sin `&`) CONSUME la lista** — es ownership, la semana que viene. Si después
la necesitas, no compila. Por eso casi siempre verás `&`.

## El ejercicio de la semana

1. Capítulo 2 completo (el juego de adivinar) y capítulo 3.
2. Rustlings: `variables`, `functions`, `if`, `primitive_types`.
3. Escribe `fn clasificar(ms: u64) -> &'static str` que devuelva `"rápido"`, `"lento"` o `"timeout"` —
   **con un `if` que sea expresión**, sin `return`.
4. Provoca el error del punto y coma de más y lee la sugerencia.
5. Escribe una función que sume un slice de enteros con `for` sobre una referencia.

## Cómo sé que lo logré

- [ ] Sé por qué `x * 2;` con punto y coma rompe una función que devuelve `i32`
- [ ] Sé qué es `()` cuando aparece en un error
- [ ] Rustlings: las cuatro secciones completas
- [ ] Sé por qué `as i64` es obligatorio
