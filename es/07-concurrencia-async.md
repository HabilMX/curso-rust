# Lección 7 — Concurrencia y async

**Tiempo:** 2 × 45 min

**Qué construyes:** el `revisor` concurrente: que revise todo a la vez

**Qué aprendes:** hilos del sistema, `Arc` y `Mutex`, canales, `async/await` con `tokio`, la comparación honesta con las gorrutinas de Go

## Al terminar vas a poder

- Lanzar hilos del sistema con `thread::spawn`, transferirles la propiedad correcta y recoger su resultado con `join`.
- Explicar por qué un dato compartido entre hilos necesita `Arc`, cuándo necesita además un `Mutex`, y qué protege el `MutexGuard`.
- Usar un canal de la biblioteca estándar para entregar resultados sin compartir una colección mutable.
- Leer el error `E0277` cuando un valor no cumple `Send` y reconocer que el compilador está protegiendo una frontera entre hilos.
- Explicar la diferencia entre concurrencia y paralelismo, y entre un hilo del sistema, una tarea async y una gorrutina de Go.
- Seguir en el `revisor` el recorrido de una consulta async, desde `#[tokio::main]` hasta `join_all`, el semáforo y el resultado ordenado.

## El porqué antes del cómo

Hasta ahora, el `revisor` puede recibir una lista de servicios, consultar uno y clasificar la respuesta. Si consulta diez servicios en serie y cada uno tarda un segundo en responder, el reporte tarda aproximadamente diez segundos. No importa que la computadora tenga varios núcleos ni que el programa sea rápido: durante casi todo ese tiempo la CPU no está calculando nada; está esperando una respuesta de red.

Esperar una respuesta de red es distinto de calcular una suma grande. En un cálculo intenso, más núcleos pueden permitir trabajo paralelo de verdad. En una consulta HTTP, en cambio, la mayor parte del tiempo está fuera del proceso: el sistema operativo espera paquetes, el servidor remoto decide qué hacer y la red transporta la respuesta. Mientras tanto, el programa podría iniciar otras consultas. Eso es concurrencia: organizar varias tareas que avanzan en periodos intercalados. Puede convertirse en paralelismo si varias tareas ejecutan código al mismo tiempo en distintos núcleos, pero no son sinónimos.

El objetivo práctico de esta lección es cambiar el tiempo total. Si cinco servicios tardan cerca de un segundo cada uno y los consultas uno por uno, el reporte tarda alrededor de cinco segundos. Si inicias las cinco consultas y esperas sus respuestas a la vez, el tiempo se acerca al del servicio más lento, no a la suma de todos. Eso no hace que los servicios remotos respondan más rápido; evita desperdiciar el tiempo que el programa pasaba esperando al primero antes de empezar el segundo.

El costo es que varias partes del programa pueden estar vivas al mismo tiempo. Aparecen preguntas que el código secuencial no tenía: ¿quién es dueño del dato?, ¿cuándo termina una tarea?, ¿qué orden tienen los resultados?, ¿dos tareas pueden modificar el mismo valor?, ¿qué pasa si una falla?, ¿cómo impides abrir miles de conexiones simultáneas? Rust no responde estas preguntas escondiendo la memoria compartida. Hace que ownership, préstamos y traits como `Send` y `Sync` sigan importando cuando hay varios hilos o tareas.

Esto conecta directamente con la lección 2. Ownership parecía una regla local: un valor tiene un dueño y los préstamos deben respetar su vida útil. En concurrencia, esas reglas se vuelven una garantía entre tareas. Un hilo no puede conservar una referencia a una variable de `main` que quizá ya desapareció. Tampoco puede recibir un tipo que Rust sabe que no es seguro mover a otro hilo. Lo que al inicio se sentía como fricción del compilador se convierte aquí en una barrera contra referencias colgantes y carreras de datos en código seguro.

Rust ofrece dos herramientas principales para el problema. La biblioteca estándar trae hilos del sistema, mutexes, contadores de referencias atómicos y canales. Son apropiados para trabajo de CPU, programas pequeños o integración con APIs bloqueantes. Para muchas operaciones de red que pasan tiempo esperando, el `revisor` usa `async/await` y Tokio. Async no significa “más rápido por definición”: significa que una cantidad moderada de hilos puede avanzar muchas operaciones que esperan E/S sin reservar un hilo bloqueado para cada una.

