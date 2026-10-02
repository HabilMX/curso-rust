// fig04_02.rs
fn saludar(n: &str) { println!("hola, {n}"); }         // ✅ acepta los dos: &String se convierte solo

fn main() {
    let propio = String::from("catalogo");
    saludar("pagos");
    saludar(&propio);
}
