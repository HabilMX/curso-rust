# Lição 2 — Ownership

**Tempo:** 2 × 45 min.

**O que você constrói:** os programas que colidem de propósito com o compilador

**O que você aprende:** as três regras da propriedade, mover contra copiar, empréstimos `&` e `&mut`, as duas regras dos empréstimos, por que não existe coletor de lixo

## Ao terminar, você vai poder

- Explicar as três regras do ownership e usá-las para antecipar quando o Rust libera um valor.
- Distinguir uma cópia implícita de um movimento, e justificar quando usar `clone()`.
- Escolher se uma função deve receber um valor, uma referência `&T` ou uma referência mutável `&mut T`.
- Aplicar a regra de “muitas leituras ou uma escrita” e corrigir os erros `E0382` e `E0502`.
- Explicar por que o Rust não precisa nem de um coletor de lixo nem de chamadas manuais a `free`.
- Escrever uma função que receba `&str` e devolva uma porção emprestada do texto.
- Resolver os exercícios `move_semantics` e `primitive_types` do Rustlings.

## O porquê antes do como

A lição 2 é o ponto em que o Rust deixa de parecer simplesmente uma linguagem compilada com sintaxe diferente da do Go. As variáveis, as funções, os tipos e o fluxo de controle da lição anterior são reconhecíveis. O ownership (propriedade) muda uma pergunta que muitas linguagens escondem: quando um programa cria um dado na memória, quem deve destruí-lo e quando?

O `revisor` vai guardar nomes de serviços, URLs, mensagens de falha, listas e relatórios. Todos esses dados podem crescer em tempo de execução. O nome de um serviço lido de um YAML não tem tamanho conhecido quando você compila; precisa de memória dinâmica. A tabela final do relatório também é construída aos poucos. Em C ou C++, quem escreve o programa teria de reservar e liberar essa memória manualmente. Se liberar duas vezes, o programa pode se corromper. Se não liberar, perde memória. Se mantiver um ponteiro depois de liberá-la, pode ler uma região que já pertence a outra coisa.

O Go toma outra decisão. O programa pode criar dados e esquecer de liberar memória, porque o coletor de lixo observa quais objetos continuam alcançáveis e recupera os demais. Isso torna a escrita de programas mais direta, mas acrescenta um componente de execução que administra memória, decide quando trabalhar e consome recursos para encontrar objetos que já não servem. Na maioria dos programas em Go essa decisão é excelente: reduz a complexidade e evita erros graves.

O Rust busca outra combinação: memória segura sem coletor de lixo e sem liberação manual. Sua proposta é verificar, antes de gerar o executável, quem possui cada valor e quem pode acessá-lo. Se o compilador consegue demonstrar que um valor não será mais usado, insere a liberação adequada ao sair do seu escopo. Se não consegue demonstrar que uma referência continuará válida, rejeita o programa. Se detecta dois acessos incompatíveis a um mesmo dado, também o rejeita.

Isso não significa que o Rust “adivinha” o que você queria fazer. Pelo contrário: exige que a sua intenção seja visível nas assinaturas e nas atribuições. Uma função que recebe `String` toma o valor; uma que recebe `&str` apenas o consulta; uma que recebe `&mut String` pode modificá-lo durante um empréstimo exclusivo. A informação que no Go às vezes fica numa convenção, num comentário ou numa revisão de código, no Rust faz parte do tipo.

O preço é real. No início você vai escrever código que parece razoável, mas não compila. A reação normal é tentar acrescentar `.clone()` até o erro desaparecer. Às vezes uma cópia é a decisão correta; muitas vezes é um sinal de que a função pediu mais propriedade do que precisava. Aprender ownership consiste em deixar de tratar esses erros como obstáculos e começar a lê-los como perguntas de design: quem deve conservar este valor? por quanto tempo ele precisa viver? quem pode modificá-lo?

