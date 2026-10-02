// fig02_02.rs
fn main() {
    let a = String::from("hola");
    let b = a;                  // NO copia: MUEVE. Ahora b es la dueña
    println!("{a}");            // ← error: valor movido
}
