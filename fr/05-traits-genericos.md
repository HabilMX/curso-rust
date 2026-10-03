# Leçon 5 — Traits, génériques et lifetimes

**Durée :** 2 × 45 min.

**Ce que tu construis :** le trait `Revisor` et une fonction générique qui l'utilise, dans des programmes à part (le `revisor` réel ne déclare aucun trait).

**Ce que tu apprends :** traits et méthodes par défaut, génériques avec contraintes, lifetimes et le `'a` qui fait peur.

## À la fin, tu vas pouvoir

- Définir un trait, implémenter son contrat pour deux types et utiliser une méthode par défaut.
- Distinguer une implémentation inhérente (`impl Tipo`) d'une implémentation de trait (`impl Trait for Tipo`).
- Écrire une fonction générique avec contraintes et expliquer quand Rust génère du code spécialisé.
- Choisir entre un paramètre générique et `dyn Trait` selon que tu dois décider du type à la compilation ou à l'exécution.
- Lire une signature avec `'a` et expliquer quelles références sont reliées par ce lifetime.
- Reconnaître une erreur de lifetime ou de trait bound, en localiser la cause et corriger la conception sans copies inutiles.

## Le pourquoi avant le comment

Jusqu'à la leçon 4, le `revisor` a déjà un modèle utile. Il peut représenter un `Servicio`, stocker une liste dans un `Vec`, distinguer des résultats avec `Estado`, charger de la configuration et rapporter des erreurs. Pourtant, il reste une question de conception qui apparaît chaque fois que le programme grandit : comment sépares-tu ce que le programme doit faire de la manière concrète dont cela se fait ?

Le `revisor` doit obtenir un `Estado` pour chaque `Servicio`. Aujourd'hui, l'implémentation réelle fait une requête HTTP avec `reqwest::Client` ; demain, tu pourrais vouloir une implémentation qui lise un fichier, interroge une base de données, mesure un processus local ou simule des réponses pour un test. Le reste du programme ne devrait pas avoir à connaître tous ces détails. Il a seulement besoin de pouvoir demander : « vérifie ce service et renvoie son état ».

En Go, ce contrat s'exprime avec une interface. Un type satisfait une interface de manière implicite : s'il a les méthodes requises, il la remplit déjà. Cette décision rend très facile l'adaptation de types existants, mais peut aussi cacher des relations importantes. Un type peut finir par remplir une interface par accident, et en lisant sa définition tu ne sais pas toujours à quels contrats il participe dans d'autres paquets.

Rust utilise les traits pour résoudre le même genre de problème, mais exige de déclarer la relation de façon explicite. Un trait décrit des capacités ; ensuite, `impl Revisor for RevisorHttp` déclare que ce type remplit cette capacité. C'est une ligne de plus, mais c'est une ligne qui documente l'architecture. En la lisant, tu sais que `RevisorHttp` n'a pas seulement une méthode appelée `revisar` : il s'est engagé envers le contrat `Revisor`.

Les traits ne remplacent ni les structs ni les enums. Chaque outil répond à une question différente. Une `struct` dit quelles données forment une chose ; un `enum` dit quelles alternatives valides existent ; un trait dit quelles opérations un type peut offrir. Le `Servicio` de la leçon 3 reste une struct parce qu'il modélise des données. `Estado` reste un enum parce qu'un service peut être sain, lent, échouer ou ne pas avoir été interrogé. `Revisor` est un trait parce qu'il décrit l'opération qui produit un état.

Les génériques permettent d'écrire une fonction qui travaille avec une famille de types sans perdre l'information sur le type concret qu'elle a reçu. La fonction `revisar_todos` de cette leçon peut accepter n'importe quel `R` qui implémente `Revisor`. Elle n'a pas besoin d'un `if` par implémentation ni de tout convertir en texte. Le compilateur connaît le type concret de `R` en compilant chaque appel et peut vérifier que la bonne méthode existe.

Les lifetimes complètent ce modèle quand tu travailles avec des références. La possession (ownership) a déjà établi que chaque valeur a un propriétaire et qu'une référence est un emprunt. Un lifetime ne crée pas une autre forme de propriété et ne prolonge aucune valeur. C'est une annotation qui aide le compilateur à démontrer qu'un emprunt restera valide pendant tout usage possible. Il apparaît surtout quand une fonction reçoit des références et renvoie une référence, ou quand une struct stocke des références.

