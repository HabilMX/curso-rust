# Leçon 2 — Possession (ownership)

**Durée :** 2 × 45 min.

**Ce que tu construis :** les programmes qui se heurtent exprès au compilateur

**Ce que tu apprends :** les trois règles de la possession, déplacer contre copier, emprunts `&` et `&mut`, les deux règles des emprunts, pourquoi il n'y a pas de ramasse-miettes

## À la fin, tu vas pouvoir

- Expliquer les trois règles de la possession (ownership) et les utiliser pour anticiper quand Rust libère une valeur.
- Distinguer une copie implicite d'un déplacement, et justifier quand utiliser `clone()`.
- Choisir si une fonction doit recevoir une valeur, une référence `&T` ou une référence mutable `&mut T`.
- Appliquer la règle « beaucoup de lectures ou une seule écriture » et corriger les erreurs `E0382` et `E0502`.
- Expliquer pourquoi Rust n'a besoin ni d'un ramasse-miettes ni d'appels manuels à `free`.
- Écrire une fonction qui reçoit un `&str` et renvoie une portion empruntée du texte.
- Résoudre les exercices `move_semantics` et `primitive_types` de Rustlings.

## Le pourquoi avant le comment

La leçon 2 est le moment où Rust cesse de ressembler simplement à un langage compilé avec une syntaxe différente de celle de Go. Les variables, les fonctions, les types et le flux de contrôle de la leçon précédente sont reconnaissables. La possession change une question que beaucoup de langages cachent : quand un programme crée une donnée en mémoire, qui doit la détruire, et quand ?

Le `revisor` stockera des noms de services, des URL, des messages d'échec, des listes et des rapports. Toutes ces données peuvent grandir à l'exécution. Le nom d'un service lu depuis du YAML n'a pas de taille connue au moment où tu compiles ; il a besoin de mémoire dynamique. Le tableau final du rapport se construit lui aussi peu à peu. En C ou en C++, celui qui écrit le programme devrait réserver et libérer cette mémoire à la main. S'il libère deux fois, le programme peut se corrompre. S'il ne la libère pas, il perd de la mémoire. S'il conserve un pointeur après l'avoir libérée, il peut lire une zone qui appartient déjà à autre chose.

Go prend une autre décision. Le programme peut créer des données et oublier de libérer la mémoire, parce que le ramasse-miettes observe quels objets sont encore accessibles et récupère les autres. Cela rend l'écriture des programmes plus directe, mais ajoute à l'exécution un composant qui gère la mémoire, décide quand travailler et consomme des ressources pour trouver les objets devenus inutiles. Dans la plupart des programmes Go, cette décision est excellente : elle réduit la complexité et évite des erreurs graves.

Rust cherche une autre combinaison : une mémoire sûre sans ramasse-miettes et sans libération manuelle. Sa proposition est de vérifier, avant de générer l'exécutable, qui possède chaque valeur et qui peut y accéder. Si le compilateur peut démontrer qu'une valeur ne sera plus utilisée, il insère la libération adéquate à la sortie de sa portée. S'il ne peut pas démontrer qu'une référence restera valide, il rejette le programme. S'il détecte deux accès incompatibles à une même donnée, il le rejette aussi.

Cela ne veut pas dire que Rust « devine » ce que tu voulais faire. Au contraire : il exige que ton intention soit visible dans les signatures et dans les affectations. Une fonction qui reçoit un `String` prend la valeur ; une qui reçoit un `&str` la consulte seulement ; une qui reçoit un `&mut String` peut la modifier pendant un emprunt exclusif. L'information qui, en Go, reste parfois dans une convention, un commentaire ou une revue de code, fait partie du type en Rust.

Le prix est réel. Au début, tu vas écrire du code qui semble raisonnable, mais qui ne compile pas. La réaction normale est d'essayer d'ajouter `.clone()` jusqu'à ce que l'erreur disparaisse. Parfois une copie est la bonne décision ; souvent, c'est le signe que la fonction a demandé plus de possession que nécessaire. Apprendre la possession consiste à cesser de traiter ces erreurs comme des obstacles et à commencer à les lire comme des questions de conception : qui doit conserver cette valeur ? combien de temps doit-elle vivre ? qui peut la modifier ?

Le chapitre 4 de *The Rust Programming Language* explique la possession, les références, les emprunts et les slices. Lis-le en entier pendant cette leçon. N'essaie pas de mémoriser tous les messages du compilateur. L'objectif est de construire un modèle mental simple : chaque valeur a un propriétaire ; déplacer transfère cette responsabilité ; emprunter permet d'utiliser une valeur sans transférer la responsabilité ; et les règles d'emprunt évitent qu'une lecture voie une donnée pendant que quelqu'un la modifie.