Go toma otra decisión. Una gorrutina se inicia con `go f()` y el runtime viene integrado al lenguaje y a la distribución. Rust obliga a distinguir un hilo de una tarea async y, para async, a elegir un runtime. Eso exige más vocabulario y más decisiones, pero permite que el tipo de dato y la frontera de propiedad sean explícitos. Ninguno de los dos enfoques elimina la necesidad de diseñar límites, manejar errores y medir. La comparación útil no es cuál lenguaje “gana”, sino qué costo paga cada uno y qué garantía ofrece a cambio.

Lee primero los capítulos 16 y 17 de The Rust Book. Después haz los ejercicios de Rustlings de hilos y canales antes de adaptar el `revisor`. El orden importa: async se vuelve mucho menos misterioso cuando ya entiendes qué significa que una clausura se mueva a otro hilo, qué quiere decir `Send` y por qué compartir mutabilidad requiere una sincronización visible.

## Los conceptos

### Hilos del sistema, `move` y `join`

`std::thread::spawn` pide una clausura y arranca un hilo del sistema operativo para ejecutarla. El valor que devuelve es un `JoinHandle<T>`: una promesa concreta de que el hilo puede terminar con un valor de tipo `T`. Llamar `join()` espera a que termine ese hilo y devuelve `Result<T, Box<dyn Any + Send>>`; el `Err` representa que el hilo hizo `panic!`.

La palabra clave `move` es importante. Una clausura sin `move` puede intentar capturar una referencia a una variable del contexto exterior. Un hilo puede seguir ejecutándose después de que ese contexto terminó, así que Rust no permite entregar al hilo una referencia que quizá deje de ser válida. `move` hace que la clausura capture por valor. En la figura, cada `Servicio` deja de pertenecer al vector y pasa a pertenecer a la clausura de su propio hilo.

**Fig. 7.1** | Un hilo por servicio.

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

El orden de los `println!` es determinista aunque los hilos terminen en otro orden. Los `JoinHandle` se guardan en el mismo orden que produce `servicios.into_iter()`, y el segundo `for` llama `join()` en ese orden. Si el hilo de `pagos` termina primero, su valor queda listo, pero el programa primero espera e imprime el de `catalogo`. Esta diferencia importa: concurrencia no obliga a que la salida sea no determinista. Puedes diseñar una frontera donde el orden observable siga siendo estable.

`join` es suficiente cuando cada tarea tiene un resultado y el número de tareas es pequeño y conocido. No necesitas un canal solo para recuperar un valor; el handle ya lo entrega. En Go normalmente combinarías una gorrutina con un `WaitGroup` para esperar y un canal o una colección protegida para recuperar resultados. Rust vuelve el resultado parte del handle, aunque eso no elimina la utilidad de los canales para comunicación progresiva.

Un hilo del sistema no es gratis. Tiene recursos del sistema operativo, una pila y un costo de planificación mayor que una tarea async. No hay una cifra universal de memoria por hilo: depende del sistema operativo, la arquitectura y la configuración. La regla de diseño es más útil que un número fijo: no abras un hilo del sistema por cada conexión si el trabajo principal consiste en esperar red. Para unas cuantas tareas de CPU o una API bloqueante, un hilo puede ser justo lo correcto. Para muchos servicios HTTP, el `revisor` usa async.

El `revisor` no crea un hilo por servicio. Su trabajo de red se expresa como futuros async; el runtime decide qué hilos ejecutan esos futuros. Sin embargo, la misma idea de propiedad aparece: una tarea debe poseer lo que conserva durante su ejecución, o recibir préstamos que sigan siendo válidos hasta que termine. Por eso es importante entender primero `move`, aunque el código final use Tokio.

### `Arc`, `Mutex` y el dato que vive dentro del candado

