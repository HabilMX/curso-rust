// fig03_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}

fn main() {
    let estado = Estado::NoIntentado;
    let texto = match estado {
        Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
        Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
        Estado::Falla(msg)           => format!("FALLA: {msg}"),
    };
    println!("{texto}");
}
