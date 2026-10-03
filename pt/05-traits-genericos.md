# Lição 5 — Traits, genéricos e lifetimes

**Tempo:** 2 × 45 min.

**O que você constrói:** o trait `Revisor` e uma função genérica que o usa, em programas separados (o `revisor` real não declara nenhum trait).

**O que você aprende:** traits e métodos padrão, genéricos com restrições, lifetimes e o `'a` que assusta.

## Ao terminar, você vai poder

- Definir um trait, implementar seu contrato para dois tipos e usar um método padrão.
- Distinguir uma implementação inerente (`impl Tipo`) de uma implementação de trait (`impl Trait for Tipo`).
- Escrever uma função genérica com restrições e explicar quando o Rust gera código especializado.
- Escolher entre um parâmetro genérico e `dyn Trait` conforme você precise decidir o tipo em tempo de compilação ou de execução.
- Ler uma assinatura com `'a` e explicar quais referências ficam relacionadas por esse lifetime.
- Reconhecer um erro de lifetime ou de trait bound, localizar sua causa e corrigir o design sem usar cópias desnecessárias.

## O porquê antes do como

Até a lição 4, o `revisor` já tem um modelo útil. Ele consegue representar um `Servicio`, guardar uma lista num `Vec`, distinguir resultados com `Estado`, carregar configuração e reportar erros. Mesmo assim, ainda existe uma pergunta de design que aparece toda vez que o programa cresce: como você separa o que o programa precisa fazer da forma concreta como isso é feito?

O `revisor` precisa obter um `Estado` para cada `Servicio`. Hoje a implementação real faz uma consulta HTTP com `reqwest::Client`; amanhã você poderia querer uma implementação que leia um arquivo, consulte um banco de dados, meça um processo local ou simule respostas para um teste. O resto do programa não deveria precisar conhecer todos esses detalhes. Ele só precisa poder pedir: “verifique este serviço e devolva o estado dele”.

Em Go, esse contrato se expressa com uma interface. Um tipo satisfaz uma interface de forma implícita: se tem os métodos exigidos, já cumpre. Essa decisão torna muito fácil adaptar tipos existentes, mas também pode esconder relações importantes. Um tipo pode acabar cumprindo uma interface por acidente, e, ao ler sua definição, nem sempre você sabe de quais contratos ele participa em outros pacotes.

O Rust usa traits para resolver a mesma classe de problema, mas exige declarar a relação de forma explícita. Um trait descreve capacidades; depois, `impl Revisor for RevisorHttp` declara que aquele tipo cumpre essa capacidade. É uma linha a mais, mas é uma linha que documenta arquitetura. Ao lê-la você sabe que `RevisorHttp` não apenas tem um método chamado `revisar`: ele se comprometeu com o contrato `Revisor`.

Os traits não substituem os structs nem os enums. Cada ferramenta responde a uma pergunta diferente. Um `struct` diz quais dados formam uma coisa; um `enum` diz quais alternativas válidas existem; um trait diz quais operações um tipo pode oferecer. O `Servicio` da lição 3 continua sendo um struct porque modela dados. `Estado` continua sendo um enum porque um serviço pode estar saudável, lento, falhar ou não ter sido consultado. `Revisor` é um trait porque descreve a operação que produz um estado.

Os genéricos tornam possível escrever uma função que trabalha com uma família de tipos sem perder a informação sobre qual tipo concreto ela recebeu. A função `revisar_todos` desta lição pode aceitar qualquer `R` que implemente `Revisor`. Ela não precisa de um `if` para cada implementação nem de converter tudo em texto. O compilador conhece o tipo concreto de `R` ao compilar cada chamada e pode verificar que o método correto existe.

Os lifetimes completam esse modelo quando você trabalha com referências. O ownership já estabeleceu que cada valor tem um dono e que uma referência é um empréstimo. Um lifetime não cria outra forma de propriedade nem prolonga um valor. É uma anotação que ajuda o compilador a demonstrar que um empréstimo continuará válido durante todo uso possível. Aparece sobretudo quando uma função recebe referências e devolve uma referência, ou quando um struct guarda referências.

