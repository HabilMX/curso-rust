# Lição 6 — Módulos, testes e Cargo

**Tempo:** 2 × 45 min.

**O que você constrói:** o projeto `revisor` organizado e com testes.

**O que você aprende:** módulos e visibilidade, testes unitários e de integração, `cargo test`, dependências e versões.

**The Rust Book, capítulos 7, 11 e 14.** Rustlings: `modules`, `tests`.

## Ao terminar, você vai poder

- Separar um programa Rust em módulos com responsabilidades claras e navegar por seus caminhos com `crate`, `self` e `super`.
- Explicar por que tudo é privado por padrão e escolher entre `pub`, `pub(crate)` e uma API privada.
- Distinguir um teste unitário de um teste de integração e saber que tipo de problema cada um detecta.
- Escrever testes com `#[test]`, `assert!`, `assert_eq!`, `matches!` e `#[should_panic]`.
- Executar, filtrar e diagnosticar testes com `cargo test`.
- Ler `Cargo.toml` e `Cargo.lock`, adicionar uma dependência com uma versão razoável e revisar sua árvore transitiva.
- Manter o `revisor` verificável com `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` e `cargo test`.

## O porquê antes do como

Até agora o `revisor` coube em poucos arquivos porque o curso estava apresentando as peças da linguagem uma a uma. Você já conhece os tipos que modelam um serviço, as coleções que guardam a lista, os erros que descrevem falhas externas e os traits que expressam contratos. O próximo problema não é escrever outra função: é evitar que essas funções se transformem numa massa única, difícil de ler, testar e mudar.

Um arquivo enorme não deixa de funcionar automaticamente. O problema aparece quando uma modificação aparentemente local obriga a entender coisas demais ao mesmo tempo. Se o código que lê YAML, valida serviços, faz requisições HTTP, produz JSON e interpreta argumentos vive misturado, um teste de formato pode acabar exigindo rede; uma modificação de configuração pode afetar o binário; e uma função privada pode acabar sendo usada de qualquer lugar só porque ninguém definiu uma fronteira.

Os módulos são essas fronteiras. Não são pastas para que o projeto “pareça organizado”; são nomes para responsabilidades e, em Rust, também uma parte explícita do controle de acesso. Um módulo pode dizer: “aqui se define o que é um serviço”, “aqui se carrega a configuração” ou “aqui um estado é convertido numa tabela”. Quem usa um módulo conhece sua interface pública; não precisa, nem deveria, depender dos detalhes internos com que ele é implementado.

A palavra importante é interface. Em Go, uma pasta define um pacote e uma inicial maiúscula decide se um nome pode cruzar a fronteira do pacote. O Rust é mais detalhado. Uma pasta pode ajudar a organizar arquivos, mas a visibilidade depende de módulos e de `pub`. Um nome sem `pub` é privado, mesmo que esteja em outro arquivo do mesmo projeto. Isso parece rígido no começo, mas evita que uma função auxiliar vire, por acidente, uma promessa para o resto do programa.

O `revisor` aplica essa ideia com dois produtos dentro do mesmo pacote Cargo. `src/lib.rs` declara uma biblioteca: ali vive a lógica reutilizável e testável. `src/main.rs` declara o binário: recebe argumentos, chama a biblioteca, imprime o resultado e decide o código de saída. Separar os dois permite testar a lógica sem precisar invocar a linha de comando em cada caso. Também permite que os testes de integração usem o `revisor` como o usaria outra aplicação: importando exclusivamente sua API pública.

Este é o mesmo princípio que você usou em Go ao separar pacotes por responsabilidade, mas o Rust torna o contrato mais visível. Em Go, uma função com inicial minúscula não pode ser importada de outro pacote; em Rust, uma função, um struct, um campo ou um módulo exige visibilidade declarada. As duas decisões buscam limitar dependências. O Rust oferece mais níveis para expressar essa intenção: público para qualquer um que importe a biblioteca, público apenas dentro do pacote atual ou público para um módulo pai.

Os testes transformam essas fronteiras em algo verificável. Um teste unitário vive perto da função que testa e pode examinar detalhes privados. É útil para regras pequenas: se o YAML não declara `timeout_ms`, o valor padrão é aplicado? Uma lista vazia é rejeitada? A tabela conserva seu alinhamento? Um teste de integração vive em `tests/`, é compilado como outro crate e só pode usar `pub`. É útil para verificar se a interface realmente basta: se alguém constrói um `Servicio`, chama `revisar` e recebe um `Estado`, o contrato público funciona sem depender de detalhes internos?

Não confunda muitos testes com boa cobertura de decisões. Uma suíte pode ter cem testes que repetem o mesmo caso saudável e nenhum que cubra uma URL inválida, um arquivo ausente ou um serviço que demora demais. Tampouco transforme o percentual de cobertura numa meta isolada. A pergunta útil é: “que comportamento importante poderia quebrar sem que um teste ficasse vermelho?” O `revisor` testa estados saudáveis, HTTP 500, tempos de espera, configuração inválida, formato JSON e códigos de saída porque esses são comportamentos que importam a quem usa o programa.

