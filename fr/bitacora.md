# Bitácora

Ici, cela compte **plus** que dans le cours de Go, et pour une raison concrète : **le borrow checker s'apprend
en se battant avec lui.** Ce qui te paraît hostile aujourd'hui te paraîtra évident dans trois semaines — mais tu ne
remarqueras ce changement que si tu l'as laissé par écrit.

---

## Mes combats avec le borrow checker

_Chaque fois que le compilateur te bloque et que tu ne comprends pas pourquoi : colle l'erreur, puis explique comment tu l'as résolue._

| # | Date | L'erreur (code EXXXX) | Ce que je tentais | Comment je l'ai résolue |
|---|---|---|---|---|
| 1 | | | | |
| 2 | | | | |
| 3 | | | | |
| 4 | | | | |
| 5 | | | | |

Attention : **relis ce tableau en arrivant à la semaine 5.** Si les trois premières te paraissent déjà triviales, le modèle
mental s'est construit. C'est le seul indicateur qui vaille.

---

## Semaine 0 — Préparation

_date :_ · Doutes :

## Semaine 1 — Fondamentaux

_date :_ · Le point-virgule en trop m'a-t-il mordu ?

## Semaine 2 — Possession (ownership)

_date :_

- Combien de fois ai-je voulu écrire `.clone()` pour faire taire une erreur ?
- Ce qui m'a le plus coûté à comprendre :
- Le moment où le déclic s'est produit (s'il a eu lieu) :

## Semaine 3 — Structs, enums et match

_date :_

- En ajoutant une variante, combien de `match` le compilateur m'a-t-il signalés ?
- Que m'a semblé `Option` comparé au `nil` de Go ?

## Semaine 4 — Collections et erreurs

_date :_

- Lignes de gestion d'erreurs en Go : ____ · en Rust : ____
- Laquelle je préfère, et pourquoi ?

## Semaine 5 — Traits, génériques et lifetimes

_date :_

- Les lifetimes ont-ils été aussi pénibles que je le craignais ?
- Relecture de mes combats de la semaine 2 : me paraissent-ils déjà évidents ?

## Semaine 6 — Modules, tests et cargo

_date :_

- Les trois suggestions de `clippy` qui m'ont le plus appris :
  1.
  2.
  3.

## Semaine 7 — Concurrence et async

_date :_

- Temps : threads ____ · async ____
- L'erreur que le compilateur m'a empêché de commettre :
- Est-ce que je comprends pourquoi le Mutex de Rust enveloppe la donnée ?

## Semaine 8 — Le programme terminé

_date :_

### Le tableau qui clôt les deux cours

| | Go | Rust |
|---|---|---|
| Lignes de code | | |
| Dépendances externes | | |
| Compilation à froid | | |
| Taille du binaire | | |
| Mémoire à l'exécution | | |
| Temps que ça m'a pris | | |
| Combats avec le compilateur | | |

**Lequel pour un outil qu'il faut avoir vendredi ?**

**Lequel pour la production, trois ans sans y toucher ?**

**Lequel m'a fait mieux comprendre ce que je faisais ?**

---

## Ce que je veux construire maintenant

-
