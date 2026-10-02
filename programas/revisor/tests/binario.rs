//! Pruebas del binario de punta a punta: corren el ejecutable `revisor` de
//! verdad, con un archivo YAML y el servidor de mentira, y miran lo que imprime
//! y con qué código de salida termina.

mod comun;

use std::process::{Command, Output};

fn revisor(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_revisor"))
        .args(args)
        .output()
        .expect("el binario arranca")
}

fn yaml_temporal(nombre: &str, contenido: &str) -> String {
    let ruta = std::env::temp_dir().join(format!("revisor-{}-{nombre}.yaml", std::process::id()));
    std::fs::write(&ruta, contenido).unwrap();
    ruta.to_string_lossy().into_owned()
}

fn texto(salida: &[u8]) -> String {
    String::from_utf8_lossy(salida).into_owned()
}

#[test]
fn todo_sano_sale_con_cero_y_tabla() {
    let d = comun::servidor_demo();
    let ruta = yaml_temporal(
        "sano",
        &format!(
            "- nombre: catalogo\n  url: http://{d}/ok\n- nombre: pagos\n  url: http://{d}/ok\n"
        ),
    );
    let r = revisor(&["--archivo", &ruta]);
    let out = texto(&r.stdout);
    assert_eq!(r.status.code(), Some(0), "salida: {out}");
    assert!(out.starts_with("SERVICIO"), "salida: {out}");
    assert!(
        out.contains("catalogo") && out.contains("pagos"),
        "salida: {out}"
    );
}

#[test]
fn un_servicio_caido_sale_con_uno() {
    let d = comun::servidor_demo();
    let ruta = yaml_temporal(
        "caido",
        &format!("- nombre: bien\n  url: http://{d}/ok\n- nombre: mal\n  url: http://{d}/error\n"),
    );
    let r = revisor(&["-a", &ruta]);
    let out = texto(&r.stdout);
    assert_eq!(r.status.code(), Some(1), "salida: {out}");
    assert!(
        out.contains("FALLA") && out.contains("codigo 500"),
        "salida: {out}"
    );
}

#[test]
fn el_formato_json_es_json_valido() {
    let d = comun::servidor_demo();
    let ruta = yaml_temporal(
        "json",
        &format!("- nombre: catalogo\n  url: http://{d}/ok\n"),
    );
    let r = revisor(&["-a", &ruta, "-f", "json"]);
    let j: serde_json::Value = serde_json::from_str(&texto(&r.stdout)).expect("stdout es JSON");
    assert_eq!(j[0]["servicio"], "catalogo");
    assert_eq!(j[0]["codigo"], 200);
    assert_eq!(r.status.code(), Some(0));
}

#[test]
fn un_archivo_que_no_existe_sale_con_dos_y_nombra_el_archivo() {
    let r = revisor(&["--archivo", "archivo-que-no-existe.yaml"]);
    let err = texto(&r.stderr);
    assert_eq!(r.status.code(), Some(2), "stderr: {err}");
    assert!(err.contains("archivo-que-no-existe.yaml"), "stderr: {err}");
}

#[test]
fn un_formato_desconocido_sale_con_dos() {
    let r = revisor(&["-f", "xml"]);
    assert_eq!(r.status.code(), Some(2), "stderr: {}", texto(&r.stderr));
}

#[test]
fn una_lista_vacia_sale_con_dos() {
    let ruta = yaml_temporal("vacia", "[]\n");
    let r = revisor(&["-a", &ruta]);
    assert_eq!(r.status.code(), Some(2), "stderr: {}", texto(&r.stderr));
}
