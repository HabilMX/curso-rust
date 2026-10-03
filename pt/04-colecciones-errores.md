# Lição 4 — Coleções e erros

**Tempo:** 2 × 45 min.

**O que você constrói:** a lista de serviços e o relatório, com erros de verdade

**O que você aprende:** `Vec`, `HashMap`, `String` contra `&str`, `Result` e `?`, `panic!` contra `Result`, `anyhow` e `thiserror`

**The Rust Book, capítulos 8 e 9.** Rustlings: `vecs`, `hashmaps`, `strings`, `error_handling`.

## Ao terminar, você vai poder

- Guardar uma lista de `Servicio` num `Vec<Servicio>` e percorrê-la sem brigar com o ownership.
- Escolher entre indexar uma coleção e usar métodos que devolvem `Option`.
- Guardar estados por nome num `HashMap<String, Estado>` e produzir um relatório ordenado.
- Explicar por que uma função normalmente recebe `&str`, enquanto um struct normalmente guarda `String`.
- Propagar erros de arquivos e de validação com `Result` e `?`.
- Distinguir um erro que quem usa o programa pode corrigir de um invariante quebrado que justifica `panic!`.
- Acrescentar contexto a um erro de aplicação com `anyhow` e reconhecer quando uma biblioteca precisa de `thiserror`.

## O porquê antes do como

Na lição 3 você modelou um serviço e os diferentes resultados de revisá-lo. Esse modelo ainda precisa viver em algum lugar. O programa recebe muitos serviços, não apenas um; precisa conservá-los enquanto consulta cada URL, associar cada resultado ao seu serviço e depois imprimir um relatório. Num programa pequeno você pode escrever duas ou três variáveis manualmente. No `revisor` real, essa estratégia deixa de servir no momento em que o arquivo YAML traz uma quantidade variável de serviços.

As coleções resolvem a parte de “quantos valores existem”, mas não resolvem por si sós o que significa um valor faltar nem o que o programa deve fazer quando algo externo falha. Um arquivo pode não existir, uma linha pode ter formato inválido, o nome de um serviço pode se repetir e uma URL pode não trazer esquema. Nenhum desses casos é raro nem impossível: todos ocorrem porque o programa recebe dados do exterior. A diferença importante é que o Rust pede que você represente essa possibilidade no tipo de retorno.

Um `Vec<Servicio>` representa uma lista ordenada de serviços. Um `HashMap<String, Estado>` representa uma associação por chave: dado o nome `"catalogo"`, busca seu estado. Um `String` é um texto que possui sua memória; um `&str` é uma visão emprestada de um texto que outra pessoa possui. E um `Result<T, E>` representa uma operação que pode terminar com um valor `T` ou com um erro `E`. Não são quatro temas desconexos: são as peças que fazem o estado do programa ter forma explícita.

O curso de Go constrói o mesmo revisor. No Go, ler uma chave inexistente de um `map` devolve o valor zero e obriga a lembrar a forma de dois resultados para distinguir “não existe” de “existe e vale zero”. O Rust escolhe outro contrato: `HashMap::get` devolve `Option<&V>`. A ausência aparece no tipo e não pode ser confundida com um estado real. O custo é que você precisa decidir o que fazer com `None`; o ganho é que essa decisão não pode ser esquecida sem que o código a torne visível.

Com os erros acontece algo parecido. O Go usa a convenção `if err != nil` depois de cada operação que pode falhar. O Rust usa `Result` e permite escrever a propagação com `?`. Não há uma resposta universal sobre qual estilo é mais legível: o Go repete uma estrutura muito explícita; o Rust concentra a mesma decisão num operador. O importante é que ambos obrigam a atender o erro. O Rust não converte um arquivo ausente numa string vazia nem deixa que uma conversão falha siga como se fosse válida.

Esta lição não quer que você use `unwrap()` para fazer o compilador se calar. Quer que você leia a assinatura de cada função como um contrato. Se uma função devolve `Option`, você deve pensar no que significa a ausência. Se devolve `Result`, deve decidir se o erro é resolvido ali, transformado ou propagado. Se recebe `&str`, só precisa ler texto; se recebe `String`, provavelmente pretende ficar com ele. As assinaturas descrevem o fluxo de dados e o fluxo de falhas antes de o programa ser executado.

## Os conceitos

### `Vec<T>`: uma lista dona de valores do mesmo tipo

`Vec<T>` é o vetor do Rust: uma coleção de tamanho variável que possui seus elementos. O parâmetro `T` diz que tipo de valores ela pode guardar. Um `Vec<Servicio>` só guarda serviços; um `Vec<Estado>` só guarda estados. Essa restrição não é um incômodo acidental. Permite ao compilador saber como deve administrar cada elemento, quais métodos são válidos e quais operações poderiam mover ou emprestar valores.

