# Lição 7 — Concorrência e async

**Tempo:** 2 × 45 min.

**O que você constrói:** o `revisor` concorrente: que verifique tudo ao mesmo tempo

**O que você aprende:** threads do sistema, `Arc` e `Mutex`, canais, `async/await` com `tokio`, a comparação honesta com as goroutines de Go

## Ao terminar, você vai poder

- Lançar threads do sistema com `thread::spawn`, transferir a elas a propriedade correta e recolher seu resultado com `join`.
- Explicar por que um dado compartilhado entre threads precisa de `Arc`, quando precisa também de um `Mutex`, e o que o `MutexGuard` protege.
- Usar um canal da biblioteca padrão para entregar resultados sem compartilhar uma coleção mutável.
- Ler o erro `E0277` quando um valor não cumpre `Send` e reconhecer que o compilador está protegendo uma fronteira entre threads.
- Explicar a diferença entre concorrência e paralelismo, e entre uma thread do sistema, uma tarefa async e uma goroutine de Go.
- Acompanhar no `revisor` o percurso de uma consulta async, desde `#[tokio::main]` até `join_all`, o semáforo e o estado que fica ligado a cada serviço.

## O porquê antes do como

Até agora, o `revisor` consegue receber uma lista de serviços, consultar um e classificar a resposta. Se ele consulta dez serviços em série e cada um demora um segundo para responder, o relatório leva aproximadamente dez segundos. Não importa que o computador tenha vários núcleos nem que o programa seja rápido: durante quase todo esse tempo a CPU não está calculando nada; está esperando uma resposta de rede.

Esperar uma resposta de rede é diferente de calcular uma soma grande. Num cálculo intenso, mais núcleos podem permitir trabalho paralelo de verdade. Numa consulta HTTP, em contrapartida, a maior parte do tempo está fora do processo: o sistema operacional espera pacotes, o servidor remoto decide o que fazer e a rede transporta a resposta. Enquanto isso, o programa poderia iniciar outras consultas. Isso é concorrência: organizar várias tarefas que avançam em períodos intercalados. Pode virar paralelismo se várias tarefas executam código ao mesmo tempo em núcleos distintos, mas os dois conceitos não são sinônimos.

O objetivo prático desta lição é mudar o tempo total. Se cinco serviços levam cerca de um segundo cada um e você os consulta um por um, o relatório leva por volta de cinco segundos. Se você inicia as cinco consultas e espera as respostas ao mesmo tempo, o tempo se aproxima do do serviço mais lento, não da soma de todos. Isso não faz os serviços remotos responderem mais rápido; evita desperdiçar o tempo que o programa passava esperando o primeiro antes de começar o segundo.

O custo é que várias partes do programa podem estar vivas ao mesmo tempo. Surgem perguntas que o código sequencial não tinha: quem é o dono do dado? Quando termina uma tarefa? Que ordem têm os resultados? Duas tarefas podem modificar o mesmo valor? O que acontece se uma falha? Como você impede de abrir milhares de conexões simultâneas? O Rust não responde a essas perguntas escondendo a memória compartilhada. Ele faz com que o ownership, os empréstimos e traits como `Send` e `Sync` continuem importando quando há várias threads ou tarefas.

Isso se conecta diretamente com a lição 2. O ownership parecia uma regra local: um valor tem um dono e os empréstimos devem respeitar seu tempo de vida. Em concorrência, essas regras viram uma garantia entre tarefas. Uma thread não pode conservar uma referência a uma variável de `main` que talvez já tenha desaparecido. Tampouco pode receber um tipo que o Rust sabe que não é seguro mover para outra thread. O que no início parecia atrito do compilador se torna aqui uma barreira contra referências pendentes e corridas de dados (data races) em código seguro.

O Rust oferece duas ferramentas principais para o problema. A biblioteca padrão traz threads do sistema, mutexes, contadores de referências atômicos e canais. São apropriados para trabalho de CPU, programas pequenos ou integração com APIs bloqueantes. Para muitas operações de rede que passam tempo esperando, o `revisor` usa `async/await` e o Tokio. Async não significa “mais rápido por definição”: significa que uma quantidade moderada de threads pode fazer avançar muitas operações que esperam E/S sem reservar uma thread bloqueada para cada uma.

O Go toma outra decisão. Uma goroutine se inicia com `go f()` e o runtime vem integrado à linguagem e à distribuição. O Rust obriga a distinguir uma thread de uma tarefa async e, para async, a escolher um runtime. Isso exige mais vocabulário e mais decisões, mas permite que o tipo de dado e a fronteira de propriedade sejam explícitos. Nenhuma das duas abordagens elimina a necessidade de desenhar limites, tratar erros e medir. A comparação útil não é qual linguagem “ganha”, mas que custo cada uma paga e que garantia oferece em troca.

Leia primeiro os capítulos 16 e 17 de The Rust Book. Depois faça os exercícios do Rustlings sobre threads e canais antes de adaptar o `revisor`. A ordem importa: o async fica muito menos misterioso quando você já entende o que significa um closure ser movido para outra thread, o que quer dizer `Send` e por que compartilhar mutabilidade exige uma sincronização visível.

