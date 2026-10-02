# Semana 6 — Módulos, pruebas y cargo

**The Book, capítulos 7, 11 y 14.** Rustlings: `modules`, `tests`.

## Módulos

    src/
      main.rs            → el binario
      lib.rs             → la biblioteca (lo que otros podrían importar)
      servicio.rs
      revisar/
        mod.rs           → declara el módulo
        http.rs

    // en lib.rs
    pub mod servicio;
    pub mod revisar;

    // usar
    use crate::servicio::Servicio;

🔑 **En Rust todo es privado por omisión**, incluso entre módulos del mismo proyecto, y se abre con `pub`.
En Go la regla es la mayúscula inicial y solo aplica entre paquetes. Rust es más granular: hay `pub`,
`pub(crate)` —público dentro de mi proyecto— y `pub(super)`.

⚠️ **Tener `main.rs` y `lib.rs` a la vez es el patrón recomendado:** la lógica en la biblioteca, y el
binario solo parsea argumentos y llama. Así la lógica es testeable desde fuera y reusable.

## Pruebas: en el mismo archivo

    #[cfg(test)]                       // 🔑 solo se compila al hacer cargo test
    mod tests {
        use super::*;

        #[test]
        fn estado_ok_con_200() {
            let e = Estado::Ok { codigo: 200, ms: 100 };
            assert!(matches!(e, Estado::Ok { .. }));
        }

        #[test]
        fn falla_sin_codigo() {
            assert_eq!(resumen(&Estado::Falla("x".into())), "FALLA: x");
        }

        #[test]
        #[should_panic(expected = "dividir por cero")]
        fn panico_esperado() { /* ... */ }
    }

**Aquí sí hay aserciones**, al contrario que Go: `assert!`, `assert_eq!`, `assert_ne!`, y `matches!` para
enums. Y cuando `assert_eq!` falla, imprime **los dos valores** sin que tengas que escribir el mensaje.

### Pruebas de integración, en su carpeta

    tests/integracion.rs          # cada archivo es un binario aparte, solo ve la API pública

**Eso obliga a que tu biblioteca tenga una API usable**, que es media batalla del diseño.

## Los comandos

    cargo test                       # todas
    cargo test estado_ok             # las que coincidan
    cargo test -- --nocapture        # deja ver los println!
    cargo test --release             # con optimizaciones
    cargo clippy -- -D warnings      # 🔑 el linter, fallando en cada aviso
    cargo fmt --check                # verifica formato sin cambiar (para CI)
    cargo doc --open                 # genera la documentación y la abre

🔑 **`cargo clippy` es el mejor profesor de Rust idiomático que existe.** No es un linter de estilo: te
dice «esto se escribe así en Rust» con la razón. Pásalo y lee **todo** lo que diga.

⚠️ **Y la trampa gemela de la de Go:** `cargo test` con cero pruebas imprime `test result: ok. 0 passed`
y sale 0. **Lee el conteo, no el código de salida.**

## Dependencias y versiones

    cargo add serde --features derive
    cargo add tokio --features full
    cargo tree                        # el árbol completo, para ver quién trae qué
    cargo update                      # actualiza dentro de lo que permite Cargo.toml

**`Cargo.lock` va al repositorio si es un binario**, y **no** si es una biblioteca. Igual que `go.sum`,
pero con esa distinción.

⚠️ **Rust compila mucho más lento que Go, y hay que saberlo de antemano.** Un proyecto con `tokio` y
`reqwest` puede tardar minutos la primera vez. `cargo check` (que verifica sin generar binario) es tu
amigo durante el desarrollo: es varias veces más rápido.

## El ejercicio de la semana

1. Capítulos 7, 11 y 14. Rustlings: `modules`, `tests`.
2. Reorganiza el proyecto con `lib.rs` + módulos, y `main.rs` que solo arranque.
3. Escribe pruebas para `resumen()` con **todas** las variantes del enum.
4. Una prueba de integración en `tests/` que use solo la API pública.
5. 🔴 **`cargo clippy -- -D warnings` y arregla TODO lo que diga.** Anota en la bitácora las tres
   sugerencias que más te enseñaron.
6. Rompe una función a propósito y comprueba que la prueba se pone roja. **Una prueba que no has visto
   fallar no sabes si sirve.**

## Cómo sé que lo logré

- [ ] `cargo test` corre y dice **cuántas** pruebas
- [ ] `cargo clippy -- -D warnings` está limpio
- [ ] Vi una prueba ponerse roja al romper el código
- [ ] La prueba de integración compila usando solo lo `pub`
