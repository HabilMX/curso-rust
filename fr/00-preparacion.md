# Leçon 0 — Installer Rust sur ton Linux Mint

**Durée :** 90 min.

**Ce que tu construis :** ton environnement et ton premier programme avec `cargo`.

**Ce que tu apprends :** pourquoi pas `apt install rustc`, `rustup`, `rustc` contre `cargo`, `cargo new/build/run/check`, le livre et Rustlings hors ligne, lire une erreur du compilateur.

## À la fin, tu vas pouvoir

- Installer Rust stable avec `rustup` et expliquer pourquoi il ne vaut pas la peine de dépendre de `apt install rustc`.
- Identifier si le terminal utilise les outils gérés par `rustup`.
- Distinguer le travail de `rustc` de celui de `cargo`, et savoir lequel utiliser dans chaque cas.
- Créer un projet avec `cargo new`, le compiler, l'exécuter et le vérifier avec `cargo check`.
- Ouvrir The Rust Book hors ligne et installer Rustlings pour t'entraîner en local.
- Lire en entier un diagnostic de `rustc`, repérer la ligne responsable et essayer la correction suggérée.

## Le pourquoi avant le comment

Cette leçon ne ressemble pas à une leçon de Rust, parce qu'elle n'enseigne encore ni la possession (ownership), ni les types, ni `match`. Pourtant, elle décide d'une part importante de la façon dont tu vas apprendre le langage : dès le premier jour, tu vas travailler avec la même chaîne d'outils, les mêmes conventions et le même type d'erreurs que dans un projet réel. Installer quelque chose qui « compile à peu près » suffit pour un exercice isolé ; installer le bon environnement est nécessaire pour suivre un cours, lire une documentation à jour et construire le `revisor` sans que l'outil devienne un problème de plus.

Le cours a été écrit et vérifié avec Rust stable 1.98.1 et l'édition 2024. C'est une référence concrète pour que les programmes, les messages et les exemples aient le même sens pour tout le monde ; avec une version stable plus récente, les programmes doivent se comporter de la même façon, même si le texte d'un message du compilateur peut changer de formulation. Rust publie une version stable environ toutes les six semaines. Linux Mint, lui, hérite une bonne partie de ses paquets d'Ubuntu, et une distribution LTS privilégie la stabilité du système : elle fige les versions majeures et applique des correctifs de sécurité. C'est une décision raisonnable pour les programmes du système ; ce n'est pas une bonne façon de suivre de près un langage dont l'écosystème, la documentation et les outils changent souvent.

C'est pourquoi `apt install rustc` semble fonctionner au début et peut semer la confusion ensuite. Il installe un compilateur appelé `rustc`, mais pas forcément le compilateur qu'utilisent The Rust Book, les exemples récents ou les projets que tu vas rencontrer. Le problème ne se manifeste pas toujours sous la forme « ta version est ancienne ». Parfois, il apparaît comme une fonctionnalité inconnue, une édition qui n'existe pas, une suggestion du compilateur différente ou une dépendance qui n'accepte plus cette version. C'est le pire type de panne de préparation : elle survient plus tard et ressemble à une erreur de ton programme.

Rust résout cela avec `rustup`. Ce n'est pas seulement un installeur : c'est le gestionnaire officiel des chaînes d'outils (toolchains) de Rust. Une *toolchain* rassemble une version de `rustc`, `cargo`, la bibliothèque standard, la documentation et des composants associés qui doivent fonctionner ensemble. `rustup` installe le canal stable et place ses exécutables à un endroit prévisible dans ton espace utilisateur. Quand il faudra mettre à jour, tu changes tout cet ensemble avec `rustup update stable`, sans mélanger des paquets du système ni télécharger des fichiers à la main.

La comparaison avec Go aide à situer la décision. En Go, tu as installé la distribution officielle parce que le paquet de la distribution Linux pouvait lui aussi rester figé ; Rust rend ce problème plus visible parce que son rythme de publication est plus court et parce que `cargo` intègre la compilation, les dépendances, les tests, le formatage et l'analyse statique. Dans les deux cours, on construit le même `revisor` : un outil qui lit une liste de services, les interroge et signale leur état. En Go, `go` concentre beaucoup de tâches. En Rust, `cargo` joue ce rôle autour de `rustc`. La différence ne change pas la discipline : on travaille à l'intérieur d'un projet, on compile avec un outil reproductible et on lit le diagnostic avant de modifier du code au hasard.

