// fig06_01.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Falla(String),
}

fn resumen(e: &Estado) -> String {
    match e {
        Estado::Ok { codigo, ms } => format!("OK {codigo} en {ms}ms"),
        Estado::Falla(msg) => format!("FALLA: {msg}"),
    }
}

fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("dividir por cero");
    }
    a / b
}

fn main() {
    println!("{}", resumen(&Estado::Ok { codigo: 200, ms: 100 }));
    println!("{}", dividir(10, 2));
}

#[cfg(test)]                       // 🔑 solo se compila al hacer cargo test
mod tests {
    use super::*;

    #[test]
    fn estado_ok_con_200() {
        let e = Estado::Ok { codigo: 200, ms: 100 };
        assert!(matches!(e, Estado::Ok { .. }));
    }

    #[test]
    fn falla_sin_codigo() {
        assert_eq!(resumen(&Estado::Falla("x".into())), "FALLA: x");
    }

    #[test]
    #[should_panic(expected = "dividir por cero")]
    fn panico_esperado() {
        dividir(1, 0);
    }
}
