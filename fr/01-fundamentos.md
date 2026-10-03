# Leçon 1 — Fondamentaux

**Durée :** 2 × 45 min.

**Ce que tu construis :** les fonctions et les types de base du `revisor`

**Ce que tu apprends :** variables et `mut`, masquage (shadowing), types scalaires et composés, fonctions, « tout est une expression », `if`, `loop`, `while` et `for`

## À la fin, tu vas pouvoir

- Déclarer des variables immuables, mutables et des constantes, et expliquer quand chacune convient.
- Choisir des types numériques, booléens, caractères, n-uplets (tuples), tableaux, vecteurs et chaînes pour des données simples du `revisor`.
- Écrire des fonctions avec des paramètres et des valeurs de retour sans dépendre de conversions implicites.
- Expliquer pourquoi un bloc, un `if` et un `loop` peuvent produire des valeurs.
- Utiliser `if`, `loop`, `while` et `for` pour classer et parcourir des données de façon lisible.
- Lire et corriger deux variantes courantes de l'erreur `E0308`.
- Résoudre les exercices `variables`, `functions`, `if` et `primitive_types` de Rustlings.

## Le pourquoi avant le comment

Le `revisor` que tu vas construire au fil du cours reçoit une liste de services, les interroge et signale ce qui s'est passé. Même s'il aura au final du HTTP, des fichiers YAML, de la concurrence et une sortie JSON, son noyau commence par des opérations bien plus petites : stocker un temps de réponse, le comparer à une limite, parcourir une liste et décider quel texte afficher. Avant de modéliser un service avec une `struct` ou une panne avec un `enum`, tu dois pouvoir exprimer ces opérations avec précision.

Pense à une première règle du programme : une réponse allant jusqu'à mille millisecondes est considérée comme normale ; une réponse plus lente est signalée comme lente. La règle paraît simple, mais elle contient plusieurs décisions que Rust veut que tu déclares : le temps ne peut pas être du texte, il doit être un nombre ; le seuil doit avoir un type compatible ; la comparaison doit produire une condition booléenne ; et la fonction doit toujours fournir une classification. Rust ne laisse pas ces décisions cachées dans des conversions automatiques ou des valeurs ambiguës. Le compilateur exige que le programme indique ce que représente chaque donnée.

Cette insistance peut sembler pesante si tu viens de Go, de Python ou de JavaScript. En Go aussi, les types sont statiques et les conversions entre nombres sont explicites, mais Rust étend cette précision à d'autres parties de la syntaxe. Une variable est immuable par défaut. Un bloc peut renvoyer une valeur. Un `if` doit produire des valeurs du même type dans ses deux branches quand il est utilisé comme expression. Un `for` distingue entre parcourir une collection, l’emprunter ou la consommer. Au début, ce sont des décisions de plus qui deviennent visibles ; ensuite, c'est de l'information qui évite que quelqu'un interprète mal ton intention en maintenant le programme.

