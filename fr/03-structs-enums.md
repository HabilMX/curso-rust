# Leçon 3 — Structs, enums et match

**Durée :** 2 × 45 min.

**Ce que tu construis :** le modèle du `revisor` : `Servicio` et `Estado`

**Ce que tu apprends :** structs et `impl`, enums qui portent des données, `match` exhaustif, `Option` à la place de `nil`

**The Rust Book, chapitres 5 et 6.** Rustlings : `structs`, `enums`, `options`.

## À la fin, tu vas pouvoir

- Modéliser un service avec une `struct` dont les champs ont un nom et un type.
- Écrire des méthodes dans un bloc `impl` et décider si elles reçoivent `&self`, `&mut self` ou `self`.
- Représenter des résultats incompatibles entre eux avec un `enum` qui porte des données.
- Écrire un `match` exhaustif qui extrait les données de chaque variante.
- Expliquer pourquoi l'ajout d'une nouvelle variante oblige à revoir des décisions existantes.
- Utiliser `Option<T>` quand une valeur peut manquer, sans recourir à `nil`.
- Choisir entre `match`, `if let` et des méthodes comme `unwrap_or` selon l'intention du code.

## Le pourquoi avant le comment

Jusqu'ici, le cours a utilisé des valeurs simples : des nombres pour les temps, des chaînes pour les noms et des conditions pour classer une réponse. Cela suffit pour s'exercer aux variables, aux fonctions, aux types et à la possession (ownership), mais ne suffit pas pour décrire le domaine réel du `revisor`. Un service n'est pas seulement un nom, une URL et un temps limite qui apparaissent par hasard ensemble dans trois variables. Ce sont trois données qui décrivent une seule chose et qui doivent circuler, être validées et être consultées comme une unité.

Stocker ces données séparément produit des erreurs silencieuses. Imagine que tu as `nombre_catalogo`, `url_catalogo`, `timeout_catalogo`, puis que tu ajoutes les trois mêmes valeurs pour les paiements et les rapports, et qu'en construisant le rapport tu associes le nom des paiements avec l'URL du catalogue. Le compilateur ne peut pas détecter le problème : les trois pièces ont des types valides, mais leur relation s'est perdue. Une `struct` permet de déclarer cette relation une fois et d'en faire une partie du type.

Le deuxième problème apparaît après avoir interrogé un service. Une réponse saine apporte un code HTTP et une durée. Une réponse lente apporte aussi les deux données, mais exige une étiquette différente. Un échec peut apporter un message et une durée, mais pas nécessairement un code HTTP. Et un service qui n'a pas encore été interrogé n'a ni code, ni durée, ni message d'échec. Si tu essayais de tout stocker dans une seule `struct` avec des champs « parfois valides », tu aurais des combinaisons absurdes : un état d'échec avec un code `200`, un service non tenté avec une durée de `0 ms` que personne ne sait interpréter, ou un message d'erreur vide qui signifie des choses différentes selon un autre champ booléen.

Rust résout cette modélisation avec `enum`. À la différence d'un enum traditionnel d'autres langages, qui est généralement une liste de nombres ou de constantes nommées, une variante de Rust peut porter des données. `Estado::Ok` porte un code et des millisecondes ; `Estado::Falla` porte un motif ; `Estado::NoIntentado` ne porte rien parce qu'il n'y a aucune donnée honnête à stocker. Le type exprime qu'une valeur est dans exactement l'un de ces états, jamais dans plusieurs à la fois.

La troisième pièce est `match`. Quand tu reçois un `Estado`, il ne suffit pas de savoir qu'il appartient à l'enum : tu dois décider quoi faire pour chaque possibilité. Rust exige que cette décision couvre toutes les variantes. Ce n'est pas une recommandation de style ni une règle d'un linter ; cela fait partie de la compilation. Si demain tu ajoutes `Estado::Rechazado`, chaque `match` qui semblait terminé devient un endroit que le compilateur te signale à revoir. Cette obligation est un filet de sécurité pour les refactorisations.

Le cours de Go construit le même `revisor`, mais ici apparaît une différence importante entre les deux langages. En Go, un résultat se modélise généralement avec une `struct`, des champs à valeur zéro, des pointeurs et des conventions sur les champs présents. En Rust, le type peut représenter directement des alternatives incompatibles. Cela n'élimine pas la nécessité de penser au domaine, mais rend les bonnes décisions plus faciles à exprimer et les incohérences plus difficiles à compiler.

La dernière partie du modèle est l'absence. Dans beaucoup de langages, une référence peut être `null` ou `nil` même si son type ne le dit pas de façon visible. Le programme arrive à une ligne qui attendait un objet, reçoit une absence et échoue pendant l'exécution. Rust n'a pas de `nil`. Quand quelque chose peut manquer, son type le déclare au moyen de `Option<T>`. Cela oblige à prendre une décision avant d'utiliser le contenu : traiter `Some(valeur)`, traiter `None` ou fournir une alternative explicite. L'absence cesse d'être un accident caché et devient une partie du contrat de la fonction.

