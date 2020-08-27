#!/usr/bin/env sh

set -o errexit

cargo_build() {
    if command -v cargo > /dev/null; then
        echo "Trying to build locally using Cargo.."
        cargo build --release
    else
        echo "Could not build binary. Your installation might be corrupt."
        return 1
    fi
}


cargo_build

