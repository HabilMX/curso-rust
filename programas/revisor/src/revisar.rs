//! Consulta los servicios: uno por HTTP, o todos a la vez con un tope.

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::sync::Semaphore;

use crate::modelo::{Estado, Servicio, UMBRAL_LENTO_MS};

/// Cuántos servicios se consultan a la vez si nadie dice otra cosa.
pub const PARALELO_POR_OMISION: usize = 5;

/// Consulta UN servicio. Nunca devuelve error: «este servicio no responde» es
/// justo lo que el programa quiere reportar, así que el fracaso va dentro del
/// [`Estado`] y no fuera de él.
pub async fn revisar(cliente: &reqwest::Client, s: &Servicio) -> Estado {
    let inicio = Instant::now();
    let respuesta = cliente
        .get(&s.url)
        .timeout(Duration::from_millis(s.timeout_ms)) // .await cede el control mientras espera
        .send()
        .await;
    let ms = inicio.elapsed().as_millis() as u64;

    match respuesta {
        Ok(r) if r.status().is_success() && ms > UMBRAL_LENTO_MS => Estado::Lento {
            codigo: r.status().as_u16(),
            ms,
        },
        Ok(r) if r.status().is_success() => Estado::Ok {
            codigo: r.status().as_u16(),
            ms,
        },
        Ok(r) => Estado::Falla {
            motivo: format!("codigo {}", r.status().as_u16()),
            ms,
        },
        Err(e) if e.is_timeout() => Estado::Falla {
            motivo: "se acabo el tiempo de espera".to_string(),
            ms,
        },
        Err(_) => Estado::Falla {
            motivo: "no responde".to_string(),
            ms,
        },
    }
}

/// Consulta TODOS los servicios a la vez, pero nunca más de `paralelo` al mismo
/// tiempo (0 significa [`PARALELO_POR_OMISION`]). Devuelve los estados en el
/// mismo orden que los servicios.
pub async fn revisar_todos(
    cliente: &reqwest::Client,
    servicios: &[Servicio],
    paralelo: usize,
) -> Vec<Estado> {
    let paralelo = if paralelo == 0 {
        PARALELO_POR_OMISION
    } else {
        paralelo
    };
    let turnos = Arc::new(Semaphore::new(paralelo));

    let futuros = servicios.iter().map(|s| {
        let turnos = Arc::clone(&turnos);
        async move {
            // pide turno; espera si ya hay `paralelo` corriendo, y lo devuelve al soltar `_turno`
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
            revisar(cliente, s).await
        }
    });
    join_all(futuros).await
}