O capítulo 4 de *The Rust Programming Language* explica ownership, referências, empréstimos e slices. Leia-o por inteiro durante esta lição. Não tente decorar todas as mensagens do compilador. O objetivo é construir um modelo mental simples: cada valor tem um dono; mover entrega essa responsabilidade; emprestar permite usar um valor sem entregar a responsabilidade; e as regras de empréstimo impedem que uma leitura veja um dado enquanto alguém o está mudando.

O projeto real já usa essa ideia, ainda que você não tenha escrito todas as suas peças. `Servicio` possui seus campos `String` porque o revisor precisa guardar um nome e uma URL além da função que os leu. Já as funções que imprimem um relatório recebem referências aos serviços e aos seus estados: só precisam consultá-los, não tomar posse deles. Mais adiante, nas lições 3, 4 e 5, essas mesmas decisões vão aparecer em structs, coleções, erros e tempos de vida (lifetimes).

## Os conceitos

### Propriedade, escopo e liberação determinística

O ownership se resume em três regras.

1. Cada valor no Rust tem um dono.
2. Só pode haver um dono de um valor por vez.
3. Quando o dono sai de escopo, o Rust libera o valor.

Um escopo é a parte do programa onde um nome existe. As chaves delimitam escopos, como na lição 1. A diferença agora é que sair de um escopo não apenas torna uma variável inacessível: também determina quando o valor associado é destruído. Para tipos que reservam recursos, o Rust chama `drop` automaticamente. `String`, por exemplo, libera o bloco de memória onde guarda seus caracteres.

**Fig. 2.1** | O escopo de uma variável.

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

`String::from("hola")` cria um `String` que possui memória dinâmica. Enquanto `s` está no escopo interno, pode ser usado para imprimir o texto. Ao chegar na chave de fechamento, `s` deixa de existir e o Rust libera sua memória. Você não escreveu `free`, não calculou tamanhos e não esperou que um coletor decidisse passar. O compilador insere o trabalho necessário porque conhece o alcance de `s`.

A palavra “propriedade” não descreve a localização física de um dado; descreve responsabilidade. O valor pode estar na pilha (stack), no heap ou conter referências a outros valores. O importante é que o Rust consegue identificar um dono responsável por limpar o recurso. Muitos tipos simples, como `u64`, `bool` ou `char`, cabem inteiramente na pilha e não exigem liberar nada especial. Um `String`, um `Vec<T>` ou um `HashMap<K, V>` administram memória dinâmica e, sim, precisam de um fim ordenado.

Essa liberação se chama determinística porque ocorre num ponto sobre o qual você pode raciocinar ao ler o programa: no final do escopo, a menos que o valor tenha sido movido antes. É importante distingui-la da administração manual. Você não escolhe quando chamar `drop` para cada valor nem deve fazê-lo em condições normais. O Rust conhece o tipo e gera a liberação correta. Se um tipo contém outros valores, seu destrutor também libera o que corresponder dentro dele.

Essa garantia não significa que o Rust proíba todo vazamento de memória imaginável. Por exemplo, é possível conservar dados com ciclos de referências contadas ou usar deliberadamente mecanismos que evitem a liberação. A garantia central é outra: o código seguro não pode usar depois um valor que o Rust já liberou, nem liberar duas vezes a mesma memória. Para um programa como o revisor, isso elimina uma classe inteira de erros sem acrescentar um coletor de lixo em tempo de execução.

No Go, uma variável local também deixa de ser útil quando sai do seu bloco, mas a memória que ficou inacessível é recuperada depois, quando o coletor determinar. No Rust, o fim do escopo é parte direta do modelo de recursos. Essa diferença não torna automaticamente melhor uma ou outra linguagem. O Go simplifica muitas aplicações; o Rust permite saber com mais precisão quando são liberados memória, arquivos, sockets ou cadeados.