La notation `'a` intimide parce qu'elle ressemble à une variable mystérieuse, mais elle se lit mieux comme une étiquette. Si une fonction reçoit deux références marquées `'a` et en renvoie une autre marquée `'a`, elle déclare : « la référence de sortie dépend de ces entrées et ne peut plus être utilisée une fois que la plus courte des références cesse d'être valide ». Elle ne dit pas combien de temps dure `'a` ; cela dépend de chaque appel. Elle ne réserve pas non plus de mémoire et ne fait pas de ramasse-miettes.

Cette leçon correspond au chapitre 10 de The Rust Book. Avant de continuer, lis ses sections sur les génériques, les traits et la validation des références, et fais les exercices `generics`, `traits` et `lifetimes` de Rustlings. L'objectif n'est pas de mémoriser toutes les syntaxes possibles de bounds et de lifetimes. C'est d'apprendre à reconnaître trois questions : quel comportement le programme a-t-il besoin d'avoir, quels types peuvent l'offrir et d'où viennent les références qui survivent à une fonction.

## Les concepts

### Traits : des contrats explicites et des méthodes par défaut

Un trait réunit des signatures de méthodes qui représentent une capacité. La signature dit ce que la méthode reçoit et ce qu'elle renvoie, sans décider comment elle fera le travail. Chaque type qui veut remplir le trait écrit sa propre implémentation. C'est pourquoi un trait ressemble à une interface de Go, mais sa relation avec le type est explicite.

La figure définit `Revisor` avec deux méthodes. `revisar` n'a pas de corps : toute implémentation doit décider comment vérifier un service. `nombre` a un corps et renvoie `"revisor"`. C'est une méthode par défaut. Une implémentation peut l'accepter telle quelle, comme `RevisorHttp`, ou la remplacer, comme `RevisorFalso`.

**Fig. 5.1** | Un trait avec méthode par défaut, et deux types qui le remplissent.

```rust
// fig05_01.rs
use std::fmt;

struct Servicio {
    nombre: String,
}

enum Estado {
    Ok { ms: u64 },
    Falla(String),
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Estado::Ok { ms } => write!(f, "OK en {ms}ms"),
            Estado::Falla(motivo) => write!(f, "FALLA: {motivo}"),
        }
    }
}

trait Revisor {
    fn revisar(&self, s: &Servicio) -> Estado;

    fn nombre(&self) -> String {
        "revisor".to_string()
    }
}

struct RevisorHttp { timeout_ms: u64 }

impl Revisor for RevisorHttp {
    fn revisar(&self, s: &Servicio) -> Estado {
        Estado::Falla(format!("{}: sin red en este ejemplo (límite {} ms)", s.nombre, self.timeout_ms))
    }
}

struct RevisorFalso;

impl Revisor for RevisorFalso {
    fn revisar(&self, _s: &Servicio) -> Estado {
        Estado::Ok { ms: 1 }
    }
    fn nombre(&self) -> String {
        "falso".to_string()
    }
}

fn main() {
    let s = Servicio { nombre: "catalogo".to_string() };
    let http = RevisorHttp { timeout_ms: 2000 };
    println!("{} -> {}", http.nombre(), http.revisar(&s));
    println!("{} -> {}", RevisorFalso.nombre(), RevisorFalso.revisar(&s));
}
```

```bash
$ rustc --edition 2024 fig05_01.rs && ./fig05_01
revisor -> FALLA: catalogo: sin red en este ejemplo (límite 2000 ms)
falso -> OK en 1ms
```

`impl Revisor for RevisorHttp` se lit de gauche à droite : « implémente le trait `Revisor` pour le type `RevisorHttp` ». À l'intérieur de ce bloc, Rust exige d'implémenter chaque méthode sans corps que le trait requiert. Si tu omets `revisar`, le programme ne compile pas. Si tu omets `nombre`, il compile, parce que le trait a déjà fourni une implémentation par défaut.

La méthode reçoit `&self`, comme les méthodes de struct de la leçon 3. Elle ne consomme pas le revisor et ne le modifie pas ; elle ne fait que l'emprunter pour consulter ses données. `revisar` reçoit aussi `&Servicio`, parce que vérifier un service ne doit pas le consommer. Le résultat, en revanche, est renvoyé par valeur : chaque vérification crée un nouvel `Estado` et l'appelant en reçoit la propriété.

Le trait `Display` de la figure vient de la bibliothèque standard. `impl fmt::Display for Estado` permet d'utiliser `{}` dans `println!`. L'implémentation décide d'une représentation destinée à une personne : `OK en 1ms` ou `FALLA: ...`. C'est différent de `Debug`, qui s'obtient normalement avec `#[derive(Debug)]` et s'affiche avec `{:?}` pour le diagnostic. Si le rapport fait partie de l'interface du programme, définir `Display` oblige à réfléchir à quel texte stable mérite de voir la personne qui l'exécute.

