#!/usr/bin/env bash
# Compila y ejecuta CADA programa de ejemplo del curso, y compara su salida
# real contra la documentada.
#
# Por qué existe: el curso promete que «cada programa se compila y se ejecuta
# automáticamente en cada cambio». Una promesa verificable que nadie verifica es
# peor que no prometer nada: una salida escrita a mano, o que dejó de ser cierta
# cuando cambió el código, enseña algo falso sin dar ningún error.
#
# Qué es un programa: un bloque ```rust cuya PRIMERA línea es `// figNN_NN.rs`.
# Justo debajo, un bloque ```bash con el comando y la salida documentada:
#
#   $ rustc --edition 2024 figNN_NN.rs && ./figNN_NN        se compila y se corre
#   $ rustc --edition 2024 figNN_NN.rs                      NO compila, a propósito
#   $ rustc --edition 2024 --test figNN_NN.rs && ./figNN_NN --test-threads=1
#                                                           pruebas (las de #[test])
#
# Lo que se compara:
#   - programa que corre: su salida estándar, línea por línea. Se compila con
#     -D warnings: un aviso del compilador es un fallo, porque quien ejecuta el
#     programa lo vería en su terminal y la lección no lo mostraría.
#   - programa que NO compila: que de verdad falle, y que el error documentado sea
#     el real COMPLETO (código EXXXX, ubicación, mensaje y notas): un texto
#     documentado distinto del que imprime rustc es un defecto. Solo se normaliza
#     lo que depende de la máquina o de la versión: la ruta /rustc/<hash>/ de la
#     biblioteca estándar y los espacios finales.
#
# Uso:  herramientas/verificar-programas.sh [idioma]      (por omisión: es)
#       herramientas/verificar-programas.sh es --mostrar  (imprime la salida real de cada uno)
#       herramientas/verificar-programas.sh es --exportar DIR
#           no corre nada: deja en DIR/<leccion>/ cada programa (figNN_NN.rs) y su
#           salida documentada: figNN_NN.salida.txt, o figNN_NN.error-esperado.txt
#           si el programa NO compila a propósito (la lección enseña ese error).
#           Lo usa generar-programas.sh.
#       herramientas/verificar-programas.sh --probar
#           autoprueba: siembra defectos en una copia y exige que se detecten.
#
# Sale 0 si todo cuadra, 1 si alguna salida no coincide, 2 si algo está mal armado
# (incluido no poder trabajar: falla CERRADO, nunca «limpio» sin haber medido).
set -uo pipefail
cd "$(dirname "$0")/.."

if [ "${1:-}" = "--probar" ]; then
  command -v python3 >/dev/null || { echo "🔴 falta python3"; exit 2; }
  python3 - "$0" <<'PY'
import os, re, shutil, subprocess, sys, tempfile
script = sys.argv[1]
defectos = {
    "salida documentada alterada":
        lambda t: t.replace("hola, ya tengo Rust\n```", "hola, ya tengo Rustt\n```", 1),
    "código de error documentado cambiado":
        lambda t: t.replace("error[E0384]", "error[E0999]", 1),
    "programa que debía fallar compila":
        lambda t: t.replace("    x = 6;\n    println!(\"{x}\");", "    println!(\"{x}\");", 1),
    "salida documentada borrada":
        lambda t: re.sub(r"(\$ rustc --edition 2024 fig00_01\.rs && \./fig00_01\n)[^\n]*\n", r"\1", t, count=1),
    "texto del error documentado cambiado (mismo código)":
        lambda t: t.replace("cannot assign twice to immutable variable", "no se puede asignar dos veces", 1),
    "programa que ya no compila":
        lambda t: t.replace('// fig00_01.rs\nfn main() {', '// fig00_01.rs\nfn main( {', 1),
    "comando documentado distinto":
        lambda t: t.replace("rustc --edition 2024 fig00_01.rs", "rustc fig00_01.rs", 1),
}
fallos = 0
for nombre, mut in defectos.items():
    tmp = tempfile.mkdtemp()
    try:
        shutil.copytree("es", tmp + "/es")
        f = tmp + "/es/00-preparacion.md"
        t = open(f, encoding="utf8").read()
        n = mut(t)
        if n == t:
            print(f"  🔴 autoprueba: no se pudo sembrar «{nombre}»"); fallos += 1; continue
        open(f, "w", encoding="utf8").write(n)
        rc = subprocess.run([script, tmp + "/es"], capture_output=True, text=True).returncode
        if rc == 0:
            print(f"  🔴 autoprueba: el defecto «{nombre}» NO se detectó"); fallos += 1
        else:
            print(f"  ✅ detectado: {nombre} (código {rc})")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