Un `Rc<T>` permite que varios dueños dentro de un solo hilo compartan un valor. Su contador de referencias no es atómico, por lo que no puede compartirse entre hilos. `Arc<T>` significa *atomic reference counted*: hace la misma función general, pero actualiza el contador de referencias de forma segura entre hilos. Clonar un `Arc` no clona `T`; solo crea otro dueño de la misma asignación.

Tener varios dueños no equivale a tener permiso para modificar. Si `T` es mutable y varias tareas pueden acceder a ella, hace falta coordinación. `Mutex<T>` contiene el dato y permite que una sola tarea a la vez reciba acceso mutable mediante `lock()`. El resultado de `lock()` es un `MutexGuard<T>`. Mientras ese guard exista, el candado sigue tomado; al salir de su alcance, su `Drop` libera el candado. No necesitas escribir una llamada separada a `unlock`, y eso reduce el riesgo de olvidar liberar el recurso en un camino de retorno.

**Fig. 7.2** | Un dato compartido entre hilos.

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

La forma `Arc<Mutex<HashMap<_, _>>>` se lee desde dentro hacia fuera. El `HashMap` es el dato. El `Mutex` es la única puerta para mutarlo. El `Arc` permite que distintos hilos sean dueños de esa puerta. Para insertar, el hilo toma el candado y recibe el guard; para imprimir, `main` toma de nuevo el candado. Nunca aparece una referencia mutable al mapa sin que el guard exista.

En Go es frecuente declarar un `sync.Mutex` junto al mapa y seguir la convención de llamar `Lock` antes de tocarlo. Esa convención puede encapsularse bien, pero el lenguaje no obliga a que el mapa esté físicamente dentro del mutex. En Rust, al envolver el dato en `Mutex<T>`, la API normal no deja obtener `&mut T` sin un `MutexGuard`. Esto no hace imposibles todos los errores de concurrencia: aún puedes provocar un interbloqueo tomando candados en órdenes inconsistentes, o mantener un guard demasiado tiempo. Sí hace imposible en código seguro una carrera de datos causada por prestar simultáneamente el mismo valor mutable sin sincronización.

No uses `lock().unwrap()` como una fórmula que no se piensa. `lock()` puede devolver error si otro hilo hizo `panic!` mientras tenía el candado; eso se llama envenenamiento del mutex. En un ejemplo pedagógico, `unwrap()` hace visible ese caso con un `panic!`. En un servicio real debes decidir si ese estado invalida el programa, si puedes recuperar el dato con `into_inner`, o si conviene rediseñar para que el estado compartido no sea necesario.

El `revisor` evita un `Mutex<HashMap<...>>` porque no tiene que ir llenando un mapa compartido conforme llega cada respuesta. Cada futuro produce su propio `Estado`, y `join_all` los reúne. El recurso que sí comparte es un límite de turnos: varios futuros necesitan pedir permiso para iniciar una consulta, no modificar una colección común. Por eso el proyecto usa `Arc<Semaphore>` y no `Arc<Mutex<Vec<Estado>>>`.

### Canales: entregar valores en vez de compartir una colección

Un canal divide la comunicación en dos extremos: un emisor, `Sender<T>`, y un receptor, `Receiver<T>`. El emisor entrega valores con `send`; el receptor los toma con `recv` o iterando sobre él. En lugar de que varios hilos escriban en una estructura compartida, cada productor entrega un valor que pasa a ser propiedad del receptor. Esta arquitectura reduce la zona compartida y aclara quién arma el resultado final.

El canal creado con `std::sync::mpsc::channel()` es de múltiples productores y un consumidor. Es “múltiples” porque puedes clonar el emisor antes de mover una copia a cada hilo. El receptor no se clona: un solo lugar decide qué hacer con cada mensaje. Cuando desaparecen todos los emisores, iterar sobre el receptor termina. Esa clausura del canal es parte del protocolo, no un detalle incidental.

**Fig. 7.3** | Un canal entrega estados al hilo que imprime el reporte.

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