Le modèle mental utile n'est pas « Rust met des obstacles avant l'exécution ». C'est « Rust transforme des décisions de conception en choses vérifiables ». Si tu nommes une mesure `u64`, le compilateur sait qu'elle ne doit pas être négative. Si tu rends une variable mutable, le lecteur sait qu'elle va changer. Si une fonction renvoie `&'static str`, il est clair qu'elle renvoie l'une de quelques étiquettes fixes et non un texte fraîchement construit. Si le résultat d'un `if` est stocké dans une variable, toutes ses branches doivent décrire le même type de résultat. L'essentiel de ce qui suit dans le cours s'appuie sur cette même idée.

Cette leçon travaille les chapitres 2 et 3 de *The Rust Programming Language*. Le chapitre 2 présente `let`, les fonctions et l'usage de `mut` dans un petit programme ; le chapitre 3 organise les fondamentaux : variables, types de données, fonctions, commentaires et flux de contrôle. N'essaie pas de mémoriser tous les types disponibles d'une traite. L'important est d'apprendre à lire une signature, à choisir une représentation raisonnable et à laisser le compilateur te signaler les contradictions.

Le vrai programme contient déjà ces fondamentaux. Dans `programas/revisor/src/modelo.rs`, il y a des limites exprimées sous forme de constantes ; dans `src/config.rs`, il y a un `for` qui valide chaque service ; dans `src/revisar.rs`, il y a des conditions qui classent les réponses ; et dans `src/reporte.rs`, il y a des variables mutables pour construire une sortie. Cette leçon ne modifie pas ce projet : elle l'utilise comme carte de la direction que prennent les petites pièces que tu vas pratiquer ici.

## Les concepts

### Variables, `mut`, constantes et masquage (shadowing)

Une variable se déclare avec `let`. Par défaut, la liaison entre le nom et sa valeur est immuable : après avoir écrit `let x = 5;`, tu ne peux pas affecter une autre valeur à `x`. Ce choix est délibéré. Quand tu lis une longue fonction, chaque nom qui ne porte pas `mut` te donne une garantie locale : ce nom continuera à représenter la même valeur pendant le reste de sa portée.

L'immuabilité ne veut pas dire que Rust interdit de changer des données. Elle veut dire que tu dois le déclarer. Si une variable représente un compteur, une sortie que tu construis peu à peu ou un indice qui décroît, utilise `let mut`. Le mot `mut` va à côté du nom parce qu'il décrit la liaison, pas toute la fonction. Évite de mettre `mut` par habitude : une variable mutable qui ne change jamais provoque un avertissement, et compiler les exemples avec `-D warnings` transforme cet avertissement en erreur. C'est un petit signal, mais utile : le code dit que quelque chose va varier alors que ce n'est pas le cas.

Les constantes s'écrivent avec `const`, portent un type explicite et sont évaluées avant l'exécution du programme. Utilise-les pour des règles dont le nom doit apparaître dans tout le code : une limite de temps, une capacité ou un nombre maximal de tentatives. Une constante n'est pas une variable immuable sous un autre nom. Elle n'occupe pas un emplacement mémoire unique que tu pourrais prêter ou modifier ; elle est substituée là où elle est utilisée. À ce stade, il suffit de retenir la règle pratique : `let` pour les valeurs locales et `const` pour une règle stable et nommée.

**Fig. 1.1** | Variables, mutabilité et constantes.

```rust
// fig01_01.rs
fn main() {
    let x: i32 = 5;             // tipo explícito (casi nunca hace falta: lo infiere)
    let mut y = 10;             // mutable
    const MAX: u32 = 100_000;   // constante, siempre con tipo

    y += x;
    println!("x = {x}, y = {y}, MAX = {MAX}");
}
```

```bash
$ rustc --edition 2024 fig01_01.rs && ./fig01_01
x = 5, y = 15, MAX = 100000
```

Le type de `x` est écrit `i32`, mais Rust pourrait l'inférer ici, parce que `y += x` et le littéral `10` donnent assez de contexte. Annoter les types aide quand une signature fait partie d'une API, quand le compilateur ne peut pas les inférer ou quand tu veux communiquer une contrainte importante. Ne les annote pas mécaniquement dans chaque `let` : une inférence bien utilisée réduit le bruit sans perdre en sécurité.

Le masquage (shadowing) est différent de la mutabilité. Avec le masquage, tu déclares une nouvelle variable portant le même nom ; l'ancienne cesse d'être accessible à partir de ce point. C'est utile quand une idée passe par des étapes et que tu veux garder un nom honnête. Par exemple, un texte avec des espaces et le nombre d'espaces sont deux valeurs différentes, mais toutes deux peuvent s'appeler `espacios`, parce que la première version n'est plus nécessaire. Contrairement à `mut`, le masquage permet de changer de type.

**Fig. 1.2** | Masquage, n-uplets et tableaux.

```rust
// fig01_02.rs
fn main() {
    let espacios = "   ";
    let espacios = espacios.len();

    let medicion: (u16, u64, bool) = (200, 750, true);
    let (codigo, ms, saludable) = medicion;
    let nombres = ["catalogo", "pagos"];

    println!("espacios = {espacios}");
    println!("codigo = {codigo}, ms = {ms}, saludable = {saludable}");
    println!("primer servicio = {}", nombres[0]);
}
```

```bash
$ rustc --edition 2024 fig01_02.rs && ./fig01_02
espacios = 3
codigo = 200, ms = 750, saludable = true
primer servicio = catalogo
```

Ici, le premier `espacios` est un `&str`, une vue de texte ; le second est un `usize`, une quantité. Ce n'est pas qu'une variable ait muté de texte en nombre : ce sont deux liaisons distinctes, avec des portées qui se chevauchent. Cette différence compte plus tard avec la possession (ownership). `let mut nombre` conserve la même valeur et permet de la modifier ; `let nombre = ...` relie à nouveau le nom et peut transformer la valeur sans conserver la version précédente.

Dans le `revisor`, une variable mutable apparaît quand on construit le tableau qui sera affiché. Le nom `salida` ne représente pas une règle fixe : c'est un accumulateur auquel on ajoute des lignes, donc `mut` communique exactement l'intention.

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
```

Tu n'as pas encore besoin de comprendre les références, les itérateurs ni `format!` pour reconnaître la décision fondamentale : `filas` et `ancho` ne changent pas ; `salida`, si. À la leçon 2, tu étudieras pourquoi `&[Servicio]` et `&[Estado]` sont des emprunts, et à la leçon 4 tu verras comment `String` permet de construire du texte dynamique. Pour l'instant, identifie le motif : déclare immuable jusqu'à ce qu'une modification fasse réellement partie du travail.

