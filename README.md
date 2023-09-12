# Sysnap WS22

## Authors

- Michael Engel (michael.engel@uni-bamberg.de)
- Timo Renk (timo.renk@stud.uni-bamberg.de)
- Fabian Adam (fabian-david.adam@stud.uni-bamberg.de)
- Leonhard Kohn (leonhard.kohn@stud.uni-bamberg.de)
- Tobias Treuheit (tobias-niklas.treuheit@stud.uni-bamberg.de))
- Max Meidinger (max_meidinger@stud.uni-bamberg.de)

## Description

In this project, we attempt to build a mikrokernel in RUST on a RISCV processor. For this we use the code of Timo Renk as a basis.

To run the code, the following is needed:

- gdb-multiarch or riscv-elf-gdb
- rust with rustup and cargo
- qemu for riscv

## Running the project
Once all the required tools are installed, the project can be compiled and run using the provided tasks (Shortcut `Strg+Alt+R`).

- Use `Build riscv_rust_os` to build the binaries for the kernel.
- Use `Build user binaries` to build the binaries for the user processes to be run
- Finally, use `Debug riscv_rust_os` to fire up qemu with the compiled kernel

Now qemu is running with the given binary! But in orderr to get anything from the emulator, the visual studio debugger needs to connect to the debug server.
How to connect to the server is already set up in launch.json, so simply pressing `F5` should suffice to connect to the debug server.

To see what instruction is executed at the moment, you can open the `Disassembly View` using the Command Palette (Keyboard Shortcut: `F1`). 

#
## License

This is a University Project.

#

#



# Risc-V Rust OS

Os written in Rust

## Install

- rustup target install riscv64gc-unknown-none-elf

### GDB

gdb-multiarch/ riscv-elf-gdb

#### Windows

msys2: -> pacman -S mingw-w64-x86_64-toolchain

### objcopy

cargo install cargo-binutils
rustup component add llvm-tools-preview

## RISC-V

[Register]<https://en.wikichip.org/wiki/risc-v/registers>
<https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#system-reset-extension-eid-0x53525354-srst>
<https://github.com/riscv/riscv-isa-manual/#readme>
<https://github.com/rust-embedded/riscv>
[Register]

## UART

<https://osblog.stephenmarz.com/ch0.html>
<https://os.phil-opp.com/>
<https://github.com/sgmarz/osblog/blob/master/risc_v/src/lds/virt.lds>
<https://github.com/skyzh/core-os-riscv/blob/master/kernel/src/uart.rs>
<https://docs.rust-embedded.org/book/start/qemu.html>

UART
<https://www.lammertbies.nl/comm/info/serial-uart>

Check riscv reader for paper info for register infos in first two lectures

## Plic

<https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc>

## Questions

- How to avoid race-conditions in UART/ Kernel?
- What is mtval?

### Answered

- Why align to 16?
  - `ALIGN(4096) tells the linker to align the current memory location (which is
       0x8000_0000 + text section + rodata section) to 4096 bytes. This is because our paging
       system's resolution is 4,096 bytes or 4 KiB.`
- >ram AT>ram?
- sdata .sbss
- use wfi?
  - Wait for interrupts

## GDB

- info registers

<https://stackoverflow.com/questions/2420813/using-gdb-to-single-step-assembly-code-outside-specified-executable-causes-error>

- gdbtui. Or run gdb with the -tui switch. Or press C-x C-a after entering gdb.
- layout asm
- Press C-x s
- use si ni
- use gdb-multiarch!
- x/100x $sp
- -exec p/x $mepc

readelf -a user_1 | less

## LLDB

Don't use it!
<https://lldb.llvm.org/use/map.html>

## Tools

### NM



Check memory layout
```x86_64-w64-mingw32-gcc-nm riscv_rust_os | sort```
