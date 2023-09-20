#!/bin/sh
#remove carriage returns ^M  sed -i -e 's/\r$//' ./.devcontainer/postinstall.sh
echo "Installing Dependencies..."

echo "Updating package list"
sudo apt-get update 

echo "Installing cargo"
sudo apt-get install -y cargo

echo "Installing riscv64gc target"
rustup target add riscv64gc-unknown-none-elf

echo "Installing cargo binutils"
sudo cargo install cargo-binutils

echo "Add llvm tools"
rustup component add llvm-tools-preview

echo "Installing qemu system"
sudo apt-get install -y qemu-system 

echo "Installing gdb multiarch"
sudo apt-get install -y gdb-multiarch

echo "Install device tree analyzer"
sudo apt-get install device-tree-compiler