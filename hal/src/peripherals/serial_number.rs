//! Serial number

use atsamd_hal_macros::hal_cfg;
use core::ptr;

// See  9.6   Memories --> Serial Number, page 24 for samd11
// See 10.3.3 Memories --> Serial Number, page 45 for samd21
#[hal_cfg(any("serial-numbers-d11", "serial-numbers-d21"))]
const SN_1: u32 = 0x0080A00C;
#[hal_cfg(any("serial-numbers-d11", "serial-numbers-d21"))]
const SN_2: u32 = 0x0080A040;
#[hal_cfg(any("serial-numbers-d11", "serial-numbers-d21"))]
const SN_3: u32 = 0x0080A044;
#[hal_cfg(any("serial-numbers-d11", "serial-numbers-d21"))]
const SN_4: u32 = 0x0080A048;

// See 9.6 Memories --> Serial Number, page 60
#[hal_cfg("serial-numbers-d5x")]
const SN_1: u32 = 0x008061FC;
#[hal_cfg("serial-numbers-d5x")]
const SN_2: u32 = 0x00806010;
#[hal_cfg("serial-numbers-d5x")]
const SN_3: u32 = 0x00806014;
#[hal_cfg("serial-numbers-d5x")]
const SN_4: u32 = 0x00806018;

/// Returns the serial number of the chip as 4 32-bit integers. The serial
/// number is only guaranteed to be unique if all 128 bits are used.
pub fn split_serial_number() -> (u32, u32, u32, u32) {
    unsafe {
        (
            ptr::read(SN_1 as *const u32),
            ptr::read(SN_2 as *const u32),
            ptr::read(SN_3 as *const u32),
            ptr::read(SN_4 as *const u32),
        )
    }
}

/// Returns the serial number of the chip as an array of bytes. The serial
/// number is only guaranteed to be unique if all 16 bytes are used.
pub fn serial_number() -> [u8; 16] {
    let sn = split_serial_number();
    [
        ((sn.0 >> 24) & 0xff) as u8,
        ((sn.0 >> 16) & 0xff) as u8,
        ((sn.0 >> 8) & 0xff) as u8,
        (sn.0 & 0xff) as u8,
        ((sn.1 >> 24) & 0xff) as u8,
        ((sn.1 >> 16) & 0xff) as u8,
        ((sn.1 >> 8) & 0xff) as u8,
        (sn.1 & 0xff) as u8,
        ((sn.2 >> 24) & 0xff) as u8,
        ((sn.2 >> 16) & 0xff) as u8,
        ((sn.2 >> 8) & 0xff) as u8,
        (sn.2 & 0xff) as u8,
        ((sn.3 >> 24) & 0xff) as u8,
        ((sn.3 >> 16) & 0xff) as u8,
        ((sn.3 >> 8) & 0xff) as u8,
        (sn.3 & 0xff) as u8,
    ]
}