Les règles de classification du `revisor` sont aussi des constantes. La valeur n'est pas répétée comme un nombre anonyme dans chaque comparaison ; elle a un nom, un type et un commentaire. Quand la politique du « lent » changera, il y aura un endroit évident à réviser.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;
```

`TIMEOUT_POR_OMISION_MS` est privée au module parce qu'elle ne sert qu'à créer des services par défaut. `UMBRAL_LENTO_MS` porte `pub` parce qu'un autre module, `revisar.rs`, doit la consulter. La visibilité des modules s'étudie formellement à la leçon 6 ; l'important aujourd'hui est que les deux valeurs ont des types explicites et des noms qui expriment les unités. Un `1000` sans nom laisse des questions : mille secondes, mille octets, mille millisecondes ? `UMBRAL_LENTO_MS` y répond.

### Types scalaires et composés

Rust est un langage à types statiques : avant l'exécution, le compilateur connaît le type de chaque valeur. Parfois il l'infère et parfois tu dois l'annoter, mais il ne traite jamais un nombre comme du texte et ne mélange jamais deux tailles d'entiers parce qu'ils seraient « à peu près compatibles ». Cette rigueur permet de détecter beaucoup d'erreurs avant de produire un binaire.

Les types scalaires stockent une seule valeur. Les entiers signés sont `i8`, `i16`, `i32`, `i64`, `i128` et `isize` ; les non signés sont `u8`, `u16`, `u32`, `u64`, `u128` et `usize`. Le nombre indique combien de bits occupe la valeur. `isize` et `usize` varient selon l'architecture et servent principalement pour les tailles, les longueurs et les indices. Pour les quantités de millisecondes du `revisor`, `u64` est une décision explicite : il n'y a pas de temps négatifs et la plage est large. Pour un code HTTP, `u16` suffit. Choisir un type ne consiste pas à chercher le plus petit nombre possible ; cela consiste à exprimer le domaine de la donnée de façon sensée.

`i32` est le type entier par défaut quand le compilateur ne reçoit pas plus de contexte. C'est un bon choix général pour les calculs entiers locaux. Ne suppose pas que tous les entiers sont des `i32` : un `usize` qui vient de `len()` ne peut pas être additionné directement à un `u64`, et un `u16` de code HTTP ne devient pas `i32` parce qu'il se trouve dans la même opération. L'avantage est que tu vois le croisement de domaines à l'endroit exact où il a lieu.

Rust ne fait pas de conversions numériques implicites. Ce n'est pas une bizarrerie isolée : cela évite qu'une affectation en apparence innocente change de taille, de signe ou de plage sans que celui qui a écrit le code y ait pensé. En Go aussi, les conversions entre types numériques se demandent explicitement ; Rust conserve cette discipline et la rend particulièrement importante parce que ses types entiers servent souvent à représenter des capacités, des longueurs et des données réseau.

**Fig. 1.3** | Pas de conversion implicite, même entre nombres.

```rust
// fig01_03.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a;             // ← no compila
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_03.rs
error[E0308]: mismatched types
 --> fig01_03.rs:4:18
  |
4 |     let b: i64 = a;             // ← no compila
  |            ---   ^ expected `i64`, found `i32`
  |            |
  |            expected due to this
  |
help: you can convert an `i32` to an `i64`
  |
4 |     let b: i64 = a.into();             // ← no compila
  |                   +++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Pour une conversion simple et connue, tu peux utiliser `as`. La conversion de `i32` vers `i64` est sûre dans ce cas, parce que tout `i32` tient dans un `i64`. Cependant, `as` permet aussi des conversions qui peuvent tronquer, réinterpréter le signe ou perdre de la précision. Ne l'utilise pas comme moyen de « faire taire le compilateur ». Quand une conversion peut échouer ou perdre de l'information, tu découvriras plus tard `TryFrom`, `TryInto` et `Result`.

**Fig. 1.4** | Une conversion explicite s’écrit avec `as`.

```rust
// fig01_04.rs
fn main() {
    let a: i32 = 5;
    let b: i64 = a as i64;      // así
    println!("{b}");
}
```

```bash
$ rustc --edition 2024 fig01_04.rs && ./fig01_04
5
```

Outre les entiers, les scalaires comprennent `f32` et `f64` pour les nombres à virgule flottante, `bool` pour `true` ou `false`, et `char` pour un caractère Unicode. Pour le `revisor`, évite d'utiliser la virgule flottante si un entier exprime mieux l'unité. Stocker `750` millisecondes en `u64` est plus clair que stocker `0.75` seconde en `f64`, et cela évite les questions d'arrondi quand tu affiches, compares ou sérialises la valeur.

Les types composés regroupent plusieurs valeurs. Un n-uplet (tuple) peut contenir des éléments de types différents et a une taille fixe. Dans la figure 1.2, `(u16, u64, bool)` représente trois résultats qui appartiennent à une même mesure : code, durée et état de santé. La déstructuration `let (codigo, ms, saludable) = medicion;` extrait ces valeurs avec des noms utiles. Les n-uplets conviennent pour de petits résultats locaux ; quand le sens des champs est central dans le programme, comme le sera un service, une structure avec des champs nommés sera meilleure. Cela vient à la leçon 3.

Un tableau comme `["catalogo", "pagos"]` contient des valeurs du même type et a une longueur fixe connue à la compilation. Un vecteur, `Vec<T>`, contient aussi des valeurs du même type, mais il peut grandir ou rétrécir à l'exécution. La figure 1.7 utilise `vec!` parce que la liste de services est une collection qui peut, conceptuellement, changer de taille. Dans le projet réel, la configuration est chargée depuis du YAML et produit un `Vec<Servicio>` pour la même raison.

