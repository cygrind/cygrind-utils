EMSDK_ROOT=~/deps/emsdk

source $EMSDK_ROOT/emsdk_env.sh

export EMSDK=$EMSDK_ROOT
export EMCC_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0 -s MAX_WEBGL_VERSION=2"

cargo build --target wasm32-unknown-emscripten
