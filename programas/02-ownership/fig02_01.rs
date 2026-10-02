// fig02_01.rs
fn main() {
    {
        let s = String::from("hola");     // s es el dueño
        println!("{s}");
    }                                     // aquí termina el ámbito: se libera. Sin free(), sin GC
}
