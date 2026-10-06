# Lección 6 — Módulos, pruebas y Cargo

**Tiempo:** 2 × 45 min.

**Qué construyes:** el proyecto `revisor` ordenado y con pruebas.

**Qué aprendes:** módulos y visibilidad, pruebas unitarias y de integración, `cargo test`, dependencias y versiones.

**The Rust Book, capítulos 7, 11 y 14.** Rustlings: `modules`, `tests`.

## Al terminar vas a poder

- Separar un programa Rust en módulos con responsabilidades claras y navegar sus rutas con `crate`, `self` y `super`.
- Explicar por qué todo es privado por omisión y elegir entre `pub`, `pub(crate)` y una API privada.
- Distinguir una prueba unitaria de una prueba de integración y saber qué clase de problema detecta cada una.
- Escribir pruebas con `#[test]`, `assert!`, `assert_eq!`, `matches!` y `#[should_panic]`.
- Ejecutar, filtrar y diagnosticar pruebas con `cargo test`.
- Leer `Cargo.toml` y `Cargo.lock`, agregar una dependencia con una versión razonable y revisar su árbol transitivo.
- Mantener el `revisor` comprobable con `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` y `cargo test`.

## El porqué antes del cómo

Hasta ahora el `revisor` pudo caber en pocos archivos porque el curso estaba presentando las piezas del lenguaje una por una. Ya conoces los tipos que modelan un servicio, las colecciones que guardan la lista, los errores que describen fallas externas y los traits que expresan contratos. El siguiente problema no es escribir otra función: es evitar que esas funciones se conviertan en una sola masa difícil de leer, probar y cambiar.

Un archivo enorme no deja de funcionar automáticamente. El problema aparece cuando una modificación aparentemente local obliga a entender demasiadas cosas a la vez. Si el código que lee YAML, valida servicios, hace solicitudes HTTP, produce JSON y parsea argumentos vive mezclado, una prueba de formato puede terminar requiriendo red; una modificación de configuración puede afectar el binario; y una función privada puede terminar siendo usada desde cualquier lugar solo porque nadie definió una frontera.

Los módulos son esas fronteras. No son carpetas para que el proyecto “se vea ordenado”; son nombres para responsabilidades y, en Rust, también una parte explícita del control de acceso. Un módulo puede decir: “aquí se define qué es un servicio”, “aquí se carga la configuración” o “aquí se convierte un estado en una tabla”. Quien usa un módulo conoce su interfaz pública; no necesita ni debería depender de los detalles internos con que se implementa.

La palabra importante es interfaz. En Go, una carpeta define un paquete y una inicial mayúscula decide si un nombre puede cruzar la frontera del paquete. Rust es más detallado. Una carpeta puede ayudar a organizar archivos, pero la visibilidad depende de módulos y de `pub`. Un nombre sin `pub` es privado, incluso si está en otro archivo del mismo proyecto. Esto parece estricto al principio, pero evita que una función auxiliar se convierta por accidente en una promesa para el resto del programa.

El `revisor` aplica esta idea con dos productos dentro del mismo paquete Cargo. `src/lib.rs` declara una biblioteca: ahí vive la lógica reutilizable y comprobable. `src/main.rs` declara el binario: recibe argumentos, llama a la biblioteca, imprime el resultado y decide el código de salida. Separar ambos permite probar la lógica sin tener que invocar la línea de comandos en cada caso. También permite que las pruebas de integración usen el `revisor` como lo usaría otra aplicación: importando exclusivamente su API pública.

Éste es el mismo principio que usaste en Go al separar paquetes por responsabilidad, pero Rust hace más visible el contrato. En Go, una función con minúscula no puede importarse desde otro paquete; en Rust, una función, un struct, un campo o un módulo requiere visibilidad declarada. Las dos decisiones buscan limitar dependencias. Rust te da más niveles para expresar esa intención: público para cualquiera que importe la biblioteca, público solo dentro del paquete actual o público para un módulo padre.

Las pruebas convierten esas fronteras en algo verificable. Una prueba unitaria vive cerca de la función que prueba y puede examinar detalles privados. Es útil para reglas pequeñas: si el YAML no declara `timeout_ms`, ¿se aplica el valor por omisión?, ¿una lista vacía se rechaza?, ¿la tabla conserva su alineación? Una prueba de integración vive bajo `tests/`, se compila como otro crate y solo puede usar `pub`. Es útil para comprobar que la interfaz realmente alcanza: si alguien construye un `Servicio`, llama a `revisar` y recibe un `Estado`, ¿el contrato público funciona sin depender de detalles internos?

No confundas muchas pruebas con buena cobertura de decisiones. Una suite puede tener cien pruebas que repitan el mismo caso sano y ninguna que cubra una URL inválida, un archivo ausente o un servicio que tarda demasiado. Tampoco conviertas el porcentaje de cobertura en una meta aislada. La pregunta útil es: “¿qué comportamiento importante podría romperse sin que una prueba se ponga roja?” El `revisor` prueba estados sanos, HTTP 500, tiempos de espera, configuración inválida, formato JSON y códigos de salida porque esos son comportamientos que importan a quien usa el programa.

