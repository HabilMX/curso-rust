#!/usr/bin/env bash
# Comprueba que el material no lleve referencias internas antes de publicarlo.
#
# Por qué existe: este curso es PÚBLICO. Una ruta interna, un nombre de host o
# un token en un ejemplo quedan indexados por los buscadores y ya no se pueden
# retirar.
#
# Uso:  ./verificar-publicable.sh          → sale 0 si está limpio, 1 si no
#       ./verificar-publicable.sh --probar → solo la autoprueba
#
# 🔴 ESTE GUION ES UNA PUERTA, y una puerta que no puede demostrar que detecta
# no es una puerta: es un adorno que dice "limpio". Por eso trae AUTOPRUEBA
# (siembra un patrón en una copia temporal y exige que lo cache) y por eso
# FALLA CERRADO: si la búsqueda no se pudo hacer, el resultado es rojo, no verde.
set -uo pipefail
cd "$(dirname "$0")"
LISTA=.publicable-prohibido.txt

# Los idiomas se DESCUBREN, no se escriben a mano: una carpeta nueva quedaría
# sin revisar, y una que se renombre haría fallar la búsqueda entera.
descubrir_dirs() {
  local d
  for d in */; do
    d="${d%/}"
    case "$d" in .*|node_modules|target) continue ;; esac
    [ -d "$d" ] && printf '%s\n' "$d"
  done
}

# Busca UN patrón en las carpetas dadas.
#   Devuelve 0 = sin coincidencias · 1 = hay coincidencias · 2 = no se pudo buscar
#
# 🔴 El "--" va DESPUÉS de las banderas y ANTES del patrón. Si se pone antes de
# "--include", grep deja de leerlo como bandera y lo trata como una RUTA que no
# existe: entonces sale con código 2 SIEMPRE —incluso habiendo encontrado
# coincidencias— y quien mire solo el código de salida lee "limpio".
buscar() {
  local patron="$1"; shift
  local salida rc
  salida=$(grep -rnIE --include="*.md" -- "$patron" "$@" 2>&1); rc=$?
  case "$rc" in
    0) printf '%s\n' "$salida"; return 1 ;;
    1) return 0 ;;
    *) printf '%s\n' "$salida" >&2; return 2 ;;
  esac
}

# --- autoprueba: sin esto, un "limpio" no vale nada ---
autoprueba() {
  local tmp patron='TOKEN_DE_AUTOPRUEBA_NO_BORRAR'
  tmp=$(mktemp -d) || return 1
  mkdir -p "$tmp/es"
  printf '%s\n' "$patron" > "$tmp/es/leccion.md"
  local rc=0
  ( cd "$tmp" && buscar "$patron" es >/dev/null 2>&1 ); [ "$?" -eq 1 ] || rc=1
  # y el negativo: un patrón que no está NO debe dar positivo
  ( cd "$tmp" && buscar 'PATRON_QUE_NO_EXISTE_EN_NINGUN_LADO' es >/dev/null 2>&1 ); [ "$?" -eq 0 ] || rc=1
  rm -rf "$tmp"
  return "$rc"
}

if ! autoprueba; then
  echo "❌ la autoprueba falló: la búsqueda no detecta lo que debería."
  echo "   NO se puede afirmar que el material esté limpio. Arregla el guion."
  exit 2
fi
[ "${1:-}" = "--probar" ] && { echo "✅ autoprueba correcta: la búsqueda detecta y descarta bien."; exit 0; }

[ -r "$LISTA" ] || { echo "❌ falta $LISTA"; exit 2; }

mapfile -t DIRS < <(descubrir_dirs)
[ "${#DIRS[@]}" -gt 0 ] || { echo "❌ no hay carpetas de contenido que revisar"; exit 2; }
echo "revisando: ${DIRS[*]}"

fallas=0
errores=0
while IFS= read -r patron || [ -n "$patron" ]; do
  case "$patron" in ''|\#*) continue ;; esac
  hits=$(buscar "$patron" "${DIRS[@]}"); rc=$?
  case "$rc" in
    1) echo "🔴 patrón prohibido: $patron"
       printf '%s\n' "$hits" | sed 's/^/     /'
       fallas=$((fallas + 1)) ;;
    2) echo "⚠️  no se pudo buscar el patrón: $patron"
       errores=$((errores + 1)) ;;
  esac
done < "$LISTA"

if [ "$errores" -gt 0 ]; then
  echo
  echo "❌ $errores patrón(es) no se pudieron revisar. Falla cerrado: NO publicar."
  exit 2
fi
if [ "$fallas" -gt 0 ]; then
  echo
  echo "❌ $fallas patrón(es) prohibido(s). NO publicar hasta limpiarlo."
  exit 1
fi
echo "✅ limpio: ningún patrón prohibido en los archivos .md"
