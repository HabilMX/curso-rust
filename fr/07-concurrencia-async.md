# Leçon 7 — Concurrence et async

**Durée :** 2 × 45 min.

**Ce que tu construis :** le `revisor` concurrent : qu'il vérifie tout à la fois

**Ce que tu apprends :** threads du système, `Arc` et `Mutex`, canaux, `async/await` avec `tokio`, la comparaison honnête avec les goroutines de Go

## À la fin, tu vas pouvoir

- Lancer des threads du système avec `thread::spawn`, leur transférer la bonne propriété et récupérer leur résultat avec `join`.
- Expliquer pourquoi une donnée partagée entre threads a besoin d'`Arc`, quand elle a en plus besoin d'un `Mutex`, et ce que protège le `MutexGuard`.
- Utiliser un canal de la bibliothèque standard pour livrer des résultats sans partager une collection mutable.
- Lire l'erreur `E0277` quand une valeur ne remplit pas `Send` et reconnaître que le compilateur protège une frontière entre threads.
- Expliquer la différence entre concurrence et parallélisme, et entre un thread du système, une tâche async et une goroutine de Go.
- Suivre dans le `revisor` le parcours d'une requête async, depuis `#[tokio::main]` jusqu'à `join_all`, au sémaphore et à l'état qui reste lié à chaque service.

## Le pourquoi avant le comment

Jusqu'ici, le `revisor` peut recevoir une liste de services, en interroger un et classer la réponse. S'il interroge dix services en série et que chacun met une seconde à répondre, le rapport prend environ dix secondes. Peu importe que l'ordinateur ait plusieurs cœurs ou que le programme soit rapide : pendant presque tout ce temps, le CPU ne calcule rien ; il attend une réponse du réseau.

Attendre une réponse du réseau est différent de calculer une grosse somme. Dans un calcul intense, davantage de cœurs peuvent permettre un vrai travail parallèle. Dans une requête HTTP, en revanche, l'essentiel du temps se passe hors du processus : le système d'exploitation attend des paquets, le serveur distant décide quoi faire et le réseau transporte la réponse. Pendant ce temps, le programme pourrait lancer d'autres requêtes. C'est cela, la concurrence : organiser plusieurs tâches qui avancent par périodes entrelacées. Elle peut devenir du parallélisme si plusieurs tâches exécutent du code en même temps sur des cœurs différents, mais ce ne sont pas des synonymes.

L'objectif pratique de cette leçon est de changer le temps total. Si cinq services mettent environ une seconde chacun et que tu les interroges un par un, le rapport prend environ cinq secondes. Si tu lances les cinq requêtes et attends leurs réponses en même temps, le temps se rapproche de celui du service le plus lent, pas de la somme de tous. Cela ne fait pas répondre plus vite les services distants ; cela évite de gaspiller le temps que le programme passait à attendre le premier avant de commencer le deuxième.

Le coût est que plusieurs parties du programme peuvent être vivantes en même temps. Apparaissent des questions que le code séquentiel n'avait pas : qui est propriétaire de la donnée ? quand une tâche se termine-t-elle ? quel ordre ont les résultats ? deux tâches peuvent-elles modifier la même valeur ? que se passe-t-il si l'une échoue ? comment empêches-tu d'ouvrir des milliers de connexions simultanées ? Rust ne répond pas à ces questions en cachant la mémoire partagée. Il fait en sorte que la possession (ownership), les emprunts et des traits comme `Send` et `Sync` continuent de compter quand il y a plusieurs threads ou tâches.

Cela se relie directement à la leçon 2. La possession semblait une règle locale : une valeur a un propriétaire et les emprunts doivent respecter sa durée de vie. En concurrence, ces règles deviennent une garantie entre tâches. Un thread ne peut pas conserver une référence à une variable de `main` qui a peut-être déjà disparu. Il ne peut pas non plus recevoir un type que Rust sait dangereux à déplacer vers un autre thread. Ce qui, au début, ressemblait à de la friction avec le compilateur devient ici une barrière contre les références pendantes (dangling references) et les courses aux données (data races) dans le code sûr.

Rust offre deux outils principaux pour ce problème. La bibliothèque standard apporte des threads du système, des mutex, des compteurs de références atomiques et des canaux. Ils conviennent au travail de CPU, aux petits programmes ou à l'intégration avec des API bloquantes. Pour beaucoup d'opérations réseau qui passent du temps à attendre, le `revisor` utilise `async/await` et Tokio. Async ne signifie pas « plus rapide par définition » : cela signifie qu'un nombre modéré de threads peut faire avancer de nombreuses opérations qui attendent des E/S sans réserver un thread bloqué pour chacune.

Go prend une autre décision. Une goroutine se lance avec `go f()` et le runtime est intégré au langage et à la distribution. Rust oblige à distinguer un thread d'une tâche async et, pour async, à choisir un runtime. Cela exige plus de vocabulaire et plus de décisions, mais permet que le type de donnée et la frontière de propriété soient explicites. Aucune des deux approches n'élimine la nécessité de concevoir des limites, de gérer les erreurs et de mesurer. La comparaison utile n'est pas de savoir quel langage « gagne », mais quel coût chacun paie et quelle garantie il offre en échange.

Lis d'abord les chapitres 16 et 17 de The Rust Book. Fais ensuite les exercices de Rustlings sur les threads et les canaux avant d'adapter le `revisor`. L'ordre compte : async devient beaucoup moins mystérieux quand tu comprends déjà ce que signifie qu'une closure soit déplacée vers un autre thread, ce que veut dire `Send` et pourquoi partager de la mutabilité exige une synchronisation visible.

