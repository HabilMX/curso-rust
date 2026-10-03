# Leçon 6 — Modules, tests et Cargo

**Durée :** 2 × 45 min.

**Ce que tu construis :** le projet `revisor` ordonné et testé.

**Ce que tu apprends :** modules et visibilité, tests unitaires et d'intégration, `cargo test`, dépendances et versions.

**The Rust Book, chapitres 7, 11 et 14.** Rustlings : `modules`, `tests`.

## À la fin, tu vas pouvoir

- Séparer un programme Rust en modules aux responsabilités claires et naviguer dans leurs chemins avec `crate`, `self` et `super`.
- Expliquer pourquoi tout est privé par défaut et choisir entre `pub`, `pub(crate)` et une API privée.
- Distinguer un test unitaire d'un test d'intégration et savoir quelle sorte de problème chacun détecte.
- Écrire des tests avec `#[test]`, `assert!`, `assert_eq!`, `matches!` et `#[should_panic]`.
- Exécuter, filtrer et diagnostiquer des tests avec `cargo test`.
- Lire `Cargo.toml` et `Cargo.lock`, ajouter une dépendance avec une version raisonnable et examiner son arbre transitif.
- Garder le `revisor` vérifiable avec `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` et `cargo test`.

## Le pourquoi avant le comment

Jusqu'ici, le `revisor` a pu tenir dans quelques fichiers parce que le cours présentait les pièces du langage une par une. Tu connais déjà les types qui modélisent un service, les collections qui stockent la liste, les erreurs qui décrivent les échecs externes et les traits qui expriment des contrats. Le problème suivant n'est pas d'écrire une autre fonction : c'est d'éviter que ces fonctions ne se transforment en une seule masse difficile à lire, à tester et à changer.

Un fichier énorme ne cesse pas de fonctionner automatiquement. Le problème apparaît quand une modification apparemment locale oblige à comprendre trop de choses à la fois. Si le code qui lit le YAML, valide les services, fait des requêtes HTTP, produit du JSON et analyse les arguments vit mélangé, un test de format peut finir par exiger le réseau ; une modification de configuration peut affecter le binaire ; et une fonction privée peut finir par être utilisée de n'importe où uniquement parce que personne n'a défini de frontière.

Les modules sont ces frontières. Ce ne sont pas des dossiers pour que le projet « ait l'air rangé » ; ce sont des noms pour des responsabilités et, en Rust, aussi une partie explicite du contrôle d'accès. Un module peut dire : « ici se définit ce qu'est un service », « ici se charge la configuration » ou « ici un état devient un tableau ». Celui qui utilise un module connaît son interface publique ; il n'a pas besoin de dépendre des détails internes avec lesquels il est implémenté, et ne devrait pas le faire.

Le mot important est interface. En Go, un dossier définit un paquet et une initiale majuscule décide si un nom peut franchir la frontière du paquet. Rust est plus détaillé. Un dossier peut aider à organiser les fichiers, mais la visibilité dépend des modules et de `pub`. Un nom sans `pub` est privé, même s'il est dans un autre fichier du même projet. Cela paraît strict au début, mais évite qu'une fonction auxiliaire ne devienne par accident une promesse pour le reste du programme.

Le `revisor` applique cette idée avec deux produits dans le même paquet Cargo. `src/lib.rs` déclare une bibliothèque : c'est là que vit la logique réutilisable et vérifiable. `src/main.rs` déclare le binaire : il reçoit les arguments, appelle la bibliothèque, affiche le résultat et décide du code de sortie. Séparer les deux permet de tester la logique sans avoir à invoquer la ligne de commande à chaque cas. Cela permet aussi aux tests d'intégration d'utiliser le `revisor` comme le ferait une autre application : en important exclusivement son API publique.

C'est le même principe que tu as utilisé en Go en séparant les paquets par responsabilité, mais Rust rend le contrat plus visible. En Go, une fonction en minuscule ne peut pas être importée depuis un autre paquet ; en Rust, une fonction, une struct, un champ ou un module exige une visibilité déclarée. Les deux décisions cherchent à limiter les dépendances. Rust te donne plus de niveaux pour exprimer cette intention : public pour quiconque importe la bibliothèque, public seulement à l'intérieur du paquet actuel ou public pour un module parent.

Les tests transforment ces frontières en quelque chose de vérifiable. Un test unitaire vit près de la fonction qu'il teste et peut examiner des détails privés. Il est utile pour de petites règles : si le YAML ne déclare pas `timeout_ms`, la valeur par défaut est-elle appliquée ? une liste vide est-elle rejetée ? le tableau conserve-t-il son alignement ? Un test d'intégration vit sous `tests/`, se compile comme un autre crate et ne peut utiliser que `pub`. Il est utile pour vérifier que l'interface suffit réellement : si quelqu'un construit un `Servicio`, appelle `revisar` et reçoit un `Estado`, le contrat public fonctionne-t-il sans dépendre de détails internes ?

