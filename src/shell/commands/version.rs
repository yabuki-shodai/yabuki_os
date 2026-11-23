use crate::console::Console;
use crate::drivers::serial;
use crate::OS_VERSION;

pub fn run(console: &mut Console) {
    console.write(OS_VERSION);
    serial::write(OS_VERSION);
    serial::write("\r\n");
}
