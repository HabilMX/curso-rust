// fig07_02.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let estado = "OK".to_string();

    let estados = Arc::new(Mutex::new(HashMap::new()));
    let copia = Arc::clone(&estados);
    let h = thread::spawn(move || {
        copia.lock().unwrap().insert("x".to_string(), estado);
    });

    h.join().unwrap();
    println!("{:?}", estados.lock().unwrap());
}