Ne confonds pas beaucoup de tests avec une bonne couverture des décisions. Une suite peut avoir cent tests qui répètent le même cas sain et aucun qui couvre une URL invalide, un fichier absent ou un service qui met trop de temps. Ne fais pas non plus du pourcentage de couverture un objectif isolé. La question utile est : « quel comportement important pourrait se casser sans qu'un test ne devienne rouge ? » Le `revisor` teste les états sains, le HTTP 500, les délais d'attente, la configuration invalide, le format JSON et les codes de sortie parce que ce sont des comportements qui comptent pour qui utilise le programme.

`cargo` réunit ces décisions. Il ne se contente pas de compiler : il sait quels fichiers forment le paquet, quelles dépendances il requiert, quelle édition de Rust il utilise, quels tests existent et quels artefacts il doit construire. Dans les figures du cours, tu continues à appeler `rustc` directement pour voir un exemple isolé. Dans le projet réel, tu utilises `cargo` parce qu'il n'existe plus d'invocation raisonnable à la main qui se souvienne de tous les modules, crates, fonctionnalités et cibles de test.

La discipline de cette leçon est simple : organise par responsabilité, ouvre la plus petite surface publique nécessaire et teste chaque frontière du bon côté. Si un test unitaire a besoin du réseau, tu as probablement mélangé une règle pure avec de l'infrastructure. Si un test d'intégration a besoin d'importer un détail privé, ton API publique n'exprime probablement pas ce dont un autre consommateur a besoin. Si `cargo test` dit qu'il n'a trouvé aucun test, ne te félicite pas encore : vérifie le décompte.

## Les concepts

### Modules : noms, chemins et responsabilités

Un module regroupe des noms liés. Il peut être déclaré dans un fichier avec `mod nombre { ... }`, ou vivre dans un autre fichier. Dans un paquet Cargo moderne, `src/lib.rs` et `src/main.rs` sont des racines de crate distinctes. Depuis l'une ou l'autre, `crate` signifie « la racine de ce crate » ; `self` signifie le module courant ; et `super` signifie le module parent.

Une erreur fréquente est de penser que fichier et module sont synonymes. Un fichier peut contenir plusieurs modules, et un module peut s'ouvrir dans un autre fichier. La structure de fichiers aide une personne à trouver du code ; la structure de modules détermine comment Rust résout les chemins et applique la visibilité. Ne conçois pas d'abord un arbre de dossiers vides. Commence par des responsabilités qui ont une raison stable de changer séparément.

**Fig. 6.1** | Un module offre une fonction publique et garde son détail privé.

```rust
// fig06_01.rs
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn linea(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::linea("catalogo", true));
    println!("{}", reporte::linea("pagos", false));
}
```

```bash
$ rustc --edition 2024 fig06_01.rs && ./fig06_01
catalogo: OK
pagos: FALLA
```

`reporte::linea` est accessible depuis `main` parce qu'elle a `pub`. La fonction `etiqueta` n'a pas `pub`, donc elle ne peut être utilisée qu'à l'intérieur de `reporte`. Cette décision ne cache pas d'information par mystère : elle exprime que les autres parties du programme ont besoin d'une ligne terminée, pas de connaître la règle interne qui traduit un booléen en texte. Si, plus tard, tu changes `"FALLA"` en `"NO DISPONIBLE"`, seul le module propriétaire doit changer.

Dans le `revisor`, la racine de la bibliothèque énumère ses responsabilités publiques. Il n'y a pas de module appelé `utilidades`, parce que ce nom n'explique pas quelle responsabilité il possède. `config` charge et valide la configuration ; `modelo` définit le vocabulaire ; `reporte` traduit les états en texte ou en JSON ; `revisar` interroge les services.

<!-- verificar:extracto:src/lib.rs -->
```rust
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
```

Le mot `pub` devant chaque `mod` fait que ces modules forment l'entrée publique de la bibliothèque. Cela ne rend pas public tout leur contenu. Chaque module décide à son tour quelles structs, fonctions et constantes il expose. Cette composition est un avantage : publier `reporte` permet d'appeler `revisor::reporte::tabla`, mais n'oblige pas à publier les fonctions auxiliaires qui trient les lignes ou calculent les étiquettes.

L'arbre de modules du `revisor` ne prétend pas être une hiérarchie universelle. Dans un petit projet, quatre modules à plat sont plus lisibles qu'une longue chaîne de dossiers. Quand une responsabilité grandit assez, elle peut se diviser en sous-modules. La question n'est pas « combien de fichiers un projet professionnel doit-il avoir ? », mais « puis-je décrire en une phrase ce qui appartient ici et ce qui n'y appartient pas ? ».

### Visibilité : privé par défaut comme conception

Rust commence fermé. Un item sans `pub` est visible dans son module et ses descendants, mais pas pour les modules frères ni pour le parent. Cette règle est plus restrictive que ce que beaucoup de programmeurs attendent après JavaScript, Python ou Go, où une fonction de fichier est généralement accessible dans le paquet. L'intention est de t'obliger à concevoir l'interface avant de dépendre d'un détail.

`pub` ouvre un item à quiconque peut atteindre le module qui le contient. `pub(crate)` ouvre l'item à tout le crate actuel, mais pas à quelqu'un qui importe la bibliothèque depuis un autre paquet. `pub(super)` ouvre l'item uniquement au module parent. Il existe aussi `pub(in ruta)`, utile quand une frontière précise de modules exprime une règle réelle, bien que moins courant dans les petits projets.