## Os conceitos

### Threads do sistema, `move` e `join`

`std::thread::spawn` pede um closure e inicia uma thread do sistema operacional para executá-lo. O valor que devolve é um `JoinHandle<T>`: uma promessa concreta de que a thread pode terminar com um valor do tipo `T`. Chamar `join()` espera essa thread terminar e devolve `Result<T, Box<dyn Any + Send>>`; o `Err` representa que a thread entrou em `panic!`.

A palavra-chave `move` é importante. Um closure sem `move` pode tentar capturar uma referência a uma variável do contexto externo. Uma thread pode continuar executando depois que esse contexto terminou, então o Rust não permite entregar à thread uma referência que talvez deixe de ser válida. `move` faz o closure capturar por valor. Na figura, cada `Servicio` deixa de pertencer ao vetor e passa a pertencer ao closure de sua própria thread.

**Fig. 7.1** | Uma thread por serviço.

```rust
// fig07_01.rs
use std::thread;

struct Servicio {
    nombre: String,
}

type Estado = String;     // en el curso es el enum de la lección 3; aquí basta un texto

fn revisar(s: &Servicio) -> Estado {
    format!("{}: OK", s.nombre)
}

fn main() {
    let servicios = vec![
        Servicio { nombre: "catalogo".to_string() },
        Servicio { nombre: "pagos".to_string() },
        Servicio { nombre: "reportes".to_string() },
    ];

    let handles: Vec<_> = servicios.into_iter().map(|s| {
        thread::spawn(move || revisar(&s))      // `move` entrega la propiedad al hilo
    }).collect();

    for h in handles {
        let estado = h.join().unwrap();          // espera y recoge el resultado
        println!("{estado}");
    }
}
```

```bash
$ rustc --edition 2024 fig07_01.rs && ./fig07_01
catalogo: OK
pagos: OK
reportes: OK
```

A ordem dos `println!` é determinística mesmo que as threads terminem em outra ordem. Os `JoinHandle` são guardados na mesma ordem que `servicios.into_iter()` produz, e o segundo `for` chama `join()` nessa ordem. Se a thread de `pagos` termina primeiro, seu valor fica pronto, mas o programa primeiro espera e imprime o de `catalogo`. Essa diferença importa: concorrência não obriga a saída a ser não determinística. Você pode desenhar uma fronteira em que a ordem observável continue estável.

`join` basta quando cada tarefa tem um resultado e o número de tarefas é pequeno e conhecido. Você não precisa de um canal só para recuperar um valor; o handle já o entrega. Em Go você normalmente combinaria uma goroutine com um `WaitGroup` para esperar e um canal ou uma coleção protegida para recuperar resultados. O Rust faz do resultado parte do handle, embora isso não elimine a utilidade dos canais para comunicação progressiva.

Uma thread do sistema não é de graça. Tem recursos do sistema operacional, uma pilha e um custo de escalonamento maior que uma tarefa async. Não existe um número universal de memória por thread: depende do sistema operacional, da arquitetura e da configuração. A regra de design é mais útil que um número fixo: não abra uma thread do sistema por conexão se o trabalho principal consiste em esperar rede. Para algumas tarefas de CPU ou uma API bloqueante, uma thread pode ser exatamente o certo. Para muitos serviços HTTP, o `revisor` usa async.

O `revisor` não cria uma thread por serviço. Seu trabalho de rede se expressa como futures async; o runtime decide quais threads executam esses futures. Mesmo assim, a mesma ideia de propriedade aparece: uma tarefa deve possuir o que conserva durante sua execução, ou receber empréstimos que continuem válidos até que ela termine. Por isso é importante entender primeiro o `move`, mesmo que o código final use o Tokio.

### `Arc`, `Mutex` e o dado que vive dentro do cadeado

Um `Rc<T>` permite que vários donos dentro de uma única thread compartilhem um valor. Seu contador de referências não é atômico, portanto não pode ser compartilhado entre threads. `Arc<T>` significa *atomic reference counted*: cumpre a mesma função geral, mas atualiza o contador de referências de forma segura entre threads. Clonar um `Arc` não clona `T`; apenas cria outro dono da mesma alocação.

Ter vários donos não equivale a ter permissão para modificar. Se `T` é mutável e várias tarefas podem acessá-lo, é preciso coordenação. `Mutex<T>` contém o dado e permite que uma única tarefa por vez receba acesso mutável por meio de `lock()`. O resultado de `lock()` é um `MutexGuard<T>`. Enquanto esse guard existir, o cadeado continua tomado; ao sair de seu escopo, seu `Drop` libera o cadeado. Você não precisa escrever uma chamada separada a `unlock`, e isso reduz o risco de esquecer de liberar o recurso em algum caminho de retorno.

**Fig. 7.2** | Um dado compartilhado entre threads.

