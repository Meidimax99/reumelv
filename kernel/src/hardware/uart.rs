use super::binary_struct::{BinaryStruct, Byte};
use super::memory_mapping::MemoryMapping;

const BASE_ADDR: usize = 0x1000_0000;

const RECEIVE_BIT: usize = 0;
const TRANSMIT_BIT: usize = 1;
const RECEIVER_LINE_STATUS_BIT: usize = 2;
const RECEIVER_TRANSMIT_STATUS_BIT: usize = 3;

/// Reserving address space for uart at [Base_ADDR].
/// This includes address space for the register:
/// - rbr_thr_dll = Receive Buffer Register
/// - ier_dlm = Interrupt Enable Register
/// - isr_fcr = Interrupt Status Register
/// - lcr = Line Control Register
/// - mcr = Modem Control Register
/// - lsr = Line Status Register
/// - msr = Memory Status Register
/// - scr = Scratched Register Read/Write
static mut UART: UART = UART {
    reg: UartRegister::new(BASE_ADDR),
};

/// Initializes the interrupts for uart in the ier_dlm register.
pub unsafe fn init() {
    let mem_ier = &mut UART.reg.ier_dlm;
    let mut ier = BinaryStruct::from(0);
    ier.at(RECEIVE_BIT, true); // receive interrupt
    ier.at(TRANSMIT_BIT, false); // transmit interrupt
    ier.at(RECEIVER_LINE_STATUS_BIT, false); // receiver line status interrupt
    ier.at(RECEIVER_TRANSMIT_STATUS_BIT, false); // receiver transmit status interrupt
    mem_ier.write(ier);
}

/// Only call if an interrupt happened. Returns the char.
pub unsafe fn read_char() -> char {
    UART.get_char()
}

/// print a str over uart on the terminal
pub unsafe fn print_str(str: &str) {
    for c in str.chars() {
        UART.print_char(c);
    }
}
/// print a char over uart on the terminal
pub unsafe fn print_char(char: char) {
    UART.print_char(char);
}
/// get a char from the user over uart
pub unsafe fn get_uart() -> &'static mut UART {
    &mut UART
}

/// implementation for print_char, get_char
#[allow(clippy::upper_case_acronyms)]
pub struct UART {
    reg: UartRegister,
}

impl UART {
    /// Print a char if the lsr is free (the bit 5 is set)
    fn print_char(&mut self, char: char) {
        unsafe {
            // Loop until char is send to the buffer register
            loop {
                let lsr = self.reg.lsr.read();
                //Check if we can overwrite the buffer register
                if lsr.is_set(5) {
                    self.reg.rbr_thr_dll.write(char as u8);
                    return;
                }
            }
        }
    }

    unsafe fn get_char(&mut self) -> char {
        let lsr = &self.reg.lsr;
        while !lsr.read().is_set(0) {}
        self.reg.rbr_thr_dll.read() as char
    }
}

/// implementation for write_str
impl core::fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.print_char(c as char);
        }
        Ok(())
    }
}

#[allow(dead_code)]
struct UartRegister {
    /// Receive Buffer Register, Transmit Holding Register | LSB of Divisor Latch when enabled.
    rbr_thr_dll: MemoryMapping<u8>,
    /// N/A, Interrupt Enable Register | MSB of Divisor Latch when enabled. previous: ier_dlm
    ier_dlm: MemoryMapping<Byte>,
    /// Interrupt Status Register, FIFO control Register
    isr_fcr: MemoryMapping<Byte>,
    /// N/A, Line Control Register
    lcr: MemoryMapping<Byte>,
    /// N/A, Modem Control Register
    mcr: MemoryMapping<Byte>,
    /// Line Status Register, N/A
    lsr: MemoryMapping<Byte>,
    /// Modem Status Register, N/A
    msr: MemoryMapping<Byte>,
    /// Scratchpad Register Read, Scratchpad Register Write
    scr: MemoryMapping<Byte>,
}
impl UartRegister {
    const fn new(addr: usize) -> Self {
        let rhr_thr_dll = MemoryMapping::new(addr);
        let ier_dlm = MemoryMapping::new(addr + 1);
        let isr_fcr = MemoryMapping::new(addr + 2);
        let lcr = MemoryMapping::new(addr + 3);
        let mcr = MemoryMapping::new(addr + 4);
        let lsr = MemoryMapping::new(addr + 5);
        let msr = MemoryMapping::new(addr + 6);
        let scr = MemoryMapping::new(addr + 7);
        UartRegister {
            rbr_thr_dll: rhr_thr_dll,
            ier_dlm,
            isr_fcr,
            lcr,
            mcr,
            lsr,
            msr,
            scr,
        }
    }
}
