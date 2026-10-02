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