`cargo` reúne estas decisiones. No solo compila: sabe qué archivos forman el paquete, qué dependencias necesita, qué edición de Rust usa, cuáles pruebas existen y qué artefactos debe construir. En las figuras del curso sigues llamando a `rustc` directamente para ver un ejemplo aislado. En el proyecto real usas `cargo` porque ya no existe una invocación razonable a mano que recuerde todos los módulos, crates, características y destinos de prueba.

La disciplina de esta lección es sencilla: organiza por responsabilidad, abre la menor superficie pública necesaria y prueba cada frontera desde el lado correcto. Si una prueba unitaria necesita red, probablemente mezclaste una regla pura con infraestructura. Si una prueba de integración necesita importar un detalle privado, probablemente tu API pública no expresa lo que otro consumidor necesita. Si `cargo test` dice que no encontró pruebas, no te felicites todavía: revisa el conteo.

## Los conceptos

### Módulos: nombres, rutas y responsabilidades

Un módulo agrupa nombres relacionados. Puede declararse dentro de un archivo con `mod nombre { ... }`, o puede vivir en otro archivo. En un paquete Cargo moderno, `src/lib.rs` y `src/main.rs` son raíces de crate distintas. Desde cualquiera de ellas, `crate` significa “la raíz de este crate”; `self` significa el módulo actual; y `super` significa el módulo padre.

Un error frecuente es pensar que archivo y módulo son sinónimos. Un archivo puede contener varios módulos, y un módulo puede abrirse en otro archivo. La estructura de archivos ayuda a una persona a encontrar código; la estructura de módulos determina cómo Rust resuelve rutas y aplica visibilidad. No diseñes primero un árbol de carpetas vacío. Empieza por responsabilidades que tengan una razón estable para cambiar por separado.

**Fig. 6.1** | Un módulo ofrece una función pública y conserva su detalle privado.

```rust
// fig06_01.rs
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn linea(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::linea("catalogo", true));
    println!("{}", reporte::linea("pagos", false));
}
```

```bash
$ rustc --edition 2024 fig06_01.rs && ./fig06_01
catalogo: OK
pagos: FALLA
```

`reporte::linea` es accesible desde `main` porque tiene `pub`. La función `etiqueta` no tiene `pub`, así que solo puede usarse dentro de `reporte`. Esa decisión no oculta información por misterio: expresa que otras partes del programa necesitan una línea terminada, no conocer la regla interna que traduce un booleano a texto. Si más adelante cambias `"FALLA"` por `"NO DISPONIBLE"`, solo el módulo dueño necesita cambiar.

En el `revisor`, la raíz de la biblioteca enumera sus responsabilidades públicas. No hay un módulo llamado `utilidades`, porque ese nombre no explica qué responsabilidad posee. `config` carga y valida configuración; `modelo` define el vocabulario; `reporte` traduce estados a texto o JSON; `revisar` consulta servicios.

<!-- verificar:extracto:src/lib.rs -->
```rust
//! El `revisor` del curso de Rust: recibe una lista de servicios, los consulta
//! todos a la vez y produce un reporte.
//!
//! La lógica vive aquí, en la biblioteca, y `main.rs` solo lee los argumentos y
//! llama (lección 6): así todo lo de abajo se puede probar desde fuera.
//!
//! - [`modelo`]: el vocabulario (`Servicio`, `Estado`, `EstadoJson`).
//! - [`config`]: lee y valida el archivo YAML de servicios.
//! - [`revisar`]: consulta un servicio por HTTP, o todos a la vez con un límite.
//! - [`reporte`]: convierte los estados en tabla o en JSON.

pub mod config;
pub mod modelo;
pub mod reporte;
pub mod revisar;
```

La palabra `pub` delante de cada `mod` hace que esos módulos formen la entrada pública de la biblioteca. Eso no vuelve público todo su contenido. Cada módulo decide a su vez qué structs, funciones y constantes expone. Esta composición es una ventaja: publicar `reporte` permite llamar `revisor::reporte::tabla`, pero no obliga a publicar las funciones auxiliares que ordenan filas o calculan etiquetas.

El árbol de módulos del `revisor` no pretende ser una jerarquía universal. En un proyecto pequeño, cuatro módulos planos son más legibles que una cadena larga de carpetas. Cuando una responsabilidad crece lo suficiente, puede dividirse en submódulos. La pregunta no es “¿cuántos archivos debe tener un proyecto profesional?”, sino “¿puedo describir en una frase qué pertenece aquí y qué no?”.

### Visibilidad: privado por omisión como diseño

Rust empieza cerrado. Un item sin `pub` es visible en su módulo y sus descendientes, pero no para módulos hermanos ni para el padre. Esta regla es más restrictiva de lo que muchos programadores esperan después de JavaScript, Python o Go, donde una función de archivo suele ser accesible dentro del paquete. La intención es obligarte a diseñar la interfaz antes de depender de un detalle.

