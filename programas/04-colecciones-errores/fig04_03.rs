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
