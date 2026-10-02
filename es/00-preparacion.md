# Lección 0 — Instalar Rust en tu Linux Mint

**Tiempo:** 90 min.

**Qué construyes:** tu entorno y tu primer programa con `cargo`.

**Qué aprendes:** por qué no `apt install rustc`, `rustup`, `rustc` contra `cargo`, `cargo new/build/run/check`, el libro y Rustlings sin conexión, leer un error del compilador.

## Al terminar vas a poder

- Instalar Rust estable con `rustup` y explicar por qué no conviene depender de `apt install rustc`.
- Identificar si la terminal está usando las herramientas administradas por `rustup`.
- Distinguir el trabajo de `rustc` del trabajo de `cargo`, y saber cuál usar en cada caso.
- Crear un proyecto con `cargo new`, compilarlo, ejecutarlo y revisarlo con `cargo check`.
- Abrir The Rust Book sin conexión e instalar Rustlings para practicar localmente.
- Leer completo un diagnóstico de `rustc`, ubicar la línea responsable y probar el arreglo sugerido.

## El porqué antes del cómo

Esta lección no parece una lección de Rust porque todavía no enseña ownership, tipos ni `match`. Sin embargo, decide una parte importante de cómo vas a aprender el lenguaje: desde el primer día vas a trabajar con la misma cadena de herramientas, las mismas convenciones y el mismo tipo de errores que usa un proyecto real. Instalar algo que “más o menos compila” es suficiente para un ejercicio aislado; instalar el entorno correcto es necesario para seguir un curso, leer documentación actual y construir el `revisor` sin que la herramienta se vuelva un problema adicional.

El curso se escribió y se comprobó con Rust estable 1.98.1 y la edición 2024. Es una referencia concreta para que los programas, mensajes y ejemplos tengan el mismo significado para todos; con una estable posterior los programas deben comportarse igual, aunque el texto de algún mensaje del compilador puede cambiar de redacción. Rust publica una versión estable aproximadamente cada seis semanas. Linux Mint, en cambio, hereda buena parte de sus paquetes de Ubuntu, y una distribución LTS prioriza estabilidad del sistema: congela versiones principales y aplica parches de seguridad. Es una decisión razonable para programas del sistema; no es una buena manera de seguir de cerca un lenguaje cuyo ecosistema, documentación y herramientas cambian con frecuencia.

Por eso `apt install rustc` parece funcionar al principio y puede causar confusión después. Instala un compilador llamado `rustc`, pero no necesariamente el compilador que usan The Rust Book, los ejemplos recientes o los proyectos que encuentres. El problema no siempre se manifiesta como “tu versión es vieja”. A veces aparece como una característica desconocida, una edición que no existe, una sugerencia del compilador distinta o una dependencia que ya no acepta esa versión. Es el peor tipo de falla de preparación: ocurre más tarde y parece un error de tu programa.

Rust resuelve esto con `rustup`. No es solo un instalador: es el administrador oficial de toolchains de Rust. Una *toolchain* reúne una versión de `rustc`, `cargo`, la biblioteca estándar, documentación y componentes relacionados que deben funcionar en conjunto. `rustup` instala el canal estable y coloca sus ejecutables en una ubicación predecible dentro de tu usuario. Cuando toque actualizar, cambias todo ese conjunto con `rustup update stable`, sin mezclar paquetes del sistema ni bajar archivos manualmente.

La comparación con Go ayuda a ubicar la decisión. En Go instalaste la distribución oficial porque el paquete de la distribución también podía quedarse congelado; Rust vuelve más visible ese problema porque su ritmo de publicación es más corto y porque `cargo` integra compilación, dependencias, pruebas, formato y análisis estático. En ambos cursos se construye el mismo `revisor`: una herramienta que lee una lista de servicios, los consulta y reporta su estado. En Go, `go` concentra muchas tareas. En Rust, `cargo` cumple ese papel alrededor de `rustc`. La diferencia no cambia la disciplina: se trabaja dentro de un proyecto, se compila con una herramienta repetible y se lee el diagnóstico antes de cambiar código al azar.