`pub` abre un item hacia quien pueda llegar al módulo que lo contiene. `pub(crate)` abre el item para todo el crate actual, pero no para alguien que importe la biblioteca desde otro paquete. `pub(super)` abre el item únicamente para el módulo padre. Existe también `pub(in ruta)`, útil cuando una frontera precisa de módulos expresa una regla real, aunque es menos común en proyectos pequeños.

No marques todo con `pub` para apagar errores de visibilidad. Hacerlo tiene un costo: cualquier consumidor puede empezar a depender de esos nombres, y después cambiar una función interna se vuelve una ruptura de API. Para un binario privado ese costo se queda dentro del repositorio; para una biblioteca publicada, puede obligarte a conservar una decisión accidental durante años. Empieza privado y abre solo lo que otra parte necesita.

El compilador distingue un nombre que no existe de un nombre que existe pero está cerrado. En este caso la función existe, pero `main` intenta atravesar una frontera privada.

**Fig. 6.2** | Acceder a una función privada produce `E0603`.

```rust
// fig06_02.rs
mod config {
    fn ruta_por_omision() -> &'static str {
        "servicios.yaml"
    }
}

fn main() {
    println!("{}", config::ruta_por_omision());
}
```

```bash
$ rustc --edition 2024 fig06_02.rs
error[E0603]: function `ruta_por_omision` is private
 --> fig06_02.rs:9:28
  |
9 |     println!("{}", config::ruta_por_omision());
  |                            ^^^^^^^^^^^^^^^^ private function
  |
note: the function `ruta_por_omision` is defined here
 --> fig06_02.rs:3:5
  |
3 |     fn ruta_por_omision() -> &'static str {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

El arreglo mecánico sería escribir `pub fn ruta_por_omision`. Antes de hacerlo, pregunta si la ruta por omisión debe ser parte del contrato de `config`. Si otro módulo de verdad necesita consultarla, puede ser una función pública razonable. Si solo quieres que `main` cargue el archivo normal, quizá convenga que `config` ofrezca una función pública de más alto nivel y conserve esa cadena como detalle privado.

El módulo `modelo` del `revisor` muestra una API pública seleccionada. `Servicio` es público porque la configuración, las pruebas de integración y otros módulos necesitan construirlo. Sus campos son públicos porque el programa necesita leer y modificar los datos declarados. La constante de timeout por omisión, en cambio, permanece privada: quien usa `Servicio::new` obtiene la regla sin depender de cómo está almacenada.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;

fn timeout_por_omision() -> u64 {
    TIMEOUT_POR_OMISION_MS
}

impl Servicio {
    /// Un servicio con el tiempo límite por omisión (5 segundos).
    pub fn new(nombre: &str, url: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            url: url.to_string(),
            timeout_ms: TIMEOUT_POR_OMISION_MS,
        }
    }
}
```

Observa la diferencia entre exponer una constante y exponer una función. `UMBRAL_LENTO_MS` es una regla que otros módulos sí necesitan para clasificar respuestas. `timeout_por_omision` solo existe para que `serde` pueda llamar el valor por omisión dentro del modelo. Publicarlo no da una capacidad útil al consumidor y sí amplía la superficie que habría que mantener.

### Pruebas unitarias: una propiedad pequeña, cerca del código

Una prueba unitaria comprueba una unidad de comportamiento en su propio módulo. Rust las escribe normalmente dentro de un módulo `tests` marcado con `#[cfg(test)]`. Ese atributo indica que el módulo se compila únicamente al construir las pruebas. El binario de producción no carga esas funciones ni sus auxiliares.

`use super::*` importa en `tests` los nombres del módulo padre. Eso permite probar detalles privados deliberadamente. No es una trampa contra la visibilidad: la prueba vive como descendiente del mismo módulo y está comprobando la implementación interna. Una prueba de integración tendrá otra restricción, porque representa a un consumidor externo.

Las aserciones principales son `assert!`, para una condición booleana; `assert_eq!`, para comparar esperado y obtenido; y `assert_ne!`, para afirmar que dos valores no son iguales. Todas aceptan un mensaje adicional con formato. `matches!` es especialmente útil con enums: permite comprobar la variante y, si hace falta, una condición sobre los datos que lleva.

**Fig. 6.3** | Pruebas unitarias, una aserción de enum y un pánico esperado.

```rust
// fig06_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Falla(String),
}

fn resumen(e: &Estado) -> String {
    match e {
        Estado::Ok { codigo, ms } => format!("OK {codigo} en {ms}ms"),
        Estado::Falla(msg) => format!("FALLA: {msg}"),
    }
}

fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("dividir por cero");
    }
    a / b
}

fn main() {
    println!("{}", resumen(&Estado::Ok { codigo: 200, ms: 100 }));
    println!("{}", dividir(10, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estado_ok_con_200() {
        let e = Estado::Ok { codigo: 200, ms: 100 };
        assert!(matches!(e, Estado::Ok { .. }));
    }

    #[test]
    fn falla_sin_codigo() {
        assert_eq!(resumen(&Estado::Falla("x".into())), "FALLA: x");
    }

    #[test]
    #[should_panic(expected = "dividir por cero")]
    fn panico_esperado() {
        dividir(1, 0);
    }
}
```