```rust
// fig07_02.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let estado = "OK".to_string();

    let estados = Arc::new(Mutex::new(HashMap::new()));
    let copia = Arc::clone(&estados);
    let h = thread::spawn(move || {
        copia.lock().unwrap().insert("x".to_string(), estado);
    });

    h.join().unwrap();
    println!("{:?}", estados.lock().unwrap());
}
```

```bash
$ rustc --edition 2024 fig07_02.rs && ./fig07_02
{"x": "OK"}
```

A forma `Arc<Mutex<HashMap<_, _>>>` se lê de dentro para fora. O `HashMap` é o dado. O `Mutex` é a única porta para mutá-lo. O `Arc` permite que threads distintas sejam donas dessa porta. Para inserir, a thread toma o cadeado e recebe o guard; para imprimir, `main` toma o cadeado de novo. Nunca aparece uma referência mutável ao mapa sem que o guard exista.

Em Go é frequente declarar um `sync.Mutex` junto ao mapa e seguir a convenção de chamar `Lock` antes de tocá-lo. Essa convenção pode ser bem encapsulada, mas a linguagem não obriga o mapa a estar fisicamente dentro do mutex. Em Rust, ao envolver o dado em `Mutex<T>`, a API normal não deixa obter `&mut T` sem um `MutexGuard`. Isso não torna impossíveis todos os erros de concorrência: você ainda pode provocar um deadlock tomando cadeados em ordens inconsistentes, ou manter um guard por tempo demais. Mas torna impossível, em código seguro, uma corrida de dados causada por emprestar simultaneamente o mesmo valor mutável sem sincronização.

Não use `lock().unwrap()` como uma fórmula que não se pensa. `lock()` pode devolver erro se outra thread entrou em `panic!` enquanto tinha o cadeado; isso se chama envenenamento (poisoning) do mutex. Num exemplo pedagógico, `unwrap()` torna esse caso visível com um `panic!`. Num serviço real você deve decidir se esse estado invalida o programa, se pode recuperar o dado com `into_inner`, ou se convém redesenhar para que o estado compartilhado não seja necessário.

O `revisor` evita um `Mutex<HashMap<...>>` porque não precisa ir preenchendo um mapa compartilhado conforme chega cada resposta. Cada future produz seu próprio `Estado`, e `join_all` os reúne. O recurso que de fato compartilha é um limite de turnos: vários futures precisam pedir permissão para iniciar uma consulta, não modificar uma coleção comum. Por isso o projeto usa `Arc<Semaphore>` e não `Arc<Mutex<Vec<Estado>>>`.

### Canais: entregar valores em vez de compartilhar uma coleção

Um canal divide a comunicação em duas pontas: um emissor, `Sender<T>`, e um receptor, `Receiver<T>`. O emissor entrega valores com `send`; o receptor os toma com `recv` ou iterando sobre ele. Em vez de várias threads escreverem numa estrutura compartilhada, cada produtor entrega um valor que passa a ser propriedade do receptor. Essa arquitetura reduz a zona compartilhada e esclarece quem monta o resultado final.

O canal criado com `std::sync::mpsc::channel()` é de múltiplos produtores e um consumidor. É “múltiplos” porque você pode clonar o emissor antes de mover uma cópia para cada thread. O receptor não é clonado: um único lugar decide o que fazer com cada mensagem. Quando todos os emissores desaparecem, a iteração sobre o receptor termina. Esse fechamento do canal faz parte do protocolo, não é um detalhe incidental.

**Fig. 7.3** | Um canal entrega estados à thread que imprime o relatório.

```rust
// fig07_03.rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (emisor, receptor) = mpsc::channel();

    let hilo = thread::spawn(move || {
        for estado in ["catalogo: OK", "pagos: OK"] {
            emisor.send(estado).unwrap();
        }
    });

    hilo.join().unwrap();

    for estado in receptor {
        println!("{estado}");
    }
}
```

```bash
$ rustc --edition 2024 fig07_03.rs && ./fig07_03
catalogo: OK
pagos: OK
```

O programa espera a thread antes de percorrer o receptor para conservar uma saída determinística. Num programa que de fato processa resultados conforme chegam, você normalmente começaria a receber enquanto os produtores continuam trabalhando. Então a ordem seria a de chegada, não necessariamente a dos serviços de entrada. Essa pode ser uma boa decisão para uma interface que informa progresso, mas não para um relatório que deve alinhar cada estado com o serviço original.

Os canais não substituem automaticamente os mutexes. Se várias tarefas precisam ler e atualizar a mesma conta, talvez um `Mutex` seja o modelo natural. Se uma tarefa produz valores e outra decide como armazená-los ou exibi-los, um canal costuma representar melhor a responsabilidade. O erro comum é escolher por moda: “os mutexes são ruins” ou “os canais são complicados”. A pergunta útil é quem deve ser dono de cada dado em cada momento.

