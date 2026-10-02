# Semana 0 — Instalar Rust en tu Linux Mint

**Al terminar** vas a tener Rust funcionando y habrás corrido tu primer programa.

⏱️ **Tiempo:** 30 minutos.

---

## 🔴 Lo primero: NO uses `apt install rustc`

Igual que con Go, los repositorios de Linux Mint traen una versión vieja y congelada. Pero en Rust el
problema es **peor**, y por una razón concreta:

**Rust saca una versión nueva cada seis semanas.** Go saca dos al año. Eso significa que un Rust de los
repositorios de Mint puede estar **diez o quince versiones atrás**, y en Rust eso sí se nota: cosas que
todo internet usa como normales simplemente no existen en tu compilador.

🔑 **La forma correcta es `rustup`**, la herramienta oficial. No instala Rust: instala un **administrador
de versiones de Rust**, que es mejor, porque después actualizas con un comando y nunca te quedas atrás.

---

## Paso 1 — Instala `rustup`

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

⚠️ **Antes de correr eso, entiende lo que hace**, porque «descargar un script de internet y ejecutarlo»
es algo que deberías cuestionar siempre:

- `curl` descarga el script del sitio oficial de Rust.
- `--proto '=https' --tlsv1.2` obliga a que la descarga sea cifrada y moderna: nadie puede
  interceptarla y cambiarte el script.
- `| sh` se lo pasa al shell para que lo ejecute.

**Es el método oficial del proyecto Rust**, y si quieres verlo antes de correrlo —buena costumbre—:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
less rustup.sh      # lo lees, sales con q
sh rustup.sh        # y entonces lo corres
```

Te va a preguntar qué quieres. **Responde `1` (instalación por omisión).**

## Paso 2 — Activa las herramientas en tu terminal

El instalador ya modificó tu `~/.profile` (o `~/.bashrc`), pero el cambio aplica a la **próxima** sesión.
Para usarlo ahora mismo:

```bash
source "$HOME/.cargo/env"
```

## Paso 3 — Comprueba

```bash
rustc --version     # el compilador
cargo --version     # el administrador de proyectos
```

Deben responder con la misma versión, algo como `1.98.1`. Si quieres confirmar que es **la última**:

```bash
rustup update stable
```

Si ya la tienes, te dirá `unchanged`. **Este comando es el que vas a correr cada pocas semanas**, y es la
razón por la que `rustup` es mejor que `apt`.

## Paso 4 — El libro y el linter, que vienen incluidos

```bash
rustup doc --book        # abre The Book en tu navegador, SIN internet
cargo clippy --version   # el linter oficial
```

🔑 **The Book es el libro oficial de Rust, es gratuito, y lo acabas de instalar completo en tu máquina.**
Este curso lo usa como texto base: cada semana te dice qué capítulos leer. No hay que comprar nada.

---

## Qué son `rustc` y `cargo`, y por qué hay dos

| | Qué hace | ¿Lo usas? |
|---|---|---|
| **`rustc`** | el compilador: traduce tu código a un programa | **casi nunca directo** |
| **`cargo`** | el que organiza todo: crea proyectos, descarga dependencias, compila, prueba | **siempre** |

`cargo` llama a `rustc` por ti. Es parecido a lo que en otros lenguajes serían `npm` + `make` + el
compilador, todo en una sola herramienta que viene incluida. **En Rust nadie discute qué herramienta de
construcción usar**, y eso ahorra muchísimo tiempo.

---

## Tu primer programa

```bash
mkdir -p ~/w/curso-rust && cd ~/w/curso-rust
cargo new hola
cd hola
```

**Mira lo que te creó:**

```
hola/
├── Cargo.toml        ← la ficha del proyecto: nombre, versión, dependencias
├── .gitignore        ← cargo ya te preparó git
└── src/
    └── main.rs       ← tu código
