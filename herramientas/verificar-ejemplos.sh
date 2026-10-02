#!/usr/bin/env bash
# Verifica los EJEMPLOS DE CARGO de las lecciones: los programas que usan un
# crate externo (tokio, reqwest, serde, clap) y por eso no se pueden compilar con
# un `rustc` a secas, como los figNN_NN.rs. Viven en programas/revisor/examples/
# y comparten las dependencias (y el Cargo.lock) del proyecto.
#
# Por qué existe: un programa con dependencias que nadie compila ni ejecuta se
# desactualiza en silencio, igual que cualquier salida escrita a mano. Aquí cada
# ejemplo se compila y se corre de verdad, y su salida documentada se compara con
# la real.
#
# Cómo se declara un ejemplo en una lección:
#
#   <!-- verificar:ejemplo:ejemplo_tokio -->      (línea justo antes del bloque)
#   ```rust
#   ...el archivo examples/ejemplo_tokio.rs COMPLETO, tal cual...
#   ```
#
#   ```bash
#   $ cargo run --example ejemplo_tokio
#   ...la salida...
#   ```
#
# Lo que se comprueba:
#   - el bloque rust es IDÉNTICO al archivo completo (no un trozo: eso es un extracto);
#   - el comando documentado es exactamente `cargo run --example NOMBRE`;
#   - el ejemplo compila, termina con código 0 e imprime EXACTAMENTE la salida
#     documentada (se ignoran solo los espacios al final de cada línea);
#   - todo archivo de examples/ está documentado en alguna lección (ninguno huérfano).
#
# Uso:   herramientas/verificar-ejemplos.sh              (sobre el curso real)
#        herramientas/verificar-ejemplos.sh --probar     (siembra defectos en una copia)
# Sale 0 si todo cuadra, 1 si algo no corresponde, 2 si algo está mal armado
# (falla CERRADO: sin ejemplos o sin poder trabajar, nunca «limpio» sin medir).
set -uo pipefail
cd "$(dirname "$0")/.."

command -v python3 >/dev/null || { echo "🔴 falta python3: no se puede verificar nada."; exit 2; }
command -v cargo >/dev/null || { echo "🔴 falta cargo: no se pueden ejecutar los ejemplos."; exit 2; }

if [ "${1:-}" = "--probar" ]; then
  python3 - "$0" <<'PY'
import os, shutil, subprocess, sys, tempfile
script = os.path.abspath(sys.argv[1])
# UNA sola copia de trabajo, con su propio target/: cada defecto se siembra, se mide y se
# deshace. Compartir el target/ con el proyecto real (o entre copias en rutas distintas)
# mezcla binarios de unas pruebas con las de otras y da lecturas falsas.
tmp = tempfile.mkdtemp()
shutil.copytree("es", tmp + "/es")
shutil.copytree("programas/revisor", tmp + "/programas/revisor", ignore=shutil.ignore_patterns("target"))
env = dict(os.environ, CARGO_TARGET_DIR=tmp + "/target")
def correr():
    return subprocess.run([script, "--sobre", tmp], capture_output=True, text=True, env=env).returncode
def leer(r): return open(tmp + "/" + r, encoding="utf8").read()
def escribir(r, t): open(tmp + "/" + r, "w", encoding="utf8").write(t)

