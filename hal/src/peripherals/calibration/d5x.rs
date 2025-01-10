//! NVM Software Calibration Area Mapping
// See 9.5 NVM Software Calibration Area Mapping, page 57

use core::ptr;

// "The NVM Software Calibration Area can be read at address 0x00800080."
const ADDR: u32 = 0x00800080;

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
    cal(5, 2, 0b111) as u8
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
    cal(2, 0, 0b111) as u8
}

/// ADC1 BIASREFBUF calibration value. Should be written to ADC1 CALIB register.
pub fn adc1_biasref_scale_cal() -> u8 {
    cal(2, 3, 0b111) as u8
}

/// ADC1 BIASR2R calibration value. Should be written to ADC1 CALIB register.
pub fn adc1_biasr2r_scale_cal() -> u8 {
    cal(2, 6, 0b111) as u8
}

/// Calibration temperature parameter 'TL', formed by TLI and TLD (TL'Integer', TL'Decimal')
pub fn tl() -> f32 {
    tli() as f32 + (tld() as f32 / 10.0)
}

/// Calibration temperature parameter 'TH', formed by THI and THD (TH'Integer', TH'Decimal')
pub fn th() -> f32 {
    thi() as f32 + (thd() as f32 / 10.0)
}

/// Temperature calibration - Integer part of calibration temperature TL
pub fn tli() -> u32 {
    cal(0x80, 0, 0b11111111)
}

/// Temperature calibration - Decimal part of calibration temperature TL
pub fn tld() -> u32 {
    cal(0x80 + 1, 0, 0b1111)
}

/// Temperature calibration - Integer part of calibration temperature TH
pub fn thi() -> u32 {
    cal(0x80 + 1, 4, 0b11111111)
}

/// Temperature calibration - Decimal part of calibration temperature TH
pub fn thd() -> u32 {
    cal(0x80 + 2, 4, 0b1111)
}

/// Temperature calibration - Parameter VPL
pub fn vpl() -> u16 {
    cal(0x80 + 5, 0, 0b111111111111) as u16
}

/// Temperature calibration - Parameter VPH
pub fn vph() -> u16 {
    cal(0x80 + 6, 4, 0b111111111111) as u16
}

/// Temperature calibration - Parameter VCL
pub fn vcl() -> u16 {
    cal(0x80 + 8, 0, 0b111111111111) as u16
}

/// Temperature calibration - Parameter VCH
pub fn vch() -> u16 {
    cal(0x80 + 9, 4, 0b111111111111) as u16
}
