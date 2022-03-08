#!/bin/bash

set -o verbose

cargo build --release --bin "$1"
mkdir -p bin/ > /dev/null
arm-none-eabi-objcopy -O binary "target/thumbv7em-none-eabihf/release/$1" "bin/${1}.bin"
sudo dfu-util -a0 -s 0x08000000 -D "bin/${1}.bin"