## Les concepts

### Threads du système, `move` et `join`

`std::thread::spawn` demande une closure et démarre un thread du système d'exploitation pour l'exécuter. La valeur qu'il renvoie est un `JoinHandle<T>` : une promesse concrète que le thread peut se terminer avec une valeur de type `T`. Appeler `join()` attend que ce thread se termine et renvoie `Result<T, Box<dyn Any + Send>>` ; l'`Err` représente que le thread a fait `panic!`.

Le mot-clé `move` est important. Une closure sans `move` peut essayer de capturer une référence à une variable du contexte extérieur. Un thread peut continuer à s'exécuter après la fin de ce contexte, donc Rust ne permet pas de remettre au thread une référence qui risque de cesser d'être valide. `move` fait que la closure capture par valeur. Dans la figure, chaque `Servicio` cesse d'appartenir au vecteur et passe à la closure de son propre thread.

**Fig. 7.1** | Un thread par service.

```rust
// fig07_01.rs
use std::thread;

struct Servicio {
    nombre: String,
}

type Estado = String;     // en el curso es el enum de la lección 3; aquí basta un texto

fn revisar(s: &Servicio) -> Estado {
    format!("{}: OK", s.nombre)
}

fn main() {
    let servicios = vec![
        Servicio { nombre: "catalogo".to_string() },
        Servicio { nombre: "pagos".to_string() },
        Servicio { nombre: "reportes".to_string() },
    ];

    let handles: Vec<_> = servicios.into_iter().map(|s| {
        thread::spawn(move || revisar(&s))      // `move` entrega la propiedad al hilo
    }).collect();

    for h in handles {
        let estado = h.join().unwrap();          // espera y recoge el resultado
        println!("{estado}");
    }
}
```

```bash
$ rustc --edition 2024 fig07_01.rs && ./fig07_01
catalogo: OK
pagos: OK
reportes: OK
```

L'ordre des `println!` est déterministe même si les threads se terminent dans un autre ordre. Les `JoinHandle` sont stockés dans le même ordre que produit `servicios.into_iter()`, et le second `for` appelle `join()` dans cet ordre. Si le thread de `pagos` se termine le premier, sa valeur est prête, mais le programme attend d'abord et affiche celle de `catalogo`. Cette différence compte : la concurrence n'oblige pas à ce que la sortie soit non déterministe. Tu peux concevoir une frontière où l'ordre observable reste stable.

`join` suffit quand chaque tâche a un résultat et que le nombre de tâches est petit et connu. Tu n'as pas besoin d'un canal juste pour récupérer une valeur ; le handle la livre déjà. En Go, tu combinerais normalement une goroutine avec un `WaitGroup` pour attendre et un canal ou une collection protégée pour récupérer les résultats. Rust fait du résultat une partie du handle, même si cela n'élimine pas l'utilité des canaux pour la communication progressive.

Un thread du système n'est pas gratuit. Il a des ressources du système d'exploitation, une pile et un coût d'ordonnancement supérieur à celui d'une tâche async. Il n'y a pas de chiffre universel de mémoire par thread : cela dépend du système d'exploitation, de l'architecture et de la configuration. La règle de conception est plus utile qu'un nombre fixe : n'ouvre pas un thread du système par connexion si le travail principal consiste à attendre le réseau. Pour quelques tâches de CPU ou une API bloquante, un thread peut être exactement ce qu'il faut. Pour de nombreux services HTTP, le `revisor` utilise async.

Le `revisor` ne crée pas un thread par service. Son travail réseau s'exprime sous forme de futures async ; le runtime décide quels threads exécutent ces futures. Cependant, la même idée de propriété apparaît : une tâche doit posséder ce qu'elle conserve pendant son exécution, ou recevoir des emprunts qui restent valides jusqu'à sa fin. C'est pourquoi il est important de comprendre d'abord `move`, même si le code final utilise Tokio.

### `Arc`, `Mutex` et la donnée qui vit dans le verrou

Un `Rc<T>` permet à plusieurs propriétaires dans un seul thread de partager une valeur. Son compteur de références n'est pas atomique, donc il ne peut pas être partagé entre threads. `Arc<T>` signifie *atomic reference counted* : il remplit la même fonction générale, mais met à jour le compteur de références de façon sûre entre threads. Cloner un `Arc` ne clone pas `T` ; cela crée seulement un autre propriétaire de la même allocation.

Avoir plusieurs propriétaires n'équivaut pas à avoir la permission de modifier. Si `T` est mutable et que plusieurs tâches peuvent y accéder, il faut de la coordination. `Mutex<T>` contient la donnée et permet à une seule tâche à la fois de recevoir un accès mutable au moyen de `lock()`. Le résultat de `lock()` est un `MutexGuard<T>`. Tant que ce guard existe, le verrou reste pris ; en sortant de sa portée (scope), son `Drop` libère le verrou. Tu n'as pas besoin d'écrire un appel séparé à `unlock`, ce qui réduit le risque d'oublier de libérer la ressource dans un chemin de retour.

**Fig. 7.2** | Une donnée partagée entre threads.

```rust
// fig07_02.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let estado = "OK".to_string();

    let estados = Arc::new(Mutex::new(HashMap::new()));
    let copia = Arc::clone(&estados);
    let h = thread::spawn(move || {
        copia.lock().unwrap().insert("x".to_string(), estado);
    });

    h.join().unwrap();
    println!("{:?}", estados.lock().unwrap());
}
```