Les chaînes demandent aussi de la précision. Un littéral comme `"catalogo"` est généralement un `&str`, une vue empruntée d'un texte déjà existant. Un `String` est un texte qui possède sa mémoire et peut grandir. Dans cette leçon, tu verras `&str` comme valeur de sortie d'étiquettes fixes ; à la leçon 2, tu étudieras pourquoi tous les textes ne peuvent pas être copiés et pourquoi on distingue les deux types. Pour le moment, retiens cette règle : un texte fixe écrit dans le code est généralement un `&str` ; un texte lu, construit ou stocké finit généralement en `String`.

Le `revisor` rend explicite son vocabulaire numérique. Le temps limite est stocké en `u64` et le code HTTP en `Option<u16>`. Tu n'as pas encore besoin de maîtriser `Option` ; la leçon 3 expliquera pourquoi il remplace `nil`. Aujourd'hui, il suffit d'observer que le type décrit une contrainte de la réalité : il peut ne pas y avoir de code HTTP si aucune réponse n'est arrivée.

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

Le type n'est pas de la documentation décorative. `ms: u64` interdit de lui affecter une chaîne ; `codigo: Option<u16>` interdit de traiter l'absence de réponse comme si c'était automatiquement un `200` ; `servicio: String` indique que le nom est un texte que le programme possède. Rust utilisera cette information pendant toute la compilation.

### Fonctions, paramètres et valeurs de retour

Une fonction se déclare avec `fn`, a un nom, des paramètres entre parenthèses et un corps entre accolades. Les paramètres portent toujours un type : `fn doble(x: i32)` dit à la fois le nom de la donnée et ce que la fonction peut recevoir. Si elle renvoie autre chose que `()`, on l'indique après une flèche : `-> i32`. Cette signature est un contrat court et vérifiable. Celui qui appelle la fonction sait ce qu'il doit fournir et ce qu'il obtiendra ; le compilateur vérifie les deux extrémités.

À la différence de Go, Rust écrit le type après le nom du paramètre, pas avant. En Go, tu écrirais `func doble(x int) int` ; en Rust, `fn doble(x: i32) -> i32`. La différence visuelle cesse d'importer après quelques fonctions. L'important est que, dans les deux langages, la signature fait partie de la conception : ce n'est ni un commentaire ni une convention informelle.

Une petite fonction ne doit pas assumer un travail qui ne lui revient pas. `doble` reçoit un nombre et en renvoie un autre ; elle n'affiche rien, ne lit aucun fichier et ne modifie aucun état externe. Cette séparation paraît élémentaire, mais elle prépare le terrain pour le `revisor` : une fonction qui classe des millisecondes peut être testée avec trois nombres sans démarrer un client HTTP ni ouvrir une configuration. Quand le programme grandira, diviser la logique en fonctions aux entrées et sorties claires sera une façon de le garder compréhensible.

**Fig. 1.5** | Tout est une expression.

```rust
// fig01_05.rs
fn main() {
    let x = 7;
    let n = if x > 5 { "grande" } else { "chico" };      // el if DEVUELVE valor

    let cuadrado = {
        let t = x * x;
        t                          // 🔑 sin punto y coma = es el valor del bloque
    };

    println!("{n} {cuadrado} {}", doble(x));
}

fn doble(x: i32) -> i32 {
    x * 2                      // sin `return` y sin `;`
}
```

```bash
$ rustc --edition 2024 fig01_05.rs && ./fig01_05
grande 49 14
```

`main` est aussi une fonction. Dans un programme exécutable, elle commence sans paramètres et n'a pas besoin de déclarer un type de retour si elle se contente de se terminer. En revanche, `doble` promet un `i32`, donc la dernière valeur de son corps doit être compatible avec `i32`. Tu peux utiliser `return x * 2;`, mais ce n'est pas la forme habituelle pour la dernière valeur d'une fonction. Rust privilégie l'expression finale parce qu'elle laisse visible le résultat que produit le corps.

Les paramètres sont passés de différentes manières selon leur type et l'intention. Les types scalaires comme `i32`, `u64` et `bool` se copient à peu de frais ; recevoir `ms: u64` n'empêche pas l'appelant de continuer à utiliser sa mesure. Avec `String`, les vecteurs et les structures plus complexes apparaîtront les règles de déplacement et d'emprunt de la leçon 2. Ne les anticipe pas en résolvant tout avec des copies. Pour l'instant, utilise des paramètres scalaires pour t'entraîner à écrire des signatures propres, et reconnais qu'un `&str` d'étiquette fixe a une vie différente d'un `String` qu'on construit.

Le projet réel a une petite fonction qui convertit une configuration textuelle en données du programme. Même si elle utilise des bibliothèques que tu étudieras plus tard, sa signature montre le motif essentiel : elle reçoit une entrée, renvoie un résultat et son corps se termine par une expression `Ok(...)`.

