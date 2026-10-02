#!/usr/bin/env bash
# Mide la profundidad de cada lección del curso, para que ACEPTADO o RECHAZADO
# sea reproducible y no una discusión. Quien revisa y quien escribe corren el
# MISMO comando y obtienen el MISMO número.
#
# Uso:  ./medir-profundidad.sh [carpeta]     (por omisión: es)
#
# Qué cuenta como «línea de explicación»: una línea NO vacía que está FUERA de
# un bloque de código cercado con ```. Se define aquí a propósito: sin una
# definición única, dos personas cuentan distinto y el criterio deja de ser un
# criterio.
#
# 🔴 POR QUÉ NO SE USA «LA MITAD DE LA MEDIANA DEL CURSO», que era la propuesta
# original. Medido en este curso el 29-sep-2026:
#
#   02, 03, 04 (densas) → 333, 323, 296 líneas de explicación
#   00, 01              → 221,  92
#   05, 06, 07 (apuntes)→  79,  77,  86
#   mediana = 156  ·  su mitad = 78
#
# Con ese piso, los tres apuntes PASAN (79, 77 y 86 contra 78) — solo uno falla,
# y por una línea. **El criterio se muerde la cola: las lecciones flacas bajan la
# mediana, la mediana baja el piso, y el piso las deja pasar.** Un curso entero
# de apuntes lo aprobaría sin una sola observación.
#
# Garantiza UNIFORMIDAD, no PROFUNDIDAD. Por eso aquí se mide contra dos varas
# ABSOLUTAS, calibradas con las lecciones que sí están bien hechas de este mismo
# curso:
#
#   PISO      = 150 líneas de explicación por lección (ninguna por debajo)
#   MEDIANA   = 250 líneas de explicación para el curso completo
#
# Las dos se pueden cambiar por variable de entorno, pero **se cambian a la vista
# y se dice por qué**, no en silencio.
set -uo pipefail
cd "$(dirname "$0")"

DIR="${1:-es}"
PISO="${PISO:-150}"
MEDIANA_MIN="${MEDIANA_MIN:-250}"

[ -d "$DIR" ] || { echo "no existe la carpeta $DIR"; exit 2; }

python3 - "$DIR" "$PISO" "$MEDIANA_MIN" <<'PY'
import glob, os, statistics, sys

dir_, piso, med_min = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])

def contar(ruta):
    """Devuelve (total, explicacion, codigo). Explicacion = linea no vacia fuera de ```."""
    total = exp = cod = 0
    dentro = False
    for linea in open(ruta, encoding="utf-8"):
        s = linea.strip()
        if s.startswith("```"):
            dentro = not dentro
            continue
        total += 1
        if not s:
            continue
        if dentro:
            cod += 1
        else:
            exp += 1
    return total, exp, cod

# Solo las lecciones: archivos que empiezan con digitos. README y bitacora no son lecciones.
archivos = sorted(f for f in glob.glob(os.path.join(dir_, "*.md"))
                  if os.path.basename(f)[:1].isdigit())
if not archivos:
    print(f"no hay lecciones numeradas en {dir_}/")
    raise SystemExit(2)

filas = [(os.path.basename(f), *contar(f)) for f in archivos]
exps = [e for _, _, e, _ in filas]
mediana = statistics.median(exps)

print(f"  {'lección':<34}{'total':>7}{'explicación':>13}{'código':>9}   veredicto")
fallan = []
for nombre, total, exp, cod in filas:
    ok = exp >= piso
    if not ok:
        fallan.append((nombre, exp))
    print(f"  {nombre:<34}{total:>7}{exp:>13}{cod:>9}   {'✅' if ok else '🔴 por debajo del piso'}")

print()
print(f"  piso exigido por lección : {piso} líneas de explicación")
print(f"  mediana del curso        : {mediana:.0f}  (mínimo exigido: {med_min})")

problemas = 0
if fallan:
    problemas += 1
    print()
    print(f"  🔴 {len(fallan)} lección(es) por debajo del piso:")
    for n, e in fallan:
        print(f"     • {n}: {e} líneas — le faltan {piso - e}")
if mediana < med_min:
    problemas += 1
    print()
    print(f"  🔴 la mediana del curso ({mediana:.0f}) está por debajo del mínimo ({med_min}).")
    print("     Eso significa que el curso es uniformemente superficial: ninguna")
    print("     lección falla sola, pero el conjunto no enseña a la profundidad pedida.")

print()
if problemas:
    print("  ❌ NO cumple la paridad de profundidad.")
    raise SystemExit(1)
print("  ✅ Cumple la paridad de profundidad.")
PY
