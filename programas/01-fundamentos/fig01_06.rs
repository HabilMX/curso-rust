// fig01_06.rs
fn main() {
    let mut x = 3;
    let servicios = vec!["catalogo", "pagos", "reportes"];

    loop { break; }                          // infinito, con break
    while x > 0 { x -= 1; }
    for i in 0..10 { print!("{i} "); }       // rango: 0 a 9
    println!();
    for i in 0..=10 { print!("{i} "); }      // inclusivo: 0 a 10
    println!();
    for s in &servicios { println!("{s}"); } // sobre una referencia, para no consumir la lista

    let r = loop { break 42; };              // 🔑 loop devuelve valor con break
    println!("x = {x}, r = {r}");
}
