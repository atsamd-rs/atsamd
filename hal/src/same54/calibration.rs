//! NVM Software Calibration Area Mapping
// See 9.5 NVM Software Calibration Area Mapping, page 55

use core::ptr;

// "The NVM Software Calibration Area can be read at address 0x00800080."
const ADDR: u32 = 0x00800080;

fn cal(addr_offset: u32, bit_shift: u32, bit_mask: u32) -> u32 {
    unsafe {
        let addr: *const u32 = (ADDR + addr_offset) as *const _;
        let value = ptr::read(addr);

        (value >> bit_shift) & bit_mask
    }
}

/// USB TRANSN calibration value. Should be written to USB PADCAL register.
pub fn usb_transn_cal() -> u8 {
    cal(4, 0, 0b11111) as u8
}

/// USB TRANSP calibration value. Should be written to USB PADCAL register.
pub fn usb_transp_cal() -> u8 {
    cal(4, 5, 0b11111) as u8
}

/// USB TRIM calibration value. Should be written to USB PADCAL register.
pub fn usb_trim_cal() -> u8 {
    cal(4, 10, 0b111) as u8
}
