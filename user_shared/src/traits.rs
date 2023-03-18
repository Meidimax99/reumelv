include!("sys_call.rs");

/// This trait is for a print with other datatypes. It works with str, char and usize.
/// All usize-input will transform to a hexadecimal output.
///
/// Examples for print:
///
/// print!("Hello World\n");
/// print!('c');
/// print!(1024);
///
/// The Output is:
/// Hello World
/// c0x0000000000000400
pub trait Print {
    fn print(&self);
}
/// The print for a str is implemented with a system_call.
/// The str is implemented with a system_call and transmit a pointer from the str and the length from the str.
impl Print for str {
    fn print(&self) {
        unsafe {
            system_call(SysCall::Print, self.as_ptr() as usize, self.len());
        }
    }
}

/// The print for a char is implemented with a system_call.
/// It transmit a pointer and the length of the char (1).
impl Print for char {
    fn print(&self) {
        // first build a buffer
        let mut slice: [u8; 4] = [0, 0, 0, 0];
        // convert buffer to string slice
        let char_string = self.encode_utf8(&mut slice);
        unsafe {
            system_call(
                SysCall::Print,
                char_string.as_ptr() as usize,
                char_string.len(),
            );
        }
    }
}

/// The print for [usize] is implemented with a system_call.
/// It transmit a pointer and the length of the array.
const HEX_SLICE_LENGTH: usize = 18;
impl Print for usize {
    fn print(&self) {
        let slice_length = 2 + (usize::BITS / 4) as usize;
        let mut shift_bits = (usize::BITS - 4) as i64;
        // creates a array with the beginning "0x"
        let mut hex_slice: [u8; 2 + (usize::BITS / 4) as usize] =
            [0; 2 + (usize::BITS / 4) as usize];
        hex_slice[0] = b'0';
        hex_slice[1] = b'x';

        // for every number convert it in hex
        (2..slice_length).for_each(|j| {
            // Allocation of the last 4 bits to a variable (one hex number)
            let d = (self >> shift_bits) & 0x0f;
            // if d is smaller than 10, save the number
            let s = if d < 10 {
                d as u8 + b'0'
            // if d is bigger or equal then 10, convert the number to a letter
            } else {
                d as u8 - 10 + b'a'
            };
            hex_slice[j] = s;
            shift_bits -= 4;
        });
        // print the array to the consol over uart
        unsafe {
            system_call(SysCall::Print, hex_slice.as_ptr() as usize, hex_slice.len());
        }
    }
}

impl Print for *const u8 {
    fn print(&self) {
        (*self as usize).print();
    }
}