No `revisor`, `join_all` cumpre uma função parecida com a de receber todos os resultados, mas com um contrato adicional: conserva a ordem dos futures de entrada, de modo que `estados[i]` é sempre o resultado de `servicios[i]`. É isso que o relatório precisa: saber a que serviço pertence cada estado. Atenção ao que ele *não* promete: o relatório não é impresso na ordem da lista YAML, porque `reporte::tabla` e `reporte::json` ordenam as linhas por nome de serviço antes de escrevê-las (você verá isso na lição 8). O que `join_all` garante é a correspondência entre cada serviço e seu estado, e com um canal, onde os resultados chegam na ordem em que terminam, seria preciso reconstruí-la à mão. Se o produto precisasse imprimir “terminou pagos” assim que chegasse a resposta, um canal ou um stream (uma sequência de valores que vão chegando ao longo do tempo) seria uma opção razoável. Não o adicione só porque existe; a escolha atual é intencional e mantém o relatório reproduzível.

### `Send`, `Sync` e o erro que o compilador barra

`Send` e `Sync` são traits marcadores. Não costumam exigir métodos próprios; descrevem propriedades de segurança que o Rust pode derivar dos campos de um tipo. Um tipo `Send` pode ser transferido por valor para outra thread. Um tipo `Sync` pode ser compartilhado por referência entre threads: se `T` é `Sync`, então `&T` é `Send`. Muitas estruturas comuns os implementam automaticamente quando seus componentes também são seguros, mas `Rc<T>` não é `Send` nem `Sync` porque seu contador não pode ser atualizado a partir de várias threads.

Essa regra não é uma lista para memorizar. É uma pergunta que o Rust responde por composição. Se você faz um struct que contém `Rc<RefCell<_>>`, ele herda as restrições dessas peças. Se muda para `Arc<Mutex<_>>`, muda a representação e também as garantias disponíveis. O compilador segue o valor até o closure enviado a `thread::spawn` e exige que a fronteira seja segura.

Em Go, uma corrida de dados pode compilar e exigir `go test -race` para ser detectada durante uma execução que alcance justamente a intercalação problemática. O detector é valioso e você deve usá-lo, mas depende de que o teste execute o caminho conflitante. O Rust evita as corridas de dados em código seguro antes de rodar o programa. Isso não prova que a lógica esteja correta nem detecta automaticamente deadlocks, inanição (starvation: uma tarefa que nunca consegue a vez porque outras monopolizam o recurso) ou protocolos mal desenhados. Também existe `unsafe`, onde quem programa assume responsabilidades adicionais. A afirmação precisa é: o Rust evita corridas de dados por meio de suas regras de tipos e empréstimos em código seguro; não promete que todo programa concorrente seja correto.

Dentro do `revisor`, `Arc<Semaphore>` é válido porque o semáforo do Tokio foi projetado para ser compartilhado entre tarefas. Cada future recebe seu próprio `Arc`, pede uma permissão e a conserva durante a requisição. O tipo da permissão e seu ciclo de vida expressam que o turno não pode ser devolvido antes de terminar a consulta. Não há um contador `usize` compartilhado que cada future incremente e decremente manualmente.

### `async`, futures e o runtime do Tokio

Uma função marcada `async fn` não executa imediatamente todo o seu corpo ao ser chamada. Produz um future: um valor que representa trabalho pendente. Esse future avança quando um executor o consulta (poll). Se chega a uma operação que ainda não está pronta, como esperar uma resposta de rede, devolve o controle ao executor. Mais tarde, quando a operação puder continuar, o executor o consulta de novo.

`.await` é o ponto onde uma função async espera o resultado de outro future. Por si só não cria uma tarefa nova nem uma thread nova. Essa distinção corrige dois mal-entendidos frequentes. Primeiro: escrever `let futuro = revisar(...);` não inicia necessariamente a requisição; apenas constrói o future. Segundo: chamar `.await` uma após a outra no mesmo bloco pode tornar as operações seriais. Para iniciar várias operações de forma concorrente, você constrói vários futures e os conduz juntos com um combinador como `join_all`, ou os converte em tarefas com `tokio::spawn` quando realmente precisa de independência.

O Rust define a sintaxe e os traits de async, mas não inclui um executor async completo na biblioteca padrão. O Tokio é o runtime escolhido por este projeto. Fornece executor, temporizadores, sincronização async e adaptações de E/S. Algumas bibliotecas async são independentes do runtime, mas os recursos do Tokio, como seus temporizadores e vários tipos de sincronização, exigem execução dentro de um contexto Tokio. Leia a documentação de cada crate antes de supor que qualquer future funciona igual com qualquer runtime.

<!-- verificar:extracto:src/main.rs -->
```rust
#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    ejecutar(&args).await
}

async fn ejecutar(args: &Args) -> ExitCode {
```

O atributo `#[tokio::main]` constrói o runtime e executa a função `main` async. O `main` do programa continua devolvendo um `ExitCode`, como você viu na lição 4; o que muda é que agora ele pode esperar operações async antes de decidir o código de saída. O binário conserva a responsabilidade de interpretar argumentos, imprimir e sair; a biblioteca conserva a lógica de consultar serviços.

