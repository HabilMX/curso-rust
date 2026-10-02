# Semana 2 — Ownership 🔴

**La semana que decide si aprendes Rust.** The Book, capítulo 4 completo. Sin prisa.

> Si alguna semana vale dos sesiones en vez de una, es ésta. Todo lo demás —structs, colecciones,
> errores, concurrencia— se explica después en términos de quién es dueño de qué.

## El problema que Rust resuelve

Todo lenguaje tiene que decidir quién libera la memoria:

| | Cómo | El precio |
|---|---|---|
| C / C++ | lo hace el programador | fugas, doble liberación, punteros colgantes |
| **Go**, Java, Python | un **recolector de basura** | pausas, más memoria, menos control |
| **Rust** | **el compilador, en tiempo de compilación** | tienes que explicarle tus intenciones |

🔑 **Rust no tiene recolector de basura y tampoco te pide liberar nada.** El compilador sabe cuándo cada
valor deja de usarse porque **rastrea quién es su dueño**. Eso es *ownership*, y es toda la idea.

## Las tres reglas

1. **Cada valor tiene un dueño.**
2. **Solo un dueño a la vez.**
3. **Cuando el dueño sale de ámbito, el valor se libera.**

**Fig. 2.1** | El ámbito de una variable.

```rust
// fig02_01.rs
fn main() {
    {
        let s = String::from("hola");     // s es la dueña
        println!("{s}");
    }                                     // aquí termina el ámbito: se libera. Sin free(), sin GC
}
```

```bash
$ rustc --edition 2024 fig02_01.rs && ./fig02_01
hola
```

## Movimiento: lo que rompe la intuición

**Fig. 2.2** | Mover, no copiar.

```rust
// fig02_02.rs
fn main() {
    let a = String::from("hola");
    let b = a;                  // NO copia: MUEVE. Ahora b es la dueña
    println!("{a}");            // ← error: valor movido
}
```

```bash
$ rustc --edition 2024 fig02_02.rs
error[E0382]: borrow of moved value: `a`
 --> fig02_02.rs:5:16
  |
3 |     let a = String::from("hola");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es la dueña
  |             - value moved here
5 |     println!("{a}");            // ← error: valor movido
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let b = a.clone();                  // NO copia: MUEVE. Ahora b es la dueña
  |              ++++++++

warning: unused variable: `b`
 --> fig02_02.rs:4:9
  |
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es la dueña
  |         ^ help: if this is intentional, prefix it with an underscore: `_b`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
```

**En Go, `b := a` copiaría el struct (y compartiría el arreglo si fuera un slice) y las dos variables
seguirían usables.** En Rust, `a` deja de existir. El compilador lo sabe y te detiene.

Dos salidas:

**Fig. 2.3** | Las dos salidas: copiar o prestar.

```rust
// fig02_03.rs
fn main() {
    let a = String::from("hola");
    let b = a.clone();          // copia explícita: pagas la copia y lo dices
    let c = &a;                 // PRESTAR en vez de mover ← esto es lo normal
    println!("{a} {b} {c}");
}
```

```bash
$ rustc --edition 2024 fig02_03.rs && ./fig02_03
hola hola hola
```

⚠️ **`clone()` es la tentación del principiante.** Compila y funciona, y el compilador hasta lo sugiere.
Pero si tu solución a cada error es `.clone()`, estás peleando con el lenguaje en vez de usarlo. **Úsalo
cuando lo decidas a propósito, no para callar un error.**

## Préstamos: las dos reglas que lo explican todo

**Fig. 2.4** | Prestar para leer.

```rust
// fig02_04.rs
fn largo(s: &String) -> usize { s.len() }      // presta, no toma posesión

fn main() {
    let s = String::from("hola");
    let n = largo(&s);
    println!("{s} mide {n}");                   // sigue siendo mía ✓
}
```

```bash
$ rustc --edition 2024 fig02_04.rs && ./fig02_04
hola mide 4
```

🔴 **Y aquí está la regla central del `borrow checker`:**

> **Puedes tener MUCHAS referencias de solo lectura, O UNA de escritura. Nunca las dos cosas a la vez.**

**Fig. 2.5** | Lecturas y escritura a la vez.

```rust
// fig02_05.rs
fn main() {
    let mut s = String::from("hola");
    let r1 = &s;                  // lectura, ok
    let r2 = &s;                  // otra lectura, ok
    let r3 = &mut s;              // ← error: ya hay lecturas vivas
    println!("{r1} {r2} {r3}");
}
```

```bash
$ rustc --edition 2024 fig02_05.rs
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> fig02_05.rs:6:14
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

**Por qué:** eso es exactamente lo que hace imposible una carrera de datos. En Go, dos goroutines
escribiendo el mismo map rompen el programa **en ejecución** y necesitas `-race` para cazarlo. En Rust
**no compila**. Es el mismo problema, atacado 20 minutos antes.

🔑 **Y de ahí sale el lema:** *«si compila, probablemente funciona»*. No es marketing: es que el
compilador ya descartó una familia entera de errores.

## Lo que te va a pasar esta semana, y es normal

Vas a escribir código que en Go sería trivial y no va a compilar. Vas a pensar que el lenguaje es
hostil. **Eso dura entre dos y tres semanas y luego se acomoda.** No estás haciendo nada mal: estás
construyendo un modelo mental que no tenías.

**Tres cosas que ayudan:**
- **Lee el error completo, siempre.** La sugerencia suele ser la respuesta.
- **Anota cada pelea en la bitácora.** En la semana 5 las vas a releer y te van a parecer obvias — ése
  es el día en que se aprendió.
- **Cuando quieras `clone()`, párate y pregúntate si puedes prestar.** El 80 % de las veces sí.

## El ejercicio de la semana

1. Lee **el capítulo 4 completo**: qué es ownership, referencias y préstamos, y el tipo *slice*.
2. Rustlings: **`move_semantics`** (los más importantes del curso) y **`primitive_types`**.
3. Escribe una función que reciba un `String` **por valor** y otra **por referencia**. Comprueba que tras
   la primera ya no puedes usar la variable, y tras la segunda sí.
4. Provoca a propósito el error de las dos referencias mutables. **Lee el error completo y guárdalo en la
   bitácora.**
5. Escribe `fn primera_palabra(s: &str) -> &str`. Aquí aparece el `&str` frente a `String`: el primero es
   una vista prestada, el segundo es dueño. **Esa distinción no tiene equivalente en Go.**

## Cómo sé que lo logré

- [ ] Puedo explicar en una frase por qué Rust no necesita recolector de basura
- [ ] Sé decir cuándo un valor se **mueve** y cuándo se **presta**
- [ ] Sé la regla de «muchas lecturas o una escritura» y **por qué** existe
- [ ] Rustlings `move_semantics` completo
- [ ] Tengo al menos tres peleas con el borrow checker anotadas
