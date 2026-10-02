# Semana 7 — Concurrencia y async

**The Book, capítulos 16 y 17.** El 17 (async) es nuevo de la edición 2024.

**Aquí se ve el contraste más fuerte con Go.** En Go la concurrencia es parte del lenguaje: `go f()` y
ya. En Rust hay **dos mundos** y hay que elegir.

## Mundo 1 — Hilos del sistema (capítulo 16)

**Fig. 7.1** | Un hilo por servicio.

```rust
// fig07_01.rs
use std::thread;

struct Servicio {
    nombre: String,
}

type Estado = String;     // en el curso es el enum de la semana 3; aquí basta un texto

fn revisar(s: &Servicio) -> Estado {
    format!("{}: OK", s.nombre)
}

fn main() {
    let servicios = vec![
        Servicio { nombre: "catalogo".to_string() },
        Servicio { nombre: "pagos".to_string() },
        Servicio { nombre: "reportes".to_string() },
    ];

    let handles: Vec<_> = servicios.into_iter().map(|s| {
        thread::spawn(move || revisar(&s))      // `move` entrega la propiedad al hilo
    }).collect();

    for h in handles {
        let estado = h.join().unwrap();          // espera y recoge el resultado
        println!("{estado}");
    }
}
```

```bash
$ rustc --edition 2024 fig07_01.rs && ./fig07_01
catalogo: OK
pagos: OK
reportes: OK
```

🔑 **`join()` devuelve el valor**, así que no necesitas canales para recoger resultados — en Go hace falta
un `WaitGroup` más un canal. Aquí es más directo.

⚠️ **Pero los hilos del sistema cuestan ~8 MB cada uno**, contra los ~2 KB de una goroutine. **Mil
servicios a la vez no es opción**; en Go sí.

### Compartir datos entre hilos

**Fig. 7.2** | Un dato compartido entre hilos.

```rust
// fig07_02.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let estado = "OK".to_string();

    let estados = Arc::new(Mutex::new(HashMap::new()));
    let copia = Arc::clone(&estados);
    let h = thread::spawn(move || {
        copia.lock().unwrap().insert("x".to_string(), estado);
    });

    h.join().unwrap();
    println!("{:?}", estados.lock().unwrap());
}
```

```bash
$ rustc --edition 2024 fig07_02.rs && ./fig07_02
{"x": "OK"}
```

🔴 **Y aquí está la gran diferencia: el `Mutex` en Rust ENVUELVE el dato.** No puedes acceder al
`HashMap` sin pasar por `lock()`, porque el dato vive dentro del candado. En Go el mutex está *al lado*
del dato, y olvidar el `Lock()` compila perfectamente y rompe el programa en producción.

**En Rust ese error no existe.** No es que sea más difícil de cometer: es imposible. Esto es lo que se
llama *fearless concurrency*, y es el título del capítulo 16.

## Mundo 2 — Async, que es lo que vas a usar (capítulo 17)

<!-- verificar:fragmento -->
```rust
// cargo add tokio --features full
#[tokio::main]
async fn main() {
    let futuros: Vec<_> = servicios.iter().map(|s| revisar(s)).collect();
    let estados = futures::future::join_all(futuros).await;   // todos a la vez
}

async fn revisar(s: &Servicio) -> Estado {
    let resp = reqwest::get(&s.url).await;    // .await cede el control mientras espera
    // ...
}
```

**Para miles de conexiones de red, async es lo correcto**: no gasta un hilo por tarea.

🔴 **Y la cosa más importante de esta semana:** **Rust no trae un runtime async.** El lenguaje define
`async`/`await`, pero **quien los ejecuta es una biblioteca** — `tokio` en la práctica. En Go el
planificador viene dentro y no hay que decidir nada.

Eso tiene una consecuencia que muerde: **las bibliotecas async están casadas con un runtime.** Una hecha
para `tokio` no funciona con otro. Es la mayor fuente de frustración al empezar en Rust, y por eso
`tokio` ganó: casi todo el ecosistema lo asume.

## La comparación honesta, ahora que viste las dos

| | Go | Rust |
|---|---|---|
| Arrancar tarea | `go f()` | `thread::spawn` o `tokio::spawn` |
| Costo por tarea | ~2 KB | ~8 MB (hilo) · ~pocos KB (async) |
| Runtime | **incluido** | **eliges biblioteca** |
| Carrera de datos | detectada en ejecución con `-race` | **imposible: no compila** |
| Complejidad para el que escribe | **baja** | alta |
| Garantías | pocas | **muchas** |

**Ninguno gana.** Go optimizó que sea fácil escribir concurrencia correcta la mayoría de las veces. Rust
optimizó que sea **imposible** escribirla incorrecta, y cobra en dificultad. Para el `revisor`, Go es más
cómodo; para algo donde una carrera cueste dinero, Rust te la ahorra de raíz.

## El ejercicio de la semana

1. Capítulo 16, luego el 17.
2. **Primero con hilos**: revisa los servicios con `thread::spawn` y recoge con `join()`.
3. **Luego con async**: agrega `tokio` y hazlo con `join_all`. **Mide los dos** con `--release`.
4. Comparte un contador entre hilos con `Arc<Mutex<_>>`.
5. 🔑 **Intenta a propósito compartir un `HashMap` entre hilos SIN `Arc<Mutex<>>`.** Lee el error:
   `` `Rc<...>` cannot be sent between threads safely ``. **Eso que el compilador te está impidiendo es
   exactamente el bug que en Go necesitas `-race` para encontrar.** Anótalo.
6. Pon un timeout con `tokio::time::timeout`.

## Cómo sé que lo logré

- [ ] Las dos versiones funcionan y tengo los tiempos de ambas
- [ ] **Vi al compilador negarse a compartir datos sin sincronizar**
- [ ] Sé por qué Rust no trae runtime async y qué implica
- [ ] Puedo explicar por qué el `Mutex` de Rust envuelve el dato y el de Go no
