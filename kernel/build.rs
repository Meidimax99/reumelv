//This build script is exectuted when the module is build with cargo,
//This will happen if you execute the "build riscv_rust_os" task
fn main() {
    //with the println! macro, cargo can be given specific instructions for different aspects of the build
    //More on that here:
    //https://doc.rust-lang.org/cargo/reference/build-scripts.html

    //rustc-link-arg tells cargo to pass custom flags to compiler
    //https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg
    //more specific: rustc-link-arg will pass the -C link-arg=FLAG flag to the compile
    //-C is the codegen flag, which can modify the way the code is generated
    //link-arg will append a given flag to the linker invocation
    println!("cargo:rustc-link-arg=-Tkernel/src/lds/kernel.lds");

    //In this case  "-Tkernel/src/lds/kernel.lds" is appended to the linker invocation
    //The default rust linker is the c linker cc
}
