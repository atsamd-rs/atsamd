//! NVM Software Calibration Area Mapping
// For samd11, see 9.5 NVM Software Calibration Area Mapping, page 24
// For samd21, see 10.3.2 NVM Software Calibration Area Mapping, page 46

use atsamd_hal_macros::hal_cfg;
use core::ptr;

const ADDR: u32 = 0x806020u32;

fn cal(word_offset: u32, bit_shift: u32, bit_mask: u32) -> u32 {
    let addr = (ADDR + 4 * word_offset) as *const u32;
    let value = unsafe { ptr::read(addr) };
    (value >> bit_shift) & bit_mask
}

fn cal_with_errata(
    word_offset: u32,
    bit_shift: u32,
    bit_mask: u32,
    bad_val: u32,
    def_val: u32,
) -> u32 {
    let val = cal(word_offset, bit_shift, bit_mask);
    // if the value matches the bad value, use an alternative value
    // specified in the the errata section of the datasheet
    if val == bad_val {
        def_val
    } else {
        val
    }
}

/// ADC Linearity Calibration. Should be written to ADC CALIB register.
#[allow(clippy::unusual_byte_groupings)]
pub fn adc_linearity_cal() -> u8 {
    // Value in flash is bits 34:27, which spans a 32b boundary
    
    // bits 4:0
    let low = cal(0, 27, 0x1F) as u8;

    // bits 7:5
    let high = cal(1, 0, 0x7) as u8;

    high << 5 | low
}

/// ADC Bias Calibration. Should be written to ADC CALIB register.
pub fn adc_bias_cal() -> u8 {
    cal(1, 3, 0b111) as u8
}

/// Returns the osc32k calibration value from the NVM calibration area
pub fn osc32k_cal() -> u8 {
    cal(1, 6, 0x7f) as u8 // 44:38
}

/// Returns the dfll48m coarse calibration value
pub fn dfll48m_coarse_cal() -> u8 {
    cal_with_errata(1, 26, 0x3f, 0x3f, 0x1f) as u8
}

/// USB TRANSN calibration value. Should be written to USB PADCAL register.
pub fn usb_transn_cal() -> u8 {
    cal_with_errata(1, 13, 0x1f, 0x1f, 5) as u8
}

/// USB TRANSP calibration value. Should be written to USB PADCAL register.
pub fn usb_transp_cal() -> u8 {
    cal_with_errata(1, 18, 0x1f, 0x1f, 29) as u8
}

/// USB TRIM calibration value. Should be written to USB PADCAL register.
#[hal_cfg("nvmctrl-d11")]
pub fn usb_trim_cal() -> u8 {
    cal_with_errata(1, 23, 7, 7, 5) as u8
}

/// USB TRIM calibration value. Should be written to USB PADCAL register.
#[hal_cfg("nvmctrl-d21")]
pub fn usb_trim_cal() -> u8 {
    cal_with_errata(1, 23, 7, 7, 3) as u8
}
