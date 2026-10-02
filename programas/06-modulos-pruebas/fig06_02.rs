// fig06_02.rs
mod config {
    fn ruta_por_omision() -> &'static str {
        "servicios.yaml"
    }
}

fn main() {
    println!("{}", config::ruta_por_omision());
}
