// fig05_04.rs
fn devolver<'a>() -> &'a str {
    let nombre = String::from("catalogo");
    &nombre
}

fn main() {
    println!("{}", devolver());
}
