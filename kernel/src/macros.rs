/// The macro print!() takes a string as input and prints it on the terminal.
/// You can also print an argument with curly brackets in the string.
/// This printing is done without a line break.
///
/// For example:
/// print!("Hello world");
/// let world = "world";
/// print!("Hello {}", world);
/// print!("{}", uart::read_char());
///
/// The output should be on input of "Test":
/// Hello worldHello worldTest
#[allow(unused)]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!(crate::hardware::uart::get_uart(), $($arg)*).ok()}
}
#[allow(unused)]
pub(crate) use print;

/// The macro println!() takes a string as input and prints it on the terminal.
/// You can also print an argument with curly brackets in the string.
/// This printing is done with a line break.
///
/// For example:
/// println!("Hello world");
/// let world = "world";
/// println!("Hello {}", world);
/// println!("{}", uart::read_char());
///
/// The output should be on input of "Test":
/// Hello world
/// Hello world
/// Test
#[allow(unused)]
macro_rules! println {
    ($($arg:tt)*) => {
        crate::hardware::uart::print_char('\n');
        use core::fmt::Write;
        write!(crate::hardware::uart::get_uart(), $($arg)*).ok()}
}
#[allow(unused)]
pub(crate) use println;

/// The macro enum_matching!() gets a number and an enumeration (enum) as input.
/// It checks whether the number is the enum.
/// If it is, the enum is returned.
///
/// For example:
/// enum_matching!(claim: IRQ::Uart);
#[allow(unused)]
macro_rules! enum_matching {
    ($num:ident: $($enum:expr), +) => {
        $(if $num == $enum as usize {
            return $enum;
        }) +
    };
}

pub(crate) use enum_matching;

#[allow(unused)]
macro_rules! log {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        #[cfg(debug)] write!(crate::hardware::uart::get_uart(), $($arg)*).ok()
    }

}
pub(crate) use log;