Um vetor vazio precisa de uma anotação de tipo se o Rust não consegue inferi-lo. Por isso a figura escreve `let mut v: Vec<Servicio> = Vec::new();`. O compilador ainda não viu nenhum elemento e não pode adivinhar o que haverá dentro. Se você criar o vetor com `vec![...]`, ou se o contexto já determinar o tipo, normalmente não precisa escrevê-lo. A palavra `mut` é necessária porque `push` altera a coleção: acrescenta um elemento e pode fazer o vetor reservar mais espaço.

O vetor é dono de cada `Servicio` que recebe. Em `v.push(s)`, a variável `s` é movida para o vetor. Isso aplica as regras de ownership da lição 2: depois de mover um `Servicio`, você não pode continuar usando a variável anterior como se ainda o possuísse. Não é uma cópia implícita. Se precisar conservar outra versão independente, você deve projetar a operação para emprestar, ou clonar deliberadamente quando o custo e a semântica o justificarem.

Há duas formas de ler um elemento. `&v[0]` produz uma referência e supõe que o índice existe. Se não existir, o programa entra em `panic!`. `v.get(0)` devolve `Option<&Servicio>`: `Some(referência)` se existe e `None` se está fora do intervalo. A segunda forma é adequada quando o índice vem de um arquivo, de um argumento, de uma requisição ou de qualquer dado que você não controla por completo. A primeira é razoável quando quebrar o programa revela um erro de programação que já deveria ter sido evitado por uma validação anterior.

**Fig. 4.1** | As coleções e seu acesso seguro.

```rust
// fig04_01.rs
use std::collections::HashMap;

struct Servicio {
    nombre: String,
}

#[derive(Debug)]
enum Estado {
    Ok,
    Falla,
}

fn main() {
    let s = Servicio { nombre: "catalogo".to_string() };
    let mut v: Vec<Servicio> = Vec::new();
    v.push(s);
    let primero = &v[0];                    // si no existe: panic
    println!("{}", primero.nombre);
    let primero = v.get(0);                 // devuelve Option<&Servicio>
    println!("{}", primero.is_some());

    let mut m: HashMap<String, Estado> = HashMap::new();
    m.insert("catalogo".to_string(), Estado::Ok);
    println!("{:?}", m.get("catalogo"));
    println!("{:?}", m.get("pagos"));       // Option<&Estado>: no hay valor cero silencioso
    println!("{:?}", Estado::Falla);

    let mut conteo: HashMap<String, u32> = HashMap::new();
    *conteo.entry("reportes".into()).or_insert(0) += 1;   // el patrón para contar
    *conteo.entry("reportes".into()).or_insert(0) += 1;
    println!("{:?}", conteo.get("reportes"));
}
```

```bash
$ rustc --edition 2024 fig04_01.rs && ./fig04_01
catalogo
true
Some(Ok)
None
Falla
Some(2)
```

Não use índices para percorrer um vetor por costume. Quando tudo o que você precisa é visitar cada elemento, `for servicio in &servicios` expressa melhor a intenção e evita cálculos de índice. Quando precisa do número da posição, use `enumerate()`: `for (i, servicio) in servicios.iter().enumerate()`. O valor `i` fica associado ao elemento correto e não existe o risco de escrever acidentalmente `i + 1` ao ler.

Também importa que uma referência a um elemento do vetor é um empréstimo do vetor inteiro. Acrescentar elementos pode exigir mover todo o armazenamento para outra região de memória. Por isso o Rust não permite conservar `let primero = &v[0]`, chamar depois `v.push(...)` e voltar a usar `primero`. A restrição evita referências pendentes: endereços que antes apontavam para um elemento válido e agora apontariam para memória liberada.

O `revisor` mantém a lista de serviços como vetor porque o arquivo declara uma sequência e o relatório precisa conservar essa ordem conceitual. As funções de relatório recebem slices emprestados, `&[Servicio]` e `&[Estado]`, em vez de tomar os vetores. Um slice dá acesso a uma sequência sem transferir a propriedade da coleção. Assim o `main` pode imprimir o relatório e depois inspecionar os estados para escolher o código de saída.

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

O valor de `filas` é um `Vec<Fila<'a>>`: uma lista nova de pares de referências. Não clona os serviços nem os estados para poder ordená-los; cria referências a ambos. Essa diferença importa num programa que pode lidar com listas grandes. Possuir dados novos custa memória e trabalho de cópia; emprestar dados existentes conserva uma fonte de verdade e torna visível que o relatório está apenas observando.

### `HashMap<K, V>`: buscar por chave sem inventar ausências

Um `HashMap<K, V>` guarda associações entre uma chave e um valor. Para o revisor, uma chave natural seria o nome do serviço e o valor seria seu estado: `"catalogo" -> Estado::Ok { ... }`. É uma coleção útil quando você sabe o que quer buscar, mas não sabe em que posição de uma lista está. Buscar linearmente num `Vec` implica revisar elementos até encontrar um; buscar por chave num mapa expressa diretamente a pergunta.