<!-- verificar:extracto:src/config.rs -->
```rust
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}
```

Tu n'as pas encore besoin de décortiquer `Result`, `?` ni `yaml_serde` ; ils arriveront à la leçon 4. Ce que tu peux déjà lire, c'est la forme : `ruta` entre comme une vue de texte, la fonction promet de renvoyer une liste de services ou une erreur, `txt` est une valeur locale immuable et `Ok(...)` est le résultat final. Les signatures te permettent de comprendre la frontière d'une fonction avant même de connaître tous ses détails internes.

### Expressions, instructions et le point-virgule

En Rust, beaucoup de constructions produisent une valeur. Une opération arithmétique comme `x * 2` produit un nombre ; un bloc entre accolades peut produire la dernière valeur qu'il contient ; un `if` peut produire l'une de deux valeurs ; et un `loop` peut se terminer avec une valeur envoyée par `break`. Ces constructions s'appellent des expressions.

Une instruction réalise une action mais ne produit pas de valeur utile. Une déclaration `let x = 7;` est une instruction. Une expression suivie d’un point-virgule est également une instruction. La valeur d'une instruction est `()`, appelé le type unité. Tu peux penser à `()` comme « il n'y a aucun résultat à fournir ». Ce n'est ni une erreur ni une valeur nulle : c'est un vrai type qui apparaît quand une opération est utilisée seulement pour son effet.

Le point-virgule détermine cette différence à des endroits importants. Dans la figure 1.5, le bloc affecté à `cuadrado` se termine par `t` sans point-virgule, donc le bloc produit la valeur de `t`. La fonction `doble` se termine par `x * 2` sans point-virgule, donc elle renvoie ce `i32`. Si tu ajoutes `;`, l'opération s'exécute et son résultat est jeté. Le corps de la fonction produit alors `()`, mais la signature exige `i32`.

**Fig. 1.6** | Le point-virgule de trop.

```rust
// fig01_06.rs
fn doble(x: i32) -> i32 {
    x * 2;                     // ← el punto y coma de más
}

fn main() {
    println!("{}", doble(4));
}
```