Un trait n'est pas une classe de base. Il ne stocke pas de champs, ne construit pas d'objets et n'hérite pas d'implémentation d'un parent. Il peut fournir un comportement par défaut, mais chaque type conserve ses propres données. `RevisorHttp` a `timeout_ms` ; `RevisorFalso` n'a besoin d'aucun champ. Les deux remplissent le même contrat parce que les deux peuvent répondre à `revisar(&Servicio)`.

Rust applique la règle de cohérence, aussi appelée règle des orphelins (orphan rule). Tu peux implémenter un trait à toi pour un type étranger, par exemple `impl Revisor for String` si cela avait un sens. Tu peux aussi implémenter un trait étranger pour un type à toi, comme `impl Display for Estado`. Ce que tu ne peux pas faire, c'est implémenter un trait étranger pour un type étranger : tu ne peux pas décider depuis ton crate comment un `Vec<String>` doit implémenter `Display`. La règle évite que deux dépendances différentes définissent des implémentations incompatibles du même contrat.

Le `revisor` réel ne déclare aucun trait pour ses requêtes HTTP. Sa fonction `revisar` reçoit un `reqwest::Client` concret, et ses tests d'intégration utilisent un serveur HTTP local au lieu d'un double de test. C'est une décision consciente : un trait qui n'aurait qu'une seule implémentation réelle ne résout encore aucun problème. Le trait `Revisor` de cette leçon vit dans des programmes à part, pour que tu t'exerces à la forme ; le projet en aurait besoin le jour où il existerait deux manières différentes de vérifier un service. En attendant, le programme utilise bien `impl` pour regrouper les méthodes propres aux types du domaine :

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

Ce bloc est une implémentation inhérente : `impl Estado`, sans `for`, ajoute une méthode qui appartient directement à `Estado`. Il n'implémente pas de trait. Distinguer les deux formes évite une confusion courante : toute implémentation de trait utilise `impl`, mais tout `impl` n'implémente pas un trait.

Un trait serait utile dans le `revisor` si l'application avait besoin d'échanger la source des vérifications au sein de la même conception. Par exemple, un test unitaire pourrait utiliser un revisor factice sans réseau. Tu ne dois pas créer un trait uniquement parce que Rust l'offre. L'abstraction a un coût de lecture : elle ajoute un contrat, des implémentations et des décisions sur la manière de les injecter. Le projet actuel teste HTTP au moyen d'un serveur local précisément parce qu'il veut vérifier le comportement réel de la couche HTTP.

### Génériques et contraintes : réutiliser sans effacer le type

Un paramètre générique est une variable de type. Dans `fn revisar_todos<R: Revisor>(...)`, `R` ne signifie pas « n'importe quelle valeur sans règles » ; il signifie « n'importe quel type qui implémente `Revisor` ». La partie après les deux-points est une contrainte, aussi appelée trait bound. Grâce à elle, le corps de la fonction peut appeler `r.revisar(s)` : le compilateur a la garantie que tout `R` admis fournit cette méthode.

**Fig. 5.2** | Une fonction générique avec contraintes.

```rust
// fig05_02.rs
use std::fmt;

struct Servicio {
    nombre: String,
}

enum Estado {
    Ok { ms: u64 },
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Estado::Ok { ms } => write!(f, "OK en {ms}ms"),
        }
    }
}

trait Revisor {
    fn revisar(&self, s: &Servicio) -> Estado;
}

struct RevisorFalso;

impl Revisor for RevisorFalso {
    fn revisar(&self, s: &Servicio) -> Estado {
        Estado::Ok { ms: s.nombre.len() as u64 }
    }
}

fn revisar_todos<R: Revisor>(r: &R, servicios: &[Servicio]) -> Vec<Estado> {
    servicios.iter().map(|s| r.revisar(s)).collect()
}

fn imprimir<T: std::fmt::Display + Clone>(x: T) { println!("{x}"); }

fn main() {
    let servicios = vec![
        Servicio { nombre: "catalogo".to_string() },
        Servicio { nombre: "pagos".to_string() },
    ];
    for estado in revisar_todos(&RevisorFalso, &servicios) {
        imprimir(estado.to_string());
    }
}
```

```bash
$ rustc --edition 2024 fig05_02.rs && ./fig05_02
OK en 8ms
OK en 5ms
```

