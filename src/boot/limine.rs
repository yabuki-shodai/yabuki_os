//! Limine 12.9.6 protocol declarations used by this kernel.
//! Based on limine-protocol/include/limine.h (0BSD, Mintsuki and contributors).
use core::ptr;

// 参考：https://github.com/limine-bootloader/limine-protocol/blob/trunk/PROTOCOL.md#request
const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];
pub const FRAMEBUFFER_RGB: u8 = 1;
pub const FIRMWARE_TYPE_EFI64: u64 = 2;

#[repr(C)]
pub struct Framebuffer {
    pub address: *mut u8,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub unused: [u8; 7],
    pub edid_size: u64,
    pub edid: *mut u8,
    pub mode_count: u64,
    pub modes: *mut u8,
    pub upright_rotation: u64,
}

#[repr(C)]
pub struct FramebufferResponse {
    pub revision: u64,
    pub framebuffer_count: u64,
    pub framebuffers: *const *const Framebuffer,
}

#[repr(C)]
pub struct FramebufferRequest {
    id: [u64; 4],
    revision: u64,
    response: *const FramebufferResponse,
}

#[repr(C)]
pub struct FirmwareTypeResponse {
    pub revision: u64,
    pub firmware_type: u64,
}

#[repr(C)]
pub struct FirmwareTypeRequest {
    id: [u64; 4],
    revision: u64,
    response: *const FirmwareTypeResponse,
}

#[used]
#[link_section = ".limine_requests_start"]
static REQUESTS_START: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
];

// LimineがBase Revision 6に対応していれば、最後の6を0に書き換えられます。
// 参考：https://github.com/limine-bootloader/limine-protocol/blob/trunk/PROTOCOL.md#base-revisions
#[used]
#[link_section = ".limine_requests"]
static mut BASE_REVISION: [u64; 3] = [
    0xf9562b2d5c95a6c8, // 識別番号1
    0x6a7b384944536bdc, // 識別番号2
    6,                  // 使いたいバージョン
];

#[used]
#[link_section = ".limine_requests"]
static mut FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest {
    id: [
        COMMON_MAGIC[0],
        COMMON_MAGIC[1],
        0x9d5827dcd881dd75,
        0xa3148604f6fab11b,
    ],
    revision: 0,
    response: ptr::null(),
};

// Limineに起動方式を問い合わせるための識別番号
// 参考:https://github.com/limine-bootloader/limine-protocol/blob/trunk/PROTOCOL.md#bootloader-info-feature
const FIRMWARE_REQUEST_ID: [u64; 4] = [
    COMMON_MAGIC[0],
    COMMON_MAGIC[1],
    0x8c2f75d90bef28a8,
    0x7045a4688eac00c3,
];

// Limineへの要求データ
#[used]
#[link_section = ".limine_requests"]
static mut FIRMWARE_REQUEST: FirmwareTypeRequest = FirmwareTypeRequest {
    id: FIRMWARE_REQUEST_ID,
    revision: 0,
    response: ptr::null(),
};

#[used]
#[link_section = ".limine_requests_end"]
static REQUESTS_END: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];

// サポートしているかを確認
pub fn base_revision_supported() -> bool {
    unsafe {
        // BASE_REVISIONの先頭アドレスを取得
        let base = ptr::addr_of!(BASE_REVISION);

        // 16バイト先のアドレスを取得(識別番号1,2がそれぞれ8バイトずつなので16バイト目を確認)
        let status = base.cast::<u64>().add(2);

        // 値を読み取る
        let value = ptr::read_volatile(status);

        // 0なら対応している
        value == 0
    }
}

// LimineからPCの起動方式を取得する
pub fn firmware_type() -> Option<u64> {
    // Limineからの返答ポインタを取得
    let response_ptr = unsafe { ptr::read_volatile(ptr::addr_of!(FIRMWARE_REQUEST.response)) };

    // ポインタを参照に変換（nullならNone）
    let response = unsafe { response_ptr.as_ref() };

    // 返答から起動方式を取得
    let firmware_type = response.map(|data| data.firmware_type);

    firmware_type
}

pub fn framebuffer() -> Option<&'static Framebuffer> {
    // Limineからの返答ポインタを取得
    let response_ptr = unsafe { ptr::read_volatile(ptr::addr_of!(FRAMEBUFFER_REQUEST.response)) };

    // 返答ポインタを参照に変換
    let response = unsafe { response_ptr.as_ref()? };

    // フレームバッファが存在するか確認
    let has_framebuffer = response.framebuffer_count > 0;
    let has_framebuffer_list = !response.framebuffers.is_null();

    if !has_framebuffer || !has_framebuffer_list {
        return None;
    }

    // 最初のフレームバッファを取得
    let first_framebuffer_ptr = unsafe { *response.framebuffers };

    let framebuffer = unsafe { first_framebuffer_ptr.as_ref() };

    framebuffer
}