if fallos:
    sys.exit(2)
print("✅ autoprueba correcta: cada defecto sembrado se detecta.")
PY
  exit $?
fi

IDI="${1:-es}"
MODO="${2:-}"
DESTINO="${3:-}"
if [ "$MODO" = "--exportar" ] && [ -z "$DESTINO" ]; then echo "falta el directorio de destino"; exit 2; fi
command -v python3 >/dev/null || { echo "🔴 falta python3: no se puede verificar nada."; exit 2; }
command -v rustc >/dev/null || { echo "🔴 falta rustc: no se pueden compilar los programas."; exit 2; }
ls "$IDI"/[0-9][0-9]-*.md >/dev/null 2>&1 || { echo "🔴 no hay lecciones en $IDI/"; exit 2; }
echo "Rust: $(rustc --version)"
echo

python3 - "$IDI" "$MODO" "$DESTINO" <<'PY'
import glob, os, re, shutil, subprocess, sys, tempfile

idioma, modo, destino = sys.argv[1], sys.argv[2], sys.argv[3]
EDICION = "2024"

def normalizar(texto):
    lineas = [l.rstrip() for l in texto.replace("\r\n", "\n").split("\n")]
    texto = "\n".join(lineas).strip("\n")
    return re.sub(r"finished in [0-9.]+s", "finished in 0.00s", texto)

def codigos(texto):
    return re.findall(r"^error\[(E\d{4})\]", texto, re.M)

def normalizar_error(texto):
    """El error de rustc listo para comparar: sin la ruta /rustc/<hash>/ (cambia en
    cada versión) y sin espacios finales. Todo lo demás se compara tal cual."""
    texto = re.sub(r"/rustc/[0-9a-f]{7,40}/", "/rustc/<hash>/", texto)
    return normalizar(texto)

# --- 1. extraer los programas de las lecciones ---------------------------------
programas = []   # (figura, leccion, codigo, comando, salida)
problemas = []
vistos = {}
for cap in sorted(glob.glob(os.path.join(idioma, "[0-9][0-9]-*.md"))):
    texto = open(cap, encoding="utf8").read()
    # cada bloque ```rust, y lo que venga después hasta el siguiente bloque ```rust o encabezado
    for m in re.finditer(r"```rust\n(.*?)```(.*?)(?=\n##|\n```rust|\Z)", texto, re.S):
        codigo, resto = m.group(1), m.group(2)
        nom = re.match(r"//\s*(fig\d{2}_\d{2})\.rs\s*\n", codigo)
        if not nom:
            continue
        fig = nom.group(1)
        lec = os.path.basename(cap)[:-3]
        if fig in vistos:
            problemas.append(f"{fig}: aparece dos veces ({vistos[fig]} y {lec})")
            continue
        vistos[fig] = lec
        if not lec.startswith(fig[3:5]):
            problemas.append(f"{fig}: está en la lección {lec}, pero su número dice otra")
        sal = re.search(r"```bash\n(\$ rustc [^\n]*)\n(.*?)```", resto, re.S)
        if not sal:
            programas.append((fig, lec, codigo, None, None))
        else:
            programas.append((fig, lec, codigo, sal.group(1), sal.group(2)))

if not programas:
    print("🔴 las lecciones no produjeron ningún programa. Falla cerrado.")
    sys.exit(2)

# --- 2. exportar -----------------------------------------------------------------
if modo == "--exportar":
    for fig, lec, codigo, cmd, sal in programas:
        d = os.path.join(destino, lec)
        os.makedirs(d, exist_ok=True)
        open(os.path.join(d, fig + ".rs"), "w", encoding="utf8").write(codigo)
        if sal is not None:
            # el comando sin «&& ./fig» es el que documenta un fallo de compilación
            ext = ".error-esperado.txt" if cmd == f"$ rustc --edition {EDICION} {fig}.rs" else ".salida.txt"
            open(os.path.join(d, fig + ext), "w", encoding="utf8").write(sal)
    sys.exit(0)