A notação `'a` intimida porque parece uma variável misteriosa, mas se lê melhor como uma etiqueta. Se uma função recebe duas referências marcadas com `'a` e devolve outra marcada com `'a`, está declarando: “a referência de saída depende destas entradas e não pode ser usada depois que deixar de ser válida a referência mais curta”. Ela não diz quanto dura `'a`; isso depende de cada chamada. Tampouco reserva memória nem faz coleta de lixo.

Esta lição corresponde ao capítulo 10 de The Rust Book. Antes de continuar, leia as seções sobre genéricos, traits e validação de referências, e faça os exercícios `generics`, `traits` e `lifetimes` do Rustlings. O objetivo não é memorizar todas as sintaxes possíveis de bounds e lifetimes. É aprender a reconhecer três perguntas: que comportamento o programa precisa, que tipos podem oferecê-lo e de onde vêm as referências que sobrevivem a uma função.

## Os conceitos

### Traits: contratos explícitos e métodos padrão

Um trait reúne assinaturas de métodos que representam uma capacidade. A assinatura diz o que o método recebe e o que devolve, sem decidir como ele fará o trabalho. Cada tipo que queira cumprir o trait escreve uma implementação própria. Por isso um trait se parece com uma interface de Go, mas sua relação com o tipo é explícita.

A figura define `Revisor` com dois métodos. `revisar` não tem corpo: toda implementação deve decidir como verificar um serviço. `nombre` tem corpo e devolve `"revisor"`. Esse é um método padrão. Uma implementação pode aceitá-lo como está, como `RevisorHttp`, ou substituí-lo, como `RevisorFalso`.

**Fig. 5.1** | Um trait com método padrão, e dois tipos que o cumprem.

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

`impl Revisor for RevisorHttp` se lê da esquerda para a direita: “implementa o trait `Revisor` para o tipo `RevisorHttp`”. Dentro desse bloco, o Rust exige implementar cada método sem corpo que o trait requer. Se você omitir `revisar`, o programa não compila. Se omitir `nombre`, compila, porque o trait já forneceu uma implementação padrão.

O método recebe `&self`, igual aos métodos de structs da lição 3. Não consome o revisor nem o modifica; apenas o empresta para consultar seus dados. `revisar` também recebe `&Servicio`, porque consultar um serviço não deve consumi-lo. O resultado, em contrapartida, é devolvido por valor: cada verificação cria um `Estado` novo e quem chama recebe sua propriedade.

O trait `Display` da figura vem da biblioteca padrão. `impl fmt::Display for Estado` permite usar `{}` dentro de `println!`. A implementação decide uma representação voltada a uma pessoa: `OK en 1ms` ou `FALLA: ...`. Isso é diferente de `Debug`, que normalmente se obtém com `#[derive(Debug)]` e se imprime com `{:?}` para diagnóstico. Se o relatório faz parte da interface do programa, definir `Display` obriga você a pensar em qual texto estável merece ser visto por quem o executa.

Um trait não é uma classe base. Não guarda campos, não constrói objetos e não herda implementação de um pai. Pode fornecer comportamento padrão, mas cada tipo conserva seus próprios dados. `RevisorHttp` tem `timeout_ms`; `RevisorFalso` não precisa de nenhum campo. Ambos cumprem o mesmo contrato porque ambos podem responder a `revisar(&Servicio)`.

O Rust aplica a regra de coerência, também chamada de regra do órfão (orphan rule). Você pode implementar um trait seu para um tipo alheio, por exemplo `impl Revisor for String`, se fizesse sentido. Também pode implementar um trait alheio para um tipo seu, como `impl Display for Estado`. O que você não pode fazer é implementar um trait alheio para um tipo alheio: você não pode decidir, a partir do seu crate, como um `Vec<String>` deve implementar `Display`. A regra evita que duas dependências diferentes definam implementações incompatíveis do mesmo contrato.

O `revisor` real não declara nenhum trait para suas consultas HTTP. Sua função `revisar` recebe um `reqwest::Client` concreto, e seus testes de integração usam um servidor HTTP local em vez de um dublê de teste. É uma decisão consciente: um trait que teria uma única implementação real ainda não resolve nenhum problema. O trait `Revisor` desta lição vive em programas separados, para que você pratique a forma; o projeto precisaria dele no dia em que existirem duas maneiras diferentes de verificar um serviço. Enquanto isso, o programa usa sim `impl` para agrupar métodos próprios dos tipos do domínio:

