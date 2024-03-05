//! NVM Software Calibration Area Mapping
// See 9.5 NVM Software Calibration Area Mapping, page 57

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

/// ADC0 BIASCOMP calibration value. Should be written to ADC0 CALIB register.
pub fn adc0_biascomp_scale_cal() -> u8 {
    cal(0, 2, 0b111) as u8
}

/// ADC0 BIASREFBUF calibration value. Should be written to ADC0 CALIB register.
pub fn adc0_biasref_scale_cal() -> u8 {
    cal(0, 5, 0b111) as u8
}

/// ADC0 BIASR2R calibration value. Should be written to ADC0 CALIB register.
pub fn adc0_biasr2r_scale_cal() -> u8 {
    cal(1, 0, 0b111) as u8
}

/// ADC1 BIASCOMP calibration value. Should be written to ADC1 CALIB register.
pub fn adc1_biascomp_scale_cal() -> u8 {
    cal(2, 2, 0b111) as u8
}

/// ADC1 BIASREFBUF calibration value. Should be written to ADC1 CALIB register.
pub fn adc1_biasref_scale_cal() -> u8 {
    cal(2, 5, 0b111) as u8
}

/// ADC1 BIASR2R calibration value. Should be written to ADC1 CALIB register.
pub fn adc1_biasr2r_scale_cal() -> u8 {
    cal(3, 0, 0b111) as u8
}
