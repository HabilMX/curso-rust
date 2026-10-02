// fig01_01.rs
fn main() {
    let x: i32 = 5;             // tipo explícito (casi nunca hace falta: lo infiere)
    let mut y = 10;             // mutable
    const MAX: u32 = 100_000;   // constante, siempre con tipo

    y += x;
    println!("x = {x}, y = {y}, MAX = {MAX}");
}
