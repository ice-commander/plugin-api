#!/bin/sh
# Builds the C example plugin into bin/. Nothing about it needs cargo: it is a
# plain shared library built from one C file against include/ic_plugin.h.
set -e
cd "$(dirname "$0")"

case "$(uname -s)" in
    Darwin) EXT=dylib; SHARED="-dynamiclib" ;;
    MINGW*|MSYS*|CYGWIN*) EXT=dll; SHARED="-shared" ;;
    *) EXT=so; SHARED="-shared" ;;
esac

CC=${CC:-cc}
PREFIX=lib
[ "$EXT" = dll ] && PREFIX=

mkdir -p bin
OUT="bin/${PREFIX}ic_hello_c.$EXT"
$CC $SHARED -fPIC -O2 -std=c99 -Wall -Wextra -Iinclude -o "$OUT" src/hello-c/hello_c.c
echo "  $OUT"
