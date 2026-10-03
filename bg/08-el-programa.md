# Урок 8 — Завършената програма

**Време:** 2 × 45 мин.

**Какво изграждаш:** пълният `revisor`, в един бинарен файл

**Какво научаваш:** `reqwest`, `serde`, `clap`, профилът release, крайният бинарен файл и сравнението му с този на Go

## След края ще можеш да

- Обясниш каква отговорност има `main.rs` и защо преизползваемата логика живее в библиотеката `revisor`.
- Четеш YAML конфигурация със `serde`, включително стойности по подразбиране и валидирания, които YAML не може да изрази.
- Използваш `clap`, за да декларираш опции на командния ред с типове, стойности по подразбиране и автоматична помощ.
- Обясниш защо едно HTTP срутване е резултат от домейна на `revisor`, докато невалиден конфигурационен файл пречи на стартирането.
- Компилираш, тестваш и преглеждаш бинарния файл с `cargo test`, `cargo clippy` и `cargo build --release`.
- Сравняваш с конкретни аргументи крайния бинарен файл на Rust и еквивалентната програма на Go.

## Защо, преди как

През предишните уроци построи частите на `revisor`: речника на проблема, четенето на YAML, доклада, тестовете и конкурентните HTTP запитвания. Една изолирана част може да е добре написана и все пак да не е инструмент, който друг човек може да използва. Остава да ги обединиш в ясна граница: изпълним файл, който получава аргументи, чете файл, пита услуги, отпечатва полезен резултат и завършва с код, който друга програма може да интерпретира.

Това изглежда като тънък слой, но там се срещат важни решения. Командният ред е публичен интерфейс: ако днес приемаш `--formato json`, някой може да го интегрира в скрипт и да зависи от него утре. YAML файлът също е интерфейс: не е код на Rust, така че може да съдържа повтарящи се имена, непълни URL адреси или времево ограничение нула. Мрежата е друга граница: един HTTP отговор 500 не означава, че самият `revisor` е счупен; означава, че проверяваната услуга е в лошо състояние. Напротив, невъзможността да се прочете YAML наистина пречи да се започне работа.

Завършената програма трябва да различава тези случаи, без да ги крие под един и същ `unwrap()`. Ако всичко е здраво, завършва с код 0. Ако е успяла да провери и е открила повреда, отпечатва доклада и завършва с код 1. Ако не е могла да стартира, защото аргументите или конфигурацията са невалидни, съобщава проблема в изхода за грешки и завършва с код 2. Това разделяне прави бинарния файл полезен както за човек, който го изпълнява в терминал, така и за система за автоматизация.

Този урок продължава същата програма, която направи в Go. Сравнението има значение, защото и двата езика стигат до разпространяем бинарен файл, но поемат различни пътища. Go включва HTTP, JSON, флагове и конкурентност в стандартната си библиотека. Rust оставя стандартната библиотека малка и стабилна; за асинхронен HTTP, YAML сериализация или декларативен интерфейс на командния ред използва crate-ове от екосистемата. Това не е автоматично предимство на едната страна, нито автоматичен недостатък на другата. Това е решение за разпределение на работата между езика, мениджъра на пакети и библиотеките.

Проектът `programas/revisor/` фиксира конкретните версии с `Cargo.toml` и `Cargo.lock`. `cargo` разрешава цялото дърво от зависимости, а lockfile-ът запазва точното разрешение, за да построи друг компютър същия набор от съвместими crate-ове. Не копирай версии от стар урок на сляпо и не изпълнявай `cargo update` като автоматична реакция: първо разбери какво се е променило, прегледай lockfile-а и изпълни пълните тестове.

The Rust Book, глава 12, е централното четиво на този урок: организира програма за командния ред около вход, конфигурация, грешки и разделяне на отговорностите. Върни се и към упражненията на Rustlings, които са ти били трудни, за `Result`, тестове и итератори. Целта вече не е да запаметяваш синтаксис; тя е да видиш как решенията от предишните уроци оцеляват, когато програмата има потребители, файлове и реална мрежа.