O revisor possui o texto de que precisa. Um serviço não pode depender de que continue viva uma variável temporária do parser de YAML: por isso seus campos são `String`, não referências a texto temporário.

<!-- verificar:extracto:src/modelo.rs -->
```rust
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}
```

`nombre` e `url` são propriedade de cada `Servicio`. Quando o vetor de serviços for destruído, seus elementos serão destruídos; quando cada elemento for destruído, seus `String` serão destruídos; e cada `String` liberará sua memória. Não existe uma lista manual de recursos a limpar. A estrutura dos valores descreve também a estrutura da responsabilidade.

### Mover, copiar e clonar

A segunda regra diz que um valor só tem um dono por vez. Por isso uma atribuição nem sempre significa copiar. Com tipos que possuem recursos, o Rust costuma mover o valor: a nova variável se torna o dono e o nome anterior deixa de poder ser usado.

Isso surpreende se você vem do Go. No Go, atribuir um `string` a outra variável copia seu cabeçalho imutável e as duas variáveis podem ser lidas. Atribuir um struct copia seus campos; se ele contém um slice ou um map, as duas cópias podem continuar apontando para dados compartilhados. No Rust, o compilador exige que essa relação seja explícita, porque uma cópia superficial de um tipo dono pode deixar dois valores tentando liberar o mesmo recurso.

**Fig. 2.2** | As duas saídas: copiar ou emprestar.

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

`a.clone()` cria um segundo `String`, com sua própria memória e seus próprios caracteres. Por isso `a` e `b` podem viver de forma independente. A cópia tem um custo proporcional ao tamanho do texto: copiar “hola” é pequeno; copiar uma resposta HTTP grande ou uma lista de milhares de serviços pode não ser. O Rust torna visível esse custo com o método `clone()`.

Você não deve interpretar isso como uma proibição de clonar. Uma cópia é correta quando o programa de fato precisa de dois valores independentes: guardar um nome para o relatório e outro para enviá-lo a uma tarefa, conservar uma configuração original antes de transformá-la ou separar dados que viverão tempos diferentes. O problema aparece quando `clone()` é usado mecanicamente para silenciar um erro sem responder quem precisa possuir o dado.

O terceiro nome, `c`, é uma referência. `&a` não copia os caracteres nem entrega a propriedade. Cria um empréstimo (borrow) somente leitura. Por isso é possível imprimir `a`, `b` e `c`: `a` continua sendo o dono; `b` é dono de outra cópia; `c` apenas aponta temporariamente para `a`.

Os tipos que implementam o trait `Copy` se comportam de modo diferente. Inteiros, booleanos, caracteres e tuplas compostas exclusivamente por valores `Copy` são copiados implicitamente, porque duplicá-los é barato e eles não exigem liberar memória. Se você atribuir `let b = a` quando `a` é um `u64`, pode usar os dois nomes. Não é que o ownership desapareça: cada variável recebe a sua própria cópia do valor.

`String` não implementa `Copy` porque copiar implicitamente seus três dados internos — ponteiro, comprimento e capacidade — produziria dois administradores para o mesmo bloco do heap. O Rust poderia copiar também os caracteres, mas então cada atribuição potencialmente esconderia trabalho custoso. Por isso diferencia movimento de clonagem.

O revisor clona somente quando precisa construir uma saída que deve possuir texto próprio. O relatório JSON não pode conservar referências a serviços locais dentro de uma função que já terminou. Por isso converte o nome emprestado do serviço num `String` independente.

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

Aqui `servicios` e `estados` são emprestados à função de relatório. `s.nombre.clone()` e `motivo.clone()` são decisões necessárias: `EstadoJson` deve sobreviver como elemento de `lineas` e depois ser convertido em JSON. O código não clona por medo do compilador; clona porque o resultado tem dono próprio.

### Referências imutáveis: emprestar para ler