Ne marque pas tout avec `pub` pour faire taire les erreurs de visibilité. Cela a un coût : n'importe quel consommateur peut commencer à dépendre de ces noms, et changer ensuite une fonction interne devient une rupture d'API. Pour un binaire privé, ce coût reste dans le dépôt ; pour une bibliothèque publiée, il peut t'obliger à conserver une décision accidentelle pendant des années. Commence privé et n'ouvre que ce dont une autre partie a besoin.

Le compilateur distingue un nom qui n'existe pas d'un nom qui existe mais qui est fermé. Dans ce cas, la fonction existe, mais `main` essaie de traverser une frontière privée.

**Fig. 6.2** | Accéder à une fonction privée produit `E0603`.

```rust
// fig06_02.rs
mod config {
    fn ruta_por_omision() -> &'static str {
        "servicios.yaml"
    }
}

fn main() {
    println!("{}", config::ruta_por_omision());
}
```

```bash
$ rustc --edition 2024 fig06_02.rs
error[E0603]: function `ruta_por_omision` is private
 --> fig06_02.rs:9:28
  |
9 |     println!("{}", config::ruta_por_omision());
  |                            ^^^^^^^^^^^^^^^^ private function
  |
note: the function `ruta_por_omision` is defined here
 --> fig06_02.rs:3:5
  |
3 |     fn ruta_por_omision() -> &'static str {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

La correction mécanique serait d'écrire `pub fn ruta_por_omision`. Avant de le faire, demande-toi si le chemin par défaut doit faire partie du contrat de `config`. Si un autre module en a vraiment besoin, ce peut être une fonction publique raisonnable. Si tu veux seulement que `main` charge le fichier normal, il convient peut-être que `config` offre une fonction publique de plus haut niveau et garde cette chaîne comme détail privé.

Le module `modelo` du `revisor` montre une API publique sélectionnée. `Servicio` est public parce que la configuration, les tests d'intégration et d'autres modules ont besoin de le construire. Ses champs sont publics parce que le programme doit lire et modifier les données déclarées. La constante de timeout par défaut, en revanche, reste privée : qui utilise `Servicio::new` obtient la règle sans dépendre de la manière dont elle est stockée.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;

fn timeout_por_omision() -> u64 {
    TIMEOUT_POR_OMISION_MS
}

impl Servicio {
    /// Un servicio con el tiempo límite por omisión (5 segundos).
    pub fn new(nombre: &str, url: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            url: url.to_string(),
            timeout_ms: TIMEOUT_POR_OMISION_MS,
        }
    }
}
```

Observe la différence entre exposer une constante et exposer une fonction. `UMBRAL_LENTO_MS` est une règle dont d'autres modules ont bien besoin pour classer les réponses. `timeout_por_omision` n'existe que pour que `serde` puisse appeler la valeur par défaut à l'intérieur du modèle. La publier ne donne aucune capacité utile au consommateur et élargit la surface qu'il faudrait maintenir.

### Tests unitaires : une petite propriété, près du code

Un test unitaire vérifie une unité de comportement dans son propre module. Rust les écrit normalement dans un module `tests` marqué `#[cfg(test)]`. Cet attribut indique que le module n'est compilé qu'à la construction des tests. Le binaire de production ne charge ni ces fonctions ni leurs auxiliaires.

`use super::*` importe dans `tests` les noms du module parent. Cela permet de tester délibérément des détails privés. Ce n'est pas un piège contre la visibilité : le test vit comme descendant du même module et vérifie l'implémentation interne. Un test d'intégration aura une autre restriction, parce qu'il représente un consommateur externe.

Les assertions principales sont `assert!`, pour une condition booléenne ; `assert_eq!`, pour comparer l'attendu et l'obtenu ; et `assert_ne!`, pour affirmer que deux valeurs ne sont pas égales. Toutes acceptent un message supplémentaire avec formatage. `matches!` est particulièrement utile avec les enums : il permet de vérifier la variante et, si nécessaire, une condition sur les données qu'elle porte.

**Fig. 6.3** | Des tests unitaires, une assertion d'enum et une panique attendue.

```rust
// fig06_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Falla(String),
}

fn resumen(e: &Estado) -> String {
    match e {
        Estado::Ok { codigo, ms } => format!("OK {codigo} en {ms}ms"),
        Estado::Falla(msg) => format!("FALLA: {msg}"),
    }
}

fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("dividir por cero");
    }
    a / b
}

fn main() {
    println!("{}", resumen(&Estado::Ok { codigo: 200, ms: 100 }));
    println!("{}", dividir(10, 2));
}

#[cfg(test)]
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
    fn panico_esperado() {
        dividir(1, 0);
    }
}
```

