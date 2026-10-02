// fig07_03.rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (emisor, receptor) = mpsc::channel();

    let hilo = thread::spawn(move || {
        for estado in ["catalogo: OK", "pagos: OK"] {
            emisor.send(estado).unwrap();
        }
    });

    hilo.join().unwrap();

    for estado in receptor {
        println!("{estado}");
    }
}