Le projet réel utilise déjà cette idée, même si tu n'as pas encore écrit toutes ses pièces. `Servicio` possède ses champs `String` parce que le revisor doit conserver un nom et une URL au-delà de la fonction qui les a lus. En revanche, les fonctions qui impriment un rapport reçoivent des références vers les services et leurs états : elles n'ont besoin que de les consulter, pas de se les approprier. Plus loin, aux leçons 3, 4 et 5, ces mêmes décisions apparaîtront dans les structs, les collections, les erreurs et les durées de vie (lifetimes).

## Les concepts

### Possession, portée et libération déterministe

La possession se résume en trois règles.

1. Chaque valeur en Rust a un propriétaire.
2. Il ne peut y avoir qu'un seul propriétaire d'une valeur à la fois.
3. Quand le propriétaire sort de la portée, Rust libère la valeur.

Une portée est la partie du programme où un nom existe. Les accolades délimitent les portées, comme à la leçon 1. La différence maintenant est que sortir d'une portée ne rend pas seulement une variable inaccessible : cela détermine aussi quand la valeur associée est détruite. Pour les types qui réservent des ressources, Rust appelle `drop` automatiquement. `String`, par exemple, libère le bloc de mémoire où il stocke ses caractères.

**Fig. 2.1** | La portée d'une variable.

```rust
// fig02_01.rs
fn main() {
    {
        let s = String::from("hola");     // s es el dueño
        println!("{s}");
    }                                     // aquí termina el ámbito: se libera. Sin free(), sin GC
}
```

```bash
$ rustc --edition 2024 fig02_01.rs && ./fig02_01
hola
```

`String::from("hola")` crée un `String` qui possède de la mémoire dynamique. Tant que `s` est dans la portée interne, il peut servir à afficher le texte. En arrivant à l'accolade fermante, `s` cesse d'exister et Rust libère sa mémoire. Tu n'as pas écrit `free`, tu n'as pas calculé de tailles et tu n'as pas attendu qu'un ramasse-miettes décide de passer. Le compilateur insère le travail nécessaire parce qu'il connaît l'étendue de `s`.

Le mot « possession » ne décrit pas l'emplacement physique d'une donnée ; il décrit une responsabilité. La valeur peut être sur la pile (stack), sur le tas (heap) ou contenir des références vers d'autres valeurs. L'important est que Rust peut identifier un propriétaire responsable de nettoyer la ressource. Beaucoup de types simples, comme `u64`, `bool` ou `char`, tiennent entièrement sur la pile et n'ont rien de spécial à libérer. Un `String`, un `Vec<T>` ou un `HashMap<K, V>` gèrent de la mémoire dynamique et ont bien besoin d'une fin ordonnée.

Cette libération est dite déterministe parce qu'elle a lieu en un point que tu peux raisonner en lisant le programme : à la fin de la portée, sauf si la valeur a été déplacée avant. Il est important de la distinguer de la gestion manuelle. Tu ne choisis pas quand appeler `drop` pour chaque valeur et tu n'as pas à le faire dans des conditions normales. Rust connaît le type et génère la bonne libération. Si un type contient d'autres valeurs, son destructeur libère aussi ce qui correspond à l'intérieur.

Cette garantie ne signifie pas que Rust interdit toute fuite de mémoire imaginable. Par exemple, il est possible de conserver des données avec des cycles de références comptées ou d'utiliser délibérément des mécanismes qui évitent la libération. La garantie centrale est autre : le code sûr ne peut pas utiliser ensuite une valeur que Rust a déjà libérée, ni libérer deux fois la même mémoire. Pour un programme comme le revisor, cela élimine toute une classe d'erreurs sans ajouter de ramasse-miettes à l'exécution.

En Go, une variable locale cesse aussi d'être utile quand elle sort de son bloc, mais la mémoire devenue inaccessible est récupérée plus tard, quand le ramasse-miettes en décide ainsi. En Rust, la fin de la portée fait directement partie du modèle de ressources. Cette différence ne rend pas automatiquement l'un des langages meilleur que l'autre. Go simplifie beaucoup d'applications ; Rust permet de savoir avec plus de précision quand sont libérés la mémoire, les fichiers, les sockets ou les verrous.

Le revisor possède le texte qu'il doit conserver. Un service ne peut pas dépendre du fait qu'une variable temporaire du parseur YAML reste vivante : c'est pourquoi ses champs sont des `String`, et non des références vers un texte temporaire.

<!-- verificar:extracto:src/modelo.rs -->
```rust
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}
```

`nombre` et `url` sont la propriété de chaque `Servicio`. Quand le vecteur de services sera détruit, ses éléments seront détruits ; quand chaque élément sera détruit, ses `String` le seront ; et chaque `String` libérera sa mémoire. Il n'y a pas de liste manuelle de ressources à nettoyer. La structure des valeurs décrit aussi la structure des responsabilités.

