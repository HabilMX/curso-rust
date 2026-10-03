# Leçon 4 — Collections et erreurs

**Durée :** 2 × 45 min.

**Ce que tu construis :** la liste des services et le rapport, avec de vraies erreurs

**Ce que tu apprends :** `Vec`, `HashMap`, `String` contre `&str`, `Result` et `?`, `panic!` contre `Result`, `anyhow` et `thiserror`

**The Rust Book, chapitres 8 et 9.** Rustlings : `vecs`, `hashmaps`, `strings`, `error_handling`.

## À la fin, tu vas pouvoir

- Stocker une liste de `Servicio` dans un `Vec<Servicio>` et la parcourir sans te battre avec la possession (ownership).
- Choisir entre l'indexation d'une collection et des méthodes qui renvoient `Option`.
- Stocker des états par nom dans un `HashMap<String, Estado>` et produire un rapport ordonné.
- Expliquer pourquoi une fonction reçoit normalement `&str`, alors qu'une struct stocke normalement `String`.
- Propager les erreurs de fichiers et de validation avec `Result` et `?`.
- Distinguer une erreur que la personne qui utilise le programme peut corriger d'un invariant rompu qui justifie `panic!`.
- Ajouter du contexte à une erreur d'application avec `anyhow` et reconnaître quand une bibliothèque a besoin de `thiserror`.

## Le pourquoi avant le comment

À la leçon 3, tu as modélisé un service et les différents résultats de sa vérification. Ce modèle doit encore vivre quelque part. Le programme reçoit de nombreux services, pas un seul ; il doit les conserver pendant qu'il interroge chaque URL, associer chaque résultat à son service, puis afficher un rapport. Dans un petit programme, tu peux écrire à la main deux ou trois variables. Dans le vrai `revisor`, cette stratégie cesse de fonctionner dès que le fichier YAML apporte un nombre variable de services.

Les collections résolvent la question du « combien de valeurs y a-t-il », mais elles ne résolvent pas à elles seules ce que signifie l'absence d'une valeur ni ce que le programme doit faire quand quelque chose d'extérieur échoue. Un fichier peut ne pas exister, une ligne peut avoir un format invalide, le nom d'un service peut être répété et une URL peut ne pas avoir de schéma. Aucun de ces cas n'est rare ni impossible : tous se produisent parce que le programme reçoit des données de l'extérieur. La différence importante est que Rust te demande de représenter cette possibilité dans le type de retour.

Un `Vec<Servicio>` représente une liste ordonnée de services. Un `HashMap<String, Estado>` (table de hachage) représente une association par clé : étant donné le nom `"catalogo"`, il cherche son état. Un `String` est un texte qui possède sa mémoire ; un `&str` est une vue empruntée d'un texte que quelqu'un d'autre possède. Et un `Result<T, E>` représente une opération qui peut se terminer par une valeur `T` ou par une erreur `E`. Ce ne sont pas quatre sujets sans rapport : ce sont les pièces qui donnent une forme explicite à l'état du programme.

Le cours de Go construit le même revisor. En Go, lire une clé inexistante d'une `map` renvoie la valeur zéro et oblige à se souvenir de la forme à deux résultats pour distinguer « n'existe pas » de « existe et vaut zéro ». Rust choisit un autre contrat : `HashMap::get` renvoie `Option<&V>`. L'absence apparaît dans le type et ne peut pas être confondue avec un état réel. Le coût est que tu dois décider quoi faire de `None` ; le gain est que cette décision ne peut pas être oubliée sans que le code ne le rende visible.

Il se passe quelque chose de semblable avec les erreurs. Go utilise la convention `if err != nil` après chaque opération qui peut échouer. Rust utilise `Result` et permet d'écrire la propagation avec `?`. Il n'y a pas de réponse universelle sur le style le plus lisible : Go répète une structure très explicite ; Rust concentre la même décision dans un opérateur. L'important est que les deux obligent à traiter l'erreur. Rust ne transforme pas un fichier absent en chaîne vide et ne laisse pas une conversion échouée continuer comme si elle était valide.

Cette leçon ne cherche pas à te faire utiliser `unwrap()` pour faire taire le compilateur. Elle cherche à te faire lire la signature de chaque fonction comme un contrat. Si une fonction renvoie `Option`, tu dois réfléchir à ce que signifie l'absence. Si elle renvoie `Result`, tu dois décider si l'erreur se résout là, se transforme ou se propage. Si elle reçoit `&str`, elle a seulement besoin de lire du texte ; si elle reçoit `String`, elle a probablement l'intention de le garder. Les signatures décrivent le flux des données et le flux des échecs avant que le programme ne s'exécute.

## Les concepts

### `Vec<T>` : une liste propriétaire de valeurs du même type

`Vec<T>` est le vecteur de Rust : une collection de taille variable qui possède ses éléments. Le paramètre `T` dit quel type de valeurs elle peut stocker. Un `Vec<Servicio>` ne stocke que des services ; un `Vec<Estado>` ne stocke que des états. Cette restriction n'est pas une gêne accidentelle. Elle permet au compilateur de savoir comment gérer chaque élément, quelles méthodes sont valides et quelles opérations pourraient déplacer ou emprunter des valeurs.

