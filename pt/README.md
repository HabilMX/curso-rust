# Curso de Rust — o segundo, depois de Go

**Por Dorian Chávez, fundador da Hábil e arquiteto de integração.**

**Para quem é:** alguém que já terminou o **[curso de Go](https://github.com/HabilMX/curso-go)** e quer entender a outra
ponta do espectro. Não se aprende Rust *em vez de* Go: aprende-se **depois**, e com isso se entende
que decisão cada um tomou.

**Faça o de Go primeiro.** Não é capricho: este curso dá por sabidos os conceitos básicos —variáveis,
funções, structs, listas, erros— e se dedica ao que o Rust faz **diferente**. Começar por aqui seria
aprender duas coisas difíceis ao mesmo tempo.

**O que você precisa:** um computador com Linux Mint e ter terminado o curso de Go. A
[lição 0](00-preparacion.md) instala o Rust do zero.

**Atenção: O Rust lança uma versão nova a cada seis semanas.** Antes de cada sessão: `rustup update stable`. E se um
tutorial não diz para qual versão foi escrito, desconfie: em Rust nove meses são **seis versões** de
diferença, e isso se nota.

## Você vai escrever o MESMO programa

O `revisor` outra vez: recebe uma lista de serviços, consulta **todos ao mesmo tempo**, produz um relatório.

**Escrever o mesmo programa duas vezes é o método.** Ler comparações «Go vs Rust» não ensina nada;
brigar com o mesmo problema nas duas linguagens, sim. E você vai descobrir que o que em Go levou uma
tarde, em Rust leva três — até você entender *por quê*, e então entende as duas coisas.

## O que ninguém conta e define este curso

**O Rust tem uma curva diferente, não mais longa: diferente.** A sintaxe é fácil. O que custa é **o
`borrow checker`**, o componente do compilador que verifica quem é dono de cada dado. O que
costuma contar quem ensina e quem aprende Rust, como ordem de grandeza e sem pretender que seja uma medição:

- **O borrow checker costuma se tornar intuitivo depois de várias semanas de prática.** Não antes. Não é que você seja lento: é que
  esse modelo mental se constrói esbarrando nele.
- Muitas pessoas dedicam **várias semanas** às bases antes de seu primeiro projeto real.
- **O compilador do Rust é o melhor professor que existe.** Seus erros explicam o problema, apontam a
  linha e **sugerem a correção**. Em Rust se aprende lendo erros, não evitando-os.

**Atenção: E aqui vai a diferença mais importante em relação ao curso de Go:** em Go a biblioteca padrão basta
para quase tudo. Em Rust **não**: async, HTTP e serialização vivem em *crates* externos (`tokio`, `reqwest`,
`serde`). Isso não é uma carência — é a decisão de que o padrão seja mínimo e estável. Mas significa
que aqui você vai, sim, usar dependências desde cedo.

## As nove lições

**O método é o que funciona, medido: ler um capítulo de The Book, fazer seus exercícios do Rustlings,
e só então seguir.** Ler de corrido retém muito menos, mesmo levando o mesmo tempo.

| | Lição | The Book | O que você constrói |
|---|---|---|---|
| 0 | [Preparação](00-preparacion.md) | cap 1 | `rustup`, `cargo` e Rustlings |
| 1 | [Fundamentos](01-fundamentos.md) | caps 2-3 | tipos, `mut`, controle de fluxo |
| 2 | [**Ownership**](02-ownership.md) | **cap 4** | **a lição que decide tudo** |
| 3 | [Structs, enums e match](03-structs-enums.md) | caps 5-6 | o modelo do `revisor`, com `Option` |
| 4 | [Coleções e erros](04-colecciones-errores.md) | caps 8-9 | `Vec`, `HashMap`, `Result`, `?` |
| 5 | [Traits, genéricos e lifetimes](05-traits-genericos.md) | cap 10 | o trait `Revisor` e os genéricos, e o `'a` que assusta |
| 6 | [Módulos, testes e cargo](06-modulos-pruebas.md) | caps 7, 11, 14 | o projeto de verdade |
| 7 | [Concorrência e async](07-concurrencia-async.md) | caps 16-17 | **que verifique tudo ao mesmo tempo** |
| 8 | [O programa terminado](08-el-programa.md) | cap 12 | `reqwest`, `serde`, `clap`, o binário |

**O Ownership vai na lição 2 e não pode ser movido.** É o capítulo 4 de The Book por uma razão: tudo o
mais —structs, coleções, erros, concorrência— se explica em termos de quem é dono de quê. Se você
o pula, as seis semanas seguintes são decorar regras sem sentido.

## As três fontes, e como combiná-las

1. **[The Book](https://doc.rust-lang.org/book/)** — oficial, 21 capítulos. Você o tem **offline**:
   `rustup doc --book`. Para este curso: **capítulos 1-12, 14 e 16-17**.
2. **[Rustlings](https://rustlings.rust-lang.org/)** — **95 exercícios interativos**, muitos voltados
   especificamente ao borrow checker. `cargo install rustlings`. **A ordem importa: capítulo, depois seus
   exercícios.**
3. **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** — como referência rápida.

**Depois, quando você já escrever Rust que funciona:** *Rust for Rustaceans* (Jon Gjengset) é o livro a
que recorrem os engenheiros que já passaram por esta etapa. Não antes: não serve como introdução.

## Como usá-lo

- **90 minutos por lição, no mínimo.** O Rust pede mais tempo sentado que o Go; com 60 não dá.
- **Leia os erros do compilador completos.** Não só a primeira linha. O Rust diz o problema, a
  causa e muitas vezes a solução exata. Ignorar isso é jogar fora a melhor ferramenta que você tem.
- **`cargo clippy` desde o primeiro dia.** É o linter oficial e ensina Rust idiomático enquanto
  você trabalha.
- **[`programas/`](../programas/)** — todos os programas das lições, prontos para compilar e executar, e o
  `revisor` completo com seus testes. Cada um é verificado automaticamente a cada mudança.
- **[`bitacora.md`](bitacora.md)** — aqui importa mais que em Go: anote **cada briga com o borrow
  checker**. Quando, na lição 5, você as reler, vai ver que as primeiras já parecem óbvias. Esse é
  o momento em que se aprendeu.
