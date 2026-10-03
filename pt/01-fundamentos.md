# Lição 1 — Fundamentos

**Tempo:** 2 × 45 min.

**O que você constrói:** as funções e os tipos base do `revisor`

**O que você aprende:** variáveis e `mut`, sombreamento, tipos escalares e compostos, funções, «tudo é uma expressão», `if`, `loop`, `while` e `for`

## Ao terminar, você vai poder

- Declarar variáveis imutáveis, mutáveis e constantes, e explicar quando cabe cada uma.
- Escolher tipos numéricos, booleanos, caracteres, tuplas, arrays, vetores e strings para dados simples do `revisor`.
- Escrever funções com parâmetros e valores de retorno sem depender de conversões implícitas.
- Explicar por que um bloco, um `if` e um `loop` podem produzir valores.
- Usar `if`, `loop`, `while` e `for` para classificar e percorrer dados de forma legível.
- Ler e corrigir duas variantes comuns do erro `E0308`.
- Resolver os exercícios `variables`, `functions`, `if` e `primitive_types` do Rustlings.

## O porquê antes do como

O `revisor` que você vai construir ao longo do curso recebe uma lista de serviços, consulta cada um e reporta o que aconteceu. Embora no final ele vá ter HTTP, arquivos YAML, concorrência e saída JSON, seu núcleo começa com operações muito menores: guardar um tempo de resposta, compará-lo com um limite, percorrer uma lista e decidir qual texto mostrar. Antes de modelar um serviço com um `struct` ou uma falha com um `enum`, você precisa ser capaz de expressar essas operações com precisão.

Pense numa regra inicial do programa: uma resposta de até mil milissegundos é considerada normal; uma mais lenta é reportada como lenta. A regra parece simples, mas contém várias decisões que o Rust quer que você declare: o tempo não pode ser texto, tem de ser um número; o limite precisa ter um tipo compatível; a comparação deve produzir uma condição booleana; e a função deve entregar sempre uma classificação. O Rust não deixa essas decisões escondidas em conversões automáticas ou valores ambíguos. O compilador exige que o programa diga o que cada dado representa.

Essa insistência pode parecer pesada se você vem do Go, do Python ou do JavaScript. No Go também existem tipos estáticos e as conversões entre números são explícitas, mas o Rust estende essa precisão a outras partes da sintaxe. Uma variável é imutável por padrão. Um bloco pode devolver um valor. Um `if` deve produzir valores do mesmo tipo nas duas ramificações quando é usado como expressão. Um `for` distingue entre percorrer uma coleção, emprestá-la ou consumi-la. No começo são mais decisões visíveis; depois são informação que evita que alguém interprete mal a sua intenção ao manter o programa.

O modelo mental útil não é “o Rust põe obstáculos antes de rodar”. É “o Rust transforma decisões de design em coisas verificáveis”. Se você nomeia uma medida como `u64`, o compilador sabe que ela não pode ser negativa. Se você torna uma variável mutável, o leitor sabe que ela vai mudar. Se uma função devolve `&'static str`, fica claro que ela devolve uma de várias etiquetas fixas e não um texto recém-construído. Se o resultado de um `if` é guardado numa variável, todas as suas ramificações devem descrever o mesmo tipo de resultado. A maior parte do que vem a seguir no curso se apoia nessa mesma ideia.

Esta lição trabalha os capítulos 2 e 3 de *The Rust Programming Language*. O capítulo 2 apresenta `let`, funções e o uso de `mut` dentro de um programa pequeno; o capítulo 3 organiza os fundamentos: variáveis, tipos de dados, funções, comentários e fluxo de controle. Não tente decorar todos os tipos disponíveis de uma vez só. O importante é aprender a ler uma assinatura, escolher uma representação razoável e deixar que o compilador aponte as contradições.

O programa real já contém esses fundamentos. Em `programas/revisor/src/modelo.rs` há limites expressos como constantes; em `src/config.rs` há um `for` que valida cada serviço; em `src/revisar.rs` há condições que classificam respostas; e em `src/reporte.rs` há variáveis mutáveis para construir uma saída. Esta lição não modifica esse projeto: usa-o como mapa de para onde vão as peças pequenas que você vai praticar aqui.