Uma referência é uma forma de permitir acesso a um valor sem transferir sua propriedade. Escreve-se `&T`: “uma referência a um `T`”. Se você tem um `String` e uma função só precisa conhecer o seu comprimento, passar `&String` evita criar uma cópia e evita que a função consuma o texto.

**Fig. 2.3** | Emprestar para ler.

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

A função `largo` recebe uma referência. Dentro dela, `s.len()` consulta o comprimento, mas não pode ficar com o `String` nem modificá-lo. Quando a chamada termina, o empréstimo termina e o dono original continua sendo a variável `s` do `main`. Isso explica por que a última linha pode imprimir tanto o texto quanto o seu comprimento.

O exemplo conserva `&String` porque mostra diretamente o contraste entre um `String` dono e uma referência a ele. Numa API geral convém receber `&str` quando você só precisa ler texto. `&str` é uma visão de uma sequência UTF-8; aceita tanto um literal quanto uma referência a `String`. A lição 4 vai aprofundar essa distinção, mas desde já você pode usar uma regra prática: guarde como `String` o texto que você possui; receba como `&str` o texto somente de leitura.

Uma referência não é uma cópia do valor. Tem uma vida útil limitada pelo valor para o qual aponta. O Rust não permite devolver uma referência a uma variável local que desaparecerá ao sair de uma função, nem conservar uma referência quando seu dono já foi movido. Essa parte da análise é conhecida como verificação de empréstimos ou *borrow checking*.

A referência torna visível o contrato de uma função. Uma assinatura que recebe `String` comunica “preciso tomar este texto”. Uma que recebe `&str` comunica “só preciso lê-lo”. No Go, passar um `string` é barato porque sua representação é copiada; passar uma struct grande por valor ou por ponteiro exige ler a documentação e conhecer a implementação. O Rust faz dessa diferença parte da assinatura.

As funções do relatório recebem slices emprestados. Não consomem o vetor de serviços nem o vetor de estados, porque o `main` ainda precisa deles para decidir o código de saída. A assinatura expressa essa intenção sem comentários adicionais.

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

`&[Servicio]` significa “slice emprestado de serviços”. Um slice empresta uma parte contígua de uma coleção e sabe quantos elementos contém. A função pode percorrê-lo, consultar nomes e calcular a largura da tabela, mas não pode esvaziar o vetor, acrescentar serviços nem ficar com eles. A mesma decisão vale para `&[Estado]`.

### Referências mutáveis: emprestar para modificar

Uma referência mutável se escreve `&mut T`. Serve quando uma função precisa modificar um valor cuja propriedade continua sendo de quem chama. Para criá-la, você precisa de duas coisas: o dono deve ser declarado com `mut`, e o empréstimo deve ser escrito como `&mut`.

**Fig. 2.4** | Emprestar exclusivamente para modificar.

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

`servicio` é mutável porque seu conteúdo vai mudar. `agregar_puerto` não recebe o `String` por valor: recebe um empréstimo exclusivo e acrescenta caracteres ao mesmo texto. Ao terminar a chamada, o empréstimo termina e o `main` volta a usar o dono para imprimi-lo.

A exclusividade é a condição importante. Durante um empréstimo `&mut`, ninguém mais pode ler nem modificar o mesmo dado por meio de outra referência. Não é uma limitação arbitrária: se uma parte do programa altera uma string enquanto outra supõe que a está lendo de forma estável, o resultado pode depender da ordem de execução. Em programas concorrentes, essa situação é uma corrida de dados (data race).

As duas regras de empréstimo são:

1. Você pode ter qualquer número de referências imutáveis a um valor.
2. Você pode ter exatamente uma referência mutável a um valor, ou referências imutáveis, mas não as duas ao mesmo tempo.

A forma breve de lembrá-las é: muitas leituras ou uma escrita. Uma leitura não altera o dado e pode ser compartilhada. Uma escrita precisa de exclusividade porque poderia alterar qualquer parte do valor.