<!-- verificar:extracto:src/modelo.rs -->
```rust
impl Estado {
    /// `true` si el servicio contestó bien (aunque haya sido lento).
    pub fn esta_bien(&self) -> bool {
        matches!(self, Estado::Ok { .. } | Estado::Lento { .. })
    }
}
```

Este bloco é uma implementação inerente: `impl Estado`, sem `for`, acrescenta um método que pertence diretamente a `Estado`. Não implementa um trait. Distinguir as duas formas evita uma confusão comum: toda implementação de trait usa `impl`, mas nem todo `impl` implementa um trait.

Um trait seria útil no `revisor` se a aplicação precisasse trocar a fonte das verificações dentro do mesmo design. Por exemplo, um teste unitário poderia usar um revisor falso sem rede. Você não deve criar um trait só porque o Rust o oferece. A abstração tem custo de leitura: acrescenta um contrato, implementações e decisões sobre como injetá-las. O projeto atual testa HTTP por meio de um servidor local justamente porque quer verificar o comportamento real da camada HTTP.

### Genéricos e restrições: reutilizar sem apagar o tipo

Um parâmetro genérico é uma variável de tipo. Em `fn revisar_todos<R: Revisor>(...)`, `R` não significa “qualquer valor sem regras”; significa “qualquer tipo que implemente `Revisor`”. A parte depois dos dois-pontos é uma restrição, também chamada de trait bound. Graças a ela, o corpo da função pode chamar `r.revisar(s)`: o compilador tem a garantia de que qualquer `R` admitido fornece esse método.

**Fig. 5.2** | Uma função genérica com restrições.

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

A função recebe `&R`, não `R`. Isso mantém a propriedade do revisor com quem chama e permite usá-lo para todos os serviços. `servicios: &[Servicio]` é um slice emprestado, igual aos slices vistos ao percorrer coleções: a função pode ler os serviços, mas não os consome nem precisa de uma cópia do `Vec`.

`map` recebe cada `&Servicio`, chama `r.revisar(s)` e produz um iterador de estados. `collect()` reúne esses estados num `Vec<Estado>` porque o tipo de retorno assim exige. A função é genérica no revisor, mas não no estado: o contrato `Revisor` fixa que toda implementação devolve `Estado`. Essa escolha é correta quando o domínio precisa de uma única representação coerente dos resultados.

A forma `T: Display + Clone` mostra várias restrições unidas com `+`. Porém, `imprimir` só usa `Display`; não chama `clone`. A restrição `Clone` está ali para ensinar a sintaxe, não porque seja necessária. Em código de produção você deve pedir apenas as capacidades de que o corpo precisa. Um bound a mais exclui tipos válidos e faz a API parecer mais exigente do que realmente é.

Quando os bounds crescem, o Rust permite escrevê-los com `where`. Por exemplo, uma assinatura longa pode terminar com `where R: Revisor, E: std::error::Error`. Isso não muda o comportamento nem a verificação; apenas coloca as restrições onde se leem melhor. Comece com a forma curta e use `where` quando a assinatura deixar de ser clara.

Os genéricos do Rust normalmente se resolvem por monomorfização. Se você chama `revisar_todos` com `RevisorFalso` e depois com outro tipo `RevisorArchivo`, o compilador gera versões especializadas para esses tipos concretos. Em tempo de execução, ele não precisa procurar o método numa tabela para essas chamadas. Isso se chama despacho estático. O benefício é desempenho previsível e verificações mais precisas; o custo é que cada combinação de tipos pode aumentar o código compilado.

`impl Revisor` num parâmetro é uma forma curta de escrever um genérico de entrada. Uma assinatura como `fn ejecutar(r: impl Revisor)` equivale, nesse caso simples, a `fn ejecutar<R: Revisor>(r: R)`. A forma com `<R: Revisor>` é preferível quando você precisa usar o mesmo tipo genérico mais de uma vez na assinatura, devolvê-lo ou acrescentar relações entre vários parâmetros.