L'objectif n'est pas de mémoriser une liste de commandes. C'est de construire un modèle mental simple. `rustc` transforme un fichier Rust en code exécutable et signale les erreurs du langage. `cargo` comprend un projet entier : il connaît son nom, son édition, ses dépendances, ses tests, ses profils de compilation et la structure de ses fichiers ; ensuite, il appelle `rustc` avec les bons arguments. Au début, tu utiliseras `rustc` directement pour voir sans bruit ce que fait le compilateur. Au quotidien, tu utiliseras `cargo`, parce qu'un programme réel consiste rarement en un seul fichier sans dépendances.

L'autre décision importante de cette leçon concerne la façon de réagir à une erreur. Rust n'essaie pas de deviner ce que tu as voulu dire et ne laisse pas passer du code douteux pour échouer plus tard. Le compilateur arrête la compilation, signale à la fois l'origine et l'usage problématique et, dans beaucoup de cas, propose un changement concret. Cela ne veut pas dire que tous les messages sont faciles dès le premier jour ; cela veut dire qu'il vaut la peine de les lire en entier. En Rust, le compilateur fait partie du processus d'apprentissage. Les prochaines leçons te feront provoquer exprès des erreurs de possession, d'emprunt et de types, parce que tu comprendras davantage en diagnostiquant une vraie panne qu'en mémorisant une règle isolée.

## Les concepts

### `rustup` installe et gère la toolchain

L'installation recommandée sur Linux Mint est celle que publie le projet Rust lui-même :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Avant d'exécuter une commande qui télécharge et exécute un script, il vaut la peine de la comprendre. `curl` télécharge l'installeur ; `--proto '=https'` limite le téléchargement à HTTPS ; `--tlsv1.2` exige une connexion TLS moderne ; `-sSf` fait échouer la commande si le serveur renvoie une erreur ; et `| sh` passe le contenu téléchargé à l'interpréteur de commandes. C'est la méthode officielle, mais qu'elle soit officielle ne supprime pas la responsabilité de vérifier ce que tu exécutes.

