// ejemplo_reqwest.rs
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

/// Un servidor HTTP de mentira, en esta misma máquina: /sano responde 200,
/// /roto responde 500 y /lento tarda medio segundo en contestar.
fn servidor_de_mentira() -> String {
    let escucha = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = escucha.local_addr().expect("el servidor tiene dirección");
    thread::spawn(move || {
        for conexion in escucha.incoming().flatten() {
            thread::spawn(move || atender(conexion));
        }
    });
    format!("http://{direccion}")
}

fn atender(mut conexion: std::net::TcpStream) {
    let mut pedido = [0u8; 1024];
    let n = conexion.read(&mut pedido).unwrap_or(0);
    let texto = String::from_utf8_lossy(&pedido[..n]);
    let ruta = texto.split_whitespace().nth(1).unwrap_or("/");
    let estado = match ruta {
        "/sano" => "200 OK",
        "/roto" => "500 Internal Server Error",
        "/lento" => {
            thread::sleep(Duration::from_millis(500));
            "200 OK"
        }
        _ => "404 Not Found",
    };
    let _ = write!(
        conexion,
        "HTTP/1.1 {estado}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
}

#[tokio::main]
async fn main() {
    let base = servidor_de_mentira();
    let cliente = reqwest::Client::new();

    for ruta in ["sano", "roto", "lento"] {
        let respuesta = cliente
            .get(format!("{base}/{ruta}"))
            .timeout(Duration::from_millis(200))
            .send()
            .await;
        match respuesta {
            Ok(r) => println!("{ruta:<6} responde {}", r.status().as_u16()),
            Err(e) if e.is_timeout() => println!("{ruta:<6} se acabó el tiempo de espera"),
            Err(_) => println!("{ruta:<6} no responde"),
        }
    }

    // Un puerto donde nadie escucha: la conexión misma falla.
    let vacio = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = vacio.local_addr().expect("tiene dirección");
    drop(vacio);
    match cliente.get(format!("http://{direccion}/")).send().await {
        Ok(r) => println!("vacío  responde {}", r.status().as_u16()),
        Err(e) if e.is_connect() => println!("vacío  no responde (rechazó la conexión)"),
        Err(e) => println!("vacío  falló de otra forma: {e}"),
    }
}
