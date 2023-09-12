#!/bin/sh
#remove carriage returns ^M  sed -i -e 's/\r$//' ./devcontainer/postinstall.sh
echo "Installing Dependencies..."

sudo apt-get update 

sudo apt-get install -y cargo

rustup target add riscv64gc-unknown-none-elf

sudo cargo install -y cargo-binutils

rustup component add llvm-tools-preview

sudo apt-get install -y qemu-system 

sudo apt-get install -y gdb-multiarch