O `cargo` reúne essas decisões. Ele não apenas compila: sabe quais arquivos formam o pacote, de quais dependências precisa, qual edição do Rust usa, quais testes existem e quais artefatos deve construir. Nas figuras do curso você continua chamando o `rustc` diretamente para ver um exemplo isolado. No projeto real você usa o `cargo` porque já não existe uma invocação manual razoável que se lembre de todos os módulos, crates, features e alvos de teste.

A disciplina desta lição é simples: organize por responsabilidade, abra a menor superfície pública necessária e teste cada fronteira pelo lado correto. Se um teste unitário precisa de rede, provavelmente você misturou uma regra pura com infraestrutura. Se um teste de integração precisa importar um detalhe privado, provavelmente sua API pública não expressa o que outro consumidor precisa. Se o `cargo test` diz que não encontrou testes, não se parabenize ainda: confira a contagem.

## Os conceitos

### Módulos: nomes, caminhos e responsabilidades

Um módulo agrupa nomes relacionados. Pode ser declarado dentro de um arquivo com `mod nombre { ... }`, ou pode viver em outro arquivo. Num pacote Cargo moderno, `src/lib.rs` e `src/main.rs` são raízes de crate distintas. A partir de qualquer uma delas, `crate` significa “a raiz deste crate”; `self` significa o módulo atual; e `super` significa o módulo pai.

Um erro frequente é pensar que arquivo e módulo são sinônimos. Um arquivo pode conter vários módulos, e um módulo pode ser aberto em outro arquivo. A estrutura de arquivos ajuda uma pessoa a encontrar código; a estrutura de módulos determina como o Rust resolve caminhos e aplica visibilidade. Não desenhe primeiro uma árvore de pastas vazia. Comece por responsabilidades que tenham uma razão estável para mudar separadamente.

**Fig. 6.1** | Um módulo oferece uma função pública e conserva seu detalhe privado.

```rust
// fig06_01.rs
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn linea(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::linea("catalogo", true));
    println!("{}", reporte::linea("pagos", false));
}
```

```bash
$ rustc --edition 2024 fig06_01.rs && ./fig06_01
catalogo: OK
pagos: FALLA
```

`reporte::linea` é acessível a partir de `main` porque tem `pub`. A função `etiqueta` não tem `pub`, então só pode ser usada dentro de `reporte`. Essa decisão não esconde informação por mistério: expressa que as outras partes do programa precisam de uma linha pronta, não de conhecer a regra interna que traduz um booleano em texto. Se mais adiante você trocar `"FALLA"` por `"NO DISPONIBLE"`, só o módulo dono precisa mudar.

No `revisor`, a raiz da biblioteca enumera suas responsabilidades públicas. Não há um módulo chamado `utilidades`, porque esse nome não explica que responsabilidade possui. `config` carrega e valida a configuração; `modelo` define o vocabulário; `reporte` traduz estados em texto ou JSON; `revisar` consulta serviços.

<!-- verificar:extracto:src/lib.rs -->
```rust
//! El `revisor` del curso de Rust: recibe una lista de servicios, los consulta
//! todos a la vez y produce un reporte.
//!
//! La lógica vive aquí, en la biblioteca, y `main.rs` solo lee los argumentos y
//! llama (lección 6): así todo lo de abajo se puede probar desde fuera.
//!
//! - [`modelo`]: el vocabulario (`Servicio`, `Estado`, `EstadoJson`).
//! - [`config`]: lee y valida el archivo YAML de servicios.
//! - [`revisar`]: consulta un servicio por HTTP, o todos a la vez con un límite.
//! - [`reporte`]: convierte los estados en tabla o en JSON.

pub mod config;
pub mod modelo;
pub mod reporte;
pub mod revisar;
```

A palavra `pub` diante de cada `mod` faz com que esses módulos formem a entrada pública da biblioteca. Isso não torna público todo o seu conteúdo. Cada módulo decide, por sua vez, quais structs, funções e constantes expõe. Essa composição é uma vantagem: publicar `reporte` permite chamar `revisor::reporte::tabla`, mas não obriga a publicar as funções auxiliares que ordenam linhas ou calculam etiquetas.

A árvore de módulos do `revisor` não pretende ser uma hierarquia universal. Num projeto pequeno, quatro módulos planos são mais legíveis do que uma longa cadeia de pastas. Quando uma responsabilidade cresce o bastante, pode ser dividida em submódulos. A pergunta não é “quantos arquivos um projeto profissional deve ter?”, mas “consigo descrever em uma frase o que pertence aqui e o que não pertence?”.

### Visibilidade: privado por padrão como design

O Rust começa fechado. Um item sem `pub` é visível em seu módulo e nos descendentes, mas não para módulos irmãos nem para o pai. Essa regra é mais restritiva do que muitos programadores esperam depois de JavaScript, Python ou Go, onde uma função de arquivo costuma ser acessível dentro do pacote. A intenção é obrigar você a desenhar a interface antes de depender de um detalhe.