```bash
$ rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1
running 3 tests
test tests::estado_ok_con_200 ... ok
test tests::falla_sin_codigo ... ok
test tests::panico_esperado - should panic ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`#[should_panic]` no significa que los pánicos sean una forma normal de manejar datos inválidos. En el `revisor`, una URL incorrecta debe terminar como `Result` y un mensaje para quien invocó el programa, no como un pánico. La prueba de pánico sirve cuando un contrato deliberadamente detiene la ejecución ante un invariante roto. El argumento `expected` importa: confirma que el pánico provino de la razón esperada y no de otra falla accidental.

Una prueba debe describir comportamiento, no implementación incidental. El nombre `falla_sin_codigo` comunica una regla del dominio: una falla se muestra sin código HTTP. Un nombre como `prueba_resumen_2` solo dice que alguien escribió una prueba. Cuando falle dentro de meses, el nombre será la primera pista para entender qué decisión del programa cambió.

En `config.rs`, las pruebas unitarias no hacen solicitudes HTTP ni ejecutan el binario. Construyen datos pequeños y llaman `validar`, que es la unidad responsable de verificar la lista. Esto mantiene la suite rápida y hace que cada falla señale una regla concreta.

<!-- verificar:fragmento -->
```rust
#[test]
fn validar_rechaza_nombre_repetido() {
    let v = vec![
        Servicio::new("a", concat!("http", "://x")),
        Servicio::new("a", concat!("http", "://y")),
    ];
    let err = validar(&v).unwrap_err().to_string();
    assert!(err.contains("repetido"), "mensaje: {err}");
}

#[test]
fn validar_rechaza_url_sin_esquema() {
    let v = vec![Servicio::new("a", "localhost:80")];
    assert!(validar(&v).is_err());
}

#[test]
fn validar_rechaza_timeout_cero() {
    let mut s = Servicio::new("a", concat!("http", "://x"));
    s.timeout_ms = 0;
    assert!(validar(&[s]).is_err());
}
```

La primera prueba inspecciona parte del mensaje porque aquí el texto es parte de la experiencia de quien corrige el YAML. Las otras solo verifican que exista un error. No todas las pruebas tienen que comparar cadenas completas. Compara el detalle exacto cuando sea contrato público; para errores internos, comprobar el tipo, una condición o la existencia del error suele producir pruebas menos frágiles.

### Pruebas de integración: la biblioteca vista desde fuera

Cargo reconoce `tests/` como el lugar de pruebas de integración. Cada archivo Rust directamente dentro de esa carpeta se compila como un crate distinto. Por eso no puede usar funciones privadas, ni importar el módulo interno `tests` de la biblioteca, ni asumir detalles de archivos. Solo puede usar lo que la biblioteca exporta con `pub`.

Esta limitación es útil. Una API puede tener excelentes pruebas unitarias y seguir ser incómoda o insuficiente para quien intenta usarla desde fuera. Las pruebas de integración encuentran ese problema porque atraviesan el mismo límite que atravesaría otro binario. Si necesitas romper la encapsulación para escribirlas, primero revisa si falta una operación pública razonable; no conviertas todo en `pub` como reacción automática.

El `revisor` tiene un archivo `tests/integracion.rs`. Importa los tipos y funciones que un consumidor público necesita: `Estado`, `Servicio`, `revisar` y `revisar_todos`. No importa funciones privadas que construyen solicitudes ni detalles de `reqwest`.

<!-- verificar:fragmento -->
```rust
use revisor::modelo::{Estado, Servicio};
use revisor::revisar::{revisar, revisar_todos};

fn servicio(nombre: &str, direccion: &str, ruta: &str, timeout_ms: u64) -> Servicio {
    Servicio {
        nombre: nombre.to_string(),
        url: ["http:", "//", direccion, ruta].concat(),
        timeout_ms,
    }
}
```

El helper `servicio` pertenece a la prueba, no a la biblioteca, porque solo existe para hacer legibles los casos de prueba. Es una distinción sana: no promociones una función a producción únicamente porque dos pruebas la repiten. La biblioteca debe contener capacidades del programa; la suite puede contener pequeñas herramientas para preparar escenarios.

La prueba siguiente levanta un servidor HTTP local definido en `tests/comun/mod.rs`, llama a la API pública y comprueba la variante resultante. No depende de un servicio real en internet, de una cuenta ni de una hora específica. Eso evita que una falla de red convierta una prueba determinista en una alarma falsa.

<!-- verificar:extracto:tests/integracion.rs -->
```rust
#[tokio::test]
async fn un_500_es_falla_con_su_codigo() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let e = revisar(&cliente, &servicio("mal", &d, "/error", 2000)).await;
    assert!(
        matches!(&e, Estado::Falla { motivo, .. } if motivo == "codigo 500"),
        "estado: {e:?}"
    );
}
```

`#[tokio::test]` aparece porque la función `revisar` es asíncrona. La lección 7 profundiza en qué significa esperar un futuro y cómo funciona el runtime. Aquí importa reconocer la frontera: la prueba de integración usa la misma API asíncrona que usará el binario, pero sustituye internet por un servidor local controlado.