A operação `insert` toma posse da chave e do valor. Por isso a figura constrói `"catalogo".to_string()`: o mapa precisa possuir um `String` que sobreviva depois de terminar a chamada. `get`, por outro lado, empresta. O tipo de `m.get("catalogo")` é `Option<&Estado>`, não `Estado`, porque a chave pode não existir e porque o mapa conserva a propriedade do estado.

Essa é uma diferença importante em relação ao Go. Um acesso como `m["pagos"]` num `map[string]Estado` do Go entrega o valor zero se a chave não existe. Se `Estado` contém números, esse zero pode parecer uma resposta real. No Rust, `None` comunica uma ausência que você deve tratar. Você pode usar `match`, `if let Some(estado) = ...`, ou métodos como `unwrap_or` quando um valor padrão for realmente correto para o domínio.

O padrão `entry(...).or_insert(0)` evita fazer duas buscas quando você quer atualizar um contador. `entry` representa a posição de uma chave que pode estar ocupada ou vaga. `or_insert(0)` mantém o valor existente ou insere zero e devolve uma referência mutável ao contador. O `*` desreferencia essa referência mutável para poder aplicar `+= 1`. Não é sintaxe decorativa: o Rust separa com precisão o valor guardado do empréstimo que permite modificá-lo.

Um mapa não promete uma ordem de percurso. A ordem interna depende de como as chaves são distribuídas e pode mudar ao inserir, apagar, mudar de execução ou usar outra versão da biblioteca. Nunca construa uma saída pública percorrendo um `HashMap` e esperando que saia alfabética por coincidência. Para um relatório reproduzível, extraia as chaves, ordene-as e percorra-as nessa ordem, ou use uma estrutura ordenada quando essa for a operação central.

O projeto atual não usa um `HashMap` para o seu relatório. Usa dois vetores paralelos: serviços e estados, ambos na mesma ordem. Isso permite que um serviço conserve sua posição desde o YAML até o resultado da consulta. No momento de imprimir, a função `ordenadas` forma pares emprestados e os ordena por `nombre`. O conceito que compartilha com um `HashMap` é decisivo: o armazenamento pode ter a ordem conveniente para trabalhar, mas a saída pública deve impor sua própria ordem de maneira explícita.

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

O lifetime `'a` vai aparecer a fundo na lição 5. Por ora basta lê-lo como uma garantia: cada par de `Fila` contém referências que não podem viver mais que os slices de entrada. O vetor `filas` é dono dos pares, mas não é dono dos serviços nem dos estados. Quando `tabla` termina, as referências temporárias desaparecem; os vetores originais continuam sendo propriedade do `main`.

### `String` e `&str`: possuir texto contra ler uma visão

O Rust distingue o texto que possui dados do texto que apenas empresta uma visão. `String` é uma string UTF-8, mutável e de tamanho variável; normalmente vive no heap e é dona de seus bytes. `&str` é uma referência a uma sequência UTF-8 que já existe em outro lugar. Um literal como `"catalogo"` tem tipo `&'static str`: é uma visão de texto armazenado dentro do binário e disponível durante toda a execução.

A regra prática é simples: receba `&str`, guarde `String`. Uma função que só vai ler um nome não precisa receber a propriedade nem obrigar quem a chama a criar uma cópia. Um struct que deve conservar o nome depois que a chamada termina, sim, precisa ser dono de um `String`. Essa regra não é absoluta, mas evita dois erros comuns: aceitar `String` por reflexo e acabar movendo valores desnecessariamente, ou tentar guardar uma referência a um texto cujo dono vai desaparecer.

**Fig. 4.2** | Receba `&str`, aceite os dois.

```rust
// fig04_02.rs
fn saludar(n: &str) { println!("hola, {n}"); }

fn main() {
    let propio = String::from("catalogo");
    saludar("pagos");
    saludar(&propio);
}
```

```bash
$ rustc --edition 2024 fig04_02.rs && ./fig04_02
hola, pagos
hola, catalogo
```

A chamada `saludar(&propio)` funciona por coerção: uma referência a `String` pode ser utilizada onde se espera `&str`. Não copie a string nem escreva `propio.to_string()` para “fazer encaixar”. A função só pede leitura, então emprestar é a operação correta. Além disso, `propio` continua disponível depois da chamada.

<!-- verificar:fragmento -->
```rust
fn saludar(n: String) { }
```

Essa assinatura compila, mas comunica outra coisa: quem chama deve entregar a propriedade de um `String`. Não aceita um literal sem conversão e não permite continuar usando o `String` original depois da chamada. Uma API assim poderia ser correta se a função vai guardar, transformar ou devolver o texto como dona, mas é uma má escolha para uma função que só imprime ou compara.