### Déplacer, copier et cloner

La deuxième règle dit qu'une valeur n'a qu'un seul propriétaire à la fois. C'est pourquoi une affectation ne signifie pas toujours copier. Avec les types qui possèdent des ressources, Rust déplace généralement la valeur (move) : la nouvelle variable devient le propriétaire et l'ancien nom ne peut plus être utilisé.

Cela surprend si tu viens de Go. En Go, affecter un `string` à une autre variable copie son en-tête immuable et les deux variables peuvent être lues. Affecter un struct copie ses champs ; s'il contient un slice ou une map, les deux copies peuvent continuer à pointer vers des données partagées. En Rust, le compilateur exige que cette relation soit explicite, parce qu'une copie superficielle d'un type propriétaire peut laisser deux valeurs essayer de libérer la même ressource.

**Fig. 2.2** | Les deux issues : copier ou emprunter.

```rust
// fig02_02.rs
fn main() {
    let a = String::from("hola");
    let b = a.clone();          // copia explícita: pagas la copia y lo dices
    let c = &a;                 // PRESTAR en vez de mover ← esto es lo normal
    println!("{a} {b} {c}");
}
```

```bash
$ rustc --edition 2024 fig02_02.rs && ./fig02_02
hola hola hola
```

`a.clone()` crée un second `String`, avec sa propre mémoire et ses propres caractères. C'est pourquoi `a` et `b` peuvent vivre indépendamment. La copie a un coût proportionnel à la taille du texte : copier « hola » est minime ; copier une grosse réponse HTTP ou une liste de milliers de services peut ne pas l'être. Rust rend ce coût visible avec la méthode `clone()`.

Tu ne dois pas interpréter cela comme une interdiction de cloner. Une copie est correcte quand le programme a vraiment besoin de deux valeurs indépendantes : garder un nom pour le rapport et un autre pour l'envoyer à une tâche, conserver une configuration d'origine avant de la transformer ou séparer des données qui vivront des durées différentes. Le problème apparaît quand `clone()` est utilisé mécaniquement pour faire taire une erreur sans répondre à la question de qui doit posséder la donnée.

Le troisième nom, `c`, est une référence. `&a` ne copie pas les caractères et ne transfère pas la possession. Il crée un emprunt en lecture seule. C'est pourquoi on peut afficher `a`, `b` et `c` : `a` reste le propriétaire ; `b` est propriétaire d'une autre copie ; `c` ne pointe que temporairement vers `a`.

Les types qui implémentent le trait `Copy` se comportent différemment. Les entiers, les booléens, les caractères et les n-uplets composés exclusivement de valeurs `Copy` se copient implicitement, parce que les dupliquer est bon marché et qu'ils n'ont pas de mémoire à libérer. Si tu écris `let b = a` quand `a` est un `u64`, tu peux utiliser les deux noms. Ce n'est pas que la possession disparaisse : chaque variable reçoit sa propre copie de la valeur.

`String` n'implémente pas `Copy` parce que copier implicitement ses trois données internes — pointeur, longueur et capacité — produirait deux gestionnaires pour le même bloc du tas. Rust pourrait aussi copier les caractères, mais alors chaque affectation cacherait potentiellement un travail coûteux. C'est pourquoi il distingue le déplacement du clonage.

Le revisor ne clone que lorsqu'il doit construire une sortie qui doit posséder son propre texte. Le rapport JSON ne peut pas conserver des références vers des services locaux dans une fonction déjà terminée. C'est pourquoi il convertit le nom emprunté du service en un `String` indépendant.

<!-- verificar:extracto:src/reporte.rs -->
```rust
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
```

Ici, `servicios` et `estados` sont prêtés à la fonction de rapport. `s.nombre.clone()` et `motivo.clone()` sont des décisions nécessaires : `EstadoJson` doit survivre comme élément de `lineas` puis être converti en JSON. Le code ne clone pas par peur du compilateur ; il clone parce que le résultat a son propre propriétaire.

### Références immuables : emprunter pour lire

Une référence est une façon de permettre l'accès à une valeur sans transférer sa propriété. Elle s'écrit `&T` : « une référence vers un `T` ». Si tu as un `String` et qu'une fonction n'a besoin que de connaître sa longueur, passer `&String` évite de créer une copie et évite que la fonction consomme le texte.

**Fig. 2.3** | Emprunter pour lire.

```rust
// fig02_03.rs
fn largo(s: &String) -> usize { s.len() }      // presta, no toma posesión

fn main() {
    let s = String::from("hola");
    let n = largo(&s);
    println!("{s} mide {n}");                   // sigue siendo mía ✓
}
```