```bash
$ rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1
running 3 tests
test tests::estado_ok_con_200 ... ok
test tests::falla_sin_codigo ... ok
test tests::panico_esperado - should panic ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`#[should_panic]` ne signifie pas que les paniques soient une façon normale de traiter des données invalides. Dans le `revisor`, une URL incorrecte doit se terminer en `Result` et en un message pour qui a invoqué le programme, pas en panique. Le test de panique sert quand un contrat arrête délibérément l'exécution devant un invariant rompu. L'argument `expected` compte : il confirme que la panique provient de la raison attendue et non d'un autre échec accidentel.

Un test doit décrire un comportement, pas une implémentation incidente. Le nom `falla_sin_codigo` communique une règle du domaine : un échec s'affiche sans code HTTP. Un nom comme `prueba_resumen_2` dit seulement que quelqu'un a écrit un test. Quand il échouera dans des mois, le nom sera le premier indice pour comprendre quelle décision du programme a changé.

Dans `config.rs`, les tests unitaires ne font pas de requêtes HTTP et n'exécutent pas le binaire. Ils construisent de petites données et appellent `validar`, qui est l'unité responsable de vérifier la liste. Cela garde la suite rapide et fait que chaque échec désigne une règle concrète.

<!-- verificar:fragmento -->
```rust
#[test]
fn validar_rechaza_nombre_repetido() {
    let v = vec![
        Servicio::new("a", concat!("http", "://x")),
        Servicio::new("a", concat!("http", "://y")),
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
    let mut s = Servicio::new("a", concat!("http", "://x"));
    s.timeout_ms = 0;
    assert!(validar(&[s]).is_err());
}
```

Le premier test inspecte une partie du message parce qu'ici le texte fait partie de l'expérience de qui corrige le YAML. Les autres vérifient seulement qu'il existe une erreur. Tous les tests n'ont pas à comparer des chaînes complètes. Compare le détail exact quand c'est un contrat public ; pour les erreurs internes, vérifier le type, une condition ou l'existence de l'erreur produit généralement des tests moins fragiles.

### Tests d'intégration : la bibliothèque vue de l'extérieur

Cargo reconnaît `tests/` comme l'endroit des tests d'intégration. Chaque fichier Rust directement dans ce dossier se compile comme un crate distinct. C'est pourquoi il ne peut pas utiliser de fonctions privées, ni importer le module interne `tests` de la bibliothèque, ni supposer des détails de fichiers. Il ne peut utiliser que ce que la bibliothèque exporte avec `pub`.

Cette limitation est utile. Une API peut avoir d'excellents tests unitaires et rester incommode ou insuffisante pour qui essaie de l'utiliser de l'extérieur. Les tests d'intégration trouvent ce problème parce qu'ils traversent la même limite que traverserait un autre binaire. Si tu dois briser l'encapsulation pour les écrire, vérifie d'abord s'il manque une opération publique raisonnable ; ne transforme pas tout en `pub` par réaction automatique.

Le `revisor` a un fichier `tests/integracion.rs`. Il importe les types et fonctions dont un consommateur public a besoin : `Estado`, `Servicio`, `revisar` et `revisar_todos`. Il n'importe ni les fonctions privées qui construisent les requêtes ni les détails de `reqwest`.

<!-- verificar:fragmento -->
```rust
use revisor::modelo::{Estado, Servicio};
use revisor::revisar::{revisar, revisar_todos};

fn servicio(nombre: &str, direccion: &str, ruta: &str, timeout_ms: u64) -> Servicio {
    Servicio {
        nombre: nombre.to_string(),
        url: ["http:", "//", direccion, ruta].concat(),
        timeout_ms,
    }
}
```

Le helper `servicio` appartient au test, pas à la bibliothèque, parce qu'il n'existe que pour rendre les cas de test lisibles. C'est une distinction saine : ne promeus pas une fonction en production uniquement parce que deux tests la répètent. La bibliothèque doit contenir des capacités du programme ; la suite peut contenir de petits outils pour préparer des scénarios.

Le test suivant lance un serveur HTTP local défini dans `tests/comun/mod.rs`, appelle l'API publique et vérifie la variante obtenue. Il ne dépend ni d'un vrai service sur internet, ni d'un compte, ni d'une heure précise. Cela évite qu'un échec réseau ne transforme un test déterministe en fausse alarme.

<!-- verificar:extracto:tests/integracion.rs -->
```rust
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
```

`#[tokio::test]` apparaît parce que la fonction `revisar` est asynchrone. La leçon 7 approfondit ce que signifie attendre un future et comment fonctionne le runtime. Ici, l'important est de reconnaître la frontière : le test d'intégration utilise la même API asynchrone que le binaire, mais remplace internet par un serveur local contrôlé.

Outre les tests d'intégration, le projet a des tests du binaire. Ceux-ci exécutent le `revisor` compilé, lui passent un YAML temporaire et examinent `stdout`, `stderr` et le code de sortie. Ils sont plus lents et plus larges qu'un test unitaire, donc ils ne remplacent pas les autres ; ils vérifient la dernière frontière, où arguments, configuration, rapports et sortie du processus se rencontrent.

### `cargo test` : construire, sélectionner et lire les résultats

`cargo test` découvre les tests unitaires, ceux d'intégration, ceux des binaires et ceux de documentation ; il compile les cibles nécessaires et exécute chaque ensemble. C'est plus qu'une abréviation de `rustc --test` : Cargo connaît les dépendances et construit chaque crate avec les bons chemins.