Cette leçon ne consiste pas à mémoriser toute la syntaxe des motifs (patterns). Elle consiste à apprendre à te demander quels états réels existent, quelles données appartiennent à chaque état et quelles décisions doivent changer quand le modèle change. Ces questions reviennent à la leçon 4 avec `Result`, à la 5 avec les traits, à la 6 avec les tests et à la 8 quand le `revisor` génère du JSON.

## Les concepts

### Structs : un nom pour des données qui vont ensemble

Une `struct` définit un type composé avec des champs nommés. Le mot important est « type » : après avoir défini `Servicio`, Rust cesse de voir une collection informelle de trois données et commence à voir une valeur qui représente un service. Chaque champ conserve son propre type, donc le compilateur continue à distinguer le texte des nombres, mais il sait maintenant aussi que ces valeurs forment une seule entité.

Une struct à champs nommés est un bon choix quand chaque position a un sens propre. Un n-uplet (tuple) comme `(String, String, u64)` peut stocker le nom, l'URL et le temps limite, mais oblige à se rappeler ce que signifient `.0`, `.1` et `.2`. Avec `Servicio`, le code dit `servicio.timeout_ms`, qui communique à la fois la donnée et son unité. La clarté n'est pas un ornement : elle réduit la possibilité d'intervertir des valeurs semblables et rend plus facile la lecture d'un code que tu as écrit il y a des semaines.

La création d'une struct utilise des accolades et des paires `champ: valeur`. L'accès est lui aussi direct, avec le point. Comme les champs de la figure appartiennent à `Servicio`, il n'y a pas d'état temporaire où existerait une URL sans nom ou un temps limite associé accidentellement à un autre service. Il reste possible de créer une valeur incorrecte, par exemple une URL sans schéma ; la leçon 4 enseignera comment la valider. Ce qui disparaît, c'est le désordre des variables éparses.

**Fig. 3.1** | Une struct avec ses méthodes.

```rust
// fig03_01.rs
#[derive(Debug, Clone)]          // el compilador te escribe esos comportamientos
struct Servicio {
    nombre: String,
    url: String,
    timeout_ms: u64,
}

impl Servicio {
    fn new(nombre: &str, url: &str) -> Self {     // no hay constructores: es convención
        Self { nombre: nombre.to_string(), url: url.to_string(), timeout_ms: 5000 }
    }
    fn etiqueta(&self) -> String {                // &self = presta, no consume
        format!("{} ({})", self.nombre, self.url)
    }
}

fn main() {
    let s = Servicio::new("catalogo", "http://localhost:8090/ok");
    println!("{}", s.etiqueta());
    println!("{:?}", s.clone());
    println!("timeout: {} ms", s.timeout_ms);
}
```

```bash
$ rustc --edition 2024 fig03_01.rs && ./fig03_01
catalogo (http://localhost:8090/ok)
Servicio { nombre: "catalogo", url: "http://localhost:8090/ok", timeout_ms: 5000 }
timeout: 5000 ms
```

`#[derive(Debug, Clone)]` demande au compilateur d'implémenter des comportements connus pour le type. `Debug` permet d'afficher une représentation utile avec `{:?}`. Ce n'est pas un format stable pour les utilisateurs finaux : c'est une vue pour le développement et le diagnostic. `Clone` permet de demander une copie explicite avec `.clone()`. Dans cet exemple, il sert seulement à démontrer que la structure peut être affichée puis rester disponible ; tu ne dois pas copier des valeurs par habitude pour éteindre des erreurs de possession.

Le `revisor` utilise le même modèle, mais ajoute les attributs nécessaires pour lire des services depuis du YAML. `pub` indique que d'autres modules du crate peuvent accéder à ces champs. `Deserialize` et `serde` apparaîtront en profondeur dans les leçons suivantes ; pour l'instant, observe que le noyau reste le même : nom, URL et limite de temps.

<!-- verificar:extracto:src/modelo.rs -->
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}
```

L'attribut `#[serde(default = "timeout_por_omision")]` ne change pas ce qu'est un service. Il décrit une règle d'entrée : si le YAML ne déclare pas `timeout_ms`, le programme utilise cinq secondes. Il est important de distinguer modélisation et validation. La struct déclare quelles données forment un service ; les règles sur le fait qu'une URL ait un schéma, que le temps soit supérieur à zéro ou que le nom se répète sont vérifiées ensuite, quand le programme reçoit une liste.

### `impl` et méthodes : le comportement qui appartient au type

Une `struct` stocke des données, mais le type peut aussi avoir des opérations qui ont du sens pour ces données. Rust regroupe ces opérations dans un bloc `impl`. Le nom signifie implementation : une implémentation de comportement pour un type. Il n'y a pas de mot réservé spécial pour les constructeurs. Par convention, une fonction associée appelée `new` crée une nouvelle valeur, mais elle reste une fonction normale dans `impl`.

