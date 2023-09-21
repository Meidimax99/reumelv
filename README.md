# reumelv
This is a microkernel created at the [University of Bamberg](https://www.uni-bamberg.de/)
as part of a Bachelor/Master project at the chair for system-oriented programming under the  supervision of Professor [Michael Engel](https://www.multicores.org) and scientific assistant Timo Renk. 
# Running the project

### Setup

The simplest way to run the microkernel is to use the [Visual Studio Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers). You only need to have [Docker](https://www.docker.com/) and the [Dev Container Extension](https://github.com/microsoft/vscode-dev-containers) installed. All required tools will be installed using the post-install script ( .devcontainer/postinstall.sh). Since those tools are not shipped directly with the docker image at the moment, the post-install script might take some time to complete.

If you do not want to take the docker route, you will need:
- [Rustup](https://www.rust-lang.org/tools/install) for installing the Rust language itself and for toolchain management
- [Cargo](https://github.com/rust-lang/cargo) for building and package management
- The [Qemu](https://www.qemu.org/) emulator, in which the kernel will be run
- [GDB-Multiarch](https://packages.debian.org/de/sid/gdb-multiarch) for debugging
- [Cargo-Binutils](https://github.com/rust-embedded/cargo-binutils) for invoking the LLVM tools
You will also need to:
- Install the `riscv64gc-unknown-none-elf` target using `rustup target add riscv64gc-unknown-none-elf`
- Add the `llvm-tools-preview` using `rustup component add llvm-tools-preview`

You can also check out the `.devcontainer/postinstall.sh` for the commands used to install those tools in the container.

On Windows it's not as straightforward to install some of those tools and you may need to use something like [MSYS2](https://www.msys2.org/). Therefore I would recommend either using the dev containers or [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) on Windows.

### Tasks for building and running

Once all the required tools are installed, the project can be compiled and run using the provided tasks (Shortcut `Strg+Alt+R`).

- Use `Build riscv_rust_os` to build the binaries for the kernel.
- Use `Build user binaries` to build the binaries for the user processes to be run
- Finally, use `Debug riscv_rust_os` to fire up qemu with the compiled kernel

Now qemu is running with the given binary! But to get anything from the emulator, the visual studio debugger needs to connect to the debug server.
How to connect to the server is already set up in launch.json, so simply pressing `F5` should suffice to connect to the debug server.

To see what instruction is executed at the moment, you can open the `Disassembly View` using the Command Palette (Keyboard Shortcut: `F1`). 

For convenience, there is also the `Build all and Debug` task, which combines all of the above steps into one. 

# Authors

- Michael Engel (michael.engel@uni-bamberg.de)
provided invaluable guidance and knowledge about microkernels, operating systems, hardware and so much more.
- Timo Renk (timo.renk@stud.uni-bamberg.de) provided a very solid foundation of an operating system already running in qemu. This foundation enabled us to dive right into the development of the microkernel, without first having to figure out how to run the kernel in qemu. He also held a series of tutorials on the rust programming language, which was very useful since it was completely new for the rest of us.
- Fabian Adam (fabian-david.adam@stud.uni-bamberg.de)
- Leonhard Kohn (leonhard.kohn@stud.uni-bamberg.de)
- Tobias Treuheit (tobias-niklas.treuheit@stud.uni-bamberg.de)
- Max Meidinger (max_meidinger@stud.uni-bamberg.de)

# Resources 
Here are some resources that were either used for the development of the kernel or might be useful for some further development. This list will be expanded as development progresses.

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
<https://www.lammertbies.nl/comm/info/serial-uart>

## Plic

<https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc>