Les commandes que tu utiliseras le plus souvent sont celles-ci :

```bash
cargo test
cargo test validar_rechaza_nombre_repetido
cargo test --test integracion
cargo test -- --nocapture
cargo test --release
```

La première commande lance tout. La deuxième filtre par une partie du nom de test ; elle est utile pour travailler sur une seule règle sans attendre la suite entière. La troisième sélectionne spécifiquement le fichier d'intégration appelé `integracion`. Le `--` sépare les options de Cargo des options de l'exécuteur de tests : `--nocapture` permet de voir les `println!` d'un test qui réussit, utile pour un diagnostic temporaire, pas comme substitut d'une assertion. `--release` compile avec optimisations ; utilise-le quand le comportement dépend réellement du profil ou quand tu mesures la performance, pas comme mode quotidien.

Les tests peuvent s'exécuter en parallèle. C'est correct si chacun crée ses propres données et ne dépend pas de l'ordre d'exécution. Si tu diagnostiques une sortie ou si un test partage une ressource que tu ne peux pas encore isoler, utilise :

```bash
cargo test -- --test-threads=1
```

N'en fais pas une habitude. Une suite qui ne fonctionne qu'en série peut cacher un état global ou des fichiers temporaires aux noms qui entrent en collision. Dans le `revisor`, les serveurs de test demandent au système un port libre et chaque cas utilise ses propres données ; cela permet d'exécuter les tests sans dépendre d'un ordre particulier.

Le piège le plus simple est que `cargo test` peut se terminer correctement sans avoir exécuté un test pertinent. Un filtre mal écrit peut produire une sortie avec des tests filtrés ; un crate peut n'avoir aucun `#[test]` ; et un fichier placé hors de `tests/` peut ne pas être une intégration. Lis toujours les lignes `running N tests` et `test result`. Le code de sortie zéro signifie que l'exécuteur n'a pas trouvé d'échec, pas que ton intention a été vérifiée.

Le projet garde la logique dans la bibliothèque et le démarrage dans le binaire. Le binaire importe l'API publique comme n'importe quel autre consommateur interne. Cette séparation est la raison pour laquelle les tests d'intégration peuvent importer `revisor` avec le même nom.

<!-- verificar:extracto:src/main.rs -->
```rust
use std::process::ExitCode;

use revisor::{config, reporte, revisar};

use clap::Parser;
```

Le chemin `revisor::{config, reporte, revisar}` n'utilise pas `crate::` parce que `main.rs` est un autre crate dans le même paquet. Du point de vue du binaire, `revisor` est la bibliothèque déclarée par `src/lib.rs`. C'est une petite différence de syntaxe avec une conséquence de conception importante : le binaire n'a aucun privilège pour atteindre les détails privés de la bibliothèque.

### Dépendances, versions et le travail de `cargo`

`Cargo.toml` est le manifeste déclaratif du paquet. Il dit comment il s'appelle, quelle édition il utilise, de quelles dépendances directes il a besoin et quels profils de construction existent. `Cargo.lock` enregistre la résolution concrète : les versions exactes des dépendances directes et transitives que Cargo a choisies quand il a construit le projet.

Le `revisor` ne dépend pas seulement de la bibliothèque standard. C'est délibéré : YAML, HTTP asynchrone, JSON et une ligne de commande complète vivent dans des crates spécialisés. Les dépendances déclarées sont celles que le projet utilise réellement.

<!-- verificar:extracto:Cargo.toml -->
```toml
[dependencies]
anyhow = "1.0.104"
clap = { version = "4.6.7", features = ["derive"] }
futures = "0.3.34"
reqwest = { version = "0.13.5", features = ["json"] }
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
yaml_serde = "0.10.7"
tokio = { version = "1.53.1", features = ["full"] }
```

Une note sur l'une de ces lignes. Jusqu'en 2024, le crate le plus utilisé pour lire du YAML avec `serde` était `serde_yaml`. Son auteur, David Tolnay, a cessé de le maintenir : sa dernière version est la `0.9.34+deprecated`, de mars 2024, et crates.io la marque comme obsolète. Elle compile et fonctionne encore, mais ne reçoit plus de corrections ni d'améliorations, donc il ne convient pas de démarrer un nouveau projet avec elle. Le `revisor` utilise `yaml_serde`, une continuation publiée par l'organisation YAML sur GitHub : son dépôt la présente comme le fork maintenu de `serde_yaml` et promet la même interface. C'est pourquoi le changement ne touche presque pas le code : ce que tu sais de `serde_yaml::from_str` sert tout autant avec `yaml_serde::from_str`. Il existe d'autres forks et alternatives ; avant d'en choisir un, regarde la date de sa dernière version et si son dépôt continue de recevoir des changements. (Données consultées sur crates.io le 2 octobre 2026.)

