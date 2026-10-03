# Lição 3 — Structs, enums e match

**Tempo:** 2 × 45 min.

**O que você constrói:** o modelo do `revisor`: `Servicio` e `Estado`

**O que você aprende:** structs e `impl`, enums que carregam dados, `match` exaustivo, `Option` em vez de `nil`

**The Rust Book, capítulos 5 e 6.** Rustlings: `structs`, `enums`, `options`.

## Ao terminar, você vai poder

- Modelar um serviço com um `struct` cujos campos tenham nome e tipo.
- Escrever métodos dentro de um bloco `impl` e decidir se recebem `&self`, `&mut self` ou `self`.
- Representar resultados incompatíveis entre si com um `enum` que carregue dados.
- Escrever um `match` exaustivo que extraia dados de cada variante.
- Explicar por que acrescentar uma variante nova obriga a revisar decisões existentes.
- Usar `Option<T>` quando um valor pode faltar, sem recorrer a `nil`.
- Escolher entre `match`, `if let` e métodos como `unwrap_or` conforme a intenção do código.

## O porquê antes do como

Até aqui o curso usou valores simples: números para tempos, strings para nomes e condições para classificar uma resposta. Isso basta para praticar variáveis, funções, tipos e ownership, mas não basta para descrever o domínio real do `revisor`. Um serviço não é apenas um nome, uma URL e um tempo limite que por acaso aparecem juntos em três variáveis. São três dados que descrevem uma única coisa e que devem viajar, ser validados e ser consultados como uma unidade.

Guardar esses dados separadamente produz erros silenciosos. Imagine que você tem `nombre_catalogo`, `url_catalogo`, `timeout_catalogo`, depois acrescenta os mesmos três valores para pagamentos e relatórios, e ao construir o relatório junta o nome de pagamentos com a URL de catálogo. O compilador não consegue detectar o problema: as três peças têm tipos válidos, mas a relação entre elas se perdeu. Um `struct` permite declarar essa relação uma vez e convertê-la em parte do tipo.

O segundo problema aparece depois de consultar um serviço. Uma resposta saudável traz um código HTTP e uma duração. Uma resposta lenta também traz os dois dados, mas exige uma etiqueta diferente. Uma falha pode trazer uma mensagem e uma duração, mas não necessariamente um código HTTP. E um serviço que ainda não foi consultado não tem código, nem duração, nem mensagem de falha. Se você tentasse guardar tudo isso em um único `struct` com campos “às vezes válidos”, teria combinações absurdas: um estado de falha com código `200`, um serviço não tentado com duração de `0 ms` que ninguém sabe interpretar, ou uma mensagem de erro vazia que significa coisas diferentes conforme outro campo booleano.

O Rust resolve essa modelagem com `enum`. Diferentemente de um enum tradicional de outras linguagens, que costuma ser uma lista de números ou constantes com nome, uma variante do Rust pode carregar dados. `Estado::Ok` leva código e milissegundos; `Estado::Falla` leva um motivo; `Estado::NoIntentado` não leva nada porque não há dados honestos para guardar. O tipo expressa que um valor está em exatamente um desses estados, nunca em vários ao mesmo tempo.

A terceira peça é o `match`. Quando você recebe um `Estado`, não basta saber que ele pertence ao enum: você precisa decidir o que fazer com cada possibilidade. O Rust exige que essa decisão cubra todas as variantes. Não é uma recomendação de estilo nem uma regra de um linter; faz parte da compilação. Se amanhã você acrescentar `Estado::Rechazado`, cada `match` que antes parecia terminado se converte num lugar que o compilador aponta para revisão. Essa obrigação é uma rede de segurança para refatorações.

O curso de Go constrói o mesmo `revisor`, mas aqui aparece uma diferença importante entre as duas linguagens. No Go, um resultado costuma ser modelado com um `struct`, campos de valor zero, ponteiros e convenções sobre quais campos estão presentes. No Rust, o tipo pode representar diretamente alternativas incompatíveis. Isso não elimina a necessidade de pensar no domínio, mas torna as decisões corretas mais fáceis de expressar e as inconsistências mais difíceis de compilar.

A última parte do modelo é a ausência. Em muitas linguagens uma referência pode ser `null` ou `nil` ainda que o seu tipo não diga isso de forma visível. O programa chega a uma linha que esperava um objeto, recebe ausência e falha durante a execução. O Rust não tem `nil`. Quando algo pode faltar, seu tipo declara isso por meio de `Option<T>`. Isso obriga a tomar uma decisão antes de usar o conteúdo: tratar `Some(valor)`, tratar `None` ou fornecer uma alternativa explícita. A ausência deixa de ser um acidente escondido e se torna parte do contrato da função.

