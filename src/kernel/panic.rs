use core::arch::asm;
use core::panic::PanicInfo;

use crate::drivers::serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    fail("Rust panic")
}

pub fn fail(reason: &str) -> ! {
    serial::write("YABUKI FAIL: ");
    serial::write(reason);
    serial::write("\r\n");
    halt()
}

fn halt() -> ! {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
