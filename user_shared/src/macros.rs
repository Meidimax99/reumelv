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

//Message Macros
//TODO Macro to infer array size
//Create Array of fitting size for registers in macro

/*Macro to write the values stored in an array in order to given registers
Example call:
   write_array_to_registers_rec!( arrayident ; "s0" , "s1")

   The array needs to be big enough to write a value in every register
*/
#[allow(unused)]
macro_rules! write_array_to_registers_rec {
    //End Case
    ($n:expr ; $array:ident ;) => {};

    //Recursion Case
    ($n:expr ; $array:ident ; $reg:literal $(, $rest:tt)*) => {
        riscv::write_function_reg!($array[$n] as u64=> $reg);
        write_array_to_registers!($n + 1 ; $array ; $($rest),*);
    };

    //Initial Case
    ($array:ident ;  $($registers:tt),+) => { write_array_to_registers!(0 ; $array ; $($registers),*); };
}

//Same as the previous macro, but works iterative internally instead of recursive
#[allow(unused)]
macro_rules! write_array_to_registers {
    ($array:ident ; $($reg:literal),+) => {
        let mut i = 0;
        $(
            riscv::write_function_reg!($array[i] as u64 => $reg);
            i += 1;
        )+
    }
}

//This macro loads registers into a given array of sufficient size
#[allow(unused)]
macro_rules! load_registers_into_array {
    ($array:ident ; $($reg:literal),+) => {
        let mut i = 0;
        let mut temp;
        $(
            riscv::read_function_reg!( $reg => temp);
            $array[i] = temp;
            i += 1;
        )+
    }
}
