// fig02_05.rs
fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let texto = String::from("revisor listo");
    let palabra = primera_palabra(&texto);
    println!("primera: {palabra}");
}
