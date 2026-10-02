# Semana 1 — Fundamentos

**The Book, capítulos 2 y 3.** Rustlings: `variables`, `functions`, `if`, `primitive_types`.

## Lo que se parece a Go y lo que no

    let x: i32 = 5;             // tipo explícito (casi nunca hace falta: lo infiere)
    let mut y = 10;             // mutable
    const MAX: u32 = 100_000;   // constante, siempre con tipo

**Los tipos numéricos son explícitos:** `i8 i16 i32 i64 i128 isize` y sus `u`. En Go tienes `int` que
depende de la máquina; en Rust eliges. `i32` es el default razonable, `usize` para índices y tamaños.

⚠️ **Y una que sorprende:** en Rust **no hay conversión implícita**, ni entre números.

    let a: i32 = 5;
    let b: i64 = a;             // ← no compila
    let b: i64 = a as i64;      // así

Molesta al principio y evita una clase entera de errores de desbordamiento.

## Todo es una expresión

    let n = if x > 5 { "grande" } else { "chico" };      // el if DEVUELVE valor

    let cuadrado = {
        let t = x * x;
        t                          // 🔑 sin punto y coma = es el valor del bloque
    };

    fn doble(x: i32) -> i32 {
        x * 2                      // sin `return` y sin `;`
    }

🔑 **El punto y coma decide si algo es una expresión o una sentencia.** `x * 2` devuelve; `x * 2;`
descarta. Es la fuente del error más desconcertante de la primera semana:

    error[E0308]: mismatched types
      expected `i32`, found `()`
      help: remove this semicolon to return this value

`()` es el tipo «nada» — el equivalente de haber devuelto vacío por poner un `;` de más.

## Bucles

    loop { break; }                          // infinito, con break
    while x > 0 { x -= 1; }
    for i in 0..10 { }                       // rango: 0 a 9
    for i in 0..=10 { }                      // inclusivo: 0 a 10
    for s in &servicios { }                  // sobre una referencia, para no consumir la lista

    let r = loop { break 42; };              // 🔑 loop devuelve valor con break

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