```bash
$ rustc --edition 2024 fig07_02.rs && ./fig07_02
{"x": "OK"}
```

La forme `Arc<Mutex<HashMap<_, _>>>` se lit de l'intérieur vers l'extérieur. Le `HashMap` est la donnée. Le `Mutex` est l'unique porte pour la muter. L'`Arc` permet à différents threads d'être propriétaires de cette porte. Pour insérer, le thread prend le verrou et reçoit le guard ; pour afficher, `main` prend à nouveau le verrou. Il n'apparaît jamais de référence mutable à la table sans que le guard existe.

En Go, il est fréquent de déclarer un `sync.Mutex` à côté de la table et de suivre la convention d'appeler `Lock` avant d'y toucher. Cette convention peut être bien encapsulée, mais le langage n'oblige pas à ce que la table soit physiquement à l'intérieur du mutex. En Rust, en enveloppant la donnée dans `Mutex<T>`, l'API normale ne permet pas d'obtenir `&mut T` sans un `MutexGuard`. Cela ne rend pas impossibles toutes les erreurs de concurrence : tu peux encore provoquer un interblocage (deadlock) en prenant des verrous dans des ordres incohérents, ou garder un guard trop longtemps. En revanche, cela rend impossible, dans le code sûr, une course aux données causée par l'emprunt simultané de la même valeur mutable sans synchronisation.

N'utilise pas `lock().unwrap()` comme une formule dont on ne réfléchit pas. `lock()` peut renvoyer une erreur si un autre thread a fait `panic!` en tenant le verrou ; cela s'appelle l'empoisonnement (poisoning) du mutex. Dans un exemple pédagogique, `unwrap()` rend ce cas visible avec un `panic!`. Dans un vrai service, tu dois décider si cet état invalide le programme, si tu peux récupérer la donnée avec `into_inner`, ou s'il convient de repenser la conception pour que l'état partagé ne soit pas nécessaire.

Le `revisor` évite un `Mutex<HashMap<...>>` parce qu'il n'a pas à remplir une table partagée au fur et à mesure que chaque réponse arrive. Chaque future produit son propre `Estado`, et `join_all` les rassemble. La ressource qu'il partage est une limite de tours : plusieurs futures ont besoin de demander la permission de lancer une requête, pas de modifier une collection commune. C'est pourquoi le projet utilise `Arc<Semaphore>` et non `Arc<Mutex<Vec<Estado>>>`.

### Canaux : livrer des valeurs au lieu de partager une collection

Un canal divise la communication en deux extrémités : un émetteur, `Sender<T>`, et un récepteur, `Receiver<T>`. L'émetteur livre des valeurs avec `send` ; le récepteur les prend avec `recv` ou en itérant dessus. Au lieu que plusieurs threads écrivent dans une structure partagée, chaque producteur livre une valeur qui devient la propriété du récepteur. Cette architecture réduit la zone partagée et clarifie qui assemble le résultat final.

Le canal créé avec `std::sync::mpsc::channel()` est à producteurs multiples et à un seul consommateur. Il est « multiple » parce que tu peux cloner l'émetteur avant de déplacer une copie vers chaque thread. Le récepteur ne se clone pas : un seul endroit décide quoi faire de chaque message. Quand tous les émetteurs disparaissent, l'itération sur le récepteur se termine. Cette fermeture du canal fait partie du protocole, pas d'un détail incident.

**Fig. 7.3** | Un canal livre des états au thread qui affiche le rapport.

```rust
// fig07_03.rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (emisor, receptor) = mpsc::channel();

    let hilo = thread::spawn(move || {
        for estado in ["catalogo: OK", "pagos: OK"] {
            emisor.send(estado).unwrap();
        }
    });

    hilo.join().unwrap();

    for estado in receptor {
        println!("{estado}");
    }
}
```

```bash
$ rustc --edition 2024 fig07_03.rs && ./fig07_03
catalogo: OK
pagos: OK
```

Le programme attend le thread avant de parcourir le récepteur pour conserver une sortie déterministe. Dans un programme qui traite vraiment les résultats au fur et à mesure qu'ils arrivent, tu commencerais normalement à recevoir pendant que les producteurs travaillent encore. L'ordre serait alors celui d'arrivée, pas nécessairement celui des services d'entrée. Cela peut être une bonne décision pour une interface qui informe de la progression, mais pas pour un rapport qui doit aligner chaque état avec le service d'origine.

Les canaux ne remplacent pas automatiquement les mutex. Si plusieurs tâches doivent lire et mettre à jour le même compte, un `Mutex` est peut-être le modèle naturel. Si une tâche produit des valeurs et qu'une autre décide comment les stocker ou les afficher, un canal représente généralement mieux la responsabilité. L'erreur courante est de choisir par mode : « les mutex sont mauvais » ou « les canaux sont compliqués ». La question utile est de savoir qui doit être propriétaire de chaque donnée à chaque moment.