Si tu préfères l'inspecter d'abord, télécharge le fichier, lis-le et exécute-le seulement après l'avoir vérifié :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
less rustup.sh
sh rustup.sh
```

L'installeur propose une installation par défaut. Choisis l'option `1`, qui installe le canal stable pour ta plateforme. Quand il a fini, ouvre un nouveau terminal. Si tu veux utiliser Rust dans le terminal actuel sans le fermer, charge le fichier d'environnement que l'installeur a configuré :

```bash
source "$HOME/.cargo/env"
```

L'emplacement important est `~/.cargo/bin`. C'est là que `rustup` place les programmes que tu invoques : `rustup`, `rustc`, `cargo`, `clippy-driver` et d'autres. Le terminal ne trouve que les commandes qui sont dans sa variable `PATH` ; c'est pourquoi une installation peut être correcte sur le disque et pourtant afficher `command not found`. L'installeur essaie de mettre à jour ta configuration de démarrage, mais chaque shell et chaque terminal peut lire des fichiers différents.

Vérifie l'installation avec ces commandes :

```bash
rustup show active-toolchain
rustc --version
cargo --version
type -a rustc
type -a cargo
```

La première indique la toolchain active. Les deux suivantes doivent afficher Rust 1.98.1 ou une version stable plus récente si tu as déjà mis ton environnement à jour. `type -a` est particulièrement utile si tu as déjà installé Rust avec `apt` : elle énumère toutes les correspondances que trouve le shell et permet de découvrir que tu exécutes un ancien binaire avant celui de `~/.cargo/bin`.

Ne confonds pas une mise à jour de Linux Mint avec une mise à jour de Rust. Pour mettre à jour la toolchain stable, utilise :

```bash
rustup update stable
```

Il n'est pas nécessaire de la lancer avant chaque commande ; il est en revanche bon de le faire périodiquement et avant de commencer une session après plusieurs semaines. Si tu es déjà à jour, `rustup` l'indiquera. L'intention n'est pas de courir après les numéros de version par sport : c'est de garder alignés le compilateur, la documentation, les exemples et les composants installés.

Dans le `revisor`, cette décision se reflète dès sa racine. Le projet déclare l'édition 2024 et Cargo construit tous ses modules avec la toolchain active. Il n'y a pas de « version de Rust » cachée par fichier : la configuration du paquet fixe la langue que parle le projet, et le fichier de verrouillage (`Cargo.lock`) fixe les versions concrètes de ses dépendances, pour qu'une compilation répétée résolve le même ensemble.

### Le `PATH` et le compilateur que tu exécutes réellement

Quand tu écris `rustc`, le shell ne cherche pas sur tout le disque. Il parcourt les dossiers listés dans `PATH`, dans l'ordre, et exécute la première correspondance. Cette règle explique deux diagnostics courants. S'il n'y a aucun `rustc` dans ces dossiers, `bash` affiche généralement :

```bash
rustc: command not found
```

S'il y en a bien un, mais qu'il provient d'une installation `apt` antérieure, la commande peut fonctionner et afficher une version inattendue. Celui-ci est plus trompeur : il ne ressemble pas à un problème d'installation, mais le cours serait compilé avec un outil différent de celui qui est attendu.

Commence par confirmer quel shell tu utilises et comment il a été lancé :

```bash
echo "$SHELL"
echo "$0"
printf '%s\n' "$PATH"
```

Dans une installation habituelle avec Bash, `~/.bashrc` est chargé pour les shells interactifs et `~/.profile` pour une session de connexion. Dans Zsh, le fichier équivalent pour les sessions interactives est généralement `~/.zshrc`. Le fichier que `rustup` crée, `~/.cargo/env`, ajoute `~/.cargo/bin` au `PATH`. Exécuter `source "$HOME/.cargo/env"` dans le terminal actuel est une vérification directe : si, après cela, `rustc --version` fonctionne, le problème venait de l'environnement du shell, pas du compilateur.

N'ajoute pas des chemins en double encore et encore sans vérifier ce qui s'est passé. Commence par utiliser `type -a rustc`. Si un chemin de `~/.cargo/bin` apparaît, `rustup` est disponible dans le `PATH`. Si un chemin comme `/usr/bin/rustc` apparaît avant, une installation du système a la priorité. Dans ce cas, identifie d'abord quels paquets sont installés et quel binaire le shell utilise ; n'essaie pas de réparer en copiant des exécutables ni en modifiant à la main des liens symboliques.

Le `revisor` ne dépend pas d'un chemin fixe du compilateur. C'est un avantage de travailler avec `cargo` : l'outil invoque le `rustc` de la toolchain active et conserve le résultat des compilations dans `target/`. C'est pourquoi un projet Cargo peut se construire de la même façon sur un autre ordinateur correctement installé, sans que le code contienne de chemins personnels ni de commandes propres à ta machine.

### `rustc` : le compilateur et le premier programme

`rustc` est le compilateur de Rust. Il reçoit du code source, vérifie qu'il respecte les règles du langage et, si tout est correct, produit un exécutable. Pour un petit fichier, il est utile de l'invoquer directement, parce que cela permet de voir la relation exacte entre la source, la compilation et le programme obtenu. Chaque figure de ce cours se compile ainsi, pour que la sortie documentée corresponde au code que tu es en train de lire.

**Fig. 0.1** | Le premier programme.

```rust
// fig00_01.rs
fn main() {
    println!("hola, ya tengo Rust");
}
```

```bash
$ rustc --edition 2024 fig00_01.rs && ./fig00_01
hola, ya tengo Rust
```

`fn main()` déclare la fonction par laquelle démarre un programme exécutable. Rust cherche précisément une fonction appelée `main` pour commencer. À l'intérieur, `println!` affiche du texte et ajoute un saut de ligne. Le signe `!` n'est pas décoratif : `println!` est une macro. Les macros génèrent ou transforment du code pendant la compilation ; pour l'instant, il suffit de retenir la convention selon laquelle les noms qui se terminent par `!` ne sont pas des fonctions ordinaires. The Rust Book revient sur ce sujet au chapitre 20.

L'indicateur `--edition 2024` sélectionne l'édition actuelle du langage. Une édition ne signifie pas que ton code se convertit automatiquement chaque année vers un autre langage ; c'est une manière pour Rust d'améliorer ses règles et sa syntaxe sans casser silencieusement les projets existants. Le projet `revisor` déclare cette même édition dans son manifeste. Les exemples du cours l'indiquent explicitement pour que la commande d'une figure ne dépende pas de l'édition par défaut d'une installation particulière.

Cet usage direct de `rustc` est délibérément réduit. Si tu avais deux fichiers, une bibliothèque, des tests, des dépendances externes, des options d'optimisation et plusieurs plateformes cibles, écrire à la main la bonne invocation du compilateur deviendrait fragile. C'est là qu'intervient `cargo`. La relation n'est pas une compétition entre deux programmes : Cargo organise, Rustc compile. Quand tu utilises `cargo build`, Cargo finit par appeler `rustc` pour toi avec les chemins, l'édition et les options dont le projet a besoin.

Le même point existe dans le `revisor` : le binaire a une fonction `main`, mais il ne se compile pas en isolant ce fichier avec `rustc src/main.rs`. Il importe des modules de la bibliothèque locale et des crates externes ; il a besoin du manifeste et de la structure que Cargo connaît. Les figures de cette leçon enseignent le mécanisme de compilation. Les leçons suivantes appliquent ce mécanisme à un programme composé.

### `cargo` : le projet avant le fichier

Crée ton premier projet dans un dossier de travail à toi :

```bash
mkdir -p "$HOME/w/curso-rust"
cd "$HOME/w/curso-rust"
cargo new hola
cd hola
```

`cargo new hola` crée un dossier appelé `hola`, un manifeste `Cargo.toml` et le fichier `src/main.rs`. Si Git est installé et que Cargo peut initialiser un dépôt, il prépare aussi Git et un `.gitignore` ; si Git n'est pas disponible, le projet reste valide. Le résultat minimal a cette structure :

```text
hola/
├── Cargo.toml
├── .gitignore
└── src/
    └── main.rs
