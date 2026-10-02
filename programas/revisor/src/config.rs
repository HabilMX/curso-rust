//! Lee el archivo YAML con la lista de servicios.
//!
//! ```yaml
//! - nombre: catalogo
//!   url: http://localhost:8090/ok
//!   timeout_ms: 2000      # opcional: por omisión 5000
//! ```

use crate::modelo::Servicio;

use anyhow::{Context, Result};
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}

/// Revisa lo que el YAML no puede garantizar por sí solo: que haya al menos un
/// servicio, que ningún nombre se repita y que cada URL traiga esquema.
pub fn validar(servicios: &[Servicio]) -> Result<()> {
    anyhow::ensure!(
        !servicios.is_empty(),
        "el archivo no declara ningún servicio"
    );
    for (i, s) in servicios.iter().enumerate() {
        anyhow::ensure!(
            !servicios[..i].iter().any(|antes| antes.nombre == s.nombre),
            "el nombre «{}» está repetido",
            s.nombre
        );
        anyhow::ensure!(
            s.url.starts_with("http://") || s.url.starts_with("https://"),
            "la URL «{}» de «{}» debe empezar con http:// o https://",
            s.url,
            s.nombre
        );
        anyhow::ensure!(
            s.timeout_ms > 0,
            "el tiempo límite de «{}» debe ser mayor que cero",
            s.nombre
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn de_yaml(txt: &str) -> Vec<Servicio> {
        yaml_serde::from_str(txt).expect("el YAML de la prueba es válido")
    }

    #[test]
    fn timeout_por_omision_si_falta() {
        let v = de_yaml("- nombre: a\n  url: http://x\n");
        assert_eq!(v[0].timeout_ms, 5000);
    }

    #[test]
    fn timeout_propio_si_viene() {
        let v = de_yaml("- nombre: a\n  url: http://x\n  timeout_ms: 250\n");
        assert_eq!(v[0].timeout_ms, 250);
    }

    #[test]
    fn archivo_ausente_nombra_el_archivo() {
        let err = cargar("no-existe-este-archivo.yaml").unwrap_err();
        assert!(format!("{err:#}").contains("no-existe-este-archivo.yaml"));
    }

    #[test]
    fn validar_rechaza_lista_vacia() {
        assert!(validar(&[]).is_err());
    }

    #[test]
    fn validar_rechaza_nombre_repetido() {
        let v = vec![
            Servicio::new("a", "http://x"),
            Servicio::new("a", "http://y"),
        ];
        let err = validar(&v).unwrap_err().to_string();
        assert!(err.contains("repetido"), "mensaje: {err}");
    }

    #[test]
    fn validar_rechaza_url_sin_esquema() {
        let v = vec![Servicio::new("a", "localhost:80")];
        assert!(validar(&v).is_err());
    }

    #[test]
    fn validar_rechaza_timeout_cero() {
        let mut s = Servicio::new("a", "http://x");
        s.timeout_ms = 0;
        assert!(validar(&[s]).is_err());
    }

    #[test]
    fn validar_acepta_lista_buena() {
        let v = vec![
            Servicio::new("a", "http://x"),
            Servicio::new("b", "https://y"),
        ];
        assert!(validar(&v).is_ok());
    }
}
