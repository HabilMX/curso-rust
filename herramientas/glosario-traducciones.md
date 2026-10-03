# Glosario de traducciones del curso de Rust (en · fr · pt-BR · bg)

Fuente única para quien traduce. Si un término está en la tabla, se usa tal cual. Si falta, se agrega aquí **antes** de usarlo (ver «Término nuevo»).

El español (`es/`) es la fuente. La terminología sale de la documentación oficial de Rust en cada idioma donde existe; donde no existe una versión oficial vigente, se usa la traducción comunitaria más extendida y se dice.

## 1. Registro: el curso tutea, en masculino genérico

| Idioma | Forma | Regla concreta |
|---|---|---|
| **es** (fuente) | tuteo | «vas a poder», «tu computadora». |
| **en** | «you / your» | Directo, sin calcos del español («you will», no «one will»). |
| **fr** | **«tu / ton / ta / tes»** | Tuteo siempre; nunca «vous». Imperativo en «tu» («installe», «lance»). Nunca «on» dirigido al lector. |
| **pt-BR** | **«você / seu / sua»** | Permitido aquí porque es tuteo. Imperativo de «você» («instale», «execute»), no de «tu». |
| **bg** | **«ти / твой / твоя / твоето»** | Singular informal, en minúscula; imperativo en singular («инсталирай», «стартирай»). Nunca «Вие/Ви». |

Reglas comunes: sin emojis ni menciones de asistentes automáticos; títulos «Lesson N — …», «Leçon N — …», «Lição N — …», «Урок N — …» (con raya larga, el número de su archivo); sin calcos del español; sin recortar nada: toda la profundidad.

## 2. Lo que NO se traduce

- **Todo bloque ``` va BYTE A BYTE idéntico al del es**: código, comentarios, salidas de programas, errores de `rustc` y de `cargo`, líneas `<!-- verificar:… -->` (esas viven fuera del bloque pero se copian idénticas), el `// figNN_NN.rs` de la primera línea. Los verificadores comparan contra la ejecución real.
- Código en línea (`` `así` ``): idéntico. Palabras clave, tipos, macros, nombres de crates y de comandos.
- Nombres propios: Rust, Cargo, rustup, Rustlings, The Rust Programming Language («The Book»), Linux Mint, Go, Tokio, Serde, Clap, Reqwest, `revisor` (el programa del curso: sigue llamándose `revisor` en el código; en la prosa se escribe `revisor` en código).
- Los nombres de tipos del dominio que salen en código (`Servicio`, `Estado`…) se quedan; en la prosa se mencionan con el nombre del código.

## 3. Prohibido en cualquier idioma (la puerta pública)

El repositorio es público y `verificar-publicable.sh` revisa todos los idiomas con los patrones de `.publicable-prohibido.txt` (léelo antes de traducir). En la prosa traducida no puede aparecer nada de lo que esa lista veta: hosts o repos internos, fechas internas, rutas de máquinas, nombres de personas internos, ni una sola mención de asistentes automáticos, modelos o sus empresas, en ningún idioma ni con siglas locales. El español ya está limpio: no introduzcas nada al traducir.

## 4. Enlaces

- **The Book y la doc oficial**: se cambian a la traducción oficial del idioma **solo si existe, está vigente y la sección existe** (comprobar con `curl -sI` que responde 200 y que el capítulo es el mismo). Si no, se queda el enlace en inglés.
  - en: sin cambio (`doc.rust-lang.org/book/`).
  - fr, pt-BR, bg: no hay traducción oficial publicada en `doc.rust-lang.org`; las comunitarias (`jimskapt.github.io/rust-book-fr/`, `rust-br.github.io/rust-book-pt-br/`) cambian de numeración y quedan atrás de versión. Regla: **se queda el enlace al original** salvo que quien traduce demuestre las tres condiciones de arriba y lo avise en el checkpoint.
- Rustlings y la documentación de `std`: sin cambio.

