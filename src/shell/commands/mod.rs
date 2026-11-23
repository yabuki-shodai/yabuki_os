mod clear;
mod echo;
mod help;
mod version;

use crate::console::Console;

pub fn execute(input: &str, console: &mut Console) -> bool {
    // コマンド名と引数を分離
    let (command, args) = input.split_once(' ').unwrap_or((input, ""));

    // コマンドを実行
    match command {
        "help" => help::run(console),
        "version" => version::run(console),
        "clear" => clear::run(console),
        "echo" => echo::run(args, console),
        _ => return false,
    }

    true
}