Un vecteur vide a besoin d'une annotation de type si Rust ne peut pas l'inférer. C'est pourquoi la figure écrit `let mut v: Vec<Servicio> = Vec::new();`. Le compilateur n'a encore vu aucun élément et ne peut pas deviner ce qu'il y aura dedans. Si tu crées le vecteur avec `vec![...]`, ou si le contexte détermine déjà le type, tu n'as normalement pas besoin de l'écrire. Le mot `mut` est nécessaire parce que `push` modifie la collection : il ajoute un élément et peut faire que le vecteur réserve plus d'espace.

Le vecteur est propriétaire de chaque `Servicio` qu'il reçoit. Dans `v.push(s)`, la variable `s` est déplacée (move) dans le vecteur. C'est l'application des règles de possession (ownership) de la leçon 2 : après avoir déplacé un `Servicio`, tu ne peux pas continuer à utiliser l'ancienne variable comme si elle le possédait encore. Ce n'est pas une copie implicite. Si tu as besoin de conserver une autre version indépendante, tu dois concevoir l'opération pour emprunter, ou cloner de manière délibérée quand le coût et la sémantique le justifient.

Il y a deux façons de lire un élément. `&v[0]` produit une référence et suppose que l'indice existe. S'il n'existe pas, le programme entre en `panic!`. `v.get(0)` renvoie `Option<&Servicio>` : `Some(referencia)` s'il existe et `None` s'il est hors de la plage. La seconde forme convient quand l'indice vient d'un fichier, d'un argument, d'une requête ou de toute donnée que tu ne contrôles pas entièrement. La première est raisonnable quand faire échouer le programme révèle une erreur de programmation qu'une validation antérieure aurait dû éviter.

**Fig. 4.1** | Les collections et leur accès sûr.

```rust
// fig04_01.rs
use std::collections::HashMap;

struct Servicio {
    nombre: String,
}

#[derive(Debug)]
enum Estado {
    Ok,
    Falla,
}

fn main() {
    let s = Servicio { nombre: "catalogo".to_string() };
    let mut v: Vec<Servicio> = Vec::new();
    v.push(s);
    let primero = &v[0];                    // si no existe: panic
    println!("{}", primero.nombre);
    let primero = v.get(0);                 // devuelve Option<&Servicio>
    println!("{}", primero.is_some());

    let mut m: HashMap<String, Estado> = HashMap::new();
    m.insert("catalogo".to_string(), Estado::Ok);
    println!("{:?}", m.get("catalogo"));
    println!("{:?}", m.get("pagos"));       // Option<&Estado>: no hay valor cero silencioso
    println!("{:?}", Estado::Falla);

    let mut conteo: HashMap<String, u32> = HashMap::new();
    *conteo.entry("reportes".into()).or_insert(0) += 1;   // el patrón para contar
    *conteo.entry("reportes".into()).or_insert(0) += 1;
    println!("{:?}", conteo.get("reportes"));
}
```

```bash
$ rustc --edition 2024 fig04_01.rs && ./fig04_01
catalogo
true
Some(Ok)
None
Falla
Some(2)
```

N'utilise pas les indices pour parcourir un vecteur par habitude. Quand tout ce dont tu as besoin est de visiter chaque élément, `for servicio in &servicios` exprime mieux l'intention et évite les calculs d'indice. Quand tu as besoin du numéro de position, utilise `enumerate()` : `for (i, servicio) in servicios.iter().enumerate()`. La valeur `i` reste associée au bon élément et il n'y a pas de risque d'écrire accidentellement `i + 1` à la lecture.

Il importe aussi qu'une référence à un élément du vecteur soit un emprunt du vecteur entier. Ajouter des éléments peut exiger de déplacer tout le stockage vers une autre zone de mémoire. C'est pourquoi Rust ne permet pas de conserver `let primero = &v[0]`, d'appeler ensuite `v.push(...)` et de réutiliser `primero`. La restriction évite les références pendantes (dangling references) : des adresses qui pointaient auparavant vers un élément valide et pointeraient maintenant vers de la mémoire libérée.

