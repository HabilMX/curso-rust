// ejemplo_clap.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}

fn main() {
    // `try_parse_from` recibe los argumentos como una lista, en vez de leer
    // los del sistema operativo: así el ejemplo corre igual en cualquier máquina.
    let a = Args::try_parse_from(["revisor", "--formato", "json", "-p", "2"])
        .expect("estos argumentos son válidos");
    println!("{a:?}");

    // Un valor que no es número: clap lo rechaza antes de que el programa arranque.
    if let Err(e) = Args::try_parse_from(["revisor", "--paralelo", "muchos"]) {
        println!("{:?}", e.kind());
        println!("{}", e.to_string().lines().next().unwrap_or(""));
    }

    // Una opción que no existe.
    if let Err(e) = Args::try_parse_from(["revisor", "--velocidad", "3"]) {
        println!("{:?}", e.kind());
    }
}