## Os conceitos

### Variáveis, `mut`, constantes e sombreamento (shadowing)

Uma variável se declara com `let`. Por padrão, a associação entre o nome e seu valor é imutável: depois de escrever `let x = 5;`, você não pode atribuir outro valor a `x`. Essa escolha é deliberada. Quando você lê uma função longa, cada nome que não leva `mut` lhe dá uma garantia local: esse nome continuará representando o mesmo valor durante o resto do seu escopo.

A imutabilidade não significa que o Rust proíba mudar dados. Significa que você precisa declarar isso. Se uma variável representa um contador, uma saída que você constrói aos poucos ou um índice que decresce, use `let mut`. A palavra `mut` vai junto do nome porque descreve a associação, não a função inteira. Evite pôr `mut` por costume: uma variável mutável que nunca muda provoca um aviso (warning), e compilar os exemplos com `-D warnings` converte esse aviso em erro. É um sinal pequeno, mas útil: o código diz que algo vai variar e na verdade isso não acontece.

As constantes se escrevem com `const`, levam tipo explícito e são avaliadas antes de o programa ser executado. Use-as para regras cujo nome deve aparecer em todo o código: um limite de tempo, uma capacidade ou um máximo de tentativas. Uma constante não é uma variável imutável com outro nome. Ela não ocupa um lugar único na memória que você possa emprestar ou modificar; é substituída onde é usada. Nesta etapa basta lembrar a regra prática: `let` para valores locais e `const` para uma regra estável e nomeada.

**Fig. 1.1** | Variáveis, mutabilidade e constantes.

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

O tipo de `x` está escrito como `i32`, mas o Rust poderia inferi-lo aqui, porque `y += x` e o literal `10` dão contexto suficiente. Anotar tipos ajuda quando uma assinatura faz parte de uma API, quando o compilador não consegue inferi-los ou quando você quer comunicar uma restrição importante. Não os anote mecanicamente em cada `let`: a inferência bem usada reduz o ruído sem perder segurança.

O sombreamento é diferente da mutabilidade. Com o sombreamento você declara uma nova variável com o mesmo nome; a anterior deixa de ser acessível a partir desse ponto. É útil quando uma ideia passa por etapas e você quer manter um nome honesto. Por exemplo, um texto com espaços e a quantidade de espaços são dois valores distintos, mas ambos podem se chamar `espacios`, porque a primeira versão já não é necessária. Diferentemente de `mut`, o sombreamento permite que o tipo mude.

**Fig. 1.2** | Sombreamento, tuplas e arrays.

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

Aqui o primeiro `espacios` é `&str`, uma visão de texto; o segundo é `usize`, uma quantidade. Não é que uma variável tenha mudado de texto para número: são duas associações distintas, com escopos sobrepostos. Essa diferença importa mais adiante, com o ownership. `let mut nombre` conserva o mesmo valor e permite modificá-lo; `let nombre = ...` volta a associar o nome e pode transformar o valor sem conservar a versão anterior.

No `revisor`, uma variável mutável aparece ao construir a tabela que será impressa. O nome `salida` não representa uma regra fixa: é um acumulador ao qual se acrescentam linhas, então `mut` comunica exatamente a intenção.

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

Você ainda não precisa entender referências, iteradores nem `format!` para reconhecer a decisão fundamental: `filas` e `ancho` não mudam; `salida` muda. Na lição 2 você vai estudar por que `&[Servicio]` e `&[Estado]` são empréstimos (borrows), e na lição 4 vai ver como `String` permite construir texto dinâmico. Por enquanto, identifique o padrão: declare como imutável até que uma modificação seja parte real do trabalho.

