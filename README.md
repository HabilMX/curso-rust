# Curso de Rust — el segundo, después de Go

[![Verificar programas](https://github.com/HabilMX/curso-rust/actions/workflows/verificar.yml/badge.svg)](https://github.com/HabilMX/curso-rust/actions/workflows/verificar.yml)

**Por Dorian Chávez, fundador de Hábil y arquitecto de integración.**

**Cada programa de este curso se compila y se ejecuta automáticamente en cada cambio; el sello verde lo comprueba y cualquiera puede ver la corrida.**
Haz clic en el sello para abrir la última corrida y ver, paso por paso, qué se ejecutó y qué salió.

> Por ahora el curso está en español; las traducciones vienen en camino.

## Dónde está el contenido

- 📘 **[Español — el curso completo](es/README.md)** ← empieza aquí
- 💻 **[`programas/`](programas/)** — todos los programas del curso, listos para ejecutar

## Cómo ver que los programas funcionan

Sin instalar nada: abre el sello de arriba. Cada corrida muestra los pasos que se ejecutaron y su resultado.

En tu computadora, con [Rust](https://rustup.rs/) instalado (la lección 0 lo instala desde cero):

```bash
cd programas/02-ownership
rustc --edition 2024 fig02_04.rs && ./fig02_04      # compara lo que imprime con fig02_04.salida.txt
```

Cada programa de las lecciones 0 a 7 es un archivo `figNN_NN.rs` dentro de la carpeta de su lección, con su
salida esperada al lado (`figNN_NN.salida.txt`). Algunos **no compilan a propósito**, porque la lección enseña
justo ese error; su salida esperada es el mensaje del compilador. Uno es de pruebas (`fig06_01.rs`) y se
compila con `--test`. El comando exacto de cada uno está en su lección, justo debajo del código.

El proyecto completo que se construye en las lecciones 4 a 8 es [`programas/revisor/`](programas/revisor/), con
sus pruebas:

```bash
cd programas/revisor
cargo test                          # pruebas unitarias, de integración y del binario
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

**Las lecciones son la fuente; `programas/` es una copia que se genera de ellas.** Los programas de las lecciones 0 a 7
se extraen de los bloques de código de `es/*.md` con `herramientas/generar-programas.sh`, y en cada cambio la
verificación comprueba que la copia es idéntica a lo que dicen las lecciones. Así lo que lees y lo que ejecutas
no pueden diferir.

## Qué hay en el repositorio

| | |
|---|---|
| `es/` | el curso en español, una lección por archivo |
| `en/`, `fr/`, `pt/`, `bg/` | **futuras:** las traducciones todavía no existen |
| `programas/` | los programas de las lecciones 0 a 7 (uno por archivo, con su salida esperada) y `revisor/`, el proyecto real de las lecciones 4 y 8 |
| `herramientas/` | los scripts que verifican el curso (ver abajo) |
| `.github/workflows/verificar.yml` | la verificación automática que muestra el sello |
| `verificar-publicable.sh` | revisa que el material no contenga rutas internas ni claves antes de publicarlo |
| `LICENSE.md` | CC BY-SA 4.0 |

Dentro de `herramientas/`:

| | |
|---|---|
| `verificar-programas.sh` | compila y corre cada programa completo de las lecciones (los marcados `// figNN_NN.rs`) y compara su salida real contra la documentada. Los que no compilan a propósito se comparan por su código de error |
| `verificar-extractos.sh` | comprueba que cada bloque de código de las lecciones esté declarado (programa, extracto o fragmento) y que cada **extracto** sea copia exacta del archivo real de `programas/revisor/` |
| `generar-programas.sh` | arma `programas/` desde las lecciones; con `--comprobar` verifica que esté al día |
| `medir-profundidad.sh` | mide las líneas de explicación por lección |
| `verificar-traducciones.sh`, `registrar-traduccion.sh`, `registro-traducciones.tsv` | llevan el control de qué traducciones están al día, para cuando existan |

## Qué es un programa, qué es un extracto, y por qué importa la diferencia

**No todo bloque de código de este curso es un programa completo, y confundirlos engaña al lector.** Hay tres tipos, y cada uno se verifica distinto:

- **Programa completo** — corre solo, de principio a fin: los bloques marcados `// figNN_NN.rs`. Se compilan y se
  ejecutan en cada cambio (con `-D warnings`: si el compilador avisa algo, falla), y su salida documentada se
  compara contra la real.
- **Extracto** — una porción exacta, copiada tal cual, de un archivo real de `programas/revisor/` (que sí es un
  proyecto completo, con sus propias pruebas). No corre por sí solo fuera de ese archivo, pero
  `herramientas/verificar-extractos.sh` confirma que sigue siendo copia fiel, línea por línea, del archivo real —para que
  un cambio en el código no deje a la lección enseñando algo que ya no existe.
- **Fragmento** — una ilustración sintáctica, una firma o un trozo que depende de otro archivo (o de un *crate* que
  el ejemplo no trae), para enseñar un patrón sin el ruido de un programa completo. No pretende ser copia
  exacta de nada y no se verifica en automático — se declara así, en vez de dejar que alguien lo confunda
  con una promesa que no se cumple.

## Cómo se usa el curso

El curso se apoya en [The Book](https://doc.rust-lang.org/book/) y en [Rustlings](https://rustlings.rust-lang.org/):
cada semana dice qué capítulos leer y qué ejercicios hacer. Se recomienda haber terminado antes el
[curso de Go](https://github.com/HabilMX/curso-go), porque este curso escribe el mismo programa —el `revisor`—
y se dedica a lo que Rust hace distinto.