`pub` abre um item para quem consiga chegar ao módulo que o contém. `pub(crate)` abre o item para todo o crate atual, mas não para quem importe a biblioteca a partir de outro pacote. `pub(super)` abre o item somente para o módulo pai. Existe também `pub(in caminho)`, útil quando uma fronteira precisa entre módulos expressa uma regra real, embora seja menos comum em projetos pequenos.

Não marque tudo com `pub` para calar erros de visibilidade. Fazer isso tem um custo: qualquer consumidor pode começar a depender desses nomes, e depois mudar uma função interna se torna uma quebra de API. Para um binário privado, esse custo fica dentro do repositório; para uma biblioteca publicada, pode obrigar você a manter uma decisão acidental durante anos. Comece privado e abra só o que outra parte precisa.

O compilador distingue um nome que não existe de um nome que existe mas está fechado. Neste caso a função existe, mas `main` tenta atravessar uma fronteira privada.

**Fig. 6.2** | Acessar uma função privada produz `E0603`.

```rust
// fig06_02.rs
mod config {
    fn ruta_por_omision() -> &'static str {
        "servicios.yaml"
    }
}

fn main() {
    println!("{}", config::ruta_por_omision());
}
```

```bash
$ rustc --edition 2024 fig06_02.rs
error[E0603]: function `ruta_por_omision` is private
 --> fig06_02.rs:9:28
  |
9 |     println!("{}", config::ruta_por_omision());
  |                            ^^^^^^^^^^^^^^^^ private function
  |
note: the function `ruta_por_omision` is defined here
 --> fig06_02.rs:3:5
  |
3 |     fn ruta_por_omision() -> &'static str {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

O conserto mecânico seria escrever `pub fn ruta_por_omision`. Antes de fazê-lo, pergunte se o caminho padrão deve fazer parte do contrato de `config`. Se outro módulo realmente precisa consultá-lo, pode ser uma função pública razoável. Se você só quer que `main` carregue o arquivo normal, talvez convenha que `config` ofereça uma função pública de mais alto nível e conserve essa string como detalhe privado.

O módulo `modelo` do `revisor` mostra uma API pública selecionada. `Servicio` é público porque a configuração, os testes de integração e outros módulos precisam construí-lo. Seus campos são públicos porque o programa precisa ler e modificar os dados declarados. A constante de timeout padrão, em contrapartida, permanece privada: quem usa `Servicio::new` obtém a regra sem depender de como ela está armazenada.

<!-- verificar:extracto:src/modelo.rs -->
```rust
/// Cuánto se le espera a un servicio que no declara su propio tiempo límite.
const TIMEOUT_POR_OMISION_MS: u64 = 5000;

/// A partir de cuántos milisegundos una respuesta sana se reporta como lenta.
pub const UMBRAL_LENTO_MS: u64 = 1000;

fn timeout_por_omision() -> u64 {
    TIMEOUT_POR_OMISION_MS
}

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

Observe a diferença entre expor uma constante e expor uma função. `UMBRAL_LENTO_MS` é uma regra de que outros módulos precisam para classificar respostas. `timeout_por_omision` só existe para que o `serde` possa chamar o valor padrão dentro do modelo. Publicá-lo não dá nenhuma capacidade útil ao consumidor e amplia a superfície que seria preciso manter.

### Testes unitários: uma propriedade pequena, perto do código

Um teste unitário verifica uma unidade de comportamento em seu próprio módulo. O Rust os escreve normalmente dentro de um módulo `tests` marcado com `#[cfg(test)]`. Esse atributo indica que o módulo só é compilado ao construir os testes. O binário de produção não carrega essas funções nem seus auxiliares.

`use super::*` importa em `tests` os nomes do módulo pai. Isso permite testar detalhes privados de propósito. Não é uma trapaça contra a visibilidade: o teste vive como descendente do mesmo módulo e está verificando a implementação interna. Um teste de integração terá outra restrição, porque representa um consumidor externo.

As asserções principais são `assert!`, para uma condição booleana; `assert_eq!`, para comparar esperado e obtido; e `assert_ne!`, para afirmar que dois valores não são iguais. Todas aceitam uma mensagem adicional com formatação. `matches!` é especialmente útil com enums: permite verificar a variante e, se for preciso, uma condição sobre os dados que ela carrega.

**Fig. 6.3** | Testes unitários, uma asserção de enum e um pânico esperado.

```rust
// fig06_03.rs
enum Estado {
    Ok { codigo: u16, ms: u64 },
    Falla(String),
}

fn resumen(e: &Estado) -> String {
    match e {
        Estado::Ok { codigo, ms } => format!("OK {codigo} en {ms}ms"),
        Estado::Falla(msg) => format!("FALLA: {msg}"),
    }
}

fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("dividir por cero");
    }
    a / b
}

fn main() {
    println!("{}", resumen(&Estado::Ok { codigo: 200, ms: 100 }));
    println!("{}", dividir(10, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estado_ok_con_200() {
        let e = Estado::Ok { codigo: 200, ms: 100 };
        assert!(matches!(e, Estado::Ok { .. }));
    }

    #[test]
    fn falla_sin_codigo() {
        assert_eq!(resumen(&Estado::Falla("x".into())), "FALLA: x");
    }

    #[test]
    #[should_panic(expected = "dividir por cero")]
    fn panico_esperado() {
        dividir(1, 0);
    }
}
```