O Rust também analisa o último uso real de uma referência. Você não precisa necessariamente esperar até a chave final do bloco para pedir um empréstimo mutável. Se uma referência imutável não será mais usada, o Rust pode considerar que o empréstimo dela terminou. Isso é conhecido como empréstimos não léxicos. Você não deve depender disso para escrever código confuso, mas isso explica por que separar uma leitura e uma modificação em passos claros costuma compilar.

No revisor, a tabela é construída com uma variável mutável. A propriedade de `salida` continua em `tabla`, mas `push_str` precisa de um empréstimo mutável temporário para acrescentar cada linha. Ao sair da função, `tabla` devolve o `String` completo e a propriedade passa a quem chamou.

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

`push_str` modifica `salida`; por isso a variável é declarada com `mut`. Já `s` e `e` são referências de leitura aos dados de cada linha. O compilador permite as duas coisas porque a função modifica o relatório novo, não os serviços nem os estados que está consultando.

### Slices, `&str` e referências que devolvem referências

O ownership não obriga a copiar quando você quer obter uma parte de um valor. Uma função pode receber uma referência e devolver outra referência a uma parte da mesma informação, desde que o Rust possa verificar que a saída não viverá mais que a entrada. Esse padrão aparece com slices de arrays, slices de vetores e `&str`.

**Fig. 2.5** | Devolver uma visão emprestada de texto.

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

`primera_palabra` não constrói um `String` novo. Devolve uma visão de uma parte de `s`. Por isso é eficiente: não copia os caracteres. Também tem uma limitação saudável: `palabra` não pode sobreviver a `texto`, porque aponta para dentro dele. Se `texto` for modificado de uma maneira que mude seu armazenamento, uma referência antiga poderia deixar de ser válida; o Rust evita que você use as duas coisas de maneira incompatível.

A assinatura `fn primera_palabra(s: &str) -> &str` usa uma regra de inferência de tempos de vida (lifetimes). O compilador entende que a referência de saída está vinculada à referência de entrada. Na lição 5 você vai ver os casos em que deve escrever uma anotação como `'a`; por ora fique com a ideia importante: a função não possui a palavra devolvida, então não pode prometer que ela existirá por mais tempo que o texto emprestado.

O revisor usa lifetimes explícitos quando monta linhas que apenas emprestam dados de dois slices. Não duplica cada serviço e cada estado antes de ordená-los; conserva referências válidas enquanto os vetores originais continuarem vivos.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

A anotação `'a` diz que as referências dentro de `Fila` não podem viver mais que os slices emprestados a `ordenadas`. `filas` é dono do vetor de referências, mas não dos serviços nem dos estados. Essa distinção é a base de muitos programas Rust eficientes: possuir a coleção não implica possuir todos os dados para os quais ela aponta.

## O erro que você vai ver

### E0382: usar um valor depois de movê-lo

O programa a seguir não compila de propósito. A atribuição `let b = a` move o `String` de `a` para `b`. A última linha tenta pedir emprestado `a` para imprimi-lo, mas `a` já não é dono nem pode ser emprestado.

**Fig. 2.6** | Mover, não copiar.

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

`E0382` significa que você tentou usar um valor depois de ter transferido sua propriedade. A mensagem aponta três lugares: onde `a` nasceu, onde ocorreu o movimento e onde você tentou usá-lo de novo. Essa sequência é mais útil do que decorar o código do erro: siga as setas e pergunte quem é o dono depois de cada linha.

Há três correções possíveis, e elas não são intercambiáveis. Se você já não precisa de `a`, imprima `b`. Se precisa de dois valores independentes, use `a.clone()` e aceite o custo da cópia. Se a segunda parte só precisa ler o valor, mude o design para emprestar `&a` em vez de movê-lo. A terceira opção costuma ser a melhor quando você escreve funções auxiliares para o revisor.