## Понятията

### Бинарният файл е граница, не мястото за цялата логика

Един бинарен файл има специфична задача: да превежда външното във вътрешното на програмата. Чете аргументите на операционната система, решава какво се отпечатва, превръща общия резултат в изходен код и делегира останалото. Ако смесиш там зареждането на YAML, HTTP заявките, формата на таблицата и изходния код, тестовете завършват зависими от терминала и от `std::process::exit`. Това прави един малък тест бавен, крехък и труден за диагностика.

Алтернативата на `revisor` е да има библиотека с публични модули (`config`, `modelo`, `reporte` и `revisar`) и кратък `main.rs`. Библиотеката може да се тества от unit тестовете си и от `tests/integracion.rs`. Бинарният файл запазва само това, което непременно зависи от командния ред. Това разделяне не е церемониално правило: намалява броя на местата, където промяна на интерфейса може да счупи поведението.

Преди да използваш `clap`, е добре да разбереш работата, която автоматизира. Един парсер на аргументи трябва да поддържа състояние: всеки флаг консумира, когато е необходимо, стойността, която следва; трябва да запазва стойности по подразбиране; и трябва да отхвърля непознат флаг, вместо да го тълкува мълчаливо. Следващата програма симулира тази граница, без да зависи от crate-ове.

**Фиг. 8.1** | Минимален парсер запазва стойности по подразбиране и превръща текст в типове.

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

Програмата илюстрира защо не е удачно да пишеш този парсер на ръка за всеки бинарен файл. Не генерира `--help`, сама не документира опциите си, не валидира допустими стойности и кодът ѝ би нараснал бързо. `clap` декларира договора и генерира голяма част от тази механика. В `revisor` `Args` е договорът за входа: `archivo`, `formato` и `paralelo` имат дълго име, кратко име, тип и стойност по подразбиране.

Фигура 8.1 състави на ръка парсера на аргументи. Сега същият договор с `clap`, crate-а, който използва `revisor`. Както другите примери със зависимости, живее в `programas/revisor/examples/` и се изпълнява с Cargo от тази папка.

**Пример за cargo с `clap`** | Същият договор за аргументи, деклариран вместо програмиран.

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

`#[derive(Parser)]` пише вместо теб кода, който превръща списъка с аргументи в `Args`. В реалната програма се извиква `Args::parse()`, който чете аргументите на операционната система; тук се използва `try_parse_from`, който получава списъка като параметър, така че примерът да дава винаги същия изход и да може да се видят грешките, без да се прекратява програмата. Първият ред показва пълен `Args`: това, което е подадено (`formato` и `paralelo`), и стойността по подразбиране на това, което е липсвало (`archivo`). Следващите показват какво става със стойност, която не е число, и с опция, която не съществува: `clap` ги отхвърля с ясен тип грешка (`ValueValidation` и `UnknownArgument`), преди да стартира логиката на програмата. Когато използваш `parse()` вместо `try_parse_from`, `clap` отпечатва това съобщение с помощта за употреба и завършва с код 2, същия код, който `revisor` запазва за „не можах да стартирам“. Освен това `--help` и `--version` идват безплатно от атрибута `#[command(version, about = ...)]`.

Така изглежда пълната декларация в `revisor`:

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

Атрибутът `#[derive(Parser)]` генерира имплементация на trait-а `Parser`. Затова `Args::parse()` може да чете `std::env::args()` и да построи типизиран `Args`. `default_value` получава текст, защото `clap` го превръща към типа на полето; `default_value_t` получава директно стойност на Rust, затова е подходящ за `usize`.

`main` на проекта не извиква `std::process::exit`. Връща `ExitCode`, което е по-лесно за тестване, когато логиката се държи в `ejecutar`. Кодът първо проверява, че форматът е един от двата обещани договора. После зарежда и валидира конфигурацията, създава HTTP клиент, иска състоянията, решава дали да отпечата таблица или JSON и превръща крайния резултат в 0, 1 или 2. Редът има значение: няма причина да се отварят мрежови връзки, ако YAML файлът дори не се чете.

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