```bash
$ rustc --edition 2024 fig02_03.rs && ./fig02_03
hola mide 4
```

La fonction `largo` reçoit une référence. À l'intérieur, `s.len()` consulte la longueur, mais elle ne peut ni garder le `String` ni le modifier. Quand l'appel se termine, l'emprunt se termine et le propriétaire d'origine reste la variable `s` de `main`. Cela explique pourquoi la dernière ligne peut afficher à la fois le texte et sa longueur.

L'exemple conserve `&String` parce qu'il montre directement le contraste entre un `String` propriétaire et une référence vers lui. Dans une API générale, il convient de recevoir `&str` quand tu as seulement besoin de lire du texte. `&str` est une vue d'une séquence UTF-8 ; il accepte aussi bien un littéral qu'une référence vers un `String`. La leçon 4 approfondira cette distinction, mais dès maintenant tu peux utiliser une règle pratique : garde le texte que tu possèdes dans un `String` ; reçois le texte en lecture seule comme `&str`.

Une référence n'est pas une copie de la valeur. Elle a une durée de vie limitée par la valeur vers laquelle elle pointe. Rust ne permet pas de renvoyer une référence vers une variable locale qui disparaîtra à la sortie d'une fonction, ni de conserver une référence quand son propriétaire a déjà été déplacé. Cette partie de l'analyse s'appelle la vérification des emprunts, ou *borrow checking*.

La référence rend visible le contrat d'une fonction. Une signature qui reçoit `String` communique « j'ai besoin de prendre ce texte ». Une qui reçoit `&str` communique « j'ai seulement besoin de le lire ». En Go, passer un `string` est bon marché parce que sa représentation est copiée ; passer une grosse structure par valeur ou par pointeur oblige à lire la documentation et à connaître son implémentation. Rust fait de cette différence une partie de la signature.

Les fonctions du rapport reçoivent des slices empruntés. Elles ne consomment ni le vecteur de services ni le vecteur d'états, parce que `main` en a encore besoin pour décider du code de sortie. La signature exprime cette intention sans commentaires supplémentaires.

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

`&[Servicio]` signifie « slice emprunté de services ». Un slice emprunte une partie contiguë d'une collection et sait combien d'éléments elle contient. La fonction peut la parcourir, consulter les noms et calculer la largeur du tableau, mais elle ne peut pas vider le vecteur, ajouter des services ni se les approprier. La même décision vaut pour `&[Estado]`.

### Références mutables : emprunter pour modifier

Une référence mutable s'écrit `&mut T`. Elle sert quand une fonction doit modifier une valeur dont la propriété reste à celui qui appelle. Pour la créer, il te faut deux choses : le propriétaire doit être déclaré avec `mut`, et l'emprunt doit s'écrire `&mut`.

**Fig. 2.4** | Emprunter de façon exclusive pour modifier.

```rust
// fig02_04.rs
fn agregar_puerto(etiqueta: &mut String) {
    etiqueta.push_str(":443");
}

fn main() {
    let mut servicio = String::from("catalogo");
    agregar_puerto(&mut servicio);
    println!("{servicio}");
}
```

```bash
$ rustc --edition 2024 fig02_04.rs && ./fig02_04
catalogo:443
```

`servicio` est mutable parce que son contenu va changer. `agregar_puerto` ne reçoit pas le `String` par valeur : elle reçoit un emprunt exclusif et ajoute des caractères au même texte. Quand l'appel se termine, l'emprunt se termine et `main` utilise de nouveau le propriétaire pour l'afficher.

L'exclusivité est la condition importante. Pendant un emprunt `&mut`, personne d'autre ne peut lire ou modifier la même donnée par une autre référence. Ce n'est pas une limitation arbitraire : si une partie du programme change une chaîne pendant qu'une autre suppose qu'elle la lit de façon stable, le résultat peut dépendre de l'ordre d'exécution. Dans les programmes concurrents, cette situation est une course aux données (data race).

Les deux règles des emprunts sont :

1. Tu peux avoir un nombre quelconque de références immuables vers une valeur.
2. Tu peux avoir exactement une référence mutable vers une valeur, ou des références immuables, mais pas les deux en même temps.

La façon brève de s'en souvenir est : beaucoup de lectures ou une seule écriture. Une lecture ne change pas la donnée et peut être partagée. Une écriture a besoin d'exclusivité parce qu'elle pourrait changer n'importe quelle partie de la valeur.

Rust analyse aussi le dernier usage réel d'une référence. Tu n'as pas forcément besoin d'attendre l'accolade finale du bloc pour demander un emprunt mutable. Si une référence immuable ne sera plus utilisée, Rust peut considérer que son emprunt est terminé. C'est ce qu'on appelle les emprunts non lexicaux. Tu ne dois pas t'appuyer dessus pour écrire du code confus, mais cela explique pourquoi séparer une lecture et une modification en étapes claires compile généralement.