La fonction reçoit `&R`, pas `R`. Cela maintient la propriété du revisor chez l'appelant et permet de l'utiliser pour tous les services. `servicios: &[Servicio]` est une tranche (slice) empruntée, comme les slices vues en parcourant des collections : la fonction peut lire les services, mais ne les consomme pas et n'a pas besoin d'une copie du `Vec`.

`map` reçoit chaque `&Servicio`, appelle `r.revisar(s)` et produit un itérateur d'états. `collect()` rassemble ces états dans un `Vec<Estado>` parce que le type de retour le demande. La fonction est générique sur le revisor, mais pas sur l'état : le contrat `Revisor` fixe que toute implémentation renvoie `Estado`. Ce choix est correct quand le domaine a besoin d'une seule représentation cohérente des résultats.

La forme `T: Display + Clone` montre plusieurs contraintes réunies avec `+`. Pourtant, `imprimir` n'utilise que `Display` ; il n'appelle pas `clone`. La contrainte `Clone` est là pour enseigner la syntaxe, pas parce qu'elle est nécessaire. Dans du code de production, tu dois demander uniquement les capacités dont le corps a besoin. Un bound de trop exclut des types valides et fait paraître l'API plus exigeante qu'elle ne l'est réellement.

Quand les bounds grossissent, Rust permet de les écrire avec `where`. Par exemple, une longue signature peut se terminer par `where R: Revisor, E: std::error::Error`. Cela ne change ni le comportement ni la vérification ; cela place seulement les contraintes là où elles se lisent mieux. Commence par la forme courte et utilise `where` quand la signature cesse d'être claire.

Les génériques de Rust se résolvent normalement par monomorphisation. Si tu appelles `revisar_todos` avec `RevisorFalso` puis avec un autre type `RevisorArchivo`, le compilateur génère des versions spécialisées pour ces types concrets. À l'exécution, il n'a pas besoin de chercher la méthode dans une table pour ces appels. Cela s'appelle le dispatch statique (static dispatch). Le bénéfice est une performance prévisible et des vérifications plus précises ; le coût est que chaque combinaison de types peut augmenter le code compilé.

`impl Revisor` dans un paramètre est une façon brève d'écrire un générique en entrée. Une signature comme `fn ejecutar(r: impl Revisor)` équivaut, pour ce cas simple, à `fn ejecutar<R: Revisor>(r: R)`. La forme avec `<R: Revisor>` est préférable quand tu dois utiliser le même type générique plus d'une fois dans la signature, le renvoyer ou ajouter des relations entre plusieurs paramètres.

Quand la décision du type doit être prise à l'exécution, le trait object (objet trait) apparaît : `Box<dyn Revisor>`. Un `Vec<Box<dyn Revisor>>` peut stocker dans la même collection un `RevisorHttp`, un `RevisorFalso` et d'autres revisors de tailles différentes. En contrepartie, chaque appel passe par une indirection et la valeur vit généralement derrière un pointeur comme `Box`, `&` ou `Arc`. C'est l'équivalent le plus proche d'une interface de Go à l'exécution.

Il n'y a pas d'option universellement meilleure. Utilise les génériques quand le type concret est connu là où l'appel est compilé et que tu veux conserver cette information. Utilise `dyn Trait` quand le programme doit choisir ou combiner des implémentations pendant l'exécution. En Go, les interfaces portent généralement un dispatch dynamique de par leur conception habituelle ; en Rust, tu choisis explicitement entre les deux modèles.

Le projet réel utilise aussi des types génériques de la bibliothèque standard, même s'il ne déclare pas de fonction propre avec `<T>`. `Option<T>` exprime qu'il peut y avoir ou non une valeur de n'importe quel type, et ici il est spécialisé en `Option<u16>` pour un code HTTP :

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

`Option<u16>` et `Option<String>` sont des usages distincts du même type générique. Le premier permet de représenter qu'un échec n'a pas eu de code HTTP ; le second permet d'omettre le champ d'erreur quand un service a bien répondu. Les génériques ne sont pas seulement une technique pour des bibliothèques sophistiquées : `Vec<T>`, `Option<T>`, `Result<T, E>` et `HashMap<K, V>` font partie du travail quotidien en Rust.

### Lifetimes : décrire des emprunts qui se relient

Un lifetime est une région de validité d'une référence. Presque toujours, Rust l'infère, comme il infère beaucoup de types locaux. Tu as besoin d'écrire une annotation quand la signature pourrait permettre plusieurs relations entre références et que le compilateur ne peut pas savoir laquelle la conception garantit.

