# Semana 3 — Structs, enums y `match`

**The Book, capítulos 5 y 6.** Rustlings: `structs`, `enums`, `options`.

**Aquí Rust se separa de Go de verdad.** Los enums de Rust no son constantes con nombre: **son tipos que
llevan datos**, y con `match` el compilador te obliga a cubrir todos los casos.

## Structs e `impl`

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

**`#[derive(...)]`** le pide al compilador que genere código: `Debug` para poder imprimir con `{:?}`,
`Clone` para copiar. En Go eso se escribe a mano o se saca con reflexión; aquí es una línea.

⚠️ **`&self` vs `self` vs `&mut self`** es la misma distinción de la semana 2 aplicada a métodos: prestar
para leer, prestar para modificar, o **consumir** el objeto. `self` sin `&` es raro y deliberado: se usa
cuando el método transforma el valor en otra cosa y el original ya no debe existir.

## Enums con datos: lo que Go no tiene

    enum Estado {
        Ok { codigo: u16, ms: u64 },
        Lento { codigo: u16, ms: u64 },
        Falla(String),                      // lleva el mensaje dentro
        NoIntentado,
    }

**Cada variante puede llevar datos distintos.** Eso permite modelar «o esto, o aquello, y nunca los dos»
de forma que el compilador lo verifique. En Go se hace con un struct que tiene campos que a veces son
válidos y a veces no, y nadie te avisa si lees el equivocado.

## `match`: exhaustivo por obligación

    let texto = match estado {
        Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
        Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
        Estado::Falla(msg)           => format!("FALLA: {msg}"),
        Estado::NoIntentado          => "sin revisar".to_string(),
    };

🔑 **Si olvidas una variante, NO COMPILA.** Y si mañana agregas `Estado::Rechazado`, el compilador te
lleva a **todos** los `match` que hay que actualizar. Eso es refactorizar con red.

    error[E0004]: non-exhaustive patterns: `Estado::NoIntentado` not covered

## `Option`: el adiós a `nil`

**En Rust no existe `nil`.** Lo que puede faltar se declara:

    enum Option<T> { Some(T), None }        // está en el estándar

    let quizas: Option<u16> = Some(200);

    match quizas {
        Some(c) => println!("código {c}"),
        None    => println!("sin respuesta"),
    }

    if let Some(c) = quizas { println!("código {c}"); }     // cuando solo importa un caso
    let c = quizas.unwrap_or(0);                            // valor por omisión

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
