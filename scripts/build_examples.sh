#!/usr/bin/env bash

# Builds the ecosystem walkthrough examples. Requires the toolchain described in the document.

set -eu -o pipefail

for ew in ew_boot ew_ral ew_hal
do
    cargo build --package=$ew --target=thumbv7em-none-eabihf
done

for board in teensy4 imxrt1010evk imxrt1170evk-cm7
do
    cargo build --package=ew_bsp --features=$board --target=thumbv7em-none-eabihf
done