`String` não admite indexação com inteiros como se cada caractere ocupasse um byte. O Rust usa UTF-8; uma letra visível pode ocupar vários bytes. Permitir `nombre[3]` seria ambíguo: o quarto byte, o quarto valor Unicode ou o quarto grupo visível? Por isso você deve decidir a unidade de que precisa. `s.as_bytes()` trabalha com bytes, `s.chars()` trabalha com valores `char`, e `s.get(intervalo)` devolve `Option<&str>` quando o intervalo pode cair no meio de uma codificação. Essa restrição evita partir uma string UTF-8 num ponto inválido.

O `revisor` guarda `nombre` como `String` porque o valor vem do YAML e deve viver dentro de cada `Servicio`. Quando calcula a largura de uma coluna, não conta bytes: usa `chars().count()`. Não resolve todos os detalhes de largura visual do Unicode, mas evita tratar um caractere multibyte como vários caracteres ao contar.

<!-- verificar:extracto:src/reporte.rs -->
```rust
        .iter()
        .map(|(s, _)| s.nombre.chars().count())
        .max()
        .unwrap_or(0)
        .max("SERVICIO".len());
```

Não converta tudo para `String` “por via das dúvidas”. Uma conversão pode alocar memória e, sobretudo, esconder quem deve possuir o texto. Comece pela assinatura: se a função só lê, `&str`; se o resultado deve sobreviver de forma independente, `String`. Se precisar aceitar vários tipos que podem ser vistos como texto, você vai conhecer `AsRef<str>` e traits genéricos na lição 5, mas não os use antes de uma API realmente precisar.

### `Result<T, E>` e `?`: tornar visível o caminho de erro

`Result<T, E>` é um enum da biblioteca padrão com duas variantes: `Ok(T)` e `Err(E)`. Uma função que devolve `Result<String, std::io::Error>` promete uma de duas coisas: devolverá texto lido corretamente, ou devolverá o erro de entrada/saída que impediu de lê-lo. Não devolve um texto vazio para sinalizar fracasso e não imprime um erro dentro de uma função que talvez seja usada a partir de outro lugar.

<!-- verificar:fragmento -->
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

O operador `?` opera sobre um `Result`. Se recebe `Ok(valor)`, extrai `valor` e a execução continua. Se recebe `Err(erro)`, termina a função atual com esse erro, convertendo-o para o tipo de erro declarado quando existe uma conversão válida. Não ignora o erro e não o transforma em pânico (panic). É uma forma compacta de escrever uma decisão que continua sendo obrigatória.

**Fig. 4.3** | O operador `?` devolve o erro a quem chamou.

```rust
// fig04_03.rs
fn leer_config(ruta: &str) -> Result<String, std::io::Error> {
    let contenido = std::fs::read_to_string(ruta)?;   // si falla, retorna el error
    Ok(contenido)
}

fn main() {
    match leer_config("servicios-que-no-existe.txt") {
        Ok(texto) => println!("{texto}"),
        Err(e) => println!("error: {e}"),
    }
}
```

```bash
$ rustc --edition 2024 fig04_03.rs && ./fig04_03
error: No such file or directory (os error 2)
```

A função `leer_config` não sabe se um arquivo ausente é fatal para toda a aplicação, recuperável por outro caminho ou esperado por um teste. Por isso devolve o erro. O `main`, sim, decide como comunicá-lo: nesta figura, imprime-o. Num binário de produção normalmente você o escreveria na saída de erro e terminaria com um código diferente de zero, para que uma pessoa e uma automação possam distinguir sucesso de fracasso.

A comparação com o Go é direta. Estas quatro linhas de Rust:

<!-- verificar:fragmento -->
```rust
let a = paso1()?;
let b = paso2(a)?;
let c = paso3(b)?;
Ok(c)
```

expressam uma cadeia de operações que no Go costuma ser escrita com uma verificação `if err != nil` depois de cada passo. O Rust reduz a repetição, mas não reduz a responsabilidade. Cada `?` marca um lugar onde a função pode sair antes. Se mais adiante o programa precisar limpar recursos, transformar um erro ou tomar uma alternativa, você deve decidir isso antes ou depois desse ponto.

O `revisor` usa `?` para ler o arquivo e para desserializar o YAML. Ambas as falhas fazem parte de iniciar o programa: não há lista válida de serviços sem arquivo legível nem sem YAML válido. A função devolve `anyhow::Result<Vec<Servicio>>`, que permite unificar erros de tipos distintos sem perder suas mensagens.

<!-- verificar:extracto:src/config.rs -->
```rust
use anyhow::{Context, Result};
pub fn cargar(ruta: &str) -> Result<Vec<Servicio>> {
    // with_context agrega a qué archivo se refería el error, como el %w de Go
    let txt = std::fs::read_to_string(ruta).with_context(|| format!("leyendo {ruta}"))?;
    Ok(yaml_serde::from_str(&txt)?)
}
```

