// fig01_02.rs
fn main() {
    let espacios = "   ";
    let espacios = espacios.len();

    let medicion: (u16, u64, bool) = (200, 750, true);
    let (codigo, ms, saludable) = medicion;
    let nombres = ["catalogo", "pagos"];

    println!("espacios = {espacios}");
    println!("codigo = {codigo}, ms = {ms}, saludable = {saludable}");
    println!("primer servicio = {}", nombres[0]);
}
