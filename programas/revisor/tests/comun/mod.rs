//! Un servidor HTTP de mentira, hecho solo con la biblioteca estándar, que se
//! comporta como los cuatro casos que el `revisor` tiene que saber reportar.
//! Vive en un hilo de la propia prueba y escucha en un puerto que elige el
//! sistema, así que las pruebas no dependen de nada externo ni chocan entre sí.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

/// Levanta el servidor y devuelve su dirección, como `127.0.0.1:41233`.
pub fn servidor_demo() -> String {
    let escucha = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = escucha.local_addr().unwrap().to_string();
    thread::spawn(move || {
        for conexion in escucha.incoming().flatten() {
            thread::spawn(move || atender(conexion));
        }
    });
    direccion
}

fn atender(mut conexion: TcpStream) {
    let mut buffer = [0u8; 1024];
    let n = conexion.read(&mut buffer).unwrap_or(0);
    let peticion = String::from_utf8_lossy(&buffer[..n]);
    let ruta = peticion.split_whitespace().nth(1).unwrap_or("/");

    match ruta {
        "/ok" => responder(&mut conexion, "200 OK"),
        "/lento" => {
            thread::sleep(Duration::from_millis(1200));
            responder(&mut conexion, "200 OK");
        }
        "/error" => responder(&mut conexion, "500 Internal Server Error"),
        // acepta la conexión y nunca contesta: solo el límite de tiempo del cliente lo corta
        _ => thread::sleep(Duration::from_secs(5)),
    }
}

fn responder(conexion: &mut TcpStream, estado: &str) {
    let respuesta = format!("HTTP/1.1 {estado}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    let _ = conexion.write_all(respuesta.as_bytes());
}
