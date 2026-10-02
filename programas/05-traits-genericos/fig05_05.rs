// fig05_05.rs
use std::fmt::Display;

fn imprimir<T: Display>(valor: T) {
    println!("{valor}");
}

fn main() {
    imprimir(vec!["catalogo"]);
}