```bash
$ rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1
running 3 tests
test tests::estado_ok_con_200 ... ok
test tests::falla_sin_codigo ... ok
test tests::panico_esperado - should panic ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`#[should_panic]` não significa que os pânicos sejam uma forma normal de lidar com dados inválidos. No `revisor`, uma URL incorreta deve terminar como `Result` e uma mensagem para quem invocou o programa, não como um pânico. O teste de pânico serve quando um contrato interrompe deliberadamente a execução diante de um invariante quebrado. O argumento `expected` importa: confirma que o pânico veio do motivo esperado e não de outra falha acidental.

Um teste deve descrever comportamento, não implementação incidental. O nome `falla_sin_codigo` comunica uma regra do domínio: uma falha é mostrada sem código HTTP. Um nome como `prueba_resumen_2` só diz que alguém escreveu um teste. Quando ele falhar daqui a meses, o nome será a primeira pista para entender qual decisão do programa mudou.

Em `config.rs`, os testes unitários não fazem requisições HTTP nem executam o binário. Constroem dados pequenos e chamam `validar`, que é a unidade responsável por verificar a lista. Isso mantém a suíte rápida e faz com que cada falha aponte para uma regra concreta.

<!-- verificar:fragmento -->
```rust
#[test]
fn validar_rechaza_nombre_repetido() {
    let v = vec![
        Servicio::new("a", concat!("http", "://x")),
        Servicio::new("a", concat!("http", "://y")),
    ];
    let err = validar(&v).unwrap_err().to_string();
    assert!(err.contains("repetido"), "mensaje: {err}");
}

#[test]
fn validar_rechaza_url_sin_esquema() {
    let v = vec![Servicio::new("a", "localhost:80")];
    assert!(validar(&v).is_err());
}

#[test]
fn validar_rechaza_timeout_cero() {
    let mut s = Servicio::new("a", concat!("http", "://x"));
    s.timeout_ms = 0;
    assert!(validar(&[s]).is_err());
}
```

O primeiro teste inspeciona parte da mensagem porque aqui o texto faz parte da experiência de quem corrige o YAML. Os outros só verificam que existe um erro. Nem todos os testes precisam comparar strings completas. Compare o detalhe exato quando ele for contrato público; para erros internos, verificar o tipo, uma condição ou a existência do erro costuma produzir testes menos frágeis.

### Testes de integração: a biblioteca vista de fora

O Cargo reconhece `tests/` como o lugar dos testes de integração. Cada arquivo Rust diretamente dentro dessa pasta é compilado como um crate distinto. Por isso ele não pode usar funções privadas, nem importar o módulo interno `tests` da biblioteca, nem supor detalhes de arquivos. Só pode usar o que a biblioteca exporta com `pub`.

Essa limitação é útil. Uma API pode ter excelentes testes unitários e ainda assim ser incômoda ou insuficiente para quem tenta usá-la de fora. Os testes de integração encontram esse problema porque atravessam a mesma fronteira que atravessaria outro binário. Se você precisa quebrar o encapsulamento para escrevê-los, primeiro verifique se falta uma operação pública razoável; não torne tudo `pub` como reação automática.

O `revisor` tem um arquivo `tests/integracion.rs`. Ele importa os tipos e funções de que um consumidor público precisa: `Estado`, `Servicio`, `revisar` e `revisar_todos`. Não importa funções privadas que constroem requisições nem detalhes do `reqwest`.

<!-- verificar:fragmento -->
```rust
use revisor::modelo::{Estado, Servicio};
use revisor::revisar::{revisar, revisar_todos};

fn servicio(nombre: &str, direccion: &str, ruta: &str, timeout_ms: u64) -> Servicio {
    Servicio {
        nombre: nombre.to_string(),
        url: ["http:", "//", direccion, ruta].concat(),
        timeout_ms,
    }
}
```

O helper `servicio` pertence ao teste, não à biblioteca, porque só existe para tornar legíveis os casos de teste. É uma distinção saudável: não promova uma função à produção só porque dois testes a repetem. A biblioteca deve conter capacidades do programa; a suíte pode conter pequenas ferramentas para preparar cenários.

O teste seguinte sobe um servidor HTTP local definido em `tests/comun/mod.rs`, chama a API pública e verifica a variante resultante. Não depende de um serviço real na internet, de uma conta nem de um horário específico. Isso evita que uma falha de rede transforme um teste determinístico num alarme falso.

<!-- verificar:extracto:tests/integracion.rs -->
```rust
#[tokio::test]
async fn un_500_es_falla_con_su_codigo() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let e = revisar(&cliente, &servicio("mal", &d, "/error", 2000)).await;
    assert!(
        matches!(&e, Estado::Falla { motivo, .. } if motivo == "codigo 500"),
        "estado: {e:?}"
    );
}
```

