// ejemplo_serde.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Servicio {
    nombre: String,
    url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    timeout_ms: u64,
}

fn timeout_por_omision() -> u64 {
    5000
}

fn main() -> Result<(), yaml_serde::Error> {
    let yaml = "\
- nombre: catalogo
  url: http://localhost:8080
- nombre: pagos
  url: http://localhost:8081
  timeout_ms: 250
";
    // YAML adentro: el mismo derive sirve para leer...
    let servicios: Vec<Servicio> = yaml_serde::from_str(yaml)?;
    for s in &servicios {
        println!("{} espera {} ms", s.nombre, s.timeout_ms);
    }

    // ...y para escribir JSON, que es otro formato con el mismo modelo.
    match serde_json::to_string(&servicios) {
        Ok(json) => println!("{json}"),
        Err(e) => println!("no se pudo escribir el JSON: {e}"),
    }

    // Un servicio sin `url` no es un Servicio: el error dice qué falta y dónde.
    let roto: Result<Vec<Servicio>, _> = yaml_serde::from_str("- nombre: sin-url\n");
    if let Err(e) = roto {
        println!("YAML inválido: {e}");
    }
    Ok(())
}