Une version comme `"1.0.104"` ne fixe pas à elle seule chaque chiffre pour toujours. Dans Cargo, cette spécification utilise la compatibilité sémantique avec l'opérateur caret implicite : elle permet des mises à jour compatibles au sein de la même version majeure. `Cargo.lock` est ce qui rend reproductible la compilation concrète du binaire. C'est pourquoi le lockfile du `revisor` doit aller dans le dépôt : une personne qui clone l'application doit résoudre les mêmes versions connues, pas une nouvelle combinaison qui semble compatible aujourd'hui.

Pour une bibliothèque publiée, la réponse est moins tranchée. `cargo new` enregistre le `Cargo.lock` dans le dépôt par défaut, et la foire aux questions de Cargo (le [Cargo FAQ](https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control)) dit que le versionner ou non dépend de ce dont ton paquet a besoin. Le versionner donne des compilations reproductibles : cela aide à trouver avec `git bisect` quel changement a introduit une erreur, à faire échouer l'intégration continue seulement à cause de nouveaux commits et non d'une dépendance qui a changé à l'extérieur, et à vérifier avec des versions connues des choses comme la version minimale de Rust ou le texte exact des messages d'erreur. Mais ce fichier ne protège pas qui utilise ta bibliothèque : les consommateurs résolvent les dépendances avec ce que déclare ton `Cargo.toml` et avec leur propre `Cargo.lock`, et `cargo install` ignore par défaut le `Cargo.lock` du paquet et choisit les versions compatibles les plus récentes, sauf si tu lui passes `--locked`. En résumé : une application comme le `revisor` doit toujours être versionnée, parce que c'est le produit final que tu veux reproduire ; pour une bibliothèque, décide selon ce que tu veux garantir, et si tu ne la versionnes pas, teste de temps en temps avec les dépendances les plus récentes.

Ajoute une dépendance avec Cargo plutôt que d'écrire à la main une ligne que tu ne comprends pas :

```bash
cargo add serde --features derive
cargo add tokio --features full
cargo tree
cargo update
```

`cargo add` met à jour le manifeste et résout le lockfile. Les fonctionnalités (features) activent des parties optionnelles d'un crate. `serde` a besoin de `derive` pour que `#[derive(Serialize, Deserialize)]` existe ; `tokio` a besoin de capacités de runtime, de réseau et de macros pour le programme actuel. N'active pas `full` par réflexe dans un nouveau projet si tu n'as besoin que d'une petite partie ; ici c'est une décision consciente du cours pour que le revisor utilise les capacités qu'il enseigne.

`cargo tree` montre l'arbre complet. C'est la façon de découvrir les dépendances transitives : des crates que tu n'as pas ajoutés directement, mais qui sont arrivés parce qu'une autre dépendance en a besoin. Ce n'est pas nécessairement un signe de problème. C'est un outil pour répondre à « qui apporte cette version ? », « pourquoi compile-t-on autant de code ? » ou « pourquoi y a-t-il deux versions de ce crate ? ».

`cargo update` met à jour dans les restrictions que tu as écrites dans `Cargo.toml`. Cela n'équivaut pas à « installer la dernière version de tout » sans limites. Avant de mettre à jour un projet stable, examine ce qui a changé dans le lockfile, lance les tests et lis les notes de version quand une dépendance centrale change. La version déclarée définit la plage acceptable ; le lockfile enregistre la décision prise.