El programa espera el hilo antes de recorrer el receptor para conservar una salida determinista. En un programa que de verdad procesa resultados conforme llegan, normalmente empezarías a recibir mientras los productores siguen trabajando. Entonces el orden sería el de llegada, no necesariamente el de los servicios de entrada. Esa puede ser una buena decisión para una interfaz que informa progreso, pero no para un reporte que debe alinear cada estado con el servicio original.

Los canales no sustituyen automáticamente a los mutexes. Si varias tareas necesitan leer y actualizar la misma cuenta, quizá un `Mutex` sea el modelo natural. Si una tarea produce valores y otra decide cómo almacenarlos o mostrarlos, un canal suele representar mejor la responsabilidad. El error común es elegir por moda: “los mutexes son malos” o “los canales son complicados”. La pregunta útil es quién debe ser dueño de cada dato en cada momento.

En el `revisor`, `join_all` cumple una función parecida a recibir todos los resultados, pero con un contrato adicional: conserva el orden de los futuros de entrada. El proyecto no usa un canal para los estados porque el reporte necesita el mismo orden que la lista YAML de servicios. Si el producto necesitara imprimir “terminó pagos” apenas llegue la respuesta, un canal o un stream sería una opción razonable. No lo agregues solo porque existe; la elección actual es intencional y mantiene el reporte reproducible.

### `Send`, `Sync` y el error que el compilador detiene

`Send` y `Sync` son traits de marcado. No suelen requerir métodos propios; describen propiedades de seguridad que Rust puede derivar de los campos de un tipo. Un tipo `Send` puede transferirse por valor a otro hilo. Un tipo `Sync` puede compartirse por referencia entre hilos: si `T` es `Sync`, entonces `&T` es `Send`. Muchas estructuras comunes lo implementan automáticamente cuando sus componentes también son seguros, pero `Rc<T>` no es `Send` ni `Sync` porque su contador no puede actualizarse desde varios hilos.

Esta regla no es una lista para memorizar. Es una pregunta que Rust contesta por composición. Si haces un struct que contiene `Rc<RefCell<_>>`, hereda las restricciones de esas piezas. Si cambias a `Arc<Mutex<_>>`, cambias la representación y también las garantías disponibles. El compilador sigue el valor hasta la clausura que se manda a `thread::spawn` y exige que la frontera sea segura.

En Go, una carrera de datos puede compilar y requerir `go test -race` para detectarse durante una ejecución que alcance justo la intercalación problemática. El detector es valioso y debes usarlo, pero depende de que la prueba ejecute el camino conflictivo. Rust evita las carreras de datos en código seguro antes de correr el programa. Eso no prueba que la lógica sea correcta ni detecta automáticamente interbloqueos, starvation o protocolos mal diseñados. También existe `unsafe`, donde el programador asume responsabilidades adicionales. La afirmación precisa es: Rust evita carreras de datos mediante sus reglas de tipos y préstamos en código seguro; no promete que todo programa concurrente sea correcto.

Dentro del `revisor`, `Arc<Semaphore>` es válido porque el semáforo de Tokio está diseñado para compartirse entre tareas. Cada futuro recibe su propio `Arc`, pide un permiso y conserva ese permiso durante la solicitud. El tipo del permiso y su ciclo de vida expresan que el turno no puede devolverse antes de terminar la consulta. No hay un contador `usize` compartido que cada futuro incremente y decremente manualmente.

### `async`, futuros y el runtime de Tokio

Una función marcada `async fn` no ejecuta inmediatamente todo su cuerpo al llamarla. Produce un futuro: un valor que representa trabajo pendiente. Ese futuro avanza cuando un ejecutor lo sondea. Si llega a una operación que todavía no está lista, como esperar una respuesta de red, devuelve control al ejecutor. Más tarde, cuando la operación pueda continuar, el ejecutor lo vuelve a sondear.

`.await` es el punto donde una función async espera el resultado de otro futuro. No crea por sí solo una tarea nueva ni un hilo nuevo. Esta distinción corrige dos malentendidos frecuentes. Primero: escribir `let futuro = revisar(...);` no inicia necesariamente la solicitud; solo construye el futuro. Segundo: llamar `.await` una tras otra vez en el mismo bloque puede volver seriales las operaciones. Para iniciar varias operaciones de manera concurrente, construyes varios futuros y los conduces juntos con una combinadora como `join_all`, o los conviertes en tareas con `tokio::spawn` cuando de verdad necesitas independencia.

