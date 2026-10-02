# Programas del curso

Aquí está cada programa del curso, por lección, listo para abrir y ejecutar.

- Los archivos `.salida.txt` muestran lo que imprime cada programa.
- Los `.error-esperado.txt` son ejemplos que a propósito no compilan: la lección enseña a leer ese error y a
  corregirlo.

Para ejecutar uno, entra a la carpeta de su lección y corre el comando que está en la lección, justo debajo del
código (por ejemplo `rustc --edition 2024 fig02_04.rs && ./fig02_04`).

`revisor/` es el proyecto completo que se construye en las lecciones 4 a 8, con sus pruebas (`cargo test`).
En `revisor/examples/` están los ejemplos que usan crates externos (`tokio`, `reqwest`, `serde`, `clap`); se ejecutan
con `cargo run --example ejemplo_tokio` (y así cada uno) desde `revisor/`.

Estas carpetas se generan a partir de las lecciones: no se editan a mano.
