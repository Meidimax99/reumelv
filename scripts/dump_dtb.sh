#!/bin/sh
qemu-system-riscv64 -machine virt -machine dumpdtb=qemu.dtb
dtc -I dtb -O dts -o a.dts qemu.dtb
code a.dts