As regras de classificação do `revisor` também são constantes. O valor não está repetido como um número anônimo em cada comparação; tem nome, tipo e comentário. Quando a política de “lento” mudar, haverá um lugar óbvio para revisar.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;
```

`TIMEOUT_POR_OMISION_MS` é privado ao módulo porque só é usado para criar serviços com o valor padrão. `UMBRAL_LENTO_MS` leva `pub` porque outro módulo, `revisar.rs`, precisa consultá-lo. A visibilidade dos módulos é estudada formalmente na lição 6; o importante hoje é que os dois valores têm tipos explícitos e nomes que expressam unidades. Um `1000` sem nome deixa perguntas: mil segundos, mil bytes, mil milissegundos? `UMBRAL_LENTO_MS` as responde.

### Tipos escalares e compostos

O Rust é uma linguagem de tipos estáticos: antes de executar, o compilador conhece o tipo de cada valor. Às vezes ele o infere e às vezes você precisa anotá-lo, mas nunca trata um número como texto nem mistura dois tamanhos inteiros porque “mais ou menos parecem compatíveis”. Esse rigor permite que muitos enganos sejam detectados antes de produzir um binário.

Os tipos escalares guardam um único valor. Os inteiros com sinal são `i8`, `i16`, `i32`, `i64`, `i128` e `isize`; os sem sinal são `u8`, `u16`, `u32`, `u64`, `u128` e `usize`. O número diz quantos bits o valor ocupa. `isize` e `usize` mudam conforme a arquitetura e são usados principalmente para tamanhos, comprimentos e índices. Para as quantidades de milissegundos do `revisor`, `u64` é uma decisão explícita: não existem tempos negativos e a faixa é ampla. Para um código HTTP, `u16` é suficiente. Escolher um tipo não consiste em procurar o menor número possível; consiste em expressar o domínio do dado de maneira sensata.

`i32` é o tipo inteiro padrão quando o compilador não recebe mais contexto. É uma boa escolha geral para cálculos inteiros locais. Não suponha que todos os inteiros são `i32`: um `usize` que vem de `len()` não pode ser somado diretamente a um `u64`, e um `u16` de um código HTTP não vira `i32` só por estar na mesma operação. A vantagem é que você vê o cruzamento de domínios no ponto exato em que ele acontece.

O Rust não faz conversões numéricas implícitas. Não é uma esquisitice isolada: evita que uma atribuição aparentemente inocente mude tamanho, sinal ou faixa sem que quem escreveu o código tenha considerado isso. No Go as conversões entre tipos numéricos também são pedidas explicitamente; o Rust conserva essa disciplina e a torna especialmente importante porque seus tipos inteiros são usados com frequência para representar capacidades, comprimentos e dados de rede.

**Fig. 1.3** | Não há conversão implícita, nem entre números.

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

Para uma conversão simples e conhecida você pode usar `as`. A conversão de `i32` para `i64` é segura neste caso porque todo `i32` cabe num `i64`. No entanto, `as` também permite conversões que podem truncar, reinterpretar o sinal ou perder precisão. Não o use como uma forma de “fazer o compilador se calar”. Quando uma conversão puder falhar ou perder informação, mais adiante você vai conhecer `TryFrom`, `TryInto` e `Result`.

**Fig. 1.4** | A conversão é pedida com `as`.

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

Além dos inteiros, os escalares incluem `f32` e `f64` para números de ponto flutuante, `bool` para `true` ou `false`, e `char` para um caractere Unicode. No `revisor`, evite usar ponto flutuante se um inteiro expressa melhor a unidade. Guardar `750` milissegundos como `u64` é mais claro do que guardar `0.75` segundos como `f64`, e evita perguntas sobre arredondamento quando você mostrar, comparar ou serializar o valor.

Os tipos compostos agrupam vários valores. Uma tupla pode guardar elementos de tipos distintos e tem tamanho fixo. Na figura 1.2, `(u16, u64, bool)` representa três resultados que pertencem a uma mesma medição: código, duração e estado de saúde. A desestruturação `let (codigo, ms, saludable) = medicion;` extrai esses valores com nomes úteis. As tuplas são adequadas para resultados pequenos e locais; quando o significado dos campos for central para o programa, como será o de um serviço, uma struct com campos nomeados será melhor. Isso chega na lição 3.

Um array como `["catalogo", "pagos"]` contém valores do mesmo tipo e tem comprimento fixo conhecido em tempo de compilação. Um vetor, `Vec<T>`, também contém valores do mesmo tipo, mas pode crescer ou encolher em tempo de execução. A figura 1.7 usa `vec!` porque a lista de serviços é uma coleção que conceitualmente pode mudar de tamanho. No projeto real, a configuração é carregada a partir de YAML e produz um `Vec<Servicio>` pela mesma razão.

As strings também exigem precisão. Um literal como `"catalogo"` costuma ser `&str`, uma visão emprestada de um texto já existente. Um `String` é um texto que possui memória e pode crescer. Nesta lição você vai ver `&str` como valor de saída de etiquetas fixas; na lição 2 você vai estudar por que nem todos os textos podem ser copiados e por que os dois tipos são distinguidos. Por ora, guarde esta regra: um texto fixo escrito no código costuma ser `&str`; um texto lido, construído ou armazenado costuma acabar como `String`.

O `revisor` torna explícito o seu vocabulário numérico. O tempo limite é guardado como `u64` e o código HTTP como `Option<u16>`. Você ainda não precisa dominar `Option`; a lição 3 vai explicar por que ele substitui o `nil`. Hoje basta observar que o tipo descreve uma restrição da realidade: pode não haver código HTTP se nenhuma resposta chegou.

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

O tipo não é documentação decorativa. `ms: u64` impede atribuir-lhe uma string; `codigo: Option<u16>` impede tratar a ausência de resposta como se fosse automaticamente um `200`; `servicio: String` indica que o nome é um texto que o programa possui. O Rust vai usar essa informação durante toda a compilação.

### Funções, parâmetros e valores de retorno

Uma função se declara com `fn`, tem um nome, parâmetros entre parênteses e um corpo entre chaves. Os parâmetros sempre levam tipo: `fn doble(x: i32)` diz tanto o nome do dado quanto o que a função pode receber. Se ela devolve algo diferente de `()`, isso é indicado depois de uma seta: `-> i32`. Essa assinatura é um contrato curto e verificável. Quem chama a função sabe o que deve entregar e o que vai obter; o compilador confere as duas pontas.

Diferentemente do Go, o Rust escreve o tipo depois do nome do parâmetro, não antes. No Go você escreveria `func doble(x int) int`; no Rust, `fn doble(x: i32) -> i32`. A diferença visual deixa de importar depois de algumas funções. O importante é que nas duas linguagens a assinatura faz parte do design: não é um comentário nem uma convenção informal.

Uma função pequena não deve assumir trabalho que não lhe cabe. `doble` recebe um número e devolve outro; não imprime, não lê arquivos e não modifica estado externo. Essa separação parece básica, mas prepara o terreno para o `revisor`: uma função que classifica milissegundos pode ser testada com três números sem iniciar um cliente HTTP nem abrir uma configuração. Quando o programa crescer, dividir a lógica em funções com entradas e saídas claras será uma forma de mantê-lo compreensível.

**Fig. 1.5** | Tudo é uma expressão.

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

`main` também é uma função. Num programa executável ela começa sem parâmetros e não precisa declarar retorno se apenas termina. Já `doble` promete um `i32`, então o último valor do seu corpo deve ser compatível com `i32`. Você pode usar `return x * 2;`, mas essa não é a forma habitual para o último valor de uma função. O Rust prefere a expressão final porque deixa visível qual resultado o corpo produz.

Os parâmetros são passados de maneiras diferentes conforme o tipo e a intenção. Os tipos escalares como `i32`, `u64` e `bool` são copiados de forma barata; receber `ms: u64` não impede quem chama de continuar usando a sua medida. Com `String`, vetores e structs mais complexas aparecerão as regras de movimento e empréstimo da lição 2. Não as antecipe resolvendo tudo com cópias. Por ora, use parâmetros escalares para praticar assinaturas limpas e reconheça que o `&str` de uma etiqueta fixa tem uma vida diferente da de um `String` que é construído.

O projeto real tem uma função pequena que converte uma configuração textual em dados do programa. Embora use bibliotecas que você vai estudar depois, sua assinatura mostra o padrão essencial: recebe uma entrada, devolve um resultado e seu corpo termina com uma expressão `Ok(...)`.

<!-- verificar:extracto:src/config.rs -->
```rust
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}
```

Você ainda não precisa destrinchar `Result`, `?` nem `yaml_serde`; eles chegam na lição 4. O que você já pode ler é a forma: `ruta` entra como uma visão de texto, a função promete devolver uma lista de serviços ou um erro, `txt` é um valor local imutável e `Ok(...)` é o resultado final. As assinaturas deixam você entender a fronteira de uma função mesmo antes de conhecer todos os seus detalhes internos.

### Expressões, instruções e o ponto e vírgula

No Rust, muitas construções produzem um valor. Uma operação aritmética como `x * 2` produz um número; um bloco entre chaves pode produzir o último valor que contém; um `if` pode produzir um de dois valores; e um `loop` pode terminar com um valor enviado por `break`. Essas construções são chamadas de expressões.

Uma instrução (statement) realiza uma ação, mas não produz um valor útil. Uma declaração `let x = 7;` é uma instrução. Também é uma instrução uma expressão à qual você acrescenta ponto e vírgula. O valor de uma instrução é `()`, chamado de tipo unidade. Você pode pensar em `()` como “não há resultado para entregar”. Não é um erro nem um valor nulo: é um tipo real que aparece quando uma operação é usada apenas pelo seu efeito.

O ponto e vírgula determina essa diferença em lugares importantes. Na figura 1.5, o bloco atribuído a `cuadrado` termina com `t` sem ponto e vírgula, e por isso o bloco produz o valor de `t`. A função `doble` termina com `x * 2` sem ponto e vírgula, e por isso devolve esse `i32`. Se você acrescentar `;`, a operação é executada e seu resultado é descartado. Então o corpo da função produz `()`, mas a assinatura exige `i32`.

**Fig. 1.6** | O ponto e vírgula a mais.

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

Esse erro é desconcertante uma vez e muito útil depois. O compilador não está dizendo que a multiplicação seja inválida; está dizendo que a função prometeu `i32` e acabou produzindo `()`. Leia as duas partes da mensagem: `expected i32, found ()` identifica a contradição, e a ajuda propõe remover o ponto e vírgula. Não acrescente um `return` sem entender por quê; neste caso o problema é que você descartou o valor correto.

O estilo de expressão faz com que transformações pequenas fiquem compactas e claras. Você pode calcular um valor intermediário num bloco, manter as variáveis locais dentro desse bloco e entregar apenas o resultado. Isso reduz escopos desnecessários e evita nomes temporários que continuam vivos quando já não significam nada. Não transforme cada linha numa expressão complicada: a legibilidade continua sendo o critério. Um bloco com dois ou três passos bem nomeados costuma ser mais claro do que uma linha engenhosa.

Em `revisar.rs`, a classificação de uma resposta usa condições dentro de um `match`; o resultado de cada ramificação é um `Estado`. Embora o `match` seja estudado a fundo na lição 3, o padrão já é familiar: cada caminho produz o valor que a função prometeu.

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

A parte que corresponde a esta lição são as condições `if` e a ideia de que uma construção de controle termina num valor. A parte nova são `match`, `Result` e as variantes de `Estado`. Você ainda não precisa copiá-los; apenas reconheça que a sintaxe que você pratica com números acabará tomando decisões reais sobre serviços.

### `if`, `loop`, `while` e `for`

`if` avalia uma condição que deve ser `bool`. O Rust não considera que `0`, uma string vazia ou uma referência nula sejam falsos de forma automática. Escreva uma comparação ou use uma variável booleana. Essa decisão evita condições acidentais e deixa evidente qual propriedade você está perguntando: `ms > UMBRAL_LENTO_MS` comunica uma regra; `if ms` não teria significado.

Quando `if` é usado para escolher um valor, suas ramificações devem devolver o mesmo tipo. Você não pode devolver `"rápido"` numa ramificação e `1000` em outra, porque a variável que recebe o resultado deve ter uma representação consistente. Essa restrição é exatamente o tipo de decisão que se torna útil num relatório: uma classificação sempre será texto, um código de saída sempre será um inteiro adequado e um estado sempre será uma variante do mesmo tipo.

`loop` inicia um laço infinito. Parece uma ferramenta extrema, mas é adequada quando você não conhece de antemão o número de iterações e a saída natural é `break`. Diferentemente de outras linguagens, `break valor` pode dar o resultado de um `loop`. Isso serve quando o laço procura ou calcula algo; o valor encontrado sai diretamente como resultado da expressão.

`while condicion` repete enquanto a condição for verdadeira. Use-o quando o avanço depende de um estado que você controla: decrementar uma contagem, ler até uma condição ou tentar de novo sob uma regra explícita. Certifique-se de que o corpo pode mudar o estado que torna a condição falsa. Um `while` cujo contador nunca é atualizado é um laço infinito disfarçado.

`for` é a opção normal para percorrer uma coleção ou um intervalo. O Rust não tem o estilo tradicional `for inicialização; condição; atualização` do C, do Java ou do Go. Em vez disso, percorre algo que sabe entregar seus elementos um a um (no Rust, algo que implementa `IntoIterator`): um intervalo como `0..10`, uma lista, um array ou um iterador. Essa forma elimina grande parte do código de índices e reduz erros de limite.

**Fig. 1.7** | Os laços.

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

Os intervalos são uma fonte comum de erros de limite. `0..10` inclui `0` e exclui `10`, por isso tem dez valores: do zero ao nove. `0..=10` inclui os dois extremos e tem onze valores. Para percorrer as posições de um array de comprimento dez, quase sempre você quer `0..10` ou, melhor ainda, percorrer diretamente os elementos. Use o intervalo inclusivo apenas quando o limite final for parte da regra e precisar aparecer.

A linha `for s in &servicios` leva uma referência à lista. Isso permite ler cada elemento sem entregar a propriedade de `servicios`. A diferença completa entre `servicios`, `&servicios` e `&mut servicios` é o tema da lição 2, mas você pode adotar desde hoje uma regra provisória: se você só quer olhar uma coleção e conservá-la, percorra-a por referência. O compilador vai impedir usos inseguros quando você conhecer as regras de empréstimo.

O `revisor` valida uma lista com um `for`. A função não precisa saber quantos serviços chegaram: ela os toma um por um. `enumerate()` acrescenta o índice para poder comparar o serviço atual com os anteriores. Embora a expressão completa pareça avançada, seu fluxo é o mesmo da figura 1.7: percorrer, verificar uma condição e terminar com um resultado.

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

Aqui `s.timeout_ms > 0` é uma condição booleana como as que você já usou. A diferença é que, em vez de imprimir uma etiqueta, `anyhow::ensure!` interrompe a validação com um erro se a condição for falsa. A lição 4 vai explicar `Result` e esse tipo de tratamento de erros. A lição 2 vai explicar as referências de `&[Servicio]`. Você já consegue ler a intenção sem conhecer cada detalhe: todos os serviços devem ter uma URL com esquema, um nome não repetido e um limite maior que zero.

## O erro que você vai ver

### `E0308`: tipos que não coincidem

`E0308` significa que o Rust esperava um tipo num ponto do programa e encontrou outro. Não é uma mensagem vaga: leia-a como uma frase com duas partes. Primeiro identifique o lugar onde a expectativa foi fixada; depois identifique o valor que contradiz essa expectativa. Na figura 1.3, a anotação `let b: i64` fixa que `b` será `i64`; a variável `a` é `i32`; por isso a atribuição falha.

A correção nem sempre será `as`. Para converter de `i32` para `i64`, o alargamento é seguro e `as i64` comunica a intenção. Para converter de um número grande para um pequeno, ou de um texto para um inteiro, você precisa decidir o que fazer quando o valor não cabe ou não tem formato válido. Essas conversões serão tratadas com resultados que podem falhar. A boa prática é resolver a discrepância na fronteira entre domínios, não converter valores repetidamente dentro de cada função.

A figura 1.6 produz o mesmo código `E0308`, mas por uma causa diferente: a função declara `-> i32` e seu último elemento é uma instrução cujo valor é `()`. Essa diferença ilustra por que você não deve resolver erros apenas pelo número. O código agrupa uma família de diagnósticos; as linhas apontadas e as palavras `expected` e `found` contam a história concreta.

Quando você vir `expected i32, found ()`, faça estas perguntas: a função prometeu um retorno? o último valor tem ponto e vírgula? uma ramificação de `if` não devolve o mesmo que a outra? pus `println!` como último elemento quando precisava produzir um valor? Com essa ordem você normalmente encontra o problema sem procurar respostas ao acaso.

### Ler uma sugestão sem obedecê-la às cegas

O Rust costuma oferecer uma seção `help:`. É uma proposta contextual, não uma ordem. Na figura 1.3 ela sugere `a.into()`, que também pode converter o valor porque existe uma conversão conhecida entre os dois tipos. A figura 1.4 conserva `as i64` porque é a forma que se quer ensinar para uma conversão numérica explícita e simples. Em outros casos, a sugestão pode ser `clone()`, acrescentar uma referência ou mudar uma assinatura. Antes de aceitá-la, pergunte a si mesmo que custo, propriedade ou comportamento ela está introduzindo.

A mensagem também termina com `rustc --explain E0308`. Esse comando abre uma explicação geral do código de erro instalada junto com o seu compilador. Use-o quando o diagnóstico local não bastar, mas comece pelo arquivo, pela linha e pelas colunas que o compilador já lhe mostrou. Quase sempre elas contêm mais informação específica sobre o seu programa do que uma busca geral.

## O que se faz errado

### Declarar tudo como `mut`

Declarar cada variável com `mut` para “ter liberdade” apaga informação. Se um nome não muda, o leitor não deve ter de rastrear a função inteira para descobrir isso. Além disso, o compilador avisa quando `mut` não é necessário. Declare como mutável unicamente o que o algoritmo modifica, como `x` numa contagem regressiva ou `salida` ao construir um relatório.

### Usar `as` para silenciar erros de tipos

Uma conversão com `as` pode estar correta, mas não é uma cura universal. Converter um `u64` grande para `u16` pode perder dados; converter um inteiro com sinal para um sem sinal pode produzir um valor surpreendente. Defina o que cada número representa e converta uma vez, na borda onde você muda de domínio. Se a conversão pode falhar, o programa deve expressar isso em vez de escondê-lo.

### Usar números sem unidades nem nome

Um `if ms > 1000` funciona, mas obriga a lembrar o que `1000` representa. São milissegundos, segundos ou bytes? Use uma constante como `UMBRAL_LENTO_MS` quando o valor for uma regra do negócio. Para valores locais óbvios, um literal pode servir; para uma política que será repetida ou mudará, um nome evita erros e melhora a leitura.

### Acrescentar ponto e vírgula à última expressão por reflexo

Em muitas linguagens cada linha termina com ponto e vírgula ou a convenção convida a usá-lo. No Rust, o último ponto e vírgula de uma função ou bloco muda o seu valor para `()`. Não decore uma exceção; reconheça a regra: uma expressão final sem ponto e vírgula pode ser o resultado. Se um bloco existe para calcular algo, revise o que ele deixa como última expressão.

### Escrever `for i in 0..lista.len()` quando você só precisa dos elementos

Percorrer índices funciona, mas acrescenta uma forma desnecessária de errar. Se você só precisa de cada serviço, escreva `for servicio in &servicios`. Use `enumerate()` quando o índice fizer parte real da lógica, como na validação do `revisor`. Use índices diretos quando precisar acessar posições concretas e puder justificar os limites.

### Usar `loop` quando o número de passos já é conhecido

Um `loop` com várias condições de saída pode estar correto, mas se você tem uma coleção ou um intervalo conhecido, `for` expressa melhor a intenção. Se depende de uma condição que muda, `while` costuma mostrar o critério de término com mais clareza. Reserve `loop` para processos que realmente esperam uma saída por meio de `break`, como um leitor de eventos ou uma busca que termina ao encontrar o dado.

## Exercícios

### Exercício 1 — Classifique uma resposta

Escreva `fn clasificar(ms: u64) -> &'static str`. Ela deve devolver `"rápido"` se o tempo for menor ou igual a `1000`, `"lento"` se for maior que `1000` e menor ou igual a `5000`, e `"timeout"` se for maior. Use um `if` como expressão: não use `return`. A partir do `main`, imprima a classificação de `700`, `1500` e `6000`, uma por linha.

