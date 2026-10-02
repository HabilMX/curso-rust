#!/usr/bin/env bash
# Verifica los bloques de código de las lecciones que NO son programas de un
# solo archivo (esos los revisa verificar-programas.sh): los EXTRACTOS del
# proyecto real en programas/revisor/ y los FRAGMENTOS.
#
# Por qué existe: un extracto copiado a mano del proyecto puede quedar
# desincronizado del código real en el siguiente cambio, y la lección seguiría
# enseñando algo que ya no existe, sin dar ningún error. Por eso cada bloque
# ```rust de las lecciones tiene que declarar QUÉ es:
#
#   // figNN_NN.rs  (primera línea del bloque)
#       Programa completo. Lo compila y lo corre verificar-programas.sh.
#
#   <!-- verificar:extracto:RUTA -->      (línea justo antes del bloque)
#       El bloque debe aparecer TAL CUAL dentro de RUTA (relativa a
#       programas/revisor/), línea por línea, con UN SOLO corrimiento de
#       indentación CONSTANTE para todo el bloque: no «ignorando la indentación»,
#       porque una indentación caótica sí debe fallar. Vale también para ```toml.
#
#   <!-- verificar:fragmento -->          (línea justo antes del bloque)
#       Ilustración sintáctica o pieza que no corre sola (una firma, un trozo
#       de otro archivo, algo que usa crates que el ejemplo no trae). No
#       pretende ser copia exacta de nada y no se verifica: se DECLARA así, en
#       vez de dejar que alguien lo confunda con una promesa que no se cumple.
#
# Un bloque ```rust sin ninguno de los tres es un error de este guion: falla
# CERRADO, no lo deja pasar en silencio. Igual un marcador sin bloque debajo.
#
# Uso:   herramientas/verificar-extractos.sh              (corre sobre el curso real)
#        herramientas/verificar-extractos.sh --probar     (autoprueba: siembra defectos
#                                                          en una copia y exige que se detecten)
# Sale 0 si todo cuadra, 1 si algo no corresponde, 2 si algo está mal armado
# (incluida la propia compuerta no habiendo podido trabajar).
set -uo pipefail
cd "$(dirname "$0")/.."

command -v python3 >/dev/null || { echo "🔴 falta python3: no se puede verificar nada."; exit 2; }

if [ "${1:-}" = "--probar" ]; then
  python3 - "$0" <<'PY'
import re, shutil, subprocess, sys, tempfile
script = sys.argv[1]
defectos = {
    "un extracto con una letra cambiada":
        ("es/04-colecciones-errores.md", lambda t: t.replace("agrega a qué archivo", "agrega a que archivo", 1)),
    "un extracto con la indentación revuelta":
        ("es/04-colecciones-errores.md", lambda t: t.replace("\n    Ok(serde_yaml", "\n      Ok(serde_yaml", 1)),
    "un bloque rust sin marcar":
        ("es/03-structs-enums.md", lambda t: t.replace("<!-- verificar:fragmento -->\n", "", 1)),
    "un extracto que apunta a un archivo que no existe":
        ("es/08-el-programa.md", lambda t: t.replace("verificar:extracto:src/modelo.rs", "verificar:extracto:src/no_existe.rs", 1)),
    "un marcador sin bloque debajo":
        ("es/05-traits-genericos.md", lambda t: t.replace("<!-- verificar:fragmento -->\n```rust", "<!-- verificar:fragmento -->\n\nTexto en medio.\n\n```rust", 1)),
    "un extracto del perfil de release cambiado":
        ("es/08-el-programa.md", lambda t: t.replace('opt-level = "z"', 'opt-level = "3"', 1)),
}
fallos = 0
for nombre, (archivo, mut) in defectos.items():
    tmp = tempfile.mkdtemp()
    try:
        shutil.copytree("es", tmp + "/es")
        f = tmp + "/" + archivo
        t = open(f, encoding="utf8").read(); n = mut(t)
        if n == t:
            print(f"  🔴 autoprueba: no se pudo sembrar «{nombre}»"); fallos += 1; continue
        open(f, "w", encoding="utf8").write(n)
        rc = subprocess.run([script, "--sobre", tmp + "/es"], capture_output=True, text=True).returncode
        if rc == 0:
            print(f"  🔴 autoprueba: el defecto «{nombre}» NO se detectó"); fallos += 1
        else:
            print(f"  ✅ detectado: {nombre} (código {rc})")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
# control positivo: sin defectos, el verificador debe pasar
rc = subprocess.run([script, "--sobre", "es"], capture_output=True, text=True).returncode
if rc != 0:
    print("  🔴 autoprueba: sobre el curso intacto el verificador NO pasa"); fallos += 1
else:
    print("  ✅ control positivo: sobre el curso intacto pasa")
if fallos:
    sys.exit(2)
print("✅ autoprueba correcta: cada defecto sembrado se detecta.")
PY
  exit $?
fi

LECCIONES=es
[ "${1:-}" = "--sobre" ] && LECCIONES="${2:?falta la carpeta}"
REVISOR=programas/revisor
[ -d "$REVISOR" ] || { echo "🔴 no existe $REVISOR: no hay contra qué comparar. NO publicar."; exit 2; }
ls "$LECCIONES"/[0-9][0-9]-*.md >/dev/null 2>&1 || { echo "🔴 no hay lecciones en $LECCIONES/"; exit 2; }