```

`Cargo.toml` est le manifeste du paquet. Il écrit l'identité du projet, son édition, ses dépendances et quelques décisions de construction. Ce fichier n'est pas un détail administratif : c'est lui qui permet à une autre personne d'exécuter le même `cargo build` sans reconstruire à la main la liste des arguments du compilateur. Quand Cargo résout les dépendances, il crée en plus `Cargo.lock` ; ce fichier enregistre la résolution concrète pour que les compilations soient reproductibles.

Le `revisor` est déjà un projet Cargo complet. Voici son vrai manifeste :

<!-- verificar:extracto:Cargo.toml -->
```toml
[package]
name = "revisor"
version = "0.1.0"
edition = "2024"
description = "El revisor del curso de Rust: consulta una lista de servicios a la vez y reporta cuáles responden."

[dependencies]
anyhow = "1.0.104"
clap = { version = "4.6.7", features = ["derive"] }
futures = "0.3.34"
reqwest = { version = "0.13.5", features = ["json"] }
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
yaml_serde = "0.10.7"
tokio = { version = "1.53.1", features = ["full"] }

[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila

[dev-dependencies]
serde_json = "1.0.151"
```

Tu n'as pas encore besoin de comprendre les dépendances ni le profil de publication. L'important est de reconnaître la forme : `[package]` décrit le paquet ; `[dependencies]` énumère ce dont il a besoin pour compiler ; `[profile.release]` change la façon dont sera construit le binaire final. À la leçon 4, tu découvriras les dépendances d'erreurs et de sérialisation, à la 6 l'organisation des modules et des tests, et à la 8 le profil de publication. Aujourd'hui, il suffit de comprendre pourquoi un projet a besoin d'un manifeste et pourquoi il ne vaut pas la peine de remplacer Cargo par une longue commande `rustc` écrite à la main.

Exécute le projet que tu viens de créer avec :

```bash
cargo run
```

La première fois, Cargo compile le paquet puis exécute le binaire. Aux exécutions suivantes, il réutilise les artefacts qui n'ont pas changé. À la différence de `rustc fig00_01.rs`, tu n'as pas à écrire le nom du fichier ni celui de l'exécutable : Cargo connaît la convention `src/main.rs` et sait que le paquet `hola` produit le binaire `hola`.

Cette convention réduit les décisions répétitives. Un programme Rust peut s'organiser de plusieurs façons, mais Cargo fournit une structure commune pour les cas fréquents. Le projet que tu as créé aujourd'hui ne contient que `src/main.rs`, le binaire. Quand tu ouvriras le `revisor`, tu reconnaîtras en plus `src/lib.rs`, la bibliothèque du paquet. Cette séparation ne s'invente pas à la leçon 6 : Cargo la reconnaît par convention, comme il reconnaît `src/main.rs`.

### `cargo build`, `run`, `check` et le cycle de travail

Les commandes de Cargo ne sont pas des synonymes. Chacune répond à une question différente que tu te poses pendant que tu travailles :

```bash
cargo run
cargo build
cargo build --release
cargo check
cargo test
cargo clippy
cargo fmt
```

`cargo run` répond à « mon programme compile-t-il, et que fait-il ? ». Il construit d'abord le nécessaire puis exécute le binaire. C'est la commande que tu utiliseras quand tu changes une sortie, que tu testes une branche du programme ou que tu veux observer un comportement. Pour le projet `hola`, elle doit afficher le message qui se trouve dans `src/main.rs`.

`cargo build` répond à « puis-je produire le binaire ? ». Il compile le paquet, mais ne l'exécute pas. En mode développement, il dépose les artefacts dans `target/debug/` ; tu n'as pas besoin d'apprendre ce chemin par cœur, mais il est utile de savoir que Cargo ne remplit pas le dossier racine du projet d'exécutables et de fichiers intermédiaires. Cela garde séparés le code source et les résultats de compilation.

`cargo build --release` produit le profil de publication, normalement avec plus d'optimisation et avec ses résultats dans `target/release/`. La différence compte quand tu voudras livrer le `revisor` terminé ou mesurer ses performances. Ne compare pas les temps d'exécution d'un binaire de développement pour en tirer des conclusions sur les performances de Rust : le profil de développement privilégie une compilation rapide et un débogage confortable ; le profil de publication privilégie le programme obtenu. Le `Cargo.toml` du `revisor` montre que ce profil peut même régler la taille, l'optimisation entre modules et le comportement face à `panic!`.

`cargo check` répond à « le compilateur accepte-t-il mon code ? ». Il vérifie les types, les emprunts, les modules et une grande partie du travail de compilation, mais ne termine pas la génération d'un binaire exécutable. Dans un projet qui grandit, il peut faire gagner du temps. Utilise-le pendant que tu écris, surtout quand tu veux seulement savoir si une modification est valide ; utilise `cargo run` quand tu veux en plus exécuter le comportement. Aucune des deux commandes ne remplace l'autre : l'une vérifie rapidement et l'autre vérifie en plus le résultat à l'exécution.

`cargo test` compile et lance les tests. Tu n'as pas encore écrit de tests dans cette leçon, mais tu commenceras à les voir à la leçon 6. `cargo clippy` exécute le linter officiel et te signale des motifs qui compilent mais qui sont souvent confus, inefficaces ou peu idiomatiques. `cargo fmt` applique le format standard de Rust. C'est la même discipline que `gofmt` en Go : on ne perd pas de temps à discuter de l'alignement de chaque fichier, on laisse l'outil donner une réponse uniforme.

Le `revisor` utilise exactement ce cycle. Avant de publier un changement, il convient d'exécuter, depuis `programas/revisor/`, ces commandes :

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

La première vérifie le comportement ; la deuxième transforme les avertissements de Clippy en échecs pour ne pas les ignorer ; la troisième confirme que le code a déjà le format attendu. Tu ne dois pas encore les exécuter pour « comprendre » leur sortie. Garde-les comme référence de la routine à laquelle tu arriveras en construisant le programme.

### Immuabilité par défaut et première conversation avec `rustc`

Rust considère comme immuable une variable créée avec `let`, sauf si tu écris `mut`. Ce n'est pas un obstacle placé pour allonger le code. C'est une déclaration visible d'intention : si une valeur doit changer, le lecteur et le compilateur doivent pouvoir le voir à l'endroit où la variable est définie.

**Fig. 0.2** | Un programme qui ne compile pas.

```rust
// fig00_02.rs
fn main() {
    let x = 5;
    x = 6;
    println!("{x}");
}
```

```bash
$ rustc --edition 2024 fig00_02.rs
error[E0384]: cannot assign twice to immutable variable `x`
 --> fig00_02.rs:4:5
  |
3 |     let x = 5;
  |         - first assignment to `x`
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
3 |     let mut x = 5;
  |         +++

warning: value assigned to `x` is never read
 --> fig00_02.rs:3:13
  |
3 |     let x = 5;
  |             ^ this value is reassigned later and never used
4 |     x = 6;
  |     ----- `x` is overwritten here before the previous value is read
  |
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0384`.
```

La solution suggérée par le compilateur est correcte si tu veux réellement changer la valeur. Le mot `mut` s'écrit dans la déclaration, pas dans l'affectation ultérieure. Ainsi, celui qui lit le bloc sait dès le départ que `x` fait partie de l'état mutable de cette fonction.

**Fig. 0.3** | La même variable, maintenant mutable.

```rust
// fig00_03.rs
fn main() {
    let mut x = 5;      // mut = mutable
    println!("{x}");
    x = 6;              // ahora sí
    println!("{x}");
}
```

```bash
$ rustc --edition 2024 fig00_03.rs && ./fig00_03
5
6
```

L'immuabilité par défaut aide à réduire les changements accidentels et prépare le terrain pour la possession et les emprunts. Dans d'autres langages, il est courant de modifier une variable parce que le langage le permet, et de se demander seulement après qui dépendait de son ancienne valeur. Rust demande de déclarer cette possibilité dès le départ. Il n'élimine pas toutes les erreurs, mais il transforme une supposition implicite en une propriété vérifiable.

Dans le `revisor`, il y a de la mutabilité seulement là où l'opération l'exige réellement. La fonction qui trie les lignes crée un vecteur puis le trie sur place ; c'est pourquoi la variable est déclarée `mut` :

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

On n'écrit pas `mut` par habitude. `filas.sort_by(...)` modifie le vecteur, donc la déclaration le communique. En revanche, `servicios` et `estados` sont des références que cette fonction se contente de consulter ; elles ne sont pas déclarées mutables. Cette différence, petite dans cette fonction, devient importante quand plusieurs parties d'un programme essaient d'utiliser les mêmes données. La leçon 2 explique les règles que Rust applique pour que ces usages soient sûrs.

### The Rust Book et Rustlings comme pratique locale

The Rust Book est le texte officiel de référence du cours. `rustup` installe une copie locale de la documentation, donc tu peux l'ouvrir hors ligne une fois l'installation terminée :

```bash
rustup doc --book
```

Cette commande ouvre le chapitre initial dans le navigateur par défaut. Si tu n'as pas d'environnement graphique ou si tu préfères repérer d'autres documents, `rustup doc --help` montre les options disponibles. La copie locale ne remplace pas les mises à jour : si tu mets à jour la toolchain, la documentation locale se met à jour avec elle. C'est une autre raison de garder ensemble le compilateur et la documentation.

Lis en entier le chapitre 1 de The Rust Book : installation, « Hello, world! » et « Hello, Cargo! ». Ne le saute pas sous prétexte que tu as déjà créé un projet. Ce que tu as fait ici te donne le contexte pour le lire plus vite ; le chapitre ordonne les concepts et explique les conventions qui reviendront pendant tout le cours. Le chapitre 2, qui comprend le jeu de devinette, correspond à la leçon 1, avec les fondamentaux du chapitre 3.

Rustlings complète le livre avec de petits exercices qui se résolvent en modifiant des fichiers locaux. Son installation initiale a besoin d'un accès au réseau pour que Cargo télécharge le programme ; ensuite, le répertoire des exercices et leur code vivent sur ton ordinateur. Installe-le et initialise-le ainsi :

```bash
cargo install rustlings
rustlings init
cd rustlings
rustlings
```

La commande interactive surveille les exercices, t'indique lequel échoue et les revérifie à mesure que tu enregistres tes changements. N'utilise pas Rustlings comme une collection de réponses à cocher. L'ordre du cours est voulu : lis d'abord le chapitre de The Rust Book, résous ensuite les exercices associés, puis applique l'idée au `revisor`. À ce stade, installe Rustlings et familiarise-toi avec son répertoire ; à la leçon 1, tu travailleras ses sections `variables`, `functions`, `if` et `primitive_types`.

Le livre et les exercices remplissent des fonctions différentes. The Rust Book explique le modèle et nomme ses pièces ; Rustlings t'oblige à toucher le code et à recevoir une erreur concrète. Le `revisor` est le problème d'intégration : ce n'est pas un exercice isolé, mais le même programme que tu as déjà construit en Go, maintenant avec les décisions de Rust. Les trois sources se soutiennent mutuellement. Si une explication te paraît abstraite, essaie un exercice ; si un exercice te semble mécanique, retourne au chapitre ; si les deux sont déjà clairs, repère le motif dans le projet réel.

## L'erreur que tu vas voir

L'erreur centrale de cette leçon est `E0384`, montrée en entier dans la Fig. 0.2. Ne la lis pas comme un mur de texte. Lis-la dans l'ordre. La première ligne nomme le code de diagnostic, `E0384`, et résume le problème : tu ne peux pas affecter deux fois une variable immuable. Ce code sert à demander une explication détaillée au compilateur :

```bash
rustc --explain E0384
```

La ligne qui commence par `--> fig00_02.rs:4:5` localise la tentative d'affectation : fichier, ligne et colonne. Les lignes numérotées montrent assez de contexte pour ne pas avoir à chercher à l'aveugle. La marque `^^^^^` désigne la partie exacte qui provoque l'erreur. Avant elle apparaît la première affectation, à la ligne 3, parce que Rust ne se contente pas d'indiquer où il a détecté le problème : il montre aussi l'origine de la condition qui le rend invalide.

La section `help:` mérite une attention particulière. Dans ce cas, elle propose de changer `let x = 5;` en `let mut x = 5;`, et les marques `+++` indiquent quel texte ajouter. N'applique pas toutes les suggestions mécaniquement. Vérifie d'abord l'intention : si `x` ne devrait pas changer, la bonne solution n'est pas d'ajouter `mut`, mais de supprimer ou de repenser l'affectation ultérieure. Le compilateur peut proposer une correction locale ; c'est à toi de décider si cette correction représente la bonne conception.

Le même diagnostic comprend aussi un avertissement : la première valeur, `5`, n'est jamais lue parce qu'elle est écrasée immédiatement. L'avertissement n'empêche pas de compiler en soi, mais il apporte une information utile. Si tu rends `x` mutable sans regarder l'avertissement, le programme gardera une affectation inutile. La version de la Fig. 0.3 affiche `5` avant de le changer, donc les deux valeurs ont un sens et le programme ne génère aucun avertissement.

Quand tu rencontres la même erreur dans un projet avec `cargo run` ou `cargo check`, Cargo montre le diagnostic de `rustc` avec le contexte du paquet et le chemin `src/main.rs`. La règle ne change pas : commence par la première erreur, lis ses notes et son aide, corrige une cause à la fois et recompile. Une erreur initiale peut en provoquer plusieurs autres ; essayer de les corriger toutes d'un coup cache souvent la cause réelle.

Il existe d'autres erreurs de préparation qui ne sont pas des codes de Rust, parce qu'elles surviennent avant que le compilateur puisse analyser ton programme. Si `cargo` ou `rustc` n'existent pas pour le terminal, le problème est le `PATH` ; recharge `~/.cargo/env` et vérifie `type -a cargo`. Si la compilation arrive jusqu'à l'édition de liens et qu'un message du genre `error: linker 'cc' not found` apparaît, il manque le compilateur C que Rust utilise pour lier sur Linux Mint. Installe le paquet d'outils de construction de la distribution et relance la commande :

```bash
sudo apt install build-essential
```

Ne confonds pas ce cas avec « Rust n'est pas installé ». `rustc --version` peut fonctionner parfaitement ; la panne apparaît ensuite, quand le compilateur doit convertir des objets compilés en un exécutable du système. Séparer l'étape qui échoue évite les corrections au hasard.

## Ce qui se fait mal

- Installer Rust avec `apt install rustc` et tenir pour acquis que le nom du paquet garantit une toolchain à jour. Le problème n'est pas que le paquet soit inutile ; c'est qu'il suit le calendrier de la distribution, pas celui de Rust. Pour ce cours, utilise `rustup`, vérifie `rustc --version` et mets à jour périodiquement le canal stable.

- Mélanger une installation `apt` avec une autre de `rustup` sans vérifier laquelle l'emporte dans le `PATH`. Avoir deux exécutables appelés `rustc` ne produit pas forcément une erreur immédiate. Tu peux compiler pendant des jours avec la mauvaise version. Utilise `type -a rustc` et `type -a cargo` avant de modifier des fichiers de démarrage ou de supprimer des paquets.

- Utiliser `rustc` pour un projet entier par habitude. Pour une figure d'un seul fichier, c'est un excellent outil pédagogique. Pour le `revisor`, cela reviendrait à reconstruire à la main les dépendances, les chemins, l'édition, les modules et les profils. Utilise `cargo` depuis la racine du projet ; laisse-le construire l'invocation de `rustc`.

- Utiliser `cargo run` chaque fois que tu veux savoir si le code compile. Ça marche, mais il construit et exécute même si tu es seulement en train de corriger des types ou des emprunts. Pendant l'édition rapide, `cargo check` donne un retour plus direct. Quand tu as besoin d'observer le comportement, utilise `cargo run`.

- Mesurer les performances avec un binaire de développement. `cargo build` et `cargo run` utilisent par défaut le profil de développement. Quand le cours en viendra à comparer la taille, la vitesse ou la livraison du binaire, utilise `cargo build --release`. Sans cette distinction, une mesure en dit plus sur le profil choisi que sur le programme.

- Ignorer un avertissement parce qu'il « n'empêche pas de compiler ». Les avertissements signalent souvent des valeurs inutilisées, du code mort ou des constructions confuses. Ce cours compile les figures correctes avec les avertissements traités comme des erreurs, pour que la sortie affichée ne cache aucun problème. Fais de même dans ta routine : comprends l'avertissement ou supprime sa cause.

- Lire seulement la première ligne d'une erreur. La première ligne nomme la catégorie ; les lignes suivantes disent où cela s'est produit, quelle valeur antérieure l'explique, quelles notes s'appliquent et quelle alternative envisage le compilateur. Ne copier que « error E0384 » pour le chercher fait perdre une bonne part de la réponse qui est déjà sous tes yeux.

- Installer Rustlings et résoudre les exercices avec des réponses copiées. Un exercice terminé sans comprendre le diagnostic ne construit pas le modèle mental dont tu auras besoin pour la possession. Fais de petits changements, lance le vérificateur, lis l'erreur et explique avec tes propres mots pourquoi la solution compile.

## Exercices

### Exercice 1 — Vérifie ta toolchain

Installe Rust avec `rustup` si tu ne l'as pas encore. Exécute `rustup show active-toolchain`, `rustc --version`, `cargo --version` et `type -a rustc`. Note quel chemin le terminal utilise pour `rustc` et confirme qu'il correspond à `~/.cargo/bin` quand tu utilises l'installation gérée par `rustup`.

### Exercice 2 — Crée et parcours un projet Cargo

Dans un dossier de travail, exécute `cargo new saludo-rust` et entre dans le répertoire créé. Lis `Cargo.toml` et `src/main.rs` avant de les modifier. Change le message pour un message à toi et lance, dans cet ordre, `cargo check`, `cargo build` et `cargo run`. Explique à quelle question chaque commande a répondu et quel fichier ou quel résultat tu attendais de chacune.

### Exercice 3 — Provoque et explique `E0384`

Remplace temporairement le contenu de `src/main.rs` par le programme de la Fig. 0.2 et exécute `cargo check`. Ne corrige rien tant que tu n'as pas identifié le fichier, la ligne, la première affectation et la suggestion marquée `help:`. Ensuite, change la déclaration en `let mut x = 5;`, observe l'avertissement restant et modifie le programme pour que les deux valeurs soient lues, comme dans la Fig. 0.3.

### Exercice 4 — Prépare la lecture et la pratique locale

Ouvre The Rust Book avec `rustup doc --book` et lis entièrement le chapitre 1. Installe Rustlings avec `cargo install rustlings`, lance-le dans son répertoire local et repère comment rouvrir les exercices sans dépendre d'une page web. Écris une courte note qui distingue ce que tu obtiens du livre, ce que tu obtiens de Rustlings et ce que tu construiras ensuite dans le `revisor`.

## Solutions

### Solution 1

Une installation correcte montre une toolchain stable active et permet d'exécuter aussi bien `rustc --version` que `cargo --version`. La sortie exacte peut changer quand tu mets Rust à jour, mais les deux outils doivent appartenir à la même installation stable. `type -a rustc` doit lister `~/.cargo/bin/rustc` comme chemin retenu ou, au moins, te permettre d'expliquer pourquoi un autre chemin a la priorité. S'il n'apparaît pas, exécute `source "$HOME/.cargo/env"` et vérifie à nouveau.

### Solution 2

`cargo check` vérifie le projet sans terminer la production d'un exécutable ; `cargo build` compile le paquet et laisse des artefacts de développement dans `target/debug/` ; `cargo run` compile le nécessaire et exécute le binaire. Les trois commandes doivent accepter le projet `saludo-rust`. La sortie de `cargo run` doit être exactement le message que tu as laissé dans `src/main.rs`.

### Solution 3

`cargo check` montre `E0384` parce que `let x = 5;` crée une liaison immuable et que la ligne suivante essaie de la réaffecter. La changer en `let mut x = 5;` permet la réaffectation, mais laisse d'abord un avertissement, parce que `5` est écrasé sans être utilisé. Afficher `x` avant et après l'affectation élimine l'avertissement et produit les deux lignes de la Fig. 0.3. Ce qu'il faut retenir n'est pas « ajoute toujours `mut` » ; c'est de déclarer la mutabilité seulement quand la modification fait partie de la conception.

### Solution 4

The Rust Book présente l'explication ordonnée de l'installation, du programme initial et de Cargo ; son chapitre 1 est disponible en local avec `rustup doc --book`. Rustlings apporte des exercices modifiables et un retour sur du code local une fois installé et son répertoire initialisé. Le `revisor` est l'endroit où ces pièces se combinent dans une application : il ne remplace ni le livre ni les exercices, mais il offre un problème continu sur lequel appliquer les concepts des leçons suivantes.

## Comment savoir que j'y suis arrivé

- [ ] `rustc --version` et `cargo --version` fonctionnent et montrent une toolchain stable compatible.
- [ ] `type -a rustc` me permet d'identifier quel compilateur mon terminal exécute.
- [ ] `rustup update stable` se termine sans erreur et je peux expliquer ce qu'il met à jour.
- [ ] `cargo new saludo-rust` a créé un projet avec `Cargo.toml` et `src/main.rs`.
- [ ] `cargo check`, `cargo build` et `cargo run` fonctionnent dans ce projet, et je sais ce que fait chacun.
- [ ] Mon programme affiche le message que j'ai écrit quand j'exécute `cargo run`.
- [ ] Je peux provoquer `E0384`, désigner sa ligne d'origine, lire son aide et corriger le programme sans laisser d'avertissements.
- [ ] `rustup doc --book` ouvre The Rust Book en local et Rustlings est initialisé dans un dossier local.

## Pour aller plus loin

- [The Rust Programming Language, chapitre 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html) — installation, premier programme et Cargo. Consulté le 2 octobre 2026.

- [Rust: Install](https://www.rust-lang.org/tools/install) — installation officielle avec `rustup`, mise à jour des toolchains et notes sur le `PATH`. Consulté le 2 octobre 2026.

- [The Cargo Book: Why Cargo Exists](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html) — pourquoi Cargo gère les paquets, les dépendances et les invocations de `rustc`. Consulté le 2 octobre 2026.

- [Rustlings](https://rustlings.rust-lang.org/) — installation, initialisation et usage des exercices locaux en parallèle avec The Rust Book. Consulté le 2 octobre 2026.
