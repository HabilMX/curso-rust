//! Convierte los estados en lo que se imprime: una tabla o un JSON.

use crate::modelo::{Estado, EstadoJson, Servicio};

/// Un servicio junto con lo que se supo de él.
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}

fn etiqueta(e: &Estado) -> &'static str {
    match e {
        Estado::Ok { .. } => "OK",
        Estado::Lento { .. } => "LENTO",
        Estado::Falla { .. } => "FALLA",
        Estado::NoIntentado => "NO",
    }
}

fn tiempo(e: &Estado) -> String {
    match e {
        Estado::Ok { ms, .. } | Estado::Lento { ms, .. } | Estado::Falla { ms, .. } => {
            format!("{ms}ms")
        }
        Estado::NoIntentado => "—".to_string(),
    }
}

fn detalle(e: &Estado) -> String {
    match e {
        Estado::Ok { codigo, .. } | Estado::Lento { codigo, .. } => codigo.to_string(),
        Estado::Falla { motivo, .. } => motivo.clone(),
        Estado::NoIntentado => "sin revisar".to_string(),
    }
}

/// La tabla, ordenada por nombre de servicio.
pub fn tabla(servicios: &[Servicio], estados: &[Estado]) -> String {
    let filas = ordenadas(servicios, estados);
    let ancho = filas
        .iter()
        .map(|(s, _)| s.nombre.chars().count())
        .max()
        .unwrap_or(0)
        .max("SERVICIO".len());

    let mut salida = format!(
        "{:<ancho$}  {:<6}  {:>8}  DETALLE\n",
        "SERVICIO", "ESTADO", "TIEMPO"
    );
    for (s, e) in filas {
        salida.push_str(&format!(
            "{:<ancho$}  {:<6}  {:>8}  {}\n",
            s.nombre,
            etiqueta(e),
            tiempo(e),
            detalle(e)
        ));
    }
    salida
}

/// El JSON, ordenado por nombre de servicio. `error` solo aparece cuando hay
/// algo que decir; `codigo` va `null` si nunca hubo respuesta.
pub fn json(servicios: &[Servicio], estados: &[Estado]) -> serde_json::Result<String> {
    let lineas: Vec<EstadoJson> = ordenadas(servicios, estados)
        .into_iter()
        .map(|(s, e)| EstadoJson {
            servicio: s.nombre.clone(),
            codigo: match e {
                Estado::Ok { codigo, .. } | Estado::Lento { codigo, .. } => Some(*codigo),
                _ => None,
            },
            ms: match e {
                Estado::Ok { ms, .. } | Estado::Lento { ms, .. } | Estado::Falla { ms, .. } => *ms,
                Estado::NoIntentado => 0,
            },
            error: match e {
                Estado::Falla { motivo, .. } => Some(motivo.clone()),
                Estado::NoIntentado => Some("sin revisar".to_string()),
                _ => None,
            },
        })
        .collect();
    serde_json::to_string_pretty(&lineas)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caso() -> (Vec<Servicio>, Vec<Estado>) {
        (
            vec![
                Servicio::new("pagos", "http://p"),
                Servicio::new("catalogo", "http://c"),
                Servicio::new("a", "http://a"),
            ],
            vec![
                Estado::Falla {
                    motivo: "codigo 500".to_string(),
                    ms: 12,
                },
                Estado::Ok { codigo: 200, ms: 7 },
                Estado::Lento {
                    codigo: 200,
                    ms: 1500,
                },
            ],
        )
    }

    #[test]
    fn la_tabla_va_ordenada_y_alineada() {
        let (s, e) = caso();
        let esperado = "\
SERVICIO  ESTADO    TIEMPO  DETALLE
a         LENTO     1500ms  200
catalogo  OK           7ms  200
pagos     FALLA       12ms  codigo 500
";
        assert_eq!(tabla(&s, &e), esperado);
    }

    #[test]
    fn la_tabla_cubre_las_cuatro_variantes() {
        let s = vec![Servicio::new("x", "http://x")];
        let t = tabla(&s, &[Estado::NoIntentado]);
        assert!(t.contains("NO") && t.contains("sin revisar"), "tabla: {t}");
    }

    #[test]
    fn el_json_omite_el_error_en_lo_sano_y_deja_codigo_nulo_en_la_falla() {
        let (s, e) = caso();
        let j: serde_json::Value = serde_json::from_str(&json(&s, &e).unwrap()).unwrap();
        let filas = j.as_array().unwrap();
        assert_eq!(filas.len(), 3);
        assert_eq!(filas[0]["servicio"], "a");
        assert!(
            filas[1].get("error").is_none(),
            "un servicio sano no lleva «error»"
        );
        assert!(
            filas[2]["codigo"].is_null(),
            "una falla no tiene código HTTP: va null"
        );
        assert_eq!(filas[2]["error"], "codigo 500");
    }
}
