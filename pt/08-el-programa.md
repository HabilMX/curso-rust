# Lição 8 — O programa terminado

**Tempo:** 2 × 45 min.

**O que você constrói:** o `revisor` completo, em um binário

**O que você aprende:** `reqwest`, `serde`, `clap`, o perfil de release, o binário final e sua comparação com o de Go

## Ao terminar, você vai poder

- Explicar que responsabilidade tem o `main.rs` e por que a lógica reutilizável vive na biblioteca `revisor`.
- Ler configuração YAML com `serde`, incluindo valores padrão e validações que o YAML não consegue expressar.
- Usar o `clap` para declarar opções de linha de comando com tipos, valores padrão e ajuda automática.
- Explicar por que uma queda HTTP é um resultado do domínio do `revisor`, enquanto um arquivo de configuração inválido impede a inicialização.
- Compilar, testar e revisar o binário com `cargo test`, `cargo clippy` e `cargo build --release`.
- Comparar, com argumentos concretos, o binário final em Rust e o programa equivalente em Go.

## O porquê antes do como

Durante as lições anteriores você construiu as peças do `revisor`: o vocabulário do problema, a leitura de YAML, o relatório, os testes e as consultas HTTP concorrentes. Uma peça isolada pode estar bem escrita e, mesmo assim, não ser uma ferramenta que outra pessoa possa usar. Falta uni-las numa fronteira clara: um executável que recebe argumentos, lê um arquivo, consulta serviços, imprime um resultado útil e termina com um código que outro programa possa interpretar.

Isso parece uma camada fina, mas é onde se encontram decisões importantes. A linha de comando é uma interface pública: se hoje você aceita `--formato json`, alguém pode integrá-la a um script e depender dela amanhã. O arquivo YAML também é uma interface: não é código Rust, então pode conter nomes repetidos, URLs incompletas ou um tempo limite de zero. A rede é outra fronteira: uma resposta HTTP 500 não significa que o próprio `revisor` esteja quebrado; significa que o serviço verificado está em mau estado. Já não conseguir ler o YAML impede, sim, de começar a trabalhar.

O programa terminado precisa distinguir esses casos sem escondê-los sob um mesmo `unwrap()`. Se tudo está saudável, termina com código 0. Se conseguiu verificar e detectou uma falha, imprime o relatório e termina com código 1. Se não conseguiu iniciar porque os argumentos ou a configuração são inválidos, informa o problema na saída de erro e termina com código 2. Essa separação torna o binário útil tanto para uma pessoa que o executa num terminal quanto para um sistema de automação.

Esta lição continua o mesmo programa que você fez em Go. A comparação importa porque ambas as linguagens chegam a um binário distribuível, mas por caminhos diferentes. O Go inclui HTTP, JSON, flags e concorrência em sua biblioteca padrão. O Rust deixa a biblioteca padrão pequena e estável; para HTTP assíncrono, serialização YAML ou uma interface de linha de comando declarativa usa crates do ecossistema. Não é uma vantagem automática de um lado nem uma carência automática do outro. É uma decisão de distribuição do trabalho entre a linguagem, o gerenciador de pacotes e as bibliotecas.

O projeto `programas/revisor/` fixa as versões concretas com `Cargo.toml` e `Cargo.lock`. O `cargo` resolve a árvore completa de dependências e o lockfile conserva a resolução exata para que outro computador construa o mesmo conjunto de crates compatíveis. Não copie versões de um tutorial antigo às cegas nem execute `cargo update` como reação automática: primeiro entenda o que mudou, revise o lockfile e rode os testes completos.

The Rust Book, capítulo 12, é a leitura central desta lição: organiza um programa de linha de comando em torno da entrada, da configuração, erros e separação de responsabilidades. Retome também os exercícios do Rustlings que você achou mais difíceis sobre `Result`, testes e iteradores. O objetivo já não é memorizar sintaxe; é ver como as decisões das lições anteriores sobrevivem quando o programa tem usuários, arquivos e rede reais.

## Os conceitos

### O binário é uma fronteira, não o lugar para toda a lógica

Um binário tem uma tarefa específica: traduzir o exterior para o interior do programa. Lê os argumentos do sistema operacional, decide o que se imprime, converte o resultado geral num código de saída e delega o resto. Se você mistura ali o carregamento de YAML, as requisições HTTP, o formato de tabela e o código de saída, os testes acabam dependendo do terminal e de `std::process::exit`. Isso faz um teste pequeno ficar lento, frágil e difícil de diagnosticar.

A alternativa do `revisor` é ter uma biblioteca com módulos públicos (`config`, `modelo`, `reporte` e `revisar`) e um `main.rs` curto. A biblioteca pode ser testada a partir de seus testes unitários e de `tests/integracion.rs`. O binário conserva apenas o que necessariamente depende da linha de comando. Essa divisão não é uma regra cerimonial: reduz o número de lugares onde uma mudança de interface pode quebrar o comportamento.

Antes de usar o `clap`, convém entender o trabalho que ele automatiza. Um parser de argumentos precisa manter estado: cada flag consome, quando for o caso, o valor que vem em seguida; precisa conservar valores padrão; e deve rejeitar uma flag desconhecida em vez de interpretá-la silenciosamente. O programa seguinte simula essa fronteira sem depender de crates.

**Fig. 8.1** | Um parser mínimo conserva valores padrão e converte texto em tipos.

