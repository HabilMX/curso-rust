# Curso de Rust — el segundo, después de Go

**Por Dorian Chávez, fundador de Hábil y arquitecto de integración.**

**Para quién es:** alguien que ya terminó el **curso de Go** (`hb-curso-go`) y quiere entender el otro
extremo del espectro. No se aprende Rust *en vez de* Go: se aprende **después**, y con eso se entiende
qué decisión tomó cada uno.

🔴 **Haz el de Go primero.** No es un capricho: este curso da por sabidos los conceptos base —variables,
funciones, structs, listas, errores— y se dedica a lo que Rust hace **distinto**. Empezar por aquí sería
aprender dos cosas difíciles a la vez.

**Lo que necesitas:** una computadora con Linux Mint y haber terminado el curso de Go. La
[semana 0](00-preparacion.md) instala Rust desde cero.

⚠️ **Rust saca versión nueva cada seis semanas.** Antes de cada sesión: `rustup update stable`. Y si un
tutorial no dice para qué versión está escrito, desconfía: en Rust nueve meses son **seis versiones** de
diferencia, y eso sí se nota.

## Vas a escribir el MISMO programa

El `revisor` otra vez: recibe una lista de servicios, los consulta **todos a la vez**, produce un reporte.

🔑 **Escribir el mismo programa dos veces es el método.** Leer comparaciones «Go vs Rust» no enseña nada;
pelearte con el mismo problema en los dos lenguajes sí. Y vas a descubrir que lo que en Go te tomó una
tarde, en Rust te toma tres — hasta que entiendes *por qué*, y entonces entiendes las dos cosas.

## 🔴 Lo que nadie te dice y define este curso

**Rust tiene una curva distinta, no más larga: distinta.** La sintaxis es fácil. Lo que cuesta es **el
`borrow checker`**, el componente del compilador que verifica quién es dueño de cada dato. Los datos
medidos por quienes enseñan Rust:

- **El borrow checker se vuelve intuitivo a las 2-3 semanas.** No antes. No es que seas lento: es que
  ese modelo mental se construye chocando.
- La mayoría dedica **4 a 8 semanas** a las bases antes de su primer proyecto real.
- **El compilador de Rust es el mejor profesor que existe.** Sus errores explican el problema, señalan la
  línea y **sugieren el arreglo**. En Rust se aprende leyendo errores, no evitándolos.

⚠️ **Y aquí va la diferencia más importante con el curso de Go:** en Go la biblioteca estándar alcanza
para casi todo. En Rust **no**: async, HTTP y serialización viven en *crates* externos (`tokio`, `reqwest`,
`serde`). Eso no es una carencia — es la decisión de que el estándar sea mínimo y estable. Pero significa
que aquí sí vas a usar dependencias desde temprano.

## Las ocho semanas

**El método es el que funciona, medido: leer un capítulo de The Book, hacer sus ejercicios de Rustlings,
y solo entonces seguir.** Leer de corrido retiene mucho menos, aunque tome el mismo tiempo.

| | Semana | The Book | Qué construyes |
|---|---|---|---|
| 0 | [Preparación](00-preparacion.md) | cap 1 | `cargo`, Rustlings y el juego de adivinar |
| 1 | [Fundamentos](01-fundamentos.md) | caps 2-3 | tipos, `mut`, control de flujo |
| 2 | 🔴 [**Ownership**](02-ownership.md) | **cap 4** | **la semana que decide todo** |
| 3 | [Structs, enums y match](03-structs-enums.md) | caps 5-6 | el modelo del `revisor`, con `Option` |
| 4 | [Colecciones y errores](04-colecciones-errores.md) | caps 8-9 | `Vec`, `HashMap`, `Result`, `?` |
| 5 | [Traits, genéricos y lifetimes](05-traits-genericos.md) | cap 10 | la abstracción, y el `'a` que asusta |
| 6 | [Módulos, pruebas y cargo](06-modulos-pruebas.md) | caps 7, 11, 14 | el proyecto de verdad |
| 7 | [Concurrencia y async](07-concurrencia-async.md) | caps 16-17 | **que revise todo a la vez** |
| 8 | [El programa terminado](08-el-programa.md) | cap 12 | `reqwest`, `serde`, `clap`, el binario |

**Ownership va en la semana 2 y no se puede mover.** Es el capítulo 4 de The Book por una razón: todo lo
demás —structs, colecciones, errores, concurrencia— se explica en términos de quién es dueño de qué. Si
lo saltas, las seis semanas siguientes son memorizar reglas sin sentido.

## Las tres fuentes, y cómo combinarlas

1. **[The Book](https://doc.rust-lang.org/book/)** — oficial, 21 capítulos. Lo tienes **offline**:
   `rustup doc --book`. Para este curso: **capítulos 1-12 y 16-17**.
2. **[Rustlings](https://rustlings.rust-lang.org/)** — **96 ejercicios interactivos**, muchos dirigidos
   específicamente al borrow checker. `cargo install rustlings`. **El orden importa: capítulo, luego sus
   ejercicios.**
3. **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** — como referencia rápida.

**Después, cuando ya escribas Rust que funciona:** *Rust for Rustaceans* (Jon Gjengset) es el libro al
que van los ingenieros que ya pasaron esta etapa. No antes: no sirve de introducción.

## Cómo usarlo

- **90 minutos por semana como mínimo.** Rust pide más asiento que Go; con 60 no alcanza.
- **Lee los errores del compilador completos.** No solo la primera línea. Rust te dice el problema, la
  causa y muchas veces la solución exacta. Ignorar eso es tirar la mejor herramienta que tienes.
- **`cargo clippy` desde el primer día.** Es el linter oficial y enseña Rust idiomático mientras
  trabajas.
- **[`programas/`](../programas/)** — todos los programas de las lecciones, listos para compilar y ejecutar, y el
  `revisor` completo con sus pruebas. Cada uno se verifica automáticamente en cada cambio.
- **[`bitacora.md`](bitacora.md)** — aquí importa más que en Go: anota **cada pelea con el borrow
  checker**. Cuando en la semana 5 las releas, vas a ver que las primeras ya te parecen obvias. Ése es
  el momento en que se aprendió.