```bash
$ rustc --edition 2024 fig01_06.rs
error[E0308]: mismatched types
 --> fig01_06.rs:2:21
  |
2 | fn doble(x: i32) -> i32 {
  |    -----            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
3 |     x * 2;                     // ← el punto y coma de más
  |          - help: remove this semicolon to return this value

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Cette erreur est déroutante la première fois et très utile ensuite. Le compilateur ne dit pas que la multiplication est invalide ; il dit que la fonction a promis `i32` et a fini par produire `()`. Lis les deux parties du message : `expected i32, found ()` identifie la contradiction, et l'aide propose de retirer le point-virgule. N'ajoute pas un `return` sans comprendre pourquoi ; dans ce cas, le problème est que tu as jeté la bonne valeur.

Le style par expressions rend de petites transformations compactes et claires. Tu peux calculer une valeur intermédiaire dans un bloc, garder les variables locales à l'intérieur de ce bloc et ne fournir que le résultat. Cela réduit les portées inutiles et évite des noms temporaires qui restent vivants alors qu'ils ne signifient plus rien. Ne transforme pas chaque ligne en une expression compliquée : la lisibilité reste le critère. Un bloc de deux ou trois étapes bien nommées est généralement plus clair qu'une ligne ingénieuse.

Dans `revisar.rs`, la classification d'une réponse utilise des conditions dans un `match` ; le résultat de chaque branche est un `Estado`. Même si `match` s'étudie à fond à la leçon 3, le motif est déjà familier : chaque chemin produit la valeur que la fonction a promise.

<!-- verificar:extracto:src/revisar.rs -->
```rust
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
```

La partie qui correspond à cette leçon, ce sont les conditions `if` et l'idée qu'une construction de contrôle se termine par une valeur. La partie nouvelle, ce sont `match`, `Result` et les variantes de `Estado`. Tu n'as pas besoin de les recopier maintenant ; reconnais seulement que la syntaxe que tu pratiques avec des nombres finira par prendre de vraies décisions sur des services.

### `if`, `loop`, `while` et `for`

`if` évalue une condition qui doit être un `bool`. Rust ne considère pas que `0`, une chaîne vide ou une référence nulle soient faux automatiquement. Écris une comparaison ou utilise une variable booléenne. Cette décision évite les conditions accidentelles et rend évident quelle propriété tu demandes : `ms > UMBRAL_LENTO_MS` communique une règle ; `if ms` n'aurait aucun sens.

Quand `if` sert à choisir une valeur, ses branches doivent renvoyer le même type. Tu ne peux pas renvoyer `"rápido"` dans une branche et `1000` dans une autre, parce que la variable qui reçoit le résultat doit avoir une représentation cohérente. Cette contrainte est exactement le genre de décision qui devient utile dans un rapport : une classification sera toujours du texte, un code de sortie sera toujours un entier adéquat et un état sera toujours une variante du même type.

`loop` démarre une boucle infinie. Cela ressemble à un outil extrême, mais il convient quand tu ne connais pas à l'avance le nombre d'itérations et que la façon naturelle d'en sortir est `break`. Contrairement à d'autres langages, `break valor` peut donner le résultat d'un `loop`. Cela sert quand la boucle cherche ou calcule quelque chose ; la valeur trouvée devient directement le résultat de l'expression.

`while condicion` répète tant que la condition est vraie. Utilise-le quand l'avancement dépend d'un état que tu contrôles : décrémenter un compteur, lire jusqu'à une condition ou réessayer selon une règle explicite. Assure-toi que le corps peut changer l'état qui rend la condition fausse. Un `while` dont le compteur n'est jamais mis à jour est une boucle infinie déguisée.

`for` est l'option normale pour parcourir une collection ou un intervalle. Rust n'a pas le style traditionnel `for initialisation; condition; mise à jour` de C, Java ou Go. À la place, il parcourt quelque chose qui sait fournir ses éléments un par un (en Rust, quelque chose qui implémente `IntoIterator`) : un intervalle comme `0..10`, une liste, un tableau ou un itérateur. Cette forme élimine une grande partie du code d'indices et réduit les erreurs de bornes.

**Fig. 1.7** | Les boucles.

```rust
// fig01_07.rs
fn main() {
    let mut x = 3;
    let servicios = vec!["catalogo", "pagos", "reportes"];

    loop { break; }                          // infinito, con break
    while x > 0 { x -= 1; }
    for i in 0..10 { print!("{i} "); }       // rango: 0 a 9
    println!();
    for i in 0..=10 { print!("{i} "); }      // inclusivo: 0 a 10
    println!();
    for s in &servicios { println!("{s}"); } // sobre una referencia, para no consumir la lista

    let r = loop { break 42; };              // 🔑 loop devuelve valor con break
    println!("x = {x}, r = {r}");
}
```

```bash
$ rustc --edition 2024 fig01_07.rs && ./fig01_07
0 1 2 3 4 5 6 7 8 9 
0 1 2 3 4 5 6 7 8 9 10 
catalogo
pagos
reportes
x = 0, r = 42
```

Les intervalles sont une source courante d'erreurs de bornes. `0..10` inclut `0` et exclut `10`, donc il a dix valeurs : de zéro à neuf. `0..=10` inclut les deux extrémités et a onze valeurs. Pour parcourir les positions d'un tableau de longueur dix, tu veux presque toujours `0..10` ou, mieux encore, parcourir directement les éléments. N'utilise l'intervalle inclusif que lorsque la borne finale fait partie de la règle et doit apparaître.

La ligne `for s in &servicios` porte une référence vers la liste. Cela permet de lire chaque élément sans céder la possession de `servicios`. La différence complète entre `servicios`, `&servicios` et `&mut servicios` est le sujet de la leçon 2, mais tu peux adopter dès aujourd'hui une règle provisoire : si tu veux seulement regarder une collection et la conserver, parcours-la par référence. Le compilateur évitera les usages dangereux quand tu connaîtras les règles d'emprunt.

Le `revisor` valide une liste avec un `for`. La fonction n'a pas besoin de savoir combien de services sont arrivés : elle les prend un par un. `enumerate()` ajoute l'indice pour pouvoir comparer le service courant avec les précédents. Même si l'expression complète paraît avancée, son flux est le même que celui de la figure 1.7 : parcourir, vérifier une condition et se terminer par un résultat.

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

Ici, `s.timeout_ms > 0` est une condition booléenne comme celles que tu as déjà utilisées. La différence est qu'au lieu d'afficher une étiquette, `anyhow::ensure!` arrête la validation avec une erreur si la condition est fausse. La leçon 4 expliquera `Result` et ce type de gestion d'erreurs. La leçon 2 expliquera les références de `&[Servicio]`. Tu peux déjà lire l'intention sans connaître chaque détail : tous les services doivent avoir une URL avec un schéma, un nom non répété et une limite supérieure à zéro.

## L'erreur que tu vas voir

### `E0308` : types qui ne correspondent pas

`E0308` signifie que Rust attendait un type à un endroit du programme et en a trouvé un autre. Ce n'est pas un message vague : lis-le comme une phrase en deux parties. Identifie d'abord l'endroit où l'attente a été fixée ; identifie ensuite la valeur qui contredit cette attente. Dans la figure 1.3, l'annotation `let b: i64` fixe que `b` sera `i64` ; la variable `a` est `i32` ; c'est pourquoi l'affectation échoue.

La correction ne sera pas toujours `as`. Pour convertir de `i32` en `i64`, l'élargissement est sûr et `as i64` communique l'intention. Pour convertir d'un grand nombre vers un petit, ou d'un texte vers un entier, tu dois décider quoi faire quand la valeur ne tient pas ou n'a pas de format valide. Ces conversions seront traitées avec des résultats qui peuvent échouer. La bonne pratique est de résoudre l'écart à la frontière entre domaines, et non de convertir des valeurs à répétition dans chaque fonction.

La figure 1.6 produit le même code `E0308`, mais pour une cause différente : la fonction déclare `-> i32` et son dernier élément est une instruction dont la valeur est `()`. Cette différence illustre pourquoi tu ne dois pas résoudre les erreurs d'après le seul numéro. Le code regroupe une famille de diagnostics ; les lignes signalées et les mots `expected` et `found` racontent l'histoire concrète.

Quand tu vois `expected i32, found ()`, pose-toi ces questions : la fonction a-t-elle promis un retour ? la dernière valeur a-t-elle un point-virgule ? une branche de `if` ne renvoie-t-elle pas la même chose que l'autre ? ai-je mis `println!` comme dernier élément alors qu'il fallait produire une valeur ? Dans cet ordre, tu trouves normalement le problème sans chercher des réponses au hasard.

### Lire une suggestion sans lui obéir aveuglément

Rust propose souvent une section `help:`. C'est une proposition contextuelle, pas un ordre. Dans la figure 1.3, elle suggère `a.into()`, qui peut aussi convertir la valeur parce qu'il existe une conversion connue entre les deux types. La figure 1.4 conserve `as i64` parce que c'est la forme qu'on veut enseigner pour une conversion numérique explicite et simple. Dans d'autres cas, la suggestion peut être `clone()`, ajouter une référence ou changer une signature. Avant de l'accepter, demande-toi quel coût, quelle propriété ou quel comportement elle introduit.

Le message se termine aussi par `rustc --explain E0308`. Cette commande ouvre une explication générale du code d'erreur installé avec ton compilateur. Utilise-la quand le diagnostic local ne suffit pas, mais commence par le fichier, la ligne et les colonnes que le compilateur t'a déjà montrés. Ils contiennent presque toujours plus d'informations spécifiques à ton programme qu'une recherche générale.

## Ce qui se fait mal

### Tout déclarer `mut`

Déclarer chaque variable avec `mut` pour « avoir de la liberté » efface de l'information. Si un nom ne change pas, le lecteur ne doit pas avoir à suivre toute la fonction pour le découvrir. De plus, le compilateur avertit quand `mut` n'est pas nécessaire. Ne déclare mutable que ce que l'algorithme modifie, comme `x` dans un compte à rebours ou `salida` quand on construit un rapport.

### Utiliser `as` pour éteindre les erreurs de types

Une conversion avec `as` peut être correcte, mais ce n'est pas un remède universel. Convertir un grand `u64` en `u16` peut perdre des données ; convertir un entier signé en non signé peut produire une valeur surprenante. Définis ce que représente chaque nombre et convertis une seule fois, à la frontière où tu changes de domaine. Si la conversion peut échouer, le programme doit l'exprimer au lieu de le cacher.

### Utiliser des nombres sans unités ni nom

Un `if ms > 1000` fonctionne, mais il oblige à se rappeler ce que représente `1000`. Des millisecondes, des secondes ou des octets ? Utilise une constante comme `UMBRAL_LENTO_MS` quand la valeur est une règle métier. Pour des valeurs locales évidentes, un littéral peut convenir ; pour une politique qui se répétera ou changera, un nom évite des erreurs et améliore la lecture.

### Ajouter un point-virgule à la dernière expression par réflexe

Dans beaucoup de langages, chaque ligne se termine par un point-virgule ou la convention invite à l'utiliser. En Rust, le dernier point-virgule d'une fonction ou d'un bloc change sa valeur en `()`. Ne mémorise pas une exception ; reconnais la règle : une expression finale sans point-virgule peut être le résultat. Si un bloc existe pour calculer quelque chose, vérifie ce qu'il laisse comme dernière expression.

### Écrire `for i in 0..lista.len()` quand tu n'as besoin que des éléments

Parcourir les indices fonctionne, mais ajoute une façon inutile de se tromper. Si tu n'as besoin que de chaque service, écris `for servicio in &servicios`. Utilise `enumerate()` quand l'indice fait réellement partie de la logique, comme dans la validation du `revisor`. Utilise des indices directs quand tu dois accéder à des positions précises et que tu peux justifier les bornes.

### Utiliser `loop` quand le nombre de pas est déjà connu

Un `loop` avec plusieurs conditions de sortie peut être correct, mais si tu as une collection ou un intervalle connu, `for` exprime mieux l'intention. Si cela dépend d'une condition changeante, `while` montre généralement plus clairement le critère de fin. Réserve `loop` aux processus qui attendent réellement une sortie par `break`, comme un lecteur d'événements ou une recherche qui s'arrête en trouvant la donnée.

## Exercices

### Exercice 1 — Classe une réponse

Écris `fn clasificar(ms: u64) -> &'static str`. Elle doit renvoyer `"rápido"` si le temps est inférieur ou égal à `1000`, `"lento"` s'il est supérieur à `1000` et inférieur ou égal à `5000`, et `"timeout"` s'il est supérieur. Utilise un `if` comme expression : n'utilise pas `return`. Depuis `main`, affiche la classification de `700`, `1500` et `6000`, une par ligne.

