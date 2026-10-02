// fig05_03.rs
fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", mas_largo("catalogo", "pagos"));
}
