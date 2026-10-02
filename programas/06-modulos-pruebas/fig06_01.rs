// fig06_01.rs
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn linea(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::linea("catalogo", true));
    println!("{}", reporte::linea("pagos", false));
}
