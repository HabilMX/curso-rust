// fig03_02.rs
#[allow(dead_code)]              // este ejemplo no lee el código de los lentos
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),                      // lleva el mensaje dentro
    NoIntentado,
}

fn main() {
    let estados = [
        Estado::Ok { codigo: 200, ms: 120 },
        Estado::Lento { codigo: 200, ms: 1800 },
        Estado::Falla("no responde".to_string()),
        Estado::NoIntentado,
    ];

    for estado in estados {
        let texto = match estado {
            Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
            Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
            Estado::Falla(msg)           => format!("FALLA: {msg}"),
            Estado::NoIntentado          => "sin revisar".to_string(),
        };
        println!("{texto}");
    }
}