Rust define la sintaxis y los traits de async, pero no incluye un ejecutor async completo en la biblioteca estándar. Tokio es el runtime elegido por este proyecto. Proporciona ejecutor, temporizadores, sincronización async y adaptaciones de E/S. Algunas bibliotecas async son independientes del runtime, pero los recursos de Tokio, como sus temporizadores y varios tipos de sincronización, requieren ejecutarse dentro de un contexto Tokio. Lee la documentación de cada crate antes de asumir que cualquier futuro funciona igual con cualquier runtime.

<!-- verificar:extracto:src/main.rs -->
```rust
#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    ejecutar(&args).await
}

async fn ejecutar(args: &Args) -> ExitCode {
```

El atributo `#[tokio::main]` construye el runtime y ejecuta la función `main` async. El `main` del programa sigue devolviendo un `ExitCode`, como aprendiste en la lección 6; lo que cambia es que ahora puede esperar operaciones async antes de decidir el código de salida. El binario conserva la responsabilidad de parsear argumentos, imprimir y salir; la biblioteca conserva la lógica de consultar servicios.

No bloquees un hilo del runtime con `std::thread::sleep`, lectura de archivos pesada o cálculo largo dentro de una función async. Un hilo bloqueado no puede sondear otros futuros asignados a él. Para trabajo bloqueante existe `tokio::task::spawn_blocking`; para E/S de red, usa APIs async como `reqwest`. El `revisor` usa `reqwest::Client` y espera su `send().await`, por lo que mientras una respuesta está pendiente el runtime puede avanzar consultas de otros servicios.

### `join_all`, semáforos y el límite de paralelo del `revisor`

Lanzar todas las solicitudes posibles al mismo tiempo no siempre es una mejora. Un archivo con miles de servicios podría abrir demasiadas conexiones, saturar la red local, agotar descriptores de archivo o cargar al servidor que justamente intentas revisar. La concurrencia necesita un límite. El argumento `--paralelo` del `revisor` expresa cuántas consultas pueden estar activas a la vez.

Un semáforo contiene permisos. Para empezar una consulta, un futuro adquiere uno; si no quedan, espera. Cuando el permiso sale de alcance, se libera automáticamente y otro futuro puede continuar. Es la misma idea de RAII que viste con `MutexGuard`: el recurso se libera al destruir el guard, incluso si la función sale por un camino normal. Aquí el recurso no es un candado exclusivo sino una capacidad limitada.

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

`servicios.iter()` conserva préstamos a la lista; no consume los servicios. Cada clausura async posee su clon de `Arc<Semaphore>`, pero toma prestados `cliente` y `s` durante la llamada a `revisar`. Esto funciona porque `join_all(futuros).await` termina antes de que `revisar_todos` pueda regresar, así que esos préstamos siguen vivos. Si usaras `tokio::spawn`, la tarea podría sobrevivir a la función que la creó y normalmente necesitaría datos con vida `'static`; ahí tendrías que mover o clonar más datos.

`join_all` devuelve un `Vec<Estado>` en el orden de los futuros de entrada, no en el orden en que las solicitudes terminan. Es una decisión útil para el reporte: el estado cero corresponde al servicio cero. El semáforo limita cuándo cada futuro puede entrar a `revisar`, pero no cambia esa relación final. Así, el programa tiene concurrencia real sin convertir el reporte en una fuente de orden aleatorio.

La función `revisar` no devuelve `Result<Estado>`. Esa decisión merece atención. Un HTTP 500, un timeout o una conexión rechazada no es un error interno que impida al programa continuar: es justo la información que el `revisor` debe reportar sobre un servicio. Por eso esos casos se convierten en variantes `Estado::Falla`. El error de `acquire` se trata distinto porque el semáforo nunca se cierra en este diseño; si ocurriera, sería una violación de un supuesto interno.