Esta lição não trata de decorar toda a sintaxe de padrões. Trata de aprender a se perguntar quais estados reais existem, quais dados pertencem a cada estado e quais decisões devem mudar quando o modelo muda. Essas perguntas reaparecem na lição 4 com `Result`, na 5 com traits, na 6 com testes e na 8 quando o `revisor` gera JSON.

## Os conceitos

### Structs: um nome para dados que pertencem juntos

Um `struct` define um tipo composto com campos nomeados. A palavra importante é “tipo”: depois de definir `Servicio`, o Rust deixa de ver uma coleção informal de três dados e passa a ver um valor que representa um serviço. Cada campo conserva seu próprio tipo, de modo que o compilador continua distinguindo texto de números, mas agora também sabe que esses valores formam uma única entidade.

Um struct com campos nomeados é uma boa escolha quando cada posição tem significado próprio. Uma tupla como `(String, String, u64)` pode armazenar nome, URL e tempo limite, mas obriga a lembrar o que significam `.0`, `.1` e `.2`. Com `Servicio`, o código diz `servicio.timeout_ms`, que comunica tanto o dado quanto sua unidade. A clareza não é um enfeite: reduz a possibilidade de trocar valores parecidos e facilita ler código que você escreveu há semanas.

A criação de um struct usa chaves e pares `campo: valor`. O acesso também é direto, por meio de ponto. Como os campos da figura pertencem a `Servicio`, não existe um estado temporário em que haja uma URL sem nome ou um limite de tempo associado acidentalmente a outro serviço. Continua sendo possível criar um valor incorreto, por exemplo uma URL sem esquema; a lição 4 vai ensinar como validá-lo. O que desaparece é a bagunça de variáveis soltas.

**Fig. 3.1** | Um struct com seus métodos.

```rust
// fig03_01.rs
#[derive(Debug, Clone)]          // el compilador te escribe esos comportamientos
struct Servicio {
    nombre: String,
    url: String,
    timeout_ms: u64,
}

impl Servicio {
    fn new(nombre: &str, url: &str) -> Self {     // no hay constructores: es convención
        Self { nombre: nombre.to_string(), url: url.to_string(), timeout_ms: 5000 }
    }
    fn etiqueta(&self) -> String {                // &self = presta, no consume
        format!("{} ({})", self.nombre, self.url)
    }
}

fn main() {
    let s = Servicio::new("catalogo", "http://localhost:8090/ok");
    println!("{}", s.etiqueta());
    println!("{:?}", s.clone());
    println!("timeout: {} ms", s.timeout_ms);
}
```

```bash
$ rustc --edition 2024 fig03_01.rs && ./fig03_01
catalogo (http://localhost:8090/ok)
Servicio { nombre: "catalogo", url: "http://localhost:8090/ok", timeout_ms: 5000 }
timeout: 5000 ms
```

`#[derive(Debug, Clone)]` pede ao compilador que implemente comportamentos conhecidos para o tipo. `Debug` permite imprimir uma representação útil com `{:?}`. Não é um formato estável para usuários finais: é uma visão para desenvolvimento e diagnóstico. `Clone` permite solicitar uma cópia explícita com `.clone()`. Neste exemplo ele é usado apenas para demonstrar que a estrutura pode ser impressa e depois continuar disponível; você não deve copiar valores por costume para apagar erros de ownership.

O `revisor` usa o mesmo modelo, mas acrescenta os atributos necessários para ler serviços a partir de YAML. `pub` indica que outros módulos do crate podem acessar esses campos. `Deserialize` e `serde` vão aparecer a fundo nas lições posteriores; por ora observe que o núcleo continua sendo o mesmo: nome, URL e limite de tempo.

<!-- verificar:extracto:src/modelo.rs -->
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}
```

O atributo `#[serde(default = "timeout_por_omision")]` não muda o que é um serviço. Descreve uma regra de entrada: se o YAML não declara `timeout_ms`, o programa usa cinco segundos. É importante distinguir modelagem de validação. O struct declara quais dados formam um serviço; as regras sobre se uma URL tem esquema, se o tempo é maior que zero ou se o nome se repete são verificadas depois, quando o programa recebe uma lista.

### `impl` e métodos: comportamento que pertence ao tipo

Um `struct` guarda dados, mas o tipo também pode ter operações que fazem sentido para esses dados. O Rust agrupa essas operações em um bloco `impl`. O nome significa implementation: uma implementação de comportamento para um tipo. Não há uma palavra reservada especial para construtores. Por convenção, uma função associada chamada `new` cria um valor novo, mas continua sendo uma função normal dentro de `impl`.