O aviso sobre `b` aparece porque o programa não chega a usá-lo. Não é o erro principal; é consequência de o exemplo usar `a` de propósito para provocar o `E0382`. Os programas que devem compilar no curso são verificados com `-D warnings`, então uma variável sem uso também se torna um problema que você deve corrigir.

### E0502: pedir escrita enquanto existem leituras

O erro a seguir representa a segunda regra de empréstimo. `r1` e `r2` são referências imutáveis vivas porque são usadas no `println!` final. Enquanto essas leituras existirem, o Rust não pode criar `r3`, uma referência mutável ao mesmo `String`.

**Fig. 2.7** | Leituras e escrita ao mesmo tempo.

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

`E0502` significa que você pediu um empréstimo mutável enquanto existe um empréstimo imutável ativo. O Rust não supõe que “com certeza não vai acontecer nada”. Um empréstimo mutável poderia substituir, encurtar, esvaziar ou realocar o conteúdo de `s`, de modo que as referências de leitura já não teriam uma visão coerente.

A correção não é copiar `s` por reflexo. Primeiro decida se você realmente precisa ler e modificar ao mesmo tempo. Se não, termine as leituras antes de pedir a escrita: imprima ou calcule com `r1` e `r2`, deixe de usá-las e depois crie a referência mutável. Se precisa conservar informação de leitura enquanto modifica, guarde uma cópia pequena do dado necessário, como um comprimento ou uma flag, não necessariamente uma cópia completa da estrutura.

Esse erro é uma versão local de uma garantia que será decisiva na lição 7. No Go, duas goroutines que leem e escrevem dados compartilhados sem coordenação podem ter uma corrida de dados que só aparece ao executar. O Rust estabelece essas regras antes de falar de threads; quando você chegar a `Arc`, `Mutex` e canais, o mesmo modelo continuará protegendo os acessos compartilhados.

## O que se faz errado

### Usar `clone()` para calar cada erro

O compilador sugere `clone()` em várias mensagens porque é uma solução mecânica e segura: cria um valor independente. No entanto, a sugestão não conhece o design do seu programa nem o tamanho dos seus dados. Se você clona um `String` pequeno uma vez, provavelmente não importa. Se clona uma lista de serviços em cada função ou duplica corpos HTTP grandes num laço, acrescenta tempo e memória sem necessidade.

Antes de escrever `.clone()`, pergunte se a função só precisa ler. Se a resposta for sim, receba `&T` ou `&str`. Se ela deve modificar algo mas o dono deve conservá-lo, receba `&mut T`. Clone quando precisar de dois donos reais, como o JSON do relatório e os dados originais que continuam vivos para outra operação.

### Receber `String` por valor quando você só vai ler

Uma assinatura que recebe `String` faz com que quem chama entregue a propriedade. Isso pode ser correto para uma função que normaliza, consome ou guarda o texto. É desnecessário para uma função que só imprime, mede ou procura uma palavra. O custo nem sempre é uma cópia: às vezes quem chama pode mover o valor. O problema é que a assinatura reduz as opções de uso de quem chama.

Para funções de consulta, prefira `&str` se você trabalha com texto e `&T` se trabalha com outro tipo. A API será mais flexível: aceitará literais, `String` e porções de texto sem obrigar a criar novos donos. A lição 4 vai mostrar por que `&str` normalmente é uma fronteira pública melhor que `&String`.

### Pensar que `mut` significa “posso emprestar de forma mutável quando quiser”

`let mut s` permite modificar `s`, mas não elimina as regras de empréstimo. A mutabilidade pertence ao dono; a exclusividade pertence a cada empréstimo. Você pode declarar uma string mutável e mesmo assim receber `E0502` se existirem referências de leitura ativas. Também pode ter uma variável imutável que contenha uma referência mutável criada em outro contexto; os dois conceitos são distintos.

Use `mut` apenas quando o nome deve mudar ou quando você vai pedir um empréstimo mutável. Se uma variável nunca muda, tirar `mut` deixa a intenção mais clara e evita avisos do compilador.

