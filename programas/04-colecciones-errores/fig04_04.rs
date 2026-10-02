// fig04_04.rs
fn main() {
    let previo = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let resultado = std::panic::catch_unwind(|| {
        panic!("la lista validada no puede estar vacia");
    });

    std::panic::set_hook(previo);
    println!("hubo panico: {}", resultado.is_err());
}
