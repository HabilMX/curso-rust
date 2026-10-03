# Lição 0 — Instalar o Rust no seu Linux Mint

**Tempo:** 90 min.

**O que você constrói:** seu ambiente e seu primeiro programa com `cargo`.

**O que você aprende:** por que não `apt install rustc`, `rustup`, `rustc` versus `cargo`, `cargo new/build/run/check`, o livro e o Rustlings sem conexão, ler um erro do compilador.

## Ao terminar, você vai poder

- Instalar o Rust estável com `rustup` e explicar por que não convém depender de `apt install rustc`.
- Identificar se o terminal está usando as ferramentas gerenciadas pelo `rustup`.
- Distinguir o trabalho do `rustc` do trabalho do `cargo`, e saber qual usar em cada caso.
- Criar um projeto com `cargo new`, compilá-lo, executá-lo e verificá-lo com `cargo check`.
- Abrir o The Rust Book sem conexão e instalar o Rustlings para praticar localmente.
- Ler por completo um diagnóstico do `rustc`, localizar a linha responsável e testar a correção sugerida.

## O porquê antes do como

Esta lição não parece uma lição de Rust, porque ainda não ensina ownership (propriedade), tipos nem `match`. Mesmo assim, ela decide uma parte importante de como você vai aprender a linguagem: desde o primeiro dia você vai trabalhar com a mesma cadeia de ferramentas, as mesmas convenções e o mesmo tipo de erros que um projeto real usa. Instalar algo que “mais ou menos compila” basta para um exercício isolado; instalar o ambiente correto é necessário para acompanhar um curso, ler a documentação atual e construir o `revisor` sem que a ferramenta se torne um problema adicional.

O curso foi escrito e verificado com o Rust estável 1.98.1 e a edição 2024. É uma referência concreta para que os programas, as mensagens e os exemplos tenham o mesmo significado para todos; com uma versão estável posterior os programas devem se comportar da mesma forma, embora o texto de alguma mensagem do compilador possa mudar de redação. O Rust publica uma versão estável aproximadamente a cada seis semanas. O Linux Mint, por sua vez, herda boa parte de seus pacotes do Ubuntu, e uma distribuição LTS prioriza a estabilidade do sistema: congela versões principais e aplica correções de segurança. É uma decisão razoável para programas do sistema; não é uma boa maneira de acompanhar de perto uma linguagem cujo ecossistema, documentação e ferramentas mudam com frequência.

Por isso `apt install rustc` parece funcionar no começo e pode causar confusão depois. Ele instala um compilador chamado `rustc`, mas não necessariamente o compilador que o The Rust Book, os exemplos recentes ou os projetos que você encontrar usam. O problema nem sempre se manifesta como “sua versão é antiga”. Às vezes aparece como um recurso desconhecido, uma edição que não existe, uma sugestão do compilador diferente ou uma dependência que já não aceita essa versão. É o pior tipo de falha de preparação: ocorre mais tarde e parece um erro do seu programa.

O Rust resolve isso com o `rustup`. Ele não é apenas um instalador: é o gerenciador oficial de toolchains do Rust. Uma *toolchain* reúne uma versão do `rustc`, o `cargo`, a biblioteca padrão, a documentação e componentes relacionados que devem funcionar em conjunto. O `rustup` instala o canal estável e coloca seus executáveis em um local previsível dentro do seu usuário. Quando chegar a hora de atualizar, você troca todo esse conjunto com `rustup update stable`, sem misturar pacotes do sistema nem baixar arquivos manualmente.

A comparação com o Go ajuda a situar a decisão. No Go você instalou a distribuição oficial porque o pacote da distribuição também podia ficar congelado; o Rust torna esse problema mais visível porque seu ritmo de publicação é mais curto e porque o `cargo` integra compilação, dependências, testes, formatação e análise estática. Nos dois cursos se constrói o mesmo `revisor`: uma ferramenta que lê uma lista de serviços, consulta cada um e reporta seu estado. No Go, o `go` concentra muitas tarefas. No Rust, o `cargo` cumpre esse papel em torno do `rustc`. A diferença não muda a disciplina: trabalha-se dentro de um projeto, compila-se com uma ferramenta repetível e lê-se o diagnóstico antes de mudar o código ao acaso.

