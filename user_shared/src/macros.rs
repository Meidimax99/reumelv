/// This macro initiates a syscall to print something via the [uart].
/// The input can be any expression.
/// However, the type of the expression must implement the [Print] trait.
/// The Print trait is defined in '[user_shared/traits.rs]'
#[allow(unused)]
#[macro_export]
macro_rules! sys_print {
    ($($arg:expr), +) => {
        $($arg.print();) +
    }
}
#[allow(unused)]
pub use sys_print;

/// Prints the given expressions and makes a line break after each expression.
/// Currently, this function depends on syscalls.
/// To be precise, this macro uses the macro [sys_print] with its dependency on the [Print](crate::traits::Print) trait.
/// TODO: Rewrite this macro after a successful UART-User-Process implementation.
#[allow(unused)]
#[macro_export]
macro_rules! println {
    ($($arg:expr), +) => {
        $(sys_print!($arg, "\n");) +
    };
    () => {
        sys_print!("\n");
    };
}
#[allow(unused)]
pub use println;