Dans le revisor, le tableau se construit avec une variable mutable. La propriété de `salida` reste dans `tabla`, mais `push_str` a besoin d'un emprunt mutable temporaire pour ajouter chaque ligne. À la sortie de la fonction, `tabla` renvoie le `String` complet et la propriété passe à celui qui a appelé.

<!-- verificar:extracto:src/reporte.rs -->
```rust
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
```

`push_str` modifie `salida` ; c'est pourquoi la variable est déclarée avec `mut`. En revanche, `s` et `e` sont des références de lecture vers les données de chaque ligne. Le compilateur permet les deux parce que la fonction modifie le nouveau rapport, pas les services ni les états qu'elle consulte.

### Slices, `&str` et références qui renvoient des références

La possession n'oblige pas à copier quand tu veux obtenir une partie d'une valeur. Une fonction peut recevoir une référence et renvoyer une autre référence vers une partie de la même information, à condition que Rust puisse vérifier que la sortie ne vivra pas plus longtemps que l'entrée. Ce motif apparaît avec les slices de tableaux, les slices de vecteurs et `&str`.

**Fig. 2.5** | Renvoyer une vue empruntée d'un texte.

```rust
// fig02_05.rs
fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let texto = String::from("revisor listo");
    let palabra = primera_palabra(&texto);
    println!("primera: {palabra}");
}
```

```bash
$ rustc --edition 2024 fig02_05.rs && ./fig02_05
primera: revisor
```

`primera_palabra` ne construit pas de nouveau `String`. Elle renvoie une vue vers une partie de `s`. C'est pourquoi elle est efficace : elle ne copie pas les caractères. Elle a aussi une limitation saine : `palabra` ne peut pas survivre à `texto`, parce qu'elle pointe à l'intérieur. Si `texto` est modifié d'une façon qui change son stockage, une ancienne référence pourrait cesser d'être valide ; Rust t'empêche d'utiliser les deux choses de manière incompatible.

La signature `fn primera_palabra(s: &str) -> &str` utilise une règle d'inférence des durées de vie (lifetimes). Le compilateur comprend que la référence de sortie est liée à la référence d'entrée. À la leçon 5, tu verras les cas où tu dois écrire une annotation comme `'a` ; pour l'instant, retiens l'idée importante : la fonction ne possède pas le mot renvoyé, donc elle ne peut pas promettre qu'il existera plus longtemps que le texte emprunté.

Le revisor utilise des durées de vie explicites quand il assemble des lignes qui ne font qu'emprunter des données de deux slices. Il ne duplique pas chaque service et chaque état avant de les trier ; il conserve des références valides tant que les vecteurs d'origine restent vivants.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

L'annotation `'a` dit que les références à l'intérieur de `Fila` ne peuvent pas vivre plus longtemps que les slices prêtés à `ordenadas`. `filas` est propriétaire du vecteur de références, mais pas des services ni des états. Cette distinction est la base de beaucoup de programmes Rust efficaces : posséder la collection n'implique pas posséder toutes les données vers lesquelles elle pointe.

## L'erreur que tu vas voir

### E0382 : utiliser une valeur après l'avoir déplacée

Le programme suivant ne compile pas, exprès. L'affectation `let b = a` déplace le `String` de `a` vers `b`. La dernière ligne essaie d'emprunter `a` pour l'afficher, mais `a` n'est plus propriétaire et ne peut plus être prêté.

**Fig. 2.6** | Déplacer, pas copier.

```rust
// fig02_06.rs
fn main() {
    let a = String::from("hola");
    let b = a;                  // NO copia: MUEVE. Ahora b es el dueño
    println!("{a}");            // ← error: valor movido
}
```

```bash
$ rustc --edition 2024 fig02_06.rs
error[E0382]: borrow of moved value: `a`
 --> fig02_06.rs:5:16
  |
3 |     let a = String::from("hola");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es el dueño
  |             - value moved here
5 |     println!("{a}");            // ← error: valor movido
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let b = a.clone();                  // NO copia: MUEVE. Ahora b es el dueño
  |              ++++++++

warning: unused variable: `b`
 --> fig02_06.rs:4:9
  |
4 |     let b = a;                  // NO copia: MUEVE. Ahora b es el dueño
  |         ^ help: if this is intentional, prefix it with an underscore: `_b`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