El timeout del proyecto se configura sobre la solicitud de `reqwest`, con el `timeout_ms` de cada `Servicio`. No confundas ese límite con el semáforo. El timeout limita cuánto puede esperar una consulta individual; el semáforo limita cuántas consultas pueden estar esperando o comunicándose al mismo tiempo. Necesitas ambos: sin timeout, un turno puede quedarse ocupado demasiado tiempo; sin semáforo, muchas solicitudes con timeout pueden arrancar todas juntas y sobrecargar recursos.

### Comparación honesta con Go

Go hace muy fácil arrancar una unidad concurrente: `go revisar(s)`. El runtime programa gorrutinas sobre hilos del sistema, crece sus pilas y administra el trabajo de red. Rust separa explícitamente la decisión: `thread::spawn` crea un hilo del sistema; un runtime como Tokio ejecuta futuros async; `tokio::spawn` crea una tarea Tokio. Esto significa que Go suele tener menos ceremonia al inicio y Rust obliga a saber cuál de las tres abstracciones estás usando.

Rust no tiene una ventaja mágica de rendimiento por escribir `async`. Una tarea async no acelera una consulta HTTP individual; mejora la utilización de hilos mientras varias consultas esperan. Para una carga de CPU, async puede ser peor si bloquea el ejecutor. En ese caso usa hilos, un pool de trabajo o `spawn_blocking`. Go tampoco convierte automáticamente un cálculo CPU-intensivo en más rápido: varias gorrutinas pueden competir por los mismos núcleos. Medir el tipo de trabajo importa más que aplicar una palabra de moda.

La garantía central de Rust aparece antes de ejecutar: un dato mutable no puede prestarse de forma incompatible, y los valores enviados a otro hilo deben cumplir los traits adecuados. Go privilegia una sintaxis pequeña y herramientas de ejecución como el detector de carreras. Go puede encapsular correctamente mutexes y canales; Rust puede tener deadlocks y errores lógicos. La diferencia no es “Go permite errores y Rust no”. Es dónde pone cada lenguaje la carga de comprobación y qué errores puede rechazar antes de correr.

Para el `revisor`, la decisión queda justificada por el problema. Hay muchas esperas HTTP, los resultados deben conservar el orden de entrada y se necesita un tope configurable de solicitudes. Tokio, `join_all` y `Semaphore` expresan esas tres necesidades. Un diseño con un hilo por servicio funcionaría para una lista pequeña, pero escalaría peor y no aporta una ventaja al caso de uso. Un diseño con un mutex y un mapa compartido también podría funcionar, pero haría más compleja una relación que `join_all` ya conserva.

## El error que vas a ver

El error más instructivo de esta lección es `E0277`. No significa que “Rust no quiere usar hilos”; significa que el tipo que intentas mover no satisface el contrato que `thread::spawn` requiere. `Rc<i32>` es útil para compartir propiedad dentro de un hilo, pero su contador de referencias no es atómico. Moverlo a una clausura que puede ejecutarse en otro hilo sería inseguro.

**Fig. 7.4** | `Rc<T>` no puede enviarse a un hilo.

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

El mensaje contiene la respuesta. La clausura captura `Rc<i32>`, `thread::spawn` exige que lo capturado sea `Send`, y `Rc<i32>` no implementa `Send`. No arregles este error añadiendo traits manualmente con `unsafe impl Send`; estarías prometiendo al compilador una seguridad que `Rc` no ofrece. Si varios hilos solo deben leer un dato, usa `Arc<T>`. Si además deben modificarlo, evalúa `Arc<Mutex<T>>` o rediseña el flujo para enviar valores por canales.

También reconoce el error de diseño que no produce `E0277`: mantener un `MutexGuard` durante una operación `.await`. Un guard de `std::sync::Mutex` bloquea un hilo; un guard de un mutex async conserva el candado mientras la tarea puede ceder el ejecutor. En ambos casos, esperar red mientras tienes el candado suele bloquear trabajo innecesariamente y puede producir interbloqueos. Extrae o actualiza el dato bajo el candado, suelta el guard y solo entonces espera.

## Lo que se hace mal

