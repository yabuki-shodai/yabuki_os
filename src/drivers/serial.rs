use core::arch::asm;

// シリアルポート
const COM1: u16 = 0x3f8; // 1016

const DATA_REGISTER: u16 = 0; // データの送受信
const INTERRUPT_ENABLE_REGISTER: u16 = 1; // 割り込みの有効・無効
const FIFO_CONTROL_REGISTER: u16 = 2; // FIFOの設定
const LINE_CONTROL_REGISTER: u16 = 3; // 通信形式の設定
const MODEM_CONTROL_REGISTER: u16 = 4; // 制御信号の設定
const LINE_STATUS_REGISTER: u16 = 5; // 送信可能かなどの状態確認
const TRANSMIT_READY: u8 = 0x20; // 送信保持レジスタが空であることを示すビット

pub fn init() -> bool {
    // 参考：https://wiki.osdev.org/Serial_Ports
    unsafe {
        // シリアル割り込みを無効化
        out8(COM1 + INTERRUPT_ENABLE_REGISTER, 0x00);

        // DLABを有効化して、通信速度の設定モードに切り替える
        out8(COM1 + LINE_CONTROL_REGISTER, 0x80);

        // divisor = 1 に設定し、通信速度を115200 baudにする
        out8(COM1 + DATA_REGISTER, 0x01);
        out8(COM1 + INTERRUPT_ENABLE_REGISTER, 0x00);

        // 8ビット、パリティなし、ストップビット1に設定
        out8(COM1 + LINE_CONTROL_REGISTER, 0x03);

        // FIFOを有効化し、送受信FIFOをクリア
        out8(COM1 + FIFO_CONTROL_REGISTER, 0xC7);

        // DTR / RTS / OUT2を有効化
        out8(COM1 + MODEM_CONTROL_REGISTER, 0x0B);

        // ループバックモードに切り替える
        out8(COM1 + MODEM_CONTROL_REGISTER, 0x1E);

        // 動作確認用のテスト値を送信
        out8(COM1 + DATA_REGISTER, 0xAE);

        // 送信した値が返ってくるか確認
        if in8(COM1 + DATA_REGISTER) != 0xAE {
            return false;
        }

        // 通常動作モードに戻す
        out8(COM1 + MODEM_CONTROL_REGISTER, 0x0F);
    }

    true
}

pub fn write(text: &str) {
    for byte in text.bytes() {
        for _ in 0..100_000 {
            if unsafe { in8(COM1 + LINE_STATUS_REGISTER) } & TRANSMIT_READY != 0 {
                break;
            }
        }

        unsafe {
            // シリアルポートにデータを書き込み
            out8(COM1 + DATA_REGISTER, byte);
        }
    }
}

unsafe fn out8(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub(crate) unsafe fn in8(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    value
}