Además de pruebas de integración, el proyecto tiene pruebas de binario. Éstas ejecutan el `revisor` compilado, le pasan un YAML temporal y revisan `stdout`, `stderr` y el código de salida. Son más lentas y más amplias que una unitaria, así que no sustituyen a las otras; comprueban la última frontera, donde argumentos, configuración, reportes y salida del proceso se encuentran.

### `cargo test`: construir, seleccionar y leer resultados

`cargo test` descubre las pruebas unitarias, las de integración, las de binarios y las de documentación; compila los targets necesarios y ejecuta cada conjunto. Es más que una abreviatura de `rustc --test`: Cargo conoce las dependencias y construye cada crate con las rutas correctas.

Los comandos que usarás con más frecuencia son éstos:

```bash
cargo test
cargo test validar_rechaza_nombre_repetido
cargo test --test integracion
cargo test -- --nocapture
cargo test --release
```

El primer comando corre todo. El segundo filtra por una parte del nombre de prueba; es útil para trabajar en una sola regla sin esperar la suite entera. El tercero selecciona específicamente el archivo de integración llamado `integracion`. El `--` separa las opciones de Cargo de las opciones del ejecutor de pruebas: `--nocapture` permite ver los `println!` de una prueba que pasa, algo útil para diagnóstico temporal, no como sustituto de una aserción. `--release` compila con optimizaciones; úsalo cuando el comportamiento dependa realmente del perfil o cuando estés midiendo rendimiento, no como modo diario.

Las pruebas pueden correr en paralelo. Eso es correcto si cada una crea sus propios datos y no depende del orden de ejecución. Si estás diagnosticando salida o una prueba comparte un recurso que todavía no puedes aislar, usa:

```bash
cargo test -- --test-threads=1
```

No conviertas esa bandera en costumbre. Una suite que solo funciona en serie puede esconder un estado global o archivos temporales con nombres que chocan. En el `revisor`, los servidores de prueba piden al sistema un puerto libre y cada caso usa sus propios datos; eso permite ejecutar pruebas sin depender de un orden particular.

La trampa más simple es que `cargo test` puede terminar correctamente sin ejecutar una prueba relevante. Un filtro mal escrito puede producir una salida con pruebas filtradas; un crate puede no tener ningún `#[test]`; y un archivo colocado fuera de `tests/` puede no ser una integración. Lee siempre las líneas `running N tests` y `test result`. El código de salida cero significa que el ejecutor no encontró una falla, no que tu intención quedó comprobada.

El proyecto mantiene la lógica en la biblioteca y el arranque en el binario. El binario importa la API pública como cualquier otro consumidor interno. Esta separación es la razón por la que las pruebas de integración pueden importar `revisor` con el mismo nombre.

<!-- verificar:extracto:src/main.rs -->
```rust
use std::process::ExitCode;

use revisor::{config, reporte, revisar};

use clap::Parser;
```

La ruta `revisor::{config, reporte, revisar}` no usa `crate::` porque `main.rs` es otro crate dentro del mismo paquete. Desde la perspectiva del binario, `revisor` es la biblioteca declarada por `src/lib.rs`. Es una diferencia pequeña de sintaxis con una consecuencia importante de diseño: el binario no tiene privilegios para llegar a detalles privados de la biblioteca.

### Dependencias, versiones y el trabajo de `cargo`

`Cargo.toml` es el manifiesto declarativo del paquete. Dice cómo se llama, qué edición usa, qué dependencias directas necesita y qué perfiles de construcción existen. `Cargo.lock` registra la resolución concreta: las versiones exactas de dependencias directas y transitivas que Cargo eligió cuando construyó el proyecto.

El `revisor` no depende solo de la biblioteca estándar. Eso es deliberado: YAML, HTTP asíncrono, JSON y una línea de comandos completa viven en crates especializadas. Las dependencias declaradas son las que el proyecto realmente usa.

<!-- verificar:extracto:Cargo.toml -->
```toml
[dependencies]
anyhow = "1.0.104"
clap = { version = "4.6.7", features = ["derive"] }
futures = "0.3.34"
reqwest = { version = "0.13.5", features = ["json"] }
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
yaml_serde = "0.10.7"
tokio = { version = "1.53.1", features = ["full"] }
```

Una nota sobre una de esas líneas. Hasta 2024, el crate más usado para leer YAML con `serde` era `serde_yaml`. Su autor, David Tolnay, dejó de mantenerlo: su última versión es la `0.9.34+deprecated`, de marzo de 2024, y crates.io la marca como obsoleta. Todavía compila y funciona, pero ya no recibe correcciones ni mejoras, así que no conviene empezar un proyecto nuevo con ella. El `revisor` usa `yaml_serde`, una continuación publicada por la organización de YAML en GitHub: su repositorio la presenta como la bifurcación mantenida de `serde_yaml` y promete la misma interfaz. Por eso el cambio casi no toca el código: lo que sabes de `serde_yaml::from_str` sirve igual con `yaml_serde::from_str`. Existen otras bifurcaciones y alternativas; antes de elegir una, mira la fecha de su última versión y si su repositorio sigue recibiendo cambios. (Datos consultados en crates.io el 2 de octubre de 2026.)