Анотацията `#[tokio::main]` създава и стартира runtime-а, необходим, за да може да се използва `.await` в главната функция. Не прави цялата програма по-бърза сама по себе си. Целта ѝ е да изпълнява future-и, докато HTTP заявките чакат данни от мрежата, без да блокира една нишка за всяко чакане.

### `serde` прави явен договора за данните

Един YAML файл пристига в програмата като текст. Текстът не знае какво е име, кое поле е задължително, нито каква стойност трябва да се използва, когато липсва `timeout_ms`. Превръщането му във `Vec<Servicio>` е преминаване от данни, които идват отвън, към стойности, които компилаторът може да преглежда. Това превръщане е граница на доверие: след десериализиране все още трябва да валидираш бизнес правилата, които форматът не познава.

`serde` разделя две посоки. `Deserialize` строи стойности на Rust от YAML, JSON, TOML или друг формат, който има съвместим адаптер. `Serialize` превръща стойности на Rust във формат за изход. Структурата на домейна се запазва; променя се форматът, който я обкръжава. Затова един и същ `EstadoJson` може да се генерира със `serde_json`, докато `Servicio` влиза с `yaml_serde`.

Следващата програма показва решение от домейна, което се появява и в JSON на `revisor`: едно здраво състояние има HTTP код и не носи грешка; една повреда не измисля код и включва причина. Програмата сглобява JSON на ръка, за да се види разликата. В реалния проект не бива да правиш това на ръка: `serde_json` се грижи да екранира текста и да запази валиден JSON.

**Фиг. 8.2** | Формата на изхода зависи от варианта на състоянието.

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

В `revisor` derive декларира точно какво трябва да се чете или пише. `Servicio` извежда `Deserialize`, защото се създава от YAML. `EstadoJson` извежда `Serialize`, защото се създава от вътрешните резултати, за да произведе JSON. `#[serde(default = "timeout_por_omision")]` не е равносилно на това полето да е незадължително в Rust: крайното поле си остава `u64`; само получава стойност, когато YAML не я декларира.

Следващата програма използва двата crate-а за данни на `revisor` в малък мащаб: `serde`, за да декларира договора, и `yaml_serde` и `serde_json` за форматите. Данните влизат като YAML текст, написан в самата програма, за да не зависи от никакъв файл.

**Пример за cargo със `serde`** | Същият `struct` чете YAML и пише JSON.

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

Същият `struct Servicio` служи и за четене, и за писане, защото извежда двете половини на `serde`: `Deserialize`, за да се построи от YAML, и `Serialize`, за да се запише като JSON (`Servicio` на `revisor` извежда само `Deserialize`, защото никога не се записва обратно). Обърни внимание на две подробности. Услугата `catalogo` не декларира `timeout_ms` в YAML и въпреки това излиза с 5000: това е ефектът на `#[serde(default = ...)]`. А невалидният YAML не срутва програмата с паника: `from_str` връща `Err`, чието съобщение казва какво липсва (``missing field `url` ``) и къде (`.[0]` е първият елемент на списъка; `line 1 column 3`, позицията в текста). Това съобщение е това, което `revisor` показва на този, който поправя файла си. И тъй като `yaml_serde` е поддържаното продължение на `serde_yaml` (оригиналният crate вече не получава промени, както видя в урок 6), всичко, което тук се прави с `from_str`, работи еднакво с което и да е от двете имена.

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

`Option<u16>` представя важна разлика: ако не е имало HTTP отговор, не съществува HTTP код. Използването на `0` или на празен низ би измислило данна и би задължило всеки потребител да помни тази конвенция. В JSON `Option::None` се превръща в `null` за `codigo`. За `error` атрибутът `skip_serializing_if` избира друга политика: когато няма грешка, ключът дори не се появява. И двете решения са валидни, но трябва да са умишлени и покрити с тестове.

