//! NVM Software Calibration Area Mapping
// For samd11, see 9.5 NVM Software Calibration Area Mapping, page 24
// For samd21, see 10.3.2 NVM Software Calibration Area Mapping, page 46


use atsamd_hal_macros::hal_cfg;
use core::ptr;

const ADDR: u32 = 0x806020u32;

// Calculations for this from the datasheet values:
// Example: Bit position: 41:37
//
// addr_offset = int(37/8) = 4
// bit_shift = 37 % 8 = 5
// Mask length = 41-37+1 = 5 bits (0b11111)
fn cal(addr_offset: u32, bit_shift: u32, bit_mask: u32) -> u32 {
    unsafe {
        let addr: *const u32 = (ADDR + addr_offset) as *const _;
        let value = ptr::read_unaligned(addr);

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

/// ADC Linearity Calibration. Should be written to ADC CALIB register.
#[allow(clippy::unusual_byte_groupings)]
pub fn adc_linearity_cal() -> u8 {
    cal(3, 3, 0b11111111) as u8
}

/// ADC Bias Calibration. Should be written to ADC CALIB register.
pub fn adc_bias_cal() -> u8 {
    cal(4, 3, 0b111) as u8
}

/// Returns the osc32k calibration value from the NVM calibration area
pub fn osc32k_cal() -> u8 {
    cal(4, 6, 0b1111111) as u8
}

/// Returns the dfll48m coarse calibration value
pub fn dfll48m_coarse_cal() -> u8 {
    cal_with_errata(7, 2, 0b111111, 0x3f, 0x1f) as u8
}

/// USB TRANSN calibration value. Should be written to USB PADCAL register.
pub fn usb_transn_cal() -> u8 {
    cal_with_errata(5, 5, 0b11111, 0x1f, 5) as u8
}

/// USB TRANSP calibration value. Should be written to USB PADCAL register.
pub fn usb_transp_cal() -> u8 {
    cal_with_errata(6, 2, 0b11111, 0x1f, 29) as u8
}

/// USB TRIM calibration value. Should be written to USB PADCAL register.
#[hal_cfg("nvmctrl-d11")]
pub fn usb_trim_cal() -> u8 {
    cal_with_errata(6, 7, 0b111, 7, 5) as u8
}

/// USB TRIM calibration value. Should be written to USB PADCAL register.
#[hal_cfg("nvmctrl-d21")]
pub fn usb_trim_cal() -> u8 {
    cal_with_errata(6, 7, 0b111, 7, 3) as u8
}

// +0x10 offset below for all values as Temperature log row is at 0x00806030

/// Room temperature in °C
pub fn room_temp() -> f32 {
    let int_val = cal(0x10, 0, 0b11111111);
    let dec_val = cal(0x10 + 1, 0, 0b1111);
    ((int_val * 10) + dec_val) as f32 / 10.0
}

/// Hot temperature in °C
pub fn hot_temp() -> f32 {
    let int_val = cal(0x10 + 1, 4, 0b11111111);
    let dec_val = cal(0x10 + 2, 4, 0b1111);
    ((int_val * 10) + dec_val) as f32 / 10.0
}

pub fn room_int1v_val() -> i8 {
    cal(0x10 + 3, 0, 0b11111111) as i8
}

pub fn hot_int1v_val() -> i8 {
    cal(0x10 + 4, 0, 0b11111111) as i8
}

/// 12-bit ADC conversion at room temperature
pub fn room_temp_adc_val() -> u16 {
    cal(0x10 + 5, 0, 0b111111111111) as u16
}

/// 12-bit ADC conversion at hot temperature
pub fn hot_temp_adc_val() -> u16 {
    cal(0x10 + 6, 4, 0b111111111111) as u16
}