La figure utilise deux façons d'appeler des fonctions dans un `impl`. `Servicio::new(...)` utilise `::` parce que `new` n'a pas encore d'instance sur laquelle travailler. `s.etiqueta()` utilise `.` parce que `etiqueta` reçoit un service concret. Rust permet cette syntaxe de méthode quand le premier paramètre s'appelle `self`, `&self` ou `&mut self`.

`&self` signifie « prête cette valeur en lecture ». La méthode peut regarder `nombre` et `url`, construire une nouvelle chaîne et la renvoyer, mais elle ne prend pas la propriété de `s` et ne modifie pas ses champs. C'est pourquoi, après `s.etiqueta()`, tu peux encore afficher `s`, lire `s.timeout_ms` ou prêter le service à une autre fonction. C'est l'application directe des emprunts de la leçon 2 à une fonction qui vit à côté de son type.

`&mut self` signifie « prête cette valeur pour la modifier ». Une méthode comme `fn cambiar_timeout(&mut self, ms: u64)` exigerait que celui qui appelle déclare une variable mutable et n'autoriserait aucune autre référence active vers la même valeur. Le compilateur applique les mêmes règles que tu as déjà vues avec `&mut String` : une seule référence mutable à la fois, ou plusieurs références immuables, mais pas les deux sortes simultanément.

`self` sans `&` consomme la valeur. C'est une décision délibérée et moins courante. Elle est utile quand la méthode transforme une valeur en une autre et que l'original ne doit plus exister, par exemple une opération qui convertit une configuration temporaire en une structure validée. Ce n'est pas une façon plus rapide d'écrire `&self` : elle change qui possède la valeur. Si tu reçois une erreur de « valeur déplacée » après avoir appelé une méthode, vérifie d'abord son récepteur.

Dans le projet réel, `Servicio::new` concentre la valeur par défaut. Cela évite que chaque appel doive répéter `5000` et réduit le risque que certains services soient créés avec une règle différente sans le vouloir.

<!-- verificar:extracto:src/modelo.rs -->
```rust
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

`Self` dans `impl Servicio` signifie `Servicio`. L'utiliser évite de répéter le nom du type et conserve l'intention si le type change de nom pendant une refactorisation. L'expression `Self { ... }` construit la valeur ; `-> Self` déclare le type que renvoie la fonction. La constante `TIMEOUT_POR_OMISION_MS` se trouve hors de l'extrait et évite que le nombre cinq mille soit réparti dans le programme comme une valeur magique.

### Enums avec données : des alternatives valides, pas des champs ambigus

Un `enum` décrit une valeur qui peut prendre l'une de plusieurs variantes. La différence essentielle avec une collection de constantes est que chaque variante peut avoir sa propre forme. `Estado::Ok` et `Estado::Lento` ont des champs nommés `codigo` et `ms`. `Estado::Falla` stocke une chaîne. `Estado::NoIntentado` ne porte pas de données parce qu'aucune consultation n'a eu lieu qui produise des résultats honnêtes.

Cette conception évite de représenter un échec par un code spécial, comme `0`, `-1` ou une chaîne vide. Ces marqueurs obligent à se rappeler des règles extérieures au type : « si le code est zéro, lis l'erreur ; si l'erreur est vide, c'était peut-être un succès ; si le temps est zéro, ce n'était peut-être pas tenté ». Un enum déplace ces règles vers le compilateur. Si tu as `Estado::Falla`, Rust sait qu'il y a un message ; si tu as `Estado::Ok`, Rust sait qu'il y a un code et une durée.

Il évite aussi la combinaison impossible de champs optionnels. Une struct comme `Resultado { codigo: Option<u16>, error: Option<String>, ms: u64 }` admet par construction aussi bien `codigo: Some(200), error: Some("no responde")` que `codigo: None, error: None`. Il peut y avoir des cas légitimes pour une telle forme, surtout lors de la sérialisation de données externes, mais ce n'est pas une bonne représentation interne d'alternatives mutuellement exclusives. Pour l'état d'une consultation, l'enum exprime mieux la réalité.

<!-- verificar:fragmento -->
```rust
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}
```

La syntaxe a trois formes qu'il convient de reconnaître. Les variantes à accolades ressemblent à de petites structs et permettent de nommer les champs à la création et à la déstructuration. Les variantes à parenthèses ressemblent à des n-uplets et servent quand la donnée a un sens principal clair, comme le message d'un échec dans ce fragment. Les variantes sans données représentent une possibilité qui n'a pas besoin d'information supplémentaire.

**Fig. 3.2** | Un enum avec données et son `match`.

```rust
// fig03_02.rs
#[allow(dead_code)]              // este ejemplo no lee el código de los lentos
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),                      // lleva el mensaje dentro
    NoIntentado,
}

