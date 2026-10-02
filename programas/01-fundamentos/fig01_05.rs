// fig01_05.rs
fn main() {
    let x = 7;
    let n = if x > 5 { "grande" } else { "chico" };      // el if DEVUELVE valor

    let cuadrado = {
        let t = x * x;
        t                          // 🔑 sin punto y coma = es el valor del bloque
    };

    println!("{n} {cuadrado} {}", doble(x));
}

fn doble(x: i32) -> i32 {
    x * 2                      // sin `return` y sin `;`
}