Não bloqueie uma thread do runtime com `std::thread::sleep`, leitura pesada de arquivos ou cálculo longo dentro de uma função async. Uma thread bloqueada não pode consultar outros futures atribuídos a ela. Para trabalho bloqueante existe `tokio::task::spawn_blocking`; para E/S de rede, use APIs async como o `reqwest`. O `revisor` usa `reqwest::Client` e espera seu `send().await`, de modo que, enquanto uma resposta está pendente, o runtime pode fazer avançar consultas de outros serviços.

Antes de ver como o `revisor` usa o Tokio, convém ver o Tokio sozinho. O programa seguinte é o menor que mostra o que importa: três “verificações” que, em vez de consultar a rede, simplesmente esperam, cada uma um tempo diferente. Não cabe num `rustc` puro, porque depende dos crates `tokio` e `futures`; por isso vive em `programas/revisor/examples/` e se executa com o Cargo, que baixa e compila essas dependências.

**Exemplo de cargo com `tokio`** | Três esperas conduzidas ao mesmo tempo com `join_all`: terminam numa ordem e são entregues em outra.

<!-- verificar:ejemplo:ejemplo_tokio -->
```rust
// ejemplo_tokio.rs
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::time::sleep;

async fn revisar(nombre: &str, espera_ms: u64) -> String {
    sleep(Duration::from_millis(espera_ms)).await;
    println!("terminó {nombre}");
    format!("{nombre}: respondió tras {espera_ms} ms")
}

#[tokio::main]
async fn main() {
    let servicios = [("catalogo", 600), ("pagos", 200), ("usuarios", 400)];
    let inicio = Instant::now();

    let futuros = servicios.iter().map(|(nombre, ms)| revisar(nombre, *ms));
    let resultados = join_all(futuros).await;

    println!("--- en el orden de la lista ---");
    for resultado in &resultados {
        println!("{resultado}");
    }
    // Esperarlos uno tras otro habría tardado 1200 ms; a la vez tardan lo del más lento.
    let a_la_vez = inicio.elapsed() < Duration::from_millis(1100);
    println!("tardó menos que la suma de las esperas: {a_la_vez}");
}
```

```bash
$ cargo run --example ejemplo_tokio
terminó pagos
terminó usuarios
terminó catalogo
--- en el orden de la lista ---
catalogo: respondió tras 600 ms
pagos: respondió tras 200 ms
usuarios: respondió tras 400 ms
tardó menos que la suma de las esperas: true
```

Execute-o a partir de `programas/revisor/` (na primeira execução o Cargo demora um pouco para compilar as dependências).

`#[tokio::main]` converte `main` numa função async: constrói o runtime e entrega a ele o future que `main` descreve. `tokio::time::sleep` é a espera do Tokio, e se parece com `std::thread::sleep` no que faz, mas não em como: com `.await`, a tarefa cede o controle ao runtime enquanto espera, e o runtime aproveita para fazer as demais avançarem. Com `std::thread::sleep`, a thread inteira ficaria dormindo e nada mais avançaria nela.

Leia a saída em duas partes. As mensagens `terminó ...` saem na ordem em que cada espera se cumpre: `pagos` (200 ms), `usuarios` (400 ms) e `catalogo` (600 ms), embora a lista os declare em outra ordem. Depois, `join_all` entrega os resultados na ordem da lista —`catalogo`, `pagos`, `usuarios`—, porque devolve um `Vec` em que cada posição corresponde ao seu future de entrada. A última linha comprova que foi concorrente: esperar as três verificações uma após a outra teria levado 1200 ms, e em conjunto leva o que demora a mais lenta, uns 600 ms.

Se você trocar `sleep` por uma chamada de rede com `.send().await`, tem a forma do `revisor`: muitos futures que esperam, um único `join_all` que os conduz e um `Vec` de resultados alinhado com a lista de serviços.

### `join_all`, semáforos e o limite de paralelismo do `revisor`

Lançar todas as requisições possíveis ao mesmo tempo nem sempre é uma melhoria. Um arquivo com milhares de serviços poderia abrir conexões demais, saturar a rede local, esgotar descritores de arquivo ou sobrecarregar o servidor que você justamente tenta verificar. A concorrência precisa de um limite. O argumento `--paralelo` do `revisor` expressa quantas consultas podem estar ativas ao mesmo tempo.

Um semáforo contém permissões. Para iniciar uma consulta, um future adquire uma; se não sobra nenhuma, espera. Quando a permissão sai de escopo, é liberada automaticamente e outro future pode continuar. É a mesma ideia de RAII (o recurso é liberado quando o valor que o representa sai de escopo) que você viu com o `MutexGuard`: o recurso é liberado ao destruir o guard, mesmo que a função saia por um caminho normal. Aqui o recurso não é um cadeado exclusivo, mas uma capacidade limitada.

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

`servicios.iter()` conserva empréstimos à lista; não consome os serviços. Cada closure async possui seu clone de `Arc<Semaphore>`, mas toma emprestados `cliente` e `s` durante a chamada a `revisar`. Isso funciona porque `join_all(futuros).await` termina antes que `revisar_todos` possa retornar, então esses empréstimos continuam vivos. Se você usasse `tokio::spawn`, a tarefa poderia sobreviver à função que a criou e normalmente precisaria de dados com tempo de vida `'static`; aí você teria de mover ou clonar mais dados.