`#[tokio::test]` aparece porque a função `revisar` é assíncrona. A lição 7 aprofunda o que significa esperar um future e como funciona o runtime. Aqui importa reconhecer a fronteira: o teste de integração usa a mesma API assíncrona que o binário usará, mas substitui a internet por um servidor local controlado.

Além dos testes de integração, o projeto tem testes de binário. Estes executam o `revisor` compilado, passam a ele um YAML temporário e conferem `stdout`, `stderr` e o código de saída. São mais lentos e mais amplos que um unitário, então não substituem os outros; verificam a última fronteira, onde argumentos, configuração, relatórios e saída do processo se encontram.

### `cargo test`: construir, selecionar e ler resultados

O `cargo test` descobre os testes unitários, os de integração, os de binários e os de documentação; compila os alvos necessários e executa cada conjunto. É mais que uma abreviação de `rustc --test`: o Cargo conhece as dependências e constrói cada crate com os caminhos corretos.

Os comandos que você usará com mais frequência são estes:

```bash
cargo test
cargo test validar_rechaza_nombre_repetido
cargo test --test integracion
cargo test -- --nocapture
cargo test --release
```

O primeiro comando executa tudo. O segundo filtra por uma parte do nome do teste; é útil para trabalhar numa única regra sem esperar a suíte inteira. O terceiro seleciona especificamente o arquivo de integração chamado `integracion`. O `--` separa as opções do Cargo das opções do executor de testes: `--nocapture` permite ver os `println!` de um teste que passa, algo útil para diagnóstico temporário, não como substituto de uma asserção. `--release` compila com otimizações; use-o quando o comportamento realmente depender do perfil ou quando estiver medindo desempenho, não como modo diário.

Os testes podem rodar em paralelo. Isso é correto se cada um cria seus próprios dados e não depende da ordem de execução. Se você está diagnosticando a saída ou um teste compartilha um recurso que ainda não consegue isolar, use:

```bash
cargo test -- --test-threads=1
```

Não transforme essa flag em hábito. Uma suíte que só funciona em série pode esconder um estado global ou arquivos temporários com nomes que colidem. No `revisor`, os servidores de teste pedem ao sistema uma porta livre e cada caso usa seus próprios dados; isso permite executar os testes sem depender de uma ordem específica.

A armadilha mais simples é que o `cargo test` pode terminar com sucesso sem ter executado um teste relevante. Um filtro mal escrito pode produzir uma saída com testes filtrados; um crate pode não ter nenhum `#[test]`; e um arquivo colocado fora de `tests/` pode não ser uma integração. Leia sempre as linhas `running N tests` e `test result`. O código de saída zero significa que o executor não encontrou uma falha, não que sua intenção ficou comprovada.

O projeto mantém a lógica na biblioteca e a inicialização no binário. O binário importa a API pública como qualquer outro consumidor interno. Essa separação é a razão pela qual os testes de integração podem importar `revisor` com o mesmo nome.

<!-- verificar:extracto:src/main.rs -->
```rust
use std::process::ExitCode;

use revisor::{config, reporte, revisar};

use clap::Parser;
```

O caminho `revisor::{config, reporte, revisar}` não usa `crate::` porque `main.rs` é outro crate dentro do mesmo pacote. Da perspectiva do binário, `revisor` é a biblioteca declarada por `src/lib.rs`. É uma pequena diferença de sintaxe com uma consequência importante de design: o binário não tem privilégios para chegar aos detalhes privados da biblioteca.

### Dependências, versões e o trabalho do `cargo`

`Cargo.toml` é o manifesto declarativo do pacote. Diz como ele se chama, qual edição usa, de quais dependências diretas precisa e quais perfis de construção existem. `Cargo.lock` registra a resolução concreta: as versões exatas das dependências diretas e transitivas que o Cargo escolheu quando construiu o projeto.

O `revisor` não depende apenas da biblioteca padrão. Isso é deliberado: YAML, HTTP assíncrono, JSON e uma linha de comando completa vivem em crates especializados. As dependências declaradas são as que o projeto realmente usa.

<!-- verificar:extracto:Cargo.toml -->
```toml
[dependencies]
anyhow = "1.0.104"
clap = { version = "4.6.7", features = ["derive"] }
futures = "0.3.34"
reqwest = { version = "0.13.5", features = ["json"] }
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
yaml_serde = "0.10.7"
tokio = { version = "1.53.1", features = ["full"] }
```

Uma nota sobre uma dessas linhas. Até 2024, o crate mais usado para ler YAML com o `serde` era o `serde_yaml`. Seu autor, David Tolnay, deixou de mantê-lo: sua última versão é a `0.9.34+deprecated`, de março de 2024, e o crates.io a marca como obsoleta. Ainda compila e funciona, mas já não recebe correções nem melhorias, então não convém começar um projeto novo com ele. O `revisor` usa `yaml_serde`, uma continuação publicada pela organização do YAML no GitHub: seu repositório a apresenta como o fork mantido do `serde_yaml` e promete a mesma interface. Por isso a mudança quase não toca o código: o que você sabe de `serde_yaml::from_str` serve igualmente com `yaml_serde::from_str`. Existem outros forks e alternativas; antes de escolher um, veja a data de sua última versão e se seu repositório continua recebendo mudanças. (Dados consultados no crates.io em 2 de outubro de 2026.)

