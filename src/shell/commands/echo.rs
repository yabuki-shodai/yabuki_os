use crate::console::Console;
use crate::drivers::serial;

pub fn run(text: &str, console: &mut Console) {
    console.write(text);
    serial::write(text);
    serial::write("\r\n");
}