fn main() {
    let estados = [
        Estado::Ok { codigo: 200, ms: 120 },
        Estado::Lento { codigo: 200, ms: 1800 },
        Estado::Falla("no responde".to_string()),
        Estado::NoIntentado,
    ];

    for estado in estados {
        let texto = match estado {
            Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
            Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
            Estado::Falla(msg)           => format!("FALLA: {msg}"),
            Estado::NoIntentado          => "sin revisar".to_string(),
        };
        println!("{texto}");
    }
}
```

```bash
$ rustc --edition 2024 fig03_02.rs && ./fig03_02
OK 200 en 120ms
LENTO 1800ms
FALLA: no responde
sin revisar
```

L'annotation `#[allow(dead_code)]` appartient à l'exemple ; ce n'est pas une recette pour masquer des avertissements dans des projets réels. La variante `Lento` conserve `codigo` parce qu'une réponse lente peut avoir été un HTTP 200, même si ce programme n'utilise que `ms`. Sans l'annotation, Rust avertirait que le champ `codigo` de cette variante n'est pas lu dans ce fichier. Le projet réel utilise bien les données là où il faut et se compile avec les avertissements traités comme des erreurs.

Le `revisor` améliore le fragment initial avec deux décisions de domaine. D'abord, un échec porte à la fois `motivo` et `ms`, parce que savoir qu'une connexion a épuisé son temps après une certaine durée est une information utile pour le rapport. Ensuite, l'enum reçoit `derive(Debug, Clone, PartialEq)`. `PartialEq` permet de comparer des états dans les tests avec `assert_eq!`, ce que tu utiliseras à la leçon 6.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Lo que se supo de un servicio después de consultarlo.
///
/// Cada variante lleva sus propios datos: así `Falla` no tiene código HTTP que
/// alguien pueda leer por error, y `Ok` no tiene mensaje de error.
#[derive(Debug, Clone, PartialEq)]
pub enum Estado {
    /// Contestó con 2xx a tiempo.
    Ok { codigo: u16, ms: u64 },
    /// Contestó con 2xx, pero tardó más de [`UMBRAL_LENTO_MS`].
    Lento { codigo: u16, ms: u64 },
    /// No contestó, contestó con error, o se acabó el tiempo.
    Falla { motivo: String, ms: u64 },
    /// Nunca se llegó a consultar.
    NoIntentado,
}
```

L'enum ne remplace pas tout usage de booléens ni tout usage de structs. Un booléen reste correct pour une question à deux réponses simples, comme « le service est-il sain ? ». Une struct reste correcte pour des données qui existent ensemble au même moment, comme le nom, l'URL et la limite. Un enum convient quand les possibilités ont des formes différentes et que le programme doit les traiter différemment.

### `match` : décider pour chaque état sans laisser de trous

`match` compare une valeur à des motifs (patterns) et produit un résultat. Dans la figure, chaque branche a la forme `motif => expression`. Le motif identifie une variante et peut en extraire les données. Dans `Estado::Ok { codigo, ms }`, les noms entre accolades créent des variables locales appelées `codigo` et `ms`. Dans `Estado::Falla(msg)`, `msg` reçoit la chaîne que porte la variante.

Le motif `..` signifie « ignore les autres champs ». Dans la branche de `Lento`, le programme a besoin de la durée pour l'afficher, mais pas du code. C'est préférable à inventer un nom comme `_codigo` quand tu ne vas pas l'utiliser : cela communique que la donnée existe et que cette décision n'en dépend pas. Si tu n'as besoin d'aucun champ d'une variante à données, tu peux écrire `Estado::Ok { .. }`.

Un `match` est une expression. C'est pourquoi la figure peut faire `let texto = match estado { ... };`. Chaque branche renvoie un `String` : trois utilisent `format!` et la dernière en construit un avec `"sin revisar".to_string()`. Rust exige que toutes les branches produisent des types compatibles. Cette règle évite qu'un chemin renvoie du texte et qu'un autre, accidentellement, ne renvoie rien.

La propriété la plus précieuse est l'exhaustivité. Le compilateur connaît toutes les variantes de `Estado` parce qu'elles sont déclarées dans le même type. Si un `match` n'en couvre pas une, il ne compile pas. C'est plus sûr qu'un `switch` qui permet de tomber sans action, et c'est aussi plus explicite qu'une chaîne de `if` qui laisse un cas comme possibilité implicite.

Dans certains cas, tu utiliseras un motif joker, `_ => ...`, pour regrouper des possibilités qui doivent réellement recevoir le même traitement. C'est valide, mais cela a un coût : si tu ajoutes une nouvelle variante, cette branche l'acceptera déjà sans t'obliger à te demander si le comportement correct est le même. Pour un enum central comme `Estado`, il vaut mieux préférer des branches explicites dans le rapport. Ainsi, une nouvelle variante devient une décision visible, pas un comportement accidentel.

**Fig. 3.3** | Si tu oublies une variante, ça ne compile pas.

```rust
// fig03_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}

fn main() {
    let estado = Estado::NoIntentado;
    let texto = match estado {
        Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
        Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
        Estado::Falla(msg)           => format!("FALLA: {msg}"),
    };
    println!("{texto}");
}
```

```bash
$ rustc --edition 2024 fig03_03.rs
error[E0004]: non-exhaustive patterns: `Estado::NoIntentado` not covered
  --> fig03_03.rs:11:23
   |