### Brigar contra o empréstimo em vez de reduzir seu alcance

Uma referência vive até o seu último uso, não necessariamente até o final visual do bloco. Se o compilador não aceita um empréstimo mutável, revise onde a referência anterior é usada pela última vez. Muitas correções consistem em reorganizar algumas linhas: termine de ler, guarde o resultado de que precisa e só então modifique. Separar as fases de leitura e escrita melhora tanto a legibilidade quanto a compatibilidade com o borrow checker.

Não esconda o problema atrás de uma referência longa, de um `unsafe` ou de uma estrutura global. O revisor ainda é pequeno; se o modelo de propriedade se torna difícil de explicar, costuma ser sinal de que uma função tem responsabilidades demais ou de que um dado está sendo compartilhado mais do que o necessário.

### Confundir `String` com `&str`

`String` possui texto e pode crescer; `&str` é uma visão de texto que pertence a outra coisa. Converter um `&str` em `String` com `to_string()` ou `String::from()` é correto quando você vai guardá-lo. Fazer isso só porque uma função poderia receber uma referência é uma cópia evitável. Do outro lado, devolver `&str` quando o texto foi construído dentro da função não pode funcionar: o texto local desaparece ao terminar a função.

A pergunta útil é sempre a mesma: quem deve possuir estes caracteres depois desta operação? Se a resposta for “a estrutura que os guarda”, use `String`. Se for “ninguém novo; só preciso observá-los agora”, use `&str`.

## Exercícios

### Exercício 1 — Siga o dono

Leia as situações a seguir e escreva, antes de compilar, qual nome pode ser usado no final: uma atribuição de `u64`; uma atribuição de `String`; e uma atribuição de `String` seguida de `clone()`. Depois crie três arquivos pequenos e confira suas respostas com `rustc --edition 2024`.

Explique em uma frase por que o inteiro é copiado, por que o `String` é movido e por que o `clone()` produz dois donos. Não use `Copy` como uma palavra mágica: relacione-o com o custo e com a necessidade de liberar memória.

### Exercício 2 — Uma função que toma e outra que empresta

Escreva duas funções sobre um nome de serviço. A primeira deve receber um `String` por valor e devolver seu comprimento. A segunda deve receber `&str` e devolver o mesmo comprimento. No `main`, demonstre que depois de chamar a primeira função você já não pode imprimir o `String`, e que depois de chamar a segunda você pode imprimi-lo.

Primeiro deixe ativa a linha que provoca o `E0382` e leia o diagnóstico completo. Depois comente essa linha para que o programa compile. Não corrija a primeira função com `clone()`: o objetivo é observar a diferença entre tomar a propriedade e emprestar.

### Exercício 3 — Atualize um serviço sem mudar de dono

Escreva `fn agregar_puerto(etiqueta: &mut String)` para anexar `:443` a uma etiqueta. Declare um `String` mutável no `main`, empreste-o à função e confirme que o `main` consegue imprimir o resultado no final.

Depois provoque o `E0502`: crie uma referência somente leitura ao mesmo texto, use-a depois de pedir uma referência mutável e observe a linha apontada pelo compilador. Reordene o programa para que a leitura termine antes de modificar.

### Exercício 4 — A primeira palavra emprestada

Implemente `fn primera_palabra(s: &str) -> &str`. Ela deve devolver a primeira palavra de uma frase ou uma string vazia se só receber espaços. Teste-a com um `String` chamado `texto`, imprima o resultado e depois imprima também `texto`.

Faça os exercícios `move_semantics` e `primitive_types` do Rustlings. Em particular, não avance por tentativa e erro com `clone()`: em cada solução identifique se o Rust está pedindo para mover, copiar ou emprestar.

## Soluções

### Solução 1

