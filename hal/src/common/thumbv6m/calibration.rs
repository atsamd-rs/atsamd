//! NVM Software Calibration Area Mapping
// For samd11, see 9.5 NVM Software Calibration Area Mapping, page 24
// For samd21, see 10.3.2 NVM Software Calibration Area Mapping, page 46

use core::ptr;

const ADDR: u32 = 0x806020u32;

fn cal(addr_offset: u32, bit_shift: u32, bit_mask: u32) -> u32 {
    unsafe {
        let addr: *const u32 = (ADDR + addr_offset) as *const _;
        let value = ptr::read(addr);

        (value >> bit_shift) & bit_mask
    }
}

fn cal_with_errata(
    addr_offset: u32,
    bit_shift: u32,
    bit_mask: u32,
    bad_val: u32,
    def_val: u32,
) -> u32 {
    let val = cal(addr_offset, bit_shift, bit_mask);
    // if the value matches the bad value, use an alternative value
    // specified in the the errata section of the datasheet
    if val == bad_val {
        def_val
    } else {
        val
    }
}

/// Returns the osc32k calibration value from the NVM calibration area
pub fn osc32k_cal() -> u8 {
    cal(4, 6, 0x7f) as u8
}

/// Returns the dfll48m coarse calibration value
pub fn dfll48m_coarse_cal() -> u8 {
    cal_with_errata(4, 26, 0x3f, 0x3f, 0x1f) as u8
}

/// USB TRANSN calibration value. Should be written to USB PADCAL register.
pub fn usb_transn_cal() -> u8 {
    cal_with_errata(4, 13, 0x1f, 0x1f, 5) as u8
}

/// USB TRANSP calibration value. Should be written to USB PADCAL register.
pub fn usb_transp_cal() -> u8 {
    cal_with_errata(4, 18, 0x1f, 0x1f, 29) as u8
}

/// USB TRIM calibration value. Should be written to USB PADCAL register.
pub fn usb_trim_cal() -> u8 {
    #[cfg(feature = "samd11")]
    return cal_with_errata(4, 23, 7, 7, 5) as u8;
    #[cfg(feature = "samd21")]
    return cal_with_errata(4, 23, 7, 7, 3) as u8;
}