Да десериализираш добре не стига. YAML може да изрази празен списък, да повтори име, да приеме низ като URL или да включва времево ограничение нула. Парсерът не знае, че тези опции правят `revisor` безполезен. `config::validar` е втората стъпка и изразява правилата на програмата, не тези на формата.

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

`anyhow::ensure!` връща по-рано грешка, когато едно условие не е изпълнено. Тук е подходящо, защото невалидна конфигурация не представлява неуспешна услуга: е условие, което пречи да започне проверката. `with_context` в `cargar` добавя името на файла към грешка при четене; форматът `{e:#}` в `main` отпечатва тази верига от контекст заедно с първоначалната причина. Резултатът е по-полезен от общо съобщение „не можа да се отвори“.

### `reqwest` превръща мрежата в състояния на домейна

Една програма за наблюдение не пита HTTP, за да получи тяло и да го забрави; пита, за да класифицира състоянието на една услуга. Затова функцията `revisar` не връща `Result<Estado>`. Ако един сървър отговори 500, откаже връзката или надхвърли времевото си ограничение, операцията на `revisor` наистина е приключила: открила е повреда и трябва да я включи в доклада. Тези случаи се превръщат в `Estado::Falla`.

Не бъркай този резултат с повреда на конфигурацията или с грешка на сериализиране при създаване на доклада. Те наистина пречат на програмата да си върши работата и я карат да завърши с код 2. Разграничението избягва две чести грешки: да се спре цялата проверка, защото една услуга е паднала, или да се продължи, сякаш нищо не е станало, когато не е могъл да се прочете файлът, който определя кои услуги съществуват.

За да видиш `reqwest`, без да зависиш от никаква реална услуга, следващата програма стартира на същата машина фалшив сървър и го пита с клиент на `reqwest`. Сървърът се сглобява с `TcpListener` от стандартната библиотека и отговаря на ръка с минималния текст на HTTP: отговаря `200` на `/sano`, `500` на `/roto` и се бави половин секунда да отговори на `/lento`. Освен това програмата се опитва да се свърже към порт, на който никой не слуша.

**Пример за cargo с `reqwest`** | Четири различни отговора на мрежата, видени от клиента.

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

Всеки ред от изхода е един от случаите, които `revisor` превръща в състояние на домейна. Един `500` не е грешка на `reqwest`: заявката е направена и сървърът е отговорил, така че `send().await` връща `Ok` с код `500`, а програмата решава какво означава това. Напротив, изчерпано време (`.timeout(...)` от 200 ms срещу сървър, който се бави 500) и отказана връзка наистина пристигат като `Err`, а самата грешка казва за кой от двата случая става дума с `is_timeout()` и `is_connect()`. Клиентът се създава само веднъж с `reqwest::Client::new()` и се преизползва във всяко запитване; времевото ограничение, напротив, се фиксира за всяка заявка. Функцията `revisar` на реалния проект е същата идея, със състоянията `Ok`, `Lento` и `Falla` вместо редове текст:

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

Мрежата принуждава и да се разделят конкурентността и редът. `revisor` може да стартира няколко запитвания едновременно, но изходът трябва да е възпроизводим и трябва да свързва всяко състояние с правилната му услуга. `join_all` запазва реда на входните future-и, дори отговорите да пристигат в друг ред. Семафорът ограничава колко запитвания влизат в активната секция; не определя крайния ред на `Vec<Estado>`.

Следващата фигура не използва мрежа нито crate, за да може всеки да я компилира с чист `rustc` и да получава винаги същия изход. Вместо HTTP представя политиката на семафора: с лимит 2 се предават редове по два. В реалния проект всеки ред живее, докато приключи заявката, а runtime-ът събужда задачата, когато мрежата отговори.

**Фиг. 8.3** | Лимит на конкурентността разделя чакащите на партиди, без да променя реда им.

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

Реалната функция създава `Arc<Semaphore>`, защото всеки future трябва да споделя същия брояч на редове. `Arc` позволява споделена собственост между задачи; `Semaphore` предава временно разрешение; а променливата `_turno` пази това разрешение до края на `revisar`. Началното долно подчертаване избягва предупреждение на компилатора, но стойността не се изхвърля: деструкторът ѝ връща разрешението, когато излезе от блока.

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