Um esclarecimento sobre o nome: `yaml_serde::from_str` é o mesmo `from_str` que o `serde_yaml` oferecia, o crate anterior, que já não é mantido. Na lição 6 você vai ver por que o `revisor` usa o primeiro.

`with_context` acrescenta informação que o sistema operacional não conhece. O erro original pode dizer “No such file or directory”, mas o contexto esclarece qual arquivo o revisor estava tentando ler. É a diferença entre um diagnóstico tecnicamente correto e um diagnóstico acionável. O operador `?` conserva essa cadeia de causas ao devolver o erro.

Observe também que nem todos os fracassos de uma consulta são `Err`. A função `revisar` do projeto devolve `Estado`, mesmo quando um serviço não responde. Isso é correto porque “um serviço falhou” é um dado que o relatório deve mostrar, não uma impossibilidade de continuar o programa. `Result` representa que o próprio programa não conseguiu completar uma operação necessária; `Estado::Falla` representa um resultado normal do domínio do revisor. Escolher entre os dois depende de quem deve decidir o que fazer e de se o programa ainda pode produzir um resultado útil.

### `panic!`: uma parada para bugs, não um substituto de `Result`

`panic!` interrompe o fluxo normal de execução porque o programa encontrou uma condição que seus próprios pressupostos declaravam impossível. Indexar um vetor fora do intervalo provoca um pânico (panic). Chamar `unwrap()` sobre `None` ou sobre `Err` também. Esses mecanismos existem porque há invariantes que, se quebrados, revelam um erro de programação e não uma situação que uma pessoa usuária deva reparar.

Um arquivo ausente não é um invariante quebrado: pode faltar por um caminho mal escrito, permissões, uma implantação incompleta ou uma decisão de quem executa o binário. Deve ser `Result`. Uma resposta HTTP 500 também não é motivo para entrar em pânico: é um estado que o revisor foi construído para reportar. Um índice calculado a partir de um arquivo também não deve ser usado com `[]` sem validar; use `get` e devolva um erro que explique o problema.

**Fig. 4.4** | Um pânico capturado para demonstrar que não é um `Result`.

```rust
// fig04_04.rs
fn main() {
    let previo = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let resultado = std::panic::catch_unwind(|| {
        panic!("la lista validada no puede estar vacia");
    });

    std::panic::set_hook(previo);
    println!("hubo panico: {}", resultado.is_err());
}
```

```bash
$ rustc --edition 2024 fig04_04.rs && ./fig04_04
hubo panico: true
```

A figura captura o pânico apenas para isolar a demonstração. Não é o padrão normal de uma aplicação. `catch_unwind` não converte os pânicos em controle de fluxo saudável nem garante que uma estrutura fique em um estado apto para continuar sendo usada. Em código cotidiano, se você está pensando em se recuperar de um `panic!` provocado por dados externos, quase sempre deve redesenhar a função para que devolva `Result`.

`.unwrap()` e `.expect("mensagem")` são pânicos em potencial. `expect` é preferível quando existe uma razão concreta e invariante para acreditar que não vai falhar, porque sua mensagem documenta essa razão. Não escreva `expect("deve funcionar")`: não explica nada. Uma mensagem útil nomeia o pressuposto, como “o semáforo nunca se fecha”, e deixa claro o que seria preciso investigar se acontecer.

O projeto usa `expect` para adquirir uma permissão de um semáforo interno. Não é um erro provocado pelo arquivo YAML nem por uma URL; seria uma contradição na coordenação que o próprio programa construiu. Por isso é um dos poucos lugares onde o pânico faz sentido.

<!-- verificar:extracto:src/revisar.rs -->
```rust
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
```

Não copie esse padrão para operações de entrada/saída. `File::open(ruta).expect(...)` converte um caminho inexistente no encerramento do processo e elimina a oportunidade de o `main` imprimir o arquivo, usar outro valor ou selecionar um código de saída apropriado. Primeiro pergunte se o caso pode ocorrer com dados válidos do exterior. Se a resposta for sim, devolva `Result`.

### `anyhow` e `thiserror`: dois papéis distintos para erros próprios

A biblioteca padrão basta para muitos programas pequenos: você pode devolver `Result<T, std::io::Error>` quando toda falha relevante é de entrada/saída. Um programa real costuma combinar vários tipos: `std::io::Error`, um erro de YAML, uma URL inválida, um argumento de linha de comando ou uma regra de validação. Se cada camada precisa conhecer todos esses tipos concretos, as assinaturas ficam difíceis de manter.

`anyhow` resolve bem a borda de uma aplicação. Seu `Result<T>` é uma forma abreviada de devolver um erro dinâmico que pode conter diferentes causas e contexto adicional. O revisor é um binário: lê configuração, inicia consultas e apresenta mensagens a uma pessoa. Nessa fronteira, a prioridade é explicar qual operação falhou e conservar a cadeia de causas. Por isso `config::cargar` usa `anyhow::{Context, Result}`.

