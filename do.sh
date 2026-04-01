#!/usr/bin/bash

CMD=${1:-build}
BIN=${2:-strip_buttons}

fn_run() {
    CMD=$1
    echo $CMD
    $1
}

FFLAG=""
case $BIN in
    random) FFLAG="--features random" ;;
    comets) FFLAG="--features comets" ;;
    one_colour) FFLAG="--features onecolour,colours" ;;
    strip_buttons) FFLAG="--features button,random,wheel,onecolour,fire,comets,colours" ;;
    panel_buttons) FFLAG="--features button,random,wheel,onecolour,firegrid,colours" ;;
esac

case $CMD in
    build) fn_run "cargo build --example $BIN $FFLAG" ;;
    run) fn_run "cargo run --example $BIN $FFLAG" ;;
    release) fn_run "cargo run --release --example $BIN $FFLAG" ;;
    attach) fn_run "probe-rs attach --chip rp235x --protocol swd target/thumbv8m.main-none-eabihf/debug/examples/$BIN" ;;
    embed) fn_run "cargo embed --example $BIN $FFLAG" ;;
    size)
        fn_run "cargo size --example $BIN $FFLAG"
        fn_run "cargo size --example $BIN $FFLAG -- -A"
        cat <<EOF!
.text   = machine code.
.data   = Initialised global and static vars RW.
.rodata = Read-only data (strings and literals). 
.bss    = Uninitialised global & static vars (not in exe file).
EOF!
        ;;
    docs) fn_run "cargo doc --open --all-features" ;;
    gdbserver) fn_run "probe-rs gdb --chip rp235x" ;;
    gdbclient) fn_run "gdb-multiarch ./target/thumbv8m.main-none-eabihf/debug/examples/$BIN" ;;
    *) echo "What! CMD=$CMD" >&2 ;;
esac