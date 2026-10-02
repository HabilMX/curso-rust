# Semana 4 — Colecciones y errores

**The Book, capítulos 8 y 9.** Rustlings: `vecs`, `hashmaps`, `strings`, `error_handling`.

## Las tres colecciones

**Fig. 4.1** | Las colecciones y su acceso seguro.

```rust
// fig04_01.rs
use std::collections::HashMap;

struct Servicio {
    nombre: String,
}

#[derive(Debug)]
enum Estado {
    Ok,
    Falla,
}

fn main() {
    let s = Servicio { nombre: "catalogo".to_string() };
    let mut v: Vec<Servicio> = Vec::new();
    v.push(s);
    let primero = &v[0];                    // 🔴 si no existe: PANIC
    println!("{}", primero.nombre);
    let primero = v.get(0);                 // devuelve Option<&Servicio> ← lo seguro
    println!("{}", primero.is_some());

    let mut m: HashMap<String, Estado> = HashMap::new();
    m.insert("catalogo".to_string(), Estado::Ok);
    println!("{:?}", m.get("catalogo"));
    println!("{:?}", m.get("pagos"));       // Option<&Estado>: no hay valor cero silencioso
    println!("{:?}", Estado::Falla);

    let mut conteo: HashMap<String, u32> = HashMap::new();
    *conteo.entry("reportes".into()).or_insert(0) += 1;   // el patrón para contar
    *conteo.entry("reportes".into()).or_insert(0) += 1;
    println!("{:?}", conteo.get("reportes"));
}
```

```bash
$ rustc --edition 2024 fig04_01.rs && ./fig04_01
catalogo
true
Some(Ok)
None
Falla
Some(2)
```

⚠️ **Diferencia con Go que importa:** en Go, leer una llave que no existe **devuelve el valor cero** y
sigue como si nada. En Rust devuelve `Option` y tienes que decidir. El mismo descuido, dos desenlaces.

## `String` vs `&str`: la confusión de la semana

| | Qué es | Cuándo |
|---|---|---|
| `String` | **dueña**, en el heap, crece | cuando lo guardas en un struct |
| `&str` | **vista prestada**, no crece | **parámetros de función** |

**Fig. 4.2** | Recibe `&str`, acepta los dos.

```rust
// fig04_02.rs
fn saludar(n: &str) { println!("hola, {n}"); }         // ✅ acepta los dos: &String se convierte solo

fn main() {
    let propio = String::from("catalogo");
    saludar("pagos");
    saludar(&propio);
}
```

```bash
$ rustc --edition 2024 fig04_02.rs && ./fig04_02
hola, pagos
hola, catalogo
```

<!-- verificar:fragmento -->
```rust
fn saludar(n: String) { }       // ❌ obliga a quien llama a entregar la propiedad
```

🔑 **La regla:** **recibe `&str`, guarda `String`.** Es el equivalente rústico de «acepta interfaces,
devuelve structs».

## `Result` y el operador `?`

<!-- verificar:fragmento -->
```rust
enum Result<T, E> { Ok(T), Err(E) }        // en el estándar
```

**Fig. 4.3** | El operador `?` devuelve el error al llamador.

```rust
// fig04_03.rs
fn leer_config(ruta: &str) -> Result<String, std::io::Error> {
    let contenido = std::fs::read_to_string(ruta)?;   // 🔑 si falla, RETORNA el error
    Ok(contenido)
}

fn main() {
    match leer_config("servicios-que-no-existe.txt") {
        Ok(texto) => println!("{texto}"),
        Err(e) => println!("error: {e}"),
    }
}
```

```bash
$ rustc --edition 2024 fig04_03.rs && ./fig04_03
error: No such file or directory (os error 2)
```

🔑 **El `?` es la respuesta de Rust al `if err != nil` de Go.** Una sola letra: si es `Ok`, saca el
valor; si es `Err`, lo devuelve al llamador. Esto:

<!-- verificar:fragmento -->
```rust
let a = paso1()?;
let b = paso2(a)?;
let c = paso3(b)?;
Ok(c)
```

en Go serían doce líneas. **Es la comparación más honesta entre los dos lenguajes**, y no hay ganador
claro: Go es más explícito sobre dónde puede fallar, Rust es mucho más corto y también obligatorio.

⚠️ **`?` solo funciona en funciones que devuelven `Result`** (u `Option`). Si lo usas en `main`, hay que
declararlo: `fn main() -> Result<(), Box<dyn Error>>`.

## `panic!` vs `Result`

| | Cuándo |
|---|---|
| `Result` | **casi siempre**: un error esperable —red caída, archivo ausente, dato inválido— |
| `panic!` | un bug: un invariante roto, algo que no debería poder pasar |

**`.unwrap()` y `.expect("...")` son `panic!` disfrazados.** `expect` al menos deja un mensaje, así que
si vas a arriesgarte, usa `expect` y explica por qué creías que no podía fallar.

## Manejar errores de varios tipos: `anyhow` y `thiserror`

Al combinar `std::io::Error` con errores de HTTP y de YAML, los tipos dejan de cuadrar. La solución de la
comunidad, y es casi universal:

```bash
cargo add anyhow        # para APLICACIONES: un error que acepta cualquiera
cargo add thiserror     # para BIBLIOTECAS: define tus propios tipos de error
```

<!-- verificar:extracto:src/config.rs -->
```rust
use anyhow::{Context, Result};
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(serde_yaml::from_str(&txt)?)
}
```

## El ejercicio de la semana

1. Capítulos 8 y 9. Rustlings: `vecs`, `hashmaps`, `strings`, `error_handling`.
2. Guarda los estados en un `HashMap<String, Estado>` y arma el reporte **ordenado** (saca las llaves,
   `sort()`).
3. Escribe `fn cargar(ruta: &str) -> Result<Vec<Servicio>, ...>` que lea un archivo de texto y devuelva
   error si no existe o si una línea está mal formada. **Usa `?` en todos los pasos.**
4. Agrega `anyhow` y usa `.with_context()` para que el mensaje diga **qué archivo** falló.
5. Que `main` devuelva `Result` y el error salga bien impreso al fallar.
6. Compara: abre tu `cargar` de Go al lado del de Rust. **Cuenta las líneas de manejo de error de cada
   uno y anótalo en la bitácora.**

## Cómo sé que lo logré

- [ ] Sé cuándo recibo `&str` y cuándo guardo `String`
- [ ] El `?` me quedó claro y sé por qué no funciona en cualquier función
- [ ] `revisor archivo-que-no-existe` imprime un error con el nombre del archivo y sale ≠ 0
- [ ] Anoté la comparación de líneas contra la versión de Go