## 5. Términos

Leyenda: «=» el término no se traduce. Entre paréntesis, la forma de la primera mención en prosa cuando conviene recordar el original: **la primera vez que aparezca en cada lección** se escribe «traducción (original)» donde el original no sea el propio término; después, solo la traducción.

### 5.1 Los conceptos centrales

| es (el curso) | en | fr | pt-BR | bg |
|---|---|---|---|---|
| ownership / propiedad | ownership | possession (ownership) | ownership (propriedade) | собственост (ownership) |
| dueño (de un valor) | owner | propriétaire | dono | собственик |
| mover / movimiento | move | déplacer / déplacement (move) | mover / movimento (move) | преместване (move) |
| copiar (`Copy`) | copy | copier | copiar | копиране |
| clonar (`clone()`) | clone | cloner | clonar | клониране |
| préstamo / prestar | borrow / borrowing | emprunt / emprunter | empréstimo / emprestar | заемане (borrow) / да заемеш |
| referencia | reference | référence | referência | референция |
| referencia mutable / inmutable | mutable / immutable reference | référence mutable / immuable | referência mutável / imutável | променлива / непроменлива референция |
| lifetime | lifetime | durée de vie (lifetime) | tempo de vida (lifetime) | време на живот (lifetime) |
| ámbito (scope) | scope | portée | escopo | област на видимост (scope) |
| liberación / liberar (drop) | drop / free | libération / libérer (drop) | liberação / liberar (drop) | освобождаване (drop) |
| recolector de basura | garbage collector | ramasse-miettes | coletor de lixo | събирач на боклук (garbage collector) |
| pila (stack) | stack | pile (stack) | pilha (stack) | стек |
| heap / montón | heap | tas (heap) | heap | хийп (heap) |
| puntero | pointer | pointeur | ponteiro | указател |
| slice | slice | tranche (slice) | slice | slice |
| sombreado (shadowing) | shadowing | masquage (shadowing) | sombreamento (shadowing) | засенчване (shadowing) |
| trait | trait | trait | trait | trait (черта) |
| método por omisión | default method | méthode par défaut | método padrão | метод по подразбиране |
| genérico / genéricos | generic / generics | générique / génériques | genérico / genéricos | generic типове (генерични) |
| restricción de trait (trait bound) | trait bound | contrainte de trait (trait bound) | restrição de trait (trait bound) | ограничение на trait (trait bound) |
| enum | enum | énumération (enum) | enum | enum (изброим тип) |
| variante | variant | variante | variante | вариант |
| struct | struct | structure (struct) | struct | struct (структура) |
| tupla | tuple | n-uplet (tuple) | tupla | кортеж (tuple) |
| coincidencia de patrones / `match` | pattern matching / `match` | filtrage par motif / `match` | correspondência de padrões / `match` | съпоставяне на шаблони / `match` |
| exhaustivo | exhaustive | exhaustif | exaustivo | изчерпателен |
| crate | crate | crate | crate | crate |
| módulo | module | module | módulo | модул |
| visibilidad / público / privado | visibility / public / private | visibilité / public / privé | visibilidade / público / privado | видимост / публичен / частен |
| ruta (de un módulo) | path | chemin | caminho | път |
| prueba unitaria / de integración | unit test / integration test | test unitaire / test d'intégration | teste unitário / teste de integração | unit тест / интеграционен тест |
| closure | closure | fermeture (closure) | closure | closure (затваряне) |
| iterador | iterator | itérateur | iterador | итератор |

### 5.2 Errores, tipos y manejo de errores