`join_all` devolve um `Vec<Estado>` na ordem dos futures de entrada, não na ordem em que as requisições terminam. É uma decisão útil para o relatório: o estado zero corresponde ao serviço zero. O semáforo limita quando cada future pode entrar em `revisar`, mas não muda essa relação final. Assim, o programa tem concorrência real sem transformar o relatório numa fonte de ordem aleatória.

A função `revisar` não devolve `Result<Estado>`. Essa decisão merece atenção. Um HTTP 500, um timeout ou uma conexão recusada não é um erro interno que impeça o programa de continuar: é justamente a informação que o `revisor` deve reportar sobre um serviço. Por isso esses casos são convertidos em variantes `Estado::Falla`. O erro de `acquire` é tratado de forma diferente porque o semáforo nunca é fechado neste design; se ocorresse, seria uma violação de uma suposição interna.

O timeout do projeto é configurado sobre a requisição do `reqwest`, com o `timeout_ms` de cada `Servicio`. Não confunda esse limite com o semáforo. O timeout limita quanto uma consulta individual pode esperar; o semáforo limita quantas consultas podem estar esperando ou se comunicando ao mesmo tempo. Você precisa dos dois: sem timeout, um turno pode ficar ocupado por tempo demais; sem semáforo, muitas requisições com timeout podem começar todas juntas e sobrecarregar recursos.

### Comparação honesta com Go

O Go torna muito fácil iniciar uma unidade concorrente: `go revisar(s)`. O runtime escalona goroutines sobre threads do sistema, faz crescer suas pilhas e administra o trabalho de rede. O Rust separa explicitamente a decisão: `thread::spawn` cria uma thread do sistema; um runtime como o Tokio executa futures async; `tokio::spawn` cria uma tarefa Tokio. Isso significa que o Go costuma ter menos cerimônia no início e o Rust obriga a saber qual das três abstrações você está usando.

O Rust não tem uma vantagem mágica de desempenho por escrever `async`. Uma tarefa async não acelera uma consulta HTTP individual; melhora a utilização das threads enquanto várias consultas esperam. Para uma carga de CPU, o async pode ser pior se bloquear o executor. Nesse caso use threads, um pool de workers ou `spawn_blocking`. O Go tampouco transforma automaticamente um cálculo intensivo de CPU em algo mais rápido: várias goroutines podem competir pelos mesmos núcleos. Medir o tipo de trabalho importa mais que aplicar uma palavra da moda.

A garantia central do Rust aparece antes de executar: um dado mutável não pode ser emprestado de forma incompatível, e os valores enviados a outra thread devem cumprir os traits adequados. O Go privilegia uma sintaxe pequena e ferramentas de execução como o detector de corridas. O Go pode encapsular corretamente mutexes e canais; o Rust pode ter deadlocks e erros lógicos. A diferença não é “o Go permite erros e o Rust não”. É onde cada linguagem coloca a carga de verificação e que erros pode rejeitar antes de rodar.

Para o `revisor`, a decisão fica justificada pelo problema. Há muitas esperas HTTP, cada resultado deve continuar ligado ao seu serviço e é preciso um teto configurável de requisições. Tokio, `join_all` e `Semaphore` expressam essas três necessidades. Um design com uma thread por serviço funcionaria para uma lista pequena, mas escalaria pior e não traz vantagem ao caso de uso. Um design com um mutex e um mapa compartilhado também poderia funcionar, mas complicaria uma relação que `join_all` já conserva.

## O erro que você vai ver

O erro mais instrutivo desta lição é o `E0277`. Não significa que “o Rust não quer usar threads”; significa que o tipo que você tenta mover não satisfaz o contrato que `thread::spawn` exige. `Rc<i32>` é útil para compartilhar propriedade dentro de uma thread, mas seu contador de referências não é atômico. Movê-lo para um closure que pode executar em outra thread seria inseguro.

**Fig. 7.4** | `Rc<T>` não pode ser enviado a uma thread.

```rust
// fig07_04.rs
use std::rc::Rc;
use std::thread;

fn main() {
    let conteo = Rc::new(0);
    thread::spawn(move || println!("{conteo}"));
}
```

