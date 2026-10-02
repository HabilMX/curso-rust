// fig02_03.rs
fn largo(s: &String) -> usize { s.len() }      // presta, no toma posesión

fn main() {
    let s = String::from("hola");
    let n = largo(&s);
    println!("{s} mide {n}");                   // sigue siendo mía ✓
}