Dans le `revisor`, `join_all` remplit une fonction semblable à recevoir tous les résultats, mais avec un contrat supplémentaire : il conserve l'ordre des futures d'entrée, de sorte que `estados[i]` est toujours le résultat de `servicios[i]`. C'est ce dont le rapport a besoin : savoir à quel service appartient chaque état. Attention à ce qu'il *ne* promet *pas* : le rapport n'est pas imprimé dans l'ordre de la liste YAML, parce que `reporte::tabla` et `reporte::json` trient les lignes par nom de service avant de les écrire (tu le verras à la leçon 8). Ce que `join_all` garantit, c'est la correspondance entre chaque service et son état, et avec un canal, où les résultats arrivent dans l'ordre où ils se terminent, il faudrait la reconstruire à la main. Si le produit devait afficher « pagos terminé » dès que la réponse arrive, un canal ou un stream (une séquence de valeurs qui arrivent au fil du temps) serait une option raisonnable. Ne l'ajoute pas seulement parce qu'il existe ; le choix actuel est intentionnel et garde le rapport reproductible.

### `Send`, `Sync` et l'erreur que le compilateur arrête

`Send` et `Sync` sont des traits marqueurs. Ils ne requièrent généralement pas de méthodes propres ; ils décrivent des propriétés de sûreté que Rust peut dériver des champs d'un type. Un type `Send` peut être transféré par valeur à un autre thread. Un type `Sync` peut être partagé par référence entre threads : si `T` est `Sync`, alors `&T` est `Send`. Beaucoup de structures courantes les implémentent automatiquement quand leurs composants sont eux aussi sûrs, mais `Rc<T>` n'est ni `Send` ni `Sync` parce que son compteur ne peut pas être mis à jour depuis plusieurs threads.

Cette règle n'est pas une liste à mémoriser. C'est une question à laquelle Rust répond par composition. Si tu fais une struct qui contient `Rc<RefCell<_>>`, elle hérite des restrictions de ces pièces. Si tu passes à `Arc<Mutex<_>>`, tu changes la représentation et aussi les garanties disponibles. Le compilateur suit la valeur jusqu'à la closure envoyée à `thread::spawn` et exige que la frontière soit sûre.

En Go, une course aux données peut compiler et nécessiter `go test -race` pour être détectée pendant une exécution qui atteint justement l'entrelacement problématique. Le détecteur est précieux et tu dois l'utiliser, mais il dépend de ce que le test exécute le chemin conflictuel. Rust évite les courses aux données dans le code sûr avant d'exécuter le programme. Cela ne prouve pas que la logique est correcte et ne détecte pas automatiquement les interblocages, la famine (une tâche qui n'obtient jamais son tour parce que d'autres accaparent la ressource) ou les protocoles mal conçus. Il existe aussi `unsafe`, où le programmeur assume des responsabilités supplémentaires. L'affirmation précise est : Rust évite les courses aux données grâce à ses règles de types et d'emprunts dans le code sûr ; il ne promet pas que tout programme concurrent soit correct.

Dans le `revisor`, `Arc<Semaphore>` est valide parce que le sémaphore de Tokio est conçu pour être partagé entre tâches. Chaque future reçoit son propre `Arc`, demande un permis et conserve ce permis pendant la requête. Le type du permis et son cycle de vie expriment que le tour ne peut pas être rendu avant la fin de la requête. Il n'y a pas de compteur `usize` partagé que chaque future incrémente et décrémente manuellement.

### `async`, futures et le runtime de Tokio

Une fonction marquée `async fn` n'exécute pas immédiatement tout son corps quand on l'appelle. Elle produit une future : une valeur qui représente du travail en attente. Cette future avance quand un exécuteur (executor) la sonde. Si elle arrive à une opération qui n'est pas encore prête, comme attendre une réponse réseau, elle rend le contrôle à l'exécuteur. Plus tard, quand l'opération peut continuer, l'exécuteur la sonde à nouveau.

`.await` est le point où une fonction async attend le résultat d'une autre future. Il ne crée pas à lui seul une nouvelle tâche ni un nouveau thread. Cette distinction corrige deux malentendus fréquents. Premièrement : écrire `let futuro = revisar(...);` ne lance pas nécessairement la requête ; cela ne fait que construire la future. Deuxièmement : appeler `.await` l'un après l'autre dans le même bloc peut rendre les opérations sérielles. Pour lancer plusieurs opérations de façon concurrente, tu construis plusieurs futures et tu les conduis ensemble avec un combinateur comme `join_all`, ou tu les convertis en tâches avec `tokio::spawn` quand tu as vraiment besoin d'indépendance.

Rust définit la syntaxe et les traits d'async, mais n'inclut pas un exécuteur async complet dans la bibliothèque standard. Tokio est le runtime choisi par ce projet. Il fournit un exécuteur, des minuteries, de la synchronisation async et des adaptations d'E/S. Certaines bibliothèques async sont indépendantes du runtime, mais les ressources de Tokio, comme ses minuteries et plusieurs types de synchronisation, exigent de s'exécuter dans un contexte Tokio. Lis la documentation de chaque crate avant de supposer que n'importe quelle future fonctionne de la même façon avec n'importe quel runtime.

<!-- verificar:extracto:src/main.rs -->
```rust
#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    ejecutar(&args).await
}

async fn ejecutar(args: &Args) -> ExitCode {
```

L'attribut `#[tokio::main]` construit le runtime et exécute la fonction `main` async. Le `main` du programme continue de renvoyer un `ExitCode`, comme tu l'as vu à la leçon 4 ; ce qui change, c'est qu'il peut maintenant attendre des opérations async avant de décider du code de sortie. Le binaire conserve la responsabilité d'analyser les arguments, d'afficher et de sortir ; la bibliothèque conserve la logique d'interrogation des services.