O objetivo não é decorar uma lista de comandos. É construir um modelo mental simples. O `rustc` transforma um arquivo Rust em código executável e reporta os erros da linguagem. O `cargo` entende um projeto completo: conhece seu nome, edição, dependências, testes, perfis de compilação e estrutura de arquivos; depois chama o `rustc` com os argumentos corretos. No início você vai usar o `rustc` diretamente para ver, sem ruído, o que o compilador faz. No trabalho diário você vai usar o `cargo`, porque um programa real raramente consiste em um arquivo sem dependências.

A outra decisão importante desta lição é como responder a um erro. O Rust não tenta adivinhar o que você quis dizer nem deixa passar código duvidoso para falhar depois. O compilador interrompe a compilação, aponta tanto a origem quanto o uso problemático e, em muitos casos, propõe uma mudança concreta. Isso não significa que todas as mensagens sejam fáceis desde o primeiro dia; significa que vale a pena lê-las por inteiro. No Rust, o compilador faz parte do processo de aprendizado. As próximas lições vão fazer você provocar de propósito erros de propriedade, empréstimos e tipos, porque você vai entender mais diagnosticando uma falha real do que decorando uma regra isolada.

## Os conceitos

### `rustup` instala e gerencia a toolchain

A instalação recomendada no Linux Mint é a publicada pelo próprio projeto Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Antes de executar um comando que baixa e executa um script, vale a pena entendê-lo. O `curl` baixa o instalador; `--proto '=https'` limita o download a HTTPS; `--tlsv1.2` exige uma conexão TLS moderna; `-sSf` faz o comando falhar se o servidor devolver um erro; e `| sh` passa o conteúdo baixado ao interpretador de comandos. É o método oficial, mas ser oficial não elimina a responsabilidade de revisar o que você executa.