A figura usa duas formas de chamar funções dentro de um `impl`. `Servicio::new(...)` usa `::` porque `new` ainda não tem uma instância sobre a qual trabalhar. `s.etiqueta()` usa `.` porque `etiqueta` recebe um serviço concreto. O Rust permite essa sintaxe de método quando o primeiro parâmetro se chama `self`, `&self` ou `&mut self`.

`&self` significa “empreste este valor para leitura”. O método pode olhar `nombre` e `url`, construir uma string nova e devolvê-la, mas não toma a propriedade de `s` nem modifica seus campos. Por isso, depois de `s.etiqueta()`, você ainda pode imprimir `s`, ler `s.timeout_ms` ou emprestar o serviço a outra função. É a aplicação direta dos empréstimos da lição 2 a uma função que vive junto do seu tipo.

`&mut self` significa “empreste este valor para modificá-lo”. Um método como `fn cambiar_timeout(&mut self, ms: u64)` exigiria que quem chama declare uma variável mutável e não permitiria outras referências ativas ao mesmo valor. O compilador aplica as mesmas regras que você já viu com `&mut String`: uma única referência mutável por vez, ou várias referências imutáveis, mas não as duas classes simultaneamente.

`self` sem `&` consome o valor. É uma decisão deliberada e menos comum. Resulta útil quando o método transforma um valor em outro e o original já não deve existir, por exemplo uma operação que converta uma configuração temporária numa estrutura validada. Não é uma forma mais rápida de escrever `&self`: muda quem possui o valor. Se você receber um erro de “valor movido” depois de chamar um método, revise primeiro o seu receptor.

No projeto real, `Servicio::new` concentra o valor padrão. Isso evita que cada chamada tenha de repetir `5000` e reduz o risco de que alguns serviços sejam criados com uma regra diferente sem querer.

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Servicio {
    /// Un servicio con el tiempo límite por omisión (5 segundos).
    pub fn new(nombre: &str, url: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            url: url.to_string(),
            timeout_ms: TIMEOUT_POR_OMISION_MS,
        }
    }
}
```

`Self` dentro de `impl Servicio` significa `Servicio`. Usá-lo evita repetir o nome do tipo e conserva a intenção se o tipo mudar de nome durante uma refatoração. A expressão `Self { ... }` constrói o valor; `-> Self` declara o tipo que a função devolve. A constante `TIMEOUT_POR_OMISION_MS` está fora do trecho e evita que o número cinco mil fique espalhado pelo programa como um valor mágico.

### Enums com dados: alternativas válidas, não campos ambíguos

Um `enum` descreve um valor que pode assumir uma de várias variantes. A diferença essencial em relação a uma coleção de constantes é que cada variante pode ter sua própria forma. `Estado::Ok` e `Estado::Lento` têm os campos nomeados `codigo` e `ms`. `Estado::Falla` guarda uma string. `Estado::NoIntentado` não carrega dados porque não ocorreu uma consulta que produza resultados honestos.

Esse design evita representar uma falha como um código especial, como `0`, `-1` ou uma string vazia. Esses marcadores obrigam a lembrar regras fora do tipo: “se o código é zero, leia o erro; se o erro está vazio, talvez tenha sido bem-sucedido; se o tempo é zero, talvez não tenha sido tentado”. Um enum leva essas regras para o compilador. Se você tem `Estado::Falla`, o Rust sabe que há uma mensagem; se você tem `Estado::Ok`, o Rust sabe que há código e duração.

Também evita a combinação impossível de campos opcionais. Um struct como `Resultado { codigo: Option<u16>, error: Option<String>, ms: u64 }` admite por construção tanto `codigo: Some(200), error: Some("no responde")` quanto `codigo: None, error: None`. Pode haver casos legítimos para uma forma assim, especialmente ao serializar dados externos, mas não é uma boa representação interna de alternativas mutuamente excludentes. Para o estado de uma consulta, o enum expressa melhor a realidade.

<!-- verificar:fragmento -->
```rust
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}
```

A sintaxe tem três formas que convém reconhecer. As variantes com chaves são parecidas com structs pequenos e permitem nomear campos ao criar e ao desestruturar. As variantes com parênteses são parecidas com tuplas e servem quando o dado tem um significado principal claro, como a mensagem de uma falha neste fragmento. As variantes sem dados representam uma possibilidade que não precisa de informação adicional.

**Fig. 3.2** | Um enum com dados e seu `match`.

```rust
// fig03_02.rs
#[allow(dead_code)]              // este ejemplo no lee el código de los lentos
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),                      // lleva el mensaje dentro
    NoIntentado,
}