Quando a decisão do tipo precisa ser tomada em execução, aparece o objeto de trait (trait object): `Box<dyn Revisor>`. Um `Vec<Box<dyn Revisor>>` pode guardar na mesma coleção um `RevisorHttp`, um `RevisorFalso` e outros revisores de tamanhos diferentes. Em troca, cada chamada passa por uma indireção e o valor costuma viver atrás de um ponteiro como `Box`, `&` ou `Arc`. É o equivalente mais próximo de uma interface de Go em tempo de execução.

Não existe uma opção universalmente melhor. Use genéricos quando o tipo concreto é conhecido onde a chamada é compilada e você quer conservar essa informação. Use `dyn Trait` quando o programa precisa escolher ou combinar implementações durante a execução. Em Go, as interfaces costumam levar despacho dinâmico por seu design habitual; no Rust você escolhe explicitamente entre os dois modelos.

O projeto real também usa tipos genéricos da biblioteca padrão, embora não declare uma função própria com `<T>`. `Option<T>` expressa que pode haver ou não um valor de qualquer tipo, e aqui é especializado como `Option<u16>` para um código HTTP:

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

`Option<u16>` e `Option<String>` são usos diferentes do mesmo tipo genérico. O primeiro permite representar que uma falha não teve código HTTP; o segundo permite omitir o campo de erro quando um serviço respondeu bem. Os genéricos não são apenas uma técnica para bibliotecas sofisticadas: `Vec<T>`, `Option<T>`, `Result<T, E>` e `HashMap<K, V>` fazem parte do trabalho diário em Rust.

### Lifetimes: descrever empréstimos que se relacionam

Um lifetime é uma região de validade de uma referência. Quase sempre o Rust o infere, assim como infere muitos tipos locais. Você precisa escrever uma anotação quando a assinatura poderia permitir várias relações entre referências e o compilador não consegue saber qual delas o design garante.

A figura devolve uma de duas referências. Sem uma anotação, a assinatura não consegue comunicar se o resultado vem de `a`, de `b` ou de outro lugar. Ao marcar as três referências com `'a`, você declara que o resultado será válido durante um período que não pode superar o de nenhuma entrada escolhida.

**Fig. 5.3** | Um lifetime que une a saída às duas entradas.

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

`'a` não significa “vive para sempre” nem “vive exatamente o mesmo que ambas as entradas”. É um nome para uma relação. Numa chamada concreta, o Rust calcula um lifetime que cabe dentro dos empréstimos válidos. Se `a` dura dez linhas e `b` dura três, o resultado só poderá ser usado durante as três linhas compatíveis. A assinatura impede que alguém guarde uma referência ao resultado e depois destrua o valor de que ela veio.

A função não escolhe qual string dura mais. Isso depende dos escopos de quem chama, não da quantidade de letras nem de um valor guardado na memória. `mas_largo` compara comprimentos para escolher conteúdo, mas o lifetime fala de validade de referências. São dois assuntos diferentes que por acaso aparecem na mesma função.

O Rust tem regras de elisão de lifetimes que tornam invisíveis muitos lifetimes. Por exemplo, `fn nombre(s: &str) -> &str` compila sem escrever `'a` porque há uma única referência de entrada e o Rust pode associar a saída a ela. Também costuma inferir lifetimes de métodos que recebem `&self`. Quando há duas entradas possíveis, como em `mas_largo`, já não é seguro adivinhar e você deve descrever a relação.

Não acrescente `'a` a tudo o que pareça complicado. Uma anotação não conserta uma referência inválida; apenas declara uma relação que o compilador verificará. Se você tentar devolver uma referência a um `String` local, não existe nenhum lifetime que possa tornar válido esse empréstimo. O `String` é destruído ao terminar a função. A solução é devolver o `String` por valor, receber uma referência que pertença a quem chama ou redesenhar quem possui o dado.

O lifetime especial `'static` merece cuidado. Uma referência `&'static str` costuma apontar para texto literal incluído no binário, como `"OK"` ou `"FALLA"`. Não significa “use `'static` para eliminar erros”. Forçar `'static` num dado que na verdade vive pouco tempo não o faz durar mais; o compilador o rejeitará. Use `'static` apenas quando o valor realmente vive durante toda a execução.