| es | en | fr | pt-BR | bg |
|---|---|---|---|---|
| `Option` / `Some` / `None` | = | = | = | = |
| `Result` / `Ok` / `Err` | = | = | = | = |
| operador `?` | the `?` operator | l'opérateur `?` | o operador `?` | операторът `?` |
| `panic!` / entrar en pánico | panic | panique / paniquer | pânico / entrar em pânico (panic) | паника |
| error de compilación | compile error | erreur de compilation | erro de compilação | грешка при компилация |
| mensaje del compilador | compiler message | message du compilateur | mensagem do compilador | съобщение от компилатора |
| aviso (warning) | warning | avertissement | aviso (warning) | предупреждение |
| antipatrón / «lo que se hace mal» | antipattern / what goes wrong | antipattern / ce qui se fait mal | antipadrão / o que se faz errado | антишаблон / какво се прави погрешно |
| tipo escalar / compuesto | scalar / compound type | type scalaire / composé | tipo escalar / composto | скаларен / съставен тип |
| expresión / sentencia | expression / statement | expression / instruction | expressão / instrução | израз / оператор |
| constante | constant | constante | constante | константа |
| inmutable por omisión | immutable by default | immuable par défaut | imutável por padrão | непроменлив по подразбиране |
| valor de retorno | return value | valeur de retour | valor de retorno | върната стойност |

### 5.3 Concurrencia y async

| es | en | fr | pt-BR | bg |
|---|---|---|---|---|
| concurrencia | concurrency | concurrence | concorrência | конкурентност |
| paralelismo | parallelism | parallélisme | paralelismo | паралелизъм |
| hilo (del sistema) | thread (OS thread) | thread / fil d'exécution (thread) | thread | нишка (thread) |
| carrera de datos | data race | course aux données (data race) | corrida de dados (data race) | състезание за данни (data race) |
| `Arc` / `Mutex` | = | = | = | = |
| exclusión mutua | mutual exclusion | exclusion mutuelle | exclusão mútua | взаимно изключване |
| canal | channel | canal | canal | канал |
| async / `async`/`await` | = | = | = | = |
| future / futuro | future | future | future | future |
| tarea | task | tâche | tarefa | задача |
| runtime (de async) | runtime | runtime | runtime | runtime |
| ejecutor | executor | exécuteur (executor) | executor | изпълнител (executor) |
| bloqueo (de un hilo) | blocking | blocage | bloqueio | блокиране |
| gorrutina (de Go) | goroutine | goroutine | goroutine | goroutine |
| compartir estado | share state | partager l'état | compartilhar estado | споделяне на състояние |

### 5.4 Herramientas y estructura del curso