Se preferir inspecioná-lo primeiro, baixe o arquivo, leia-o e execute-o só depois de revisá-lo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
less rustup.sh
sh rustup.sh
```

O instalador oferece uma instalação padrão. Escolha a opção `1`, que instala o canal estável para a sua plataforma. Depois de terminar, abra um terminal novo. Se quiser usar o Rust no terminal atual sem fechá-lo, carregue o arquivo de ambiente que o instalador configurou:

```bash
source "$HOME/.cargo/env"
```

O local importante é `~/.cargo/bin`. É ali que o `rustup` coloca os programas que você invoca: `rustup`, `rustc`, `cargo`, `clippy-driver` e outros. O terminal só encontra comandos que estejam na sua variável `PATH`; por isso pode existir uma instalação correta no disco e, mesmo assim, aparecer `command not found`. O instalador tenta atualizar sua configuração de inicialização, mas cada shell e cada terminal pode ler arquivos diferentes.

Confirme a instalação com estes comandos:

```bash
rustup show active-toolchain
rustc --version
cargo --version
type -a rustc
type -a cargo
```

O primeiro indica a toolchain ativa. Os dois seguintes devem mostrar o Rust 1.98.1 ou uma versão estável posterior, se você já atualizou o ambiente. O `type -a` é especialmente útil se você alguma vez instalou o Rust com o `apt`: ele lista todas as ocorrências que a shell encontra e permite descobrir que você está executando um binário antigo antes do de `~/.cargo/bin`.

Não confunda uma atualização do Linux Mint com uma atualização do Rust. Para atualizar a toolchain estável, use:

```bash
rustup update stable
```

Não é preciso rodá-lo antes de cada comando; convém fazê-lo periodicamente e antes de começar uma sessão depois de várias semanas. Se você já estiver em dia, o `rustup` vai avisar. A intenção não é perseguir números de versão por esporte: é manter alinhados o compilador, a documentação, os exemplos e os componentes instalados.

No `revisor`, essa decisão se reflete desde a raiz. O projeto declara a edição 2024 e o Cargo constrói todos os seus módulos com a toolchain ativa. Não há uma “versão do Rust” escondida por arquivo: a configuração do pacote fixa a linguagem que o projeto fala e o lockfile fixa as versões concretas de suas dependências, para que uma compilação repetida resolva o mesmo conjunto.

### O `PATH` e o compilador que você realmente executa

Quando você digita `rustc`, a shell não procura em todo o disco. Ela percorre as pastas listadas em `PATH`, em ordem, e executa a primeira ocorrência. Essa regra explica dois diagnósticos comuns. Se não há nenhum `rustc` nessas pastas, o `bash` costuma mostrar:

```bash
rustc: command not found
```

Se existe um, mas ele vem de uma instalação anterior do `apt`, o comando pode funcionar e mostrar uma versão inesperada. Esse caso engana mais: não parece um problema de instalação, mas o curso estaria sendo compilado com uma ferramenta diferente da esperada.

Primeiro confirme qual shell você usa e como ela foi iniciada:

```bash
echo "$SHELL"
echo "$0"
printf '%s\n' "$PATH"
```

Em uma instalação usual com Bash, o `~/.bashrc` é carregado para shells interativas e o `~/.profile` para uma sessão de login. No Zsh, o arquivo equivalente para sessões interativas costuma ser o `~/.zshrc`. O arquivo que o `rustup` cria, `~/.cargo/env`, adiciona `~/.cargo/bin` ao `PATH`. Executar `source "$HOME/.cargo/env"` no terminal atual é uma verificação direta: se, depois disso, `rustc --version` funcionar, o problema era o ambiente da shell, não o compilador.

Não adicione caminhos duplicados uma e outra vez sem verificar o que aconteceu. Primeiro use `type -a rustc`. Se aparecer um caminho de `~/.cargo/bin`, o `rustup` está disponível no `PATH`. Se aparecer antes um caminho como `/usr/bin/rustc`, há uma instalação do sistema tomando prioridade. Nesse caso, identifique primeiro quais pacotes estão instalados e qual binário a shell está usando; não tente consertar copiando executáveis nem modificando links simbólicos à mão.

O `revisor` não depende de um caminho fixo do compilador. Essa é uma vantagem de trabalhar com o `cargo`: a ferramenta invoca o `rustc` da toolchain ativa e guarda o resultado das compilações dentro de `target/`. Por isso um projeto Cargo pode ser construído da mesma forma em outro computador com uma instalação correta, sem que o código contenha caminhos pessoais nem comandos específicos da sua máquina.

### `rustc`: o compilador e o primeiro programa

O `rustc` é o compilador do Rust. Ele recebe código-fonte, verifica se respeita as regras da linguagem e, se tudo estiver certo, produz um executável. Para um arquivo pequeno é útil invocá-lo diretamente, porque permite ver a relação exata entre fonte, compilação e programa resultante. Cada figura deste curso é compilada assim, para que a saída documentada corresponda ao código que você está lendo.

**Fig. 0.1** | O primeiro programa.

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

`fn main()` declara a função pela qual um programa executável começa. O Rust procura justamente uma função chamada `main` para começar. Dentro dela, `println!` imprime texto e acrescenta uma quebra de linha. O sinal `!` não é decorativo: `println!` é uma macro. As macros geram ou transformam código durante a compilação; por enquanto basta reconhecer a convenção de que os nomes que terminam em `!` não são funções normais. O The Rust Book volta a esse tema no capítulo 20.

O indicador `--edition 2024` seleciona a edição atual da linguagem. Uma edição não significa que seu código se converte automaticamente para um idioma diferente a cada ano; é uma forma de o Rust poder melhorar regras e sintaxe sem quebrar silenciosamente projetos existentes. O projeto `revisor` declara essa mesma edição em seu manifesto. Os exemplos do curso a indicam explicitamente para que o comando de uma figura não dependa da edição padrão de uma instalação em particular.

Esse uso direto do `rustc` é deliberadamente pequeno. Se você tivesse dois arquivos, uma biblioteca, testes, dependências externas, opções de otimização e diferentes plataformas de destino, escrever à mão a invocação correta do compilador se tornaria frágil. É aí que entra o `cargo`. A relação não é uma competição entre dois programas: o Cargo organiza, o Rustc compila. Quando você usa `cargo build`, o Cargo acaba chamando o `rustc` por você, com os caminhos, a edição e as flags de que o projeto precisa.

O mesmo ponto vale para o `revisor`: o binário tem uma função `main`, mas ele não se compila isolando esse arquivo com `rustc src/main.rs`. Ele importa módulos da biblioteca local e crates externos; precisa do manifesto e da estrutura que o Cargo conhece. As figuras desta lição ensinam o mecanismo de compilação. As lições posteriores aplicam esse mecanismo a um programa composto.

### `cargo`: o projeto antes do arquivo

Crie seu primeiro projeto em uma pasta de trabalho própria:

```bash
mkdir -p "$HOME/w/curso-rust"
cd "$HOME/w/curso-rust"
cargo new hola
cd hola
```

`cargo new hola` cria uma pasta chamada `hola`, um manifesto `Cargo.toml` e o arquivo `src/main.rs`. Se o Git estiver instalado e o Cargo puder inicializar um repositório, ele também prepara o Git e um `.gitignore`; se o Git não estiver disponível, o projeto continua sendo válido. O resultado mínimo tem esta estrutura:

```text
hola/
├── Cargo.toml
├── .gitignore
└── src/
    └── main.rs
