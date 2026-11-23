use crate::boot::limine::Framebuffer;
use crate::graphics::framebuffer;
use core::ptr;

const MARGIN: usize = 16;
const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 16;

pub struct Console {
    pub(crate) fb: &'static Framebuffer,
    white: u32,
    pub(crate) x: usize,
    y: usize,
}

impl Console {
    pub fn new(fb: &'static Framebuffer, white: u32) -> Self {
        Self {
            fb,
            white,
            x: MARGIN,
            y: MARGIN,
        }
    }

    // 画面を1行分上にスクロール
    fn scroll(&mut self) {
        self.move_screen_up();
        self.clear_bottom_line();

        self.y -= CHAR_HEIGHT;
    }

    // 画面の内容を1行分上に移動
    fn move_screen_up(&self) {
        let width = self.fb.width as usize;
        let height = self.fb.height as usize;
        let pixels_per_row = self.fb.pitch as usize / 4;
        let framebuffer_ptr = self.fb.address.cast::<u32>();

        for row in 0..height - CHAR_HEIGHT {
            for col in 0..width {
                let source_index = (row + CHAR_HEIGHT) * pixels_per_row + col;

                let destination_index = row * pixels_per_row + col;

                unsafe {
                    let pixel = ptr::read_volatile(framebuffer_ptr.add(source_index));

                    ptr::write_volatile(framebuffer_ptr.add(destination_index), pixel);
                }
            }
        }
    }

    // 画面の最下行を消去
    fn clear_bottom_line(&self) {
        let width = self.fb.width as usize;
        let height = self.fb.height as usize;
        let pixels_per_row = self.fb.pitch as usize / 4;
        let framebuffer_ptr = self.fb.address.cast::<u32>();

        for row in height - CHAR_HEIGHT..height {
            for col in 0..width {
                let pixel_index = row * pixels_per_row + col;

                unsafe {
                    ptr::write_volatile(framebuffer_ptr.add(pixel_index), 0);
                }
            }
        }
    }

    // 改行
    fn newline(&mut self) {
        self.x = MARGIN;
        self.y += CHAR_HEIGHT;

        // 画面下端に達したらスクロール
        if self.y + CHAR_HEIGHT > self.fb.height as usize {
            self.scroll();
        }
    }

    // 1文字表示
    pub fn putc(&mut self, character: u8) {
        // 改行文字
        if character == b'\n' {
            self.newline();
            return;
        }

        // 右端に達したら改行
        let right_edge = self.fb.width as usize - MARGIN;

        if self.x + CHAR_WIDTH > right_edge {
            self.newline();
        }

        // 文字を描画
        framebuffer::draw_char(self.fb, self.x, self.y, character, self.white);

        // 次の文字の描画位置へ移動
        self.x += CHAR_WIDTH;
    }

    // 文字列を表示
    pub fn write(&mut self, text: &str) {
        for character in text.bytes() {
            self.putc(character);
        }
    }

    // 画面を消去して描画位置を初期化
    pub fn clear(&mut self) {
        framebuffer::clear_screen(self.fb);

        self.x = MARGIN;
        self.y = MARGIN;
    }
}
