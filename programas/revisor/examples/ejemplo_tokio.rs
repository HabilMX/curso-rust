// ejemplo_tokio.rs
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::time::sleep;

async fn revisar(nombre: &str, espera_ms: u64) -> String {
    sleep(Duration::from_millis(espera_ms)).await;
    println!("terminó {nombre}");
    format!("{nombre}: respondió tras {espera_ms} ms")
}

#[tokio::main]
async fn main() {
    let servicios = [("catalogo", 300), ("pagos", 100), ("usuarios", 200)];
    let inicio = Instant::now();

    let futuros = servicios.iter().map(|(nombre, ms)| revisar(nombre, *ms));
    let resultados = join_all(futuros).await;

    println!("--- en el orden de la lista ---");
    for resultado in &resultados {
        println!("{resultado}");
    }
    // Esperarlos uno tras otro habría tardado 600 ms; a la vez tardan lo del más lento.
    let a_la_vez = inicio.elapsed() < Duration::from_millis(550);
    println!("tardó menos que la suma de las esperas: {a_la_vez}");
}