Uma versão como `"1.0.104"` não fixa por si só cada dígito para sempre. No Cargo, essa especificação usa compatibilidade semântica com o operador circunflexo (caret) implícito: permite atualizações compatíveis dentro da mesma versão maior. O `Cargo.lock` é o que torna repetível a compilação concreta do binário. Por isso o lockfile do `revisor` deve ir para o repositório: uma pessoa que clone a aplicação deve resolver as mesmas versões conhecidas, não uma combinação nova que hoje pareça compatível.

Para uma biblioteca publicada, a resposta é menos categórica. O `cargo new` registra o `Cargo.lock` no repositório por padrão, e as perguntas frequentes do Cargo (o [Cargo FAQ](https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control)) dizem que versioná-lo ou não depende do que seu pacote precisa. Versioná-lo dá compilações repetíveis: ajuda a encontrar com `git bisect` qual mudança introduziu um erro, a fazer com que a integração contínua falhe apenas por commits novos e não por uma dependência que mudou do lado de fora, e a verificar com versões conhecidas coisas como a versão mínima do Rust ou o texto exato das mensagens de erro. Mas esse arquivo não protege quem usa sua biblioteca: os consumidores resolvem as dependências com o que seu `Cargo.toml` declara e com seu próprio `Cargo.lock`, e o `cargo install` ignora por padrão o `Cargo.lock` do pacote e escolhe as versões compatíveis mais recentes, a menos que você passe `--locked`. Em resumo: uma aplicação como o `revisor` convém versionar sempre, porque é o produto final que você quer reproduzir; para uma biblioteca, decida conforme o que você quer garantir e, se não a versionar, teste de vez em quando com as dependências mais novas.

Adicione uma dependência com o Cargo em vez de escrever à mão uma linha que você não entende:

```bash
cargo add serde --features derive
cargo add tokio --features full
cargo tree
cargo update
```

O `cargo add` atualiza o manifesto e resolve o lockfile. As características, ou *features*, ativam partes opcionais de um crate. O `serde` precisa de `derive` para que `#[derive(Serialize, Deserialize)]` exista; o `tokio` precisa de capacidades de runtime, rede e macros para o programa atual. Não habilite `full` por reflexo num projeto novo se só precisa de uma parte pequena; aqui é uma decisão consciente do curso para que o revisor use as capacidades que ensina.

O `cargo tree` mostra a árvore completa. É a forma de descobrir dependências transitivas: crates que você não adicionou diretamente, mas que chegaram porque outra dependência precisa deles. Não é necessariamente um sinal de problema. É uma ferramenta para responder “quem traz esta versão?”, “por que tanto código é compilado?” ou “por que há duas versões deste crate?”.

O `cargo update` atualiza dentro das restrições que você escreveu no `Cargo.toml`. Não equivale a “instalar a última versão de tudo” sem limites. Antes de atualizar um projeto estável, revise o que mudou no lockfile, rode os testes e leia as notas de versão quando uma dependência central mudar. A versão declarada define o intervalo aceitável; o lockfile registra a decisão tomada.