```rust
// fig08_01.rs
struct Args {
    archivo: String,
    formato: String,
    paralelo: usize,
}

fn leer(args: &[&str]) -> Result<Args, String> {
    let mut resultado = Args {
        archivo: "servicios.yaml".to_string(),
        formato: "tabla".to_string(),
        paralelo: 5,
    };
    let mut i = 0;

    while i < args.len() {
        match args[i] {
            "-a" | "--archivo" => {
                i += 1;
                resultado.archivo = args.get(i).ok_or("falta archivo")?.to_string();
            }
            "-f" | "--formato" => {
                i += 1;
                resultado.formato = args.get(i).ok_or("falta formato")?.to_string();
            }
            "-p" | "--paralelo" => {
                i += 1;
                resultado.paralelo = args
                    .get(i)
                    .ok_or("falta paralelo")?
                    .parse()
                    .map_err(|_| "paralelo no es un número")?;
            }
            otro => return Err(format!("argumento desconocido: {otro}")),
        }
        i += 1;
    }

    Ok(resultado)
}

fn main() {
    let a = leer(&["--archivo", "demo.yaml", "-f", "json", "-p", "2"]).unwrap();
    println!("{} | {} | {}", a.archivo, a.formato, a.paralelo);
}
```

```bash
$ rustc --edition 2024 fig08_01.rs && ./fig08_01
demo.yaml | json | 2
```

O programa ilustra por que não convém escrever esse parser à mão para cada binário. Não gera `--help`, não documenta suas opções por si só, não valida valores permitidos e seu código cresceria rápido. O `clap` declara o contrato e gera boa parte dessa mecânica. No `revisor`, `Args` é o contrato de entrada: `archivo`, `formato` e `paralelo` têm nome longo, nome curto, tipo e valor padrão.

A figura 8.1 montou à mão o parser de argumentos. Agora, o mesmo contrato com o `clap`, o crate que o `revisor` usa. Como os demais exemplos com dependências, vive em `programas/revisor/examples/` e é executado com o Cargo a partir dessa pasta.

**Exemplo de cargo com `clap`** | O mesmo contrato de argumentos, declarado em vez de programado.

<!-- verificar:ejemplo:ejemplo_clap -->
```rust
// ejemplo_clap.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}

fn main() {
    // `try_parse_from` recibe los argumentos como una lista, en vez de leer
    // los del sistema operativo: así el ejemplo corre igual en cualquier máquina.
    let a = Args::try_parse_from(["revisor", "--formato", "json", "-p", "2"])
        .expect("estos argumentos son válidos");
    println!("{a:?}");

    // Un valor que no es número: clap lo rechaza antes de que el programa arranque.
    if let Err(e) = Args::try_parse_from(["revisor", "--paralelo", "muchos"]) {
        println!("{:?}", e.kind());
        println!("{}", e.to_string().lines().next().unwrap_or(""));
    }

    // Una opción que no existe.
    if let Err(e) = Args::try_parse_from(["revisor", "--velocidad", "3"]) {
        println!("{:?}", e.kind());
    }
}
```

```bash
$ cargo run --example ejemplo_clap
Args { archivo: "servicios.yaml", formato: "json", paralelo: 2 }
ValueValidation
error: invalid value 'muchos' for '--paralelo <PARALELO>': invalid digit found in string
UnknownArgument
```

`#[derive(Parser)]` escreve para você o código que converte a lista de argumentos num `Args`. No programa real chama-se `Args::parse()`, que lê os argumentos do sistema operacional; aqui usa-se `try_parse_from`, que recebe a lista como parâmetro, para que o exemplo dê sempre a mesma saída e para que você possa ver os erros sem encerrar o programa. A primeira linha mostra um `Args` completo: o que foi passado (`formato` e `paralelo`) e o valor padrão do que faltou (`archivo`). As seguintes mostram o que acontece com um valor que não é número e com uma opção que não existe: o `clap` as rejeita com um tipo de erro claro (`ValueValidation` e `UnknownArgument`) antes que a lógica do programa comece. Quando você usa `parse()` em vez de `try_parse_from`, o `clap` imprime essa mensagem com a ajuda de uso e termina com código 2, o mesmo código que o `revisor` reserva para «não consegui iniciar». Além disso, `--help` e `--version` saem de graça do atributo `#[command(version, about = ...)]`.

Assim se vê a declaração completa dentro do `revisor`:

<!-- verificar:extracto:src/main.rs -->
```rust
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Revisa servicios en paralelo")]
struct Args {
    #[arg(short, long, default_value = "servicios.yaml")]
    archivo: String,
    #[arg(short, long, default_value = "tabla")]
    formato: String,
    #[arg(short, long, default_value_t = 5)]
    paralelo: usize,
}
```

O atributo `#[derive(Parser)]` gera uma implementação do trait `Parser`. Por isso `Args::parse()` pode ler `std::env::args()` e construir um `Args` tipado. `default_value` recebe texto porque o `clap` o converte para o tipo do campo; `default_value_t` recebe diretamente um valor Rust, por isso é apropriado para `usize`.

O `main` do projeto não chama `std::process::exit`. Retorna `ExitCode`, que é mais fácil de testar quando a lógica se mantém em `ejecutar`. O código verifica primeiro que o formato seja um dos dois contratos prometidos. Depois carrega e valida a configuração, cria um cliente HTTP, pede os estados, decide se imprime tabela ou JSON e converte o resultado final em 0, 1 ou 2. A ordem importa: não há razão para abrir conexões de rede se o arquivo YAML nem sequer é legível.

