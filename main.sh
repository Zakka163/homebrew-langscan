#!/bin/bash

printf "\n🔎 SCANNING PROGRAMMING LANGUAGES ON macOS\n"
printf "=========================================\n\n"

print_section() {
  printf "\n📦 %s\n" "$1"
  printf "-----------------------------------------\n"
}

print_cmd() {
  local cmd="$1"

  if command -v "$cmd" >/dev/null 2>&1; then
    local version
    version=$("$cmd" --version 2>&1 | head -n 1 | tr -d '\n')
    printf "✅ %-15s %s\n" "$cmd" "$version"
    return 0
  fi
  return 1
}

FOUND=0

# ---- Popular ----
print_section "Popular"
for cmd in python python3 pypy node npm deno bun ruby irb php perl lua luajit \
           go rustc cargo swift javac java kotlin kotlinc scala groovy \
           dotnet fsi csc gcc g++ clang clang++ make cmake meson ninja; do
  print_cmd "$cmd" && FOUND=1
done

# ---- Functional ----
print_section "Functional"
for cmd in elixir iex erl ghc runghc ocaml opam fsharp clojure lein bb \
           racket scheme guile chez; do
  print_cmd "$cmd" && FOUND=1
done

# ---- Systems / Low level ----
print_section "Systems / Low-level"
for cmd in zig nim crystal dmd ldc2 gdc v odin jai red vala \
           fortran gfortran pascal fpc; do
  print_cmd "$cmd" && FOUND=1
done

# ---- Data / Scientific ----
print_section "Data / Scientific"
for cmd in R julia octave matlab sage; do
  print_cmd "$cmd" && FOUND=1
done

# ---- Shell / Scripting ----
print_section "Shell / Scripting"
for cmd in bash zsh fish dash ksh awk sed; do
  print_cmd "$cmd" && FOUND=1
done

# ---- Web / Misc ----
print_section "Web / Misc"
for cmd in wasm wat2wasm haxe reason rescript dart flutter; do
  print_cmd "$cmd" && FOUND=1
done

# ---- Esoteric / Rare ----
print_section "Esoteric / Rare"
for cmd in cobc clisp sbcl io pony factor idris agda; do
  print_cmd "$cmd" && FOUND=1
done

# ---- Apple / Mobile ----
print_section "Apple / Mobile"
for cmd in xcodebuild metal; do
  print_cmd "$cmd" && FOUND=1
done

if [ "$FOUND" -eq 0 ]; then
  printf "\n❌ No programming languages detected\n"
else
  printf "\n🎉 Scan complete\n"
fi
