// fig02_06.rs
fn main() {
    let a = String::from("hola");
    let b = a;                  // NO copia: MUEVE. Ahora b es el dueño
    println!("{a}");            // ← error: valor movido
}
