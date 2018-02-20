#!/bin/sh

cd "$(dirname "$0")/.."
docker run -t --rm -v "$PWD:/src" -v "$HOME/.cargo/registry:/root/.cargo/registry" -e EMMAKEN_CFLAGS='-s USE_SDL=2 --preload-file assets' remram/emscripten-rust-sdl
cp target/asmjs-unknown-emscripten/release/deps/piston_emscripten_text-*.data .