python3 - "$LECCIONES" "$REVISOR" <<'PY'
import glob, os, re, sys

lecciones, revisor = sys.argv[1], sys.argv[2]
extractos_ok = extractos_mal = fragmentos = programas = sin_marcar = otros_marcados = 0
total_bloques = 0
malos = []

def encontrar(bloque, archivo):
    """¿Aparece `bloque` tal cual en `archivo`, con una indentación constante?"""
    f = open(archivo, encoding="utf8").read().split("\n")
    b = bloque.split("\n")
    while b and not b[-1].strip(): b.pop()
    primera = next((i for i, l in enumerate(b) if l.strip()), None)
    if primera is None:
        return False
    for ini in range(len(f) - len(b) + 1):
        # el prefijo se lee de la primera línea con texto y debe ser solo espacios
        linea = f[ini + primera]
        if not linea.endswith(b[primera]):
            continue
        pref = linea[: len(linea) - len(b[primera])]
        if pref.strip():
            continue
        if all((f[ini + i].strip() == "" if not b[i].strip() else f[ini + i] == pref + b[i]) for i in range(len(b))):
            return True
    return False

for cap in sorted(glob.glob(os.path.join(lecciones, "[0-9][0-9]-*.md"))):
    L = open(cap, encoding="utf8").read().split("\n")
    nombre = os.path.basename(cap)
    i = 0
    marca = None   # (tipo, ruta, línea)
    while i < len(L):
        linea = L[i]
        m = re.match(r"<!--\s*verificar:(extracto:([^\s]+)|fragmento)\s*-->\s*$", linea)
        if m:
            if marca:
                malos.append(f"{nombre}:{marca[2]}: marcador sin bloque debajo")
            marca = ("fragmento", None, i + 1) if m.group(1) == "fragmento" else ("extracto", m.group(2), i + 1)
            i += 1
            continue
        f = re.match(r"```(\w*)\s*$", linea)
        if f:
            lang = f.group(1)
            j = i + 1
            while j < len(L) and not L[j].startswith("```"):
                j += 1
            if j >= len(L):
                malos.append(f"{nombre}:{i+1}: bloque de código sin cerrar")
                break
            cuerpo = "\n".join(L[i + 1 : j])
            if lang in ("rust", "toml"):
                total_bloques += 1
                es_programa = lang == "rust" and re.match(r"//\s*fig\d{2}_\d{2}\.rs\s*(\n|$)", cuerpo)
                if es_programa:
                    programas += 1
                    if marca:
                        malos.append(f"{nombre}:{marca[2]}: un programa (figNN_NN.rs) no lleva marcador")
                elif marca is None:
                    sin_marcar += 1
                    malos.append(f"{nombre}:{i+1}: bloque ```{lang} sin declarar qué es (extracto, fragmento o programa)")
                elif marca[0] == "fragmento":
                    fragmentos += 1
                else:
                    ruta = os.path.join(revisor, marca[1])
                    if not os.path.isfile(ruta):
                        extractos_mal += 1
                        malos.append(f"{nombre}:{i+1}: el extracto apunta a {marca[1]}, que no existe en {revisor}/")
                    elif encontrar(cuerpo, ruta):
                        extractos_ok += 1
                    else:
                        extractos_mal += 1
                        malos.append(f"{nombre}:{i+1}: el extracto NO coincide con {marca[1]} (ni con una indentación constante)")
                marca = None
            elif marca:
                otros_marcados += 1
                malos.append(f"{nombre}:{marca[2]}: marcador sobre un bloque ```{lang or '(sin lenguaje)'}, que no es rust ni toml")
                marca = None
            i = j + 1
            continue
        if marca and linea.strip():
            malos.append(f"{nombre}:{marca[2]}: marcador sin bloque debajo")
            marca = None
        i += 1
    if marca:
        malos.append(f"{nombre}:{marca[2]}: marcador sin bloque debajo")

if total_bloques == 0:
    print("🔴 las lecciones no tienen ningún bloque rust/toml. Falla cerrado: no se midió nada.")
    sys.exit(2)

print(f"  bloques de código verificables: {total_bloques}")
print(f"    programas completos (los corre verificar-programas.sh): {programas}")
print(f"    extractos exactos de {revisor}/: {extractos_ok} coinciden · {extractos_mal} NO coinciden")
print(f"    fragmentos declarados: {fragmentos}")
print(f"    sin declarar qué son: {sin_marcar}")
if programas + extractos_ok + extractos_mal + fragmentos + sin_marcar != total_bloques and not malos:
    print("🔴 la cuenta no cuadra: algún bloque no se clasificó. Falla cerrado.")
    sys.exit(2)
if malos:
    print()
    for m in malos:
        print("  🔴 " + m)
    print()
    print("  🔴 Hay bloques que no corresponden al código real. NO publicar.")
    sys.exit(1)
print("  ✅ Cada bloque de código está declarado, y cada extracto es copia fiel del código real.")
PY