| es | en | fr | pt-BR | bg |
|---|---|---|---|---|
| Lección N | Lesson N | Leçon N | Lição N | Урок N |
| Al terminar vas a poder | By the end you will be able to | À la fin, tu vas pouvoir | Ao terminar, você vai poder | След края ще можеш да |
| El porqué antes del cómo | The why before the how | Le pourquoi avant le comment | O porquê antes do como | Защо, преди как |
| Los conceptos | The concepts | Les concepts | Os conceitos | Понятията |
| El error que vas a ver | The error you will see | L'erreur que tu vas voir | O erro que você vai ver | Грешката, която ще видиш |
| Lo que se hace mal | What goes wrong | Ce qui se fait mal | O que se faz errado | Какво се прави погрешно |
| Ejercicios | Exercises | Exercices | Exercícios | Упражнения |
| Soluciones | Solutions | Solutions | Soluções | Решения |
| Cómo sé que lo logré | How do I know I got it | Comment savoir que j'y suis arrivé | Como sei que consegui | Как да разбера, че съм успял |
| Para leer más | Further reading | Pour aller plus loin | Para ler mais | За по-нататъшно четене |
| Al terminar / objetivo | goal / objective | objectif | objetivo | цел |
| Cargo / `cargo build` | = | = | = | = |
| compilador | compiler | compilateur | compilador | компилатор |
| entorno (de desarrollo) | environment | environnement | ambiente | среда |
| dependencia | dependency | dépendance | dependência | зависимост |
| perfil de release | release profile | profil `release` | perfil de release | профил release |
| binario | binary | binaire | binário | бинарен файл |
| extracto (del proyecto) | excerpt | extrait | trecho | откъс |
| fragmento | fragment | fragment | fragmento | фрагмент |
| toolchain | toolchain |  | toolchain | toolchain (набор от инструменти) |
| manifiesto (`Cargo.toml`) | manifest |  | manifesto | манифест |
| edición (2024) | edition |  | edição | издание (edition) |
| macro | macro |  | macro | макрос |
| linter | linter |  | linter | linter |
| lockfile (`Cargo.lock`) | lockfile |  | lockfile (`Cargo.lock`) | lockfile |
| enlazar / enlazador (linker) | link / linker |  | ligar / ligador (linker) | свързване / линкер (linker) |
| shell (intérprete de comandos) | shell |  | shell | shell |
| canal estable | stable channel |  | canal estável | стабилен канал |
| diagnóstico (del compilador) | diagnostic |  | diagnóstico | диагностично съобщение |
| artefacto (de compilación) | build artifact |  | artefato (de compilação) | артефакт |
| orden / comando | command |  | comando | команда |
| ejecutable (archivo) | executable |  | executável | изпълним файл |
| código fuente | source code |  | código-fonte | изходен код |
| biblioteca (estándar) | library / standard library |  | biblioteca / biblioteca padrão | библиотека (стандартна библиотека) |
| toolchain | toolchain | chaîne d'outils (toolchain) | toolchain | — |
| edición (de Rust) | edition | édition | edição | — |
| manifiesto (`Cargo.toml`) | manifest | manifeste | manifesto | — |
| biblioteca (library) | library | bibliothèque | biblioteca | — |
| artefacto (de compilación) | build artifact | artefact | artefato (de compilação) | — |
| perfil de desarrollo | development profile | profil de développement | perfil de desenvolvimento | — |
| lockfile (`Cargo.lock`) | lockfile | fichier de verrouillage (`Cargo.lock`) | lockfile (`Cargo.lock`) | — |
| enlace (binding de un nombre a un valor) |  |  | associação (binding) | обвързване (binding) |
| literal |  |  | literal | литерал |
| rango (`0..10`) |  |  | intervalo (`0..10`) | диапазон |
| bucle / ciclo |  |  | laço | цикъл |
| arreglo (array) |  |  | array | масив (array) |
| vector (`Vec<T>`) |  |  | vetor (`Vec<T>`) | вектор |
| cadena (de texto) |  |  | string | низ |
| desestructuración |  |  | desestruturação | деструктуриране |
| tipo unidad (`()`) |  |  | tipo unidade (`()`) | unit тип |
| acumulador |  |  | acumulador | акумулатор |
| conversión implícita / explícita |  |  | conversão implícita / explícita | неявно / явно преобразуване |
| inferencia de tipos |  |  | inferência de tipos | извеждане на типове |
| firma (de una función) |  |  | assinatura (de uma função) | сигнатура |
| parámetro |  |  | parâmetro | параметър |
| punto y coma |  |  | ponto e vírgula | точка и запетая |
| variable mutable / inmutable |  |  | variável mutável / imutável | променлива с `mut` / непроменлива променлива |
| brazo (de un `match`) |  |  |  | клон (arm) |
| patrón comodín (`_`) |  |  |  | заместващ шаблон (`_`) |
| función asociada |  |  |  | асоциирана функция |
| campo (de un struct) |  |  |  | поле |
| método |  |  |  | метод |
| instancia |  |  |  | инстанция |
| serializar / deserializar |  |  |  | сериализиране / десериализиране |
| valor centinela |  |  |  | сентинелна стойност |

### 5.4b Términos de la ronda 2 (lecciones 5 a 8): traits, genéricos, concurrencia, async

Una sola forma por idioma; los traductores la usan igual desde la fecha de esta fila.

