//! Pruebas de integración: solo ven la API pública (`pub`) de la biblioteca,
//! igual que la vería otro programa que la usara.

mod comun;

use revisor::modelo::{Estado, Servicio};
use revisor::revisar::{revisar, revisar_todos};

fn servicio(nombre: &str, direccion: &str, ruta: &str, timeout_ms: u64) -> Servicio {
    Servicio {
        nombre: nombre.to_string(),
        url: format!("http://{direccion}{ruta}"),
        timeout_ms,
    }
}

#[tokio::test]
async fn un_servicio_sano_sale_ok() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let e = revisar(&cliente, &servicio("ok", &d, "/ok", 2000)).await;
    assert!(matches!(e, Estado::Ok { codigo: 200, .. }), "estado: {e:?}");
}

#[tokio::test]
async fn un_500_es_falla_con_su_codigo() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let e = revisar(&cliente, &servicio("mal", &d, "/error", 2000)).await;
    assert!(
        matches!(&e, Estado::Falla { motivo, .. } if motivo == "codigo 500"),
        "estado: {e:?}"
    );
}

#[tokio::test]
async fn un_servicio_que_no_contesta_se_corta_por_tiempo() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let e = revisar(&cliente, &servicio("mudo", &d, "/nunca", 300)).await;
    assert!(
        matches!(&e, Estado::Falla { motivo, .. } if motivo == "se acabo el tiempo de espera"),
        "estado: {e:?}"
    );
}

#[tokio::test]
async fn un_servicio_lento_pero_sano_sale_lento() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let e = revisar(&cliente, &servicio("lento", &d, "/lento", 3000)).await;
    assert!(
        matches!(e, Estado::Lento { codigo: 200, .. }),
        "estado: {e:?}"
    );
}

#[tokio::test]
async fn un_puerto_cerrado_es_falla() {
    let cliente = reqwest::Client::new();
    // el puerto 1 en la máquina local no tiene a nadie escuchando
    let e = revisar(&cliente, &servicio("nadie", "127.0.0.1:1", "/", 1000)).await;
    assert!(matches!(e, Estado::Falla { .. }), "estado: {e:?}");
}

#[tokio::test]
async fn revisar_todos_devuelve_los_estados_en_el_orden_de_los_servicios() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let servicios = vec![
        servicio("a", &d, "/ok", 2000),
        servicio("b", &d, "/error", 2000),
        servicio("c", &d, "/ok", 2000),
    ];
    let estados = revisar_todos(&cliente, &servicios, 2).await;
    assert_eq!(estados.len(), 3);
    assert!(estados[0].esta_bien());
    assert!(!estados[1].esta_bien());
    assert!(estados[2].esta_bien());
}

#[tokio::test]
async fn el_tope_de_paralelo_se_respeta() {
    // tres servicios que tardan ~1.2 s cada uno: con tope 1 se hacen en fila
    // (≥ 3.6 s) y con tope 3 a la vez (< 3.6 s). Mide el tiempo total, no el de cada uno.
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let servicios: Vec<Servicio> = (0..3)
        .map(|i| servicio(&format!("s{i}"), &d, "/lento", 4000))
        .collect();

    let t = std::time::Instant::now();
    revisar_todos(&cliente, &servicios, 3).await;
    let a_la_vez = t.elapsed();

    let t = std::time::Instant::now();
    revisar_todos(&cliente, &servicios, 1).await;
    let en_fila = t.elapsed();

    assert!(
        a_la_vez < std::time::Duration::from_millis(3000),
        "a la vez: {a_la_vez:?}"
    );
    assert!(
        en_fila >= std::time::Duration::from_millis(3600),
        "en fila: {en_fila:?}"
    );
}
