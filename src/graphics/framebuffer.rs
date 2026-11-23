use super::font::glyph_rows;
use crate::boot::limine::{self, Framebuffer};
use core::ptr;

fn mask_valid(size: u8, shift: u8) -> bool {
    size > 0 && size <= 8 && shift < 32 && u16::from(size) + u16::from(shift) <= 32
}

pub fn mask_bits(size: u8, shift: u8) -> u32 {
    ((1u32 << size) - 1) << shift
}

pub fn valid(fb: &Framebuffer) -> bool {
    if fb.address.is_null()
        || fb.width < 160
        || fb.height < 64
        || fb.bpp != 32
        || fb.memory_model != limine::FRAMEBUFFER_RGB
    {
        return false;
    }
    if !mask_valid(fb.red_mask_size, fb.red_mask_shift)
        || !mask_valid(fb.green_mask_size, fb.green_mask_shift)
        || !mask_valid(fb.blue_mask_size, fb.blue_mask_shift)
    {
        return false;
    }
    let r = mask_bits(fb.red_mask_size, fb.red_mask_shift);
    let g = mask_bits(fb.green_mask_size, fb.green_mask_shift);
    let b = mask_bits(fb.blue_mask_size, fb.blue_mask_shift);
    if r & g != 0 || r & b != 0 || g & b != 0 {
        return false;
    }
    if fb.width > usize::MAX as u64 / 4
        || fb.pitch < fb.width * 4
        || fb.pitch > usize::MAX as u64 / fb.height
    {
        return false;
    }
    let length = (fb.pitch * fb.height) as usize;
    (fb.address as usize).checked_add(length).is_some()
        && (fb.address as usize) & 3 == 0
        && fb.pitch & 3 == 0
}

fn pixel(fb: &Framebuffer, x: usize, y: usize, color: u32) {
    if x >= fb.width as usize || y >= fb.height as usize {
        return;
    }
    let offset = y * fb.pitch as usize + x * 4;
    unsafe {
        ptr::write_volatile(fb.address.add(offset).cast::<u32>(), color);
    }
}

pub fn clear_screen(fb: &Framebuffer) {
    for y in 0..fb.height as usize {
        for x in 0..fb.width as usize {
            pixel(fb, x, y, 0);
        }
    }
}

pub fn draw_char(fb: &Framebuffer, x: usize, y: usize, c: u8, foreground: u32) {
    let rows = glyph_rows(c);

    // 文字の背景を黒くする
    for row in 0..16 {
        for col in 0..8 {
            pixel(fb, x + col, y + row, 0);
        }
    }

    // 文字を描画する
    for (row, bits) in rows.iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                pixel(fb, x + col + 1, y + row * 2, foreground);
                pixel(fb, x + col + 1, y + row * 2 + 1, foreground);
            }
        }
    }
}