Una versión como `"1.0.104"` no fija por sí sola cada dígito para siempre. En Cargo, esa especificación usa compatibilidad semántica con operador caret implícito: permite actualizaciones compatibles dentro de la misma versión mayor. `Cargo.lock` es lo que hace repetible la compilación concreta del binario. Por eso el lockfile del `revisor` debe ir al repositorio: una persona que clone la aplicación debe resolver las mismas versiones conocidas, no una combinación nueva que hoy parezca compatible.

Para una biblioteca publicada, la respuesta es menos tajante. `cargo new` registra el `Cargo.lock` en el repositorio por omisión, y las preguntas frecuentes de Cargo (el [Cargo FAQ](https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control)) dicen que versionarlo o no depende de lo que necesite tu paquete. Versionarlo da compilaciones repetibles: ayuda a encontrar con `git bisect` qué cambio introdujo un error, a que la integración continua falle solo por commits nuevos y no por una dependencia que cambió afuera, y a verificar con versiones conocidas cosas como la versión mínima de Rust o el texto exacto de los mensajes de error. Pero ese archivo no protege a quien usa tu biblioteca: los consumidores resuelven las dependencias con lo que declara tu `Cargo.toml` y con su propio `Cargo.lock`, y `cargo install` ignora por omisión el `Cargo.lock` del paquete y elige las versiones compatibles más recientes, a menos que le pases `--locked`. En resumen: una aplicación como el `revisor` conviene versionarla siempre, porque es el producto final que quieres reproducir; para una biblioteca, decídelo según qué quieres garantizar, y si no la versionas, prueba de vez en cuando con las dependencias más nuevas.

Agrega una dependencia con Cargo en lugar de escribir a mano una línea que no entiendes:

```bash
cargo add serde --features derive
cargo add tokio --features full
cargo tree
cargo update
```

`cargo add` actualiza el manifiesto y resuelve el lockfile. Las características, o *features*, activan partes opcionales de un crate. `serde` necesita `derive` para que `#[derive(Serialize, Deserialize)]` exista; `tokio` necesita capacidades de runtime, red y macros para el programa actual. No habilites `full` por reflejo en un proyecto nuevo si solo necesitas una parte pequeña; aquí es una decisión consciente del curso para que el revisor use las capacidades que enseña.

`cargo tree` muestra el árbol completo. Es la forma de descubrir dependencias transitivas: crates que no agregaste directamente, pero que llegaron porque otra dependencia las necesita. No es necesariamente una señal de problema. Sí es una herramienta para responder “¿quién trae esta versión?”, “¿por qué se compila tanto código?” o “¿por qué hay dos versiones de esta crate?”.

`cargo update` actualiza dentro de las restricciones que escribiste en `Cargo.toml`. No equivale a “instalar la última versión de todo” sin límites. Antes de actualizar un proyecto estable, revisa qué cambió en el lockfile, corre pruebas y lee notas de versión cuando una dependencia central cambie. La versión declarada define el rango aceptable; el lockfile registra la decisión tomada.