Параметърът `paralelo` има нюанс: програмата приема `0` като „използвай стойността по подразбиране“. Това е решение за интерфейса; не означава, че Tokio може да изпълни нула заявки. Константата `PARALELO_POR_OMISION` пази това правило близо до логиката, която я използва.

Отделното запитване създава лимит за всяка услуга с `.timeout(Duration::from_millis(s.timeout_ms))`. Този лимит не замества семафора. Timeout-ът решава колко може да чака една заявка; семафорът решава колко заявки могат да чакат едновременно. Без timeout един ред може да остане зает твърде дълго. Без семафор един огромен списък може да отвори твърде много връзки едновременно. Двата лимита защитават различни ресурси.

`reqwest::Client::new()` се създава веднъж и се заема на всички запитвания. Не строй клиент за всяка услуга: един клиент може да запазва и преизползва връзки, докато нов за всяко извикване губи това предимство и добавя ненужна работа. Споделеното заемане `&reqwest::Client` е достатъчно, защото заявките на клиента не изискват изключителна променлива референция.

### Докладът е стабилен интерфейс за хора и за програми

Една таблица служи за четене в терминал. JSON служи, за да може друга програма да консумира резултати, без да налучква колони, интервали или подравняване. Не е удачно да се използва таблицата като формат за интеграция: ако утре промениш ширината ѝ, скрипт, който я обработва с `awk`, може да се счупи, макар информацията да е същата. Не е удачно и да се отпечатват човешки диагностики в стандартния изход, когато е поискан JSON, защото той би престанал да е валиден JSON.

`revisor` използва `stdout` за доклада и `stderr` за грешките при стартиране. Тази конвенция позволява да се пренасочи само докладът към файл:

```bash
cargo run -- --archivo servicios.yaml --formato json > estado.json
```

Ако конфигурацията е валидна, `estado.json` съдържа единствено JSON. Ако нещо пречи на стартирането, съобщението се появява в терминала чрез `stderr`; не се смесва с формат, който друг процес очаква да анализира.

Функцията `json` тръгва от услуги и състояния, подрежда ги по име и строи колекция от `EstadoJson`. Подреждането преди сериализиране не е задължително, за да е JSON валиден, но помага резултатът да е стабилен. Стабилният резултат се сравнява по-добре в тестове, прегледи на промени и автоматизации.

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

`match`-овете принуждават да се реши какво представлява всеки вариант. `NoIntentado` няма продължителност нито реален код, така че се изразява като `ms: 0`, `codigo: null` и изрична грешка. `Falla` запазва изминалото си време, защото да знаеш, че една връзка е изчерпала лимита си при 5 000 ms, е полезна информация. `Ok` и `Lento` имат код; и двата са здрави за изходния код, макар докладът да ги различава.

Тази част разкрива полезна разлика с Go. В Go можеш да строиш struct-ове с тагове `json:"..."` само с `encoding/json`; за YAML обикновено добавяш друга библиотека със собствена конвенция за тагове. В Rust `serde` централизира дефиницията на сериализацията и позволява на адаптерите на формати да работят върху същия derive. В замяна трябва да включиш външни crate-ове и да разбереш кои части от договора са в атрибутите и кои във явното валидиране.

### Профилът release подготвя артефакт, не замества проверката

`cargo run` и `cargo test` използват по подразбиране профила за разработка. Този профил дава предимство на бързото компилиране по време на редактиране и запазва полезна информация за отстраняване на грешки. Разпространяем бинарен файл се построява с `cargo build --release`; тогава Cargo прилага профила `release`, дефиниран от проекта.

Профилът не превръща неправилна програма в правилна. Първо трябва да минат тестовете, форматът и Clippy. После профилът решава баланса между размер на бинарния файл, време за компилиране, оптимизация и налична диагностика, ако се случи паника. В този проект е избран размерът: махат се символите, оптимизира се за размер, включва се оптимизация между модулите, използва се една-единствена единица за генериране и паниките прекратяват процеса.