Pendant le développement, `cargo check` est généralement plus rapide que `cargo build` parce qu'il vérifie les types et les emprunts sans générer l'exécutable final. Il ne remplace pas les tests, mais réduit le temps de retour d'information pendant que tu modifies une fonction. Pour maintenir la qualité sur tout le projet, utilise cette séquence :

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo fmt --check` confirme le format sans modifier de fichiers. `cargo clippy --all-targets -- -D warnings` examine la bibliothèque, le binaire et les tests, et convertit ses avertissements en échecs pour que le projet n'accumule pas de dette connue. `cargo test` vérifie le comportement. Ce sont trois signaux distincts : format cohérent, usage idiomatique et comportement attendu.

## L'erreur que tu vas voir

### `E0603` : le nom existe, mais ne fait pas partie de l'API

`E0603` apparaît quand Rust a trouvé l'item que tu as nommé, mais que le chemin essaie de traverser une frontière privée. Le diagnostic de la figure 6.2 donne trois indices : il désigne l'usage illégal, dit que la fonction est privée et pointe vers l'endroit où elle a été déclarée. C'est différent d'une erreur de frappe comme « cette fonction est introuvable » ; ici Rust sait bien quelle fonction tu voulais utiliser.

La correction dépend de la conception. Si la fonction doit faire partie de l'interface, déclare-la `pub`. Si elle ne doit servir qu'à un module frère, envisage de déplacer l'opération vers un module propriétaire plus approprié ou d'exposer une fonction publique de niveau supérieur. Si tu dois la limiter au crate, `pub(crate)` exprime mieux que personne hors du paquet ne doit en dépendre.

Dans le `revisor`, les auxiliaires de `reporte.rs`, comme `etiqueta`, `tiempo` et `detalle`, restent privés. L'API publique est `tabla` et `json`, parce que ce sont les opérations que le binaire et tout consommateur peuvent demander. Ce choix empêche que des tests d'intégration ou du code futur dépendent d'une étiquette interne et figent une décision de format accidentelle.

### Un test rouge n'est pas une erreur de compilation

Quand une assertion échoue, Cargo compile correctement puis l'exécuteur marque le test comme échoué. Il n'y aura pas de code `EXXXX`, parce que ce n'est pas une violation des règles statiques du compilateur. Tu verras le nom du test, la valeur gauche et droite si tu as utilisé `assert_eq!`, et tout message supplémentaire que tu as ajouté.

C'est une différence de diagnostic importante. Les erreurs de `rustc` te disent que le programme ne peut pas être construit selon les règles du langage. Un test rouge te dit que le programme a été construit, mais qu'il a cassé le comportement que tu avais déclaré. Ne répare pas un test rouge en retirant l'assertion ni en changeant la valeur attendue sans examiner quel contrat devait être maintenu.

Provoque un échec exprès une fois. Dans le test `falla_sin_codigo`, change temporairement le texte attendu en `"OK: x"` et lance le filtre correspondant. Tu dois voir le test devenir rouge. Ensuite, restaure le comportement correct. Un test que tu n'as jamais vu échouer peut couvrir une autre branche que celle que tu crois, ou affirmer quelque chose de trop faible pour détecter une régression.

## Ce qui se fait mal

### Tout rendre `pub`

Ouvrir chaque struct, champ et fonction commence souvent comme une façon rapide de vaincre `E0603`. Le résultat est une bibliothèque sans frontières : n'importe quel module peut s'appuyer sur des détails internes et chaque changement exige de revoir beaucoup plus d'appels que nécessaire. Publie des opérations qui représentent des capacités du domaine, pas chaque étape auxiliaire avec laquelle tu les implémentes.

L'alternative pratique n'est pas de deviner l'API parfaite dès le premier jour. Garde privé ce qui n'a pas encore de consommateur clair. Quand un autre module a besoin d'une opération, ouvre l'interface minimale et laisse cet usage réel guider la conception.

### Organiser par des noms vagues comme `utils` ou `helpers`

Un dossier appelé `utils` ne décrit pas une responsabilité ; il décrit que quelqu'un n'a pas su où mettre quelque chose. Avec le temps, il accumule conversion de texte, accès aux fichiers, format, HTTP et fonctions que personne n'ose déplacer. Chercher du code devient plus lent et la dépendance entre modules devient arbitraire.

Dans le `revisor`, une règle de YAML vit dans `config`, une traduction d'état vit dans `reporte` et le vocabulaire du domaine vit dans `modelo`. Si une fonction ne tient dans aucun module, demande-toi d'abord s'il manque un concept avec un nom propre. Souvent, le nouveau nom révèle une responsabilité qui était mélangée.

### Ne tester que le chemin sain

Un test qui vérifie `200 OK` est nécessaire, mais ne suffit pas pour un vérificateur de services. Un HTTP 500, une URL invalide, un fichier manquant, un délai d'attente, une liste vide et un format inconnu doivent aussi être couverts. Les erreurs ne sont pas des exceptions improbables dans ce domaine : elles font partie de ce que le programme existe pour rapporter.

Ne convertis pas chaque échec externe en un test de réseau réel. Le `revisor` utilise un faux serveur local pour reproduire des réponses connues. Ainsi il teste son propre comportement, pas la disponibilité d'un service tiers.

### Utiliser `unwrap()` pour écrire des tests plus courts

`unwrap()` est raisonnable pour préparer des données que le test contrôle lui-même, comme un YAML littéral qui doit être valide. Si ce YAML échoue, le test est mal construit et s'arrêter est correct. Ne l'utilise pas sur le résultat que tu essaies de tester. Si tu veux démontrer que `validar` rejette une entrée, utilise `is_err`, `unwrap_err` ou `matches!` selon le contrat.

La règle est de distinguer préparation et vérification. Dans la préparation, un `expect("el YAML de la prueba es válido")` donne un contexte utile. Dans la vérification, une assertion exprime exactement la propriété que tu veux maintenir.

### Se fier au code de sortie de `cargo test` sans lire le décompte

Un filtre sans correspondance peut renvoyer un succès parce qu'aucun test n'a échoué. Un nouveau crate peut compiler sans tests. Une intégration mal placée peut ne pas être découverte. Lis `running N tests`, les noms qui apparaissent et le résumé final. Le résultat utile n'est pas seulement « ça a rendu zéro » ; c'est « le test que j'attendais s'est exécuté et a réussi ».

### Mettre à jour les dépendances sans examiner le lockfile

`cargo update` peut changer plusieurs dépendances transitives même si tu n'as demandé qu'une mise à jour. Cela ne le rend pas dangereux en soi, mais exige une révision. Regarde le changement dans `Cargo.lock`, comprends quels crates ont été mis à jour et lance la suite complète. Une version compatible en théorie peut révéler une hypothèse fragile ou changer sensiblement les temps de compilation.

## Exercices

### Exercice 1 — Divise un rapport sans trop ouvrir

Crée un programme avec un module `reporte`. Il doit exposer une fonction publique `resumen(nombre, sano)` qui renvoie une `String` avec le nom et l'étiquette `OK` ou `FALLA`. La fonction qui décide de l'étiquette doit rester privée. Depuis `main`, affiche deux lignes : une saine et une échouée.

### Exercice 2 — Teste toutes les variantes de l'état

Écris une fonction `es_sano(&Estado) -> bool` pour les quatre variantes de l'`Estado` du revisor : `Ok`, `Lento`, `Falla` et `NoIntentado`. Ajoute un test par variante. Utilise `assert!` ou `assert!(!...)` et nomme chaque test d'après la règle qu'il vérifie.

### Exercice 3 — Une intégration qui ne connaît pas les détails internes

Dans `programas/revisor`, lis `tests/integracion.rs`. Ajoute un test d'intégration qui utilise exclusivement `revisor::modelo` et `revisor::revisar`. Il doit utiliser le serveur local partagé et vérifier que `revisar_todos` renvoie le même nombre d'états que de services, même quand l'un reçoit un HTTP 500. Écris-le avant de lire les tests que `tests/integracion.rs` contient déjà, puis compare : qu'est-ce que le tien vérifie que les autres ne vérifient pas ?

### Exercice 4 — Fais un test rouge et remets-le au vert

Choisis un test existant de `config.rs` ou `reporte.rs`. Change temporairement une attente pour qu'il échoue, exécute uniquement ce test avec `cargo test nombre_de_la_prueba`, lis le diagnostic et restaure le comportement correct. Enfin, exécute `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` et `cargo test`.

## Solutions

### Solution 1

<!-- verificar:fragmento -->
```rust
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn resumen(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::resumen("catalogo", true));
    println!("{}", reporte::resumen("pagos", false));
}
```

`etiqueta` n'a pas besoin de `pub` parce que seule `resumen` l'utilise. La fonction publique fournit le résultat dont `main` a besoin, pas le détail intermédiaire.

### Solution 2

<!-- verificar:fragmento -->
```rust
#[derive(Debug)]
enum Estado {
    Ok,
    Lento,
    Falla,
    NoIntentado,
}