- Crear un hilo por cada servicio sin límite. Funciona con tres ejemplos y falla como estrategia cuando la lista crece. Los hilos del sistema consumen recursos del sistema operativo y una cantidad grande de ellos vuelve más difícil planear, medir y depurar. Para E/S de red, usa async con un límite de paralelo; para CPU, usa una cantidad de hilos proporcional al trabajo y a los núcleos disponibles.

- Usar `Arc<Mutex<_>>` como respuesta automática a cualquier error de ownership. Esta combinación es correcta cuando hay estado mutable genuinamente compartido, pero puede esconder un diseño donde varias tareas hacen demasiado. Si cada tarea puede devolver un valor y una sola parte lo junta, un canal, `join_all` o una reducción posterior suele ser más claro y reduce contención.

- Mantener un candado durante una solicitud de red o un `.await`. El guard existe para proteger una sección crítica pequeña. Si lo conservas mientras esperas, conviertes tareas concurrentes en una fila y aumentas la posibilidad de interbloqueo. Limita el alcance con llaves o con una variable temporal para que el guard se destruya antes de la espera.

- Llamar funciones bloqueantes dentro de código Tokio. `std::thread::sleep` bloquea el hilo, no solo una tarea. Una lectura grande, una consulta bloqueante o cálculo pesado tiene el mismo problema. Usa APIs async para E/S, `tokio::time` para temporizadores o `spawn_blocking` para trabajo que de verdad debe bloquear.

- Confundir `async` con paralelismo. Un futuro puede avanzar concurrentemente con otros y aun así ejecutarse sobre un solo hilo. Si necesitas acelerar cálculo de CPU, debes decidir cómo repartirlo entre núcleos. Si esperas red, async mejora la utilización de los hilos existentes. Antes de optimizar, mide qué está esperando el programa.

- Usar `tokio::spawn` solo para “hacerlo concurrente”. En el `revisor`, `join_all` puede conducir futuros que prestan `cliente` y `servicios`, conserva el orden y evita exigir propiedad `'static`. `tokio::spawn` es útil para tareas independientes que deben vivir más allá del bloque actual, pero implica otro contrato de vida y de tipos.

- Tratar la salida que llega primero como si fuera el orden correcto del reporte. En una interfaz de progreso puede ser útil informar por llegada. En un reporte que se compara contra el YAML de entrada, cambia la relación entre servicio y resultado. El `revisor` elige conservar el orden con `join_all`, y sus pruebas de integración comprueban esa propiedad.

## Ejercicios

### Ejercicio 1 — Tres revisiones con `join`

Escribe un programa con tres `Servicio` de texto. Usa `thread::spawn(move || ...)` para producir un estado por cada uno, conserva los handles en un vector y usa `join` para imprimir los resultados en el mismo orden de entrada. No uses `sleep` para “dar tiempo” a los hilos.

### Ejercicio 2 — Contador protegido

Crea un `Arc<Mutex<u32>>` con valor inicial cero. Lanza cuatro hilos; cada uno debe incrementar el contador una vez. Espera todos los handles e imprime `total: 4`. Después cambia a propósito `Arc` por `Rc` y confirma que aparece `E0277`.

### Ejercicio 3 — Resultados por canal

Crea un canal y tres hilos productores. Cada productor debe mandar el nombre de un servicio y un estado. El hilo principal debe recibir exactamente tres mensajes y ordenarlos antes de imprimirlos por nombre. Explica en un comentario por qué no debes depender del orden de llegada.

### Ejercicio 4 — Explica el límite del `revisor`

Lee `programas/revisor/src/revisar.rs`. Ejecuta las pruebas de integración y localiza la prueba `el_tope_de_paralelo_se_respeta`. En tu bitácora responde: qué recurso controla el `Semaphore`, cuándo se adquiere el permiso, cuándo se libera y por qué `join_all` conserva el orden de `servicios`.

## Soluciones

### Solución 1

La solución correcta mueve cada `Servicio` al hilo y recoge los handles después. El punto decisivo no es el `map`, sino que el vector de handles mantiene la obligación de esperar cada trabajo antes de terminar `main`. Si imprimes después de cada `join`, el orden observable es el orden del vector de handles.

