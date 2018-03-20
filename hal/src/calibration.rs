//! NVM Software Calibration Area Mapping
// See 10.3.2 NVM Software Calibration Area Mapping, page 46

use core::ptr;

const ADDR: u32 = 0x806020u32;

/// Returns the osc32k calibration value from the NVM calibration area
pub fn osc32k_cal() -> u8 {
    unsafe {
        let addr: *const u32 = (ADDR + 4) as *const _;
        let value = ptr::read(addr);

        ((value >> 6) & 0x7f) as u8
    }
}

/// Returns the dfll48m coarse calibration value
pub fn dfll48m_coarse_cal() -> u8 {
    unsafe {
        let addr: *const u32 = (ADDR + 4) as *const _;
        let value = ptr::read(addr);

        let cal = ((value >> 26) & 0x3f) as u8;

        if cal == 0x3f {
            // Some revisions of some chips have bad calibration
            0x1f
        } else {
            cal
        }
    }
}