La figure renvoie l'une de deux références. Sans annotation, la signature ne peut pas communiquer si le résultat vient de `a`, de `b` ou d'ailleurs. En marquant les trois références avec `'a`, tu déclares que le résultat sera valide pendant une période qui ne peut pas dépasser celle d'aucune des entrées choisies.

**Fig. 5.3** | Un lifetime qui relie la sortie aux deux entrées.

```rust
// fig05_03.rs
fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", mas_largo("catalogo", "pagos"));
}
```

```bash
$ rustc --edition 2024 fig05_03.rs && ./fig05_03
catalogo
```

`'a` ne signifie pas « vit pour toujours » ni « vit exactement aussi longtemps que les deux entrées ». C'est un nom pour une relation. Dans un appel concret, Rust calcule un lifetime qui tient dans les emprunts valides. Si `a` dure dix lignes et `b` en dure trois, le résultat ne pourra être utilisé que pendant les trois lignes compatibles. La signature évite que quelqu'un conserve une référence au résultat puis détruise la valeur dont elle provenait.

La fonction ne choisit pas quelle chaîne dure le plus longtemps. Cela dépend des portées (scopes) de l'appelant, pas du nombre de lettres ni d'une valeur stockée en mémoire. `mas_largo` compare des longueurs pour choisir un contenu, mais le lifetime parle de la validité des références. Ce sont deux affaires différentes qui apparaissent par hasard dans la même fonction.

Rust a des règles d'élision qui rendent invisibles beaucoup de lifetimes. Par exemple, `fn nombre(s: &str) -> &str` compile sans écrire `'a` parce qu'il n'y a qu'une seule référence en entrée et que Rust peut y associer la sortie. Il infère aussi généralement les lifetimes des méthodes qui reçoivent `&self`. Quand il y a deux entrées possibles, comme dans `mas_largo`, il n'est plus sûr de deviner et tu dois décrire la relation.

N'ajoute pas `'a` à tout ce qui te paraît compliqué. Une annotation ne répare pas une référence invalide ; elle déclare seulement une relation que le compilateur vérifiera. Si tu essaies de renvoyer une référence à un `String` local, il n'existe aucun lifetime qui puisse rendre cet emprunt valide. Le `String` est détruit à la fin de la fonction. La solution est de renvoyer le `String` par valeur, de recevoir une référence qui appartient à l'appelant ou de repenser qui possède la donnée.

Le lifetime spécial `'static` mérite de l'attention. Une référence `&'static str` pointe généralement vers un texte littéral inclus dans le binaire, comme `"OK"` ou `"FALLA"`. Cela ne signifie pas « utilise `'static` pour supprimer les erreurs ». Forcer `'static` sur une donnée qui vit en réalité peu de temps ne la fait pas durer plus longtemps ; le compilateur le rejettera. Utilise `'static` seulement quand la valeur vit réellement pendant toute l'exécution.

Le `revisor` réel utilise des lifetimes là où ils sont nécessaires : dans une ligne temporaire qui emprunte un `Servicio` et son `Estado` correspondant pour trier le rapport. Il ne copie pas ces valeurs seulement pour les trier. Il construit des références, les stocke dans un vecteur local et laisse le compilateur vérifier que le vecteur ne survit pas à ses sources.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

`Fila<'a>` est un alias pour un n-uplet (tuple) de deux références. Il ne possède ni `Servicio` ni `Estado` ; il ne fait que les emprunter. `ordenadas` reçoit deux slices avec le même lifetime annoté et renvoie des lignes qui portent aussi ce lifetime. Par conséquent, personne ne peut conserver les lignes après la disparition des vecteurs d'origine. Le vecteur de lignes peut changer d'ordre parce qu'il est propriétaire du vecteur, mais il ne peut pas modifier les services ni les états parce qu'il ne fait que les emprunter.

La même fonction montre une raison pratique de préférer les références : elle évite de cloner de l'information seulement pour l'afficher triée. Cloner serait valide si tu avais besoin d'une collection indépendante qui survive au rapport, mais ce n'est pas nécessaire ici. Le rapport finit d'utiliser `filas` avant que `servicios` et `estados` ne se terminent, donc les emprunts expriment exactement le modèle de données.

Quand un lifetime apparaît dans une struct ou un alias, ne le lis pas comme une syntaxe cérémonielle. Demande-toi : « ce type stocke-t-il une référence ? » Si la réponse est oui, l'annotation lie le type à la durée de la valeur empruntée. Si la réponse est non, le type devrait probablement posséder un `String`, un `Vec<T>` ou une autre valeur et n'a pas besoin de lifetime explicite.

