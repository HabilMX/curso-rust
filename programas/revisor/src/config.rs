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
    // el error de yaml_serde repite el valor que no entendió, y ese valor
    // puede ser una URL con su clave: del error solo se dice dónde ocurrió
    let servicios = yaml_serde::from_str(&txt).map_err(|e| sin_el_valor(ruta, &e))?;
    Ok(servicios)
}

/// Convierte un error de `yaml_serde` en uno que dice en qué archivo y en qué
/// línea, pero no qué valor encontró ahí.
fn sin_el_valor(ruta: &str, e: &yaml_serde::Error) -> anyhow::Error {
    match e.location() {
        Some(lugar) => anyhow::anyhow!(
            "{ruta}: no es una lista de servicios válida (línea {}, columna {})",
            lugar.line(),
            lugar.column()
        ),
        None => anyhow::anyhow!("{ruta}: no es una lista de servicios válida"),
    }
}

/// Revisa lo que el YAML no puede garantizar por sí solo: que haya al menos un
/// servicio, que ningún nombre se repita y que cada URL se pueda usar.
///
/// Ningún error de esta función repite la URL. Una URL puede llevar un
/// secreto (`https://usuario:clave@...`, `?token=...`), y un mensaje de error
/// termina en una terminal, un log o un sistema de alertas. Cada error nombra
/// el servicio y dice qué le falta, nunca el valor.
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
        if let Some(problema) = problema_de_url(&s.url) {
            anyhow::bail!("la URL de «{}» {problema}", s.nombre);
        }
        anyhow::ensure!(
            s.timeout_ms > 0,
            "el tiempo límite de «{}» debe ser mayor que cero",
            s.nombre
        );
    }
    Ok(())
}

/// Qué le impide al revisor usar una URL, o `None` si se puede usar. Devuelve
/// siempre una frase fija, nunca un pedazo de la URL.
///
/// Exige `http` o `https` y un servidor, y rechaza usuario y clave: si un
/// servicio pide credenciales, van en otro lado (una variable de entorno, un
/// gestor de secretos), nunca en el archivo de servicios.
fn problema_de_url(texto: &str) -> Option<&'static str> {
    // el texto del error de `Url::parse` no se usa: basta con saber que falló
    let Ok(url) = reqwest::Url::parse(texto) else {
        return Some("no se puede leer como URL");
    };
    if url.scheme() != "http" && url.scheme() != "https" {
        return Some("debe empezar con http:// o https://");
    }
    if url.host_str().is_none_or(str::is_empty) {
        return Some("no tiene servidor");
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Some("trae usuario o clave; las credenciales no van en la URL");
    }
    None
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

    /// Una URL con todos los lugares donde viaja un secreto: usuario y clave,
    /// servidor interno, ruta, query string y fragmento.
    const URL_CON_SECRETOS: &str =
        "http://yo:supersecreto@pagos.interno:8443/estado?token=SECRETO#ancla";

    /// Ningún pedazo de [`URL_CON_SECRETOS`] puede aparecer en un mensaje.
    fn sin_secretos(mensaje: &str) {
        for pedazo in [
            "yo:",
            "supersecreto",
            "pagos.interno",
            "/estado",
            "token",
            "SECRETO",
            "ancla",
        ] {
            assert!(
                !mensaje.contains(pedazo),
                "el mensaje repite «{pedazo}»: {mensaje}"
            );
        }
    }

    #[test]
    fn validar_rechaza_url_con_usuario_y_clave() {
        let err = validar(&[Servicio::new("pagos", URL_CON_SECRETOS)])
            .expect_err("una URL con usuario y clave no se acepta");
        let err = format!("{err:#}");
        sin_secretos(&err);
        assert!(
            err.contains("las credenciales no van en la URL"),
            "mensaje: {err}"
        );
    }

    #[test]
    fn validar_rechaza_url_con_solo_usuario_o_solo_clave() {
        for url in ["https://token123@a.mx", "https://:supersecreto@a.mx"] {
            assert!(validar(&[Servicio::new("a", url)]).is_err(), "aceptó {url}");
        }
    }

    #[test]
    fn validar_rechaza_url_sin_servidor() {
        assert!(validar(&[Servicio::new("a", "http://")]).is_err());
    }

    #[test]
    fn ningun_error_de_url_repite_la_url() {
        // Sin esquema, con un esquema ajeno y con credenciales: tres razones
        // distintas para rechazarla, y ninguna puede imprimir lo que rechaza.
        for url in [
            "yo:supersecreto@pagos.interno/estado?token=SECRETO#ancla",
            "ftp://yo:supersecreto@pagos.interno/estado?token=SECRETO#ancla",
            URL_CON_SECRETOS,
        ] {
            let err = validar(&[Servicio::new("pagos", url)]).expect_err("la URL no se acepta");
            sin_secretos(&format!("{err:#}"));
        }
    }

    #[test]
    fn un_error_del_yaml_no_repite_el_valor() {
        // El texto de un error de yaml_serde repite el valor que no entendió;
        // aquí ese valor es la URL, en dos lugares donde no va.
        for (caso, contenido) in [
            ("lista", format!("- {URL_CON_SECRETOS}\n")),
            (
                "timeout",
                format!(
                    "- nombre: pagos\n  url: https://a.mx\n  timeout_ms: \"{URL_CON_SECRETOS}\"\n"
                ),
            ),
        ] {
            let ruta = std::env::temp_dir()
                .join(format!("revisor-{}-yaml-{caso}.yaml", std::process::id()));
            std::fs::write(&ruta, contenido).unwrap();
            let err = cargar(ruta.to_str().unwrap()).expect_err("no es una lista de servicios");
            let _ = std::fs::remove_file(&ruta);
            let err = format!("{err:#}");
            sin_secretos(&err);
            assert!(
                err.contains("(línea "),
                "el mensaje debe decir dónde: {err}"
            );
        }
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