# --- 3. compilar, correr, comparar ----------------------------------------------
total = ok = malos = sin_salida = 0
trabajo = tempfile.mkdtemp()
try:
    for fig, lec, codigo, cmd, esperado in programas:
        total += 1
        d = os.path.join(trabajo, fig); os.makedirs(d)
        open(os.path.join(d, fig + ".rs"), "w", encoding="utf8").write(codigo)
        etiqueta = f"  {fig:<10} {lec:<26}"
        if cmd is None:
            print(f"{etiqueta} ⬜ sin salida documentada"); sin_salida += 1; continue

        # el comando documentado tiene que ser exactamente uno de los tres conocidos
        c_falla = f"$ rustc --edition {EDICION} {fig}.rs"
        c_corre = f"{c_falla} && ./{fig}"
        c_test  = f"$ rustc --edition {EDICION} --test {fig}.rs && ./{fig} --test-threads=1"
        if cmd not in (c_falla, c_corre, c_test):
            print(f"{etiqueta} 🔴 comando documentado no reconocido: {cmd}"); malos += 1; continue

        flags = ["--test"] if cmd == c_test else []
        exe = os.path.join(d, fig)
        esperado_n = normalizar(esperado)
        espera_error = cmd == c_falla

        if espera_error:
            if not codigos(esperado):
                print(f"{etiqueta} 🔴 documenta un fallo de compilación pero no trae ningún código E0000"); malos += 1; continue
            r = subprocess.run(["rustc", "--edition", EDICION, *flags, fig + ".rs", "-o", exe],
                               capture_output=True, text=True, cwd=d)
            if r.returncode == 0:
                print(f"{etiqueta} 🔴 DEBÍA NO COMPILAR y compiló"); malos += 1; continue
            real, ref = codigos(r.stderr), codigos(esperado)
            if real != ref:
                print(f"{etiqueta} 🔴 códigos de error distintos: documentados {ref}, reales {real}"); malos += 1
            else:
                a, b = normalizar_error(esperado), normalizar_error(r.stderr)
                if a == b:
                    print(f"{etiqueta} ✅ no compila, como documenta ({', '.join(ref)})"); ok += 1
                else:
                    print(f"{etiqueta} 🔴 el TEXTO del error no coincide ({', '.join(ref)})")
                    import difflib
                    for l in list(difflib.unified_diff(a.split("\n"), b.split("\n"), "documentado", "real", lineterm="", n=0))[:14]:
                        print("        " + l)
                    malos += 1
            if modo == "--mostrar": print(r.stderr)
            continue

        r = subprocess.run(["rustc", "--edition", EDICION, "-D", "warnings", *flags, fig + ".rs", "-o", exe],
                           capture_output=True, text=True, cwd=d)
        if r.returncode != 0:
            print(f"{etiqueta} 🔴 NO COMPILA (o avisó, y se compila con -D warnings)")
            print("\n".join("        " + l for l in r.stderr.splitlines()[:12])); malos += 1; continue
        args = ["--test-threads=1"] if cmd == c_test else []
        r = subprocess.run([exe, *args], capture_output=True, text=True, cwd=d)
        real = normalizar(r.stdout)
        if modo == "--mostrar":
            print(f"──── {fig} ({lec}) ────\n{real}\n")
        if r.returncode != 0:
            print(f"{etiqueta} 🔴 terminó con código {r.returncode}"); malos += 1; continue
        if real == esperado_n:
            print(f"{etiqueta} ✅"); ok += 1
        else:
            print(f"{etiqueta} 🔴 NO COINCIDE")
            import difflib
            for l in list(difflib.unified_diff(esperado_n.split("\n"), real.split("\n"), "documentada", "real", lineterm=""))[:12]:
                print("        " + l)
            malos += 1
finally:
    shutil.rmtree(trabajo, ignore_errors=True)

for p in problemas:
    print(f"  🔴 {p}"); malos += 1

if modo == "--mostrar":
    sys.exit(0)
print()
print(f"  programas: {total} · coinciden: {ok} · NO coinciden: {malos} · sin salida documentada: {sin_salida}")
if malos:
    print("  🔴 Hay salidas documentadas que no corresponden. NO publicar."); sys.exit(1)
# FALLA CERRADO: un programa sin salida documentada NO es un programa verificado.
if sin_salida:
    print(f"  🔴 {sin_salida} figura(s) sin salida documentada. NO publicar."); sys.exit(1)
if ok != total:
    print("  🔴 la cuenta no cuadra: algo no se midió. NO publicar."); sys.exit(2)
print("  ✅ Todas las salidas documentadas corresponden a la ejecución real.")
PY
