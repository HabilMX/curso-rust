# Leçon 8 — Le programme terminé

**Durée :** 2 × 45 min.

**Ce que tu construis :** le `revisor` complet, en un seul binaire

**Ce que tu apprends :** `reqwest`, `serde`, `clap`, le profil de release, le binaire final et sa comparaison avec celui de Go

## À la fin, tu vas pouvoir

- Expliquer quelle responsabilité a `main.rs` et pourquoi la logique réutilisable vit dans la bibliothèque `revisor`.
- Lire une configuration YAML avec `serde`, y compris les valeurs par défaut et les validations que YAML ne peut pas exprimer.
- Utiliser `clap` pour déclarer des options de ligne de commande avec types, valeurs par défaut et aide automatique.
- Expliquer pourquoi une panne HTTP est un résultat du domaine du `revisor`, alors qu'un fichier de configuration invalide empêche de démarrer.
- Compiler, tester et vérifier le binaire avec `cargo test`, `cargo clippy` et `cargo build --release`.
- Comparer, avec des arguments concrets, le binaire final de Rust et le programme équivalent de Go.

## Le pourquoi avant le comment

Au cours des leçons précédentes, tu as construit les pièces du `revisor` : le vocabulaire du problème, la lecture du YAML, le rapport, les tests et les requêtes HTTP concurrentes. Une pièce isolée peut être bien écrite et, malgré cela, ne pas être un outil qu'une autre personne puisse utiliser. Il manque de les unir dans une frontière claire : un exécutable qui reçoit des arguments, lit un fichier, interroge des services, affiche un résultat utile et se termine avec un code qu'un autre programme peut interpréter.

Cela ressemble à une fine couche, mais c'est là que se rencontrent des décisions importantes. La ligne de commande est une interface publique : si aujourd'hui tu acceptes `--formato json`, quelqu'un peut l'intégrer dans un script et en dépendre demain. Le fichier YAML est aussi une interface : ce n'est pas du code Rust, donc il peut contenir des noms répétés, des URL incomplètes ou un temps limite de zéro. Le réseau est une autre frontière : une réponse HTTP 500 ne signifie pas que le `revisor` lui-même soit cassé ; elle signifie que le service vérifié est en mauvais état. En revanche, ne pas pouvoir lire le YAML empêche bien de commencer à travailler.

Le programme terminé doit distinguer ces cas sans les cacher sous un même `unwrap()`. Si tout est sain, il se termine avec le code 0. S'il a pu vérifier et détecté un échec, il affiche le rapport et se termine avec le code 1. S'il n'a pas pu démarrer parce que les arguments ou la configuration sont invalides, il signale le problème sur la sortie d'erreur et se termine avec le code 2. Cette séparation rend le binaire utile aussi bien pour une personne qui l'exécute dans un terminal que pour un système d'automatisation.

Cette leçon continue le même programme que tu as fait en Go. La comparaison compte parce que les deux langages arrivent à un binaire distribuable, mais par des chemins différents. Go inclut HTTP, JSON, drapeaux et concurrence dans sa bibliothèque standard. Rust laisse la bibliothèque standard petite et stable ; pour du HTTP asynchrone, de la sérialisation YAML ou une interface de ligne de commande déclarative, il utilise des crates de l'écosystème. Ce n'est pas un avantage automatique d'un côté ni un manque automatique de l'autre. C'est une décision de répartition du travail entre le langage, le gestionnaire de paquets et les bibliothèques.

Le projet `programas/revisor/` fixe les versions concrètes avec `Cargo.toml` et `Cargo.lock`. `cargo` résout l'arbre complet des dépendances et le lockfile conserve la résolution exacte pour qu'un autre ordinateur construise le même ensemble de crates compatibles. Ne copie pas à l'aveugle des versions d'un vieux tutoriel et n'exécute pas `cargo update` comme une réaction automatique : comprends d'abord ce qui a changé, examine le lockfile et lance les tests complets.

The Rust Book, chapitre 12, est la lecture centrale de cette leçon : il organise un programme de ligne de commande autour de l'entrée, de la configuration, des erreurs et de la séparation des responsabilités. Reprends aussi les exercices de Rustlings qui t'ont coûté le plus sur `Result`, les tests et les itérateurs. L'objectif n'est plus de mémoriser la syntaxe ; c'est de voir comment les décisions des leçons précédentes survivent quand le programme a de vrais utilisateurs, de vrais fichiers et un vrai réseau.

## Les concepts

### Le binaire est une frontière, pas l'endroit de toute la logique

Un binaire a une tâche précise : traduire l'extérieur vers l'intérieur du programme. Il lit les arguments du système d'exploitation, décide ce qui est affiché, convertit le résultat général en code de sortie et délègue le reste. Si tu y mélanges le chargement du YAML, les requêtes HTTP, le format de tableau et le code de sortie, les tests finissent par dépendre du terminal et de `std::process::exit`. Cela rend un petit test lent, fragile et difficile à diagnostiquer.

L'alternative du `revisor` est d'avoir une bibliothèque avec des modules publics (`config`, `modelo`, `reporte` et `revisar`) et un `main.rs` court. La bibliothèque peut être testée depuis ses tests unitaires et depuis `tests/integracion.rs`. Le binaire ne garde que ce qui dépend nécessairement de la ligne de commande. Cette division n'est pas une règle cérémonielle : elle réduit le nombre d'endroits où un changement d'interface peut casser le comportement.

Avant d'utiliser `clap`, il convient de comprendre le travail qu'il automatise. Un analyseur d'arguments doit porter un état : chaque drapeau consomme, le cas échéant, la valeur qui suit ; il doit conserver les valeurs par défaut ; et il doit rejeter un drapeau inconnu au lieu de l'interpréter silencieusement. Le programme suivant simule cette frontière sans dépendre de crates.

**Fig. 8.1** | Un analyseur minimal conserve les valeurs par défaut et convertit le texte en types.