```

`E0382` signifie que tu as essayé d'utiliser une valeur après avoir transféré sa propriété. Le message signale trois endroits : où `a` est né, où le déplacement a eu lieu et où tu as essayé de l'utiliser à nouveau. Cette séquence est plus utile que de mémoriser le code de l'erreur : suis les flèches et demande-toi qui est le propriétaire après chaque ligne.

Il y a trois corrections possibles, et elles ne sont pas interchangeables. Si tu n'as plus besoin de `a`, affiche `b`. Si tu as besoin de deux valeurs indépendantes, utilise `a.clone()` et accepte le coût de la copie. Si la seconde partie a seulement besoin de lire la valeur, change la conception pour emprunter `&a` au lieu de la déplacer. La troisième option est généralement la meilleure quand tu écris des fonctions auxiliaires pour le revisor.

L'avertissement sur `b` apparaît parce que le programme n'arrive pas à l'utiliser. Ce n'est pas l'erreur principale ; c'est une conséquence du fait que l'exemple utilise `a` exprès pour provoquer `E0382`. Les programmes qui doivent compiler dans le cours sont vérifiés avec `-D warnings`, donc une variable inutilisée devient aussi un problème que tu dois corriger.

### E0502 : demander une écriture alors que des lectures existent

L'erreur suivante représente la deuxième règle des emprunts. `r1` et `r2` sont des références immuables vivantes parce qu'elles sont utilisées dans le `println!` final. Tant que ces lectures existent, Rust ne peut pas créer `r3`, une référence mutable vers le même `String`.

**Fig. 2.7** | Lectures et écriture en même temps.

```rust
// fig02_07.rs
fn main() {
    let mut s = String::from("hola");
    let r1 = &s;                  // lectura, ok
    let r2 = &s;                  // otra lectura, ok
    let r3 = &mut s;              // ← error: ya hay lecturas vivas
    println!("{r1} {r2} {r3}");
}
```

```bash
$ rustc --edition 2024 fig02_07.rs
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> fig02_07.rs:6:14
  |
4 |     let r1 = &s;                  // lectura, ok
  |              -- immutable borrow occurs here
5 |     let r2 = &s;                  // otra lectura, ok
6 |     let r3 = &mut s;              // ← error: ya hay lecturas vivas
  |              ^^^^^^ mutable borrow occurs here
7 |     println!("{r1} {r2} {r3}");
  |                -- immutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

`E0502` signifie que tu as demandé un emprunt mutable alors qu'un emprunt immuable est actif. Rust ne suppose pas que « sûrement, il ne se passera rien ». Un emprunt mutable pourrait remplacer, raccourcir, vider ou réallouer le contenu de `s`, de sorte que les références de lecture n'auraient plus une vue cohérente.

La correction n'est pas de copier `s` par réflexe. Décide d'abord si tu as vraiment besoin de lire et de modifier en même temps. Si ce n'est pas le cas, termine les lectures avant de demander l'écriture : affiche ou calcule avec `r1` et `r2`, cesse de les utiliser, puis crée la référence mutable. Si tu as besoin de conserver de l'information de lecture pendant que tu modifies, garde une petite copie de la donnée nécessaire, comme une longueur ou un indicateur, pas nécessairement une copie complète de la structure.

Cette erreur est une version locale d'une garantie qui sera décisive à la leçon 7. En Go, deux goroutines qui lisent et écrivent des données partagées sans coordination peuvent avoir une course aux données (data race) qui n'apparaît qu'à l'exécution. Rust établit ces règles avant de parler de threads ; quand tu arriveras à `Arc`, `Mutex` et aux canaux, le même modèle continuera à protéger les accès partagés.

## Ce qui se fait mal

### Utiliser `clone()` pour faire taire chaque erreur

Le compilateur suggère `clone()` dans plusieurs messages parce que c'est une solution mécanique et sûre : elle crée une valeur indépendante. Cependant, la suggestion ne connaît ni la conception de ton programme ni la taille de tes données. Si tu clones un petit `String` une fois, cela n'a probablement pas d'importance. Si tu clones une liste de services dans chaque fonction ou si tu dupliques de gros corps HTTP dans une boucle, tu ajoutes du temps et de la mémoire sans en avoir besoin.

Avant d'écrire `.clone()`, demande-toi si la fonction a seulement besoin de lire. Si la réponse est oui, reçois `&T` ou `&str`. Si elle doit modifier quelque chose mais que le propriétaire doit le conserver, reçois `&mut T`. Clone quand tu as besoin de deux propriétaires réels, comme le JSON du rapport et les données d'origine qui restent vivantes pour une autre opération.

### Recevoir `String` par valeur quand tu vas seulement lire

Une signature qui reçoit `String` fait que celui qui appelle cède la propriété. Cela peut être correct pour une fonction qui normalise, consomme ou stocke le texte. C'est inutile pour une fonction qui se contente d'afficher, de mesurer ou de chercher un mot. Le coût n'est pas toujours une copie : parfois, celui qui appelle peut déplacer la valeur. Le problème est que la signature réduit les options d'usage de la personne qui appelle.

