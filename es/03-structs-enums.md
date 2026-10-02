# Semana 3 — Structs, enums y `match`

**The Book, capítulos 5 y 6.** Rustlings: `structs`, `enums`, `options`.

**Aquí Rust se separa de Go de verdad.** Los enums de Rust no son constantes con nombre: **son tipos que
llevan datos**, y con `match` el compilador te obliga a cubrir todos los casos.

## Structs e `impl`

**Fig. 3.1** | Un struct con sus métodos.

```rust
// fig03_01.rs
#[derive(Debug, Clone)]          // el compilador te escribe esos comportamientos
struct Servicio {
    nombre: String,
    url: String,
    timeout_ms: u64,
}

impl Servicio {
    fn new(nombre: &str, url: &str) -> Self {     // no hay constructores: es convención
        Self { nombre: nombre.to_string(), url: url.to_string(), timeout_ms: 5000 }
    }
    fn etiqueta(&self) -> String {                // &self = presta, no consume
        format!("{} ({})", self.nombre, self.url)
    }
}

fn main() {
    let s = Servicio::new("catalogo", "http://localhost:8090/ok");
    println!("{}", s.etiqueta());
    println!("{:?}", s.clone());
    println!("timeout: {} ms", s.timeout_ms);
}
```

```bash
$ rustc --edition 2024 fig03_01.rs && ./fig03_01
catalogo (http://localhost:8090/ok)
Servicio { nombre: "catalogo", url: "http://localhost:8090/ok", timeout_ms: 5000 }
timeout: 5000 ms
```

**`#[derive(...)]`** le pide al compilador que genere código: `Debug` para poder imprimir con `{:?}`,
`Clone` para copiar. En Go eso se escribe a mano o se saca con reflexión; aquí es una línea.

⚠️ **`&self` vs `self` vs `&mut self`** es la misma distinción de la semana 2 aplicada a métodos: prestar
para leer, prestar para modificar, o **consumir** el objeto. `self` sin `&` es raro y deliberado: se usa
cuando el método transforma el valor en otra cosa y el original ya no debe existir.

## Enums con datos: lo que Go no tiene

<!-- verificar:fragmento -->
```rust
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),                      // lleva el mensaje dentro
    NoIntentado,
}
```

**Cada variante puede llevar datos distintos.** Eso permite modelar «o esto, o aquello, y nunca los dos»
de forma que el compilador lo verifique. En Go se hace con un struct que tiene campos que a veces son
válidos y a veces no, y nadie te avisa si lees el equivocado.

## `match`: exhaustivo por obligación

**Fig. 3.2** | Un enum con datos y su `match`.

```rust
// fig03_02.rs
#[allow(dead_code)]              // este ejemplo no lee el código de los lentos
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),                      // lleva el mensaje dentro
    NoIntentado,
}

fn main() {
    let estados = [
        Estado::Ok { codigo: 200, ms: 120 },
        Estado::Lento { codigo: 200, ms: 1800 },
        Estado::Falla("no responde".to_string()),
        Estado::NoIntentado,
    ];

    for estado in estados {
        let texto = match estado {
            Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
            Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
            Estado::Falla(msg)           => format!("FALLA: {msg}"),
            Estado::NoIntentado          => "sin revisar".to_string(),
        };
        println!("{texto}");
    }
}
```

```bash
$ rustc --edition 2024 fig03_02.rs && ./fig03_02
OK 200 en 120ms
LENTO 1800ms
FALLA: no responde
sin revisar
```

🔑 **Si olvidas una variante, NO COMPILA.** Y si mañana agregas `Estado::Rechazado`, el compilador te
lleva a **todos** los `match` que hay que actualizar. Eso es refactorizar con red.

**Fig. 3.3** | Si olvidas una variante, no compila.

```rust
// fig03_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}

fn main() {
    let estado = Estado::NoIntentado;
    let texto = match estado {
        Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
        Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
        Estado::Falla(msg)           => format!("FALLA: {msg}"),
    };
    println!("{texto}");
}
```

```bash
$ rustc --edition 2024 fig03_03.rs
error[E0004]: non-exhaustive patterns: `Estado::NoIntentado` not covered
  --> fig03_03.rs:11:23
   |
11 |     let texto = match estado {
   |                       ^^^^^^ pattern `Estado::NoIntentado` not covered
   |
note: `Estado` defined here
  --> fig03_03.rs:2:6
   |
 2 | enum Estado {
   |      ^^^^^^
...
 6 |     NoIntentado,
   |     ----------- not covered
   = note: the matched value is of type `Estado`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
14 ~         Estado::Falla(msg)           => format!("FALLA: {msg}"),
15 ~         Estado::NoIntentado => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

## `Option`: el adiós a `nil`

**En Rust no existe `nil`.** Lo que puede faltar se declara:

<!-- verificar:fragmento -->
```rust
enum Option<T> { Some(T), None }        // está en el estándar
```

**Fig. 3.4** | Consumir un `Option`.

```rust
// fig03_04.rs
fn main() {
    let quizas: Option<u16> = Some(200);

    match quizas {
        Some(c) => println!("código {c}"),
        None    => println!("sin respuesta"),
    }

    if let Some(c) = quizas { println!("código {c}"); }     // cuando solo importa un caso
    let c = quizas.unwrap_or(0);                            // valor por omisión
    println!("{c}");
}
```

```bash
$ rustc --edition 2024 fig03_04.rs && ./fig03_04
código 200
código 200
200
```

🔴 **Aquí está la diferencia que más vale de todo el curso.** En Go, un puntero `nil` compila
perfectamente y explota en ejecución — el `nil pointer dereference` es el panic más común del lenguaje.
En Rust **el compilador no te deja usar un `Option` sin decir qué haces si está vacío.** La categoría
entera de error desaparece.

⚠️ **`.unwrap()` es la trampa:** significa «dame el valor y si está vacío truena el programa». Sirve en
pruebas y prototipos. **En código de verdad es deuda**, y `clippy` te lo dirá.

## El ejercicio de la semana

1. Capítulos 5 y 6. Rustlings: `structs`, `enums`, `options`.
2. Modela `Servicio` y `Estado` como arriba, con `Estado` como **enum**, no struct.
3. Escribe `fn resumen(e: &Estado) -> String` con un `match` exhaustivo.
4. Agrega una variante nueva al enum **y comprueba que el compilador te lleva a cada `match`** que hay
   que arreglar. Ese momento es el que justifica el lenguaje.
5. Escribe algo que devuelva `Option<Servicio>` buscando por nombre en un vector, y consúmelo con
   `match` y con `if let`.

## Cómo sé que lo logré

- [ ] Mi `Estado` es un enum con datos, no un struct con campos opcionales
- [ ] Agregué una variante y vi al compilador señalarme todos los `match`
- [ ] Sé por qué `Option` hace imposible el `nil pointer dereference`
- [ ] Sé por qué `.unwrap()` no va en código de verdad