Le `revisor` garde la liste des services dans un vecteur parce que le fichier déclare une séquence et que le rapport doit conserver cet ordre conceptuel. Les fonctions de rapport reçoivent des tranches (slices) empruntées, `&[Servicio]` et `&[Estado]`, au lieu de prendre les vecteurs. Une slice donne accès à une séquence sans transférer la propriété de la collection. Ainsi `main` peut afficher le rapport puis inspecter les états pour choisir le code de sortie.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub fn tabla(servicios: &[Servicio], estados: &[Estado]) -> String {
    let filas = ordenadas(servicios, estados);
    let ancho = filas
        .iter()
        .map(|(s, _)| s.nombre.chars().count())
        .max()
        .unwrap_or(0)
        .max("SERVICIO".len());
```

La valeur de `filas` est un `Vec<Fila<'a>>` : une nouvelle liste de paires de références. Elle ne clone ni les services ni les états pour pouvoir les trier ; elle crée des références vers les deux. Cette différence compte dans un programme qui peut manipuler de grandes listes. Posséder de nouvelles données coûte de la mémoire et du travail de copie ; emprunter des données existantes conserve une source de vérité et rend visible que le rapport ne fait qu'observer.

### `HashMap<K, V>` : chercher par clé sans inventer d'absences

Un `HashMap<K, V>` stocke des associations entre une clé et une valeur. Pour le revisor, une clé naturelle serait le nom du service et la valeur serait son état : `"catalogo" -> Estado::Ok { ... }`. C'est une collection utile quand tu sais ce que tu veux chercher, mais pas à quelle position d'une liste cela se trouve. Chercher linéairement dans un `Vec` implique d'examiner des éléments jusqu'à en trouver un ; chercher par clé dans une table exprime directement la question.

L'opération `insert` prend possession de la clé et de la valeur. C'est pourquoi la figure construit `"catalogo".to_string()` : la table a besoin de posséder un `String` qui survive à la fin de l'appel. `get`, en revanche, emprunte. Le type de `m.get("catalogo")` est `Option<&Estado>`, pas `Estado`, parce que la clé peut ne pas exister et parce que la table conserve la propriété de l'état.

C'est une différence importante avec Go. Un accès comme `m["pagos"]` dans une `map[string]Estado` de Go fournit la valeur zéro si la clé n'existe pas. Si `Estado` contient des nombres, ce zéro peut passer pour une vraie réponse. En Rust, `None` communique une absence que tu dois gérer. Tu peux utiliser `match`, `if let Some(estado) = ...`, ou des méthodes comme `unwrap_or` quand une valeur par défaut est vraiment correcte pour le domaine.

Le motif `entry(...).or_insert(0)` évite de faire deux recherches quand tu veux mettre à jour un compteur. `entry` représente la position d'une clé qui peut être occupée ou vacante. `or_insert(0)` laisse la valeur existante ou insère zéro et renvoie une référence mutable au compteur. Le `*` déréférence cette référence mutable pour pouvoir appliquer `+= 1`. Ce n'est pas une syntaxe décorative : Rust sépare avec précision la valeur stockée de l'emprunt qui permet de la modifier.

Une table ne promet aucun ordre de parcours. L'ordre interne dépend de la façon dont les clés sont réparties et peut changer à l'insertion, à la suppression, d'une exécution à l'autre ou avec une autre version de la bibliothèque. Ne construis jamais une sortie publique en parcourant un `HashMap` en espérant qu'elle sorte par hasard dans l'ordre alphabétique. Pour un rapport reproductible, extrais les clés, trie-les et parcours-les dans cet ordre, ou utilise une structure ordonnée quand c'est l'opération centrale.

Le projet actuel n'utilise pas de `HashMap` pour son rapport. Il utilise deux vecteurs parallèles : les services et les états, tous deux dans le même ordre. Cela permet à un service de conserver sa position depuis le YAML jusqu'au résultat de la requête. Au moment d'afficher, la fonction `ordenadas` forme des paires empruntées et les trie par `nombre`. Le concept qu'elle partage avec un `HashMap` est décisif : le stockage peut avoir l'ordre commode pour travailler, mais la sortie publique doit imposer son propre ordre de manière explicite.

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

Le lifetime `'a` apparaîtra en profondeur à la leçon 5. Pour l'instant, il suffit de le lire comme une garantie : chaque paire de `Fila` contient des références qui ne peuvent pas vivre plus longtemps que les slices d'entrée. Le vecteur `filas` est propriétaire des paires, mais pas des services ni des états. Quand `tabla` se termine, les références temporaires disparaissent ; les vecteurs d'origine restent la propriété de `main`.

### `String` et `&str` : posséder du texte contre lire une vue

Rust distingue le texte qui possède des données du texte qui n'emprunte qu'une vue. `String` est une chaîne UTF-8, mutable et de taille variable ; elle vit normalement dans le tas (heap) et est propriétaire de ses octets. `&str` est une référence à une séquence UTF-8 qui existe déjà ailleurs. Un littéral comme `"catalogo"` a le type `&'static str` : c'est une vue d'un texte stocké dans le binaire et disponible pendant toute l'exécution.

La règle pratique est simple : reçois `&str`, stocke `String`. Une fonction qui va seulement lire un nom n'a pas besoin de recevoir la propriété ni d'obliger l'appelant à créer une copie. Une struct qui doit conserver le nom après la fin de l'appel doit, elle, être propriétaire d'un `String`. Cette règle n'est pas absolue, mais elle évite deux erreurs courantes : accepter `String` par réflexe et finir par déplacer des valeurs inutilement, ou essayer de stocker une référence à un texte dont le propriétaire va disparaître.

**Fig. 4.2** | Reçois `&str`, accepte les deux.

```rust
// fig04_02.rs
fn saludar(n: &str) { println!("hola, {n}"); }

fn main() {
    let propio = String::from("catalogo");
    saludar("pagos");
    saludar(&propio);
}
```

```bash
$ rustc --edition 2024 fig04_02.rs && ./fig04_02
hola, pagos
hola, catalogo
```

L'appel `saludar(&propio)` fonctionne par coercition (deref coercion) : une référence à `String` peut être utilisée là où l'on attend `&str`. Ne copie pas la chaîne et n'écris pas `propio.to_string()` pour « que ça colle ». La fonction demande seulement la lecture, donc emprunter est l'opération correcte. De plus, `propio` reste disponible après l'appel.

<!-- verificar:fragmento -->
```rust
fn saludar(n: String) { }
```

Cette signature compile, mais elle communique autre chose : l'appelant doit remettre la propriété d'un `String`. Elle n'accepte pas un littéral sans conversion et ne permet pas de continuer à utiliser le `String` d'origine après l'appel. Une API de ce genre pourrait être correcte si la fonction va stocker, transformer ou renvoyer le texte en tant que propriétaire, mais c'est un mauvais choix pour une fonction qui ne fait qu'afficher ou comparer.

`String` n'admet pas l'indexation par entiers comme si chaque caractère occupait un octet. Rust utilise UTF-8 ; une lettre visible peut occuper plusieurs octets. Autoriser `nombre[3]` serait ambigu : le quatrième octet, la quatrième valeur Unicode ou le quatrième groupe visible ? C'est pourquoi tu dois décider de l'unité dont tu as besoin. `s.as_bytes()` travaille avec des octets, `s.chars()` travaille avec des valeurs `char`, et `s.get(rango)` renvoie `Option<&str>` quand la plage peut tomber au milieu d'un encodage. Cette restriction évite de couper une chaîne UTF-8 en un point invalide.

Le `revisor` stocke `nombre` comme `String` parce que la valeur vient du YAML et doit vivre à l'intérieur de chaque `Servicio`. Quand il calcule la largeur d'une colonne, il ne compte pas les octets : il utilise `chars().count()`. Cela ne résout pas tous les détails de largeur visuelle d'Unicode, mais évite de traiter un caractère multioctet comme plusieurs caractères au comptage.

<!-- verificar:extracto:src/reporte.rs -->
```rust
        .iter()
        .map(|(s, _)| s.nombre.chars().count())
        .max()
        .unwrap_or(0)
        .max("SERVICIO".len());
```

Ne convertis pas tout en `String` « au cas où ». Une conversion peut allouer de la mémoire et, surtout, cacher qui doit posséder le texte. Commence par la signature : si la fonction ne fait que lire, `&str` ; si le résultat doit survivre de façon indépendante, `String`. Si tu dois accepter plusieurs types qui peuvent être vus comme du texte, tu connaîtras `AsRef<str>` et les traits génériques à la leçon 5, mais ne les utilise pas avant qu'une API en ait réellement besoin.

### `Result<T, E>` et `?` : rendre visible le chemin de l'erreur

`Result<T, E>` est un enum de la bibliothèque standard avec deux variantes : `Ok(T)` et `Err(E)`. Une fonction qui renvoie `Result<String, std::io::Error>` promet l'une de deux choses : elle renverra du texte lu correctement, ou elle renverra l'erreur d'entrée/sortie qui a empêché de le lire. Elle ne renvoie pas un texte vide pour signaler l'échec et n'affiche pas d'erreur à l'intérieur d'une fonction qui sera peut-être utilisée depuis un autre endroit.

<!-- verificar:fragmento -->
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

L'opérateur `?` opère sur un `Result`. S'il reçoit `Ok(valor)`, il extrait `valor` et l'exécution continue. S'il reçoit `Err(error)`, il termine la fonction courante avec cette erreur, en la convertissant vers le type d'erreur déclaré quand une conversion valide existe. Il n'ignore pas l'erreur et n'en fait pas une panique. C'est une façon compacte d'écrire une décision qui reste obligatoire.

**Fig. 4.3** | L'opérateur `?` renvoie l'erreur à l'appelant.

```rust
// fig04_03.rs
fn leer_config(ruta: &str) -> Result<String, std::io::Error> {
    let contenido = std::fs::read_to_string(ruta)?;   // si falla, retorna el error
    Ok(contenido)
}

fn main() {
    match leer_config("servicios-que-no-existe.txt") {
        Ok(texto) => println!("{texto}"),
        Err(e) => println!("error: {e}"),
    }
}
```

```bash
$ rustc --edition 2024 fig04_03.rs && ./fig04_03
error: No such file or directory (os error 2)
```

La fonction `leer_config` ne sait pas si un fichier absent est fatal pour toute l'application, récupérable par un autre chemin ou attendu par un test. C'est pourquoi elle renvoie l'erreur. `main`, lui, décide comment la communiquer : dans cette figure, il l'affiche. Dans un binaire de production, tu l'écrirais normalement sur la sortie d'erreur et tu terminerais avec un code différent de zéro, pour qu'une personne et une automatisation puissent distinguer le succès de l'échec.

La comparaison avec Go est directe. Ces quatre lignes de Rust :

<!-- verificar:fragmento -->
```rust
let a = paso1()?;
let b = paso2(a)?;
let c = paso3(b)?;
Ok(c)
```

expriment une chaîne d'opérations qui, en Go, s'écrit généralement avec une vérification `if err != nil` après chaque étape. Rust réduit la répétition, mais ne réduit pas la responsabilité. Chaque `?` marque un endroit où la fonction peut sortir plus tôt. Si, plus tard, le programme doit nettoyer des ressources, transformer une erreur ou prendre une alternative, tu dois le décider avant ou après ce point.

Le `revisor` utilise `?` pour lire le fichier et pour désérialiser le YAML. Les deux échecs font partie du démarrage du programme : il n'y a pas de liste valide de services sans fichier lisible ni sans YAML valide. La fonction renvoie `anyhow::Result<Vec<Servicio>>`, ce qui permet d'unifier des erreurs de types différents sans perdre leurs messages.

<!-- verificar:extracto:src/config.rs -->
```rust
use anyhow::{Context, Result};
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}
```

Une précision sur le nom : `yaml_serde::from_str` est le même `from_str` qu'offrait `serde_yaml`, le crate précédent, qui n'est plus maintenu. À la leçon 6, tu verras pourquoi le `revisor` utilise le premier.

`with_context` ajoute une information que le système d'exploitation ne connaît pas. L'erreur d'origine peut dire « No such file or directory », mais le contexte précise quel fichier le revisor essayait de lire. C'est la différence entre un diagnostic techniquement correct et un diagnostic exploitable. L'opérateur `?` conserve cette chaîne de causes en renvoyant l'erreur.

Observe aussi que tous les échecs d'une requête ne sont pas des `Err`. La fonction `revisar` du projet renvoie `Estado`, même quand un service ne répond pas. C'est correct parce que « un service a échoué » est une donnée que le rapport doit montrer, pas une impossibilité de poursuivre le programme. `Result` représente le fait que le programme lui-même n'a pas pu mener à bien une opération nécessaire ; `Estado::Falla` représente un résultat normal du domaine du revisor. Choisir entre les deux dépend de qui doit décider quoi faire et de la capacité du programme à produire encore un résultat utile.

### `panic!` : un arrêt pour les bugs, pas un substitut de `Result`

`panic!` interrompt le flux normal d'exécution parce que le programme a rencontré une condition que ses propres hypothèses déclaraient impossible. Indexer un vecteur hors de la plage provoque une panique. Appeler `unwrap()` sur `None` ou sur `Err` aussi. Ces mécanismes existent parce qu'il y a des invariants qui, une fois rompus, révèlent une erreur de programmation et non une situation qu'une personne utilisatrice doit réparer.

Un fichier absent n'est pas un invariant rompu : il peut manquer à cause d'un chemin mal écrit, de permissions, d'un déploiement incomplet ou d'une décision de la personne qui exécute le binaire. Cela doit être un `Result`. Une réponse HTTP 500 n'est pas non plus une raison de paniquer : c'est un état que le revisor a été construit pour rapporter. Un indice calculé à partir d'un fichier ne doit pas non plus être utilisé avec `[]` sans validation ; utilise `get` et renvoie une erreur qui explique le problème.

**Fig. 4.4** | Une panique interceptée pour démontrer que ce n'est pas un `Result`.

```rust
// fig04_04.rs
fn main() {
    let previo = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let resultado = std::panic::catch_unwind(|| {
        panic!("la lista validada no puede estar vacia");
    });

    std::panic::set_hook(previo);
    println!("hubo panico: {}", resultado.is_err());
}
```

```bash
$ rustc --edition 2024 fig04_04.rs && ./fig04_04
hubo panico: true
```

La figure intercepte la panique uniquement pour isoler la démonstration. Ce n'est pas le motif normal d'une application. `catch_unwind` ne transforme pas les paniques en un contrôle de flux sain et ne garantit pas qu'une structure reste dans un état propre à continuer d'être utilisée. Dans le code de tous les jours, si tu penses à récupérer un `panic!` provoqué par des données externes, tu dois presque toujours repenser la fonction pour qu'elle renvoie `Result`.

`.unwrap()` et `.expect("mensaje")` sont des paniques potentielles. `expect` est préférable quand il existe une raison concrète et invariante de croire que l'appel n'échouera pas, parce que son message documente cette raison. N'écris pas `expect("debe funcionar")` : cela n'explique rien. Un message utile nomme l'hypothèse, comme « le sémaphore n'est jamais fermé », et indique clairement ce qu'il faudrait investiguer si cela se produit.

Le projet utilise `expect` pour acquérir un permis d'un sémaphore interne. Ce n'est pas une erreur provoquée par le fichier YAML ni par une URL ; ce serait une contradiction dans la coordination que le programme a construite lui-même. C'est pourquoi c'est l'un des rares endroits où la panique a du sens.

<!-- verificar:extracto:src/revisar.rs -->
```rust
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
```

Ne copie pas ce motif pour des opérations d'entrée/sortie. `File::open(ruta).expect(...)` transforme un chemin inexistant en terminaison du processus et supprime la possibilité que `main` affiche le fichier, utilise une autre valeur ou sélectionne un code de sortie approprié. Demande-toi d'abord si le cas peut se produire avec des données externes valides. Si la réponse est oui, renvoie `Result`.

### `anyhow` et `thiserror` : deux rôles distincts pour les erreurs propres

La bibliothèque standard suffit pour de nombreux petits programmes : tu peux renvoyer `Result<T, std::io::Error>` quand tout échec pertinent est d'entrée/sortie. Un vrai programme combine souvent plusieurs types : `std::io::Error`, une erreur YAML, une URL invalide, un argument de ligne de commande ou une règle de validation. Si chaque couche doit connaître tous ces types concrets, les signatures deviennent difficiles à maintenir.

`anyhow` résout bien la frontière d'une application. Son `Result<T>` est une forme abrégée pour renvoyer une erreur dynamique qui peut contenir différentes causes et du contexte supplémentaire. Le revisor est un binaire : il lit de la configuration, lance des requêtes et présente des messages à une personne. À cette frontière, la priorité est d'expliquer quelle opération a échoué et de conserver la chaîne des causes. C'est pourquoi `config::cargar` utilise `anyhow::{Context, Result}`.

Cela ne veut pas dire qu'`anyhow` soit un permis d'effacer le sens. Si une fonction renvoie un état qu'une autre partie du programme doit distinguer pour prendre des décisions, un enum propre peut être préférable. Par exemple, une bibliothèque qui doit permettre à l'appelant de différencier `NombreRepetido`, `UrlSinEsquema` et `TimeoutCero` ne doit pas fournir seulement une chaîne. Elle doit publier un type d'erreur avec des variantes qui représentent ces causes.

`thiserror` aide à déclarer ce type d'erreur propre sans écrire à la main des implémentations répétitives de `Display`, `Error` et de conversions depuis des erreurs internes. Il s'utilise surtout dans les bibliothèques, où le type d'erreur fait partie de l'API publique. Le `Cargo.lock` du projet contient `thiserror`, mais le `Cargo.toml` du revisor ne le déclare pas comme dépendance directe et son code actuel n'expose pas d'enum d'erreurs propre. C'est pourquoi le `revisor` ne l'utilise pas : ses erreurs sont des messages de texte qu'`anyhow` accompagne de contexte.

<!-- verificar:fragmento -->
```rust
#[derive(Debug, thiserror::Error)]
enum ErrorConfiguracion {
    #[error("el nombre «{0}» está repetido")]
    NombreRepetido(String),
    #[error("la URL «{0}» no tiene esquema HTTP")]
    UrlSinEsquema(String),
    #[error("no se pudo leer la configuración")]
    Lectura(#[from] std::io::Error),
}
```

Ce fragment illustre une API de bibliothèque, il ne fait pas partie du revisor actuel. `#[from]` permet de convertir automatiquement un `std::io::Error` en `ErrorConfiguracion`, de sorte que `?` reste utile. Les autres variantes conservent des données que la personne qui appelle peut inspecter au moyen de `match`. Dans une application à une seule couche, convertir ces erreurs en `anyhow::Error` à la fin peut être commode ; dans une bibliothèque, les cacher trop tôt retire des options à qui l'utilise.

La frontière pratique est celle-ci : `anyhow` pour exécuter une application et expliquer un échec complet ; `thiserror` pour offrir un contrat d'erreurs que d'autres programmes doivent gérer par variante. Tu peux combiner les deux, mais ne les ajoute pas par mode. Commence par le type qui permet à la couche suivante de prendre la bonne décision.

## L'erreur que tu vas voir

### E0277 : utiliser `?` dans une fonction qui ne peut pas renvoyer d'erreur

L'erreur la plus fréquente quand on commence à utiliser `?` apparaît quand la fonction déclare un retour simple, comme `String`, mais essaie à l'intérieur de propager un `Result`. Rust ne peut pas inventer où stocker l'erreur ni comment la communiquer à l'appelant. L'exécution réelle suivante, avec `rustc 1.98.1`, lit le programme depuis l'entrée standard, c'est pourquoi le compilateur nomme le fichier `<anon>` ; avec un fichier sur disque, tu verras son nom à la place de `<anon>`.

<!-- verificar:fragmento -->
```rust
fn leer() -> String {
    let texto = std::fs::read_to_string("faltante.txt")?;
    Ok(texto)
}

fn main() {}
```

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> <anon>:2:56
  |
1 | fn leer() -> String {
  | ------------------- this function should return `Result` or `Option` to accept `?`
2 |     let texto = std::fs::read_to_string("faltante.txt")?;
  |                                                        ^ cannot use the `?` operator in a function that returns `String`

error[E0308]: mismatched types
 --> <anon>:3:5
  |
1 | fn leer() -> String {
  |              ------ expected `String` because of return type
2 |     let texto = std::fs::read_to_string("faltante.txt")?;
3 |     Ok(texto)
  |     ^^^^^^^^^ expected `String`, found `Result<String, _>`
  |
  = note: expected struct `String`
               found enum `Result<String, _>`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

`E0277` dit que `?` a besoin d'une fonction capable de renvoyer un résidu d'erreur. `E0308` en est la conséquence : `Ok(texto)` est un `Result`, mais la signature promettait un `String`. La correction n'est pas de retirer `?` et d'utiliser `unwrap()`. Tu dois corriger le contrat pour qu'il décrive la possibilité réelle d'échec.

<!-- verificar:fragmento -->
```rust
fn leer() -> Result<String, std::io::Error> {
    let texto = std::fs::read_to_string("faltante.txt")?;
    Ok(texto)
}
```

Si tu es dans `main`, tu peux aussi renvoyer `Result` quand l'erreur doit terminer le programme. Cependant, le revisor actuel a besoin de contrôler ce qui est affiché et quel code de sortie il renvoie, c'est pourquoi `main` transforme le résultat de `config::cargar` en une sortie visible et en `ExitCode::from(2)`. Le message va sur `stderr` ; le tableau ou le JSON réussis restent disponibles sur `stdout`.

### Une panique par indice hors de la plage

L'accès `v[indice]` n'est pas une erreur de compilation si `indice` est une variable. Rust ne peut pas savoir à la compilation quel nombre arrivera. Si le nombre tombe hors de la plage, le programme entre en panique pendant l'exécution. Le diagnostic mentionne l'indice demandé et la longueur réelle du vecteur. Par exemple, demander la position `3` d'une liste de trois éléments échoue parce que les positions valides sont `0`, `1` et `2`.

La correction dépend de l'origine de l'indice. Si c'est une constante écrite à côté d'une liste fixe et que l'indice est faux, corrige le programme. S'il vient d'un fichier, d'une requête ou d'une option utilisateur, utilise `get(indice)` et convertis `None` en un `Result` qui explique quelle était la plage valide. Ne laisse pas une entrée récupérable terminer le processus avec un backtrace.

### Diagnostiquer avant de réparer

Lis d'abord la signature de la fonction et le type concret de l'expression qui a échoué. S'il dit `Option`, décide ce que signifie l'absence. S'il dit `Result`, lis la variante d'erreur et décide si elle doit être propagée, contextualisée ou gérée là. Si `panic!` apparaît, demande-toi quelle hypothèse du programme s'est rompue. Copier `.clone()`, `.unwrap()` ou `as` jusqu'à ce que ça compile efface souvent des informations utiles et déplace le problème vers l'exécution.

`rustc --explain E0277` développe la signification générale du code, mais le diagnostic local reste la source principale. Ses flèches désignent le type attendu, le type trouvé et la ligne où le contrat a cessé de coïncider. Apprendre à suivre ces trois pistes vaut mieux que mémoriser une liste de codes.

## Ce qui se fait mal

- **Utiliser `v[i]` pour des indices qui viennent de l'extérieur.** L'indice direct affirme que la position existe. Si cette affirmation dépend d'un fichier ou d'une saisie humaine, utilise `get` ; `None` est une donnée que tu dois transformer en explication utile.

- **Parcourir un `HashMap` et publier son ordre accidentel.** Un rapport qui change d'ordre est difficile à lire, à tester et à comparer. Extrais les clés ou les lignes, trie-les et seulement ensuite affiche. La sortie du revisor doit être reproductible même si le stockage interne change.

- **Utiliser `String` dans tous les paramètres.** Cela oblige à transférer la propriété ou à créer des allocations inutiles. Si une fonction ne fait que lire, déclare `&str` ; réserve `String` aux structures et aux résultats qui doivent posséder du texte.

- **Indexer un `String` par octet ou supposer que `len()` compte les lettres visibles.** Rust stocke du texte UTF-8. Utilise `chars`, `bytes` ou des plages avec `get` selon l'unité dont tu as réellement besoin. Un nom qui ne contient aujourd'hui que de l'ASCII peut contenir demain des caractères valides de plus d'un octet.

- **Convertir toutes les erreurs en `unwrap()` ou `expect()`.** Cela fait paraître le programme court tout en supprimant des chemins de récupération et du contexte. `unwrap` est acceptable dans un test quand l'échec invalide le test lui-même ; ce n'est pas la façon normale de lire des fichiers ni de traiter des arguments.

- **Utiliser `panic!` pour des données de configuration invalides.** Une URL incorrecte, un fichier absent ou un nom répété sont des échecs qu'une personne peut corriger. Renvoie un `Result` avec la donnée manquante et une cause compréhensible.

- **Perdre la cause d'origine en créant un nouveau message.** Un texte comme `"no se pudo cargar"` ne dit pas quel chemin a échoué ni pourquoi. Utilise `with_context` pour ajouter l'opération et les données locales sans jeter la cause renvoyée par le système ou le parseur.

- **Utiliser `anyhow` dans une bibliothèque qui a besoin d'erreurs distinguables.** Si l'appelant doit réagir différemment à une URL invalide et à un nom répété, publie un enum propre, normalement avec `thiserror`. La commodité d'une chaîne ne doit pas effacer des décisions de domaine.

## Exercices

### Exercice 1 — Accès honnête à une liste

Crée un `Vec<Servicio>` avec deux services. Écris une fonction `nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str>` qui renvoie le nom du service quand il existe et `None` sinon. Teste-la avec les indices `0`, `1` et `2`. N'utilise pas `[]` dans la fonction.

Explique par écrit pourquoi renvoyer `Option<&str>` est plus honnête que renvoyer une chaîne vide. Réfléchis à ce qui se passerait si un nom vide était une donnée autorisée.

### Exercice 2 — États par nom et rapport déterministe

Crée un `HashMap<String, Estado>` avec trois noms, dont un échec. Écris une fonction qui produit un `Vec<String>` avec les noms triés par ordre alphabétique. Ensuite, parcours ces noms et génère des lignes au format `nombre: estado`.

Lance le programme plusieurs fois. La sortie doit conserver exactement le même ordre. Ne trie pas le `HashMap` : il ne se trie pas. Trie une collection séparée de clés ou de lignes.

### Exercice 3 — Charger, valider et contextualiser

Écris une fonction `cargar(ruta: &str) -> Result<Vec<Servicio>, ...>` qui lit un fichier texte avec une ligne par service. Chaque ligne doit contenir un nom et une URL séparés par une virgule. Rejette une ligne sans deux champs, une URL sans `http://` ni `https://`, et une liste vide. Propage les erreurs de lecture avec `?`.

Ensuite, adapte la fonction pour utiliser `anyhow::Context` et ajouter le chemin au message de lecture. Fais en sorte que `main` affiche les erreurs sur la sortie d'erreur et termine avec le code `2` ; si toutes les données sont valides, affiche la liste triée et termine avec le code `0`.

## Solutions

### Solution 1

La fonction doit emprunter la slice, pas prendre le vecteur. `get` renvoie déjà `Option<&Servicio>`, et `map` transforme le contenu de `Some` sans toucher à `None`.

<!-- verificar:fragmento -->
```rust
fn nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str> {
    servicios.get(indice).map(|servicio| servicio.nombre.as_str())
}
```

`as_str()` convertit la référence à `String` en une référence à `str` ; elle n'alloue pas de mémoire et ne clone pas le texte. La durée de vie du `&str` est limitée par celle de la slice empruntée, ce qui est précisément le bon contrat. Une chaîne vide serait ambiguë : elle pourrait signifier « je n'ai pas trouvé cet indice » ou « j'ai bien trouvé le service et son nom est vide ».

### Solution 2

La solution doit séparer la structure utile pour chercher de la structure utile pour présenter. La table conserve l'association ; le vecteur de clés reçoit l'ordre qu'exige le rapport.

<!-- verificar:fragmento -->
```rust
let mut nombres: Vec<&str> = estados.keys().map(String::as_str).collect();
nombres.sort();

for nombre in nombres {
    let estado = &estados[nombre];
    println!("{nombre}: {estado:?}");
}
```

L'accès `estados[nombre]` est raisonnable ici parce que `nombre` provient directement de `estados.keys()` : le programme a déjà démontré que la clé existe. Si `nombre` venait d'un fichier ou d'un argument, cette forme réaffirmerait quelque chose que tu n'as pas validé et tu devrais utiliser `get`.

### Solution 3

La signature de chargement doit laisser les problèmes externes remonter sous forme de `Result`. Les règles de format doivent aussi devenir des erreurs, pas des paniques. Si tu utilises `anyhow`, une implémentation peut suivre cette forme :

<!-- verificar:fragmento -->
```rust
fn cargar(ruta: &str) -> anyhow::Result<Vec<Servicio>> {
    let texto = std::fs::read_to_string(ruta)
        .with_context(|| format!("leyendo {ruta}"))?;

    let mut servicios = Vec::new();
    for (numero, linea) in texto.lines().enumerate() {
        let (nombre, url) = linea
            .split_once(',')
            .with_context(|| format!("línea {} sin coma", numero + 1))?;

        anyhow::ensure!(
            url.starts_with("http://") || url.starts_with("https://"),
            "línea {}: URL sin esquema: {url}",
            numero + 1
        );

        servicios.push(Servicio::new(nombre, url));
    }

    anyhow::ensure!(!servicios.is_empty(), "el archivo no declara servicios");
    Ok(servicios)
}
```

La solution n'utilise pas `unwrap` parce que chaque échec peut provenir d'un contenu externe. `split_once` renvoie `Option` ; `with_context` le convertit en une erreur explicative. `ensure!` termine la fonction avec `Err` si la condition n'est pas remplie. Le contexte inclut le numéro de ligne ou le chemin pour que la personne qui corrige le fichier n'ait pas à deviner par où commencer.

## Comment savoir que j'y suis arrivé

- Tu exécutes `rustc --edition 2024 fig04_01.rs && ./fig04_01` et tu obtiens exactement six lignes, dont `None` pour `pagos` et `Some(2)` pour le compteur.
- Tu exécutes `rustc --edition 2024 fig04_02.rs && ./fig04_02` et tu peux expliquer pourquoi le même paramètre `&str` accepte un littéral et une référence à `String`.
- Tu exécutes `rustc --edition 2024 fig04_03.rs && ./fig04_03` et tu obtiens une erreur de fichier absent sans panique.
- Ta solution de l'exercice 1 renvoie `None` pour l'indice hors de la plage et ne contient pas d'accès avec `servicios[indice]`.
- Ton rapport de l'exercice 2 produit les mêmes lignes, dans le même ordre, après au moins dix exécutions.
- Ta solution de l'exercice 3 nomme le chemin quand le fichier n'existe pas, nomme la ligne quand le format est incorrect et n'utilise pas `unwrap` dans le chemin normal d'exécution.
- Depuis `programas/revisor`, `cargo test`, `cargo clippy --all-targets -- -D warnings` et `cargo fmt --check` se terminent correctement.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 8 : collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html), en particulier les vecteurs, les chaînes et les tables de hachage. Consulté le 2 octobre 2026.

- [The Rust Programming Language, chapitre 9 : gestion des erreurs](https://doc.rust-lang.org/book/ch09-00-error-handling.html). Consulté le 2 octobre 2026.

- [Documentation officielle de `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) et [de `HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html). Consulté le 2 octobre 2026.

- [Documentation d'`anyhow`](https://docs.rs/anyhow/) et [documentation de `thiserror`](https://docs.rs/thiserror/). Consulté le 2 octobre 2026.