<!-- verificar:extracto:Cargo.toml -->
```toml
[profile.release]
strip = true              # quita símbolos
opt-level = "z"           # optimiza para tamaño
lto = true                # optimización entre módulos
codegen-units = 1
panic = "abort"           # sin desenrollado de pila
```

Всяка опция има цена. `lto = true` и `codegen-units = 1` могат да увеличат времето за компилиране, защото позволяват оптимизиране с повече глобална информация. `panic = "abort"` намалява бинарния файл и избягва размотаването на стека, но жертва възможността за почистване чрез размотаване и предлага по-малко контекст, ако една паника стигне до продукция. За този инструмент на командния ред е разумен избор; не е универсална рецепта за всяка библиотека.

Не заявявай, че Rust или Go „печелят“ по размера на бинарния файл, без да си измерил на своята машина и със същата функционалност. Бинарният файл на Rust може да нарасне, като включи Tokio, Reqwest, TLS и транзитивните им зависимости; този на Go също включва своя runtime и зависи от опциите си за компилиране. Валидното сравнение отбелязва платформа, архитектура, компилиране на студено или инкрементално, използван профил и какви зависимости включва всяка програма.

Кръстосаното компилиране оставя друга практическа разлика. Go обикновено позволява да се избере платформа с променливи като `GOOS` и `GOARCH`. Rust изисква да се инсталира съответният target и, според target-а, да разполагаш и с линкер и съвместими библиотеки. За статичен Linux, базиран на musl, първата стъпка би била:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Това не означава, че кръстосаното компилиране е невъзможно в Rust; означава, че трябва да моделираш target-а като част от средата за изграждане. Провери получения бинарен файл в системата, където ще се използва, особено ако включва TLS или ако сменяш между libc на Linux.

## Грешката, която ще видиш

Парсерът на аргументи трябва да превръща текст в типизирани стойности. Ако се опиташ да присвоиш директно текст на `usize`, Rust не налучква, че искаш да интерпретираш текста като число. Следващата програма се проваля, преди да се изпълни.

**Фиг. 8.4** | Един текст не е число, макар да представя опция на командния ред.

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

`E0308` означава, че два типа не съвпадат. Анотацията `: usize` установява какво трябва да произведе изразът отдясно, но `"cinco"` е `&str`. Поправката не е да махнеш типа, за да се компилира с низ: `paralelo` трябва да остане число, защото контролира лимит на конкурентността. Трябва да преобразуваш с `.parse::<usize>()` и да решиш какво да се прави, ако преобразуването се провали.

В минималната програма това решение се появява като `Result<Args, String>`. В реалния проект `clap` прави преобразуването на декларираните полета и показва диагностика за употреба, когато не може да построи очаквания тип. След това `main` все още валидира собствените правила на приложението: `formato` да е `tabla` или `json`, YAML да има услуги и стойностите на всяка услуга да имат смисъл.

Друга честа грешка е да се бърка неуспешен HTTP с грешка, която трябва да се разпространи чрез `?`. В тази програма един HTTP 500 се превръща в `Estado::Falla`, отпечатва се и дава изходен код 1. Разпространяването му като грешка при стартиране би направило една паднала услуга да скрие резултата на останалите. Операторът `?` си остава правилен за повреди, които пречат да се продължи, като невъзможността да се прочете YAML или да се сериализира JSON докладът.

## Какво се прави погрешно

### Поставяне на `unwrap()` по границите на програмата

`unwrap()` е приемлив в тест, когато подготвяш литерални данни, които ти контролираш. Не е за аргументи, YAML файлове, нито за мрежа. Лошо написан аргумент не бива да произвежда паника с backtrace; трябва да даде съобщение, което казва коя опция е невалидна, и да завърши с код 2. Липсващ файл не е програмна изненада: е условие за употреба, което бинарният файл трябва да съобщи с името на файла.