```rust
// fig08_01.rs
struct Args {
    archivo: String,
    formato: String,
    paralelo: usize,
}

fn leer(args: &[&str]) -> Result<Args, String> {
    let mut resultado = Args {
        archivo: "servicios.yaml".to_string(),
        formato: "tabla".to_string(),
        paralelo: 5,
    };
    let mut i = 0;

    while i < args.len() {
        match args[i] {
            "-a" | "--archivo" => {
                i += 1;
                resultado.archivo = args.get(i).ok_or("falta archivo")?.to_string();
            }
            "-f" | "--formato" => {
                i += 1;
                resultado.formato = args.get(i).ok_or("falta formato")?.to_string();
            }
            "-p" | "--paralelo" => {
                i += 1;
                resultado.paralelo = args
                    .get(i)
                    .ok_or("falta paralelo")?
                    .parse()
                    .map_err(|_| "paralelo no es un número")?;
            }
            otro => return Err(format!("argumento desconocido: {otro}")),
        }
        i += 1;
    }

    Ok(resultado)
}

fn main() {
    let a = leer(&["--archivo", "demo.yaml", "-f", "json", "-p", "2"]).unwrap();
    println!("{} | {} | {}", a.archivo, a.formato, a.paralelo);
}
```

```bash
$ rustc --edition 2024 fig08_01.rs && ./fig08_01
demo.yaml | json | 2
```

Le programme illustre pourquoi il n'est pas conseillé d'écrire cet analyseur à la main pour chaque binaire. Il ne génère pas `--help`, ne documente pas ses options de lui-même, ne valide pas les valeurs autorisées et son code grossirait vite. `clap` déclare le contrat et génère une grande partie de cette mécanique. Dans le `revisor`, `Args` est le contrat d'entrée : `archivo`, `formato` et `paralelo` ont un nom long, un nom court, un type et une valeur par défaut.

La figure 8.1 a monté à la main l'analyseur d'arguments. Voici maintenant le même contrat avec `clap`, le crate qu'utilise le `revisor`. Comme les autres exemples avec dépendances, il vit dans `programas/revisor/examples/` et s'exécute avec Cargo depuis ce dossier.

**Exemple cargo avec `clap`** | Le même contrat d'arguments, déclaré au lieu d'être programmé.

<!-- verificar:ejemplo:ejemplo_clap -->
```rust
// ejemplo_clap.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}

fn main() {
    // `try_parse_from` recibe los argumentos como una lista, en vez de leer
    // los del sistema operativo: así el ejemplo corre igual en cualquier máquina.
    let a = Args::try_parse_from(["revisor", "--formato", "json", "-p", "2"])
        .expect("estos argumentos son válidos");
    println!("{a:?}");

    // Un valor que no es número: clap lo rechaza antes de que el programa arranque.
    if let Err(e) = Args::try_parse_from(["revisor", "--paralelo", "muchos"]) {
        println!("{:?}", e.kind());
        println!("{}", e.to_string().lines().next().unwrap_or(""));
    }

    // Una opción que no existe.
    if let Err(e) = Args::try_parse_from(["revisor", "--velocidad", "3"]) {
        println!("{:?}", e.kind());
    }
}
```

```bash
$ cargo run --example ejemplo_clap
Args { archivo: "servicios.yaml", formato: "json", paralelo: 2 }
ValueValidation
error: invalid value 'muchos' for '--paralelo <PARALELO>': invalid digit found in string
UnknownArgument
```

`#[derive(Parser)]` écrit pour toi le code qui convertit la liste d'arguments en un `Args`. Dans le vrai programme, on appelle `Args::parse()`, qui lit les arguments du système d'exploitation ; ici on utilise `try_parse_from`, qui reçoit la liste en paramètre, pour que l'exemple donne toujours la même sortie et pour pouvoir voir les erreurs sans terminer le programme. La première ligne montre un `Args` complet : ce qui a été passé (`formato` et `paralelo`) et la valeur par défaut de ce qui manquait (`archivo`). Les suivantes montrent ce qui arrive avec une valeur qui n'est pas un nombre et avec une option qui n'existe pas : `clap` les rejette avec un type d'erreur clair (`ValueValidation` et `UnknownArgument`) avant que la logique du programme ne démarre. Quand tu utilises `parse()` au lieu de `try_parse_from`, `clap` affiche ce message avec l'aide d'utilisation et se termine avec le code 2, le même code que le `revisor` réserve à « je n'ai pas pu démarrer ». De plus, `--help` et `--version` sortent gratuitement de l'attribut `#[command(version, about = ...)]`.

Voici la déclaration complète à l'intérieur du `revisor` :

<!-- verificar:extracto:src/main.rs -->
```rust
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}
```

L'attribut `#[derive(Parser)]` génère une implémentation du trait `Parser`. C'est pourquoi `Args::parse()` peut lire `std::env::args()` et construire un `Args` typé. `default_value` reçoit du texte parce que `clap` le convertit vers le type du champ ; `default_value_t` reçoit directement une valeur Rust, c'est pourquoi il convient pour `usize`.

Le `main` du projet n'appelle pas `std::process::exit`. Il renvoie `ExitCode`, plus facile à tester quand la logique reste dans `ejecutar`. Le code vérifie d'abord que le format est l'un des deux contrats promis. Ensuite, il charge et valide la configuration, crée un client HTTP, demande les états, décide s'il affiche un tableau ou du JSON et convertit le résultat final en 0, 1 ou 2. L'ordre compte : il n'y a aucune raison d'ouvrir des connexions réseau si le fichier YAML n'est même pas lisible.

