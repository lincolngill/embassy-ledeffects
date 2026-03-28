#!/usr/bin/bash

CMD=${1:-build}
BIN=${2:-effect_buttons}

fn_run() {
    CMD=$1
    echo $CMD
    $1
}

FFLAG=""
case $BIN in
    effect_buttons) FFLAG="--features button,random,wheel,onecolour,fire,firegrid,comets" ;;
    random) FFLAG="--features random" ;;
    strip_buttons) FFLAG="--features button,random,wheel,onecolour,fire,comets" ;;
    panel_buttons) FFLAG="--features button,random,wheel,onecolour,firegrid" ;;
esac

case $CMD in
    build) fn_run "cargo build --example $BIN $FFLAG" ;;
    run) fn_run "cargo run --example $BIN $FFLAG" ;;
    release) fn_run "cargo run --release --example $BIN $FFLAG" ;;
    attach) fn_run "probe-rs attach --chip rp235x --protocol swd target/thumbv8m.main-none-eabihf/debug/examples/$BIN" ;;
    embed) fn_run "cargo embed --example $BIN $FFLAG" ;;
    gdbserver) fn_run "probe-rs gdb --chip rp235x" ;;
    gdbclient) fn_run "gdb-multiarch ./target/thumbv8m.main-none-eabihf/debug/examples/$BIN" ;;
    *) echo "What! CMD=$CMD" >2 ;;
esac