Полезното правило е да се попита кой контролира данната. Ако я контролира тестът, `expect` може да направи един провал ясен. Ако я контролира човек, файл, операционната система или мрежата, превърни я в контекстуална грешка или в състояние на домейна.

### Създаване на `reqwest::Client` за всяка услуга

Създаването на клиента вътре в `revisar` изглежда локално и просто, но губи преизползването на връзки и кара конфигурацията на клиента да се разпилява. Проектът създава един-единствен `reqwest::Client`, след като конфигурацията е валидирана, и го заема на всички запитвания. Така жизненият цикъл на клиента съвпада с жизнения цикъл на едно изпълнение на `revisor`.

Не бъркай споделянето на клиента със споделяне на променливо състояние без контрол. Клиентът се използва чрез споделени референции, а семафорът защитава ресурса, който наистина трябва да се ограничава: колко заявки са активни.

### Използване в продукция на JSON, построен с `format!`

Фигурата с JSON показва формата на данните, не техника за безопасна сериализация. Ако името на една услуга съдържа кавички, обратна наклонена черта или нов ред, низ, построен на ръка, може да престане да е валиден JSON или да промени значението си. `serde_json` екранира тези данни правилно и запазва връзката между `Option`, `null` и пропуснатите полета.

Добро правило е структурираните формати да се строят със сериализатор и да се тестват, като се четат обратно. В тестовете на проекта JSON се превръща в `serde_json::Value`, преди да се проверят конкретни полета.

### Третиране на изходния код като невидима подробност

Таблица с думата `FALLA` помага на човек, но една планирана задача трябва да знае дали да алармира. Ако `revisor` винаги завършва с 0, един cron, един pipeline или един супервайзор може да вярва, че всичко е здраво, макар да има паднали услуги. Ако завършва с 1 при всеки проблем, също не различава наблюдавано падане от лоша конфигурация.

Поддържай договора прост и документиран: 0 за здрава проверка, 1 за неуспешни услуги и 2 за грешки при стартиране или употреба. Тестовете на бинарния файл трябва да проверяват тези три пътя.

### Измерване на бинарния файл на Rust срещу този на Go без контрол на експеримента

Да сравняваш само размера на два файла е беден начин да се стигне до изводи. Резултатът се променя, ако единият включва TLS, ако другият използва външна HTTP библиотека, ако единият е компилиран за разработка, а другият за release, ако са премахнати символи или ако системата компресира изпълними файлове. Времето също се променя между студено компилиране, инкрементално и ново изграждане след промяна на един ред.

Мери със същия случай на употреба и записвай условията. Понякога Go ще е практичният избор, защото кръстосаното компилиране и стандартната библиотека намаляват стъпките. Понякога Rust ще е за предпочитане, защото искаш да изразиш определени гаранции, преди да се изпълни, и да приемеш по-скъпо компилиране. Целта на двата курса не е да повторят лозунг; е да имаш собствено доказателство.

## Упражнения

### Упражнение 1 — Декларирай допълнителна опция

Добави към независима програма опция `--silencioso`, която по подразбиране е невярна. Когато е вярна, програмата трябва да отпечатва единствено числовия код на всяко състояние; когато е невярна, трябва да отпечатва име, състояние и код. Накарай непознат флаг да произвежда `Err` с ясно съобщение.

После обясни какво би декларирал в `clap`: поле `bool`, опция с `#[arg(long)]` и поведението по подразбиране, което типът му вече изразява.

### Упражнение 2 — Раздели невалидните данни от неуспешните услуги

Напиши функция, която получава списък с минимални услуги с `nombre`, `url` и `timeout_ms`. Трябва да връща грешка, ако списъкът е празен, ако име се повтаря, ако URL не започва с `http://` или `https://` или ако времевото ограничение е нула. Отделно представи един HTTP 503 като вариант на състояние, не като грешка при валидиране.

Използвай правилата на `config::validar` като справка, но първо напиши своите тестови случаи: валиден списък, празен, един с дублирано име и един с timeout нула.

### Упражнение 3 — Тествай договора на бинарния файл