```bash
$ rustc --edition 2024 fig07_04.rs
error[E0277]: `Rc<i32>` cannot be sent between threads safely
 --> fig07_04.rs:7:19
  |
7 |     thread::spawn(move || println!("{conteo}"));
  |     ------------- -------^^^^^^^^^^^^^^^^^^^^^
  |     |             |
  |     |             `Rc<i32>` cannot be sent between threads safely
  |     |             within this `{closure@fig07_04.rs:7:19: 7:26}`
  |     required by a bound introduced by this call
  |
  = help: within `{closure@fig07_04.rs:7:19: 7:26}`, the trait `Send` is not implemented for `Rc<i32>`
note: required because it's used within this closure
 --> fig07_04.rs:7:19
  |
7 |     thread::spawn(move || println!("{conteo}"));
  |                   ^^^^^^^
note: required by a bound in `spawn`
 --> /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library/std/src/thread/functions.rs:125:0

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

A mensagem contém a resposta. O closure captura `Rc<i32>`, `thread::spawn` exige que o capturado seja `Send`, e `Rc<i32>` não implementa `Send`. Não conserte este erro acrescentando traits manualmente com `unsafe impl Send`; você estaria prometendo ao compilador uma segurança que o `Rc` não oferece. Se várias threads só precisam ler um dado, use `Arc<T>`. Se além disso precisam modificá-lo, avalie `Arc<Mutex<T>>` ou redesenhe o fluxo para enviar valores por canais.

Reconheça também o erro de design que às vezes o compilador não rejeita: manter um `MutexGuard` durante uma operação `.await`. Se você lança a tarefa com `tokio::spawn` e o guard é de `std::sync::Mutex`, o `rustc` de fato a rejeita (`future cannot be sent between threads safely`, porque esse guard não é `Send`); mas se o future é esperado na mesma thread, como faz `join_all` no `revisor`, o programa compila. Para esse caso o `clippy` traz por padrão o aviso `await_holding_lock`. Um guard de `std::sync::Mutex` bloqueia uma thread; um guard de um mutex async conserva o cadeado enquanto a tarefa pode ceder o executor. Em ambos os casos, esperar rede enquanto você tem o cadeado costuma bloquear trabalho desnecessariamente e pode produzir deadlocks. Extraia ou atualize o dado sob o cadeado, solte o guard e só então espere.

## O que se faz errado

- Criar uma thread para cada serviço sem limite. Funciona com três exemplos e falha como estratégia quando a lista cresce. As threads do sistema consomem recursos do sistema operacional e uma quantidade grande delas torna mais difícil planejar, medir e depurar. Para E/S de rede, use async com um limite de paralelismo; para CPU, use uma quantidade de threads proporcional ao trabalho e aos núcleos disponíveis.

- Usar `Arc<Mutex<_>>` como resposta automática a qualquer erro de ownership. Essa combinação é correta quando há estado mutável genuinamente compartilhado, mas pode esconder um design em que várias tarefas fazem coisa demais. Se cada tarefa pode devolver um valor e uma única parte os junta, um canal, `join_all` ou uma redução posterior costuma ser mais claro e reduz a contenção.

- Manter um cadeado durante uma requisição de rede ou um `.await`. O guard existe para proteger uma seção crítica pequena. Se você o conserva enquanto espera, transforma tarefas concorrentes numa fila e aumenta a possibilidade de deadlock. Limite o escopo com chaves ou com uma variável temporária para que o guard seja destruído antes da espera.

- Chamar funções bloqueantes dentro de código Tokio. `std::thread::sleep` bloqueia a thread, não apenas uma tarefa. Uma leitura grande, uma consulta bloqueante ou um cálculo pesado têm o mesmo problema. Use APIs async para E/S, `tokio::time` para temporizadores ou `spawn_blocking` para trabalho que realmente deve bloquear.

- Confundir `async` com paralelismo. Um future pode avançar concorrentemente com outros e ainda assim executar sobre uma única thread. Se você precisa acelerar cálculo de CPU, deve decidir como distribuí-lo entre núcleos. Se espera rede, o async melhora a utilização das threads existentes. Antes de otimizar, meça o que o programa está esperando.

- Usar `tokio::spawn` só para “torná-lo concorrente”. No `revisor`, `join_all` pode conduzir futures que emprestam `cliente` e `servicios`, conserva a ordem e evita exigir propriedade `'static`. `tokio::spawn` é útil para tarefas independentes que devem viver além do bloco atual, mas implica outro contrato de vida e de tipos.

- Tratar a saída que chega primeiro como se fosse o estado do serviço que ocupa aquela posição. Numa interface de progresso pode ser útil informar por chegada. Num relatório, se o resultado de outro serviço se infiltra numa linha, cada linha mente sobre seu serviço. O `revisor` evita esse risco com `join_all`, que conserva a correspondência entre `servicios[i]` e `estados[i]`, e seus testes de integração verificam essa propriedade; a ordem em que as linhas são impressas é outra decisão, que o relatório toma ao ordená-las por nome.

## Exercícios

### Exercício 1 — Três verificações com `join`

Escreva um programa com três `Servicio` de texto. Use `thread::spawn(move || ...)` para produzir um estado para cada um, conserve os handles num vetor e use `join` para imprimir os resultados na mesma ordem de entrada. Não use `sleep` para “dar tempo” às threads.

### Exercício 2 — Contador protegido

Crie um `Arc<Mutex<u32>>` com valor inicial zero. Lance quatro threads; cada uma deve incrementar o contador uma vez. Espere todos os handles e imprima `total: 4`. Depois troque de propósito `Arc` por `Rc` e confirme que aparece `E0277`.

### Exercício 3 — Resultados por canal