<!-- verificar:extracto:src/main.rs -->
```rust
#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    ejecutar(&args).await
}

async fn ejecutar(args: &Args) -> ExitCode {
    if args.formato != "tabla" && args.formato != "json" {
        eprintln!(
            "revisor: --formato debe ser «tabla» o «json», no «{}»",
            args.formato
        );
        return ExitCode::from(2);
    }

    let servicios =
        match config::cargar(&args.archivo).and_then(|s| config::validar(&s).map(|()| s)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("revisor: {e:#}");
                return ExitCode::from(2);
            }
        };

    let cliente = reqwest::Client::new();
    let estados = revisar::revisar_todos(&cliente, &servicios, args.paralelo).await;

    if args.formato == "json" {
        match reporte::json(&servicios, &estados) {
            Ok(j) => println!("{j}"),
            Err(e) => {
                eprintln!("revisor: escribiendo el reporte: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        print!("{}", reporte::tabla(&servicios, &estados));
    }

    if estados.iter().all(|e| e.esta_bien()) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
```

A anotação `#[tokio::main]` cria e inicia o runtime necessário para poder usar `.await` na função principal. Não faz o programa inteiro ficar mais rápido por si só. Seu propósito é executar futures enquanto as requisições HTTP esperam dados da rede, sem bloquear uma thread para cada espera.

### O `serde` torna explícito o contrato de dados

Um arquivo YAML chega ao programa como texto. O texto não sabe o que é um nome, qual campo é obrigatório nem que valor deve ser usado quando falta `timeout_ms`. Convertê-lo para um `Vec<Servicio>` é passar de dados que vêm do exterior para valores que o compilador pode verificar. Essa conversão é um limite de confiança: depois de desserializar, você ainda precisa validar as regras de negócio que o formato não conhece.

O `serde` separa duas direções. `Deserialize` constrói valores Rust a partir de YAML, JSON, TOML ou outro formato que tenha um adaptador compatível. `Serialize` converte valores Rust num formato de saída. A estrutura do domínio se conserva; muda o formato que a envolve. Por isso o mesmo `EstadoJson` pode ser gerado com `serde_json`, enquanto `Servicio` entra com `yaml_serde`.

O programa seguinte mostra uma decisão do domínio que também aparece no JSON do `revisor`: um estado saudável tem código HTTP e não leva erro; uma falha não inventa um código e inclui um motivo. O programa monta o JSON manualmente para que a diferença fique visível. No projeto real você não deve fazer isso à mão: o `serde_json` se encarrega de escapar texto e preservar um JSON válido.

**Fig. 8.2** | A forma da saída depende da variante do estado.

```rust
// fig08_02.rs
enum Estado {
    Ok(u16),
    Falla(&'static str),
}

fn json(nombre: &str, estado: &Estado, ms: u64) -> String {
    match estado {
        Estado::Ok(codigo) => {
            format!("{{\"servicio\":\"{nombre}\",\"codigo\":{codigo},\"ms\":{ms}}}")
        }
        Estado::Falla(motivo) => {
            format!(
                "{{\"servicio\":\"{nombre}\",\"codigo\":null,\"ms\":{ms},\"error\":\"{motivo}\"}}"
            )
        }
    }
}

fn main() {
    println!("{}", json("catalogo", &Estado::Ok(200), 7));
    println!("{}", json("pagos", &Estado::Falla("codigo 500"), 12));
}
```

```bash
$ rustc --edition 2024 fig08_02.rs && ./fig08_02
{"servicio":"catalogo","codigo":200,"ms":7}
{"servicio":"pagos","codigo":null,"ms":12,"error":"codigo 500"}
```

No `revisor`, o derive declara exatamente o que deve ler ou escrever. `Servicio` deriva `Deserialize` porque é criado a partir de YAML. `EstadoJson` deriva `Serialize` porque é criado a partir dos resultados internos para produzir JSON. `#[serde(default = "timeout_por_omision")]` não equivale a o campo ser opcional em Rust: o campo final continua sendo um `u64`; apenas recebe um valor quando o YAML não o declara.

O programa seguinte usa em escala reduzida os dois crates de dados do `revisor`: `serde` para declarar o contrato, e `yaml_serde` e `serde_json` para os formatos. Os dados entram como um texto YAML escrito dentro do próprio programa, para não depender de nenhum arquivo.

**Exemplo de cargo com `serde`** | O mesmo `struct` lê YAML e escreve JSON.

<!-- verificar:ejemplo:ejemplo_serde -->
```rust
// ejemplo_serde.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Servicio {
    nombre: String,
    url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    timeout_ms: u64,
}

fn timeout_por_omision() -> u64 {
    5000
}

fn main() -> Result<(), yaml_serde::Error> {
    let yaml = "\
- nombre: catalogo
  url: http://localhost:8080
- nombre: pagos
  url: http://localhost:8081
  timeout_ms: 250
";
    // YAML adentro: el mismo derive sirve para leer...
    let servicios: Vec<Servicio> = yaml_serde::from_str(yaml)?;
    for s in &servicios {
        println!("{} espera {} ms", s.nombre, s.timeout_ms);
    }

    // ...y para escribir JSON, que es otro formato con el mismo modelo.
    match serde_json::to_string(&servicios) {
        Ok(json) => println!("{json}"),
        Err(e) => println!("no se pudo escribir el JSON: {e}"),
    }

    // Un servicio sin `url` no es un Servicio: el error dice qué falta y dónde.
    let roto: Result<Vec<Servicio>, _> = yaml_serde::from_str("- nombre: sin-url\n");
    if let Err(e) = roto {
        println!("YAML inválido: {e}");
    }
    Ok(())
}
```

```bash
$ cargo run --example ejemplo_serde
catalogo espera 5000 ms
pagos espera 250 ms
[{"nombre":"catalogo","url":"http://localhost:8080","timeout_ms":5000},{"nombre":"pagos","url":"http://localhost:8081","timeout_ms":250}]
YAML inválido: .[0]: missing field `url` at line 1 column 3
```