Без още да отваряш `tests/binario.rs`, напиши тест от край до край за невалидния формат `-f xml` (в копие на проекта или с друго име, ако работиш в оригинала). Трябва да провери, че изходният код е 2, че няма таблица в стандартния изход и че изходът за грешки назовава `--formato`.

После изпълни само този тест и след това целия набор. Не променяй кода на проекта, за да мине лошо написан тест: тестът трябва да описва договора, който `main.rs` вече излага. Накрая отвори `tests/binario.rs`: проектът вече носи тест за този договор и това е референтното решение; сравни какво проверява всеки.

### Упражнение 4 — Сравни двете имплементации

Построй в release `revisor` на Rust и еквивалентната програма на Go. В дневник запиши, за двете, собствени редове код, преки зависимости, време за студено компилиране, размер на бинарния файл, използване на памет при една и съща конфигурация и времето, което ти е отнело да завършиш програмата.

Отговори с едно обосновано изречение: коя би предал за вътрешен инструмент с близка дата, коя за инструмент, който трябва да се поддържа с години, и кое конкретно решение на всеки език те доведе до този извод.

## Решения

### Решение 1

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

Опцията не променя състоянието на домейна; променя представянето. Затова трябва да се прилага близо до границата на изхода, не вътре в `Estado` нито в HTTP логиката. С `clap` един `bool`, означен с `#[arg(long)]`, изразява, че липсата на флага е `false`, а присъствието му е `true`.

### Решение 2

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

Невалидният timeout е проблем на входа: не бива да се започва проверката. Един код 503, напротив, е информация, получена при проверката, и принадлежи на състоянието на услугата. Разделянето позволява на бинарния файл да завърши с 2 в първия случай и с 1 във втория.

### Решение 3

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

Тестът изпълнява реалния бинарен файл, затова проверява трите му наблюдавани изхода: код, стандартен изход и изход за грешки. Не е нужно да познава частни функции на `main.rs`; познава само договора, който ще види този, който изпълни `revisor -f xml`.

### Решение 4

Сравнението трябва да започне с възпроизводими команди:

```bash
cd programas/revisor
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

После повтори експеримента с програмата на Go, на същия компютър и със същата конфигурация на услугите. Не копирай чужди резултати, сякаш са универсални. Стойността на упражнението е да определиш каква част от времето е била компилиране, каква част е било учене на екосистемата и каква част е било решаване на един и същ проблем от дизайна.

## Как да разбера, че съм успял

- `cargo run -- --help` показва опциите `--archivo`, `--formato` и `--paralelo`.
- `cargo run -- --archivo archivo-que-no-existe.yaml` завършва с код 2 и записва името на файла в изхода за грешки.
- `cargo run -- --formato xml` завършва с код 2 и обяснява, че валидните формати са `tabla` и `json`.
- `cargo run --example ejemplo_clap`, `ejemplo_serde` и `ejemplo_reqwest` отпечатват същото, което документира този урок, а `herramientas/verificar-ejemplos.sh` завършва без грешки.
- `cargo test` завършва с правилни резултати за библиотеката, интеграцията и бинарния файл.
- `cargo clippy --all-targets -- -D warnings` завършва без предупреждения.
- `cargo fmt --check` завършва без чакащи промени.
- `cargo build --release` създава бинарния файл в `target/release/revisor`.
- Можеш да обясниш защо един HTTP 500 произвежда изходен код 1, докато невалиден YAML произвежда код 2.

## За по-нататъшно четене

- [The Rust Programming Language, глава 12: An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) — консултирано на 2 октомври 2026 г.
- [Официална референция на Cargo: профили](https://doc.rust-lang.org/cargo/reference/profiles.html) — консултирано на 2 октомври 2026 г.
- [Документация на `clap`](https://docs.rs/clap/latest/clap/) — консултирано на 2 октомври 2026 г.
- [Документация на `reqwest`](https://docs.rs/reqwest/latest/reqwest/) — консултирано на 2 октомври 2026 г.