fn main() {
    let estados = [
        Estado::Ok { codigo: 200, ms: 120 },
        Estado::Lento { codigo: 200, ms: 1800 },
        Estado::Falla("no responde".to_string()),
        Estado::NoIntentado,
    ];

    for estado in estados {
        let texto = match estado {
            Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
            Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
            Estado::Falla(msg)           => format!("FALLA: {msg}"),
            Estado::NoIntentado          => "sin revisar".to_string(),
        };
        println!("{texto}");
    }
}
```

```bash
$ rustc --edition 2024 fig03_02.rs && ./fig03_02
OK 200 en 120ms
LENTO 1800ms
FALLA: no responde
sin revisar
```

A anotação `#[allow(dead_code)]` pertence ao exemplo, não é uma receita para ocultar avisos em projetos reais. A variante `Lento` conserva `codigo` porque uma resposta lenta pode ter sido HTTP 200, embora este programa só use `ms`. Sem a anotação, o Rust avisaria que o campo `codigo` dessa variante não é lido neste arquivo. O projeto real, sim, utiliza os dados onde correspondem e é compilado com avisos tratados como erros.

O `revisor` melhora o fragmento inicial com duas decisões de domínio. Primeiro, uma falha leva tanto `motivo` quanto `ms`, porque saber que uma conexão esgotou o tempo depois de certa duração é informação útil para o relatório. Segundo, o enum recebe `derive(Debug, Clone, PartialEq)`. `PartialEq` permite comparar estados em testes com `assert_eq!`, algo que você vai usar na lição 6.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Lo que se supo de un servicio después de consultarlo.
///
/// Cada variante lleva sus propios datos: así `Falla` no tiene código HTTP que
/// alguien pueda leer por error, y `Ok` no tiene mensaje de error.
#[derive(Debug, Clone, PartialEq)]
pub enum Estado {
    /// Contestó con 2xx a tiempo.
    Ok { codigo: u16, ms: u64 },
    /// Contestó con 2xx, pero tardó más de [`UMBRAL_LENTO_MS`].
    Lento { codigo: u16, ms: u64 },
    /// No contestó, contestó con error, o se acabó el tiempo.
    Falla { motivo: String, ms: u64 },
    /// Nunca se llegó a consultar.
    NoIntentado,
}
```

O enum não substitui todo uso de booleanos nem todo uso de structs. Um booleano continua sendo correto para uma pergunta com duas respostas simples, como “o serviço está saudável?”. Um struct continua sendo correto para dados que existem juntos ao mesmo tempo, como nome, URL e limite. Um enum é adequado quando as possibilidades têm formas distintas e o programa deve tratá-las de maneira distinta.

### `match`: decidir sobre cada estado sem deixar lacunas

`match` compara um valor com padrões e produz um resultado. Na figura, cada braço tem a forma `padrão => expressão`. O padrão identifica uma variante e pode extrair seus dados. Em `Estado::Ok { codigo, ms }`, os nomes dentro das chaves criam variáveis locais chamadas `codigo` e `ms`. Em `Estado::Falla(msg)`, `msg` recebe a string que a variante carrega.

O padrão `..` significa “ignore os demais campos”. No braço de `Lento`, o programa precisa da duração para imprimi-la, mas não precisa do código. É preferível a inventar um nome como `_codigo` quando você não vai usá-lo: comunica que o dado existe e que esta decisão não depende dele. Se você não precisa de nenhum campo de uma variante com dados, pode escrever `Estado::Ok { .. }`.

Um `match` é uma expressão. Por isso a figura pode fazer `let texto = match estado { ... };`. Cada braço devolve um `String`: três usam `format!` e o último constrói um com `"sin revisar".to_string()`. O Rust exige que todas as ramificações produzam tipos compatíveis. Essa regra evita que um caminho devolva texto e outro, acidentalmente, não devolva nada.

A propriedade mais valiosa é a exaustividade. O compilador conhece todas as variantes de `Estado` porque estão declaradas no mesmo tipo. Se um `match` não cobre uma delas, não compila. Isso é mais seguro do que um `switch` que permite passar sem ação, e também é mais explícito do que uma cadeia de `if` que deixa um caso como possibilidade implícita.

Em alguns casos você vai usar um padrão curinga, `_ => ...`, para agrupar possibilidades que realmente devem receber o mesmo tratamento. É válido, mas tem um custo: se você acrescentar uma variante nova, esse braço já a aceitará sem obrigar você a pensar se o comportamento correto é o mesmo. Para um enum central como `Estado`, convém preferir braços explícitos no relatório. Assim uma nova variante se torna uma decisão visível, não um comportamento acidental.

**Fig. 3.3** | Se você esquecer uma variante, não compila.

```rust
// fig03_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Lento { codigo: u16, ms: u64 },
    Falla(String),
    NoIntentado,
}