O `revisor` real usa lifetimes onde eles de fato fazem falta: numa linha temporária que empresta um `Servicio` e seu `Estado` correspondente para ordenar o relatório. Ele não copia esses valores só para ordená-los. Constrói referências, guarda-as num vetor local e deixa o compilador verificar que o vetor não sobrevive às suas fontes.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub type Fila<'a> = (&'a Servicio, &'a Estado);

fn ordenadas<'a>(servicios: &'a [Servicio], estados: &'a [Estado]) -> Vec<Fila<'a>> {
    let mut filas: Vec<Fila<'a>> = servicios.iter().zip(estados).collect();
    filas.sort_by(|a, b| a.0.nombre.cmp(&b.0.nombre));
    filas
}
```

`Fila<'a>` é um alias para uma tupla de duas referências. Não possui um `Servicio` nem um `Estado`; apenas os empresta. `ordenadas` recebe dois slices com o mesmo lifetime anotado e devolve linhas que também levam esse lifetime. Portanto, ninguém pode conservar as linhas depois que os vetores originais desaparecerem. O vetor de linhas pode mudar de ordem porque é dono do vetor, mas não pode modificar os serviços nem os estados porque apenas os empresta.

A mesma função mostra uma razão prática para preferir referências: evita clonar informação só para exibi-la ordenada. Clonar seria válido se você precisasse de uma coleção independente que sobrevivesse ao relatório, mas não é necessário aqui. O relatório termina de usar `filas` antes que terminem `servicios` e `estados`, então os empréstimos expressam exatamente o modelo de dados.

Quando um lifetime aparece num struct ou alias, você não deve lê-lo como sintaxe cerimonial. Pergunte: “este tipo guarda uma referência?” Se a resposta é sim, a anotação vincula o tipo à duração do valor emprestado. Se a resposta é não, provavelmente o tipo deveria possuir um `String`, `Vec<T>` ou outro valor e não precisa de lifetime explícito.

## O erro que você vai ver

### E0515: devolver uma referência a um valor local

Este erro aparece quando uma função tenta emprestar algo que deixa de existir ao retornar. A figura seguinte falha de propósito. O lifetime `'a` da assinatura não pode salvar `nombre`: esse `String` é propriedade de `devolver` e é destruído ao fechar a função.

**Fig. 5.4** | Um empréstimo que tenta escapar do valor que o possui.

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

A mensagem aponta exatamente a referência que pretende escapar. Não acrescente outro lifetime nem use `&'static str`: nenhum dos dois muda quem possui `nombre`. Se a função precisa criar o texto, a correção é devolver `String`. Se precisa devolver uma visão de um texto que já existia, receba `&str` como parâmetro e relacione o lifetime de saída com essa entrada.

### E0277: o tipo não cumpre a restrição pedida

Um trait bound também é um contrato verificável. Nesta figura, `imprimir` pede um tipo que implemente `Display`, mas `Vec<&str>` não tem essa implementação. O Rust não o converte em texto de forma implícita porque não existe uma única representação correta para todas as coleções.

**Fig. 5.5** | Um argumento que não cumpre o trait bound.

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

`E0277` diz que uma restrição de trait não foi cumprida. A nota leva você à assinatura que introduziu a exigência. A correção depende da intenção: você pode imprimir com `{:?}` se quer uma representação de depuração e o tipo implementa `Debug`; pode percorrer o vetor e imprimir cada elemento; ou pode convertê-lo explicitamente num `String` com o formato de que seu relatório precisa. Não implemente `Display` para um tipo alheio só para calar o erro: a regra do órfão vai impedir e, além disso, seria uma decisão global difícil de justificar.

## O que se faz errado

### Criar um trait para cada struct

Um trait deve representar uma capacidade compartilhada, não repetir o nome de um tipo. Se existe apenas uma implementação e não há uma razão concreta para trocá-la, um método inerente costuma ser mais claro. `impl Estado { fn esta_bien(...) }` expressa que a operação pertence naturalmente a `Estado`. Criar um trait `EstadoConsultable` para uma única função só acrescenta nomes e arquivos sem separar uma dependência real.