Durante o desenvolvimento, o `cargo check` costuma ser mais rápido que o `cargo build` porque verifica tipos e empréstimos sem gerar o executável final. Não substitui os testes, mas reduz o tempo de retorno enquanto você edita uma função. Para manter a qualidade do projeto completo, use esta sequência:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo fmt --check` confirma a formatação sem modificar arquivos. `cargo clippy --all-targets -- -D warnings` revisa biblioteca, binário e testes, e converte seus avisos em falhas para que o projeto não acumule dívida conhecida. `cargo test` verifica o comportamento. São três sinais distintos: formatação consistente, uso idiomático e comportamento esperado.

## O erro que você vai ver

### `E0603`: o nome existe, mas não faz parte da API

`E0603` aparece quando o Rust encontrou o item que você nomeou, mas o caminho tenta cruzar uma fronteira privada. O diagnóstico da figura 6.2 dá três pistas: aponta o uso ilegal, diz que a função é privada e indica onde ela foi declarada. Isso é diferente de um erro de digitação como “esta função não foi encontrada”; aqui o Rust sabe sim qual função você queria usar.

A correção depende do design. Se a função deve fazer parte da interface, declare-a `pub`. Se só deve servir a um módulo irmão, considere mover a operação para um módulo dono mais apropriado ou expor uma função pública de nível superior. Se precisa limitá-la ao crate, `pub(crate)` expressa melhor que ninguém fora do pacote deve depender dela.

No `revisor`, os auxiliares de `reporte.rs`, como `etiqueta`, `tiempo` e `detalle`, continuam privados. A API pública é `tabla` e `json`, porque são as operações que o binário e qualquer consumidor podem pedir. A escolha impede que testes de integração ou código futuro dependam de uma etiqueta interna e congelem uma decisão de formato acidental.

### Um teste vermelho não é um erro de compilação

Quando uma asserção falha, o Cargo compila corretamente e depois o executor marca o teste como falho. Não haverá um código `EXXXX`, porque não é uma violação das regras estáticas do compilador. Você verá o nome do teste, o valor esquerdo e direito se usou `assert_eq!`, e qualquer mensagem extra que tenha acrescentado.

Esta é uma diferença importante de diagnóstico. Os erros do `rustc` dizem que o programa não pode ser construído sob as regras da linguagem. Um teste vermelho diz que o programa foi construído, mas quebrou o comportamento que você declarou. Não conserte um teste vermelho removendo a asserção nem mudando o valor esperado sem revisar que contrato devia ser sustentado.

Provoque uma falha de propósito uma vez. No teste `falla_sin_codigo`, mude temporariamente o texto esperado para `"OK: x"` e rode o filtro correspondente. Você deve ver o teste ficar vermelho. Depois restaure o comportamento correto. Um teste que você nunca viu falhar pode estar cobrindo um ramo diferente do que você pensa, ou pode afirmar algo fraco demais para detectar uma regressão.

## O que se faz errado

### Tornar tudo `pub`

Abrir cada struct, campo e função costuma começar como uma forma rápida de vencer o `E0603`. O resultado é uma biblioteca sem fronteiras: qualquer módulo pode se apoiar em detalhes internos e cada mudança exige revisar muito mais chamadas do que o necessário. Publique operações que representem capacidades do domínio, não cada passo auxiliar com que você as implementa.

A alternativa prática não é adivinhar a API perfeita desde o primeiro dia. Mantenha privado o que ainda não tem um consumidor claro. Quando outro módulo precisar de uma operação, abra a interface mínima e deixe que esse uso real guie o design.

### Organizar por nomes vagos como `utils` ou `helpers`

Uma pasta chamada `utils` não descreve uma responsabilidade; descreve que alguém não soube onde colocar algo. Com o tempo acumula conversão de texto, acesso a arquivos, formatação, HTTP e funções que ninguém se atreve a mover. Procurar código fica mais lento e a dependência entre módulos fica arbitrária.

No `revisor`, uma regra de YAML vive em `config`, uma tradução de estado vive em `reporte` e o vocabulário do domínio vive em `modelo`. Se uma função não cabe em nenhum módulo, primeiro pergunte se falta um conceito com nome próprio. Muitas vezes o novo nome revela uma responsabilidade que estava misturada.

### Testar apenas o caminho saudável

Um teste que verifica `200 OK` é necessário, mas não basta para um verificador de serviços. Também devem estar cobertos um HTTP 500, uma URL inválida, um arquivo ausente, um tempo de espera, uma lista vazia e um formato desconhecido. Os erros não são exceções improváveis neste domínio: fazem parte do que o programa existe para reportar.

Não transforme cada falha externa num teste de rede real. O `revisor` usa um servidor local de mentira para reproduzir respostas conhecidas. Assim testa o comportamento próprio, não a disponibilidade de um serviço alheio.

### Usar `unwrap()` para escrever testes mais curtos

`unwrap()` é razoável para preparar dados que o próprio teste controla, como um YAML literal que deve ser válido. Se esse YAML falhar, o teste está mal construído e parar é correto. Não o use sobre o resultado que você está tentando testar. Se quer demonstrar que `validar` rejeita uma entrada, use `is_err`, `unwrap_err` ou `matches!` conforme o contrato.

A regra é distinguir preparação de verificação. Na preparação, um `expect("el YAML de la prueba es válido")` dá contexto útil. Na verificação, uma asserção expressa exatamente a propriedade que você quer sustentar.

### Confiar no código de saída do `cargo test` sem ler a contagem

Um filtro sem correspondências pode devolver sucesso porque não houve testes que falhassem. Um crate novo pode compilar sem testes. Uma integração mal posicionada pode não ser descoberta. Leia `running N tests`, os nomes que aparecem e o resumo final. O resultado útil não é apenas “saiu zero”; é “rodou o teste que eu esperava e ele passou”.

### Atualizar dependências sem revisar o lockfile

O `cargo update` pode mudar várias dependências transitivas mesmo que você só tenha pedido uma atualização. Isso não o torna perigoso por si só, mas exige revisão. Veja a mudança no `Cargo.lock`, entenda quais crates foram atualizados e rode a suíte completa. Uma versão compatível em teoria pode revelar uma suposição frágil ou mudar tempos de compilação de forma importante.

## Exercícios

### Exercício 1 — Divida um relatório sem abrir demais

Crie um programa com um módulo `reporte`. Ele deve expor uma função pública `resumen(nombre, sano)` que devolva uma `String` com o nome e a etiqueta `OK` ou `FALLA`. A função que decide a etiqueta deve permanecer privada. A partir de `main`, imprima duas linhas: uma saudável e uma falha.

### Exercício 2 — Teste todas as variantes do estado

Escreva uma função `es_sano(&Estado) -> bool` para as quatro variantes do `Estado` do revisor: `Ok`, `Lento`, `Falla` e `NoIntentado`. Adicione um teste por variante. Use `assert!` ou `assert!(!...)` e nomeie cada teste conforme a regra que ele verifica.

### Exercício 3 — Uma integração que não conhece detalhes internos

Em `programas/revisor`, leia `tests/integracion.rs`. Adicione um teste de integração que use exclusivamente `revisor::modelo` e `revisor::revisar`. Ele deve usar o servidor local compartilhado e verificar que `revisar_todos` devolve o mesmo número de estados que de serviços, inclusive quando um recebe HTTP 500. Escreva-o antes de ler os testes que `tests/integracion.rs` já traz e depois compare: o que o seu verifica que os outros não verificam?

### Exercício 4 — Faça um teste vermelho e deixe-o verde de novo

Escolha um teste existente de `config.rs` ou `reporte.rs`. Mude temporariamente uma expectativa para que falhe, execute apenas esse teste com `cargo test nombre_de_la_prueba`, leia o diagnóstico e restaure o comportamento correto. Por fim, execute `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` e `cargo test`.

## Soluções

### Solução 1

<!-- verificar:fragmento -->
```rust
mod reporte {
    fn etiqueta(sano: bool) -> &'static str {
        if sano {
            "OK"
        } else {
            "FALLA"
        }
    }

    pub fn resumen(nombre: &str, sano: bool) -> String {
        format!("{nombre}: {}", etiqueta(sano))
    }
}

