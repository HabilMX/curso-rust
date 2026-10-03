# Cours de Rust — le deuxième, après Go

**Par Dorian Chávez, fondateur de Hábil et architecte d'intégration.**

**Pour qui :** quelqu'un qui a déjà terminé le **[cours de Go](https://github.com/HabilMX/curso-go)** et veut comprendre l'autre
extrémité du spectre. On n'apprend pas Rust *à la place de* Go : on l'apprend **après**, et c'est ainsi qu'on comprend
quelle décision chacun a prise.

**Fais d'abord celui de Go.** Ce n'est pas un caprice : ce cours tient pour acquis les concepts de base —variables,
fonctions, structs, listes, erreurs— et se consacre à ce que Rust fait **différemment**. Commencer par ici reviendrait à
apprendre deux choses difficiles à la fois.

**Ce dont tu as besoin :** un ordinateur avec Linux Mint et avoir terminé le cours de Go. La
[leçon 0](00-preparacion.md) installe Rust depuis zéro.

**Rust sort une nouvelle version toutes les six semaines.** Avant chaque session : `rustup update stable`. Et si un
tutoriel ne dit pas pour quelle version il est écrit, méfie-toi : en Rust, neuf mois représentent **six versions** d'écart,
et cela se remarque.

## Tu vas écrire le MÊME programme

Le `revisor` encore une fois : il reçoit une liste de services, les interroge **tous à la fois**, produit un rapport.

**Écrire deux fois le même programme est la méthode.** Lire des comparaisons « Go vs Rust » n'apprend rien ;
se battre avec le même problème dans les deux langages, si. Et tu vas découvrir que ce qui t'a pris une
après-midi en Go t'en prend trois en Rust — jusqu'à ce que tu comprennes *pourquoi*, et alors tu comprends les deux choses.

## Ce que personne ne te dit et qui définit ce cours

**Rust a une courbe différente, pas plus longue : différente.** La syntaxe est facile. Ce qui coûte, c'est **le
`borrow checker`**, le composant du compilateur qui vérifie qui est propriétaire de chaque donnée. Voici ce
que racontent habituellement ceux qui enseignent et ceux qui apprennent Rust, comme ordre de grandeur et sans prétendre à une mesure :

- **Le borrow checker devient généralement intuitif après plusieurs semaines de pratique.** Pas avant. Ce n'est pas que tu sois lent : c'est que
  ce modèle mental se construit en se cognant.
- Beaucoup de personnes consacrent **plusieurs semaines** aux bases avant leur premier vrai projet.
- **Le compilateur de Rust est le meilleur professeur qui existe.** Ses erreurs expliquent le problème, désignent la
  ligne et **suggèrent la correction**. En Rust, on apprend en lisant les erreurs, pas en les évitant.

**Et voici la différence la plus importante avec le cours de Go :** en Go, la bibliothèque standard suffit
à presque tout. En Rust, **non** : async, HTTP et sérialisation vivent dans des *crates* externes (`tokio`, `reqwest`,
`serde`). Ce n'est pas un manque — c'est la décision que le standard soit minimal et stable. Mais cela signifie
qu'ici tu vas bien utiliser des dépendances dès le début.

## Les neuf leçons

**La méthode est celle qui fonctionne, mesurée : lire un chapitre de The Book, faire ses exercices de Rustlings,
et seulement ensuite continuer.** Lire d'une traite retient beaucoup moins, même si cela prend le même temps.

| | Leçon | The Book | Ce que tu construis |
|---|---|---|---|
| 0 | [Préparation](00-preparacion.md) | chap. 1 | `rustup`, `cargo` et Rustlings |
| 1 | [Fondamentaux](01-fundamentos.md) | chap. 2-3 | types, `mut`, contrôle de flux |
| 2 | [**Possession (ownership)**](02-ownership.md) | **chap. 4** | **la leçon qui décide de tout** |
| 3 | [Structs, enums et match](03-structs-enums.md) | chap. 5-6 | le modèle du `revisor`, avec `Option` |
| 4 | [Collections et erreurs](04-colecciones-errores.md) | chap. 8-9 | `Vec`, `HashMap`, `Result`, `?` |
| 5 | [Traits, génériques et lifetimes](05-traits-genericos.md) | chap. 10 | le trait `Revisor` et les génériques, et le `'a` qui fait peur |
| 6 | [Modules, tests et cargo](06-modulos-pruebas.md) | chap. 7, 11, 14 | le vrai projet |
| 7 | [Concurrence et async](07-concurrencia-async.md) | chap. 16-17 | **qu'il vérifie tout à la fois** |
| 8 | [Le programme terminé](08-el-programa.md) | chap. 12 | `reqwest`, `serde`, `clap`, le binaire |

**La possession (ownership) est à la leçon 2 et ne peut pas être déplacée.** C'est le chapitre 4 de The Book pour une raison : tout le
reste —structs, collections, erreurs, concurrence— s'explique en termes de qui est propriétaire de quoi. Si
tu la sautes, les six semaines suivantes consistent à mémoriser des règles sans queue ni tête.

## Les trois sources, et comment les combiner

1. **[The Book](https://doc.rust-lang.org/book/)** — officiel, 21 chapitres. Tu l'as **hors ligne** :
   `rustup doc --book`. Pour ce cours : **chapitres 1-12, 14 et 16-17**.
2. **[Rustlings](https://rustlings.rust-lang.org/)** — **95 exercices interactifs**, beaucoup visant
   spécifiquement le borrow checker. `cargo install rustlings`. **L'ordre compte : un chapitre, puis ses
   exercices.**
3. **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** — comme référence rapide.

**Ensuite, quand tu écris déjà du Rust qui fonctionne :** *Rust for Rustaceans* (Jon Gjengset) est le livre vers
lequel se tournent les ingénieurs qui ont déjà passé cette étape. Pas avant : il ne sert pas d'introduction.

## Comment l'utiliser

- **90 minutes par leçon au minimum.** Rust demande plus de temps assis que Go ; avec 60, cela ne suffit pas.
- **Lis les erreurs du compilateur en entier.** Pas seulement la première ligne. Rust te dit le problème, la
  cause et souvent la solution exacte. L'ignorer, c'est jeter le meilleur outil que tu aies.
- **`cargo clippy` dès le premier jour.** C'est le linter officiel et il enseigne du Rust idiomatique pendant que tu
  travailles.
- **[`programas/`](../programas/)** — tous les programmes des leçons, prêts à compiler et à exécuter, et le
  `revisor` complet avec ses tests. Chacun est vérifié automatiquement à chaque changement.
- **[`bitacora.md`](bitacora.md)** — ici cela compte plus qu'en Go : note **chaque combat avec le borrow
  checker**. Quand, à la leçon 5, tu les relis, tu verras que les premiers te paraissent déjà évidents. C'est
  le moment où l'apprentissage a eu lieu.