## L'erreur que tu vas voir

### E0515 : renvoyer une référence à une valeur locale

Cette erreur apparaît quand une fonction essaie d'emprunter quelque chose qui cesse d'exister au retour. La figure suivante échoue exprès. Le lifetime `'a` de la signature ne peut pas sauver `nombre` : ce `String` est la propriété de `devolver` et est détruit à la fermeture de la fonction.

**Fig. 5.4** | Un emprunt qui essaie de s'échapper de la valeur qui le possède.

```rust
// fig05_04.rs
fn devolver<'a>() -> &'a str {
    let nombre = String::from("catalogo");
    &nombre
}

fn main() {
    println!("{}", devolver());
}
```

```bash
$ rustc --edition 2024 fig05_04.rs
error[E0515]: cannot return reference to local variable `nombre`
 --> fig05_04.rs:4:5
  |
4 |     &nombre
  |     ^^^^^^^ returns a reference to data owned by the current function

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0515`.
```

Le message désigne exactement la référence qui prétend s'échapper. N'ajoute pas un autre lifetime et n'utilise pas `&'static str` : aucun ne change qui possède `nombre`. Si la fonction doit créer le texte, la correction est de renvoyer `String`. Si elle doit renvoyer une vue d'un texte qui existait déjà, reçois `&str` en paramètre et relie le lifetime de sortie à cette entrée.

### E0277 : le type ne remplit pas la contrainte demandée

Un trait bound est aussi un contrat vérifiable. Dans cette figure, `imprimir` demande un type qui implémente `Display`, mais `Vec<&str>` n'a pas cette implémentation. Rust ne le convertit pas implicitement en texte parce qu'il n'existe pas de représentation unique correcte pour toutes les collections.

**Fig. 5.5** | Un argument qui ne remplit pas le trait bound.

```rust
// fig05_05.rs
use std::fmt::Display;

fn imprimir<T: Display>(valor: T) {
    println!("{valor}");
}

fn main() {
    imprimir(vec!["catalogo"]);
}
```

```bash
$ rustc --edition 2024 fig05_05.rs
error[E0277]: `Vec<&str>` doesn't implement `std::fmt::Display`
 --> fig05_05.rs:9:14
  |
9 |     imprimir(vec!["catalogo"]);
  |     -------- ^^^^^^^^^^^^^^^^ the trait `std::fmt::Display` is not implemented for `Vec<&str>`
  |     |
  |     required by a bound introduced by this call
  |
note: required by a bound in `imprimir`
 --> fig05_05.rs:4:16
  |
4 | fn imprimir<T: Display>(valor: T) {
  |                ^^^^^^^ required by this bound in `imprimir`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

`E0277` dit qu'une contrainte de trait n'a pas été remplie. La note te mène à la signature qui a introduit l'exigence. La correction dépend de l'intention : tu peux afficher avec `{:?}` si tu veux une représentation de débogage et que le type implémente `Debug` ; tu peux parcourir le vecteur et afficher chaque élément ; ou tu peux le convertir explicitement en `String` avec le format dont ton rapport a besoin. N'implémente pas `Display` pour un type étranger seulement pour faire taire l'erreur : la règle des orphelins l'empêchera et, de plus, ce serait une décision globale difficile à justifier.

## Ce qui se fait mal

### Créer un trait pour chaque struct

Un trait doit représenter une capacité partagée, pas répéter le nom d'un type. S'il n'existe qu'une seule implémentation et qu'il n'y a aucune raison concrète de l'échanger, une méthode inhérente est généralement plus claire. `impl Estado { fn esta_bien(...) }` exprime que l'opération appartient naturellement à `Estado`. Créer un trait `EstadoConsultable` pour une seule fonction ne fait qu'ajouter des noms et des fichiers sans séparer de vraie dépendance.

Commence par des structs, des enums et des fonctions directes. Extrais un trait quand plusieurs implémentations doivent remplir le même contrat, quand tu as besoin de recevoir une capacité au lieu d'un type concret ou quand une frontière de tests le justifie réellement.

### Ajouter des bounds « au cas où »

Il est courant de copier une signature comme `T: Clone + Debug + Display` et de conserver tous les bounds alors que le corps n'utilise que `Display`. Chaque bound limite les types qui peuvent appeler la fonction. De plus, chaque capacité promise par une signature devient une partie de l'API que les autres doivent comprendre.

Demande `Clone` seulement si le corps appelle `clone`, `Ord` seulement s'il trie et `Send` seulement s'il déplace des données vers un autre thread. Un petit bound est une abstraction plus flexible et décrit mieux le besoin réel. La figure 5.2 conserve `Clone` pour montrer qu'on peut combiner des contraintes, mais ce n'est pas un modèle à copier littéralement.

### Utiliser `Box<dyn Trait>` par habitude

Un trait object résout un vrai problème : stocker ou choisir des implémentations différentes à l'exécution. Ce n'est pas la façon obligatoire d'utiliser les traits. Si le type est connu à la compilation, un paramètre générique est normalement plus simple, évite des allocations inutiles dans le tas (heap) et permet le dispatch statique.

La question utile n'est pas « traits ou génériques ? ». Un trait décrit une capacité ; ensuite, tu choisis si tu la reçois avec des génériques, comme `impl Trait`, au moyen d'une référence `&dyn Trait` ou derrière un `Box<dyn Trait>`. Le choix dépend de la propriété, de la taille et du moment où tu connais le type.

### Cloner pour faire taire les erreurs du borrow checker

Si une référence ne vit pas assez longtemps, copier un `String` avec `.clone()` peut faire compiler le programme, mais ne résout pas toujours la bonne conception. Parfois, cela cache seulement qu'une fonction devrait renvoyer une référence, qu'un type devrait posséder ses données ou qu'un emprunt dure plus que nécessaire.

Fais d'abord le diagnostic : identifie le propriétaire, identifie qui a besoin d'utiliser la donnée ensuite et décide s'il a besoin d'une vue ou d'une copie indépendante. Clone quand deux propriétaires légitimes ont besoin de conserver des valeurs séparées. Le vecteur de lignes du `revisor` ne clone ni services ni états parce qu'il n'a besoin que de les trier pendant que leurs propriétaires sont encore vivants.

### Lire `'a` comme une durée concrète