```

`cargo new` ya inicializó un repositorio de git y escribió un `.gitignore` correcto. Es un detalle, pero
dice mucho de la herramienta: asume que vas a hacer las cosas bien y te lo deja listo.

Abre `src/main.rs`:

**Fig. 0.1** | El primer programa.

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

Y córrelo con cargo:

```bash
cargo run
```

Cada programa de este curso también se puede compilar suelto, sin proyecto, con `rustc`: el primer comando de
la figura de arriba lo hace, y `--edition 2024` pide la edición actual del lenguaje. Así es como se comprueba
cada figura del curso, y la salida que ves ahí es la real.

Verás que compila y luego ejecuta. **Fíjate en el `!` de `println!`**: no es un error de escritura. En
Rust, lo que termina en `!` es una **macro**, no una función normal. Una macro es código que genera código
al compilar. Por ahora te basta con saber que `println!` lleva `!` y `!` significa macro — en el capítulo
20 de The Book se explica por qué.

---

## Las órdenes de `cargo` que vas a usar siempre

```bash
cargo run                # compila y ejecuta
cargo build              # solo compila (modo desarrollo)
cargo build --release    # compila optimizado (para cuando el programa ya sirve)
cargo check              # 🔑 solo REVISA que compile, sin generar el programa: mucho más rápido
cargo test               # corre las pruebas
cargo clippy             # el linter: te dice cómo escribirlo mejor
cargo fmt                # ordena el código, como gofmt
```

### Dos que conviene entender desde hoy

**`cargo check` va a ser tu mejor amigo.** Rust compila **lento** —mucho más lento que Go, y hay que
saberlo de antemano para no frustrarse—. Pero `cargo check` solo verifica que el código esté correcto sin
producir el programa, y es varias veces más rápido. **Mientras programas, usa `check`; cuando quieras
probarlo, `run`.**

**`--release` importa más de lo que parece.** Sin él, Rust compila sin optimizar y con comprobaciones
extra, así que el programa corre **mucho** más lento. Si algún día mides velocidad, **mídela siempre con
`--release`** o vas a medir algo que no existe.

---

## La primera diferencia con casi todo lo que hayas visto

Escribe esto en `src/main.rs`:

**Fig. 0.2** | Un programa que no compila.

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

Y corre `cargo run` (o `rustc`, como en la figura). **No compila:** el mensaje es el mismo, solo cambia la ruta
del archivo que nombra.

**En Rust, todo es inmutable por omisión.** Una variable, una vez que tiene valor, no cambia — a menos
que pidas permiso explícitamente:

**Fig. 0.3** | La misma variable, ahora mutable.

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

**¿Por qué?** Porque la mayoría de las variables de un programa real **nunca deberían cambiar**, y cuando
cambian sin que nadie lo espere aparecen los errores más difíciles de encontrar. Rust invierte el
omisión: si algo va a cambiar, tienes que decirlo, y así quien lea tu código sabe de un vistazo qué se
mueve y qué no.

## 🔑 Y lo más importante de esta semana: lee los errores

Vuelve a mirar ese mensaje de error. Te dijo:

1. **Qué** pasó: no puedes asignar dos veces a una variable inmutable.
2. **Dónde** empezó el problema: la línea 3, donde la creaste (la línea 1 de la figura es su nombre).
3. **Dónde** truena: la línea 4.
4. **Cómo arreglarlo**: `consider making this binding mutable`, y te muestra exactamente qué escribir.

**Casi ningún compilador del mundo hace esto.** El de Rust es famoso por ello, y tiene una consecuencia
práctica para ti: **en Rust se aprende leyendo errores.** Vas a ver muchos, sobre todo las primeras
semanas. No son un castigo: son la clase.

⚠️ **La costumbre que tienes que construir desde hoy: leer el error COMPLETO.** No solo la primera línea.
La ayuda casi siempre está al final, y es la respuesta.

---

## ✅ Cómo sé que lo logré

- [ ] `rustc --version` y `cargo --version` responden con la misma versión
- [ ] `rustup update stable` dice que ya estoy al día
- [ ] `cargo run` imprimió mi mensaje
- [ ] `rustup doc --book` me abrió el libro
- [ ] Provoqué el error de la variable inmutable y **leí el mensaje completo**
- [ ] Sé la diferencia entre `rustc` y `cargo`, y cuál voy a usar
- [ ] Sé para qué sirve `cargo check` y por qué me va a importar

## 📚 Lo que te toca leer

**The Book, capítulo 1** completo (instalación, hola mundo, hola cargo). Son 20 minutos y varias cosas ya
las hiciste, así que va a ser rápido: `rustup doc --book`.

## 😕 Si algo no funcionó

| El síntoma | Qué pasa |
|---|---|
| `cargo: command not found` | falta `source "$HOME/.cargo/env"`, o cierra y abre la terminal |
| `rustc` dice una versión vieja (1.7x, 1.8x) | tienes el de `apt` estorbando. `sudo apt remove rustc cargo` y vuelve a abrir la terminal |
| `error: linker 'cc' not found` | te falta el compilador de C, que Rust usa para enlazar: `sudo apt install build-essential` |
| La compilación tarda muchísimo | es normal en Rust, sobre todo la primera vez. Usa `cargo check` mientras programas |

⚠️ **El tercero les pasa a casi todos en una Linux Mint recién instalada**, así que si te sale, no es que
hayas hecho nada mal: simplemente faltaba `build-essential`.