El objetivo no es memorizar una lista de órdenes. Es construir un modelo mental sencillo. `rustc` transforma un archivo Rust en código ejecutable y reporta errores del lenguaje. `cargo` entiende un proyecto completo: conoce su nombre, edición, dependencias, pruebas, perfiles de compilación y estructura de archivos; después llama a `rustc` con los argumentos correctos. Al comenzar usarás `rustc` directamente para ver sin ruido qué hace el compilador. En el trabajo diario usarás `cargo`, porque un programa real rara vez consiste en un archivo sin dependencias.

La otra decisión importante de esta lección es cómo responder a un error. Rust no intenta adivinar lo que quisiste decir ni deja pasar código dudoso para fallar después. El compilador detiene la compilación, señala tanto el origen como el uso problemático y, en muchos casos, propone un cambio concreto. Esto no significa que todos los mensajes sean fáciles desde el primer día; significa que vale la pena leerlos completos. En Rust, el compilador es parte del proceso de aprendizaje. Las próximas lecciones te harán provocar errores de propiedad, préstamos y tipos a propósito porque entenderás más al diagnosticar una falla real que al memorizar una regla aislada.

## Los conceptos

### `rustup` instala y administra la toolchain

La instalación recomendada en Linux Mint es la publicada por el propio proyecto Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Antes de ejecutar una orden que descarga y ejecuta un script, vale la pena entenderla. `curl` descarga el instalador; `--proto '=https'` limita la descarga a HTTPS; `--tlsv1.2` exige una conexión TLS moderna; `-sSf` hace que la orden falle si el servidor devuelve un error; y `| sh` pasa el contenido descargado al intérprete de comandos. Es el método oficial, pero que sea oficial no elimina la responsabilidad de revisar qué ejecutas.

