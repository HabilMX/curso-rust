// fig07_04.rs
use std::rc::Rc;
use std::thread;

fn main() {
    let conteo = Rc::new(0);
    thread::spawn(move || println!("{conteo}"));
}
