#!/bin/bash

echo "Installing Dependencies..."

sudo apt-get install cargo

rustup target add riscv64gc-unknown-none-elf

sudo cargo install cargo-binutils

rustup component add llvm-tools-preview

sudo apt-get install -y qemu-system 

sudo apt-get install -y gdb-multiarch