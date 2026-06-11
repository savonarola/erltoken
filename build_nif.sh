#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_name=erltoken

cargo build --manifest-path "$root/native/erltoken_nif/Cargo.toml" --release

case "$(uname -s)" in
    Darwin)
        lib="$root/native/erltoken_nif/target/release/liberltoken_nif.dylib"
        ;;
    *)
        lib="$root/native/erltoken_nif/target/release/liberltoken_nif.so"
        ;;
esac

if [ "${REBAR_BARE_COMPILER_OUTPUT_DIR:-}" ]; then
    priv_dir="$REBAR_BARE_COMPILER_OUTPUT_DIR/priv"
elif [ "${REBAR_BUILD_DIR:-}" ]; then
    priv_dir="$REBAR_BUILD_DIR/lib/$app_name/priv"
else
    priv_dir="$root/priv"
fi

mkdir -p "$priv_dir"
cp "$lib" "$priv_dir/erltoken_nif.so"