Ne bloque pas un thread du runtime avec `std::thread::sleep`, une lecture de fichier lourde ou un long calcul dans une fonction async. Un thread bloqué ne peut pas sonder les autres futures qui lui sont assignées. Pour le travail bloquant, il existe `tokio::task::spawn_blocking` ; pour les E/S réseau, utilise des API async comme `reqwest`. Le `revisor` utilise `reqwest::Client` et attend son `send().await`, donc tant qu'une réponse est en attente, le runtime peut faire avancer les requêtes d'autres services.

Avant de voir comment le `revisor` utilise Tokio, il convient de voir Tokio seul. Le programme suivant est le plus petit qui montre l'essentiel : trois « vérifications » qui, au lieu d'interroger le réseau, attendent simplement, chacune un temps différent. Il ne tient pas dans un simple `rustc`, parce qu'il dépend des crates `tokio` et `futures` ; c'est pourquoi il vit dans `programas/revisor/examples/` et s'exécute avec Cargo, qui télécharge et compile ces dépendances.

**Exemple cargo avec `tokio`** | Trois attentes conduites à la fois avec `join_all` : elles se terminent dans un ordre et sont livrées dans un autre.

<!-- verificar:ejemplo:ejemplo_tokio -->
```rust
// ejemplo_tokio.rs
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::time::sleep;

async fn revisar(nombre: &str, espera_ms: u64) -> String {
    sleep(Duration::from_millis(espera_ms)).await;
    println!("terminó {nombre}");
    format!("{nombre}: respondió tras {espera_ms} ms")
}

#[tokio::main]
async fn main() {
    let servicios = [("catalogo", 600), ("pagos", 200), ("usuarios", 400)];
    let inicio = Instant::now();

    let futuros = servicios.iter().map(|(nombre, ms)| revisar(nombre, *ms));
    let resultados = join_all(futuros).await;

    println!("--- en el orden de la lista ---");
    for resultado in &resultados {
        println!("{resultado}");
    }
    // Esperarlos uno tras otro habría tardado 1200 ms; a la vez tardan lo del más lento.
    let a_la_vez = inicio.elapsed() < Duration::from_millis(1100);
    println!("tardó menos que la suma de las esperas: {a_la_vez}");
}
```

```bash
$ cargo run --example ejemplo_tokio
terminó pagos
terminó usuarios
terminó catalogo
--- en el orden de la lista ---
catalogo: respondió tras 600 ms
pagos: respondió tras 200 ms
usuarios: respondió tras 400 ms
tardó menos que la suma de las esperas: true
```

Exécute-le depuis `programas/revisor/` (à la première exécution, Cargo met un moment à compiler les dépendances).

`#[tokio::main]` transforme `main` en fonction async : il construit le runtime et lui remet la future que `main` décrit. `tokio::time::sleep` est l'attente de Tokio, et ressemble à `std::thread::sleep` par ce qu'elle fait mais pas par la manière : avec `.await`, la tâche cède le contrôle au runtime pendant qu'elle attend, et le runtime en profite pour faire avancer les autres. Avec `std::thread::sleep`, le thread entier s'endormirait et plus rien n'avancerait dessus.

Lis la sortie en deux parties. Les messages `terminó ...` sortent dans l'ordre où chaque attente se termine : `pagos` (200 ms), `usuarios` (400 ms) et `catalogo` (600 ms), bien que la liste les déclare dans un autre ordre. Ensuite, `join_all` livre les résultats dans l'ordre de la liste —`catalogo`, `pagos`, `usuarios`—, parce qu'il renvoie un `Vec` où chaque position correspond à sa future d'entrée. La dernière ligne vérifie que c'était concurrent : attendre les trois vérifications l'une après l'autre aurait pris 1200 ms, alors que, simultanément, cela prend le temps de la plus lente, environ 600 ms.

Si tu remplaces `sleep` par un appel réseau avec `.send().await`, tu as la forme du `revisor` : beaucoup de futures qui attendent, un seul `join_all` qui les conduit et un `Vec` de résultats aligné sur la liste de services.

### `join_all`, sémaphores et la limite de parallélisme du `revisor`

Lancer toutes les requêtes possibles en même temps n'est pas toujours une amélioration. Un fichier avec des milliers de services pourrait ouvrir trop de connexions, saturer le réseau local, épuiser les descripteurs de fichiers ou charger le serveur que tu essaies justement de vérifier. La concurrence a besoin d'une limite. L'argument `--paralelo` du `revisor` exprime combien de requêtes peuvent être actives à la fois.

Un sémaphore contient des permis. Pour démarrer une requête, une future en acquiert un ; s'il n'en reste pas, elle attend. Quand le permis sort de la portée, il est libéré automatiquement et une autre future peut continuer. C'est la même idée de RAII (la ressource est libérée quand la valeur qui la représente sort de la portée) que tu as vue avec `MutexGuard` : la ressource est libérée à la destruction du guard, même si la fonction sort par un chemin normal. Ici, la ressource n'est pas un verrou exclusif mais une capacité limitée.

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

