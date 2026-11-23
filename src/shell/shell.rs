use crate::console::Console;
use crate::drivers::{keyboard, serial};
use crate::INPUT_CAPACITY;
use core::arch::asm;

use super::commands;

pub fn run(console: &mut Console) -> ! {
    // 入力バッファを初期化
    let mut input = [0u8; INPUT_CAPACITY];
    let mut input_length = 0;

    // 画面幅に応じて最大入力文字数を設定
    let max_input_length = ((console.fb.width as usize - 32) / 8 - 9).min(INPUT_CAPACITY - 1);

    // キーボードの拡張キー判定用
    let mut extended = false;

    // プロンプトを表示
    console.write("yabuki> ");
    console.putc(b'_');

    // キーボード入力を監視
    loop {
        let key = keyboard::poll(&mut extended);

        // 入力がなければ次のループへ
        if key == 0 {
            unsafe {
                asm!("pause", options(nomem, nostack));
            }
            continue;
        }

        // カーソルを消去
        console.x -= 8;
        console.putc(b' ');
        console.x -= 8;

        // Backspace
        if key == b'\x08' {
            if input_length > 0 {
                input_length -= 1;

                console.x -= 8;
                console.putc(b' ');
                console.x -= 8;
            }
        }
        // Enter
        else if key == b'\n' {
            console.putc(b'\n');

            serial::write("YABUKI CLI: ");

            // 入力バッファを文字列に変換
            if let Ok(command) = core::str::from_utf8(&input[..input_length]) {
                serial::write(command);
                serial::write("\r\n");

                // コマンドを実行
                let has_input = input_length > 0;
                let command_found = commands::execute(command, console);

                if has_input && !command_found {
                    console.write("unknown command");
                    serial::write("unknown command\r\n");
                }
            }

            // 次の入力を準備
            console.putc(b'\n');
            input_length = 0;
            console.write("yabuki> ");
        }
        // 通常の文字入力
        else if input_length < max_input_length {
            input[input_length] = key;
            input_length += 1;

            console.putc(key);
        }

        // カーソルを再表示
        console.putc(b'_');
    }
}