Comece com structs, enums e funções diretas. Extraia um trait quando várias implementações precisarem cumprir o mesmo contrato, quando você precisar receber uma capacidade em vez de um tipo concreto ou quando uma fronteira de testes realmente o justificar.

### Acrescentar bounds “por via das dúvidas”

É comum copiar uma assinatura como `T: Clone + Debug + Display` e conservar todos os bounds mesmo que o corpo só use `Display`. Cada bound limita os tipos que podem chamar a função. Além disso, cada capacidade prometida por uma assinatura passa a fazer parte da API que outras pessoas devem entender.

Peça `Clone` só se o corpo chama `clone`, `Ord` só se ordena e `Send` só se move dados para outra thread. Um bound pequeno é uma abstração mais flexível e descreve melhor a necessidade real. A figura 5.2 conserva `Clone` para mostrar que é possível combinar restrições, mas não é um modelo para copiar literalmente.

### Usar `Box<dyn Trait>` por costume

Um objeto de trait resolve um problema real: armazenar ou escolher implementações diferentes em tempo de execução. Não é a forma obrigatória de usar traits. Se o tipo é conhecido na compilação, um parâmetro genérico normalmente é mais simples, evita alocações desnecessárias no heap e permite despacho estático.

A pergunta útil não é “traits ou genéricos?”. Um trait descreve uma capacidade; depois você escolhe se a recebe com genéricos, como `impl Trait`, por meio de uma referência `&dyn Trait` ou atrás de um `Box<dyn Trait>`. A escolha depende de propriedade, tamanho e do momento em que você conhece o tipo.

### Clonar para calar erros do borrow checker

Se uma referência não vive o suficiente, copiar um `String` com `.clone()` pode fazer o programa compilar, mas nem sempre resolve o design correto. Às vezes só esconde que uma função deveria devolver uma referência, que um tipo deveria possuir seus dados ou que um empréstimo dura mais do que o necessário.

Faça primeiro o diagnóstico: identifique o dono, identifique quem precisa usar o dado depois e decida se precisa de uma visão ou de uma cópia independente. Clone quando dois donos legítimos precisam conservar valores separados. O vetor de linhas do `revisor` não clona serviços nem estados porque só precisa ordená-los enquanto seus donos continuam vivos.

### Ler `'a` como uma duração concreta

`'a` não significa um segundo, um escopo fixo nem uma variável criada no início do programa. É uma etiqueta que o Rust substitui por uma região válida em cada chamada. Duas funções podem usar o nome `'a` sem compartilhar absolutamente nada; o nome só tem significado dentro da própria assinatura.

Também é um erro pensar que mais anotações são mais seguras. As anotações devem refletir de onde vem uma referência. Se você não consegue explicar qual referência de entrada sustenta a saída, provavelmente a função deve devolver um valor próprio em vez de uma referência.

## Exercícios

### Exercício 1 — Um revisor falso com nome padrão

Defina um trait `Revisor` com `revisar(&self, servicio: &Servicio) -> Estado` e um método padrão `nombre() -> String`. Crie `RevisorFalso`, que devolva `Estado::Ok { ms: 1 }` sem substituir `nombre`. Confirme que imprime `revisor -> OK en 1ms`.

Depois acrescente `RevisorArchivo`, que substitua `nombre` por `"archivo"` e devolva uma falha determinística. Explique em uma frase por que os dois tipos podem ser usados onde se espera um `Revisor`.

### Exercício 2 — Contar resultados saudáveis de forma genérica

Use o trait da figura 5.1 e escreva uma função `contar_sanos<R: Revisor>`. Ela deve receber um revisor e um slice de serviços, verificá-los e devolver quantos estados são `Ok`. Teste a função com três serviços e `RevisorFalso`.

Antes de programar, decida o que cada parte deve possuir: a função não deve consumir o revisor nem o vetor de serviços. Use `&R` e `&[Servicio]`, sem clonar.

### Exercício 3 — Ler o lifetime do relatório

Abra `programas/revisor/src/reporte.rs` e localize `Fila<'a>` e `ordenadas<'a>`. Escreva com suas palavras o que o `Vec<Fila<'a>>` possui, o que toma emprestado e o que aconteceria se você tentasse devolver essas linhas depois de destruir `servicios` ou `estados`.