<!-- verificar:extracto:src/main.rs -->
```rust
#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    ejecutar(&args).await
}

async fn ejecutar(args: &Args) -> ExitCode {
    if args.formato != "tabla" && args.formato != "json" {
        eprintln!(
            "revisor: --formato debe ser «tabla» o «json», no «{}»",
            args.formato
        );
        return ExitCode::from(2);
    }

    let servicios =
        match config::cargar(&args.archivo).and_then(|s| config::validar(&s).map(|()| s)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("revisor: {e:#}");
                return ExitCode::from(2);
            }
        };

    let cliente = reqwest::Client::new();
    let estados = revisar::revisar_todos(&cliente, &servicios, args.paralelo).await;

    if args.formato == "json" {
        match reporte::json(&servicios, &estados) {
            Ok(j) => println!("{j}"),
            Err(e) => {
                eprintln!("revisor: escribiendo el reporte: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        print!("{}", reporte::tabla(&servicios, &estados));
    }

    if estados.iter().all(|e| e.esta_bien()) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
```

L'annotation `#[tokio::main]` crée et démarre le runtime nécessaire pour pouvoir utiliser `.await` dans la fonction principale. Elle ne rend pas à elle seule tout le programme plus rapide. Son but est d'exécuter des futures pendant que les requêtes HTTP attendent des données du réseau, sans bloquer un thread par attente.

### `serde` rend explicite le contrat de données

Un fichier YAML arrive au programme sous forme de texte. Le texte ne sait pas ce qu'est un nom, quel champ est obligatoire ni quelle valeur utiliser quand `timeout_ms` manque. Le convertir en `Vec<Servicio>` revient à passer de données qui viennent de l'extérieur à des valeurs que le compilateur peut vérifier. Cette conversion est une limite de confiance : après la désérialisation, tu dois encore valider les règles métier que le format ne connaît pas.

`serde` sépare deux directions. `Deserialize` construit des valeurs Rust depuis YAML, JSON, TOML ou un autre format qui a un adaptateur compatible. `Serialize` convertit des valeurs Rust en un format de sortie. La structure du domaine est conservée ; c'est le format qui l'entoure qui change. C'est pourquoi le même `EstadoJson` peut être généré avec `serde_json`, tandis que `Servicio` entre avec `yaml_serde`.

Le programme suivant montre une décision du domaine qui apparaît aussi dans le JSON du `revisor` : un état sain a un code HTTP et ne porte pas d'erreur ; un échec n'invente pas de code et inclut bien un motif. Le programme assemble le JSON manuellement pour que la différence se voie. Dans le vrai projet, tu ne dois pas faire cela à la main : `serde_json` se charge d'échapper le texte et de préserver un JSON valide.

**Fig. 8.2** | La forme de la sortie dépend de la variante de l'état.

```rust
// fig08_02.rs
enum Estado {
    Ok(u16),
    Falla(&'static str),
}

fn json(nombre: &str, estado: &Estado, ms: u64) -> String {
    match estado {
        Estado::Ok(codigo) => {
            format!("{{\"servicio\":\"{nombre}\",\"codigo\":{codigo},\"ms\":{ms}}}")
        }
        Estado::Falla(motivo) => {
            format!(
                "{{\"servicio\":\"{nombre}\",\"codigo\":null,\"ms\":{ms},\"error\":\"{motivo}\"}}"
            )
        }
    }
}

fn main() {
    println!("{}", json("catalogo", &Estado::Ok(200), 7));
    println!("{}", json("pagos", &Estado::Falla("codigo 500"), 12));
}
```

```bash
$ rustc --edition 2024 fig08_02.rs && ./fig08_02
{"servicio":"catalogo","codigo":200,"ms":7}
{"servicio":"pagos","codigo":null,"ms":12,"error":"codigo 500"}
```

Dans le `revisor`, le derive déclare exactement ce qu'il faut lire ou écrire. `Servicio` dérive `Deserialize` parce qu'il est créé depuis le YAML. `EstadoJson` dérive `Serialize` parce qu'il est créé depuis les résultats internes pour produire du JSON. `#[serde(default = "timeout_por_omision")]` n'équivaut pas à ce que le champ soit optionnel en Rust : le champ final reste un `u64` ; il n'obtient une valeur que lorsque le YAML ne le déclare pas.

Le programme suivant utilise en petit les deux crates de données du `revisor` : `serde` pour déclarer le contrat, et `yaml_serde` et `serde_json` pour les formats. Les données entrent sous forme d'un texte YAML écrit dans le programme lui-même, pour ne dépendre d'aucun fichier.

**Exemple cargo avec `serde`** | La même `struct` lit du YAML et écrit du JSON.

<!-- verificar:ejemplo:ejemplo_serde -->
```rust
// ejemplo_serde.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Servicio {
    nombre: String,
    url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    timeout_ms: u64,
}

fn timeout_por_omision() -> u64 {
    5000
}

fn main() -> Result<(), yaml_serde::Error> {
    let yaml = "\
- nombre: catalogo
  url: http://localhost:8080
- nombre: pagos
  url: http://localhost:8081
  timeout_ms: 250
";
    // YAML adentro: el mismo derive sirve para leer...
    let servicios: Vec<Servicio> = yaml_serde::from_str(yaml)?;
    for s in &servicios {
        println!("{} espera {} ms", s.nombre, s.timeout_ms);
    }

    // ...y para escribir JSON, que es otro formato con el mismo modelo.
    match serde_json::to_string(&servicios) {
        Ok(json) => println!("{json}"),
        Err(e) => println!("no se pudo escribir el JSON: {e}"),
    }

    // Un servicio sin `url` no es un Servicio: el error dice qué falta y dónde.
    let roto: Result<Vec<Servicio>, _> = yaml_serde::from_str("- nombre: sin-url\n");
    if let Err(e) = roto {
        println!("YAML inválido: {e}");
    }
    Ok(())
}
```

```bash
$ cargo run --example ejemplo_serde
catalogo espera 5000 ms
pagos espera 250 ms
[{"nombre":"catalogo","url":"http://localhost:8080","timeout_ms":5000},{"nombre":"pagos","url":"http://localhost:8081","timeout_ms":250}]
YAML inválido: .[0]: missing field `url` at line 1 column 3
```

