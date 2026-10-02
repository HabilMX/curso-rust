// fig02_04.rs
fn agregar_puerto(etiqueta: &mut String) {
    etiqueta.push_str(":443");
}

fn main() {
    let mut servicio = String::from("catalogo");
    agregar_puerto(&mut servicio);
    println!("{servicio}");
}