`servicios.iter()` conserve des emprunts à la liste ; elle ne consomme pas les services. Chaque closure async possède son clone d'`Arc<Semaphore>`, mais emprunte `cliente` et `s` pendant l'appel à `revisar`. Cela fonctionne parce que `join_all(futuros).await` se termine avant que `revisar_todos` puisse revenir, donc ces emprunts restent vivants. Si tu utilisais `tokio::spawn`, la tâche pourrait survivre à la fonction qui l'a créée et aurait normalement besoin de données de durée `'static` ; là, tu devrais déplacer ou cloner davantage de données.

`join_all` renvoie un `Vec<Estado>` dans l'ordre des futures d'entrée, pas dans l'ordre où les requêtes se terminent. C'est une décision utile pour le rapport : l'état zéro correspond au service zéro. Le sémaphore limite le moment où chaque future peut entrer dans `revisar`, mais ne change pas cette relation finale. Ainsi, le programme a une vraie concurrence sans faire du rapport une source d'ordre aléatoire.

La fonction `revisar` ne renvoie pas `Result<Estado>`. Cette décision mérite attention. Un HTTP 500, un timeout ou une connexion refusée n'est pas une erreur interne qui empêche le programme de continuer : c'est justement l'information que le `revisor` doit rapporter sur un service. C'est pourquoi ces cas se convertissent en variantes `Estado::Falla`. L'erreur d'`acquire` est traitée différemment parce que le sémaphore n'est jamais fermé dans cette conception ; si cela arrivait, ce serait une violation d'une hypothèse interne.

Le timeout du projet se configure sur la requête de `reqwest`, avec le `timeout_ms` de chaque `Servicio`. Ne confonds pas cette limite avec le sémaphore. Le timeout limite combien de temps peut attendre une requête individuelle ; le sémaphore limite combien de requêtes peuvent attendre ou communiquer en même temps. Tu as besoin des deux : sans timeout, un tour peut rester occupé trop longtemps ; sans sémaphore, de nombreuses requêtes avec timeout peuvent démarrer toutes ensemble et surcharger les ressources.

### Comparaison honnête avec Go

Go rend très facile de démarrer une unité concurrente : `go revisar(s)`. Le runtime ordonnance les goroutines sur des threads du système, agrandit leurs piles et gère le travail réseau. Rust sépare explicitement la décision : `thread::spawn` crée un thread du système ; un runtime comme Tokio exécute des futures async ; `tokio::spawn` crée une tâche Tokio. Cela signifie que Go a généralement moins de cérémonie au départ et que Rust oblige à savoir laquelle des trois abstractions tu utilises.

Rust n'a pas d'avantage magique de performance du fait d'écrire `async`. Une tâche async n'accélère pas une requête HTTP individuelle ; elle améliore l'utilisation des threads pendant que plusieurs requêtes attendent. Pour une charge de CPU, async peut être pire s'il bloque l'exécuteur. Dans ce cas, utilise des threads, un pool de workers ou `spawn_blocking`. Go ne rend pas non plus automatiquement plus rapide un calcul intensif en CPU : plusieurs goroutines peuvent se disputer les mêmes cœurs. Mesurer le type de travail compte plus que d'appliquer un mot à la mode.

La garantie centrale de Rust apparaît avant l'exécution : une donnée mutable ne peut pas être empruntée de façon incompatible, et les valeurs envoyées à un autre thread doivent remplir les traits adéquats. Go privilégie une syntaxe réduite et des outils d'exécution comme le détecteur de courses. Go peut encapsuler correctement les mutex et les canaux ; Rust peut avoir des deadlocks et des erreurs logiques. La différence n'est pas « Go permet des erreurs et Rust non ». C'est où chaque langage met la charge de la vérification et quelles erreurs il peut rejeter avant l'exécution.

Pour le `revisor`, la décision est justifiée par le problème. Il y a beaucoup d'attentes HTTP, chaque résultat doit rester lié à son service et il faut un plafond configurable de requêtes. Tokio, `join_all` et `Semaphore` expriment ces trois besoins. Une conception avec un thread par service fonctionnerait pour une petite liste, mais passerait moins bien à l'échelle et n'apporte aucun avantage au cas d'usage. Une conception avec un mutex et une table partagée pourrait aussi fonctionner, mais compliquerait une relation que `join_all` conserve déjà.

## L'erreur que tu vas voir

L'erreur la plus instructive de cette leçon est `E0277`. Elle ne signifie pas que « Rust ne veut pas utiliser de threads » ; elle signifie que le type que tu essaies de déplacer ne satisfait pas le contrat que `thread::spawn` exige. `Rc<i32>` est utile pour partager la propriété dans un thread, mais son compteur de références n'est pas atomique. Le déplacer vers une closure qui peut s'exécuter dans un autre thread serait dangereux.

**Fig. 7.4** | `Rc<T>` ne peut pas être envoyé à un thread.

```rust
// fig07_04.rs
use std::rc::Rc;
use std::thread;

fn main() {
    let conteo = Rc::new(0);
    thread::spawn(move || println!("{conteo}"));
}
```

```bash
$ rustc --edition 2024 fig07_04.rs
error[E0277]: `Rc<i32>` cannot be sent between threads safely
 --> fig07_04.rs:7:19
  |
7 |     thread::spawn(move || println!("{conteo}"));
  |     ------------- -------^^^^^^^^^^^^^^^^^^^^^
  |     |             |
  |     |             `Rc<i32>` cannot be sent between threads safely
  |     |             within this `{closure@fig07_04.rs:7:19: 7:26}`
  |     required by a bound introduced by this call
  |
  = help: within `{closure@fig07_04.rs:7:19: 7:26}`, the trait `Send` is not implemented for `Rc<i32>`
note: required because it's used within this closure
 --> fig07_04.rs:7:19
  |