fn main() {
    println!("{}", reporte::resumen("catalogo", true));
    println!("{}", reporte::resumen("pagos", false));
}
```

`etiqueta` não precisa de `pub` porque só `resumen` a usa. A função pública entrega o resultado de que `main` precisa, não o detalhe intermediário.

### Solução 2

<!-- verificar:fragmento -->
```rust
#[derive(Debug)]
enum Estado {
    Ok,
    Lento,
    Falla,
    NoIntentado,
}

fn es_sano(estado: &Estado) -> bool {
    matches!(estado, Estado::Ok | Estado::Lento)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_es_sano() {
        assert!(es_sano(&Estado::Ok));
    }

    #[test]
    fn lento_es_sano() {
        assert!(es_sano(&Estado::Lento));
    }

    #[test]
    fn falla_no_es_sana() {
        assert!(!es_sano(&Estado::Falla));
    }

    #[test]
    fn no_intentado_no_es_sano() {
        assert!(!es_sano(&Estado::NoIntentado));
    }
}
```

Os quatro testes não são redundantes. A função contém dois grupos de variantes e cada uma expressa uma decisão do domínio. Se alguém alterar o `matches!` de forma incompleta, pelo menos um teste identifica qual estado perdeu seu significado.

### Solução 3

<!-- verificar:fragmento -->
```rust
#[tokio::test]
async fn revisar_todos_conserva_un_estado_por_servicio() {
    let d = comun::servidor_demo();
    let cliente = reqwest::Client::new();
    let servicios = vec![
        servicio("bien", &d, "/ok", 2000),
        servicio("mal", &d, "/error", 2000),
    ];

    let estados = revisar_todos(&cliente, &servicios, 2).await;

    assert_eq!(estados.len(), servicios.len());
    assert!(estados[0].esta_bien());
    assert!(!estados[1].esta_bien());
}
```

O teste usa apenas tipos e funções públicas do `revisor`. O helper local constrói os serviços; o servidor compartilhado controla as respostas. Não exige abrir nenhuma função privada do cliente HTTP.

### Solução 4

Execute primeiro um teste concreto, por exemplo:

```bash
cargo test validar_rechaza_timeout_cero
```

Mude temporariamente `assert!(validar(&[s]).is_err())` para `assert!(validar(&[s]).is_ok())`. O teste deve falhar. Restaure `is_err()` e termine com:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

A solução não é conservar a mudança que faz o teste passar; é verificar que a suíte detecta uma modificação que quebra a regra e depois recuperar a regra correta.

## Como sei que consegui

- `rustc --edition 2024 --test fig06_03.rs && ./fig06_03 --test-threads=1` imprime `running 3 tests` e termina com `3 passed; 0 failed`.
- `cargo test validar_rechaza_nombre_repetido` termina com o teste `config::tests::validar_rechaza_nombre_repetido ... ok`.
- `cargo test --test integracion` executa os testes que importam apenas a API pública da biblioteca.
- `cargo fmt --check` termina sem mudanças pendentes de formatação.
- `cargo clippy --all-targets -- -D warnings` termina sem avisos.
- `cargo test` termina com resultados `ok` para biblioteca, binário e testes de integração.
- Você consegue explicar por que `main.rs` importa `revisor::{config, reporte, revisar}` e não detalhes privados de `src/lib.rs`.

## Para ler mais

- [The Rust Programming Language, capítulo 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) — consultado em 2 de outubro de 2026.
- [The Rust Programming Language, capítulo 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html) — consultado em 2 de outubro de 2026.
- [The Rust Programming Language, capítulo 14: More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) — consultado em 2 de outubro de 2026.
- [Referência oficial do Cargo: especificar dependências](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) — consultado em 2 de outubro de 2026.