Pour les fonctions de consultation, préfère `&str` si tu travailles avec du texte et `&T` si tu travailles avec un autre type. L'API sera plus flexible : elle acceptera des littéraux, des `String` et des portions de texte sans obliger à créer de nouveaux propriétaires. La leçon 4 montrera pourquoi `&str` est normalement une meilleure frontière publique que `&String`.

### Penser que `mut` signifie « je peux emprunter en mutable quand je veux »

`let mut s` permet de modifier `s`, mais n'élimine pas les règles d'emprunt. La mutabilité appartient au propriétaire ; l'exclusivité appartient à chaque emprunt. Tu peux déclarer une chaîne mutable et recevoir quand même `E0502` si des références de lecture sont actives. Tu peux aussi avoir une variable immuable qui contient une référence mutable créée dans un autre contexte ; les deux concepts sont distincts.

Utilise `mut` seulement quand le nom doit changer ou quand tu vas demander un emprunt mutable. Si une variable ne change jamais, retirer `mut` laisse une intention plus claire et évite des avertissements du compilateur.

### Se battre contre l'emprunt au lieu de réduire sa portée

Une référence vit jusqu'à son dernier usage, pas forcément jusqu'à la fin visuelle du bloc. Si le compilateur n'accepte pas un emprunt mutable, vérifie où la référence précédente est utilisée pour la dernière fois. Beaucoup de corrections consistent à réorganiser quelques lignes : finis de lire, garde le résultat dont tu as besoin, puis modifie. Séparer les phases de lecture et d'écriture améliore à la fois la lisibilité et la compatibilité avec le borrow checker.

Ne cache pas le problème derrière une longue référence, un `unsafe` ou une structure globale. Le revisor est encore petit ; si le modèle de propriété devient difficile à expliquer, c'est généralement le signe qu'une fonction a trop de responsabilités ou qu'une donnée est partagée plus que nécessaire.

### Confondre `String` et `&str`

`String` possède du texte et peut grandir ; `&str` est une vue d'un texte qui appartient à autre chose. Convertir un `&str` en `String` avec `to_string()` ou `String::from()` est correct quand tu vas le stocker. Le faire seulement parce qu'une fonction pourrait recevoir une référence est une copie évitable. De l'autre côté, renvoyer `&str` quand le texte a été construit à l'intérieur de la fonction ne peut pas fonctionner : le texte local disparaît à la fin de la fonction.

La question utile est toujours la même : qui doit posséder ces caractères après cette opération ? Si la réponse est « la structure qui les stocke », utilise `String`. Si la réponse est « personne de nouveau ; j'ai seulement besoin de les observer maintenant », utilise `&str`.

## Exercices

### Exercice 1 — Suis le propriétaire

Lis les situations suivantes et écris, avant de compiler, quel nom peut être utilisé à la fin : une affectation de `u64` ; une affectation de `String` ; et une affectation de `String` suivie de `clone()`. Ensuite, crée trois petits fichiers et vérifie tes réponses avec `rustc --edition 2024`.

Explique en une phrase pourquoi l'entier se copie, pourquoi le `String` se déplace et pourquoi le `clone()` produit deux propriétaires. N'utilise pas `Copy` comme un mot magique : relie-le au coût et à la nécessité de libérer la mémoire.

### Exercice 2 — Une fonction qui prend et une autre qui emprunte

Écris deux fonctions sur un nom de service. La première doit recevoir un `String` par valeur et renvoyer sa longueur. La seconde doit recevoir un `&str` et renvoyer la même longueur. Dans `main`, démontre qu'après avoir appelé la première fonction tu ne peux plus afficher le `String`, et qu'après avoir appelé la seconde tu peux l'afficher.

Laisse d'abord active la ligne qui provoque `E0382` et lis le diagnostic complet. Ensuite, mets cette ligne en commentaire pour que le programme compile. Ne corrige pas la première fonction avec `clone()` : l'objectif est d'observer la différence entre prendre la propriété et emprunter.

### Exercice 3 — Mets à jour un service sans changer de propriétaire

Écris `fn agregar_puerto(etiqueta: &mut String)` pour ajouter `:443` à une étiquette. Déclare un `String` mutable dans `main`, prête-le à la fonction et vérifie que `main` peut afficher le résultat à la fin.

Ensuite, provoque `E0502` : crée une référence en lecture seule vers le même texte, utilise-la après avoir demandé une référence mutable et observe la ligne signalée par le compilateur. Réordonne le programme pour que la lecture se termine avant de modifier.

### Exercice 4 — Le premier mot emprunté