Si prefieres inspeccionarlo primero, descarga el archivo, léelo y ejecútalo solo después de revisarlo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
less rustup.sh
sh rustup.sh
```

El instalador ofrece una instalación por omisión. Elige la opción `1`, que instala el canal estable para tu plataforma. Después de terminar, abre una terminal nueva. Si quieres usar Rust en la terminal actual sin cerrarla, carga el archivo de entorno que configuró el instalador:

```bash
source "$HOME/.cargo/env"
```

La ubicación importante es `~/.cargo/bin`. Ahí `rustup` coloca los programas que invocas: `rustup`, `rustc`, `cargo`, `clippy-driver` y otros. La terminal solo encuentra comandos que estén en su variable `PATH`; por eso puede existir una instalación correcta en disco y, aun así, aparecer `command not found`. El instalador intenta actualizar tu configuración de inicio, pero cada shell y cada terminal puede leer archivos distintos.

Comprueba la instalación con estas órdenes:

```bash
rustup show active-toolchain
rustc --version
cargo --version
type -a rustc
type -a cargo
```

La primera indica la toolchain activa. Las dos siguientes deben mostrar Rust 1.98.1 o una versión estable posterior si ya actualizaste el entorno. `type -a` es especialmente útil si alguna vez instalaste Rust con `apt`: enumera todas las coincidencias que encuentra la shell y permite descubrir que estás ejecutando un binario antiguo antes que el de `~/.cargo/bin`.

No confundas una actualización de Linux Mint con una actualización de Rust. Para actualizar la toolchain estable usa:

```bash
rustup update stable
```

No hace falta correrlo antes de cada comando; sí conviene hacerlo periódicamente y antes de empezar una sesión después de varias semanas. Si ya estás al día, `rustup` lo indicará. La intención no es perseguir números de versión por deporte: es mantener alineados compilador, documentación, ejemplos y componentes instalados.

En el `revisor`, esta decisión se refleja desde su raíz. El proyecto declara la edición 2024 y Cargo construye todos sus módulos con la toolchain activa. No hay una “versión de Rust” escondida por archivo: la configuración del paquete fija el idioma que habla el proyecto y el lockfile fija las versiones concretas de sus dependencias para que una compilación repetida resuelva el mismo conjunto.

### El `PATH` y el compilador que realmente ejecutas

Cuando escribes `rustc`, la shell no busca en todo el disco. Recorre las carpetas listadas en `PATH`, en orden, y ejecuta la primera coincidencia. Esa regla explica dos diagnósticos comunes. Si no hay ningún `rustc` en esas carpetas, `bash` suele mostrar:

```bash
rustc: command not found
```

Si sí existe uno, pero proviene de una instalación anterior de `apt`, el comando puede funcionar y mostrar una versión inesperada. Éste es más engañoso: no parece un problema de instalación, pero el curso se estaría compilando con una herramienta distinta de la esperada.

Primero confirma qué shell usas y cómo fue iniciada:

```bash
echo "$SHELL"
echo "$0"
printf '%s\n' "$PATH"
```

En una instalación usual con Bash, `~/.bashrc` se carga para shells interactivas y `~/.profile` para una sesión de inicio. En Zsh, el archivo equivalente para sesiones interactivas suele ser `~/.zshrc`. El archivo que `rustup` crea, `~/.cargo/env`, agrega `~/.cargo/bin` al `PATH`. Ejecutar `source "$HOME/.cargo/env"` en la terminal actual es una comprobación directa: si tras hacerlo `rustc --version` funciona, el problema era el entorno de la shell, no el compilador.

No agregues rutas duplicadas una y otra vez sin comprobar qué pasó. Primero usa `type -a rustc`. Si aparece una ruta de `~/.cargo/bin`, `rustup` está disponible en el `PATH`. Si aparece antes una ruta como `/usr/bin/rustc`, hay una instalación del sistema tomando prioridad. En ese caso, identifica primero qué paquetes están instalados y cuál binario está usando la shell; no intentes arreglarlo copiando ejecutables ni modificando enlaces simbólicos a mano.

El `revisor` no depende de una ruta fija del compilador. Esto es una ventaja de trabajar con `cargo`: la herramienta invoca el `rustc` de la toolchain activa y conserva el resultado de las compilaciones dentro de `target/`. Por eso un proyecto Cargo puede construirse igual en otra computadora con una instalación correcta, sin que el código contenga rutas personales ni comandos específicos de tu máquina.

### `rustc`: el compilador y el primer programa

`rustc` es el compilador de Rust. Recibe código fuente, verifica que respete las reglas del lenguaje y, si todo está bien, produce un ejecutable. Para un archivo pequeño es útil invocarlo directamente porque permite ver la relación exacta entre fuente, compilación y programa resultante. Cada figura de este curso se compila así para que la salida documentada corresponda con el código que estás leyendo.

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

`fn main()` declara la función por la que inicia un programa ejecutable. Rust busca precisamente una función llamada `main` para empezar. Dentro, `println!` imprime texto y añade un salto de línea. El signo `!` no es decorativo: `println!` es una macro. Las macros generan o transforman código durante la compilación; por ahora basta con reconocer la convención de que los nombres que terminan en `!` no son funciones normales. The Rust Book vuelve a ese tema en el capítulo 20.

El indicador `--edition 2024` selecciona la edición actual del lenguaje. Una edición no significa que tu código se convierte automáticamente a un idioma distinto cada año; es una forma de que Rust pueda mejorar reglas y sintaxis sin romper silenciosamente proyectos existentes. El proyecto `revisor` declara esa misma edición en su manifiesto. Los ejemplos del curso la indican explícitamente para que el comando de una figura no dependa de la edición predeterminada de una instalación particular.

Este uso directo de `rustc` es deliberadamente pequeño. Si tuvieras dos archivos, una biblioteca, pruebas, dependencias externas, opciones de optimización y distintas plataformas de destino, escribir a mano la invocación correcta del compilador se volvería frágil. Ahí entra `cargo`. La relación no es una competencia entre dos programas: Cargo organiza, Rustc compila. Cuando usas `cargo build`, Cargo termina llamando a `rustc` por ti con las rutas, edición y banderas que el proyecto necesita.

El mismo punto existe en el `revisor`: el binario tiene una función `main`, pero no se compila aislando ese archivo con `rustc src/main.rs`. Importa módulos de la biblioteca local y crates externos; necesita el manifiesto y la estructura que Cargo conoce. Las figuras de esta lección enseñan el mecanismo de compilación. Las lecciones posteriores aplican ese mecanismo a un programa compuesto.

### `cargo`: el proyecto antes que el archivo

Crea tu primer proyecto en una carpeta de trabajo propia:

```bash
mkdir -p "$HOME/w/curso-rust"
cd "$HOME/w/curso-rust"
cargo new hola
cd hola
```

`cargo new hola` crea una carpeta llamada `hola`, un manifiesto `Cargo.toml` y el archivo `src/main.rs`. Si Git está instalado y Cargo puede inicializar un repositorio, también prepara Git y un `.gitignore`; si Git no está disponible, el proyecto sigue siendo válido. El resultado mínimo tiene esta estructura:

```text
hola/
├── Cargo.toml
├── .gitignore
└── src/
    └── main.rs
