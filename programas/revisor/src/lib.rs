//! El `revisor` del curso de Rust: recibe una lista de servicios, los consulta
//! todos a la vez y produce un reporte.
//!
//! La lógica vive aquí, en la biblioteca, y `main.rs` solo lee los argumentos y
//! llama (lección 6): así todo lo de abajo se puede probar desde fuera.
//!
//! - [`modelo`]: el vocabulario (`Servicio`, `Estado`, `EstadoJson`).
//! - [`config`]: lee y valida el archivo YAML de servicios.
//! - [`revisar`]: consulta un servicio por HTTP, o todos a la vez con un límite.
//! - [`reporte`]: convierte los estados en tabla o en JSON.

pub mod config;
pub mod modelo;
pub mod reporte;
pub mod revisar;