O mesmo `struct Servicio` serve para ler e para escrever porque deriva as duas metades do `serde`: `Deserialize` para construí-lo a partir de YAML e `Serialize` para escrevê-lo como JSON (o `Servicio` do `revisor` só deriva `Deserialize`, porque nunca é escrito de volta). Repare em dois detalhes. O serviço `catalogo` não declara `timeout_ms` no YAML e mesmo assim sai com 5000: é o efeito de `#[serde(default = ...)]`. E o YAML inválido não derruba o programa com um pânico: `from_str` devolve um `Err` cuja mensagem diz o que falta (``missing field `url` ``) e onde (`.[0]` é o primeiro elemento da lista; `line 1 column 3`, a posição no texto). Essa mensagem é a que o `revisor` mostra a quem corrige seu arquivo. E como o `yaml_serde` é a continuação mantida do `serde_yaml` (o crate original já não recebe mudanças, como você viu na lição 6), tudo o que aqui se faz com `from_str` funciona igual com qualquer um dos dois nomes.

<!-- verificar:extracto:src/modelo.rs -->
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Servicio {
    pub nombre: String,
    pub url: String,
    #[serde(default = "timeout_por_omision")] // si falta en el YAML
    pub timeout_ms: u64,
}

#[derive(Serialize)]
pub struct EstadoJson {
    pub servicio: String,
    pub codigo: Option<u16>,
    pub ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

`Option<u16>` representa uma diferença importante: se não houve resposta HTTP, não existe código HTTP. Usar `0` ou uma string vazia inventaria um dado e obrigaria todo consumidor a lembrar essa convenção. Em JSON, `Option::None` se converte em `null` para `codigo`. Para `error`, o atributo `skip_serializing_if` escolhe outra política: quando não há erro, a chave nem sequer aparece. As duas decisões são válidas, mas devem ser intencionais e estar cobertas por testes.

Desserializar bem não basta. O YAML pode expressar uma lista vazia, repetir um nome, aceitar uma string como URL ou incluir um tempo limite de zero. O parser não sabe que essas opções tornam o `revisor` inútil. `config::validar` é o segundo passo e expressa as regras do programa, não as do formato.

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

`anyhow::ensure!` devolve cedo um erro quando uma condição não se cumpre. Aqui é adequado porque uma configuração inválida não representa um serviço com falha: é uma condição que impede iniciar a verificação. `with_context` em `cargar` acrescenta o nome do arquivo a um erro de leitura; o formato `{e:#}` em `main` imprime essa cadeia de contexto junto com a causa original. O resultado é mais útil que uma mensagem genérica de “não foi possível abrir”.

### O `reqwest` converte a rede em estados do domínio

Um programa de monitoramento não consulta HTTP para obter um corpo e esquecê-lo; consulta para classificar o estado de um serviço. Por isso a função `revisar` não devolve `Result<Estado>`. Se um servidor responde 500, recusa a conexão ou excede seu tempo limite, a operação do `revisor` sim terminou: descobriu uma falha e deve incluí-la no relatório. Esses casos são convertidos em `Estado::Falla`.

Não confunda esse resultado com uma falha de configuração ou com um erro de serialização ao produzir o relatório. Esses sim impedem que o programa cumpra seu trabalho e fazem com que termine com código 2. A distinção evita dois erros frequentes: interromper toda a verificação porque um serviço caiu, ou continuar como se nada tivesse acontecido quando não foi possível ler o arquivo que define quais serviços existem.

Para ver o `reqwest` sem depender de nenhum serviço real, o programa seguinte sobe na mesma máquina um servidor de mentira e o consulta com um cliente do `reqwest`. O servidor é montado com `TcpListener`, da biblioteca padrão, e responde à mão o texto mínimo de HTTP: responde `200` em `/sano`, `500` em `/roto` e leva meio segundo para responder `/lento`. Além disso, o programa tenta se conectar a uma porta onde ninguém escuta.

**Exemplo de cargo com `reqwest`** | Quatro respostas diferentes da rede, vistas do lado do cliente.

<!-- verificar:ejemplo:ejemplo_reqwest -->
```rust
// ejemplo_reqwest.rs
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

/// Un servidor HTTP de mentira, en esta misma máquina: /sano responde 200,
/// /roto responde 500 y /lento tarda medio segundo en contestar.
fn servidor_de_mentira() -> String {
    let escucha = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = escucha.local_addr().expect("el servidor tiene dirección");
    thread::spawn(move || {
        for conexion in escucha.incoming().flatten() {
            thread::spawn(move || atender(conexion));
        }
    });
    format!("http://{direccion}")
}

fn atender(mut conexion: std::net::TcpStream) {
    let mut pedido = [0u8; 1024];
    let n = conexion.read(&mut pedido).unwrap_or(0);
    let texto = String::from_utf8_lossy(&pedido[..n]);
    let ruta = texto.split_whitespace().nth(1).unwrap_or("/");
    let estado = match ruta {
        "/sano" => "200 OK",
        "/roto" => "500 Internal Server Error",
        "/lento" => {
            thread::sleep(Duration::from_millis(500));
            "200 OK"
        }
        _ => "404 Not Found",
    };
    let _ = write!(
        conexion,
        "HTTP/1.1 {estado}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
}

#[tokio::main]
async fn main() {
    let base = servidor_de_mentira();
    let cliente = reqwest::Client::new();

    for ruta in ["sano", "roto", "lento"] {
        let respuesta = cliente
            .get(format!("{base}/{ruta}"))
            .timeout(Duration::from_millis(200))
            .send()
            .await;
        match respuesta {
            Ok(r) => println!("{ruta:<6} responde {}", r.status().as_u16()),
            Err(e) if e.is_timeout() => println!("{ruta:<6} se acabó el tiempo de espera"),
            Err(_) => println!("{ruta:<6} no responde"),
        }
    }

    // Un puerto donde nadie escucha: la conexión misma falla.
    let vacio = TcpListener::bind("127.0.0.1:0").expect("hay un puerto libre");
    let direccion = vacio.local_addr().expect("tiene dirección");
    drop(vacio);
    match cliente.get(format!("http://{direccion}/")).send().await {
        Ok(r) => println!("vacío  responde {}", r.status().as_u16()),
        Err(e) if e.is_connect() => println!("vacío  no responde (rechazó la conexión)"),
        Err(e) => println!("vacío  falló de otra forma: {e}"),
    }
}
```

```bash
$ cargo run --example ejemplo_reqwest
sano   responde 200
roto   responde 500
lento  se acabó el tiempo de espera
vacío  no responde (rechazó la conexión)
```

Cada linha da saída é um dos casos que o `revisor` converte em um estado do domínio. Um `500` não é um erro do `reqwest`: a requisição foi feita e o servidor respondeu, então `send().await` devolve `Ok` com o código `500`, e é o programa quem decide o que isso significa. Em contrapartida, um tempo limite excedido (`.timeout(...)` de 200 ms contra um servidor que leva 500) e uma conexão recusada sim chegam como `Err`, e o próprio erro diz de qual dos dois casos se trata com `is_timeout()` e `is_connect()`. O cliente é criado uma única vez com `reqwest::Client::new()` e reutilizado em cada consulta; o tempo limite, em contrapartida, é definido por requisição. A função `revisar` do projeto real é essa mesma ideia, com os estados `Ok`, `Lento` e `Falla` em vez de linhas de texto:

<!-- verificar:extracto:src/revisar.rs -->
```rust
pub async fn revisar(cliente: &reqwest::Client, s: &Servicio) -> Estado {
    let inicio = Instant::now();
    let respuesta = cliente
        .get(&s.url)
        .timeout(Duration::from_millis(s.timeout_ms)) // .await cede el control mientras espera
        .send()
        .await;
    let ms = inicio.elapsed().as_millis() as u64;

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
}
```

A rede também obriga a separar concorrência de ordem. O `revisor` pode iniciar várias consultas ao mesmo tempo, mas a saída deve ser reproduzível e deve relacionar cada estado com seu serviço correto. `join_all` conserva a ordem dos futures de entrada, mesmo que as respostas cheguem em outra ordem. O semáforo limita quantas consultas entram na seção ativa; não determina a ordem final do `Vec<Estado>`.

A figura seguinte não usa a rede nem nenhum crate, para que qualquer pessoa possa compilá-la com o `rustc` puro e obter sempre a mesma saída. Em vez de HTTP, representa a política do semáforo: com limite 2, os turnos são entregues de dois em dois. No projeto real cada turno vive até que a requisição termine e o runtime acorda a tarefa quando a rede responde.

**Fig. 8.3** | Um limite de concorrência divide os pendentes em lotes sem mudar sua ordem.

```rust
// fig08_03.rs
fn lote<T>(pendientes: &mut Vec<T>, limite: usize) -> Vec<T> {
    let n = limite.min(pendientes.len());
    pendientes.drain(..n).collect()
}

fn main() {
    let mut servicios = vec!["catalogo", "pagos", "usuarios", "correo", "facturas"];

    while !servicios.is_empty() {
        println!("arrancan: {}", lote(&mut servicios, 2).join(", "));
    }
}
```

```bash
$ rustc --edition 2024 fig08_03.rs && ./fig08_03
arrancan: catalogo, pagos
arrancan: usuarios, correo
arrancan: facturas
```

A função real cria um `Arc<Semaphore>` porque cada future precisa compartilhar o mesmo contador de turnos. `Arc` permite propriedade compartilhada entre tarefas; `Semaphore` entrega uma permissão temporária; e a variável `_turno` conserva essa permissão até terminar `revisar`. O sublinhado inicial evita um aviso do compilador, mas o valor não é descartado: seu destrutor devolve a permissão quando a variável sai do bloco.

<!-- verificar:extracto:src/revisar.rs -->
```rust
pub async fn revisar_todos(
    cliente: &reqwest::Client,
    servicios: &[Servicio],
    paralelo: usize,
) -> Vec<Estado> {
    let paralelo = if paralelo == 0 {
        PARALELO_POR_OMISION
    } else {
        paralelo
    };
    let turnos = Arc::new(Semaphore::new(paralelo));

    let futuros = servicios.iter().map(|s| {
        let turnos = Arc::clone(&turnos);
        async move {
            // pide turno; espera si ya hay `paralelo` corriendo, y lo devuelve al soltar `_turno`
            let _turno = turnos.acquire().await.expect("el semáforo nunca se cierra");
            revisar(cliente, s).await
        }
    });
    join_all(futuros).await
}
```

O parâmetro `paralelo` tem uma sutileza: o programa aceita `0` como “use o valor padrão”. É uma decisão de interface; não significa que o Tokio possa executar zero requisições. A constante `PARALELO_POR_OMISION` conserva essa regra perto da lógica que a utiliza.

A consulta individual cria um limite por serviço com `.timeout(Duration::from_millis(s.timeout_ms))`. Esse limite não substitui o semáforo. O timeout decide quanto uma requisição pode esperar; o semáforo decide quantas requisições podem estar esperando ao mesmo tempo. Sem timeout, um turno pode ficar ocupado por tempo demais. Sem semáforo, uma lista enorme pode abrir conexões demais ao mesmo tempo. Os dois limites protegem recursos diferentes.

`reqwest::Client::new()` é criado uma vez e emprestado a todas as consultas. Não construa um cliente por serviço: um cliente pode conservar e reutilizar conexões, enquanto um novo por chamada perde essa vantagem e acrescenta trabalho desnecessário. O empréstimo compartilhado `&reqwest::Client` é suficiente porque as requisições do cliente não exigem uma referência mutável exclusiva.

### O relatório é uma interface estável para pessoas e programas

Uma tabela serve para ler num terminal. O JSON serve para que outro programa consuma resultados sem precisar adivinhar colunas, espaços ou alinhamento. Não convém usar a tabela como formato de integração: se amanhã você mudar sua largura, um script que a processe com `awk` pode quebrar mesmo que a informação seja a mesma. Tampouco convém imprimir diagnósticos humanos na saída padrão quando se pediu JSON, porque deixaria de ser JSON válido.

O `revisor` usa `stdout` para o relatório e `stderr` para os erros de inicialização. Essa convenção permite redirecionar apenas o relatório para um arquivo:

```bash
cargo run -- --archivo servicios.yaml --formato json > estado.json
```

Se a configuração é válida, `estado.json` contém somente JSON. Se algo impede a inicialização, a mensagem aparece no terminal por meio de `stderr`; não se mistura com um formato que outro processo espera analisar.

A função `json` parte de serviços e estados, ordena-os por nome e constrói uma coleção de `EstadoJson`. Ordenar antes de serializar não é obrigatório para que o JSON seja válido, mas ajuda a tornar o resultado estável. Um resultado estável se compara melhor em testes, revisões de mudanças e automações.

<!-- verificar:extracto:src/reporte.rs -->
```rust
pub fn json(servicios: &[Servicio], estados: &[Estado]) -> serde_json::Result<String> {
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
    serde_json::to_string_pretty(&lineas)
}
```

Os `match` forçam a decidir o que cada variante representa. `NoIntentado` não tem duração nem código real, então é expresso como `ms: 0`, `codigo: null` e um erro explícito. `Falla` conserva seu tempo decorrido porque saber que uma conexão esgotou seu limite em 5 000 ms é informação útil. `Ok` e `Lento` têm código; ambos são saudáveis para o código de saída, embora o relatório os diferencie.

Esta parte revela uma diferença útil com o Go. Em Go você pode construir structs com etiquetas `json:"..."` usando apenas `encoding/json`; para YAML normalmente acrescenta outra biblioteca com sua própria convenção de etiquetas. Em Rust, o `serde` centraliza a definição de serialização e permite que os adaptadores de formato trabalhem sobre o mesmo derive. Em troca, você precisa incorporar crates externos e entender quais partes do contrato estão nos atributos e quais na validação explícita.

### O perfil de release prepara um artefato, não substitui a verificação

`cargo run` e `cargo test` usam por padrão o perfil de desenvolvimento. Esse perfil privilegia que compilar durante a edição seja rápido e conserva informação útil para depurar. Um binário distribuível se constrói com `cargo build --release`; então o Cargo aplica o perfil `release` definido pelo projeto.

O perfil não transforma um programa incorreto em um correto. Primeiro devem passar os testes, a formatação e o Clippy. Depois o perfil decide o equilíbrio entre tamanho do binário, tempo de compilação, otimização e diagnóstico disponível se ocorrer um pânico. Neste projeto se escolheu tamanho: os símbolos são removidos, otimiza-se para tamanho, ativa-se a otimização entre módulos, usa-se uma única unidade de geração e os pânicos abortam o processo.

<!-- verificar:extracto:Cargo.toml -->
```toml
[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila
```

Cada opção tem custo. `lto = true` e `codegen-units = 1` podem aumentar o tempo de compilação porque permitem otimizar com mais informação global. `panic = "abort"` reduz o binário e evita o desenrolamento da pilha, mas sacrifica a possibilidade de limpar por meio de desenrolamento e oferece menos contexto se um pânico chegar à produção. Para esta ferramenta de linha de comando é uma escolha razoável; não é uma receita universal para toda biblioteca.

Não declare que o Rust ou o Go “ganham” pelo tamanho de um binário sem medir na sua máquina e com a mesma funcionalidade. O binário do Rust pode crescer ao incluir Tokio, Reqwest, TLS e suas dependências transitivas; o do Go também inclui seu runtime e depende de suas opções de compilação. A comparação válida anota plataforma, arquitetura, compilação a frio ou incremental, perfil usado e quais dependências cada programa inclui.

A compilação cruzada deixa outra diferença prática. O Go costuma permitir selecionar a plataforma com variáveis como `GOOS` e `GOARCH`. O Rust exige instalar o target correspondente e, conforme o target, dispor também de um ligador (linker) e bibliotecas compatíveis. Para um Linux estático baseado em musl, o primeiro passo seria:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Isso não significa que a compilação cruzada seja impossível em Rust; significa que você deve modelar o target como parte do ambiente de construção. Verifique o binário resultante no sistema onde será usado, especialmente se incluir TLS ou se você alternar entre libcs de Linux.

## O erro que você vai ver

O parser de argumentos deve converter texto em valores tipados. Se você tenta atribuir diretamente um texto a um `usize`, o Rust não adivinha que você quer interpretar o texto como número. O programa seguinte falha antes de executar.

**Fig. 8.4** | Um texto não é um número, mesmo que represente uma opção de linha de comando.

```rust
// fig08_04.rs
fn main() {
    let paralelo: usize = "cinco";
    println!("{paralelo}");
}
```

```bash
$ rustc --edition 2024 fig08_04.rs
error[E0308]: mismatched types
 --> fig08_04.rs:3:27
  |
3 |     let paralelo: usize = "cinco";
  |                   -----   ^^^^^^^ expected `usize`, found `&str`
  |                   |
  |                   expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`E0308` significa que dois tipos não coincidem. A anotação `: usize` estabelece o que a expressão da direita deve produzir, mas `"cinco"` é um `&str`. O conserto não é remover o tipo para que compile com uma string: `paralelo` deve continuar sendo um número porque controla um limite de concorrência. Você deve converter com `.parse::<usize>()` e decidir o que fazer se a conversão falhar.

No programa mínimo essa decisão aparece como `Result<Args, String>`. No projeto real, o `clap` faz a conversão dos campos declarados e mostra um diagnóstico de uso quando não consegue construir o tipo esperado. Depois `main` ainda valida as regras próprias da aplicação: que `formato` seja `tabla` ou `json`, que o YAML tenha serviços e que os valores de cada serviço façam sentido.

Outro erro frequente é confundir um resposta HTTP sem sucesso com um erro que deve ser propagado por meio de `?`. Neste programa, um HTTP 500 se converte em `Estado::Falla`, é impresso e dá código de saída 1. Propagá-lo como erro de inicialização faria com que um serviço caído escondesse o resultado dos demais. O operador `?` continua correto para falhas que impedem continuar, como não conseguir ler o YAML ou não conseguir serializar o relatório JSON.

## O que se faz errado

### Colocar `unwrap()` nas fronteiras do programa

`unwrap()` é aceitável num teste quando você prepara dados literais que controla. Não é para argumentos, arquivos YAML nem rede. Um argumento mal escrito não deve produzir um pânico com um backtrace; deve dar uma mensagem que diga que opção é inválida e terminar com código 2. Um arquivo ausente não é uma surpresa de programação: é uma condição de uso que o binário deve informar com o nome do arquivo.

A regra útil é perguntar quem controla o dado. Se o teste o controla, `expect` pode tornar uma falha clara. Se o controla uma pessoa, um arquivo, o sistema operacional ou a rede, converta-o num erro contextual ou num estado do domínio.

### Criar um `reqwest::Client` para cada serviço

Criar o cliente dentro de `revisar` parece local e simples, mas perde a reutilização de conexões e faz a configuração do cliente se dispersar. O projeto cria um único `reqwest::Client` depois que a configuração foi validada e o empresta a todas as consultas. Assim o ciclo de vida do cliente coincide com o ciclo de vida de uma execução do `revisor`.

Não confunda compartilhar o cliente com compartilhar estado mutável sem controle. O cliente é usado por meio de referências compartilhadas, e o semáforo protege o recurso que de fato deve ser limitado: quantas requisições estão ativas.

### Usar JSON construído com `format!` em produção

A figura de JSON mostra a forma dos dados, não uma técnica de serialização segura. Se o nome de um serviço contém aspas, uma barra invertida ou uma quebra de linha, uma string construída à mão pode deixar de ser JSON válido ou mudar de significado. O `serde_json` escapa esses dados corretamente e conserva a relação entre `Option`, `null` e campos omitidos.

Uma boa regra é que os formatos estruturados sejam construídos com um serializador e testados lendo-os de volta. Nos testes do projeto, o JSON é convertido em `serde_json::Value` antes de verificar campos específicos.

### Tratar o código de saída como um detalhe invisível

Uma tabela com a palavra `FALLA` ajuda uma pessoa, mas um trabalho agendado precisa saber se deve alertar. Se o `revisor` sempre termina com 0, um cron, um pipeline ou um supervisor pode acreditar que tudo está saudável mesmo havendo serviços caídos. Se termina com 1 diante de qualquer problema, também não distingue entre uma queda monitorada e uma configuração errada.

Mantenha o contrato simples e documentado: 0 para uma verificação saudável, 1 para serviços com falha e 2 para erros de inicialização ou de uso. Os testes do binário devem verificar esses três caminhos.

### Medir o binário do Rust contra o do Go sem controlar o experimento

Comparar apenas o tamanho de dois arquivos é uma forma pobre de tirar conclusões. O resultado muda se um inclui TLS, se o outro usa uma biblioteca HTTP externa, se um foi compilado em desenvolvimento e outro em release, se os símbolos foram removidos ou se o sistema comprime executáveis. O tempo também muda entre uma compilação a frio, uma incremental e uma reconstrução depois de tocar uma linha.

Meça com o mesmo caso de uso e registre as condições. Às vezes o Go será a escolha prática porque a compilação cruzada e a biblioteca padrão reduzem passos. Às vezes o Rust será preferível porque você quer expressar certas garantias antes de executar e aceitar uma compilação mais custosa. O propósito de fazer os dois cursos não é repetir um slogan; é ter evidência própria.

## Exercícios

### Exercício 1 — Declare uma opção adicional

Acrescente a um programa independente uma opção `--silencioso` que por padrão seja falsa. Quando for verdadeira, o programa deve imprimir apenas o código numérico de cada estado; quando for falsa, deve imprimir nome, estado e código. Faça com que uma flag desconhecida produza um `Err` com uma mensagem clara.

Depois explique o que você declararia no `clap`: um campo `bool`, uma opção com `#[arg(long)]` e o comportamento padrão que seu tipo já expressa.

### Exercício 2 — Separe dados inválidos de serviços com falha

Escreva uma função que receba uma lista de serviços mínimos com `nombre`, `url` e `timeout_ms`. Ela deve devolver um erro se a lista estiver vazia, se um nome se repetir, se a URL não começar com `http://` ou `https://`, ou se o tempo limite for zero. Separadamente, represente um HTTP 503 como uma variante de estado, não como erro de validação.

Use as regras de `config::validar` como referência, mas escreva primeiro seus casos de teste: uma lista válida, uma vazia, uma com nome duplicado e uma com timeout zero.

### Exercício 3 — Teste o contrato do binário

Sem abrir ainda `tests/binario.rs`, escreva um teste de ponta a ponta para o formato inválido `-f xml` (numa cópia do projeto, ou com outro nome se trabalhar no original). Ele deve verificar que o código de saída seja 2, que não haja tabela na saída padrão e que a saída de erro mencione `--formato`.

Depois rode apenas esse teste e em seguida toda a suíte. Não mude o código do projeto para que passe um teste mal escrito: o teste deve descrever o contrato que `main.rs` já expõe. Ao terminar, abra `tests/binario.rs`: o projeto já traz um teste para esse contrato e ele é a solução de referência; compare o que cada um verifica.

### Exercício 4 — Compare as duas implementações

Construa em release o `revisor` em Rust e o programa equivalente em Go. Num diário de bordo anote, para ambos, linhas de código próprias, dependências diretas, tempo de compilação a frio, tamanho do binário, uso de memória numa mesma configuração e o tempo que você levou para completar o programa.

Responda com uma frase fundamentada: qual você entregaria para uma ferramenta interna com prazo apertado, qual para uma ferramenta que deve ser mantida por anos e qual decisão concreta de cada linguagem levou você a essa conclusão.

## Soluções

### Solução 1

<!-- verificar:fragmento -->
```rust
fn imprimir(nombre: &str, codigo: u16, silencioso: bool) {
    if silencioso {
        println!("{codigo}");
    } else {
        println!("{nombre} OK {codigo}");
    }
}
```

A opção não muda o estado do domínio; muda a apresentação. Por isso deve ser aplicada perto da fronteira de saída, não dentro de `Estado` nem da lógica HTTP. Com o `clap`, um `bool` marcado com `#[arg(long)]` expressa que a ausência da flag vale `false` e sua presença vale `true`.

### Solução 2

<!-- verificar:fragmento -->
```rust
fn validar_timeout(timeout_ms: u64) -> Result<(), String> {
    if timeout_ms == 0 {
        Err("el tiempo límite debe ser mayor que cero".to_string())
    } else {
        Ok(())
    }
}

enum Estado {
    Falla { codigo: u16 },
}
```

O timeout inválido é um problema de entrada: não se deve iniciar a verificação. Um código 503, em contrapartida, é informação obtida ao verificar e pertence ao estado do serviço. A separação permite que o binário termine com 2 no primeiro caso e com 1 no segundo.

### Solução 3

<!-- verificar:fragmento -->
```rust
#[test]
fn un_formato_desconocido_sale_con_dos() {
    let r = revisor(&["-f", "xml"]);
    assert_eq!(r.status.code(), Some(2), "stderr: {}", texto(&r.stderr));
    assert!(texto(&r.stdout).is_empty(), "stdout: {}", texto(&r.stdout));
    assert!(
        texto(&r.stderr).contains("--formato"),
        "stderr: {}",
        texto(&r.stderr)
    );
}
```

O teste executa o binário real, por isso verifica suas três saídas observáveis: código, saída padrão e saída de erro. Não precisa conhecer funções privadas de `main.rs`; só conhece o contrato que verá quem executar `revisor -f xml`.

### Solução 4

A comparação deve começar com comandos reproduzíveis:

```bash
cd programas/revisor
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

Depois repita o experimento com o programa em Go, no mesmo computador e com a mesma configuração de serviços. Não copie resultados alheios como se fossem universais. O valor do exercício é identificar que parte do tempo foi compilação, que parte foi aprender o ecossistema e que parte foi resolver o mesmo problema de design.

## Como sei que consegui

- `cargo run -- --help` mostra as opções `--archivo`, `--formato` e `--paralelo`.
- `cargo run -- --archivo archivo-que-no-existe.yaml` termina com código 2 e escreve o nome do arquivo na saída de erro.
- `cargo run -- --formato xml` termina com código 2 e explica que os formatos válidos são `tabla` e `json`.
- `cargo run --example ejemplo_clap`, `ejemplo_serde` e `ejemplo_reqwest` imprimem o mesmo que esta lição documenta, e `herramientas/verificar-ejemplos.sh` termina sem erros.
- `cargo test` termina com resultados corretos para biblioteca, integração e binário.
- `cargo clippy --all-targets -- -D warnings` termina sem avisos.
- `cargo fmt --check` termina sem mudanças pendentes.
- `cargo build --release` cria o binário em `target/release/revisor`.
- Você consegue explicar por que um HTTP 500 produz código de saída 1, enquanto um YAML inválido produz código 2.

## Para ler mais

- [The Rust Programming Language, capítulo 12: An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) — consultado em 2 de outubro de 2026.
- [Referência oficial do Cargo: perfis](https://doc.rust-lang.org/cargo/reference/profiles.html) — consultado em 2 de outubro de 2026.
- [Documentação do `clap`](https://docs.rs/clap/latest/clap/) — consultado em 2 de outubro de 2026.
- [Documentação do `reqwest`](https://docs.rs/reqwest/latest/reqwest/) — consultado em 2 de outubro de 2026.
