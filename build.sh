#!/bin/sh
# Builds the example plugins in release.
set -e
cd "$(dirname "$0")"

case "$(uname -s)" in
    Darwin) EXT=dylib ;;
    MINGW*|MSYS*|CYGWIN*) EXT=dll ;;
    *) EXT=so ;;
esac

cargo build --release
echo "  bin/target/release/$([ "$EXT" = dll ] || printf lib)ic_example_plugin.$EXT"
