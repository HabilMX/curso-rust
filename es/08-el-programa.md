# Semana 8 — El programa terminado

**The Book, capítulo 12** (el proyecto de línea de comandos). Al terminar: el `revisor` en Rust, completo.

## Las dependencias, y por qué cada una

```bash
cargo add tokio --features full          # el runtime async
cargo add reqwest --features json        # HTTP (usa tokio por debajo)
cargo add serde --features derive        # serializar: JSON, YAML, todo
cargo add serde_yaml                     # el formato de la config
cargo add clap --features derive         # línea de comandos
cargo add anyhow                         # errores de aplicación
cargo add serde_json                     # el JSON de la salida
cargo add futures                        # join_all, para esperar a todos a la vez
```

⚠️ **Ocho dependencias donde Go usó cero.** Es la diferencia de filosofía más concreta del curso: el
estándar de Rust es deliberadamente mínimo y el ecosistema pone el resto. A cambio, `serde` y `clap` son
más potentes que sus equivalentes de Go.

## `serde`: serializar con una línea

<!-- verificar:extracto:src/modelo.rs -->
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}

#[derive(Serialize)]
pub struct EstadoJson {
    pub servicio: String,
    pub codigo: Option<u16>,
    pub ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

🔑 **`serde` lee YAML, JSON, TOML y una docena más con el mismo struct.** En Go pusiste etiquetas
`json:"..."` y necesitabas otra biblioteca para YAML; aquí es el mismo derive.

## `clap`: la línea de comandos declarada

<!-- verificar:extracto:src/main.rs -->
```rust
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}
```

Y en `main` se leen así:

<!-- verificar:fragmento -->
```rust
let args = Args::parse();
```

**`--help` sale escrito solo**, con tipos, valores por omisión y validación. Es bastante más de lo que
da `flag` en Go, y es la dependencia que menos se discute.

## El binario

```bash
cargo build --release                    # target/release/revisor
```

⚠️ **Y aquí una diferencia que hay que decir sin adornos:** el binario de Rust con `tokio` y `reqwest`
pesa **más** que el de Go y **tarda mucho más en compilar** — minutos contra segundos. Lo que ganas es
que no hay recolector de basura, así que el uso de memoria es más bajo y predecible.

<!-- verificar:extracto:Cargo.toml -->
```toml
[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila

[dev-dependencies]
serde_json = "1.0.151"
```

Con eso baja bastante, a cambio de compilar más lento todavía.

**Compilación cruzada:** aquí Go gana de calle. En Rust necesitas instalar el *target* y a menudo un
enlazador cruzado:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Contra el `GOOS=linux go build` de Go, que no pide nada. **Es la ventaja más práctica de Go y conviene
reconocerla.**

## El ejercicio final

1. Capítulo 12 de The Book.
2. Junta todo: config en YAML con `serde`, banderas con `clap`, HTTP con `reqwest`, todos a la vez con
   `join_all`, límite de concurrencia con un semáforo de `tokio`.
3. `--formato json` con `serde_json`, y tabla por omisión.
4. Códigos de salida: 0 todo bien, 1 algún servicio falló, 2 no pudo arrancar.
5. `cargo clippy -- -D warnings` limpio y `cargo test` verde.
6. Compila para Linux y córrelo en otra máquina.

## 🔑 El ejercicio que de verdad cierra los dos cursos

**Pon los dos programas lado a lado y llena esta tabla en tu bitácora, con números tuyos:**

| | Go | Rust |
|---|---|---|
| Líneas de código | | |
| Dependencias externas | | |
| Tiempo de compilación en frío | | |
| Tamaño del binario | | |
| Memoria en ejecución (`/usr/bin/time -l`) | | |
| Tiempo que me tomó escribirlo | | |
| Veces que peleé con el compilador | | |

**Y responde en una frase cada una:**
- ¿Cuál escribirías para una herramienta interna que hay que tener el viernes?
- ¿Cuál para algo que va a correr en producción tres años sin que nadie lo toque?
- ¿Cuál te hizo entender mejor lo que estabas haciendo?

⚠️ **No hay respuesta correcta, y ése es el punto.** Después de haber escrito el mismo programa dos veces
tienes una opinión propia y fundada — que es exactamente lo que no se consigue leyendo comparaciones.

## Lo que sigue

1. **`cargo clippy` en todo lo que escribas.** Sigue enseñando meses después.
2. **[Rust for Rustaceans](https://nostarch.com/rust-rustaceans)** — el libro para cuando ya escribes
   Rust que funciona. Ahora sí.
3. **[Rust Atomics and Locks](https://marabos.nl/atomics/)** (gratis en línea) si la concurrencia te
   atrapó.
4. **Lee código:** `ripgrep` es de los proyectos Rust mejor escritos y es una herramienta que ya usas.
