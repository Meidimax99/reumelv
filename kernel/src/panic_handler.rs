use crate::hardware::uart;
use core::fmt::Write;

#[panic_handler]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    uart::print_str("\n\n\n### System Crash ###\n");
    write!(uart::get_uart(), "{}", info).ok();
    loop {}
}