fn main() {
    let estado = Estado::NoIntentado;
    let texto = match estado {
        Estado::Ok { codigo, ms }    => format!("OK {codigo} en {ms}ms"),
        Estado::Lento { ms, .. }     => format!("LENTO {ms}ms"),
        Estado::Falla(msg)           => format!("FALLA: {msg}"),
    };
    println!("{texto}");
}
```

```bash
$ rustc --edition 2024 fig03_03.rs
error[E0004]: non-exhaustive patterns: `Estado::NoIntentado` not covered
  --> fig03_03.rs:11:23
   |
11 |     let texto = match estado {
   |                       ^^^^^^ pattern `Estado::NoIntentado` not covered
   |
note: `Estado` defined here
  --> fig03_03.rs:2:6
   |
 2 | enum Estado {
   |      ^^^^^^
...
 6 |     NoIntentado,
   |     ----------- not covered
   = note: the matched value is of type `Estado`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
14 ~         Estado::Falla(msg)           => format!("FALLA: {msg}"),
15 ~         Estado::NoIntentado => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

No `revisor`, `match` aparece no módulo de relatórios para converter um estado técnico em uma etiqueta que uma pessoa possa ler. Observe que cada variante é nomeada de forma explícita. O código não pressupõe que “tudo o que não é OK” é uma falha: `Lento` e `NoIntentado` têm significado próprio.

<!-- verificar:extracto:src/reporte.rs -->
```rust
fn etiqueta(e: &Estado) -> &'static str {
    match e {
        Estado::Ok { .. } => "OK",
        Estado::Lento { .. } => "LENTO",
        Estado::Falla { .. } => "FALLA",
        Estado::NoIntentado => "NO",
    }
}
```

O parâmetro é `&Estado`, uma referência imutável. O relatório precisa ler o estado várias vezes para obter etiqueta, duração e detalhe, então não convém consumi-lo. O Rust permite fazer `match` sobre uma referência: os padrões leem os campos de que precisam sem mover o `Estado` original. Essa combinação de empréstimos e padrões será muito frequente em código Rust.

Existe também `matches!`, uma macro útil quando você só quer uma resposta booleana. O método `esta_bien` do projeto não precisa produzir um texto nem extrair códigos; pergunta se o valor é uma de duas variantes saudáveis. Agrupar padrões com `|` expressa essa regra sem repetir lógica.

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

Não use `matches!` para substituir um `match` que deve transformar dados. Seu resultado é sempre `bool`; é uma pergunta, não uma decisão completa. Quando o programa precisa construir um relatório, obter a duração ou escolher um detalhe específico, `match` continua sendo a ferramenta adequada.

### `Option<T>`: a ausência declarada no tipo

O Rust não tem `null` nem `nil`. Um valor do tipo `String` sempre é uma string válida; um valor do tipo `&Servicio` sempre é uma referência válida enquanto o empréstimo for válido. Quando um dado pode faltar, seu tipo deve declarar isso. A forma padrão é `Option<T>`.

<!-- verificar:fragmento -->
```rust
enum Option<T> {
    Some(T),
    None,
}
```

A definição real pertence à biblioteca padrão e tem mais atributos internos, mas este fragmento mostra sua ideia central. `Option<u16>` significa “pode haver um código `u16`, ou pode não haver”. `Option<Servicio>` significa “uma busca pode devolver um serviço ou não encontrar nenhum”. O tipo não decide o que fazer diante da ausência; obriga quem consome o valor a fazê-lo de forma explícita.

A ausência nem sempre é um erro. Buscar um serviço pelo nome pode não encontrá-lo porque o nome não está configurado. Um cabeçalho HTTP pode ser opcional. Uma falha de rede pode não produzir código HTTP. Nesses casos, `Option` comunica que a falta de valor é uma possibilidade prevista pelo contrato, não um valor secreto como `0`, `""` ou um ponteiro nulo que poderia explodir mais adiante.

**Fig. 3.4** | Consumir um `Option`.

```rust
// fig03_04.rs
fn main() {
    let quizas: Option<u16> = Some(200);

    match quizas {
        Some(c) => println!("código {c}"),
        None    => println!("sin respuesta"),
    }

    if let Some(c) = quizas { println!("código {c}"); }     // cuando solo importa un caso
    let c = quizas.unwrap_or(0);                            // valor por omisión
    println!("{c}");
}
```

```bash
$ rustc --edition 2024 fig03_04.rs && ./fig03_04
código 200
código 200
200
```

O primeiro consumo usa `match` porque os dois casos importam: há uma saída para `Some` e outra para `None`. O segundo usa `if let` porque só há trabalho a fazer quando existe um código; se não existe, o programa não precisa fazer nada. `if let Some(c) = quizas` é uma forma breve de escrever um `match` cujo outro braço seria `_ => {}`.

`unwrap_or(0)` devolve o conteúdo quando existe e o valor padrão quando não existe. A decisão de usar `0` só é correta se quem chama entende que zero representa “sem resposta” nesse contexto. Num relatório HTTP público, pode ser mais claro conservar `Option<u16>` até o ponto em que o dado é apresentado, para não confundir uma ausência com um código HTTP real.

Não confunda `unwrap_or` com `unwrap`. `unwrap()` diz: “sei que aqui há um valor; se não houver, termine o programa com um `panic!`”. Pode ser razoável num teste em que a ausência demonstra que a preparação do caso falhou, mas em código de aplicação costuma ocultar uma decisão pendente. O `clippy`, com sua configuração padrão, não avisa sobre um `unwrap` comum; existe um aviso opcional (`clippy::unwrap_used`) que quem mantém um projeto pode ativar para proibi-lo. A lição 6 vai mostrar como executar o `clippy`. Antes de escrevê-lo, pergunte-se se `None` pode ocorrer em produção. Se pode, você precisa tratá-lo.

O projeto usa `Option` para a forma JSON do relatório. Uma falha não tem um código HTTP inventado, por isso `codigo` é `Option<u16>`. O campo `error` também é opcional: aparece numa falha ou num serviço não tentado, mas é omitido para uma resposta saudável. Esse struct representa uma saída serializável, não substitui o enum interno `Estado`; os dois tipos têm responsabilidades distintas.

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

Essa separação é útil. `Estado` modela alternativas exclusivas para que a lógica interna seja segura. `EstadoJson` modela a forma que uma ferramenta externa espera ler, onde alguns campos podem ser `null` ou omitidos. O Rust não impede que uma API externa tenha valores opcionais; impede que a sua lógica interna os trate como se sempre existissem.

## O erro que você vai ver

### `E0004`: um `match` não cobre todos os padrões

A figura 3.3 produz `E0004`, “non-exhaustive patterns”. O compilador viu que `estado` é do tipo `Estado`, leu a definição do enum e verificou que existe a variante `NoIntentado`. Depois percorreu os braços do `match` e não encontrou nenhum padrão que a cobrisse.

A seta sob `estado` indica o valor sobre o qual a decisão está sendo tomada. A nota posterior mostra onde `Estado` foi definido e sublinha a variante que falta. A ajuda propõe duas correções: acrescentar um braço explícito para `Estado::NoIntentado` ou acrescentar um padrão curinga. Para este caso, a correção certa é a explícita, porque “sin revisar” merece uma saída visível:

<!-- verificar:fragmento -->
```rust
Estado::NoIntentado => "sin revisar".to_string(),
```

Não copie `todo!()` da sugestão como solução final. O Rust o propõe porque completa o padrão e deixa um marcador visível para que você decida o que fazer; se esse ramo for executado, `todo!()` termina o programa com um `panic!`. É útil durante uma refatoração breve, não como comportamento do `revisor`.

Esse erro aparece também quando você acrescenta uma variante nova. Essa é justamente uma de suas vantagens. Em vez de depender de uma busca manual pelo repositório, deixe que o tipo e o compilador enumerem os lugares que devem decidir como responder ao novo estado. Corrija cada um com uma regra de negócio, não com `_ =>` por reflexo.

### Ler o erro como um guia de mudança

Os erros do Rust costumam trazer quatro partes: o código estável como `E0004`, a localização, notas com contexto e uma ajuda. Comece pelo código e pela frase principal; neste caso bastam para saber que falta uma variante. Depois leia a nota para confirmar o tipo e a ajuda para conhecer uma forma sintática válida de corrigi-lo.

A ajuda do compilador não conhece o seu domínio. Pode dizer como completar um `match`, mas não pode decidir se um estado novo deve contar como saudável, falho, lento ou não revisado. Essa decisão continua sendo sua. A vantagem é que o Rust separa os dois problemas: garante que você não esqueceu de tratar o caso e deixa você definir o tratamento correto.

Você pode pedir uma explicação ampliada com `rustc --explain E0004`. Faça isso ao encontrar um código de erro que você não entende. Não é preciso decorar códigos; importa aprender a reconhecer que eles são identificadores consultáveis e que a mensagem contém evidência concreta sobre o tipo e a linha envolvida.

## O que se faz errado

### Modelar alternativas com um struct cheio de campos opcionais

Um struct como `Resultado { codigo: Option<u16>, error: Option<String>, ms: Option<u64> }` parece flexível, mas aceita estados incoerentes demais. Pode conter ao mesmo tempo código de sucesso e mensagem de falha, ou não conter nenhum dos dois. Se esses casos são inválidos, o tipo deveria torná-los difíceis ou impossíveis de construir.

Use um enum quando as possibilidades se excluem entre si e carregam dados diferentes. Conserve structs com `Option` para limites externos, como JSON, formulários ou configurações parciais, onde você realmente precisa representar campos que podem faltar de maneira independente.

### Usar `_ =>` para silenciar a exaustividade

O padrão curinga é correto quando todas as variantes restantes recebem exatamente o mesmo tratamento. O problema aparece quando é usado apenas para fazer o compilador parar de reclamar. Num enum de negócio, `_` pode converter uma variante nova numa falha genérica ou, pior, numa resposta saudável por acidente.

Em `Estado`, escreva os quatro braços de forma explícita. Se você acrescentar uma variante, aceite que o compilador obrigue você a revisar o relatório e os testes. O pequeno trabalho imediato evita um comportamento não revisado mais adiante.

### Escrever `unwrap()` num caminho normal do programa

`unwrap()` não resolve a ausência: converte-a num `panic!`. Se um serviço pode não ser encontrado, uma resposta pode não trazer código ou um arquivo pode não existir, a ausência faz parte da realidade do programa. Ela deve virar um `match`, um `if let`, um valor padrão justificado ou, na lição 4, um `Result`.

Em testes, `unwrap()` pode ser útil para declarar que um caso deve ser preparado corretamente. Na lógica de produção, use-o apenas quando você tiver demonstrado que `None` é impossível e a falha representa um erro de programação, não uma condição esperável.

### Copiar com `clone()` para evitar pensar em ownership

`Clone` não é uma saída automática diante de um erro de movimento. Copiar um `Servicio` só para emprestá-lo a uma função duplica seus `String` e pode ocultar que a função deveria receber `&Servicio`. Na figura 3.1, `clone()` existe para demonstrar o trait e tornar visível a estrutura; não é a forma recomendada de passar serviços pelo programa.

Prefira empréstimos para ler (`&Servicio`), empréstimos mutáveis quando houver uma modificação real (`&mut Servicio`) e movimento quando a função deve tomar a propriedade do valor. Copie unicamente quando o programa precisa de fato de dois valores independentes.

### Usar números ou strings mágicas para representar estados

Representar uma falha com `codigo == 0`, uma consulta pendente com `ms == 0` ou um erro com `mensaje == ""` obriga a lembrar convenções que o tipo não expressa. Também dificulta responder a perguntas simples: uma resposta real pode demorar zero milissegundos? uma mensagem vazia é uma falha ou a ausência de falha?

Nomeie o estado com uma variante. `Estado::NoIntentado` comunica mais do que um número especial e permite que o `match` obrigue a tratá-lo. Quando o estado tem dados, ponha-os dentro da variante que os torna válidos.

### Confundir `Option` com `Result`

`Option<T>` responde “há valor ou não?”. `Result<T, E>` responde “houve valor ou houve um erro que preciso conhecer?”. Uma busca que não encontra um nome pode devolver `Option<Servicio>`; ler um arquivo que não existe normalmente deve devolver `Result<String, Error>`, porque quem chama precisa saber o que deu errado. A lição 4 aprofunda `Result` e `?`.

Não invente mensagens de erro dentro de `Option` nem use `None` para esconder uma falha que o usuário precisa diagnosticar. Escolha o tipo conforme o contrato da operação.

## Exercícios

### Exercício 1 — Descreva um serviço

Crie um `struct ServicioLocal` com `nombre: String`, `url: String` e `timeout_ms: u64`. Escreva uma função associada `new(nombre: &str, url: &str) -> Self` que atribua `3000` como tempo padrão. Acrescente um método `etiqueta(&self) -> String` que devolva `nombre (url)`.

No `main`, crie um serviço chamado `pagos`, imprima a etiqueta e depois imprima o tempo limite. Confirme que você não precisa de `mut` nem de `clone()` para essas operações.

### Exercício 2 — Resuma todos os estados

Declare um enum `EstadoLocal` com as variantes `Ok { codigo: u16, ms: u64 }`, `Lento { codigo: u16, ms: u64 }`, `Falla(String)` e `NoIntentado`. Escreva `fn resumen(estado: &EstadoLocal) -> String` usando um `match` exaustivo.

O resumo deve usar exatamente estas formas: `OK 200 en 80ms`, `LENTO 1200ms`, `FALLA: sin conexión` e `sin revisar`. Teste-o com uma instância de cada variante.

### Exercício 3 — Acrescente uma variante e deixe o Rust encontrar o trabalho

Acrescente `Rechazado { codigo: u16, ms: u64 }` a `EstadoLocal`. Compile sem modificar `resumen` e observe o `E0004`. Depois acrescente o braço que produza `RECHAZADO 403 en 15ms`.

Não use `_ =>`. O objetivo é verificar que o compilador aponta uma decisão de negócio pendente. Explique em uma frase por que `Rechazado` não deve ser classificado automaticamente como `Falla`: o servidor respondeu, sim, mas a resposta não foi aceita.

### Exercício 4 — Busque sem usar `nil`

Crie um array ou vetor de dois `ServicioLocal`: `catalogo` e `pagos`. Escreva uma função que receba um slice de serviços e um nome, e devolva `Option<&ServicioLocal>`. Busque primeiro `pagos` e depois `reportes`.

Consuma o primeiro resultado com `if let` para imprimir sua URL. Consuma o segundo com `match` para imprimir `no existe reportes`. Não use índices com um valor sentinela, referências nulas nem `unwrap()`.

## Soluções

### Solução 1

`ServicioLocal` deve ser um struct com três campos nomeados. A função `new` deve usar `Self` e converter `nombre` e `url` de `&str` para `String`; o método `etiqueta` deve receber `&self`, pois só lê os campos. Uma saída correta contém:

```text
pagos (http://localhost:8091/ok)
timeout: 3000 ms
```

Se você precisa declarar `let mut servicio`, revise o exercício: nenhuma operação solicitada modifica o valor. Se precisa de `clone()`, provavelmente mudou uma assinatura para receber `self` quando devia receber `&self`.

### Solução 2

`resumen` deve receber `&EstadoLocal` para ler o estado sem consumi-lo. Deve ter quatro braços explícitos. O braço de `Ok` extrai `codigo` e `ms`; o de `Lento` pode usar `ms` e ignorar o código com `..`; o de `Falla` extrai a mensagem; o de `NoIntentado` devolve o texto fixo.

As quatro chamadas devem produzir estas linhas:

```text
OK 200 en 80ms
LENTO 1200ms
FALLA: sin conexión
sin revisar
```

Se um ramo devolve `&str` e os demais devolvem `String`, faça com que todos produzam o mesmo tipo. `format!` devolve `String`; para uma etiqueta fixa você pode usar `.to_string()`.

### Solução 3

Ao acrescentar `Rechazado`, a compilação deve falhar com `E0004` até que você acrescente um braço explícito. O braço correto extrai os dois campos e produz:

```text
RECHAZADO 403 en 15ms
```

A solução não consiste em trocar o último braço por `_ => "FALLA"`. Essa forma faria o programa compilar, mas perderia a diferença entre uma rede caída e um servidor que respondeu com uma política de autorização. O enum oferece uma possibilidade nova; o `match` deve convertê-la numa decisão explícita.

### Solução 4

A função de busca deve devolver `Option<&ServicioLocal>`, não `Option<ServicioLocal>`. A referência permite emprestar o serviço encontrado a partir da lista sem copiar suas strings nem movê-lo para fora do vetor. Uma busca pode percorrer os serviços e devolver o primeiro cujo `nombre` coincida; se não encontrar nenhum, devolve `None`.

Para `pagos`, `if let Some(servicio)` deve imprimir sua URL. Para `reportes`, um `match` deve incluir os dois braços e produzir:

```text
no existe reportes
```

Se o compilador reclamar de lifetimes, revise a assinatura: a referência de saída deve vir do slice de entrada. Na maioria desses casos, o Rust consegue inferir o lifetime correto sem que você o escreva. A lição 5 vai explicar os casos em que você de fato precisa declará-lo.

## Como sei que consegui

- [ ] Compilo a figura 3.1 com `rustc --edition 2024 fig03_01.rs && ./fig03_01` e obtenho as três linhas documentadas.
- [ ] Compilo a figura 3.2 e sei explicar por que cada variante de `Estado` carrega dados distintos.
- [ ] Compilo a figura 3.3, vejo `error[E0004]` e faço-a compilar acrescentando um braço explícito para `NoIntentado`.
- [ ] Consigo acrescentar uma variante ao meu enum e localizar cada decisão pendente por meio dos erros de `match`.
- [ ] Meu exercício 4 devolve `Option<&ServicioLocal>` e trata tanto `Some` quanto `None` sem `unwrap()`.
- [ ] Sei explicar por que o `revisor` usa um enum para `Estado` e um struct com `Option` para `EstadoJson`.
- [ ] Completei os exercícios `structs`, `enums` e `options` do Rustlings.

## Para ler mais

- [The Rust Programming Language, capítulo 5: Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html) — consultado em 2 de outubro de 2026.
- [The Rust Programming Language, capítulo 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) — consultado em 2 de outubro de 2026.
- [Documentação oficial de `std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html) — consultado em 2 de outubro de 2026.
- [Rustlings: exercícios de structs, enums e options](https://github.com/rust-lang/rustlings/tree/main/exercises) — consultado em 2 de outubro de 2026.