| es | en | fr | pt-BR | bg |
|---|---|---|---|---|
| regla huérfana / de coherencia | orphan rule / coherence rule | règle des orphelins (orphan rule) | regra do órfão (orphan rule) | правило за сираците (orphan rule) |
| implementación inherente | inherent implementation | implémentation inhérente | implementação inerente | присъща имплементация (inherent impl) |
| objeto de trait (`dyn Trait`) | trait object | objet trait (trait object) | objeto de trait (trait object) | trait обект (trait object) |
| despacho estático / dinámico | static / dynamic dispatch | dispatch statique / dynamique | despacho estático / dinâmico | статично / динамично диспечериране |
| monomorfización | monomorphization | monomorphisation | monomorfização | мономорфизация |
| elisión (de lifetimes) | lifetime elision | élision des durées de vie | elisão de lifetimes | елизия на lifetime |
| referencia colgante | dangling reference | référence pendante (dangling reference) | referência pendente (dangling) | висяща референция (dangling) |
| doble de pruebas | test double | double de test | dublê de teste | тестов дубликат (test double) |
| llave (de un HashMap) | key | clé | chave | ключ |
| interbloqueo | deadlock | interblocage (deadlock) | deadlock | мъртва хватка (deadlock) |
| envenenamiento (de un mutex) | poisoning | empoisonnement (poisoning) | envenenamento (poisoning) | отравяне (poisoning) |
| semáforo | semaphore | sémaphore | semáforo | семафор |
| trait de marcado (`Send`, `Sync`) | marker trait | trait marqueur | trait marcador | маркерен trait |
| combinador | combinator | combinateur | combinador | комбинатор |
| emisor / receptor (de un canal) | sender / receiver | émetteur / récepteur | emissor / receptor | изпращач / получател |
| contención | contention | contention | contenção | конкуренция за ресурс (contention) |
| pool de trabajo | worker pool | pool de workers | pool de workers | пул от worker-и |
| atributo (`#[derive]`) | attribute | attribut | atributo | атрибут |
| exhaustividad | exhaustiveness | exhaustivité | exaustividade | изчерпателност |
| invariante | invariant | invariant | invariante | инвариант |

*(Provisional, por revisar: las celdas que ningún traductor propuso las fijó el coordinador por analogía con el glosario; si tu idioma ya usó otra forma en lecciones entregadas, avisa y se unifica.)*

### 5.5 Decisiones de estilo por idioma

- **fr:** «tu» y masculino genérico en los participios que concuerdan con el lector («tu es prêt», «tu as réussi»). Espacio fino antes de `:`, `;`, `?`, `!` según la tipografía francesa (espacio normal sin saltos está bien). Comillas «…».
- **pt-BR:** «você» con verbo en 3.ª persona; ortografía brasileira («arquivo», «tela», «usuário», «ônibus»). «Ownership» se mantiene en inglés, como en la comunidad brasileña, con «propriedade» solo como glosa de la primera mención.
- **bg:** cirílico, tratamiento «ти»; masculino genérico («готов», «успял»). Los términos que la comunidad búlgara usa en latín se dejan en latín (`trait`, `struct`, `enum`, `crate`, `lifetime`, `slice`) y se declinan con guion: «trait-ове», «struct-ът». Comillas „…“.
- **en:** inglés estadounidense; contracciones permitidas.

## 6. Término nuevo

Si te topas con un término que no está aquí:

1. Toma el candado: `mkdir /tmp/glosario-rust.lock` (si falla, otro lo tiene: espera unos segundos y repite; nunca borres un candado ajeno por tu cuenta salvo que lleve más de 5 minutos).
2. **Avisa por SendMessage a los otros tres traductores y al coordinador ANTES de usarlo** (término en es, tu traducción, por qué).
3. Agrega la fila en la subsección que corresponda de esta tabla (solo tu columna si los demás aún no decidieron; el avisado completa la suya).
4. Suelta el candado: `rmdir /tmp/glosario-rust.lock`.

Si otro traductor avisa de un término nuevo, úsalo igual en tu idioma desde ese momento.