Crie um canal e três threads produtoras. Cada produtor deve mandar o nome de um serviço e um estado. A thread principal deve receber exatamente três mensagens e ordená-las por nome antes de imprimi-las. Explique num comentário por que você não deve depender da ordem de chegada.

### Exercício 4 — Explique o limite do `revisor`

Leia `programas/revisor/src/revisar.rs`. Execute os testes de integração e localize o teste `el_tope_de_paralelo_se_respeta`. No seu diário de bordo responda: que recurso o `Semaphore` controla, quando a permissão é adquirida, quando é liberada e por que `join_all` conserva a ordem de `servicios`.

## Soluções

### Solução 1

A solução correta move cada `Servicio` para a thread e recolhe os handles depois. O ponto decisivo não é o `map`, mas que o vetor de handles mantém a obrigação de esperar cada trabalho antes de terminar `main`. Se você imprime depois de cada `join`, a ordem observável é a ordem do vetor de handles.

Uma forma de comprovar que você não dependeu de `sleep` é executar o programa várias vezes: ele deve imprimir as três linhas sempre. O escalonador pode mudar qual thread termina primeiro, mas não pode evitar que `join` espere.

### Solução 2

Cada thread deve receber seu próprio `Arc::clone(&contador)`. Dentro da thread, tome o guard, incremente e deixe o guard sair de escopo. Depois de esperar os quatro handles, tome um último guard para imprimir o valor. Não tente manter um empréstimo mutável do contador fora do mutex; esse empréstimo não pode coexistir com as demais threads.

Ao trocar `Arc` por `Rc`, o resultado esperado não é uma saída numérica, mas `E0277`. A correção não consiste em “silenciar” o compilador: `Rc` serve para referências compartilhadas de uma única thread; `Arc` é o tipo adequado para que o contador tenha donos em várias threads.

### Solução 3

Cada produtor recebe um clone do emissor e manda uma estrutura ou tupla com o nome e o estado. O emissor original deve deixar de existir antes de percorrer todo o receptor, ou você pode chamar `recv` exatamente três vezes porque conhece o número de produtores. Guarde as mensagens recebidas num vetor e ordene-o por nome antes de imprimir.

A parte importante é que o receptor é o único dono do vetor final. Os produtores não têm acesso mutável a ele. Por isso você não precisa de um mutex para reunir os resultados; a propriedade viaja pelo canal junto com cada mensagem.

### Solução 4

O semáforo controla o número de consultas HTTP que podem estar ativas ao mesmo tempo, não o número total de serviços. Cada future adquire uma permissão logo antes de chamar `revisar(cliente, s).await`. A permissão vive em `_turno`; quando essa chamada termina, `_turno` sai de escopo e devolve a capacidade ao semáforo.

`join_all` recebe futures construídos ao percorrer `servicios` e produz o vetor de estados nessa mesma sequência. Por isso os resultados podem terminar em tempos distintos sem se desalinhar do serviço que os originou. O teste mede tempos para confirmar que o limite muda o comportamento e não é apenas uma flag decorativa.

## Como sei que consegui

Execute as figuras desta lição a partir de seus diretórios e compare a saída exata:

```bash
cd programas/07-concurrencia-async
rustc --edition 2024 fig07_01.rs && ./fig07_01
rustc --edition 2024 fig07_02.rs && ./fig07_02
rustc --edition 2024 fig07_03.rs && ./fig07_03
rustc --edition 2024 fig07_04.rs
```

As três primeiras compilações devem terminar sem avisos e produzir as saídas documentadas. A última deve falhar com `E0277`; essa falha é o resultado correto do exercício.

Confirme que os blocos e suas saídas continuam verificáveis a partir da raiz do curso:

```bash
herramientas/verificar-programas.sh es
herramientas/verificar-extractos.sh
herramientas/verificar-ejemplos.sh
```

Os três comandos devem terminar corretamente. O primeiro confirma que cada figura compila, executa e coincide com sua saída documentada, exceto a figura desenhada para falhar. O segundo confirma que os trechos do `revisor` continuam sendo cópias exatas do projeto real. O terceiro compila e executa o exemplo com `tokio` desta lição e compara o que ele imprime com o documentado.

Por fim, verifique o comportamento concorrente real do projeto:

```bash
cd programas/revisor
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

`cargo test` deve informar resultados bem-sucedidos, incluindo o teste `el_tope_de_paralelo_se_respeta`. `cargo clippy --all-targets -- -D warnings` deve terminar sem warnings, e `cargo fmt --check` não deve propor mudanças. Se você consegue explicar por que o semáforo limita consultas, por que `join_all` conserva a ordem e por que `Rc` provoca `E0277`, você terminou a lição.

## Para ler mais

- [The Rust Programming Language, capítulo 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — consultado em 2 de outubro de 2026.

- [The Rust Programming Language, capítulo 17: Fundamentals of Asynchronous Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html) — consultado em 2 de outubro de 2026.

- [Documentação de `std::thread`](https://doc.rust-lang.org/std/thread/) — consultado em 2 de outubro de 2026.

- [Documentação de `tokio::sync::Semaphore`](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html) — consultado em 2 de outubro de 2026.
