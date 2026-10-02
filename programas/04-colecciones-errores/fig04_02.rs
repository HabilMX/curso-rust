// fig04_02.rs
fn saludar(n: &str) { println!("hola, {n}"); }

fn main() {
    let propio = String::from("catalogo");
    saludar("pagos");
    saludar(&propio);
}