Isso não significa que `anyhow` seja uma licença para apagar significado. Se uma função devolve um estado que outra parte do programa precisa distinguir para tomar decisões, um enum próprio pode ser melhor. Por exemplo, uma biblioteca que precisa permitir que quem a chama diferencie `NombreRepetido`, `UrlSinEsquema` e `TimeoutCero` não deve entregar apenas uma string. Deve publicar um tipo de erro com variantes que representem essas causas.

`thiserror` ajuda a declarar esse tipo de erro próprio sem escrever manualmente implementações repetitivas de `Display`, `Error` e conversões a partir de erros internos. É usado sobretudo em bibliotecas, onde o tipo de erro faz parte da API pública. O `Cargo.lock` do projeto contém `thiserror`, mas o `Cargo.toml` do revisor não o declara como dependência direta e seu código atual não expõe um enum de erros próprio. Por isso o `revisor` não o usa: seus erros são mensagens de texto que o `anyhow` acompanha com contexto.

<!-- verificar:fragmento -->
```rust
#[derive(Debug, thiserror::Error)]
enum ErrorConfiguracion {
    #[error("el nombre «{0}» está repetido")]
    NombreRepetido(String),
    #[error("la URL «{0}» no tiene esquema HTTP")]
    UrlSinEsquema(String),
    #[error("no se pudo leer la configuración")]
    Lectura(#[from] std::io::Error),
}
```

Este fragmento ilustra uma API de biblioteca, não faz parte do revisor atual. `#[from]` permite converter automaticamente um `std::io::Error` em `ErrorConfiguracion`, de modo que `?` continue sendo útil. As outras variantes conservam dados que a pessoa que chama pode inspecionar por meio de `match`. Numa aplicação de uma só camada, converter no final esses erros para `anyhow::Error` pode ser cômodo; numa biblioteca, escondê-los cedo demais tira opções de quem a usa.

A fronteira prática é esta: `anyhow` para executar uma aplicação e explicar uma falha completa; `thiserror` para oferecer um contrato de erros que outros programas devam tratar por variante. Você pode combinar ambos, mas não os acrescente por moda. Comece com o tipo que permite à camada seguinte tomar a decisão correta.

## O erro que você vai ver

### E0277: usar `?` numa função que não pode devolver um erro

O erro mais frequente ao começar a usar `?` aparece quando a função declara um retorno simples, como `String`, mas dentro dela tenta propagar um `Result`. O Rust não pode inventar onde guardar o erro nem como comunicá-lo a quem chamou. A execução real a seguir, com `rustc 1.98.1`, lê o programa a partir da entrada padrão, por isso o compilador nomeia o arquivo como `<anon>`; com um arquivo em disco você verá o nome dele no lugar de `<anon>`.

<!-- verificar:fragmento -->
```rust
fn leer() -> String {
    let texto = std::fs::read_to_string("faltante.txt")?;
    Ok(texto)
}

fn main() {}
```

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> <anon>:2:56
  |