Durante el desarrollo, `cargo check` suele ser más rápido que `cargo build` porque verifica tipos y préstamos sin generar el ejecutable final. No reemplaza pruebas, pero reduce el tiempo de retroalimentación mientras editas una función. Para mantener calidad en el proyecto completo, usa esta secuencia:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo fmt --check` confirma formato sin modificar archivos. `cargo clippy --all-targets -- -D warnings` revisa biblioteca, binario y pruebas, y convierte sus avisos en fallas para que el proyecto no acumule deuda conocida. `cargo test` comprueba comportamiento. Son tres señales distintas: formato consistente, uso idiomático y comportamiento esperado.

## El error que vas a ver

### `E0603`: el nombre existe, pero no forma parte de la API

`E0603` aparece cuando Rust encontró el item que nombraste, pero la ruta intenta cruzar una frontera privada. El diagnóstico de la figura 6.2 da tres pistas: señala el uso ilegal, dice que la función es privada y apunta a dónde fue declarada. Eso es distinto de un error de escritura como “no se encontró esta función”; aquí Rust sí sabe qué función querías usar.

La corrección depende del diseño. Si la función debe ser parte de la interfaz, declárala `pub`. Si solo debe servir a un módulo hermano, considera mover la operación a un módulo dueño más apropiado o exponer una función pública de nivel superior. Si necesitas limitarla al crate, `pub(crate)` expresa mejor que nadie fuera del paquete debe depender de ella.

En el `revisor`, los auxiliares de `reporte.rs`, como `etiqueta`, `tiempo` y `detalle`, siguen privados. La API pública es `tabla` y `json`, porque son las operaciones que el binario y cualquier consumidor pueden pedir. La elección impide que pruebas de integración o código futuro dependan de una etiqueta interna y congelen una decisión de formato accidental.

### Una prueba roja no es un error de compilación

Cuando falla una aserción, Cargo compila correctamente y después el ejecutor marca la prueba como fallida. No habrá un código `EXXXX`, porque no es una violación de reglas estáticas del compilador. Verás el nombre de la prueba, el valor izquierdo y derecho si usaste `assert_eq!`, y cualquier mensaje extra que agregaste.

Ésta es una diferencia importante de diagnóstico. Los errores de `rustc` te dicen que el programa no puede construirse bajo las reglas del lenguaje. Una prueba roja te dice que el programa se construyó, pero rompió el comportamiento que declaraste. No arregles una prueba roja quitando la aserción ni cambiando el valor esperado sin revisar qué contrato debía sostenerse.

Provoca una falla a propósito una vez. En la prueba `falla_sin_codigo`, cambia temporalmente el texto esperado a `"OK: x"` y corre el filtro correspondiente. Debes ver la prueba ponerse roja. Después restaura el comportamiento correcto. Una prueba que nunca has visto fallar puede estar cubriendo una rama distinta de la que crees, o puede afirmar algo demasiado débil para detectar una regresión.

## Lo que se hace mal

### Hacer todo `pub`

Abrir cada struct, campo y función suele empezar como una forma rápida de vencer `E0603`. El resultado es una biblioteca sin fronteras: cualquier módulo puede apoyarse en detalles internos y cada cambio exige revisar muchas más llamadas de las necesarias. Publica operaciones que representen capacidades del dominio, no cada paso auxiliar con que las implementas.

La alternativa práctica no es adivinar la API perfecta desde el primer día. Mantén privado lo que todavía no tiene un consumidor claro. Cuando otro módulo necesite una operación, abre la interfaz mínima y deja que ese uso real guíe el diseño.

### Organizar por nombres vagos como `utils` o `helpers`

Una carpeta llamada `utils` no describe una responsabilidad; describe que alguien no supo dónde poner algo. Con el tiempo acumula conversión de texto, acceso a archivos, formato, HTTP y funciones que nadie se atreve a mover. Buscar código se vuelve más lento y la dependencia entre módulos se vuelve arbitraria.

En el `revisor`, una regla de YAML vive en `config`, una traducción de estado vive en `reporte` y el vocabulario del dominio vive en `modelo`. Si una función no cabe en ningún módulo, primero pregunta si falta un concepto con nombre propio. A menudo el nuevo nombre revela una responsabilidad que estaba mezclada.

### Probar solo el camino sano

Una prueba que comprueba `200 OK` es necesaria, pero no basta para un revisor de servicios. También deben estar cubiertos un HTTP 500, una URL inválida, un archivo faltante, un tiempo de espera, una lista vacía y un formato desconocido. Los errores no son excepciones improbables en este dominio: son parte de lo que el programa existe para reportar.

No conviertas cada fallo externo en una prueba de red real. El `revisor` usa un servidor local de mentira para reproducir respuestas conocidas. Así prueba el comportamiento propio, no la disponibilidad de un servicio ajeno.

### Usar `unwrap()` para escribir pruebas más cortas

`unwrap()` es razonable para preparar datos que la propia prueba controla, como YAML literal que debe ser válido. Si ese YAML falla, la prueba está mal construida y detenerse es correcto. No lo uses sobre el resultado que estás intentando probar. Si quieres demostrar que `validar` rechaza una entrada, usa `is_err`, `unwrap_err` o `matches!` según el contrato.

La regla es distinguir preparación de verificación. En la preparación, un `expect("el YAML de la prueba es válido")` da contexto útil. En la verificación, una aserción expresa exactamente la propiedad que quieres sostener.

### Confiar en el código de salida de `cargo test` sin leer el conteo

Un filtro sin coincidencias puede devolver éxito porque no hubo pruebas que fallaran. Un crate nuevo puede compilar sin pruebas. Una integración mal ubicada puede no ser descubierta. Lee `running N tests`, los nombres que aparecen y el resumen final. El resultado útil no es solo “salió cero”; es “corrió la prueba que esperaba y pasó”.

### Actualizar dependencias sin revisar el lockfile

`cargo update` puede cambiar varias dependencias transitivas aunque solo hayas pedido una actualización. Eso no lo vuelve peligroso por sí mismo, pero sí exige revisión. Mira el cambio en `Cargo.lock`, entiende qué crates se actualizaron y corre la suite completa. Una versión compatible en teoría puede revelar una suposición frágil o cambiar tiempos de compilación de manera importante.

## Ejercicios

### Ejercicio 1 — Divide un reporte sin abrir de más

Crea un programa con un módulo `reporte`. Debe exponer una función pública `resumen(nombre, sano)` que devuelva una `String` con el nombre y la etiqueta `OK` o `FALLA`. La función que decide la etiqueta debe quedarse privada. Desde `main`, imprime dos líneas: una sana y una fallida.

### Ejercicio 2 — Prueba todas las variantes del estado

Escribe una función `es_sano(&Estado) -> bool` para las cuatro variantes del `Estado` del revisor: `Ok`, `Lento`, `Falla` y `NoIntentado`. Agrega una prueba por variante. Usa `assert!` o `assert!(!...)` y nombra cada prueba según la regla que comprueba.

### Ejercicio 3 — Una integración que no conoce detalles internos

En `programas/revisor`, lee `tests/integracion.rs`. Agrega una prueba de integración que use exclusivamente `revisor::modelo` y `revisor::revisar`. Debe usar el servidor local compartido y comprobar que `revisar_todos` devuelve el mismo número de estados que servicios, incluso cuando uno recibe HTTP 500. Escríbela antes de leer las pruebas que ya trae `tests/integracion.rs`, y luego compara: ¿qué comprueba la tuya que las otras no?

### Ejercicio 4 — Haz una prueba roja y vuelve a dejarla verde

Elige una prueba existente de `config.rs` o `reporte.rs`. Cambia temporalmente una expectativa para que falle, ejecuta únicamente esa prueba con `cargo test nombre_de_la_prueba`, lee el diagnóstico y restaura el comportamiento correcto. Finalmente ejecuta `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` y `cargo test`.

## Soluciones

### Solución 1

<!-- verificar:fragmento -->
```rust
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn resumen(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::resumen("catalogo", true));
    println!("{}", reporte::resumen("pagos", false));
}
```

`etiqueta` no necesita `pub` porque solo `resumen` la usa. La función pública entrega el resultado que necesita `main`, no el detalle intermedio.

### Solución 2

<!-- verificar:fragmento -->
```rust
#[derive(Debug)]
enum Estado {
    Ok,
    Lento,
    Falla,
    NoIntentado,
}