L7, L8 = "es/07-concurrencia-async.md", "es/08-el-programa.md"
CLAP = "programas/revisor/examples/ejemplo_clap.rs"
defectos = [
    ("una línea de la salida documentada alterada", [(L7, "\nterminó pagos\n", "\nterminó pagoss\n")]),
    ("el código de la lección distinto del archivo", [(L8, "default_value_t = 5)]\n    paralelo: usize,\n}\n\nfn main", "default_value_t = 6)]\n    paralelo: usize,\n}\n\nfn main")]),
    ("el comando documentado distinto", [(L8, "$ cargo run --example ejemplo_reqwest", "$ cargo run ejemplo_reqwest")]),
    ("un ejemplo sin su bloque de salida", [(L8, "```bash\n$ cargo run --example ejemplo_serde", "```text\n$ cargo run --example ejemplo_serde")]),
    # solo el código de salida: archivo y lección cambian igual, la salida impresa no
    ("un ejemplo que termina con código distinto de 0",
        [(CLAP, "    println!(\"{a:?}\");", "    println!(\"{a:?}\");\n    if a.paralelo == 2 {\n        std::process::exit(3);\n    }"),
         (L8, "    println!(\"{a:?}\");", "    println!(\"{a:?}\");\n    if a.paralelo == 2 {\n        std::process::exit(3);\n    }")]),
]
fallos = 0
try:
    # control positivo primero: sobre el curso intacto debe pasar
    if correr() != 0:
        print("  🔴 autoprueba: sobre el curso intacto el verificador NO pasa"); fallos += 1
    else:
        print("  ✅ control positivo: sobre el curso intacto pasa")
    for nombre, cambios in defectos:
        originales = {r: leer(r) for r, _, _ in cambios}
        try:
            sembrado = True
            for r, a, b in cambios:
                t = leer(r)
                if a not in t:
                    sembrado = False; break
                escribir(r, t.replace(a, b, 1))
            if not sembrado:
                print(f"  🔴 autoprueba: no se pudo sembrar «{nombre}»"); fallos += 1; continue
            rc = correr()
            if rc == 0:
                print(f"  🔴 autoprueba: el defecto «{nombre}» NO se detectó"); fallos += 1
            else:
                print(f"  ✅ detectado: {nombre} (código {rc})")
        finally:
            for r, t in originales.items(): escribir(r, t)
    # un archivo huérfano en examples/
    huerfano = tmp + "/programas/revisor/examples/huerfano.rs"
    try:
        open(huerfano, "w").write("fn main() {}\n")
        rc = correr()
        if rc == 0:
            print("  🔴 autoprueba: un ejemplo huérfano NO se detectó"); fallos += 1
        else:
            print(f"  ✅ detectado: un ejemplo huérfano (código {rc})")
    finally:
        os.remove(huerfano)
    # tras deshacer todo, vuelve a pasar (las siembras no dejaron rastro)
    if correr() != 0:
        print("  🔴 autoprueba: tras deshacer las siembras el verificador NO pasa"); fallos += 1
    else:
        print("  ✅ control positivo final: tras deshacer las siembras pasa")
finally:
    shutil.rmtree(tmp, ignore_errors=True)
if fallos:
    sys.exit(2)
print("✅ autoprueba correcta: cada defecto sembrado se detecta.")
PY
  exit $?
fi

RAIZ=.
[ "${1:-}" = "--sobre" ] && RAIZ="${2:?falta la carpeta}"
[ -d "$RAIZ/programas/revisor/examples" ] || { echo "🔴 no existe $RAIZ/programas/revisor/examples: no hay ejemplos que medir."; exit 2; }
ls "$RAIZ"/es/[0-9][0-9]-*.md >/dev/null 2>&1 || { echo "🔴 no hay lecciones en $RAIZ/es/"; exit 2; }

python3 - "$RAIZ" <<'PY'
import glob, os, re, subprocess, sys

raiz = sys.argv[1]
revisor = os.path.join(raiz, "programas", "revisor")
malos, vistos, ok = [], {}, 0

def sin_espacios_finales(t):
    return "\n".join(l.rstrip() for l in t.rstrip("\n").split("\n"))