`'a` ne signifie ni une seconde, ni une portée (scope) fixe, ni une variable créée au début du programme. C'est une étiquette que Rust remplace par une région valide à chaque appel. Deux fonctions peuvent utiliser le nom `'a` sans partager absolument rien ; le nom n'a de sens qu'à l'intérieur de sa propre signature.

C'est aussi une erreur de penser que plus d'annotations sont plus sûres. Les annotations doivent refléter d'où vient une référence. Si tu ne peux pas expliquer quelle référence d'entrée garantit la sortie, la fonction doit probablement renvoyer une valeur propre au lieu d'une référence.

## Exercices

### Exercice 1 — Un revisor factice avec nom par défaut

Définis un trait `Revisor` avec `revisar(&self, servicio: &Servicio) -> Estado` et une méthode par défaut `nombre() -> String`. Crée `RevisorFalso` qui renvoie `Estado::Ok { ms: 1 }` sans remplacer `nombre`. Vérifie qu'il affiche `revisor -> OK en 1ms`.

Ensuite, ajoute `RevisorArchivo`, qui remplace `nombre` par `"archivo"` et renvoie un échec déterministe. Explique en une phrase pourquoi les deux types peuvent être utilisés là où l'on attend un `Revisor`.

### Exercice 2 — Compter les résultats sains de manière générique

Utilise le trait de la figure 5.1 et écris une fonction `contar_sanos<R: Revisor>`. Elle doit recevoir un revisor et une slice de services, les vérifier et renvoyer combien d'états sont `Ok`. Teste la fonction avec trois services et `RevisorFalso`.

Avant de programmer, décide ce que chaque partie doit posséder : la fonction ne doit consommer ni le revisor ni le vecteur de services. Utilise `&R` et `&[Servicio]`, ne clone pas.

### Exercice 3 — Lire le lifetime du rapport

Ouvre `programas/revisor/src/reporte.rs` et repère `Fila<'a>` et `ordenadas<'a>`. Écris avec tes mots ce que possède le `Vec<Fila<'a>>`, ce qu'il emprunte et ce qui se passerait si tu essayais de renvoyer ces lignes après avoir détruit `servicios` ou `estados`.

Ensuite, écris une fonction `primero<'a>` qui reçoit `&'a str` et renvoie `&'a str`. Compare-la avec `devolver` de la figure 5.4 et explique pourquoi l'une compile et pas l'autre.

## Solutions

### Solution 1

L'implémentation qui accepte la méthode par défaut n'écrit pas `nombre` ; le trait en fournit le corps. La seconde implémentation le remplace, parce qu'elle a besoin d'une étiquette différente.