fn es_sano(estado: &Estado) -> bool {
    matches!(estado, Estado::Ok | Estado::Lento)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_es_sano() {
        assert!(es_sano(&Estado::Ok));
    }

    #[test]
    fn lento_es_sano() {
        assert!(es_sano(&Estado::Lento));
    }

    #[test]
    fn falla_no_es_sana() {
        assert!(!es_sano(&Estado::Falla));
    }

    #[test]
    fn no_intentado_no_es_sano() {
        assert!(!es_sano(&Estado::NoIntentado));
    }
}
```

Las cuatro pruebas no son redundantes. La función contiene dos grupos de variantes y cada una expresa una decisión del dominio. Si alguien cambia `matches!` de forma incompleta, al menos una prueba identifica qué estado perdió su significado.

### Solución 3

<!-- verificar:fragmento -->
```rust
#[tokio::test]
async fn revisar_todos_conserva_un_estado_por_servicio() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let servicios = vec![
        servicio("bien", &d, "/ok", 2000),
        servicio("mal", &d, "/error", 2000),
    ];

    let estados = revisar_todos(&cliente, &servicios, 2).await;

    assert_eq!(estados.len(), servicios.len());
    assert!(estados[0].esta_bien());
    assert!(!estados[1].esta_bien());
}
```

La prueba usa solo tipos y funciones públicas del `revisor`. El helper local construye los servicios; el servidor compartido controla las respuestas. No requiere abrir ninguna función privada del cliente HTTP.

### Solución 4

Ejecuta primero una prueba concreta, por ejemplo:

```bash
cargo test validar_rechaza_timeout_cero
```

Cambia temporalmente `assert!(validar(&[s]).is_err())` por `assert!(validar(&[s]).is_ok())`. La prueba debe fallar. Restaura `is_err()` y termina con:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

La solución no es conservar el cambio que hace pasar la prueba; es comprobar que la suite detecta una modificación que rompe la regla y después recuperar la regla correcta.

## Cómo sé que lo logré

- `rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1` imprime `running 3 tests` y termina con `3 passed; 0 failed`.
- `cargo test validar_rechaza_nombre_repetido` termina con la prueba `config::tests::validar_rechaza_nombre_repetido ... ok`.
- `cargo test --test integracion` ejecuta las pruebas que importan únicamente la API pública de la biblioteca.
- `cargo fmt --check` termina sin cambios pendientes de formato.
- `cargo clippy --all-targets -- -D warnings` termina sin avisos.
- `cargo test` termina con resultados `ok` para biblioteca, binario y pruebas de integración.
- Puedes explicar por qué `main.rs` importa `revisor::{config, reporte, revisar}` y no detalles privados de `src/lib.rs`.

## Para leer más

- [The Rust Programming Language, capítulo 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) — consultado el 2 de octubre de 2026.
- [The Rust Programming Language, capítulo 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html) — consultado el 2 de octubre de 2026.
- [The Rust Programming Language, capítulo 14: More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) — consultado el 2 de octubre de 2026.
- [Referencia oficial de Cargo: especificar dependencias](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) — consultado el 2 de octubre de 2026.
- [Pruebas que sí atrapan errores: más allá de la cobertura y del quality gate en verde](https://www.habil.mx/es/blog/pruebas-que-atrapan-errores-cobertura-quality-gate/) — artículo sobre por qué un tablero en verde o una cobertura alta no bastan y por qué conviene contar las pruebas del informe y no del código de salida; consultado el 6 de octubre de 2026.
