#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(dirname $(readlink -f $0))

if [[ ! -d "$1" ]] || [[ ! -f "$2" ]] || [[ ! -x "$2" ]]; then
    echo -e "Usage: $0 <path/to/C2Rust repository> <path/to/c2rust binary>"
    exit 1
fi

C2RUST_REPO=$(readlink -f $1)
C2RUST_EXE=$(readlink -f $2)
CC_WRAPPER=$C2RUST_REPO/scripts/cc-wrappers/cc
export BUILD_COMMANDS_DIRECTORY=$SCRIPT_DIR/build_commands
CC_DB=$SCRIPT_DIR/compile_commands.json
OUTPUT_DIR=$SCRIPT_DIR/quake3-rs

rm -rf $BUILD_COMMANDS_DIRECTORY
mkdir -p $BUILD_COMMANDS_DIRECTORY

make clean
make BUILD_MISSIONPACK=0 BUILD_SERVER=0 BUILD_GAME_SO=1 BUILD_GAME_QVM=0 BUILD_RENDERER_OPENGL2=0 CC=$CC_WRAPPER
$C2RUST_REPO/scripts/convert_build_commands.py $BUILD_COMMANDS_DIRECTORY $CC_DB
$C2RUST_EXE transpile $CC_DB -e -o $OUTPUT_DIR --overwrite-existing --reorganize-definitions

# Until we implemented custom alignments for globals, we need to fix
# asm/snapvector.rs manually
patch -d $OUTPUT_DIR -l -p2 < $SCRIPT_DIR/asm-snapvector.patch
