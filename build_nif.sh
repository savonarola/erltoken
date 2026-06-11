#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

cargo build --manifest-path "$root/native/erltoken_nif/Cargo.toml" --release

case "$(uname -s)" in
    Darwin)
        lib="$root/native/erltoken_nif/target/release/liberltoken_nif.dylib"
        ;;
    *)
        lib="$root/native/erltoken_nif/target/release/liberltoken_nif.so"
        ;;
esac

mkdir -p "$root/priv"
cp "$lib" "$root/priv/erltoken_nif.so"

if [ -n "${REBAR_BUILD_DIR:-}" ]; then
    out_dir="$REBAR_BUILD_DIR/lib/erltoken/priv"
    mkdir -p "$out_dir"
    cp "$lib" "$out_dir/erltoken_nif.so"
fi