```

`Cargo.toml` é o manifesto do pacote. Ele registra a identidade do projeto, sua edição, suas dependências e algumas decisões de construção. O arquivo não é um detalhe administrativo: é o que permite que outra pessoa execute o mesmo `cargo build` sem reconstruir à mão a lista de argumentos do compilador. Quando o Cargo resolve dependências, ele cria também o `Cargo.lock`; esse arquivo registra a resolução concreta para que as compilações sejam repetíveis.

O `revisor` já é um projeto Cargo completo. Este é o manifesto real dele:

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

Você ainda não precisa entender as dependências nem o perfil de release. O importante é reconhecer a forma: `[package]` descreve o pacote; `[dependencies]` enumera o que ele precisa para compilar; `[profile.release]` muda como o binário final será construído. Na lição 4 você vai conhecer as dependências de erros e serialização, na 6 a ordem dos módulos e dos testes, e na 8 o perfil de release. Hoje basta entender por que um projeto precisa de um manifesto e por que não convém substituir o Cargo por um longo comando manual do `rustc`.

Execute o projeto recém-criado com:

```bash
cargo run
```

Na primeira vez o Cargo compila o pacote e depois executa o binário. Nas execuções seguintes, ele reaproveita os artefatos que não mudaram. Diferentemente de `rustc fig00_01.rs`, você não precisa digitar o nome do arquivo nem o nome do executável: o Cargo conhece a convenção `src/main.rs` e sabe que o pacote `hola` produz o binário `hola`.

Essa convenção reduz decisões repetitivas. Um programa Rust pode ser organizado de várias maneiras, mas o Cargo oferece uma estrutura comum para os casos frequentes. O projeto que você criou hoje traz apenas `src/main.rs`, o binário. Quando você abrir o `revisor`, vai reconhecer também `src/lib.rs`, a biblioteca do pacote. Essa separação não é inventada na lição 6: o Cargo a reconhece por convenção, assim como reconhece `src/main.rs`.

### `cargo build`, `run`, `check` e o ciclo de trabalho

Os comandos do Cargo não são sinônimos. Cada um responde a uma pergunta diferente que você se faz enquanto trabalha:

```bash
cargo run
cargo build
cargo build --release
cargo check
cargo test
cargo clippy
cargo fmt
```

`cargo run` responde “meu programa compila e o que ele faz?”. Primeiro ele constrói o necessário e depois executa o binário. É o comando que você vai usar quando mudar uma saída, testar um ramo do programa ou quiser observar um comportamento. Para o projeto `hola`, ele deve imprimir a mensagem que estiver em `src/main.rs`.

`cargo build` responde “consigo produzir o binário?”. Ele compila o pacote, mas não o executa. No modo de desenvolvimento, deixa os artefatos em `target/debug/`; você não precisa decorar esse caminho, mas ajuda saber que o Cargo não enche a pasta raiz do projeto de executáveis e arquivos intermediários. Isso mantém separados o código-fonte e os resultados da compilação.

`cargo build --release` produz o perfil de release, normalmente com mais otimização e com os resultados dentro de `target/release/`. A diferença importa quando você for entregar o `revisor` terminado ou medir seu desempenho. Não compare tempos de execução de um binário de desenvolvimento para tirar conclusões sobre o desempenho do Rust: o perfil de desenvolvimento prioriza compilar rápido e depurar com conforto; o perfil de release prioriza o programa resultante. O `Cargo.toml` do `revisor` mostra que esse perfil pode até ajustar o tamanho, a otimização entre módulos e o comportamento diante de `panic!`.

`cargo check` responde “o compilador aceita meu código?”. Ele verifica tipos, empréstimos, módulos e boa parte do trabalho de compilação, mas não completa a geração de um binário executável. Em um projeto que está crescendo, pode poupar tempo. Use-o enquanto escreve, especialmente quando você só quer saber se uma modificação é válida; use `cargo run` quando também quiser executar o comportamento. Nenhum dos dois comandos substitui o outro: um verifica com rapidez e o outro verifica, além disso, o resultado em execução.

`cargo test` compila e roda testes. Você ainda não escreveu testes nesta lição, mas vai começar a vê-los na lição 6. `cargo clippy` executa o linter oficial e aponta padrões que compilam, mas costumam ser confusos, ineficientes ou pouco idiomáticos. `cargo fmt` aplica a formatação padrão do Rust. É a mesma disciplina do `gofmt` no Go: não se gasta tempo discutindo o alinhamento de cada arquivo, deixa-se a ferramenta dar uma resposta uniforme.

O `revisor` usa exatamente esse ciclo. Antes de publicar uma mudança, convém executar, a partir de `programas/revisor/`, estes comandos:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

O primeiro verifica o comportamento; o segundo transforma os avisos do Clippy em falhas para que não sejam ignorados; o terceiro confirma que o código já tem a formatação esperada. Você não deve executá-los agora para “entender” a saída deles. Guarde-os como referência da rotina a que você vai chegar ao construir o programa.

### Imutabilidade por padrão e a primeira conversa com o `rustc`

O Rust considera imutável uma variável criada com `let`, a menos que você escreva `mut`. Isso não é um obstáculo posto para deixar o código mais longo. É uma declaração visível de intenção: se um valor deve mudar, o leitor e o compilador precisam poder ver isso no lugar onde a variável é definida.

**Fig. 0.2** | Um programa que não compila.

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

A solução sugerida pelo compilador está correta se você realmente quer mudar o valor. A palavra `mut` se escreve na declaração, não na atribuição posterior. Assim, quem ler o bloco sabe desde o início que `x` faz parte do estado mutável daquela função.

**Fig. 0.3** | A mesma variável, agora mutável.

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

A imutabilidade por padrão ajuda a reduzir mudanças acidentais e prepara o terreno para o ownership e os empréstimos. Em outras linguagens é comum modificar uma variável porque a linguagem permite e só depois perguntar quem dependia do valor anterior. O Rust pede que você declare essa possibilidade desde o princípio. Ele não elimina todos os erros, mas transforma uma suposição implícita em uma propriedade verificável.

No `revisor` há mutabilidade apenas onde a operação realmente a exige. A função que ordena as linhas cria um vetor e depois o ordena no lugar; por isso a variável é declarada `mut`:

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

Não se escreve `mut` por costume. `filas.sort_by(...)` modifica o vetor, então a declaração comunica isso. Já `servicios` e `estados` são referências que essa função apenas consulta; não são declaradas mutáveis. Essa diferença, pequena nesta função, se torna importante quando várias partes de um programa tentam usar os mesmos dados. A lição 2 explica as regras que o Rust aplica para que esses usos sejam seguros.

### The Rust Book e o Rustlings como prática local

O The Rust Book é o texto oficial de referência do curso. O `rustup` instala uma cópia local da documentação, de modo que você pode abri-la sem conexão depois de concluída a instalação:

```bash
rustup doc --book
```

Esse comando abre o capítulo inicial no navegador padrão. Se você está sem ambiente gráfico ou prefere localizar outros documentos, `rustup doc --help` mostra as opções disponíveis. A cópia local não dispensa as atualizações: se você atualizar a toolchain, a documentação local é atualizada junto. Essa é outra razão para manter compilador e documentação juntos.

Leia por completo o capítulo 1 do The Rust Book: instalação, “Hello, world!” e “Hello, Cargo!”. Não o pule só porque você já criou um projeto. O que você fez aqui lhe dá contexto para lê-lo mais rápido; o capítulo organiza os conceitos e explica as convenções que vão reaparecer ao longo de todo o curso. O capítulo 2, que inclui o jogo de adivinhação, corresponde à lição 1 junto com os fundamentos do capítulo 3.

O Rustlings complementa o livro com pequenos exercícios que se resolvem editando arquivos locais. A instalação inicial precisa, sim, de acesso à rede para que o Cargo baixe o programa; depois disso, o diretório de exercícios e o código vivem no seu computador. Instale-o e inicialize-o assim:

```bash
cargo install rustlings
rustlings init
cd rustlings
rustlings
```

O comando interativo observa os exercícios, indica qual deles falha e volta a verificá-los conforme você salva as mudanças. Não use o Rustlings como uma coleção de respostas para riscar da lista. A ordem do curso é intencional: primeiro leia o capítulo do The Rust Book, depois resolva os exercícios correspondentes e então aplique a ideia ao `revisor`. Nesta etapa, instale o Rustlings e familiarize-se com o diretório dele; na lição 1 você vai trabalhar as seções `variables`, `functions`, `if` e `primitive_types`.

O livro e os exercícios cumprem funções diferentes. O The Rust Book explica o modelo e dá nome às suas peças; o Rustlings obriga você a mexer no código e receber um erro concreto. O `revisor` é o problema de integração: não é um exercício isolado, e sim o mesmo programa que você já construiu em Go, agora com as decisões do Rust. As três fontes se apoiam entre si. Se uma explicação parecer abstrata, tente um exercício; se um exercício parecer mecânico, volte ao capítulo; se os dois já estiverem claros, localize o padrão dentro do projeto real.

## O erro que você vai ver

O erro central desta lição é o `E0384`, mostrado por completo na Fig. 0.2. Não o leia como uma parede de texto. Leia-o em ordem. A primeira linha nomeia o código de diagnóstico, `E0384`, e resume o problema: você não pode atribuir duas vezes a uma variável imutável. Esse código serve para pedir ao compilador uma explicação ampliada:

```bash
rustc --explain E0384
```

A linha que começa com `--> fig00_02.rs:4:5` localiza a tentativa de atribuição: arquivo, linha e coluna. As linhas numeradas mostram contexto suficiente para você não precisar procurar às cegas. A marca `^^^^^` aponta a parte exata que provoca o erro. Antes dela aparece a primeira atribuição, na linha 3, porque o Rust não apenas informa onde detectou o problema: também mostra a origem da condição que o torna inválido.

A seção `help:` merece atenção especial. Neste caso ela propõe trocar `let x = 5;` por `let mut x = 5;`, e as marcas `+++` indicam qual texto acrescentar. Não aplique todas as sugestões mecanicamente. Primeiro verifique a intenção: se `x` não deveria mudar, a solução correta não é acrescentar `mut`, e sim eliminar ou repensar a atribuição posterior. O compilador pode oferecer uma correção local; você decide se essa correção representa o design correto.

O mesmo diagnóstico também inclui um aviso (warning): o primeiro valor, `5`, nunca é lido porque é sobrescrito imediatamente. O aviso não impede a compilação por si só, mas traz informação útil. Se você tornar `x` mutável sem olhar o aviso, o programa continuará tendo uma atribuição desnecessária. A versão da Fig. 0.3 imprime `5` antes de alterá-lo, então os dois valores fazem sentido e o programa não gera avisos.

Quando você executar o mesmo erro dentro de um projeto com `cargo run` ou `cargo check`, o Cargo vai mostrar o diagnóstico do `rustc` junto com o contexto do pacote e o caminho `src/main.rs`. A regra não muda: comece pelo primeiro erro, leia suas notas e sua ajuda, corrija uma causa de cada vez e compile de novo. Um erro inicial pode provocar vários posteriores; tentar corrigir todos de uma só vez costuma esconder a causa real.

Há outros erros de preparação que não são códigos do Rust, porque ocorrem antes de o compilador poder analisar o seu programa. Se `cargo` ou `rustc` não existem para o terminal, o problema é o `PATH`; carregue `~/.cargo/env` de novo e revise `type -a cargo`. Se a compilação chegar a fazer a ligação (linking) e aparecer uma mensagem parecida com `error: linker 'cc' not found`, falta o compilador C que o Rust usa para fazer a ligação no Linux Mint. Instale o pacote de ferramentas de construção da distribuição e execute o comando novamente:

```bash
sudo apt install build-essential
```

Não confunda esse caso com “o Rust não foi instalado”. O `rustc --version` pode funcionar perfeitamente; a falha aparece depois, quando o compilador precisa converter objetos compilados em um executável do sistema. Separar a etapa que falha evita correções aleatórias.

## O que se faz errado

- Instalar o Rust com `apt install rustc` e dar por certo que o nome do pacote garante uma toolchain atual. O problema não é que o pacote seja inútil; é que ele segue o calendário da distribuição, não o do Rust. Para este curso use o `rustup`, confira `rustc --version` e atualize o canal estável periodicamente.

- Misturar uma instalação do `apt` com outra do `rustup` sem verificar qual vence no `PATH`. Ter dois executáveis chamados `rustc` não produz necessariamente um erro imediato. Você pode compilar durante dias com a versão errada. Use `type -a rustc` e `type -a cargo` antes de modificar arquivos de inicialização ou remover pacotes.

- Usar o `rustc` para um projeto completo por costume. Para uma figura de um único arquivo, é uma ferramenta didática excelente. Para o `revisor`, significaria reconstruir manualmente dependências, caminhos, edição, módulos e perfis. Use o `cargo` a partir da raiz do projeto; deixe que ele monte a invocação do `rustc`.

- Usar `cargo run` toda vez que você quer saber se o código compila. Funciona, mas constrói e executa mesmo quando você só está corrigindo tipos ou empréstimos. Durante a edição rápida, `cargo check` dá um retorno mais direto. Quando precisar observar o comportamento, use `cargo run`.

- Medir desempenho com um binário de desenvolvimento. `cargo build` e `cargo run` usam o perfil de desenvolvimento por padrão. Quando o curso chegar a comparar tamanho, velocidade ou entrega do binário, use `cargo build --release`. Sem essa distinção, uma medição diz mais sobre o perfil escolhido do que sobre o programa.

- Ignorar um aviso porque “não impede de compilar”. Os avisos costumam apontar valores não usados, código morto ou construções confusas. Este curso compila as figuras corretas com avisos tratados como erros, para que a saída mostrada não esconda problemas. Faça o mesmo na sua rotina: entenda o aviso ou elimine a causa dele.

- Ler só a primeira linha de um erro. A primeira linha nomeia a categoria; as linhas seguintes dizem onde ocorreu, qual valor anterior o explica, quais notas se aplicam e que alternativa o compilador considera. Copiar apenas “error E0384” para pesquisá-lo perde boa parte da resposta que já está diante de você.

- Instalar o Rustlings e resolver exercícios com respostas copiadas. Um exercício terminado sem entender o diagnóstico não constrói o modelo mental de que você vai precisar para o ownership. Faça mudanças pequenas, execute o verificador, leia o erro e explique com suas palavras por que a solução compila.

## Exercícios

### Exercício 1 — Verifique sua toolchain

Instale o Rust com o `rustup`, se ainda não o tiver. Execute `rustup show active-toolchain`, `rustc --version`, `cargo --version` e `type -a rustc`. Anote qual caminho o terminal está usando para o `rustc` e confirme que corresponde a `~/.cargo/bin` quando você usa a instalação gerenciada pelo `rustup`.

### Exercício 2 — Crie e percorra um projeto Cargo

Em uma pasta de trabalho, execute `cargo new saludo-rust` e entre no diretório criado. Leia `Cargo.toml` e `src/main.rs` antes de modificá-los. Troque a mensagem por uma sua e rode, nesta ordem, `cargo check`, `cargo build` e `cargo run`. Explique qual pergunta cada comando respondeu e qual arquivo ou resultado você esperava de cada um.

### Exercício 3 — Provoque e explique o `E0384`

Substitua temporariamente o conteúdo de `src/main.rs` pelo programa da Fig. 0.2 e execute `cargo check`. Não conserte nada até ter identificado o arquivo, a linha, a primeira atribuição e a sugestão marcada como `help:`. Depois mude a declaração para `let mut x = 5;`, observe o aviso restante e modifique o programa para que os dois valores sejam lidos, como na Fig. 0.3.

### Exercício 4 — Prepare a leitura e a prática local

Abra o The Rust Book com `rustup doc --book` e leia por completo o capítulo 1. Instale o Rustlings com `cargo install rustlings`, execute-o no seu diretório local e descubra como reabrir os exercícios sem depender de uma página da web. Escreva uma nota breve que distinga o que você obtém do livro, o que obtém do Rustlings e o que você vai construir depois no `revisor`.

## Soluções

### Solução 1

Uma instalação correta mostra uma toolchain estável ativa e permite executar tanto `rustc --version` quanto `cargo --version`. A saída exata pode mudar ao atualizar o Rust, mas as duas ferramentas devem pertencer à mesma instalação estável. `type -a rustc` deve listar `~/.cargo/bin/rustc` como o caminho escolhido ou, pelo menos, permitir que você explique por que outro caminho tem prioridade. Se ele não aparecer, execute `source "$HOME/.cargo/env"` e verifique de novo.

### Solução 2

`cargo check` verifica o projeto sem terminar de produzir um executável; `cargo build` compila o pacote e deixa artefatos de desenvolvimento dentro de `target/debug/`; `cargo run` compila o necessário e executa o binário. Os três comandos devem aceitar o projeto `saludo-rust`. A saída de `cargo run` deve ser exatamente a mensagem que você deixou em `src/main.rs`.

### Solução 3

`cargo check` mostra `E0384` porque `let x = 5;` cria uma associação imutável e a linha seguinte tenta reatribuí-la. Mudá-la para `let mut x = 5;` permite a reatribuição, mas inicialmente deixa um aviso porque o `5` é sobrescrito sem ser usado. Imprimir `x` antes e depois da atribuição elimina o aviso e produz as duas linhas da Fig. 0.3. O aprendizado não é “acrescente `mut` sempre”; é declarar mutabilidade apenas quando a modificação faz parte do design.

### Solução 4

O The Rust Book apresenta a explicação ordenada da instalação, do programa inicial e do Cargo; o capítulo 1 fica disponível localmente com `rustup doc --book`. O Rustlings oferece exercícios editáveis e retorno sobre o código local depois de instalá-lo e inicializar o diretório. O `revisor` é onde essas peças se combinam em uma aplicação: ele não substitui o livro nem os exercícios, e sim fornece um problema contínuo no qual aplicar os conceitos das lições seguintes.

## Como sei que consegui

- [ ] `rustc --version` e `cargo --version` funcionam e mostram uma toolchain estável compatível.
- [ ] `type -a rustc` me permite identificar qual compilador meu terminal está executando.
- [ ] `rustup update stable` termina sem erros e sei explicar o que ele atualiza.
- [ ] `cargo new saludo-rust` criou um projeto com `Cargo.toml` e `src/main.rs`.
- [ ] `cargo check`, `cargo build` e `cargo run` funcionam dentro desse projeto, e sei o que cada um faz.
- [ ] Meu programa imprime a mensagem que escrevi quando executo `cargo run`.
- [ ] Consigo provocar o `E0384`, apontar a linha de origem, ler a ajuda dele e consertar o programa sem deixar avisos.
- [ ] `rustup doc --book` abre o The Rust Book local e o Rustlings está inicializado em uma pasta local.

## Para ler mais

- [The Rust Programming Language, capítulo 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html) — instalação, primeiro programa e Cargo. Consultado em 2 de outubro de 2026.

- [Rust: Install](https://www.rust-lang.org/tools/install) — instalação oficial com `rustup`, atualização de toolchains e notas sobre o `PATH`. Consultado em 2 de outubro de 2026.

- [The Cargo Book: Why Cargo Exists](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html) — por que o Cargo gerencia pacotes, dependências e as invocações ao `rustc`. Consultado em 2 de outubro de 2026.

- [Rustlings](https://rustlings.rust-lang.org/) — instalação, inicialização e uso de exercícios locais em paralelo com o The Rust Book. Consultado em 2 de outubro de 2026.