7 |     thread::spawn(move || println!("{conteo}"));
  |                   ^^^^^^^
note: required by a bound in `spawn`
 --> /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library/std/src/thread/functions.rs:125:0

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Le message contient la réponse. La closure capture `Rc<i32>`, `thread::spawn` exige que ce qui est capturé soit `Send`, et `Rc<i32>` n'implémente pas `Send`. Ne répare pas cette erreur en ajoutant des traits à la main avec `unsafe impl Send` ; tu promettrais au compilateur une sûreté que `Rc` n'offre pas. Si plusieurs threads doivent seulement lire une donnée, utilise `Arc<T>`. S'ils doivent en plus la modifier, évalue `Arc<Mutex<T>>` ou repense le flux pour envoyer des valeurs par des canaux.

Reconnais aussi l'erreur de conception que le compilateur ne rejette parfois pas : garder un `MutexGuard` pendant une opération `.await`. Si tu lances la tâche avec `tokio::spawn` et que le guard est de `std::sync::Mutex`, `rustc` la rejette bien (`future cannot be sent between threads safely`, parce que ce guard n'est pas `Send`) ; mais si la future est attendue dans le même thread, comme le fait `join_all` dans le `revisor`, le programme compile. Pour ce cas, `clippy` apporte par défaut l'avertissement `await_holding_lock`. Un guard de `std::sync::Mutex` bloque un thread ; un guard d'un mutex async conserve le verrou pendant que la tâche peut céder l'exécuteur. Dans les deux cas, attendre le réseau en tenant le verrou bloque généralement du travail inutilement et peut produire des interblocages. Extrais ou mets à jour la donnée sous le verrou, relâche le guard et seulement ensuite attends.

## Ce qui se fait mal

- Créer un thread par service sans limite. Cela fonctionne avec trois exemples et échoue comme stratégie quand la liste grandit. Les threads du système consomment des ressources du système d'exploitation et une grande quantité d'entre eux rend plus difficile de planifier, mesurer et déboguer. Pour les E/S réseau, utilise async avec une limite de parallélisme ; pour le CPU, utilise un nombre de threads proportionnel au travail et aux cœurs disponibles.

- Utiliser `Arc<Mutex<_>>` comme réponse automatique à toute erreur de possession (ownership). Cette combinaison est correcte quand il y a un état mutable réellement partagé, mais elle peut cacher une conception où plusieurs tâches en font trop. Si chaque tâche peut renvoyer une valeur et qu'une seule partie les rassemble, un canal, `join_all` ou une réduction ultérieure est généralement plus clair et réduit la contention.

- Garder un verrou pendant une requête réseau ou un `.await`. Le guard existe pour protéger une petite section critique. Si tu le gardes pendant que tu attends, tu transformes des tâches concurrentes en file d'attente et tu augmentes la possibilité d'interblocage. Limite la portée avec des accolades ou une variable temporaire pour que le guard soit détruit avant l'attente.

- Appeler des fonctions bloquantes dans du code Tokio. `std::thread::sleep` bloque le thread, pas seulement une tâche. Une grosse lecture, une requête bloquante ou un calcul lourd ont le même problème. Utilise des API async pour les E/S, `tokio::time` pour les minuteries ou `spawn_blocking` pour le travail qui doit vraiment bloquer.

- Confondre `async` avec parallélisme. Une future peut avancer de façon concurrente avec d'autres et pourtant s'exécuter sur un seul thread. Si tu dois accélérer du calcul de CPU, tu dois décider comment le répartir entre les cœurs. Si tu attends le réseau, async améliore l'utilisation des threads existants. Avant d'optimiser, mesure ce qu'attend le programme.

- Utiliser `tokio::spawn` seulement pour « le rendre concurrent ». Dans le `revisor`, `join_all` peut conduire des futures qui empruntent `cliente` et `servicios`, conserve l'ordre et évite d'exiger la propriété `'static`. `tokio::spawn` est utile pour des tâches indépendantes qui doivent vivre au-delà du bloc actuel, mais implique un autre contrat de durée de vie et de types.

- Traiter la sortie qui arrive en premier comme si c'était l'état du service qui occupe cette position. Dans une interface de progression, il peut être utile d'informer par arrivée. Dans un rapport, si le résultat d'un autre service se glisse dans une ligne, chaque ligne ment sur son service. Le `revisor` évite ce risque avec `join_all`, qui conserve la correspondance entre `servicios[i]` et `estados[i]`, et ses tests d'intégration vérifient cette propriété ; l'ordre dans lequel les lignes sont affichées est une autre décision, que prend le rapport en les triant par nom.

## Exercices

### Exercice 1 — Trois vérifications avec `join`

Écris un programme avec trois `Servicio` de texte. Utilise `thread::spawn(move || ...)` pour produire un état pour chacun, conserve les handles dans un vecteur et utilise `join` pour afficher les résultats dans le même ordre d'entrée. N'utilise pas `sleep` pour « laisser du temps » aux threads.

### Exercice 2 — Compteur protégé

Crée un `Arc<Mutex<u32>>` avec la valeur initiale zéro. Lance quatre threads ; chacun doit incrémenter le compteur une fois. Attends tous les handles et affiche `total: 4`. Ensuite, remplace exprès `Arc` par `Rc` et confirme que `E0277` apparaît.

### Exercice 3 — Résultats par canal