Avant de voir la solution, vérifie que les trois branches renvoient le même type. La signature n'a pas besoin de créer un `String` : les trois étiquettes sont des littéraux fixes et peuvent donc être des `&'static str`.

### Exercice 2 — Additionne sans consommer la liste

Écris `fn sumar(valores: &[i32]) -> i32` qui utilise `for` pour additionner une tranche (slice). Depuis `main`, crée `let valores = vec![3, 5, 8];`, affiche le résultat puis affiche la longueur de `valores`. Le second affichage doit compiler : il démontre que le parcours n'a pas consommé le vecteur.

Fais en sorte que seul l'accumulateur soit mutable. Ne rends pas le vecteur mutable : tu n'ajoutes, ne retires ni ne modifies ses éléments.

### Exercice 3 — Un rapport minimal du revisor

Déclare `const UMBRAL_LENTO_MS: u64 = 1000;` et écris `fn etiqueta(ms: u64) -> &'static str` qui renvoie `"OK"` jusqu'au seuil et `"LENTO"` au-dessus. Dans `main`, utilise un tableau avec `[120_u64, 1000, 1500]` et un `for` pour afficher exactement ces lignes :

```text
120ms: OK
1000ms: OK
1500ms: LENTO
```

Ensuite, change le type du tableau en `i32` sans changer la signature de `etiqueta`. Lis `E0308`, corrige-le de façon explicite et explique avec tes propres mots pourquoi Rust n'a pas fait la conversion à ta place.