for cap in sorted(glob.glob(os.path.join(raiz, "es", "[0-9][0-9]-*.md"))):
    nombre = os.path.basename(cap)
    L = open(cap, encoding="utf8").read().split("\n")
    i = 0
    while i < len(L):
        m = re.match(r"<!--\s*verificar:ejemplo:([A-Za-z0-9_]+)\s*-->\s*$", L[i])
        if not m:
            i += 1; continue
        ej, marca = m.group(1), i + 1
        if ej in vistos:
            malos.append(f"{nombre}:{marca}: el ejemplo {ej} ya estaba declarado en {vistos[ej]}")
        vistos[ej] = nombre
        # el bloque rust, justo debajo
        j = i + 1
        if j >= len(L) or L[j].strip() != "```rust":
            malos.append(f"{nombre}:{marca}: el marcador no tiene un bloque ```rust justo debajo"); i += 1; continue
        k = j + 1
        while k < len(L) and not L[k].startswith("```"): k += 1
        codigo = "\n".join(L[j + 1:k])
        archivo = os.path.join(revisor, "examples", ej + ".rs")
        if not os.path.isfile(archivo):
            malos.append(f"{nombre}:{marca}: no existe examples/{ej}.rs"); i = k + 1; continue
        real = open(archivo, encoding="utf8").read()
        if sin_espacios_finales(codigo) != sin_espacios_finales(real):
            malos.append(f"{nombre}:{marca}: el bloque NO es idéntico a examples/{ej}.rs completo")
        # el bloque bash con el comando y la salida, antes del siguiente marcador o encabezado
        b = k + 1
        while b < len(L) and not L[b].startswith("```bash") and not L[b].startswith("## ") \
                and not L[b].startswith("### ") and not L[b].startswith("<!--"):
            b += 1
        if b >= len(L) or not L[b].startswith("```bash"):
            malos.append(f"{nombre}:{marca}: el ejemplo {ej} no trae su bloque ```bash con la salida"); i = k + 1; continue
        e = b + 1
        while e < len(L) and not L[e].startswith("```"): e += 1
        bloque = L[b + 1:e]
        comando = f"$ cargo run --example {ej}"
        if not bloque or bloque[0] != comando:
            malos.append(f"{nombre}:{b+1}: el comando documentado debe ser exactamente «{comando}»")
            i = e + 1; continue
        esperado = sin_espacios_finales("\n".join(bloque[1:]))
        r = subprocess.run(["cargo", "run", "--quiet", "--example", ej], cwd=revisor,
                           capture_output=True, text=True)
        if r.returncode != 0:
            malos.append(f"{nombre}:{marca}: {ej} terminó con código {r.returncode}\n        " + "\n        ".join(r.stderr.splitlines()[:10]))
        elif sin_espacios_finales(r.stdout) != esperado:
            import difflib
            d = list(difflib.unified_diff(esperado.split("\n"), sin_espacios_finales(r.stdout).split("\n"), "documentada", "real", lineterm="", n=0))[:12]
            malos.append(f"{nombre}:{marca}: la salida de {ej} NO coincide con la documentada\n        " + "\n        ".join(d))
        else:
            print(f"  {ej:<18} {nombre:<28} ✅"); ok += 1
        i = e + 1

archivos = sorted(os.path.basename(f)[:-3] for f in glob.glob(os.path.join(revisor, "examples", "*.rs")))
huerfanos = [a for a in archivos if a not in vistos]
for h in huerfanos:
    malos.append(f"examples/{h}.rs no está documentado en ninguna lección")

if not vistos and not malos:
    print("🔴 las lecciones no declaran ningún ejemplo de cargo. Falla cerrado: no se midió nada."); sys.exit(2)
print()
print(f"  ejemplos de cargo: {len(archivos)} archivos · {len(vistos)} declarados en lecciones · {ok} coinciden")
if malos:
    print()
    for m in malos: print("  🔴 " + m)
    print("\n  🔴 Hay ejemplos que no corresponden a la ejecución real. NO publicar."); sys.exit(1)
if ok != len(archivos):
    print("  🔴 la cuenta no cuadra: algo no se midió. NO publicar."); sys.exit(2)
print("  ✅ Cada ejemplo de cargo es copia fiel de su archivo, compila, corre y su salida coincide.")
PY