<!-- verificar:fragmento -->
```rust
trait Revisor {
    fn revisar(&self, servicio: &Servicio) -> Estado;

    fn nombre(&self) -> String {
        "revisor".to_string()
    }
}

struct RevisorFalso;
struct RevisorArchivo;

impl Revisor for RevisorFalso {
    fn revisar(&self, _servicio: &Servicio) -> Estado {
        Estado::Ok { ms: 1 }
    }
}

impl Revisor for RevisorArchivo {
    fn revisar(&self, servicio: &Servicio) -> Estado {
        Estado::Falla(format!("{}: no existe el archivo", servicio.nombre))
    }

    fn nombre(&self) -> String {
        "archivo".to_string()
    }
}
```

Les deux types peuvent être utilisés là où l'on attend un `Revisor` parce que les deux ont écrit `impl Revisor for ...` et fournissent la méthode obligatoire `revisar`. La différence entre leurs données et leur algorithme reste encapsulée à l'intérieur de chaque implémentation.

### Solution 2

La fonction prend des emprunts parce qu'elle n'a besoin que de consulter les données. Chaque résultat est temporaire : il est compté puis jeté. Il n'y a aucune raison de stocker un `Vec<Estado>` ni de cloner le revisor ou les services.

<!-- verificar:fragmento -->
```rust
fn contar_sanos<R: Revisor>(revisor: &R, servicios: &[Servicio]) -> usize {
    servicios
        .iter()
        .filter(|servicio| matches!(revisor.revisar(servicio), Estado::Ok { .. }))
        .count()
}
```

`iter()` produit `&Servicio` ; la closure reçoit chaque emprunt et appelle le trait au moyen de `&R`. `matches!` décide si l'état appartient à la variante `Ok`, et `count()` renvoie le total. Si `Estado` avait aussi `Lento`, tu devrais décider explicitement s'il compte comme sain ; le `revisor` réel répond à cette question avec `Estado::esta_bien()`.

### Solution 3

`Vec<Fila<'a>>` possède le vecteur et l'ordre de ses éléments, mais ne possède ni les services ni les états. Chaque élément contient deux références. C'est pourquoi ses lignes ne peuvent vivre que tant que les slices empruntées par `ordenadas` restent vivantes. Essayer de les renvoyer pour les utiliser après la destruction des collections d'origine produirait une erreur du borrow checker : ce seraient des références pendantes (dangling references).

La fonction correcte renvoie une référence qui appartient à l'appelant :

<!-- verificar:fragmento -->
```rust
fn primero<'a>(texto: &'a str) -> &'a str {
    texto
}
```

`primero` ne crée pas le texte et n'essaie pas de l'emprunter après l'avoir détruit. Elle renvoie seulement le même emprunt qu'elle a reçu. En revanche, `devolver` crée un `String` local, en est le propriétaire et le détruit en sortant ; c'est pourquoi la référence de la figure 5.4 ne peut pas s'échapper.

## Comment savoir que j'y suis arrivé

- `rustc --edition 2024 fig05_01.rs && ./fig05_01` affiche les deux lignes documentées, y compris l'étiquette par défaut de `RevisorHttp`.
- `rustc --edition 2024 fig05_02.rs && ./fig05_02` affiche `OK en 8ms` et `OK en 5ms`.
- `rustc --edition 2024 fig05_03.rs && ./fig05_03` affiche `catalogo`.
- `rustc --edition 2024 fig05_04.rs` échoue avec `error[E0515]` ; tu peux expliquer pourquoi changer `'a` ne répare pas l'emprunt local.
- `rustc --edition 2024 fig05_05.rs` échoue avec `error[E0277]` ; tu peux localiser à la fois l'appel incorrect et le bound qui l'a rejeté.
- Tu peux désigner `Fila<'a>` dans `programas/revisor/src/reporte.rs` et expliquer qu'il emprunte les services et les états au lieu de les cloner.
- Tu as terminé les exercices `generics`, `traits` et `lifetimes` de Rustlings.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 10.1 : syntaxe des types génériques](https://doc.rust-lang.org/book/ch10-01-syntax.html), consulté le 2 octobre 2026.
- [The Rust Programming Language, chapitre 10.2 : traits](https://doc.rust-lang.org/book/ch10-02-traits.html), consulté le 2 octobre 2026.
- [The Rust Programming Language, chapitre 10.3 : validation des références avec les lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), consulté le 2 octobre 2026.
- [Rustlings : exercices de génériques, traits et lifetimes](https://rustlings.rust-lang.org/), consulté le 2 octobre 2026.
