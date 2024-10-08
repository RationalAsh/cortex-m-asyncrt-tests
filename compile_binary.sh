#!/bin/bash

# Get the first argument and if it does not exist set it to the default.
# This is the name of the binary to compile.
BINARY_NAME=${1:-blinky}

# Echo everything
set -x

# First, compile
cargo build --release

# List of binaries to be flashed
BINARIES=("blinky" "qemu-test")

# Loop through the binaries and create the assembly and binary files.
for binary in "${BINARIES[@]}"
do
    # Then, create the assembly file.
    cargo objdump --bin ${binary} --release -- -d --no-show-raw-insn --print-imm-hex > firmware/${binary}.asm

    # Then create the binary file.
    cargo objcopy --release -- -O binary firmware/${binary}.bin

    # Run cargo size to get the size of the binary.
    cargo size --bin ${binary} --release
done

# Then, create the assembly file.
# cargo objdump --bin ${BINARY_NAME} --release -- -d --no-show-raw-insn --print-imm-hex > firmware/${BINARY_NAME}.asm

# # Then create the binary file.
# # arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/blinky firmware.bin
# cargo objcopy --release -- -O binary firmware/${BINARY_NAME}.bin

# # Run cargo size to get the size of the binary.
# cargo size --bin ${BINARY_NAME} --release
