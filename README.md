# Curso de Rust — el segundo, después de Go

[![Verificar programas](https://github.com/HabilMX/curso-rust/actions/workflows/verificar.yml/badge.svg)](https://github.com/HabilMX/curso-rust/actions/workflows/verificar.yml)

**Por Dorian Chávez, fundador de Hábil y arquitecto de integración.**

**Cada programa de este curso se compila y se ejecuta automáticamente en cada cambio; el sello verde lo comprueba y cualquiera puede ver la corrida.**
Haz clic en el sello para abrir la última corrida y ver, paso por paso, qué se ejecutó y qué salió.

> Por ahora el curso está en español; las traducciones vienen en camino.

## Dónde está el contenido

- 📘 **[Español — el curso completo](es/README.md)** ← empieza aquí
- 💻 **[`programas/`](programas/)** — todos los programas del curso, listos para ejecutar

## Las lecciones

Numeración única y continua: «Lección N», de la 0 a la 8. Cada una se apoya en uno o dos capítulos de [The Rust Programming Language](https://doc.rust-lang.org/book/) y en sus ejercicios de [Rustlings](https://rustlings.rust-lang.org/).

| # | Lección | Qué construyes | Qué aprendes |
|---|---|---|---|
| 0 | Instalar Rust en tu Linux Mint | tu entorno y tu primer programa con `cargo` | por qué no `apt install rustc`, `rustup`, `rustc` contra `cargo`, `cargo new/build/run/check`, el libro y Rustlings sin conexión, leer un error del compilador |
| 1 | Fundamentos | las funciones y los tipos base del `revisor` | variables y `mut`, sombreado, tipos escalares y compuestos, funciones, «todo es una expresión», `if`, `loop`, `while` y `for` |
| 2 | Ownership | los programas que chocan a propósito con el compilador | las tres reglas de la propiedad, mover contra copiar, préstamos `&` y `&mut`, las dos reglas de los préstamos, por qué no hay recolector de basura |
| 3 | Structs, enums y `match` | el modelo del `revisor`: `Servicio` y `Estado` | structs e `impl`, enums que llevan datos, `match` exhaustivo, `Option` en lugar de `nil` |
| 4 | Colecciones y errores | la lista de servicios y el reporte, con errores de verdad | `Vec`, `HashMap`, `String` contra `&str`, `Result` y `?`, `panic!` contra `Result`, `anyhow` y `thiserror` |
| 5 | Traits, genéricos y lifetimes | el trait `Revisor` y una función genérica, en programas aparte del `revisor` real | traits y métodos por omisión, genéricos con restricciones, lifetimes y el `'a` que asusta |
| 6 | Módulos, pruebas y `cargo` | el proyecto `revisor` ordenado y con pruebas | módulos y visibilidad, pruebas unitarias y de integración, `cargo test`, dependencias y versiones |
| 7 | Concurrencia y async | el `revisor` concurrente: que revise todo a la vez | hilos del sistema, `Arc` y `Mutex`, canales, `async/await` con `tokio`, la comparación honesta con las gorrutinas de Go |
| 8 | El programa terminado | el `revisor` completo, en un binario | `reqwest`, `serde`, `clap`, el perfil de release, el binario final y cómo compararlo con el de Go |

Lo que la tabla promete por lección es lo que la lección trae.

## La plantilla de cada lección

Igual en todas:

1. **Encabezado:** «Lección N — título», tiempo (90 min o 2 × 45) y «Al terminar vas a poder…» (3 a 7 objetivos).
2. **El porqué antes del cómo:** qué problema resuelve el concepto, con un ejemplo del `revisor`.
3. **Los conceptos**, uno por sección: explicación, ejemplo mínimo ejecutable y ejemplo en el `revisor`.
4. **El error que vas a ver:** el mensaje real de `rustc` o de `cargo`, con su código `EXXXX`, qué significa y cómo se arregla.
5. **Lo que se hace mal** (antipatrones), con el porqué.
6. **Ejercicios** (2 a 4), de menor a mayor, con sus **soluciones** aparte.
7. **Cómo sé que lo logré:** medible; tal comando da tal salida.
8. **Para leer más:** 2 a 4 fuentes, The Rust Book y la documentación oficial primero.

## La profundidad se mide

Cada lección tiene entre 3,000 y 5,000 palabras de explicación, como las del curso de Go. `herramientas/medir-profundidad.sh` cuenta las **palabras de explicación** (lo que está fuera de los bloques de código) y exige dos varas absolutas: **piso de 3,000 palabras por lección y mediana del curso de 4,000**. Sale con código 1 si no cumple, así que la verificación automática se pone roja.

Se cuentan palabras y no líneas porque las líneas se inflan: una frase por línea suma muchas líneas sin explicar más. Y las varas son absolutas y no «la mitad de la mediana del curso» porque ese criterio se muerde la cola: unas lecciones flacas bajan la mediana y entonces pasan. Un criterio relativo garantiza uniformidad, no profundidad. Las varas se calibraron con el curso de Go medido con este mismo guion: lecciones de 3,269 a 5,097 palabras, mediana 4,321.

## Cómo ver que los programas funcionan

Sin instalar nada: abre el sello de arriba. Cada corrida muestra los pasos que se ejecutaron y su resultado.

En tu computadora, con [Rust](https://rustup.rs/) instalado (la lección 0 lo instala desde cero):

```bash
cd programas/02-ownership
rustc --edition 2024 fig02_03.rs && ./fig02_03      # compara lo que imprime con fig02_03.salida.txt
```

Cada programa de las lecciones 0 a 8 es un archivo `figNN_NN.rs` dentro de la carpeta de su lección, con su
salida esperada al lado (`figNN_NN.salida.txt`). Los que **no compilan a propósito**, porque la lección enseña
justo ese error, traen `figNN_NN.error-esperado.txt` (el mensaje del compilador) en lugar de la salida. Uno es de pruebas (`fig06_03.rs`) y se
compila con `--test`. El comando exacto de cada uno está en su lección, justo debajo del código.

El proyecto completo que se construye en las lecciones 4 a 8 es [`programas/revisor/`](programas/revisor/), con
sus pruebas:

```bash
cd programas/revisor
cargo test                          # pruebas unitarias, de integración y del binario
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

**Las lecciones son la fuente; `programas/` es una copia que se genera de ellas.** Los programas de las lecciones 0 a 8
se extraen de los bloques de código de `es/*.md` con `herramientas/generar-programas.sh`, y en cada cambio la
verificación comprueba que la copia es idéntica a lo que dicen las lecciones. Así lo que lees y lo que ejecutas
no pueden diferir.

## Qué hay en el repositorio

| | |
|---|---|
| `es/` | el curso en español, una lección por archivo |
| `en/`, `fr/`, `pt/`, `bg/` | **futuras:** las traducciones todavía no existen |
| `programas/` | los programas de las lecciones 0 a 8 (uno por archivo, con su salida esperada) y `revisor/`, el proyecto real de las lecciones 4 y 8 |
| `herramientas/` | los scripts que verifican el curso (ver abajo) |
| `.github/workflows/verificar.yml` | la verificación automática que muestra el sello |
| `verificar-publicable.sh` | revisa que el material no contenga rutas internas ni claves antes de publicarlo |
| `LICENSE.md` | CC BY-SA 4.0 |

Dentro de `herramientas/`:

| | |
|---|---|
| `verificar-programas.sh` | compila y corre cada programa completo de las lecciones (los marcados `// figNN_NN.rs`) y compara su salida real contra la documentada. Los que no compilan a propósito se comparan por su código de error |
| `verificar-ejemplos.sh` | compila y ejecuta cada ejemplo de cargo de `programas/revisor/examples/` y compara su salida con la documentada |
| `verificar-extractos.sh` | comprueba que cada bloque de código de las lecciones esté declarado (programa, extracto o fragmento) y que cada **extracto** sea copia exacta del archivo real de `programas/revisor/` |
| `generar-programas.sh` | arma `programas/` desde las lecciones; con `--comprobar` verifica que esté al día |
| `verificar-plantilla.sh` | comprueba que cada lección tenga las partes de la plantilla, objetivos, ejercicios y fuentes en el número pedido |
| `medir-profundidad.sh` | mide las palabras de explicación por lección (piso de 3,000 y mediana de 4,000) |
| `verificar-traducciones.sh`, `registrar-traduccion.sh`, `registro-traducciones.tsv` | llevan el control de qué traducciones están al día, para cuando existan |

## Qué es un programa, qué es un extracto, y por qué importa la diferencia

**No todo bloque de código de este curso es un programa completo, y confundirlos engaña al lector.** Hay cuatro tipos, y cada uno se verifica distinto:

- **Programa completo** — corre solo, de principio a fin: los bloques marcados `// figNN_NN.rs`. Se compilan y se
  ejecutan en cada cambio (con `-D warnings`: si el compilador avisa algo, falla), y su salida documentada se
  compara contra la real.
- **Extracto** — una porción exacta, copiada tal cual, de un archivo real de `programas/revisor/` (que sí es un
  proyecto completo, con sus propias pruebas). No corre por sí solo fuera de ese archivo, pero
  `herramientas/verificar-extractos.sh` confirma que sigue siendo copia fiel, línea por línea, del archivo real —para que
  un cambio en el código no deje a la lección enseñando algo que ya no existe.
- **Ejemplo de cargo** — un programa completo que usa un *crate* externo (`tokio`, `reqwest`, `serde`, `clap`) y por
  eso no se compila con un `rustc` a secas. Vive en `programas/revisor/examples/`, comparte las dependencias del
  `revisor` y se ejecuta con `cargo run --example NOMBRE`. `herramientas/verificar-ejemplos.sh` comprueba que el
  bloque de la lección sea idéntico al archivo, que el ejemplo compile y corra, y que imprima exactamente la salida
  documentada. Hay uno de `tokio` en la lección 7 y tres (`clap`, `serde`, `reqwest`) en la 8.
- **Fragmento** — una ilustración sintáctica, una firma o un trozo que depende de otro archivo (o de un *crate* que
  el ejemplo no trae), para enseñar un patrón sin el ruido de un programa completo. No pretende ser copia
  exacta de nada y no se verifica en automático — se declara así, en vez de dejar que alguien lo confunda
  con una promesa que no se cumple.

## Cómo se usa el curso

El curso se apoya en [The Book](https://doc.rust-lang.org/book/) y en [Rustlings](https://rustlings.rust-lang.org/):
cada lección dice qué capítulos leer y qué ejercicios hacer. Se recomienda haber terminado antes el
[curso de Go](https://github.com/HabilMX/curso-go), porque este curso escribe el mismo programa —el `revisor`—
y se dedica a lo que Rust hace distinto.