fn es_sano(estado: &Estado) -> bool {
    matches!(estado, Estado::Ok | Estado::Lento)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_es_sano() {
        assert!(es_sano(&Estado::Ok));
    }

    #[test]
    fn lento_es_sano() {
        assert!(es_sano(&Estado::Lento));
    }

    #[test]
    fn falla_no_es_sana() {
        assert!(!es_sano(&Estado::Falla));
    }

    #[test]
    fn no_intentado_no_es_sano() {
        assert!(!es_sano(&Estado::NoIntentado));
    }
}
```

Les quatre tests ne sont pas redondants. La fonction contient deux groupes de variantes et chacune exprime une décision du domaine. Si quelqu'un change `matches!` de façon incomplète, au moins un test identifie quel état a perdu son sens.

### Solution 3

<!-- verificar:fragmento -->
```rust
#[tokio::test]
async fn revisar_todos_conserva_un_estado_por_servicio() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let servicios = vec![
        servicio("bien", &d, "/ok", 2000),
        servicio("mal", &d, "/error", 2000),
    ];

    let estados = revisar_todos(&cliente, &servicios, 2).await;

    assert_eq!(estados.len(), servicios.len());
    assert!(estados[0].esta_bien());
    assert!(!estados[1].esta_bien());
}
```

Le test n'utilise que des types et des fonctions publics du `revisor`. Le helper local construit les services ; le serveur partagé contrôle les réponses. Il n'exige d'ouvrir aucune fonction privée du client HTTP.

### Solution 4

Exécute d'abord un test concret, par exemple :

```bash
cargo test validar_rechaza_timeout_cero
```

Change temporairement `assert!(validar(&[s]).is_err())` en `assert!(validar(&[s]).is_ok())`. Le test doit échouer. Restaure `is_err()` et termine avec :

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

La solution n'est pas de conserver le changement qui fait passer le test ; c'est de vérifier que la suite détecte une modification qui casse la règle, puis de récupérer la règle correcte.

## Comment savoir que j'y suis arrivé

- `rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1` affiche `running 3 tests` et se termine par `3 passed; 0 failed`.
- `cargo test validar_rechaza_nombre_repetido` se termine avec le test `config::tests::validar_rechaza_nombre_repetido ... ok`.
- `cargo test --test integracion` exécute les tests qui n'importent que l'API publique de la bibliothèque.
- `cargo fmt --check` se termine sans changements de format en attente.
- `cargo clippy --all-targets -- -D warnings` se termine sans avertissements.
- `cargo test` se termine avec des résultats `ok` pour la bibliothèque, le binaire et les tests d'intégration.
- Tu peux expliquer pourquoi `main.rs` importe `revisor::{config, reporte, revisar}` et non des détails privés de `src/lib.rs`.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 7 : Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) — consulté le 2 octobre 2026.
- [The Rust Programming Language, chapitre 11 : Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html) — consulté le 2 octobre 2026.
- [The Rust Programming Language, chapitre 14 : More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) — consulté le 2 octobre 2026.
- [Référence officielle de Cargo : spécifier les dépendances](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) — consulté le 2 octobre 2026.