Implémente `fn primera_palabra(s: &str) -> &str`. Elle doit renvoyer le premier mot d'une phrase, ou une chaîne vide si elle ne reçoit que des espaces. Teste-la avec un `String` appelé `texto`, affiche le résultat puis affiche aussi `texto`.

Fais les exercices `move_semantics` et `primitive_types` de Rustlings. En particulier, n'avance pas par essais et erreurs avec `clone()` : dans chaque solution, identifie si Rust te demande de déplacer, de copier ou d'emprunter.

## Solutions

### Solution 1

Un `u64` implémente `Copy`, donc après `let b = a` il existe deux valeurs indépendantes et les deux noms sont utilisables. Un `String` n'implémente pas `Copy` ; la même affectation déplace la propriété vers `b`, donc `a` cesse d'être utilisable. Si tu écris `let b = a.clone()`, `a` et `b` possèdent deux blocs de texte distincts et les deux peuvent être utilisés.

L'épreuve ne consiste pas à se souvenir de quels types implémentent `Copy`, mais à faire une prédiction et à la vérifier. Quand tu as un doute sur un type à toi, le compilateur te dira s'il implémente `Copy`. Dans les structs du revisor qui contiennent un `String`, suppose d'abord que la valeur se déplace.

### Solution 2

La fonction qui reçoit `String` consomme l'argument. Sa signature doit ressembler à `fn largo_tomando(s: String) -> usize` ; après l'appel, le `String` d'origine n'est plus disponible. La fonction qui reçoit `&str` doit ressembler à `fn largo_prestando(s: &str) -> usize` ; appelle-la avec `&nombre` puis affiche `nombre`.

La différence ne tient pas au nombre qu'elles renvoient, mais au contrat d'entrée. Pour une fonction qui ne fait que calculer la longueur, la seconde signature est la bonne. La première existe pour que tu observes explicitement le déplacement et pour les cas réels où une fonction a effectivement besoin de garder la valeur.

### Solution 3

La solution est celle de la figure 2.4 : le propriétaire est déclaré `let mut servicio`, on appelle la fonction avec `&mut servicio` et on affiche après la fin de l'appel. La fonction ne renvoie pas le `String` parce qu'elle ne l'a jamais reçu en propriété.

Pour corriger le conflit d'emprunts, utilise complètement la référence de lecture avant de créer la référence mutable. Le point important n'est pas de mettre les deux références dans des blocs artificiels, mais de rendre visible que la phase de lecture s'est terminée avant la phase d'écriture.

### Solution 4

La solution est celle de la figure 2.5. `split_whitespace()` ignore les espaces initiaux et sépare les mots ; `next()` produit un `Option<&str>` ; `unwrap_or("")` renvoie une chaîne vide s'il n'y avait aucun mot. Le résultat est une référence prise dans l'entrée, pas un nouveau `String`.

Le bon test affiche d'abord le mot puis le `String` d'origine. Cela démontre que `primera_palabra` n'a pas pris la propriété de `texto`. Si tu essayais de renvoyer une référence vers un `String` créé à l'intérieur de la fonction, Rust le rejetterait, parce que ce `String` serait détruit à la fin de l'appel.

## Comment savoir que j'y suis arrivé

- [ ] `rustc --edition 2024 fig02_01.rs && ./fig02_01` affiche `hola`.
- [ ] `rustc --edition 2024 fig02_02.rs && ./fig02_02` affiche trois fois `hola` et je peux expliquer quelle valeur a été clonée et laquelle a été empruntée.
- [ ] `rustc --edition 2024 fig02_06.rs` échoue avec `E0382`, et je sais expliquer à quelle ligne la propriété a été déplacée.
- [ ] `rustc --edition 2024 fig02_07.rs` échoue avec `E0502`, et je sais le corriger en terminant d'abord les lectures.
- [ ] `rustc --edition 2024 fig02_04.rs && ./fig02_04` affiche `catalogo:443`.
- [ ] `rustc --edition 2024 fig02_05.rs && ./fig02_05` affiche `primera: revisor`.
- [ ] J'ai terminé `move_semantics` et `primitive_types` de Rustlings sans utiliser `clone()` comme solution automatique.
- [ ] Je peux expliquer en une phrase pourquoi Rust libère la mémoire à la sortie de la portée sans avoir besoin d'un ramasse-miettes.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 4 : Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html), consulté le 2 octobre 2026.
- [The Rust Programming Language, références et emprunts](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html), consulté le 2 octobre 2026.
- [Documentation officielle de `String`](https://doc.rust-lang.org/std/string/struct.String.html), consulté le 2 octobre 2026.
- [Rustlings](https://rustlings.rust-lang.org/), exercices `move_semantics` et `primitive_types`, consulté le 2 octobre 2026.