Em seguida, escreva uma função `primero<'a>` que receba `&'a str` e devolva `&'a str`. Compare-a com `devolver` da figura 5.4 e explique por que uma compila e a outra não.

## Soluções

### Solução 1

A implementação que aceita o método padrão não escreve `nombre`; o trait fornece o corpo. A segunda implementação o substitui porque precisa de uma etiqueta diferente.

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

Os dois tipos podem ser usados onde se espera um `Revisor` porque ambos escreveram `impl Revisor for ...` e fornecem o método obrigatório `revisar`. A diferença entre seus dados e seu algoritmo fica encapsulada dentro de cada implementação.

### Solução 2

A função toma empréstimos porque só precisa consultar os dados. Cada resultado é temporário: é contado e descartado. Não há razão para armazenar um `Vec<Estado>` nem para clonar o revisor ou os serviços.

<!-- verificar:fragmento -->
```rust
fn contar_sanos<R: Revisor>(revisor: &R, servicios: &[Servicio]) -> usize {
    servicios
        .iter()
        .filter(|servicio| matches!(revisor.revisar(servicio), Estado::Ok { .. }))
        .count()
}
```

`iter()` produz `&Servicio`; o closure recebe cada empréstimo e chama o trait por meio de `&R`. `matches!` decide se o estado pertence à variante `Ok`, e `count()` devolve o total. Se `Estado` tivesse também `Lento`, você deveria decidir explicitamente se ele conta como saudável; o `revisor` real responde a essa pergunta com `Estado::esta_bien()`.

### Solução 3

`Vec<Fila<'a>>` possui o vetor e a ordem de seus elementos, mas não possui os serviços nem os estados. Cada elemento contém duas referências. Por isso suas linhas só podem viver enquanto os slices emprestados a `ordenadas` continuarem vivos. Tentar devolvê-las para usá-las depois de destruir as coleções originais produziria um erro do borrow checker: seriam referências pendentes (dangling).

A função correta devolve uma referência que pertence a quem chama:

<!-- verificar:fragmento -->
```rust
fn primero<'a>(texto: &'a str) -> &'a str {
    texto
}
```

`primero` não cria o texto nem tenta emprestá-lo depois de destruí-lo. Apenas devolve o mesmo empréstimo que recebeu. Já `devolver` cria um `String` local, é seu dono e o destrói ao sair; por isso a referência da figura 5.4 não pode escapar.

## Como sei que consegui

- `rustc --edition 2024 fig05_01.rs && ./fig05_01` imprime as duas linhas documentadas, incluindo a etiqueta padrão de `RevisorHttp`.
- `rustc --edition 2024 fig05_02.rs && ./fig05_02` imprime `OK en 8ms` e `OK en 5ms`.
- `rustc --edition 2024 fig05_03.rs && ./fig05_03` imprime `catalogo`.
- `rustc --edition 2024 fig05_04.rs` falha com `error[E0515]`; você consegue explicar por que mudar `'a` não conserta o empréstimo local.
- `rustc --edition 2024 fig05_05.rs` falha com `error[E0277]`; você consegue localizar tanto a chamada incorreta quanto o bound que a rejeitou.
- Você consegue apontar `Fila<'a>` em `programas/revisor/src/reporte.rs` e explicar que ela empresta serviços e estados em vez de cloná-los.
- Você terminou os exercícios `generics`, `traits` e `lifetimes` do Rustlings.

## Para ler mais

- [The Rust Programming Language, capítulo 10.1: sintaxe de tipos genéricos](https://doc.rust-lang.org/book/ch10-01-syntax.html), consultado em 2 de outubro de 2026.
- [The Rust Programming Language, capítulo 10.2: traits](https://doc.rust-lang.org/book/ch10-02-traits.html), consultado em 2 de outubro de 2026.
- [The Rust Programming Language, capítulo 10.3: validação de referências com lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), consultado em 2 de outubro de 2026.
- [Rustlings: exercícios de genéricos, traits e lifetimes](https://rustlings.rust-lang.org/), consultado em 2 de outubro de 2026.