Um `u64` implementa `Copy`, então depois de `let b = a` existem dois valores independentes e os dois nomes são utilizáveis. Um `String` não implementa `Copy`; a mesma atribuição move a propriedade para `b`, e por isso `a` deixa de ser utilizável. Se você escrever `let b = a.clone()`, `a` e `b` possuem dois blocos de texto distintos e os dois podem ser usados.

A prova não consiste em lembrar quais tipos implementam `Copy`, e sim em fazer uma previsão e conferi-la. Quando tiver dúvidas sobre um tipo próprio, o compilador dirá se ele implementa `Copy`. Nos structs do revisor que contêm `String`, suponha inicialmente que o valor é movido.

### Solução 2

A função que recebe `String` consome o argumento. Sua assinatura deve ser algo como `fn largo_tomando(s: String) -> usize`; depois da chamada, o `String` original já não está disponível. A função que recebe `&str` deve ser algo como `fn largo_prestando(s: &str) -> usize`; chame-a com `&nombre` e depois imprima `nombre`.

A diferença não está no número que devolvem, e sim no contrato de entrada. Para uma função que só calcula o comprimento, a segunda assinatura é a adequada. A primeira existe para que você observe explicitamente o movimento e para os casos reais em que uma função de fato precisa ficar com o valor.

### Solução 3

A solução é a da figura 2.4: o dono é declarado como `let mut servicio`, a função é chamada com `&mut servicio` e o resultado é impresso depois que a chamada termina. A função não devolve o `String` porque nunca o recebeu como propriedade.

Para corrigir o conflito de empréstimos, use por completo a referência de leitura antes de criar a referência mutável. O ponto importante não é pôr as duas referências em blocos artificiais, e sim tornar visível que a fase de leitura terminou antes da fase de escrita.

### Solução 4

A solução é a da figura 2.5. `split_whitespace()` ignora espaços iniciais e separa as palavras; `next()` produz um `Option<&str>`; `unwrap_or("")` devolve uma string vazia se não havia nenhuma palavra. O resultado é uma referência tirada da entrada, não um `String` novo.

O teste correto imprime primeiro a palavra e depois o `String` original. Isso demonstra que `primera_palabra` não tomou a propriedade de `texto`. Se você tentasse devolver uma referência a um `String` criado dentro da função, o Rust o rejeitaria porque esse `String` seria destruído ao terminar a chamada.

## Como sei que consegui

- [ ] `rustc --edition 2024 fig02_01.rs && ./fig02_01` imprime `hola`.
- [ ] `rustc --edition 2024 fig02_02.rs && ./fig02_02` imprime três vezes `hola` e sei explicar qual valor foi clonado e qual foi emprestado.
- [ ] `rustc --edition 2024 fig02_06.rs` falha com `E0382`, e sei explicar em que linha a propriedade foi movida.
- [ ] `rustc --edition 2024 fig02_07.rs` falha com `E0502`, e sei corrigi-lo terminando primeiro as leituras.
- [ ] `rustc --edition 2024 fig02_04.rs && ./fig02_04` imprime `catalogo:443`.
- [ ] `rustc --edition 2024 fig02_05.rs && ./fig02_05` imprime `primera: revisor`.
- [ ] Terminei `move_semantics` e `primitive_types` do Rustlings sem usar `clone()` como solução automática.
- [ ] Sei explicar em uma frase por que o Rust libera memória ao sair de escopo sem exigir um coletor de lixo.

## Para ler mais

- [The Rust Programming Language, capítulo 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html), consultado em 2 de outubro de 2026.
- [The Rust Programming Language, referências e empréstimos](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html), consultado em 2 de outubro de 2026.
- [Documentação oficial de `String`](https://doc.rust-lang.org/std/string/struct.String.html), consultado em 2 de outubro de 2026.
- [Rustlings](https://rustlings.rust-lang.org/), exercícios `move_semantics` e `primitive_types`, consultado em 2 de outubro de 2026.
