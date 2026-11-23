use crate::console::Console;
use crate::drivers::serial;

const MESSAGE: &str = "help - list commands\nversion - show version\necho TEXT - print TEXT";

pub fn run(console: &mut Console) {
    console.write(MESSAGE);
    for line in MESSAGE.split('\n') {
        serial::write(line);
        serial::write("\r\n");
    }
}