La même `struct Servicio` sert à lire et à écrire parce qu'elle dérive les deux moitiés de `serde` : `Deserialize` pour la construire depuis le YAML et `Serialize` pour l'écrire en JSON (le `Servicio` du `revisor` ne dérive que `Deserialize`, parce qu'il n'est jamais réécrit). Remarque deux détails. Le service `catalogo` ne déclare pas `timeout_ms` dans le YAML et sort pourtant avec 5000 : c'est l'effet de `#[serde(default = ...)]`. Et le YAML invalide ne fait pas tomber le programme avec une panique : `from_str` renvoie un `Err` dont le message dit ce qui manque (``missing field `url` ``) et où (`.[0]` est le premier élément de la liste ; `line 1 column 3`, la position dans le texte). Ce message est celui que le `revisor` montre à qui corrige son fichier. Et comme `yaml_serde` est la continuation maintenue de `serde_yaml` (le crate d'origine ne reçoit plus de changements, comme tu l'as vu à la leçon 6), tout ce qui se fait ici avec `from_str` fonctionne de la même façon avec l'un ou l'autre nom.

<!-- verificar:extracto:src/modelo.rs -->
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}

#[derive(Serialize)]
pub struct EstadoJson {
    pub servicio: String,
    pub codigo: Option<u16>,
    pub ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

`Option<u16>` représente une différence importante : s'il n'y a pas eu de réponse HTTP, il n'existe pas de code HTTP. Utiliser `0` ou une chaîne vide inventerait une donnée et obligerait tout consommateur à se souvenir de cette convention. En JSON, `Option::None` devient `null` pour `codigo`. Pour `error`, l'attribut `skip_serializing_if` choisit une autre politique : quand il n'y a pas d'erreur, la clé n'apparaît même pas. Les deux décisions sont valides, mais elles doivent être intentionnelles et couvertes par des tests.

Bien désérialiser ne suffit pas. YAML peut exprimer une liste vide, répéter un nom, accepter une chaîne comme URL ou inclure un temps limite de zéro. L'analyseur ne sait pas que ces options rendent le `revisor` inutile. `config::validar` est la seconde étape et exprime les règles du programme, pas celles du format.

<!-- verificar:extracto:src/config.rs -->
```rust
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
```

`anyhow::ensure!` renvoie tôt une erreur quand une condition n'est pas remplie. Ici, c'est adéquat parce qu'une configuration invalide ne représente pas un service échoué : c'est une condition qui empêche de lancer la vérification. `with_context` dans `cargar` ajoute le nom du fichier à une erreur de lecture ; le format `{e:#}` dans `main` affiche cette chaîne de contexte avec la cause d'origine. Le résultat est plus utile qu'un message générique de type « impossible d'ouvrir ».

### `reqwest` convertit le réseau en états du domaine

Un programme de supervision n'interroge pas HTTP pour obtenir un corps et l'oublier ; il interroge pour classer l'état d'un service. C'est pourquoi la fonction `revisar` ne renvoie pas `Result<Estado>`. Si un serveur répond 500, refuse la connexion ou dépasse son temps limite, l'opération du `revisor` s'est bien terminée : elle a découvert un échec et doit l'inclure dans le rapport. Ces cas se convertissent en `Estado::Falla`.

Ne confonds pas ce résultat avec un échec de configuration ou avec une erreur de sérialisation en produisant le rapport. Ceux-là empêchent bien le programme de remplir son travail et le font se terminer avec le code 2. La distinction évite deux erreurs fréquentes : arrêter toute la vérification parce qu'un service est tombé, ou continuer comme si de rien n'était alors qu'on n'a pas pu lire le fichier qui définit quels services existent.

Pour voir `reqwest` sans dépendre d'aucun vrai service, le programme suivant lance sur la même machine un faux serveur et l'interroge avec un client `reqwest`. Le serveur est monté avec `TcpListener`, de la bibliothèque standard, et répond à la main le texte minimal de HTTP : il répond `200` sur `/sano`, `500` sur `/roto` et met une demi-seconde à répondre à `/lento`. De plus, le programme essaie de se connecter à un port où personne n'écoute.

**Exemple cargo avec `reqwest`** | Quatre réponses différentes du réseau, vues depuis le client.

<!-- verificar:ejemplo:ejemplo_reqwest -->
```rust
// ejemplo_reqwest.rs
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

/// Un servidor HTTP de mentira, en esta misma máquina: /sano responde 200,
/// /roto responde 500 y /lento tarda medio segundo en contestar.
fn servidor_de_mentira() -> String {
    let escucha = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = escucha.local_addr().expect("el servidor tiene dirección");
    thread::spawn(move || {
        for conexion in escucha.incoming().flatten() {
            thread::spawn(move || atender(conexion));
        }
    });
    format!("http://{direccion}")
}

fn atender(mut conexion: std::net::TcpStream) {
    let mut pedido = [0u8; 1024];
    let n = conexion.read(&mut pedido).unwrap_or(0);
    let texto = String::from_utf8_lossy(&pedido[..n]);
    let ruta = texto.split_whitespace().nth(1).unwrap_or("/");
    let estado = match ruta {
        "/sano" => "200 OK",
        "/roto" => "500 Internal Server Error",
        "/lento" => {
            thread::sleep(Duration::from_millis(500));
            "200 OK"
        }
        _ => "404 Not Found",
    };
    let _ = write!(
        conexion,
        "HTTP/1.1 {estado}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
}

#[tokio::main]
async fn main() {
    let base = servidor_de_mentira();
    let cliente = reqwest::Client::new();

    for ruta in ["sano", "roto", "lento"] {
        let respuesta = cliente
            .get(format!("{base}/{ruta}"))
            .timeout(Duration::from_millis(200))
            .send()
            .await;
        match respuesta {
            Ok(r) => println!("{ruta:<6} responde {}", r.status().as_u16()),
            Err(e) if e.is_timeout() => println!("{ruta:<6} se acabó el tiempo de espera"),
            Err(_) => println!("{ruta:<6} no responde"),
        }
    }

    // Un puerto donde nadie escucha: la conexión misma falla.
    let vacio = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = vacio.local_addr().expect("tiene dirección");
    drop(vacio);
    match cliente.get(format!("http://{direccion}/")).send().await {
        Ok(r) => println!("vacío  responde {}", r.status().as_u16()),
        Err(e) if e.is_connect() => println!("vacío  no responde (rechazó la conexión)"),
        Err(e) => println!("vacío  falló de otra forma: {e}"),
    }
}
```

```bash
$ cargo run --example ejemplo_reqwest
sano   responde 200
roto   responde 500
lento  se acabó el tiempo de espera
vacío  no responde (rechazó la conexión)
```

Chaque ligne de la sortie est l'un des cas que le `revisor` convertit en un état du domaine. Un `500` n'est pas une erreur de `reqwest` : la requête a été faite et le serveur a répondu, donc `send().await` renvoie `Ok` avec le code `500`, et c'est le programme qui décide ce que cela signifie. En revanche, un temps écoulé (`.timeout(...)` de 200 ms contre un serveur qui met 500) et une connexion refusée arrivent bien comme `Err`, et l'erreur elle-même dit de quel cas il s'agit avec `is_timeout()` et `is_connect()`. Le client est créé une seule fois avec `reqwest::Client::new()` et réutilisé à chaque requête ; le temps limite, lui, se fixe par requête. La fonction `revisar` du vrai projet est cette même idée, avec les états `Ok`, `Lento` et `Falla` au lieu de lignes de texte :

<!-- verificar:extracto:src/revisar.rs -->
```rust
pub async fn revisar(cliente: &reqwest::Client, s: &Servicio) -> Estado {
    let inicio = Instant::now();
    let respuesta = cliente
        .get(&s.url)
        .timeout(Duration::from_millis(s.timeout_ms)) // .await cede el control mientras espera
        .send()
        .await;
    let ms = inicio.elapsed().as_millis() as u64;

    match respuesta {
        Ok(r) if r.status().is_success() && ms > UMBRAL_LENTO_MS => Estado::Lento {
            codigo: r.status().as_u16(),
            ms,
        },
        Ok(r) if r.status().is_success() => Estado::Ok {
            codigo: r.status().as_u16(),
            ms,
        },
        Ok(r) => Estado::Falla {
            motivo: format!("codigo {}", r.status().as_u16()),
            ms,
        },
        Err(e) if e.is_timeout() => Estado::Falla {
            motivo: "se acabo el tiempo de espera".to_string(),
            ms,
        },
        Err(_) => Estado::Falla {
            motivo: "no responde".to_string(),
            ms,
        },
    }
}
```

Le réseau oblige aussi à séparer concurrence et ordre. Le `revisor` peut lancer plusieurs requêtes en même temps, mais la sortie doit être reproductible et doit relier chaque état à son bon service. `join_all` conserve l'ordre des futures d'entrée, même si les réponses arrivent dans un autre ordre. Le sémaphore limite combien de requêtes entrent dans la section active ; il ne détermine pas l'ordre final du `Vec<Estado>`.

La figure suivante n'utilise ni le réseau ni aucun crate, pour que n'importe qui puisse la compiler avec un simple `rustc` et obtenir toujours la même sortie. Au lieu de HTTP, elle représente la politique du sémaphore : avec une limite de 2, les tours sont remis deux par deux. Dans le vrai projet, chaque tour vit jusqu'à la fin de la requête et le runtime réveille la tâche quand le réseau répond.

**Fig. 8.3** | Une limite de concurrence divise les éléments en attente en lots sans changer leur ordre.

```rust
// fig08_03.rs
fn lote<T>(pendientes: &mut Vec<T>, limite: usize) -> Vec<T> {
    let n = limite.min(pendientes.len());
    pendientes.drain(..n).collect()
}

fn main() {
    let mut servicios = vec!["catalogo", "pagos", "usuarios", "correo", "facturas"];

    while !servicios.is_empty() {
        println!("arrancan: {}", lote(&mut servicios, 2).join(", "));
    }
}
```

```bash
$ rustc --edition 2024 fig08_03.rs && ./fig08_03
arrancan: catalogo, pagos
arrancan: usuarios, correo
arrancan: facturas
```

La vraie fonction crée un `Arc<Semaphore>` parce que chaque future a besoin de partager le même compteur de tours. `Arc` permet la propriété partagée entre tâches ; `Semaphore` remet un permis temporaire ; et la variable `_turno` conserve ce permis jusqu'à la fin de `revisar`. Le tiret bas initial évite un avertissement du compilateur, mais la valeur n'est pas jetée : son destructeur rend le permis quand elle sort du bloc.

<!-- verificar:extracto:src/revisar.rs -->
```rust
pub async fn revisar_todos(
    cliente: &reqwest::Client,
    servicios: &[Servicio],
    paralelo: usize,
) -> Vec<Estado> {
    let paralelo = if paralelo == 0 {
        PARALELO_POR_OMISION
    } else {
        paralelo
    };
    let turnos = Arc::new(Semaphore::new(paralelo));

    let futuros = servicios.iter().map(|s| {
        let turnos = Arc::clone(&turnos);
        async move {
            // pide turno; espera si ya hay `paralelo` corriendo, y lo devuelve al soltar `_turno`
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
            revisar(cliente, s).await
        }
    });
    join_all(futuros).await
}
```

Le paramètre `paralelo` a une nuance : le programme accepte `0` comme « utilise la valeur par défaut ». C'est une décision d'interface ; cela ne signifie pas que Tokio puisse exécuter zéro requête. La constante `PARALELO_POR_OMISION` garde cette règle près de la logique qui l'utilise.

La requête individuelle crée une limite par service avec `.timeout(Duration::from_millis(s.timeout_ms))`. Cette limite ne remplace pas le sémaphore. Le timeout décide combien de temps une requête peut attendre ; le sémaphore décide combien de requêtes peuvent attendre en même temps. Sans timeout, un tour peut rester occupé trop longtemps. Sans sémaphore, une énorme liste peut ouvrir trop de connexions en même temps. Les deux limites protègent des ressources différentes.

`reqwest::Client::new()` se crée une fois et s'emprunte à toutes les requêtes. Ne construis pas un client par service : un client peut conserver et réutiliser des connexions, alors qu'un nouveau à chaque appel perd cet avantage et ajoute du travail inutile. L'emprunt partagé `&reqwest::Client` suffit parce que les requêtes du client n'exigent pas de référence mutable exclusive.

### Le rapport est une interface stable pour les personnes et les programmes

Un tableau sert à lire dans un terminal. Le JSON sert à ce qu'un autre programme consomme les résultats sans avoir à deviner colonnes, espaces ou alignement. Il n'est pas conseillé d'utiliser le tableau comme format d'intégration : si demain tu changes sa largeur, un script qui le traite avec `awk` peut se casser même si l'information est la même. Il n'est pas non plus conseillé d'afficher des diagnostics humains sur la sortie standard quand du JSON a été demandé, parce que cela cesserait d'être du JSON valide.

Le `revisor` utilise `stdout` pour le rapport et `stderr` pour les erreurs de démarrage. Cette convention permet de rediriger uniquement le rapport vers un fichier :

```bash
cargo run -- --archivo servicios.yaml --formato json > estado.json
```

Si la configuration est valide, `estado.json` ne contient que du JSON. Si quelque chose empêche de démarrer, le message apparaît dans le terminal par `stderr` ; il ne se mélange pas à un format qu'un autre processus s'attend à analyser.

La fonction `json` part des services et des états, les trie par nom et construit une collection d'`EstadoJson`. Trier avant de sérialiser n'est pas obligatoire pour que le JSON soit valide, mais cela aide à ce que le résultat soit stable. Un résultat stable se compare mieux dans les tests, les revues de changements et les automatisations.

<!-- verificar:extracto:src/reporte.rs -->
```rust
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
```

Les `match` forcent à décider ce que représente chaque variante. `NoIntentado` n'a ni durée ni vrai code, donc il s'exprime par `ms: 0`, `codigo: null` et une erreur explicite. `Falla` conserve son temps écoulé parce que savoir qu'une connexion a épuisé sa limite à 5 000 ms est une information utile. `Ok` et `Lento` ont un code ; les deux sont sains pour le code de sortie, même si le rapport les différencie.

Cette partie révèle une différence utile avec Go. En Go, tu peux construire des structs avec des étiquettes `json:"..."` en n'utilisant que `encoding/json` ; pour YAML, tu ajoutes normalement une autre bibliothèque avec sa propre convention d'étiquettes. En Rust, `serde` centralise la définition de la sérialisation et permet aux adaptateurs de format de travailler sur le même derive. En contrepartie, tu dois incorporer des crates externes et comprendre quelles parties du contrat sont dans les attributs et lesquelles dans la validation explicite.

### Le profil de release prépare un artefact, il ne remplace pas la vérification

`cargo run` et `cargo test` utilisent par défaut le profil de développement. Ce profil privilégie une compilation rapide pendant l'édition et conserve des informations utiles pour déboguer. Un binaire distribuable se construit avec `cargo build --release` ; Cargo applique alors le profil `release` défini par le projet.

Le profil ne transforme pas un programme incorrect en programme correct. Les tests, le format et Clippy doivent d'abord passer. Ensuite, le profil décide de l'équilibre entre taille du binaire, temps de compilation, optimisation et diagnostic disponible en cas de panique. Dans ce projet, on a choisi la taille : on retire les symboles, on optimise pour la taille, on active l'optimisation entre modules, on utilise une seule unité de génération et les paniques font avorter le processus.

<!-- verificar:extracto:Cargo.toml -->
```toml
[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila
```

Chaque option a un coût. `lto = true` et `codegen-units = 1` peuvent augmenter le temps de compilation parce qu'ils permettent d'optimiser avec plus d'information globale. `panic = "abort"` réduit le binaire et évite le déroulement de la pile, mais sacrifie la possibilité de nettoyer par déroulement et offre moins de contexte si une panique arrive en production. Pour cet outil de ligne de commande, c'est un choix raisonnable ; ce n'est pas une recette universelle pour toute bibliothèque.

Ne déclare pas que Rust ou Go « gagnent » sur la taille d'un binaire sans mesurer sur ta machine et avec la même fonctionnalité. Le binaire de Rust peut grossir en incluant Tokio, Reqwest, TLS et leurs dépendances transitives ; celui de Go inclut aussi son runtime et dépend de ses options de compilation. La comparaison valide note la plateforme, l'architecture, la compilation à froid ou incrémentale, le profil utilisé et les dépendances qu'inclut chaque programme.

La compilation croisée laisse une autre différence pratique. Go permet généralement de sélectionner la plateforme avec des variables comme `GOOS` et `GOARCH`. Rust exige d'installer la cible (target) correspondante et, selon la cible, de disposer aussi d'un éditeur de liens (linker) et de bibliothèques compatibles. Pour un Linux statique basé sur musl, la première étape serait :

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Cela ne signifie pas que la compilation croisée soit impossible en Rust ; cela signifie que tu dois modéliser la cible comme partie de l'environnement de construction. Vérifie le binaire obtenu sur le système où il sera utilisé, surtout s'il inclut TLS ou si tu changes de libc Linux.

## L'erreur que tu vas voir

L'analyseur d'arguments doit convertir du texte en valeurs typées. Si tu essaies d'assigner directement un texte à un `usize`, Rust ne devine pas que tu veux interpréter le texte comme un nombre. Le programme suivant échoue avant de s'exécuter.

**Fig. 8.4** | Un texte n'est pas un nombre même s'il représente une option de ligne de commande.

```rust
// fig08_04.rs
fn main() {
    let paralelo: usize = "cinco";
    println!("{paralelo}");
}
```

```bash
$ rustc --edition 2024 fig08_04.rs
error[E0308]: mismatched types
 --> fig08_04.rs:3:27
  |
3 |     let paralelo: usize = "cinco";
  |                   -----   ^^^^^^^ expected `usize`, found `&str`
  |                   |
  |                   expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`E0308` signifie que deux types ne coïncident pas. L'annotation `: usize` établit ce que doit produire l'expression de droite, mais `"cinco"` est un `&str`. La correction n'est pas de retirer le type pour que cela compile avec une chaîne : `paralelo` doit rester un nombre parce qu'il contrôle une limite de concurrence. Tu dois convertir avec `.parse::<usize>()` et décider quoi faire si la conversion échoue.

Dans le programme minimal, cette décision apparaît sous la forme `Result<Args, String>`. Dans le vrai projet, `clap` fait la conversion des champs déclarés et affiche un diagnostic d'utilisation quand il ne peut pas construire le type attendu. Ensuite, `main` valide encore les règles propres à l'application : que `formato` soit `tabla` ou `json`, que le YAML ait des services et que les valeurs de chaque service aient un sens.

Une autre erreur fréquente est de confondre un HTTP non réussi avec une erreur qui doit se propager avec `?`. Dans ce programme, un HTTP 500 se convertit en `Estado::Falla`, s'affiche et donne le code de sortie 1. Le propager comme erreur de démarrage ferait qu'un service tombé cacherait le résultat des autres. L'opérateur `?` reste correct pour les échecs qui empêchent de continuer, comme ne pas pouvoir lire le YAML ou ne pas pouvoir sérialiser le rapport JSON.

## Ce qui se fait mal

### Mettre `unwrap()` aux frontières du programme

`unwrap()` est acceptable dans un test quand tu prépares des données littérales que tu contrôles. Il ne l'est pas pour des arguments, des fichiers YAML ni le réseau. Un argument mal écrit ne doit pas produire une panique avec un backtrace ; il doit donner un message qui dise quelle option est invalide et se terminer avec le code 2. Un fichier absent n'est pas une surprise de programmation : c'est une condition d'usage que le binaire doit signaler avec le nom du fichier.

La règle utile est de demander qui contrôle la donnée. Si c'est le test, `expect` peut rendre un échec clair. Si c'est une personne, un fichier, le système d'exploitation ou le réseau, convertis-la en une erreur contextuelle ou en un état du domaine.

### Créer un `reqwest::Client` pour chaque service

Créer le client dans `revisar` paraît local et simple, mais perd la réutilisation des connexions et disperse la configuration du client. Le projet crée un seul `reqwest::Client` après que la configuration a été validée et l'emprunte à toutes les requêtes. Ainsi le cycle de vie du client coïncide avec le cycle de vie d'une exécution du `revisor`.

Ne confonds pas partager le client avec partager un état mutable sans contrôle. Le client s'utilise au moyen de références partagées, et le sémaphore protège la ressource qui doit effectivement être limitée : combien de requêtes sont actives.

### Utiliser du JSON construit avec `format!` en production

La figure du JSON montre la forme des données, pas une technique de sérialisation sûre. Si le nom d'un service contient des guillemets, une barre oblique inverse ou un saut de ligne, une chaîne construite à la main peut cesser d'être du JSON valide ou changer de sens. `serde_json` échappe correctement ces données et conserve la relation entre `Option`, `null` et champs omis.

Une bonne règle est que les formats structurés se construisent avec un sérialiseur et se testent en les relisant. Dans les tests du projet, le JSON est converti en `serde_json::Value` avant de vérifier des champs précis.

### Traiter le code de sortie comme un détail invisible

Un tableau avec le mot `FALLA` aide une personne, mais une tâche planifiée a besoin de savoir si elle doit alerter. Si le `revisor` se termine toujours avec 0, un cron, un pipeline ou un superviseur peut croire que tout est sain même s'il y a des services tombés. S'il se termine avec 1 face à n'importe quel problème, il ne distingue pas non plus une panne surveillée d'une mauvaise configuration.

Garde le contrat simple et documenté : 0 pour une vérification saine, 1 pour des services échoués et 2 pour les erreurs de démarrage ou d'utilisation. Les tests du binaire doivent vérifier ces trois chemins.

### Mesurer le binaire de Rust contre celui de Go sans contrôler l'expérience

Comparer seulement la taille de deux fichiers est une façon pauvre de tirer des conclusions. Le résultat change si l'un inclut TLS, si l'autre utilise une bibliothèque HTTP externe, si l'un a été compilé en développement et l'autre en release, si les symboles ont été retirés ou si le système compresse les exécutables. Le temps change aussi entre une compilation à froid, une incrémentale et une reconstruction après avoir touché une ligne.

Mesure avec le même cas d'usage et note les conditions. Parfois, Go sera le choix pratique parce que la compilation croisée et la bibliothèque standard réduisent des étapes. Parfois, Rust sera préférable parce que tu veux exprimer certaines garanties avant d'exécuter et accepter une compilation plus coûteuse. Le but de faire les deux cours n'est pas de répéter un slogan ; c'est d'avoir tes propres preuves.

## Exercices

### Exercice 1 — Déclare une option supplémentaire

Ajoute à un programme indépendant une option `--silencioso` qui soit fausse par défaut. Quand elle est vraie, le programme doit afficher uniquement le code numérique de chaque état ; quand elle est fausse, il doit afficher le nom, l'état et le code. Fais qu'un drapeau inconnu produise un `Err` avec un message clair.

Ensuite, explique ce que tu déclarerais dans `clap` : un champ `bool`, une option avec `#[arg(long)]` et le comportement par défaut que son type exprime déjà.

### Exercice 2 — Sépare les données invalides des services échoués

Écris une fonction qui reçoit une liste de services minimaux avec `nombre`, `url` et `timeout_ms`. Elle doit renvoyer une erreur si la liste est vide, si un nom se répète, si l'URL ne commence pas par `http://` ou `https://`, ou si le temps limite est zéro. Séparément, représente un HTTP 503 comme une variante d'état, pas comme une erreur de validation.

Utilise les règles de `config::validar` comme référence, mais écris d'abord tes cas de test : une liste valide, une vide, une avec un nom en double et une avec un timeout de zéro.

### Exercice 3 — Teste le contrat du binaire

Sans ouvrir encore `tests/binario.rs`, écris un test de bout en bout pour le format invalide `-f xml` (dans une copie du projet, ou avec un autre nom si tu travailles dans l'original). Il doit vérifier que le code de sortie est 2, qu'il n'y a pas de tableau sur la sortie standard et que la sortie d'erreur nomme `--formato`.

Ensuite, lance seulement ce test puis toute la suite. Ne change pas le code du projet pour faire passer un test mal écrit : le test doit décrire le contrat que `main.rs` expose déjà. À la fin, ouvre `tests/binario.rs` : le projet apporte déjà un test pour ce contrat et c'est la solution de référence ; compare ce que vérifie chacun.

### Exercice 4 — Compare les deux implémentations

Construis en release le `revisor` de Rust et le programme équivalent de Go. Dans une bitácora (journal de bord), note, pour les deux, les lignes de code propres, les dépendances directes, le temps de compilation à froid, la taille du binaire, l'usage de mémoire sur une même configuration et le temps que tu as mis à terminer le programme.

Réponds par une phrase argumentée : lequel livrerais-tu pour un outil interne avec une échéance proche, lequel pour un outil qui doit être maintenu des années, et quelle décision concrète de chaque langage t'a mené à cette conclusion.

## Solutions

### Solution 1

<!-- verificar:fragmento -->
```rust
fn imprimir(nombre: &str, codigo: u16, silencioso: bool) {
    if silencioso {
        println!("{codigo}");
    } else {
        println!("{nombre} OK {codigo}");
    }
}
```

L'option ne change pas l'état du domaine ; elle change la présentation. C'est pourquoi elle doit s'appliquer près de la frontière de sortie, pas dans `Estado` ni dans la logique HTTP. Avec `clap`, un `bool` marqué `#[arg(long)]` exprime que l'absence du drapeau vaut `false` et sa présence vaut `true`.

### Solution 2

<!-- verificar:fragmento -->
```rust
fn validar_timeout(timeout_ms: u64) -> Result<(), String> {
    if timeout_ms == 0 {
        Err("el tiempo límite debe ser mayor que cero".to_string())
    } else {
        Ok(())
    }
}

enum Estado {
    Falla { codigo: u16 },
}
```

Le timeout invalide est un problème d'entrée : il ne faut pas lancer la vérification. Un code 503, en revanche, est une information obtenue en vérifiant et appartient à l'état du service. La séparation permet que le binaire se termine avec 2 dans le premier cas et avec 1 dans le second.

### Solution 3

<!-- verificar:fragmento -->
```rust
#[test]
fn un_formato_desconocido_sale_con_dos() {
    let r = revisor(&["-f", "xml"]);
    assert_eq!(r.status.code(), Some(2), "stderr: {}", texto(&r.stderr));
    assert!(texto(&r.stdout).is_empty(), "stdout: {}", texto(&r.stdout));
    assert!(
        texto(&r.stderr).contains("--formato"),
        "stderr: {}",
        texto(&r.stderr)
    );
}
```

Le test exécute le vrai binaire, c'est pourquoi il examine ses trois sorties observables : code, sortie standard et sortie d'erreur. Il n'a pas besoin de connaître les fonctions privées de `main.rs` ; il ne connaît que le contrat que verra qui exécute `revisor -f xml`.

### Solution 4

La comparaison doit commencer par des commandes reproductibles :

```bash
cd programas/revisor
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

Ensuite, répète l'expérience avec le programme de Go, sur le même ordinateur et avec la même configuration de services. Ne copie pas les résultats des autres comme s'ils étaient universels. La valeur de l'exercice est d'identifier quelle part du temps a été de la compilation, quelle part a été d'apprendre l'écosystème et quelle part a été de résoudre le même problème de conception.

## Comment savoir que j'y suis arrivé

- `cargo run -- --help` montre les options `--archivo`, `--formato` et `--paralelo`.
- `cargo run -- --archivo archivo-que-no-existe.yaml` se termine avec le code 2 et écrit le nom du fichier sur la sortie d'erreur.
- `cargo run -- --formato xml` se termine avec le code 2 et explique que les formats valides sont `tabla` et `json`.
- `cargo run --example ejemplo_clap`, `ejemplo_serde` et `ejemplo_reqwest` affichent la même chose que documente cette leçon, et `herramientas/verificar-ejemplos.sh` se termine sans erreurs.
- `cargo test` se termine avec des résultats corrects pour la bibliothèque, l'intégration et le binaire.
- `cargo clippy --all-targets -- -D warnings` se termine sans avertissements.
- `cargo fmt --check` se termine sans changements en attente.
- `cargo build --release` crée le binaire dans `target/release/revisor`.
- Tu peux expliquer pourquoi un HTTP 500 produit le code de sortie 1, alors qu'un YAML invalide produit le code 2.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 12 : An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) — consulté le 2 octobre 2026.
- [Référence officielle de Cargo : profils](https://doc.rust-lang.org/cargo/reference/profiles.html) — consulté le 2 octobre 2026.
- [Documentation de `clap`](https://docs.rs/clap/latest/clap/) — consulté le 2 octobre 2026.
- [Documentation de `reqwest`](https://docs.rs/reqwest/latest/reqwest/) — consulté le 2 octobre 2026.