Una forma de comprobar que no dependiste de `sleep` es ejecutar el programa varias veces: debe imprimir las tres líneas siempre. El planificador puede cambiar qué hilo termina primero, pero no puede evitar que `join` espere.

### Solución 2

Cada hilo debe recibir su propio `Arc::clone(&contador)`. Dentro del hilo, toma el guard, incrementa y deja que el guard salga de alcance. Después de esperar los cuatro handles, toma un último guard para imprimir el valor. No intentes mantener un préstamo mutable del contador fuera del mutex; ese préstamo no puede coexistir con los demás hilos.

Al cambiar `Arc` por `Rc`, el resultado esperado no es una salida numérica sino `E0277`. La corrección no consiste en “silenciar” el compilador: `Rc` sirve para referencias compartidas de un solo hilo; `Arc` es el tipo adecuado para que el contador tenga dueños en varios hilos.

### Solución 3

Cada productor recibe un clon del emisor y manda una estructura o tupla con el nombre y el estado. El emisor original debe dejar de existir antes de recorrer todo el receptor, o puedes llamar `recv` exactamente tres veces porque conoces el número de productores. Guarda los mensajes recibidos en un vector y ordénalo por nombre antes de imprimir.

La parte importante es que el receptor es el único dueño del vector final. Los productores no tienen acceso mutable a él. Por eso no necesitas un mutex para reunir los resultados; la propiedad viaja por el canal junto con cada mensaje.

### Solución 4

El semáforo controla el número de consultas HTTP que pueden estar activas a la vez, no el número total de servicios. Cada futuro adquiere un permiso justo antes de llamar `revisar(cliente, s).await`. El permiso vive en `_turno`; cuando termina esa llamada, `_turno` sale de alcance y devuelve la capacidad al semáforo.

`join_all` recibe futuros construidos al recorrer `servicios` y produce el vector de estados en esa misma secuencia. Por eso los resultados pueden terminar en tiempos distintos sin desalinearse del servicio que los originó. La prueba mide tiempos para confirmar que el límite cambia el comportamiento y no es solo una bandera decorativa.

## Cómo sé que lo logré

Ejecuta las figuras de esta lección desde sus directorios y compara la salida exacta:

```bash
cd programas/07-concurrencia-async
rustc --edition 2024 fig07_01.rs && ./fig07_01
rustc --edition 2024 fig07_02.rs && ./fig07_02
rustc --edition 2024 fig07_03.rs && ./fig07_03
rustc --edition 2024 fig07_04.rs
```

Las primeras tres compilaciones deben terminar sin avisos y producir las salidas documentadas. La última debe fallar con `E0277`; ese fallo es el resultado correcto del ejercicio.

Comprueba que los bloques y sus salidas siguen siendo verificables desde la raíz del curso:

```bash
herramientas/verificar-programas.sh es
herramientas/verificar-extractos.sh
```

Ambos comandos deben terminar correctamente. El primero confirma que cada figura compila, corre y coincide con su salida documentada, salvo la figura diseñada para fallar. El segundo confirma que los extractos del `revisor` siguen siendo copias exactas del proyecto real.

Finalmente, verifica el comportamiento concurrente real del proyecto:

```bash
cd programas/revisor
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

`cargo test` debe informar resultados exitosos, incluida la prueba `el_tope_de_paralelo_se_respeta`. `cargo clippy --all-targets -- -D warnings` debe terminar sin warnings, y `cargo fmt --check` no debe proponer cambios. Si puedes explicar por qué el semáforo limita consultas, por qué `join_all` conserva orden y por qué `Rc` provoca `E0277`, terminaste la lección.

## Para leer más

- [The Rust Programming Language, capítulo 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — consulta: 2 de octubre de 2026.

- [The Rust Programming Language, capítulo 17: Fundamentals of Asynchronous Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html) — consulta: 2 de octubre de 2026.

- [Documentación de `std::thread`](https://doc.rust-lang.org/std/thread/) — consulta: 2 de octubre de 2026.

- [Documentación de `tokio::sync::Semaphore`](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html) — consulta: 2 de octubre de 2026.
