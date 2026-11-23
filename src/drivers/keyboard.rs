use crate::drivers::serial::in8;

// QEMU exposes a translated PS/2 scan set 1 keyboard. USB HID is separate.
fn key_for_scan(scan: u8) -> u8 {
    match scan {
        0x02..=0x0a => b'1' + scan - 0x02,
        0x0b => b'0',
        0x10 => b'q',
        0x11 => b'w',
        0x12 => b'e',
        0x13 => b'r',
        0x14 => b't',
        0x15 => b'y',
        0x16 => b'u',
        0x17 => b'i',
        0x18 => b'o',
        0x19 => b'p',
        0x1e => b'a',
        0x1f => b's',
        0x20 => b'd',
        0x21 => b'f',
        0x22 => b'g',
        0x23 => b'h',
        0x24 => b'j',
        0x25 => b'k',
        0x26 => b'l',
        0x2c => b'z',
        0x2d => b'x',
        0x2e => b'c',
        0x2f => b'v',
        0x30 => b'b',
        0x31 => b'n',
        0x32 => b'm',
        0x39 => b' ',
        _ => 0,
    }
}

pub fn poll(extended: &mut bool) -> u8 {
    let status = unsafe { in8(0x64) };
    if status & 1 == 0 {
        return 0;
    }
    let scan = unsafe { in8(0x60) };
    if status & 0xe0 != 0 {
        return 0;
    }
    if scan == 0xe0 || scan == 0xe1 {
        *extended = true;
        return 0;
    }
    if *extended {
        *extended = false;
        return 0;
    }
    if scan & 0x80 != 0 {
        return 0;
    }
    match scan {
        0x1c => b'\n',
        0x0e => b'\x08',
        _ => key_for_scan(scan),
    }
}
