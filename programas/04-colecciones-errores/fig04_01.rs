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
