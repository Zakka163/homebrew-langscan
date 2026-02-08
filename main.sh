#!/bin/bash

# ========= CONFIG =========
JSON_FILE="languages.json"

LANGS=(
python python3 pypy
node npm deno bun
ruby irb
php perl lua luajit
go rustc cargo
swift
javac java kotlin kotlinc scala groovy
dotnet fsi csc
gcc g++ clang clang++
zig nim crystal dmd ldc2 gdc v odin jai
fortran gfortran pascal fpc
R julia octave matlab
bash zsh fish dash ksh awk sed
)

# ========= COLORS =========
GREEN=$(tput setaf 2)
YELLOW=$(tput setaf 3)
RESET=$(tput sgr0)

# ========= FUNCTIONS =========
is_compiler() {
  case "$1" in
    gcc|g++|clang|clang++|rustc|go|javac|kotlinc|swiftc|zig|nim|crystal|dmd|ldc2|gdc|csc|gfortran|fpc|odin|jai)
      return 0 ;;
    *)
      return 1 ;;
  esac
}

get_type() {
  if is_compiler "$1"; then
    echo "compiler"
  else
    echo "interpreter"
  fi
}

get_color() {
  if is_compiler "$1"; then
    echo "$GREEN"
  else
    echo "$YELLOW"
  fi
}

# ========= SCAN =========
FOUND_ITEMS=""

for cmd in "${LANGS[@]}"; do
  if command -v "$cmd" >/dev/null 2>&1; then
    FOUND_ITEMS+="$cmd"$'\n'
  fi
done

SORTED=$(printf "%s" "$FOUND_ITEMS" | sort -u)

# ========= OUTPUT =========
echo
echo "🔎 Programming Languages Detected"
echo "================================"
echo

TOTAL=0
COMPILERS=0
INTERPRETERS=0

JSON_ENTRIES=""

for cmd in $SORTED; do
  VERSION=$("$cmd" --version 2>&1 | head -n 1 | sed 's/"/\\"/g')
  TYPE=$(get_type "$cmd")
  COLOR=$(get_color "$cmd")

  printf "%s%-12s%s  %-11s  %s\n" \
    "$COLOR" "$cmd" "$RESET" "$TYPE" "$VERSION"

  JSON_ENTRIES+="  {
    \"name\": \"$cmd\",
    \"type\": \"$TYPE\",
    \"version\": \"$VERSION\"
  },"

  TOTAL=$((TOTAL+1))
  [ "$TYPE" = "compiler" ] && COMPILERS=$((COMPILERS+1))
  [ "$TYPE" = "interpreter" ] && INTERPRETERS=$((INTERPRETERS+1))
done

# remove trailing comma
JSON_ENTRIES=$(printf "%s" "$JSON_ENTRIES" | sed '$ s/,$//')

# ========= WRITE JSON =========
cat > "$JSON_FILE" <<EOF
{
  "summary": {
    "total": $TOTAL,
    "compiler": $COMPILERS,
    "interpreter": $INTERPRETERS
  },
  "languages": [
$JSON_ENTRIES
  ]
}
EOF

# ========= SUMMARY =========
echo
echo "📊 Summary"
echo "--------------------------------"
echo "Total        : $TOTAL"
echo "Compilers    : $COMPILERS"
echo "Interpreters : $INTERPRETERS"
echo
echo "📁 Exported to: $JSON_FILE"
