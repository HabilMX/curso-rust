# Semana 5 — Traits, genéricos y lifetimes

**The Book, capítulo 10.** Rustlings: `generics`, `traits`, `lifetimes`.

**Los traits son las interfaces de Go, con una diferencia que cambia todo.**

## Traits

**Fig. 5.1** | Un trait con método por omisión, y dos tipos que lo cumplen.

```rust
// fig05_01.rs
use std::fmt;

struct Servicio {
    nombre: String,
}

enum Estado {
    Ok { ms: u64 },
    Falla(String),
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Estado::Ok { ms } => write!(f, "OK en {ms}ms"),
            Estado::Falla(motivo) => write!(f, "FALLA: {motivo}"),
        }
    }
}

trait Revisor {
    fn revisar(&self, s: &Servicio) -> Estado;

    fn nombre(&self) -> String {          // 🔑 los traits pueden traer implementación por omisión
        "revisor".to_string()
    }
}

struct RevisorHttp { timeout_ms: u64 }

impl Revisor for RevisorHttp {            // 🔴 aquí está la diferencia: es EXPLÍCITO
    fn revisar(&self, s: &Servicio) -> Estado {
        Estado::Falla(format!("{}: sin red en este ejemplo (límite {} ms)", s.nombre, self.timeout_ms))
    }
}

struct RevisorFalso;

impl Revisor for RevisorFalso {
    fn revisar(&self, _s: &Servicio) -> Estado {
        Estado::Ok { ms: 1 }
    }
    fn nombre(&self) -> String {
        "falso".to_string()
    }
}

fn main() {
    let s = Servicio { nombre: "catalogo".to_string() };
    let http = RevisorHttp { timeout_ms: 2000 };
    println!("{} -> {}", http.nombre(), http.revisar(&s));
    println!("{} -> {}", RevisorFalso.nombre(), RevisorFalso.revisar(&s));
}
```

```bash
$ rustc --edition 2024 fig05_01.rs && ./fig05_01
revisor -> FALLA: catalogo: sin red en este ejemplo (límite 2000 ms)
falso -> OK en 1ms
```

| | Go | Rust |
|---|---|---|
| Cumplir una interfaz | **implícito**: si tienes los métodos, ya cumples | **explícito**: hay que escribir `impl X for Y` |
| Métodos por omisión | no existen | **sí**, y es muy usado |
| Sobre tipos ajenos | no puedes | **sí**, puedes implementar *tu* trait para `String` |

⚠️ **La contra de lo explícito:** más escritura. **La ventaja:** cumplir una interfaz por accidente es
imposible, y al leer el código sabes exactamente qué contratos cumple un tipo.

## Genéricos, con restricciones

**Fig. 5.2** | Una función genérica con restricciones.

```rust
// fig05_02.rs
use std::fmt;

struct Servicio {
    nombre: String,
}

enum Estado {
    Ok { ms: u64 },
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Estado::Ok { ms } => write!(f, "OK en {ms}ms"),
        }
    }
}

trait Revisor {
    fn revisar(&self, s: &Servicio) -> Estado;
}

struct RevisorFalso;

impl Revisor for RevisorFalso {
    fn revisar(&self, s: &Servicio) -> Estado {
        Estado::Ok { ms: s.nombre.len() as u64 }
    }
}

fn revisar_todos<R: Revisor>(r: &R, servicios: &[Servicio]) -> Vec<Estado> {
    servicios.iter().map(|s| r.revisar(s)).collect()
}

fn imprimir<T: std::fmt::Display + Clone>(x: T) { println!("{x}"); }      // varias restricciones

fn main() {
    let servicios = vec![
        Servicio { nombre: "catalogo".to_string() },
        Servicio { nombre: "pagos".to_string() },
    ];
    for estado in revisar_todos(&RevisorFalso, &servicios) {
        imprimir(estado.to_string());
    }
}
```

```bash
$ rustc --edition 2024 fig05_02.rs && ./fig05_02
OK en 8ms
OK en 5ms
```

🔑 **`<R: Revisor>` se resuelve en compilación** (monomorfización): el compilador genera una versión por
cada tipo concreto. Sin costo en ejecución. Cuando necesitas decidir en tiempo de ejecución, se usa
`Box<dyn Revisor>`, que sí tiene una indirección — como las interfaces de Go, que **siempre** la tienen.

## Lifetimes: el `'a` que asusta y no es para tanto

<!-- verificar:fragmento -->
```rust
fn primera<'a>(s: &'a str) -> &'a str { }
```

**No es magia ni gestión de memoria manual.** Es una anotación que dice: *«el valor que devuelvo vive
tanto como el que recibí»*. El compilador lo necesita cuando devuelves una referencia y hay más de una
entrada posible:

**Fig. 5.3** | Un lifetime que une la salida con las dos entradas.

```rust
// fig05_03.rs
fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", mas_largo("catalogo", "pagos"));
}
```

```bash
$ rustc --edition 2024 fig05_03.rs && ./fig05_03
catalogo
```

Sin el `'a`, el compilador no puede saber si el resultado apunta a `a` o a `b`, y por tanto no sabe
cuánto debe vivir.

🔑 **Y la buena noticia: casi nunca los escribes.** El compilador los infiere en la gran mayoría de los
casos. Solo aparecen cuando devuelves referencias o las guardas en structs. **Si te encuentras peleando
con lifetimes en la semana 5, la salida suele ser devolver un `String` en vez de un `&str`** — pagas una
copia y sigues. Optimizar eso es para después.

## El ejercicio de la semana

1. Capítulo 10 completo. Rustlings: `generics`, `traits`, `lifetimes`.
2. Define el trait `Revisor` y dos implementaciones: `RevisorHttp` y `RevisorFalso`.
3. Escribe `revisar_todos<R: Revisor>` genérica. **Es la misma función de la semana 2 de Go: compara las
   dos y anota la diferencia.**
4. Agrega un método por omisión al trait y comprueba que una implementación puede no escribirlo.
5. Implementa `std::fmt::Display` para `Estado` — así puedes imprimirlo con `{}` en vez de `{:?}`.
6. Provoca a propósito un error de lifetime devolviendo una referencia a algo local. **Lee el error
   completo** (`borrowed value does not live long enough`) y anótalo.

## Cómo sé que lo logré

- [ ] `revisar_todos` funciona con los dos revisores sin cambiar nada
- [ ] Sé la diferencia entre `<R: Revisor>` y `Box<dyn Revisor>`, y qué cuesta cada uno
- [ ] Puedo explicar qué dice `'a` en una firma
- [ ] Rustlings de las tres secciones completo
