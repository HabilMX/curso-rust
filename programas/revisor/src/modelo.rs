//! El vocabulario del programa: qué es un servicio y qué puede pasarle al
//! consultarlo.

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

/// Lo que se supo de un servicio después de consultarlo.
///
/// Cada variante lleva sus propios datos: así `Falla` no tiene código HTTP que
/// alguien pueda leer por error, y `Ok` no tiene mensaje de error.
#[derive(Debug, Clone, PartialEq)]
pub enum Estado {
    /// Contestó con 2xx a tiempo.
    Ok { codigo: u16, ms: u64 },
    /// Contestó con 2xx, pero tardó más de [`UMBRAL_LENTO_MS`].
    Lento { codigo: u16, ms: u64 },
    /// No contestó, contestó con error, o se acabó el tiempo.
    Falla { motivo: String, ms: u64 },
    /// Nunca se llegó a consultar.
    NoIntentado,
}

impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