## Solutions

### Solution 1

<!-- verificar:fragmento -->
```rust
fn clasificar(ms: u64) -> &'static str {
    if ms <= 1000 {
        "rápido"
    } else if ms <= 5000 {
        "lento"
    } else {
        "timeout"
    }
}
```

Le `if` complet est l'expression finale de la fonction. Chaque branche renvoie un littéral de type `&'static str`, donc la signature et le résultat coïncident. L'ordre compte : la deuxième condition n'est évaluée que si la première était fausse, donc il n'est pas nécessaire de répéter `ms > 1000`.

### Solution 2

<!-- verificar:fragmento -->
```rust
fn sumar(valores: &[i32]) -> i32 {
    let mut total = 0;

    for valor in valores {
        total += valor;
    }

    total
}
```

`valores` reçoit une référence vers une tranche (slice), donc la fonction observe les nombres sans s'approprier le vecteur de l'appelant. Dans le `for`, `valor` est une référence vers chaque `i32` ; l'addition fonctionne parce que `i32` implémente l'addition avec une référence vers un autre `i32` (`total += valor`), sans que tu aies à écrire `*valor`. La dernière expression, `total`, fournit le résultat sans point-virgule.

### Solution 3

<!-- verificar:fragmento -->
```rust
const UMBRAL_LENTO_MS: u64 = 1000;

fn etiqueta(ms: u64) -> &'static str {
    if ms <= UMBRAL_LENTO_MS {
        "OK"
    } else {
        "LENTO"
    }
}

fn main() {
    let mediciones = [120_u64, 1000, 1500];

    for ms in mediciones {
        println!("{ms}ms: {}", etiqueta(ms));
    }
}
```

Le suffixe `_u64` du premier littéral fixe le type du tableau. Les autres éléments doivent être du même type, donc Rust les interprète eux aussi comme `u64`. La constante exprime à la fois la valeur et l'unité de la règle. Si tu changeais le tableau en `i32`, tu devrais convertir chaque donnée explicitement ou changer le contrat de la fonction ; ces deux décisions ont un sens et ne doivent pas arriver par accident.

## Comment savoir que j'y suis arrivé

- Depuis `programas/01-fundamentos`, `rustc --edition 2024 -D warnings fig01_01.rs && ./fig01_01` affiche `x = 5, y = 15, MAX = 100000` sans avertissements.
- `rustc --edition 2024 fig01_03.rs` échoue avec `error[E0308]`, et tu peux montrer que `b` attend `i64` alors que `a` est `i32`.
- `rustc --edition 2024 fig01_06.rs` échoue avec `error[E0308]`, et tu peux expliquer que le point-virgule a fait que la fonction a renvoyé `()`.
- `rustc --edition 2024 -D warnings fig01_07.rs && ./fig01_07` affiche les deux intervalles, les trois services et se termine par `x = 0, r = 42`.
- `rustc --edition 2024 -D warnings fig01_02.rs && ./fig01_02` affiche les trois résultats documentés et ne signale aucun avertissement.
- Tu as terminé les sections `variables`, `functions`, `if` et `primitive_types` de Rustlings, et tu peux résoudre les trois exercices sans copier les solutions.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 2 : Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) — consulté le 2 octobre 2026.
- [The Rust Programming Language, chapitre 3 : Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) — consulté le 2 octobre 2026.
- [Documentation officielle de `i32` et des types numériques primitifs](https://doc.rust-lang.org/std/primitive.i32.html) — consulté le 2 octobre 2026.
- [Rustlings](https://rustlings.rust-lang.org/) — complète `variables`, `functions`, `if` et `primitive_types` ; consulté le 2 octobre 2026.