1 | fn leer() -> String {
  | ------------------- this function should return `Result` or `Option` to accept `?`
2 |     let texto = std::fs::read_to_string("faltante.txt")?;
  |                                                        ^ cannot use the `?` operator in a function that returns `String`

error[E0308]: mismatched types
 --> <anon>:3:5
  |
1 | fn leer() -> String {
  |              ------ expected `String` because of return type
2 |     let texto = std::fs::read_to_string("faltante.txt")?;
3 |     Ok(texto)
  |     ^^^^^^^^^ expected `String`, found `Result<String, _>`
  |
  = note: expected struct `String`
               found enum `Result<String, _>`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

`E0277` diz que `?` precisa de uma função capaz de devolver um resíduo de erro. `E0308` é a consequência: `Ok(texto)` é um `Result`, mas a assinatura prometia um `String`. A correção não é tirar o `?` e usar `unwrap()`. Você deve corrigir o contrato para que descreva a possibilidade real de falha.

<!-- verificar:fragmento -->
```rust
fn leer() -> Result<String, std::io::Error> {
    let texto = std::fs::read_to_string("faltante.txt")?;
    Ok(texto)
}
```

Se você está no `main`, também pode devolver `Result` quando o erro deve terminar o programa. No entanto, o revisor atual precisa controlar o que é impresso e que código de saída devolve, por isso o `main` transforma o resultado de `config::cargar` numa saída visível e em `ExitCode::from(2)`. A mensagem vai para o `stderr`; a tabela ou o JSON bem-sucedidos ficam disponíveis no `stdout`.

### Um pânico por índice fora do intervalo

O acesso `v[indice]` não é um erro de compilação se `indice` é uma variável. O Rust não pode saber em tempo de compilação qual número vai chegar. Se o número cair fora do intervalo, o programa entra em pânico durante a execução. O diagnóstico menciona o índice pedido e o comprimento real do vetor. Por exemplo, pedir a posição `3` de uma lista de três elementos falha porque as posições válidas são `0`, `1` e `2`.

A correção depende da origem do índice. Se é uma constante escrita junto a uma lista fixa e o índice está errado, corrija o programa. Se vem de um arquivo, de uma requisição ou de uma opção do usuário, use `get(indice)` e converta `None` num `Result` que explique qual era o intervalo válido. Não deixe que uma entrada recuperável termine o processo com um backtrace.

### Diagnosticar antes de consertar

Leia primeiro a assinatura da função e o tipo concreto da expressão que falhou. Se diz `Option`, decida o que significa a ausência. Se diz `Result`, leia a variante de erro e decida se deve ser propagada, contextualizada ou tratada ali. Se aparece `panic!`, pergunte qual pressuposto do programa foi quebrado. Copiar `.clone()`, `.unwrap()` ou `as` até compilar costuma apagar informação útil e empurra o problema para a execução.

`rustc --explain E0277` amplia o significado geral do código, mas o diagnóstico local continua sendo a fonte principal. Suas setas apontam o tipo esperado, o tipo encontrado e a linha onde o contrato deixou de coincidir. Aprender a seguir essas três pistas vale mais do que decorar uma lista de códigos.

## O que se faz errado

- **Usar `v[i]` para índices que vêm de fora.** O índice direto afirma que a posição existe. Se essa afirmação depende de um arquivo ou de uma entrada humana, use `get`; `None` é um dado que você deve transformar numa explicação útil.

- **Percorrer um `HashMap` e publicar sua ordem acidental.** Um relatório que muda de ordem é difícil de ler, testar e comparar. Extraia chaves ou linhas, ordene-as e só então imprima. A saída do revisor deve ser reproduzível mesmo que o armazenamento interno mude.

- **Usar `String` em todos os parâmetros.** Obriga a transferir propriedade ou criar alocações desnecessárias. Se uma função só lê, declare `&str`; reserve `String` para estruturas e resultados que devam possuir texto.

- **Indexar um `String` por byte ou supor que `len()` conta letras visíveis.** O Rust armazena texto UTF-8. Use `chars`, `bytes` ou intervalos com `get` conforme a unidade de que você realmente precisa. Um nome que hoje só tem ASCII pode amanhã conter caracteres válidos de mais de um byte.

- **Converter todos os erros em `unwrap()` ou `expect()`.** Faz o programa parecer curto enquanto elimina caminhos de recuperação e contexto. `unwrap` é aceitável num teste quando a falha invalida o próprio teste; não é a forma normal de ler arquivos nem de processar argumentos.

- **Usar `panic!` para dados inválidos de configuração.** Uma URL incorreta, um arquivo ausente ou um nome repetido são falhas que uma pessoa pode corrigir. Devolva `Result` com o dado que falta e uma causa compreensível.

- **Perder a causa original ao criar uma mensagem nova.** Um texto como `"não foi possível carregar"` não diz qual caminho falhou nem por quê. Use `with_context` para acrescentar operação e dados locais sem descartar a causa que o sistema ou o parser devolveu.

- **Usar `anyhow` numa biblioteca que precisa de erros distinguíveis.** Se quem chama deve reagir de forma diferente diante de uma URL inválida e de um nome repetido, publique um enum próprio, normalmente com `thiserror`. A conveniência de uma string não deve apagar decisões de domínio.

## Exercícios

### Exercício 1 — Acesso honesto a uma lista

Crie um `Vec<Servicio>` com dois serviços. Escreva uma função `nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str>` que devolva o nome do serviço quando existe e `None` quando não existe. Teste-a com os índices `0`, `1` e `2`. Não use `[]` dentro da função.

Explique por escrito por que devolver `Option<&str>` é mais honesto do que devolver uma string vazia. Pense no que aconteceria se um nome vazio fosse um dado permitido.

### Exercício 2 — Estados por nome e relatório determinístico

Crie um `HashMap<String, Estado>` com três nomes, incluindo uma falha. Escreva uma função que produza um `Vec<String>` com os nomes ordenados alfabeticamente. Depois percorra esses nomes e gere linhas no formato `nombre: estado`.

Rode o programa várias vezes. A saída deve conservar exatamente a mesma ordem. Não ordene o `HashMap`: ele não se ordena. Ordene uma coleção separada de chaves ou de linhas.

### Exercício 3 — Carregar, validar e contextualizar

Escreva uma função `cargar(ruta: &str) -> Result<Vec<Servicio>, ...>` que leia um arquivo de texto com uma linha por serviço. Cada linha deve conter nome e URL separados por vírgula. Rejeite uma linha sem dois campos, uma URL sem `http://` ou `https://`, e uma lista vazia. Propague os erros de leitura com `?`.

Depois adapte a função para usar `anyhow::Context` e acrescentar o caminho à mensagem de leitura. Faça o `main` imprimir os erros na saída de erro e terminar com código `2`; se todos os dados forem válidos, imprima a lista ordenada e termine com código `0`.

## Soluções

### Solução 1

A função deve emprestar o slice, não tomar o vetor. `get` já devolve `Option<&Servicio>`, e `map` transforma o conteúdo de `Some` sem tocar em `None`.

<!-- verificar:fragmento -->
```rust
fn nombre_en(servicios: &[Servicio], indice: usize) -> Option<&str> {
    servicios.get(indice).map(|servicio| servicio.nombre.as_str())
}
```

`as_str()` converte a referência a `String` numa referência a `str`; não aloca memória nem clona texto. A vida do `&str` fica limitada pela vida do slice emprestado, que é precisamente o contrato correto. Uma string vazia seria ambígua: poderia significar “não encontrei esse índice” ou “encontrei o serviço e seu nome é vazio”.

### Solução 2

A solução precisa separar a estrutura útil para buscar da estrutura útil para apresentar. O mapa conserva a associação; o vetor de chaves recebe a ordem que o relatório exige.

<!-- verificar:fragmento -->
```rust
let mut nombres: Vec<&str> = estados.keys().map(String::as_str).collect();
nombres.sort();

for nombre in nombres {
    let estado = &estados[nombre];
    println!("{nombre}: {estado:?}");
}
```

O acesso `estados[nombre]` é razoável aqui porque `nombre` vem diretamente de `estados.keys()`: o programa já demonstrou que a chave existe. Se `nombre` viesse de um arquivo ou de um argumento, essa forma voltaria a afirmar algo que você não validou e você deveria usar `get`.

### Solução 3

A assinatura de carga deve deixar que os problemas externos subam como `Result`. As regras de formato também devem se converter em erros, não em pânicos. Se você usar `anyhow`, uma implementação pode seguir esta forma:

<!-- verificar:fragmento -->
```rust
fn cargar(ruta: &str) -> anyhow::Result<Vec<Servicio>> {
    let texto = std::fs::read_to_string(ruta)
        .with_context(|| format!("leyendo {ruta}"))?;

    let mut servicios = Vec::new();
    for (numero, linea) in texto.lines().enumerate() {
        let (nombre, url) = linea
            .split_once(',')
            .with_context(|| format!("línea {} sin coma", numero + 1))?;

        anyhow::ensure!(
            url.starts_with("http://") || url.starts_with("https://"),
            "línea {}: URL sin esquema: {url}",
            numero + 1
        );

        servicios.push(Servicio::new(nombre, url));
    }

    anyhow::ensure!(!servicios.is_empty(), "el archivo no declara servicios");
    Ok(servicios)
}
```

A solução não usa `unwrap` porque cada falha pode se originar em conteúdo externo. `split_once` devolve `Option`; `with_context` o converte num erro explicativo. `ensure!` termina a função com `Err` se a condição não é cumprida. O contexto inclui número de linha ou caminho para que quem corrige o arquivo não tenha de adivinhar por onde começar.

## Como sei que consegui

- Você executa `rustc --edition 2024 fig04_01.rs && ./fig04_01` e obtém exatamente seis linhas, incluindo `None` para `pagos` e `Some(2)` para o contador.
- Você executa `rustc --edition 2024 fig04_02.rs && ./fig04_02` e consegue explicar por que o mesmo parâmetro `&str` aceita um literal e uma referência a `String`.
- Você executa `rustc --edition 2024 fig04_03.rs && ./fig04_03` e obtém um erro de arquivo ausente sem um pânico.
- Sua solução do exercício 1 devolve `None` para o índice fora do intervalo e não contém acesso com `servicios[indice]`.
- Seu relatório do exercício 2 produz as mesmas linhas, na mesma ordem, depois de pelo menos dez execuções.
- Sua solução do exercício 3 nomeia o caminho quando o arquivo não existe, nomeia a linha quando o formato está errado e não usa `unwrap` no caminho normal de execução.
- A partir de `programas/revisor`, `cargo test`, `cargo clippy --all-targets -- -D warnings` e `cargo fmt --check` terminam corretamente.

## Para ler mais

- [The Rust Programming Language, capítulo 8: coleções](https://doc.rust-lang.org/book/ch08-00-common-collections.html), em particular vetores, strings e mapas hash. Consultado em 2 de outubro de 2026.

- [The Rust Programming Language, capítulo 9: tratamento de erros](https://doc.rust-lang.org/book/ch09-00-error-handling.html). Consultado em 2 de outubro de 2026.

- [Documentação oficial de `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) e [de `HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html). Consultado em 2 de outubro de 2026.

- [Documentação do `anyhow`](https://docs.rs/anyhow/) e [documentação do `thiserror`](https://docs.rs/thiserror/). Consultado em 2 de outubro de 2026.