Crée un canal et trois threads producteurs. Chaque producteur doit envoyer le nom d'un service et un état. Le thread principal doit recevoir exactement trois messages et les trier par nom avant de les afficher. Explique dans un commentaire pourquoi tu ne dois pas dépendre de l'ordre d'arrivée.

### Exercice 4 — Explique la limite du `revisor`

Lis `programas/revisor/src/revisar.rs`. Exécute les tests d'intégration et repère le test `el_tope_de_paralelo_se_respeta`. Dans ta bitácora (journal de bord), réponds : quelle ressource contrôle le `Semaphore`, quand le permis est-il acquis, quand est-il libéré et pourquoi `join_all` conserve-t-il l'ordre de `servicios`.

## Solutions

### Solution 1

La bonne solution déplace chaque `Servicio` vers le thread et récupère les handles ensuite. Le point décisif n'est pas le `map`, mais que le vecteur de handles maintient l'obligation d'attendre chaque travail avant de terminer `main`. Si tu affiches après chaque `join`, l'ordre observable est l'ordre du vecteur de handles.

Une façon de vérifier que tu n'as pas dépendu de `sleep` est d'exécuter le programme plusieurs fois : il doit toujours afficher les trois lignes. L'ordonnanceur peut changer quel thread se termine en premier, mais ne peut pas empêcher que `join` attende.

### Solution 2

Chaque thread doit recevoir son propre `Arc::clone(&contador)`. Dans le thread, prends le guard, incrémente et laisse le guard sortir de la portée. Après avoir attendu les quatre handles, prends un dernier guard pour afficher la valeur. N'essaie pas de garder un emprunt mutable du compteur hors du mutex ; cet emprunt ne peut pas coexister avec les autres threads.

En remplaçant `Arc` par `Rc`, le résultat attendu n'est pas une sortie numérique mais `E0277`. La correction ne consiste pas à « faire taire » le compilateur : `Rc` sert aux références partagées d'un seul thread ; `Arc` est le type adéquat pour que le compteur ait des propriétaires dans plusieurs threads.

### Solution 3

Chaque producteur reçoit un clone de l'émetteur et envoie une structure ou un n-uplet (tuple) avec le nom et l'état. L'émetteur d'origine doit cesser d'exister avant de parcourir tout le récepteur, ou tu peux appeler `recv` exactement trois fois puisque tu connais le nombre de producteurs. Stocke les messages reçus dans un vecteur et trie-le par nom avant d'afficher.

La partie importante est que le récepteur est l'unique propriétaire du vecteur final. Les producteurs n'y ont pas d'accès mutable. C'est pourquoi tu n'as pas besoin d'un mutex pour rassembler les résultats ; la propriété voyage par le canal avec chaque message.

### Solution 4

Le sémaphore contrôle le nombre de requêtes HTTP qui peuvent être actives à la fois, pas le nombre total de services. Chaque future acquiert un permis juste avant d'appeler `revisar(cliente, s).await`. Le permis vit dans `_turno` ; quand cet appel se termine, `_turno` sort de la portée et rend la capacité au sémaphore.

`join_all` reçoit des futures construites en parcourant `servicios` et produit le vecteur d'états dans cette même séquence. C'est pourquoi les résultats peuvent se terminer à des moments différents sans se désaligner du service qui les a originés. Le test mesure des temps pour confirmer que la limite change le comportement et n'est pas qu'un drapeau décoratif.

## Comment savoir que j'y suis arrivé

Exécute les figures de cette leçon depuis leurs répertoires et compare la sortie exacte :

```bash
cd programas/07-concurrencia-async
rustc --edition 2024 fig07_01.rs && ./fig07_01
rustc --edition 2024 fig07_02.rs && ./fig07_02
rustc --edition 2024 fig07_03.rs && ./fig07_03
rustc --edition 2024 fig07_04.rs
```

Les trois premières compilations doivent se terminer sans avertissements et produire les sorties documentées. La dernière doit échouer avec `E0277` ; cet échec est le résultat correct de l'exercice.

Vérifie que les blocs et leurs sorties restent vérifiables depuis la racine du cours :

```bash
herramientas/verificar-programas.sh es
herramientas/verificar-extractos.sh
herramientas/verificar-ejemplos.sh
```

Les trois commandes doivent se terminer correctement. La première confirme que chaque figure compile, s'exécute et coïncide avec sa sortie documentée, sauf la figure conçue pour échouer. La deuxième confirme que les extraits du `revisor` restent des copies exactes du vrai projet. La troisième compile et exécute l'exemple avec `tokio` de cette leçon et compare ce qu'il affiche avec ce qui est documenté.

Enfin, vérifie le comportement concurrent réel du projet :

```bash
cd programas/revisor
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

`cargo test` doit signaler des résultats réussis, y compris le test `el_tope_de_paralelo_se_respeta`. `cargo clippy --all-targets -- -D warnings` doit se terminer sans warnings, et `cargo fmt --check` ne doit proposer aucun changement. Si tu peux expliquer pourquoi le sémaphore limite les requêtes, pourquoi `join_all` conserve l'ordre et pourquoi `Rc` provoque `E0277`, tu as terminé la leçon.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 16 : Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — consulté le 2 octobre 2026.

- [The Rust Programming Language, chapitre 17 : Fundamentals of Asynchronous Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html) — consulté le 2 octobre 2026.

- [Documentation de `std::thread`](https://doc.rust-lang.org/std/thread/) — consulté le 2 octobre 2026.

- [Documentation de `tokio::sync::Semaphore`](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html) — consulté le 2 octobre 2026.