Antes de ver a solução, verifique que as três ramificações devolvem o mesmo tipo. A assinatura não precisa criar um `String`: as três etiquetas são literais fixos e por isso podem ser `&'static str`.

### Exercício 2 — Some sem consumir a lista

Escreva `fn sumar(valores: &[i32]) -> i32` que use `for` para somar um slice. A partir do `main`, crie `let valores = vec![3, 5, 8];`, imprima o resultado e depois imprima o comprimento de `valores`. A segunda impressão deve compilar: demonstra que o percurso não consumiu o vetor.

Faça com que apenas o acumulador seja mutável. Não torne o vetor mutável: você não está acrescentando, removendo nem modificando os elementos dele.

### Exercício 3 — Um relatório mínimo do revisor

Declare `const UMBRAL_LENTO_MS: u64 = 1000;` e escreva `fn etiqueta(ms: u64) -> &'static str` que devolva `"OK"` até o limite e `"LENTO"` acima dele. No `main`, use um array com `[120_u64, 1000, 1500]` e um `for` para imprimir exatamente estas linhas:

```text
120ms: OK
1000ms: OK
1500ms: LENTO
```

Depois mude o tipo do array para `i32` sem mudar a assinatura de `etiqueta`. Leia o `E0308`, corrija-o de forma explícita e explique com suas palavras por que o Rust não fez a conversão por você.

