// Rustの標準ライブラリ(std)を使わないという宣言
#![no_std]
// 通常のRustプログラムのエントリポイント main() を使わないという宣言
#![no_main]

// ========================================
// モジュール
// ========================================
mod boot;
mod console;
mod drivers;
mod graphics;
mod kernel;
mod shell;

// ========================================
// インポート
// ========================================
use console::Console;
use drivers::serial;
use kernel::panic::fail;

//=========================================
// 定数
//=========================================
const OS_VERSION: &str = "YABUKI OS v1.0.0";
const INPUT_CAPACITY: usize = 64;

#[no_mangle]
// "C"をつけることでABIで呼び出せるようにしている
pub extern "C" fn kernel_main() -> ! {
    // serialの初期化
    if !serial::init() {
        fail("serial initialization failed");
    }
    serial::write("YABUKI: kernel entry\r\n");

    // LimineでYABUKI OSが使えるかの確認
    if !boot::limine::base_revision_supported() {
        fail("unsupported Limine base revision");
    }

    // YABUKI OSがUEFIの64ビット環境で起動されたか確認
    let firmware = boot::limine::firmware_type();
    let expected = Some(boot::limine::FIRMWARE_TYPE_EFI64);

    if firmware != expected {
        fail("x86-64 UEFI firmware required");
    }

    serial::write("[BOOT] UEFI x86-64 detected\r\n");

    // Limineからフレームバッファを取得
    let fb = match boot::limine::framebuffer() {
        Some(framebuffer) => framebuffer,
        None => fail("no framebuffer"),
    };

    // フレームバッファが使用可能か確認
    let is_valid = graphics::framebuffer::valid(fb);
    if !is_valid {
        fail("unsupported framebuffer");
    }

    serial::write("[BOOT] Framebuffer: validated\r\n");

    // 赤・緑・青それぞれの色情報を作成
    let red = graphics::framebuffer::mask_bits(fb.red_mask_size, fb.red_mask_shift);
    let green = graphics::framebuffer::mask_bits(fb.green_mask_size, fb.green_mask_shift);
    let blue = graphics::framebuffer::mask_bits(fb.blue_mask_size, fb.blue_mask_shift);

    // 3色を合成して白色を作成
    let white = red | green | blue;

    // 画面をクリア
    graphics::framebuffer::clear_screen(fb);

    // コンソールを初期化
    let mut console = Console::new(fb, white);

    // OS情報
    console.write("YABUKI OS\n");
    console.write(OS_VERSION);
    console.write("\n\n");

    // 起動環境
    console.write("Architecture : x86_64\n");
    console.write("Firmware     : UEFI\n");
    console.write("Display      : Framebuffer\n");
    console.write("\n");

    // 起動完了
    console.write("System initialized successfully.\n\n");
    console.write("Welcome to YABUKI OS.\n");

    // シリアルログ
    serial::write("[BOOT] Console: initial output complete\r\n");

    // shellを起動
    shell::run(&mut console)
}