11 |     let texto = match estado {
   |                       ^^^^^^ pattern `Estado::NoIntentado` not covered
   |
note: `Estado` defined here
  --> fig03_03.rs:2:6
   |
 2 | enum Estado {
   |      ^^^^^^
...
 6 |     NoIntentado,
   |     ----------- not covered
   = note: the matched value is of type `Estado`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
14 ~         Estado::Falla(msg)           => format!("FALLA: {msg}"),
15 ~         Estado::NoIntentado => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

Dans le `revisor`, `match` apparaît dans le module des rapports pour convertir un état technique en une étiquette qu'une personne peut lire. Observe que chaque variante est nommée explicitement. Le code ne présuppose pas que « tout ce qui n'est pas OK » est un échec : `Lento` et `NoIntentado` ont leur propre sens.

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn etiqueta(e: &Estado) -> &'static str {
    match e {
        Estado::Ok { .. } => "OK",
        Estado::Lento { .. } => "LENTO",
        Estado::Falla { .. } => "FALLA",
        Estado::NoIntentado => "NO",
    }
}
```

Le paramètre est `&Estado`, une référence immuable. Le rapport doit lire l'état plusieurs fois pour obtenir l'étiquette, la durée et le détail, donc il ne convient pas de le consommer. Rust permet de faire `match` sur une référence : les motifs lisent les champs dont ils ont besoin sans déplacer l'`Estado` d'origine. Cette combinaison d'emprunts et de motifs sera très fréquente dans le code Rust.

Il existe aussi `matches!`, une macro utile quand tu veux seulement une réponse booléenne. La méthode `esta_bien` du projet n'a pas besoin de produire un texte ni d'extraire des codes ; elle demande si la valeur est l'une de deux variantes saines. Regrouper des motifs avec `|` exprime cette règle sans répéter la logique.

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

N'utilise pas `matches!` pour remplacer un `match` qui doit transformer des données. Son résultat est toujours un `bool` ; c'est une question, pas une décision complète. Quand le programme a besoin de construire un rapport, d'obtenir une durée ou de choisir un détail précis, `match` reste l'outil adéquat.

### `Option<T>` : l'absence déclarée dans le type

Rust n'a ni `null` ni `nil`. Une valeur de type `String` est toujours une chaîne valide ; une valeur de type `&Servicio` est toujours une référence valide tant que l'emprunt est valide. Quand une donnée peut manquer, son type doit le déclarer. La forme standard est `Option<T>`.

<!-- verificar:fragmento -->
```rust
enum Option<T> {
    Some(T),
    None,
}
```

La définition réelle appartient à la bibliothèque standard et comporte plus d'attributs internes, mais ce fragment montre son idée centrale. `Option<u16>` signifie « il peut y avoir un code `u16`, ou il peut ne pas y en avoir ». `Option<Servicio>` signifie « une recherche peut renvoyer un service ou n'en trouver aucun ». Le type ne décide pas quoi faire face à l'absence ; il oblige celui qui consomme la valeur à le faire de façon explicite.

L'absence n'est pas toujours une erreur. Chercher un service par nom peut ne pas le trouver parce que le nom n'est pas configuré. Un en-tête HTTP peut être optionnel. Un échec réseau peut ne pas produire de code HTTP. Dans ces cas, `Option` communique que le manque de valeur est une possibilité prévue par le contrat, et non une valeur secrète comme `0`, `""` ou un pointeur nul qui pourrait exploser plus tard.

**Fig. 3.4** | Consommer un `Option`.

```rust
// fig03_04.rs
fn main() {
    let quizas: Option<u16> = Some(200);

    match quizas {
        Some(c) => println!("código {c}"),
        None    => println!("sin respuesta"),
    }

    if let Some(c) = quizas { println!("código {c}"); }     // cuando solo importa un caso
    let c = quizas.unwrap_or(0);                            // valor por omisión
    println!("{c}");
}
```

```bash
$ rustc --edition 2024 fig03_04.rs && ./fig03_04
código 200
código 200
200
```

La première consommation utilise `match` parce que les deux cas comptent : il y a une sortie pour `Some` et une autre pour `None`. La seconde utilise `if let` parce qu'il n'y a du travail à faire que quand un code existe ; s'il n'existe pas, le programme n'a rien à faire. `if let Some(c) = quizas` est une façon brève d'écrire un `match` dont l'autre branche serait `_ => {}`.

`unwrap_or(0)` renvoie le contenu quand il existe et la valeur par défaut quand il n'existe pas. La décision d'utiliser `0` n'est correcte que si l'appelant comprend que zéro représente « pas de réponse » dans ce contexte. Dans un rapport HTTP public, il peut être plus clair de conserver `Option<u16>` jusqu'au point où la donnée est présentée, pour ne pas confondre une absence avec un vrai code HTTP.

Ne confonds pas `unwrap_or` avec `unwrap`. `unwrap()` dit : « je sais qu'il y a ici une valeur ; s'il n'y en a pas, termine le programme avec un `panic!` ». Cela peut être raisonnable dans un test où l'absence démontre que la préparation du cas a échoué, mais dans du code d'application, cela cache souvent une décision en suspens. `clippy`, avec sa configuration par défaut, ne signale pas un `unwrap` ordinaire ; il existe un avertissement optionnel (`clippy::unwrap_used`) que celui qui maintient un projet peut activer pour l'interdire. La leçon 6 montrera comment exécuter `clippy`. Avant de l'écrire, demande-toi si `None` peut se produire en production. Si c'est possible, tu dois le gérer.

Le projet utilise `Option` pour la forme JSON du rapport. Un échec n'a pas de code HTTP inventé, c'est pourquoi `codigo` est `Option<u16>`. Le champ `error` est aussi optionnel : il apparaît dans un échec ou dans un service non tenté, mais il est omis pour une réponse saine. Cette struct représente une sortie sérialisable ; elle ne remplace pas l'enum interne `Estado` ; les deux types ont des responsabilités différentes.

<!-- verificar:extracto:src/modelo.rs -->
```rust
#[derive(Serialize)]
pub struct EstadoJson {
    pub servicio: String,
    pub codigo: Option<u16>,
    pub ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

Cette séparation est utile. `Estado` modélise des alternatives exclusives pour que la logique interne soit sûre. `EstadoJson` modélise la forme qu'un outil externe s'attend à lire, où certains champs peuvent être `null` ou omis. Rust n'empêche pas une API externe d'avoir des valeurs optionnelles ; il empêche ta logique interne de les traiter comme si elles existaient toujours.

## L'erreur que tu vas voir

### `E0004` : un `match` ne couvre pas tous les motifs

La figure 3.3 produit `E0004`, « non-exhaustive patterns ». Le compilateur a vu que `estado` est de type `Estado`, a lu la définition de l'enum et a vérifié qu'il existe la variante `NoIntentado`. Ensuite, il a parcouru les branches du `match` et n'a trouvé aucun motif qui la couvre.

La flèche sous `estado` indique la valeur sur laquelle porte la décision. La note qui suit montre où `Estado` a été défini et souligne la variante manquante. L'aide propose deux corrections : ajouter une branche explicite pour `Estado::NoIntentado` ou ajouter un motif joker. Dans ce cas, la bonne correction est explicite, parce que « sin revisar » mérite une sortie visible :

<!-- verificar:fragmento -->
```rust
Estado::NoIntentado => "sin revisar".to_string(),
```

Ne recopie pas `todo!()` de la suggestion comme solution finale. Rust le propose parce qu'il complète le motif et laisse un marqueur visible pour que tu décides quoi faire ; si cette branche s'exécute, `todo!()` termine le programme avec un `panic!`. C'est utile pendant une courte refactorisation, pas comme comportement du `revisor`.

Cette erreur apparaît aussi quand tu ajoutes une nouvelle variante. C'est précisément l'un de ses avantages. Au lieu de dépendre d'une recherche manuelle dans le dépôt, laisse le type et le compilateur énumérer les endroits qui doivent décider comment répondre au nouvel état. Corrige chacun avec une règle métier, pas avec `_ =>` par réflexe.

### Lire l'erreur comme un guide de changement

Les erreurs de Rust comportent généralement quatre parties : le code stable comme `E0004`, l'emplacement, des notes de contexte et une aide. Commence par le code et la phrase principale ; dans ce cas, ils suffisent pour savoir qu'il manque une variante. Lis ensuite la note pour confirmer le type et l'aide pour connaître une forme syntaxique valide de correction.

L'aide du compilateur ne connaît pas ton domaine. Elle peut te dire comment compléter un `match`, mais elle ne peut pas décider si un nouvel état doit compter comme sain, échoué, lent ou non vérifié. Cette décision reste la tienne. L'avantage est que Rust sépare les deux problèmes : il te garantit que tu n'as pas oublié de traiter le cas et te laisse définir le bon traitement.

Tu peux demander une explication détaillée avec `rustc --explain E0004`. Fais-le quand tu rencontres un code d'erreur que tu ne comprends pas. Il n'est pas nécessaire de mémoriser les codes ; ce qui compte, c'est d'apprendre à reconnaître que ce sont des identifiants consultables et que le message contient des preuves concrètes sur le type et la ligne concernés.

## Ce qui se fait mal

### Modéliser des alternatives avec une struct pleine de champs optionnels

Une struct comme `Resultado { codigo: Option<u16>, error: Option<String>, ms: Option<u64> }` paraît flexible, mais accepte trop d'états incohérents. Elle peut contenir à la fois un code de succès et un message d'échec, ou ne contenir aucun des deux. Si ces cas sont invalides, le type devrait les rendre difficiles ou impossibles à construire.

Utilise un enum quand les possibilités s'excluent mutuellement et portent des données différentes. Garde les structs avec `Option` pour les limites externes, comme le JSON, les formulaires ou les configurations partielles, où tu as réellement besoin de représenter des champs pouvant manquer indépendamment.

### Utiliser `_ =>` pour faire taire l'exhaustivité

Le motif joker est correct quand toutes les variantes restantes reçoivent exactement le même traitement. Le problème apparaît quand on l'utilise seulement pour que le compilateur cesse de protester. Dans un enum métier, `_` peut transformer une nouvelle variante en échec générique ou, pire, en réponse saine par accident.

Dans `Estado`, écris les quatre branches de façon explicite. Si tu ajoutes une variante, accepte que le compilateur t'oblige à revoir le rapport et les tests. Le petit travail immédiat évite un comportement non revu plus tard.

### Écrire `unwrap()` sur un chemin normal du programme

`unwrap()` ne résout pas l'absence : il la convertit en `panic!`. Si un service peut ne pas être trouvé, si une réponse peut ne pas avoir de code ou si un fichier peut ne pas exister, l'absence fait partie de la réalité du programme. Elle doit se convertir en un `match`, un `if let`, une valeur par défaut justifiée ou, à la leçon 4, un `Result`.

Dans les tests, `unwrap()` peut être utile pour déclarer qu'un cas doit être correctement préparé. Dans la logique de production, utilise-le seulement quand tu as démontré que `None` est impossible et que l'échec représente une erreur de programmation, et non une condition prévisible.

### Copier avec `clone()` pour éviter de penser à la possession

`Clone` n'est pas une sortie automatique face à une erreur de déplacement. Copier un `Servicio` seulement pour le prêter à une fonction duplique ses `String` et peut cacher que la fonction devrait recevoir `&Servicio`. Dans la figure 3.1, `clone()` existe pour démontrer le trait et rendre visible la structure ; ce n'est pas la forme recommandée pour passer des services dans le programme.

Préfère des emprunts pour lire (`&Servicio`), des emprunts mutables quand il y a une vraie modification (`&mut Servicio`) et le déplacement quand la fonction doit prendre possession de la valeur. Ne copie que lorsque le programme a réellement besoin de deux valeurs indépendantes.

### Utiliser des nombres ou des chaînes magiques pour représenter des états

Représenter un échec par `codigo == 0`, une consultation en attente par `ms == 0` ou une erreur par `mensaje == ""` oblige à se rappeler des conventions que le type n'exprime pas. Cela complique aussi la réponse à des questions simples : une vraie réponse peut-elle durer zéro milliseconde ? un message vide est-il un échec ou l'absence d'échec ?

Nomme l'état avec une variante. `Estado::NoIntentado` communique plus qu'un nombre spécial et permet à `match` d'obliger à le traiter. Quand l'état a des données, place-les dans la variante qui les rend valides.

### Confondre `Option` et `Result`

`Option<T>` répond à « y a-t-il une valeur ou non ? ». `Result<T, E>` répond à « y a-t-il eu une valeur ou y a-t-il eu une erreur que j'ai besoin de connaître ? ». Une recherche qui ne trouve pas un nom peut renvoyer `Option<Servicio>` ; lire un fichier qui n'existe pas doit normalement renvoyer `Result<String, Error>`, parce que celui qui appelle a besoin de savoir ce qui s'est mal passé. La leçon 4 approfondit `Result` et `?`.

N'invente pas de messages d'erreur dans un `Option` et n'utilise pas `None` pour cacher un échec que l'utilisateur a besoin de diagnostiquer. Choisis le type selon le contrat de l'opération.

## Exercices

### Exercice 1 — Décris un service

Crée une `struct ServicioLocal` avec `nombre: String`, `url: String` et `timeout_ms: u64`. Écris une fonction associée `new(nombre: &str, url: &str) -> Self` qui affecte `3000` comme temps par défaut. Ajoute une méthode `etiqueta(&self) -> String` qui renvoie `nombre (url)`.

Dans `main`, crée un service appelé `pagos`, affiche l'étiquette puis affiche le temps limite. Vérifie que tu n'as besoin ni de `mut` ni de `clone()` pour ces opérations.

### Exercice 2 — Résume tous les états

Déclare un enum `EstadoLocal` avec les variantes `Ok { codigo: u16, ms: u64 }`, `Lento { codigo: u16, ms: u64 }`, `Falla(String)` et `NoIntentado`. Écris `fn resumen(estado: &EstadoLocal) -> String` en utilisant un `match` exhaustif.

Le résumé doit utiliser exactement ces formes : `OK 200 en 80ms`, `LENTO 1200ms`, `FALLA: sin conexión` et `sin revisar`. Teste-le avec une instance de chaque variante.

### Exercice 3 — Ajoute une variante et laisse Rust trouver le travail

Ajoute `Rechazado { codigo: u16, ms: u64 }` à `EstadoLocal`. Compile sans modifier `resumen` et observe `E0004`. Ajoute ensuite la branche qui produit `RECHAZADO 403 en 15ms`.

N'utilise pas `_ =>`. L'objectif est de constater que le compilateur signale une décision métier en suspens. Explique en une phrase pourquoi `Rechazado` ne doit pas être classé automatiquement comme `Falla` : le serveur a bien répondu, mais la réponse n'a pas été acceptée.

### Exercice 4 — Cherche sans utiliser `nil`

Crée un tableau ou un vecteur de deux `ServicioLocal` : `catalogo` et `pagos`. Écris une fonction qui reçoit une tranche (slice) de services et un nom, et renvoie `Option<&ServicioLocal>`. Cherche d'abord `pagos` puis `reportes`.

Consomme le premier résultat avec `if let` pour afficher son URL. Consomme le second avec `match` pour afficher `no existe reportes`. N'utilise pas d'indices avec une valeur sentinelle, de références nulles ni `unwrap()`.

## Solutions

### Solution 1

`ServicioLocal` doit être une struct à trois champs nommés. La fonction `new` doit utiliser `Self` et convertir `nombre` et `url` de `&str` en `String` ; la méthode `etiqueta` doit recevoir `&self`, puisqu'elle ne fait que lire les champs. Une sortie correcte contient :

```text
pagos (http://localhost:8091/ok)
timeout: 3000 ms
```

Si tu as besoin de déclarer `let mut servicio`, revois l'exercice : aucune opération demandée ne modifie la valeur. Si tu as besoin de `clone()`, tu as probablement changé une signature pour recevoir `self` alors qu'elle devait recevoir `&self`.

### Solution 2

`resumen` doit recevoir `&EstadoLocal` pour lire l'état sans le consommer. Elle doit avoir quatre branches explicites. La branche de `Ok` extrait `codigo` et `ms` ; celle de `Lento` peut utiliser `ms` et ignorer le code avec `..` ; celle de `Falla` extrait le message ; celle de `NoIntentado` renvoie le texte fixe.

Les quatre appels doivent produire ces lignes :

```text
OK 200 en 80ms
LENTO 1200ms
FALLA: sin conexión
sin revisar
```

Si une branche renvoie un `&str` et que les autres renvoient un `String`, fais en sorte que toutes produisent le même type. `format!` renvoie un `String` ; pour une étiquette fixe, tu peux utiliser `.to_string()`.

### Solution 3

En ajoutant `Rechazado`, la compilation doit échouer avec `E0004` tant que tu n'ajoutes pas une branche explicite. La bonne branche extrait les deux champs et produit :

```text
RECHAZADO 403 en 15ms
```

La solution ne consiste pas à changer la dernière branche en `_ => "FALLA"`. Cette forme ferait compiler le programme, mais perdrait la différence entre un réseau tombé et un serveur qui a répondu avec une politique d'autorisation. L'enum offre une nouvelle possibilité ; le `match` doit la convertir en une décision explicite.

### Solution 4

La fonction de recherche doit renvoyer `Option<&ServicioLocal>`, pas `Option<ServicioLocal>`. La référence permet de prêter le service trouvé depuis la liste sans copier ses chaînes ni le déplacer hors du vecteur. Une recherche peut parcourir les services et renvoyer le premier dont le `nombre` coïncide ; si elle n'en trouve aucun, elle renvoie `None`.

Pour `pagos`, `if let Some(servicio)` doit afficher son URL. Pour `reportes`, un `match` doit inclure les deux branches et produire :

```text
no existe reportes
```

Si le compilateur se plaint à propos des lifetimes, vérifie la signature : la référence de sortie doit provenir de la tranche d'entrée. Dans la plupart de ces cas, Rust peut inférer la bonne durée de vie (lifetime) sans que tu l'écrives. La leçon 5 expliquera les cas où tu dois la déclarer.

## Comment savoir que j'y suis arrivé

- [ ] Je compile la figure 3.1 avec `rustc --edition 2024 fig03_01.rs && ./fig03_01` et j'obtiens les trois lignes documentées.
- [ ] Je compile la figure 3.2 et je peux expliquer pourquoi chaque variante de `Estado` porte des données différentes.
- [ ] Je compile la figure 3.3, je vois `error[E0004]` et je la fais compiler en ajoutant une branche explicite pour `NoIntentado`.
- [ ] Je peux ajouter une variante à mon enum et localiser chaque décision en suspens grâce aux erreurs de `match`.
- [ ] Mon exercice 4 renvoie `Option<&ServicioLocal>` et gère à la fois `Some` et `None` sans `unwrap()`.
- [ ] Je peux expliquer pourquoi le `revisor` utilise un enum pour `Estado` et une struct avec `Option` pour `EstadoJson`.
- [ ] J'ai terminé les exercices de Rustlings `structs`, `enums` et `options`.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 5 : Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html) — consulté le 2 octobre 2026.
- [The Rust Programming Language, chapitre 6 : Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) — consulté le 2 octobre 2026.
- [Documentation officielle de `std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html) — consulté le 2 octobre 2026.
- [Rustlings : exercices de structs, enums et options](https://github.com/rust-lang/rustlings/tree/main/exercises) — consulté le 2 octobre 2026.