## Soluções

### Solução 1

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

O `if` completo é a expressão final da função. Cada ramificação devolve um literal de tipo `&'static str`, então a assinatura e o resultado coincidem. A ordem importa: a segunda condição só é avaliada se a primeira foi falsa, por isso não é preciso repetir `ms > 1000`.

### Solução 2

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

`valores` recebe uma referência a um slice, de modo que a função observa os números sem ficar com o vetor de quem chamou. Dentro do `for`, `valor` é uma referência a cada `i32`; a soma funciona porque `i32` implementa a soma com uma referência a outro `i32` (`total += valor`), sem que você precise escrever `*valor`. A última expressão, `total`, entrega o resultado sem ponto e vírgula.

### Solução 3

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

O sufixo `_u64` no primeiro literal fixa o tipo do array. Os demais elementos devem ser do mesmo tipo, então o Rust os interpreta também como `u64`. A constante expressa tanto o valor quanto a unidade da regra. Se você mudasse o array para `i32`, teria de converter cada dado de maneira explícita ou mudar o contrato da função; as duas decisões têm significado e não devem acontecer por acidente.

## Como sei que consegui

- A partir de `programas/01-fundamentos`, `rustc --edition 2024 -D warnings fig01_01.rs && ./fig01_01` imprime `x = 5, y = 15, MAX = 100000` sem avisos.
- `rustc --edition 2024 fig01_03.rs` falha com `error[E0308]`, e você consegue apontar que `b` espera `i64` enquanto `a` é `i32`.
- `rustc --edition 2024 fig01_06.rs` falha com `error[E0308]`, e você consegue explicar que o ponto e vírgula fez a função devolver `()`.
- `rustc --edition 2024 -D warnings fig01_07.rs && ./fig01_07` imprime os dois intervalos, os três serviços e termina com `x = 0, r = 42`.
- `rustc --edition 2024 -D warnings fig01_02.rs && ./fig01_02` imprime os três resultados documentados e não reporta avisos.
- Você terminou as seções `variables`, `functions`, `if` e `primitive_types` do Rustlings, e consegue resolver os três exercícios sem copiar as soluções.

## Para ler mais

- [The Rust Programming Language, capítulo 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) — consultado em 2 de outubro de 2026.
- [The Rust Programming Language, capítulo 3: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) — consultado em 2 de outubro de 2026.
- [Documentação oficial de `i32` e dos tipos numéricos primitivos](https://doc.rust-lang.org/std/primitive.i32.html) — consultado em 2 de outubro de 2026.
- [Rustlings](https://rustlings.rust-lang.org/) — complete `variables`, `functions`, `if` e `primitive_types`; consultado em 2 de outubro de 2026.