```

`Cargo.toml` es el manifiesto del paquete. Escribe la identidad del proyecto, su edición, sus dependencias y algunas decisiones de construcción. El archivo no es un detalle administrativo: es lo que permite que otra persona ejecute el mismo `cargo build` sin reconstruir a mano la lista de argumentos del compilador. Cuando Cargo resuelve dependencias, crea además `Cargo.lock`; ese archivo registra la resolución concreta para que las compilaciones sean repetibles.

El `revisor` ya es un proyecto Cargo completo. Éste es su manifiesto real:

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

No necesitas entender todavía las dependencias ni el perfil de liberación. Lo importante es reconocer la forma: `[package]` describe el paquete; `[dependencies]` enumera lo que necesita para compilar; `[profile.release]` cambia cómo se construirá el binario final. En la lección 4 conocerás las dependencias de errores y serialización, en la 6 el orden de módulos y pruebas, y en la 8 el perfil de liberación. Hoy basta con entender por qué un proyecto necesita un manifiesto y por qué no conviene sustituir Cargo por una larga orden manual de `rustc`.

Ejecuta el proyecto recién creado con:

```bash
cargo run
```

La primera vez Cargo compila el paquete y después ejecuta el binario. En las siguientes ejecuciones, reutiliza artefactos que no cambiaron. A diferencia de `rustc fig00_01.rs`, no tienes que escribir el nombre del archivo ni el nombre del ejecutable: Cargo conoce la convención `src/main.rs` y sabe que el paquete `hola` produce el binario `hola`.

Esta convención reduce decisiones repetitivas. Un programa Rust puede organizarse de varias maneras, pero Cargo da una estructura común para los casos frecuentes. El proyecto que creaste hoy trae solo `src/main.rs`, el binario. Cuando abras el `revisor`, reconocerás además `src/lib.rs`, la biblioteca del paquete. Esa separación no se inventa en la lección 6: Cargo la reconoce por convención, igual que reconoce `src/main.rs`.

### `cargo build`, `run`, `check` y el ciclo de trabajo

Las órdenes de Cargo no son sinónimos. Cada una responde a una pregunta distinta que te haces mientras trabajas:

```bash
cargo run
cargo build
cargo build --release
cargo check
cargo test
cargo clippy
cargo fmt
```

`cargo run` responde “¿mi programa compila y qué hace?”. Primero construye lo necesario y luego ejecuta el binario. Es la orden que usarás cuando cambies una salida, pruebes una rama del programa o quieras observar un comportamiento. Para el proyecto `hola`, debe imprimir el mensaje que haya en `src/main.rs`.

`cargo build` responde “¿puedo producir el binario?”. Compila el paquete, pero no lo ejecuta. En modo de desarrollo deja los artefactos en `target/debug/`; no necesitas aprender esa ruta de memoria, pero ayuda saber que Cargo no llena la carpeta raíz del proyecto de ejecutables y archivos intermedios. Eso mantiene separados el código fuente y los resultados de compilación.

`cargo build --release` produce el perfil de liberación, normalmente con más optimización y con resultados dentro de `target/release/`. La diferencia importa cuando vayas a entregar el `revisor` terminado o medir su desempeño. No compares tiempos de ejecución de un binario de desarrollo con conclusiones sobre el rendimiento de Rust: el perfil de desarrollo prioriza compilar rápido y depurar cómodamente; el perfil de liberación prioriza el programa resultante. El `Cargo.toml` del `revisor` muestra que ese perfil incluso puede ajustar tamaño, optimización entre módulos y comportamiento ante `panic!`.

`cargo check` responde “¿el compilador acepta mi código?”. Revisa tipos, préstamos, módulos y gran parte del trabajo de compilación, pero no completa la generación de un binario ejecutable. En un proyecto que está creciendo puede ahorrar tiempo. Úsalo mientras escribes, especialmente cuando solo quieres saber si una modificación es válida; usa `cargo run` cuando además quieras ejecutar el comportamiento. Ninguna de las dos órdenes reemplaza a la otra: una verifica con rapidez y la otra verifica además el resultado en ejecución.

`cargo test` compila y corre pruebas. Todavía no has escrito pruebas en esta lección, pero empezarás a verlo en la lección 6. `cargo clippy` ejecuta el linter oficial y te señala patrones que compilan pero suelen ser confusos, ineficientes o poco idiomáticos. `cargo fmt` aplica el formato estándar de Rust. Es la misma disciplina que `gofmt` en Go: no se invierte tiempo discutiendo la alineación de cada archivo, se deja que la herramienta dé una respuesta uniforme.

El `revisor` usa exactamente ese ciclo. Antes de publicar un cambio conviene ejecutar, desde `programas/revisor/`, estas órdenes:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

La primera comprueba comportamiento; la segunda convierte avisos de Clippy en fallas para no ignorarlos; la tercera confirma que el código ya tiene el formato esperado. No debes ejecutarlas todavía para “entender” su salida. Guárdalas como referencia de la rutina a la que llegarás al construir el programa.

### Inmutabilidad por omisión y la primera conversación con `rustc`

Rust considera inmutable una variable creada con `let`, salvo que escribas `mut`. Esto no es un obstáculo puesto para hacer más largo el código. Es una declaración visible de intención: si un valor debe cambiar, el lector y el compilador deben poder verlo en el lugar donde se define la variable.

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

La solución sugerida por el compilador es correcta si realmente quieres cambiar el valor. La palabra `mut` se escribe en la declaración, no en la asignación posterior. Así, quien lea el bloque sabe desde el inicio que `x` es parte del estado mutable de esa función.

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

La inmutabilidad por omisión ayuda a reducir cambios accidentales y prepara el terreno para ownership y préstamos. En otros lenguajes es común modificar una variable porque el lenguaje lo permite y solo después preguntarse quién dependía de su valor anterior. Rust pide declarar esa posibilidad desde el principio. No elimina todos los errores, pero transforma una suposición implícita en una propiedad comprobable.

En el `revisor` hay mutabilidad solo donde la operación realmente la requiere. La función que ordena las filas crea un vector y después lo ordena en el lugar; por eso la variable se declara `mut`:

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

No se escribe `mut` por costumbre. `filas.sort_by(...)` modifica el vector, así que la declaración lo comunica. En cambio, `servicios` y `estados` son referencias que esa función solo consulta; no se declaran mutables. Esa diferencia, pequeña en esta función, se vuelve importante cuando varias partes de un programa intentan usar los mismos datos. La lección 2 explica las reglas que Rust aplica para que esos usos sean seguros.

### The Rust Book y Rustlings como práctica local

The Rust Book es el texto oficial de referencia del curso. `rustup` instala una copia local de la documentación, así que puedes abrirla sin conexión una vez terminada la instalación:

```bash
rustup doc --book
```

Ese comando abre el capítulo inicial en el navegador predeterminado. Si estás sin entorno gráfico o prefieres ubicar otros documentos, `rustup doc --help` muestra las opciones disponibles. La copia local no sustituye las actualizaciones: si actualizas la toolchain, la documentación local se actualiza con ella. Ésa es otra razón para mantener juntas compilador y documentación.

Lee completo el capítulo 1 de The Rust Book: instalación, “Hello, world!” y “Hello, Cargo!”. No lo saltes porque ya creaste un proyecto. Lo que hiciste aquí te da contexto para leerlo más rápido; el capítulo ordena los conceptos y explica las convenciones que volverán a aparecer durante todo el curso. El capítulo 2, que incluye el juego de adivinar, corresponde a la lección 1 junto con los fundamentos del capítulo 3.

Rustlings complementa al libro con ejercicios pequeños que se resuelven editando archivos locales. Su instalación inicial sí necesita acceso a la red para que Cargo descargue el programa; después, el directorio de ejercicios y su código viven en tu computadora. Instálalo e inicialízalo así:

```bash
cargo install rustlings
rustlings init
cd rustlings
rustlings
```

El comando interactivo observa los ejercicios, te indica cuál falla y vuelve a comprobarlos conforme guardas cambios. No uses Rustlings como una colección de respuestas por tachar. El orden del curso es intencional: primero lee el capítulo de The Rust Book, luego resuelve los ejercicios vinculados y después aplica la idea al `revisor`. En esta etapa, instala Rustlings y familiarízate con su directorio; en la lección 1 trabajarás sus secciones `variables`, `functions`, `if` y `primitive_types`.

El libro y los ejercicios cumplen funciones distintas. The Rust Book explica el modelo y nombra sus piezas; Rustlings te obliga a tocar el código y recibir un error concreto. El `revisor` es el problema de integración: no es un ejercicio aislado, sino el mismo programa que ya construiste en Go, ahora con las decisiones de Rust. Las tres fuentes se apoyan entre sí. Si una explicación parece abstracta, prueba un ejercicio; si un ejercicio se siente mecánico, vuelve al capítulo; si ambos ya son claros, ubica el patrón dentro del proyecto real.

## El error que vas a ver

El error central de esta lección es `E0384`, mostrado completo en la Fig. 0.2. No lo leas como una pared de texto. Léelo en orden. La primera línea nombra el código de diagnóstico, `E0384`, y resume el problema: no puedes asignar dos veces a una variable inmutable. Ese código sirve para pedir una explicación ampliada al compilador:

```bash
rustc --explain E0384
```

La línea que inicia con `--> fig00_02.rs:4:5` localiza el intento de asignación: archivo, línea y columna. Las líneas con números muestran contexto suficiente para no tener que buscar a ciegas. La marca `^^^^^` señala la parte exacta que provoca el error. Antes de ella aparece la primera asignación, en la línea 3, porque Rust no solo informa dónde detectó el problema: también enseña el origen de la condición que lo hace inválido.

La sección `help:` merece atención especial. En este caso propone cambiar `let x = 5;` por `let mut x = 5;`, y las marcas `+++` indican qué texto añadir. No apliques todas las sugerencias mecánicamente. Primero verifica la intención: si `x` no debería cambiar, la solución correcta no es añadir `mut`, sino eliminar o replantear la asignación posterior. El compilador puede ofrecer un arreglo local; tú decides si ese arreglo representa el diseño correcto.

El mismo diagnóstico también incluye un aviso: el primer valor, `5`, nunca se lee porque se sobrescribe inmediatamente. El aviso no impide compilar por sí mismo, pero aporta información útil. Si conviertes `x` en mutable sin mirar el aviso, el programa seguirá teniendo una asignación innecesaria. La versión de la Fig. 0.3 imprime `5` antes de cambiarlo, así que ambos valores tienen sentido y el programa no genera avisos.

Cuando ejecutes el mismo error dentro de un proyecto con `cargo run` o `cargo check`, Cargo mostrará el diagnóstico de `rustc` junto con contexto del paquete y la ruta `src/main.rs`. La regla no cambia: empieza por el primer error, lee sus notas y su ayuda, corrige una causa a la vez y vuelve a compilar. Un error inicial puede provocar varios posteriores; intentar arreglar todos de una sola vez suele ocultar la causa real.

Hay otros errores de preparación que no son códigos de Rust porque ocurren antes de que el compilador pueda analizar tu programa. Si `cargo` o `rustc` no existen para la terminal, el problema es `PATH`; vuelve a cargar `~/.cargo/env` y revisa `type -a cargo`. Si la compilación llega a enlazar y aparece un mensaje parecido a `error: linker 'cc' not found`, falta el compilador de C que Rust usa para enlazar en Linux Mint. Instala el paquete de herramientas de construcción de la distribución y vuelve a ejecutar la orden:

```bash
sudo apt install build-essential
```

No confundas ese caso con “Rust no se instaló”. `rustc --version` puede funcionar perfectamente; la falla aparece después, cuando el compilador necesita convertir objetos compilados en un ejecutable del sistema. Separar la etapa que falla evita arreglos aleatorios.

## Lo que se hace mal

- Instalar Rust con `apt install rustc` y dar por hecho que el nombre del paquete garantiza una toolchain actual. El problema no es que el paquete sea inútil; es que sigue el calendario de la distribución, no el de Rust. Para este curso usa `rustup`, comprueba `rustc --version` y actualiza el canal estable periódicamente.

- Mezclar una instalación de `apt` con otra de `rustup` sin revisar cuál gana en `PATH`. Tener dos ejecutables llamados `rustc` no produce necesariamente un error inmediato. Puede compilar durante días con la versión equivocada. Usa `type -a rustc` y `type -a cargo` antes de modificar archivos de inicio o eliminar paquetes.

- Usar `rustc` para un proyecto completo por costumbre. Para una figura de un archivo es una herramienta didáctica excelente. Para el `revisor`, significaría reconstruir manualmente dependencias, rutas, edición, módulos y perfiles. Usa `cargo` desde la raíz del proyecto; deja que él construya la invocación de `rustc`.

- Usar `cargo run` cada vez que quieres saber si el código compila. Funciona, pero construye y ejecuta aunque solo estés corrigiendo tipos o préstamos. Durante edición rápida, `cargo check` da retroalimentación más directa. Cuando necesites observar el comportamiento, usa `cargo run`.

- Medir rendimiento con un binario de desarrollo. `cargo build` y `cargo run` usan el perfil de desarrollo por omisión. Cuando el curso llegue a comparar tamaño, velocidad o entrega del binario, usa `cargo build --release`. Sin esa distinción, una medición responde más sobre el perfil elegido que sobre el programa.

- Ignorar un aviso porque “no impide compilar”. Los avisos suelen señalar valores no usados, código muerto o construcciones confusas. Este curso compila las figuras correctas con avisos tratados como errores para que la salida mostrada no oculte problemas. Haz lo mismo en tu rutina: entiende el aviso o elimina su causa.

- Leer solo la primera línea de un error. La primera línea nombra la categoría; las líneas siguientes dicen dónde ocurrió, qué valor previo la explica, qué notas aplican y qué alternativa considera el compilador. Copiar solo “error E0384” para buscarlo pierde buena parte de la respuesta que ya está frente a ti.

- Instalar Rustlings y resolver ejercicios con respuestas copiadas. Un ejercicio terminado sin entender el diagnóstico no construye el modelo mental que necesitarás para ownership. Haz cambios pequeños, ejecuta el verificador, lee el error y explica con tus palabras por qué la solución compila.

## Ejercicios

### Ejercicio 1 — Comprueba tu toolchain

Instala Rust con `rustup` si todavía no lo tienes. Ejecuta `rustup show active-toolchain`, `rustc --version`, `cargo --version` y `type -a rustc`. Anota qué ruta está usando la terminal para `rustc` y confirma que corresponde a `~/.cargo/bin` cuando usas la instalación administrada por `rustup`.

### Ejercicio 2 — Crea y recorre un proyecto Cargo

En una carpeta de trabajo, ejecuta `cargo new saludo-rust` y entra al directorio creado. Lee `Cargo.toml` y `src/main.rs` antes de modificarlos. Cambia el mensaje por uno tuyo y corre, en este orden, `cargo check`, `cargo build` y `cargo run`. Explica qué pregunta respondió cada comando y qué archivo o resultado esperabas de cada uno.

### Ejercicio 3 — Provoca y explica `E0384`

Sustituye temporalmente el contenido de `src/main.rs` por el programa de la Fig. 0.2 y ejecuta `cargo check`. No arregles nada hasta haber identificado el archivo, la línea, la primera asignación y la sugerencia marcada como `help:`. Después cambia la declaración a `let mut x = 5;`, observa el aviso restante y modifica el programa para que los dos valores se lean, como en la Fig. 0.3.

### Ejercicio 4 — Prepara la lectura y la práctica local

Abre The Rust Book con `rustup doc --book` y lee por completo el capítulo 1. Instala Rustlings con `cargo install rustlings`, ejecútalo en su directorio local y localiza cómo volver a abrir los ejercicios sin depender de una página web. Escribe una nota breve que distinga qué obtienes del libro, qué obtienes de Rustlings y qué construirás después en el `revisor`.

## Soluciones

### Solución 1

Una instalación correcta muestra una toolchain estable activa y permite ejecutar tanto `rustc --version` como `cargo --version`. La salida exacta puede cambiar al actualizar Rust, pero ambas herramientas deben pertenecer a la misma instalación estable. `type -a rustc` debe listar `~/.cargo/bin/rustc` como la ruta elegida o, al menos, permitirte explicar por qué otra ruta tiene prioridad. Si no aparece, ejecuta `source "$HOME/.cargo/env"` y vuelve a comprobarlo.

### Solución 2

`cargo check` verifica el proyecto sin terminar de producir un ejecutable; `cargo build` compila el paquete y deja artefactos de desarrollo dentro de `target/debug/`; `cargo run` compila lo necesario y ejecuta el binario. Los tres comandos deben aceptar el proyecto `saludo-rust`. La salida de `cargo run` debe ser exactamente el mensaje que dejaste en `src/main.rs`.

### Solución 3

`cargo check` muestra `E0384` porque `let x = 5;` crea una asociación inmutable y la línea siguiente intenta reasignarla. Cambiarla por `let mut x = 5;` permite la reasignación, pero inicialmente deja un aviso porque `5` se sobrescribe sin usarse. Imprimir `x` antes y después de la asignación elimina el aviso y produce las dos líneas de la Fig. 0.3. El aprendizaje no es “añade `mut` siempre”; es declarar mutabilidad solo cuando la modificación es parte del diseño.

### Solución 4

The Rust Book presenta la explicación ordenada de instalación, programa inicial y Cargo; su capítulo 1 queda disponible localmente con `rustup doc --book`. Rustlings aporta ejercicios editables y retroalimentación sobre código local después de instalarlo e inicializar su directorio. El `revisor` es donde esas piezas se combinan en una aplicación: no reemplaza al libro ni a los ejercicios, sino que da un problema continuo sobre el cual aplicar los conceptos de las siguientes lecciones.

## Cómo sé que lo logré

- [ ] `rustc --version` y `cargo --version` funcionan y muestran una toolchain estable compatible.
- [ ] `type -a rustc` me permite identificar qué compilador está ejecutando mi terminal.
- [ ] `rustup update stable` termina sin errores y puedo explicar qué actualiza.
- [ ] `cargo new saludo-rust` creó un proyecto con `Cargo.toml` y `src/main.rs`.
- [ ] `cargo check`, `cargo build` y `cargo run` funcionan dentro de ese proyecto, y sé qué hace cada uno.
- [ ] Mi programa imprime el mensaje que escribí cuando ejecuto `cargo run`.
- [ ] Puedo provocar `E0384`, señalar su línea de origen, leer su ayuda y arreglar el programa sin dejar avisos.
- [ ] `rustup doc --book` abre The Rust Book local y Rustlings está inicializado en una carpeta local.

## Para leer más

- [The Rust Programming Language, capítulo 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html) — instalación, primer programa y Cargo. Consultado el 2 de octubre de 2026.

- [Rust: Install](https://www.rust-lang.org/tools/install) — instalación oficial con `rustup`, actualización de toolchains y notas sobre `PATH`. Consultado el 2 de octubre de 2026.

- [The Cargo Book: Why Cargo Exists](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html) — por qué Cargo administra paquetes, dependencias y las invocaciones a `rustc`. Consultado el 2 de octubre de 2026.

- [Rustlings](https://rustlings.rust-lang.org/) — instalación, inicialización y uso de ejercicios locales en paralelo con The Rust Book. Consultado el 2 de octubre de 2026.
