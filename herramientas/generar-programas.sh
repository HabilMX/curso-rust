#!/usr/bin/env bash
# Arma las carpetas de programas/ que salen de las lecciones 0 a 7: cada programa
# completo como archivo que se puede compilar, con su salida documentada al lado.
#
# Las lecciones son la fuente. Esas carpetas son una copia derivada: no se
# editan a mano. (programas/revisor/ es distinta: es el proyecto real de las
# lecciones 4 y 8, con cargo, y no se genera: lo verifica herramientas/verificar-extractos.sh.)
#
#   herramientas/generar-programas.sh              reescribe programas/ desde las lecciones
#   herramientas/generar-programas.sh --comprobar  falla si programas/ no es idéntica a lo
#                                                  que las lecciones dicen hoy (la usa el CI)
set -euo pipefail
cd "$(dirname "$0")/.."
GEN=$(mktemp -d); trap 'rm -rf "$GEN"' EXIT
herramientas/verificar-programas.sh es --exportar "$GEN" >/dev/null
n=$(find "$GEN" -name 'fig*.rs' | wc -l | tr -d ' ')
[ "$n" -gt 0 ] || { echo "🔴 las lecciones no produjeron ningún programa"; exit 1; }
if [ "${1:-}" = "--comprobar" ]; then
  rc=0
  for d in "$GEN"/*/; do
    l=$(basename "$d")
    diff -r "$d" "programas/$l" >/dev/null 2>&1 || { echo "🔴 programas/$l no coincide con la lección"; diff -r "$d" "programas/$l" | head -10; rc=1; }
  done
  # no debe sobrar ninguna carpeta de lección que ya no exista (revisor/ es aparte)
  for d in programas/*/; do
    l=$(basename "$d"); [ "$l" = revisor ] && continue
    [ -d "$GEN/$l" ] || { echo "🔴 programas/$l ya no sale de ninguna lección"; rc=1; }
  done
  [ "$rc" -eq 0 ] && echo "programas/ coincide con las lecciones: $n programas." \
    || echo "🔴 Corre herramientas/generar-programas.sh y sube el resultado."
  exit "$rc"
else
  for d in "$GEN"/*/; do l=$(basename "$d"); rm -rf "programas/$l"; mkdir -p programas; cp -R "$d" "programas/$l"; done
  echo "programas/ generada desde las lecciones: $n programas."
fi
