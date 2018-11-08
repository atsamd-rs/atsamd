#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir0: DIR,
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr0: DIRCLR,
    #[doc = "0x08 - Data Direction Set"]
    pub dirset0: DIRSET,
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl0: DIRTGL,
    #[doc = "0x10 - Data Output Value"]
    pub out0: OUT,
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr0: OUTCLR,
    #[doc = "0x18 - Data Output Value Set"]
    pub outset0: OUTSET,
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl0: OUTTGL,
    #[doc = "0x20 - Data Input Value"]
    pub in0: IN,
    #[doc = "0x24 - Control"]
    pub ctrl0: CTRL,
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig0: WRCONFIG,
    #[doc = "0x2c - Event Input Control"]
    pub evctrl0: EVCTRL,
    #[doc = "0x30 - Peripheral Multiplexing - Group 0"]
    pub pmux0_: [PMUX0_; 16],
    #[doc = "0x40 - Pin Configuration - Group 0"]
    pub pincfg0_: [PINCFG0_; 32],
    _reserved14: [u8; 32usize],
    #[doc = "0x80 - Data Direction"]
    pub dir1: DIR,
    #[doc = "0x84 - Data Direction Clear"]
    pub dirclr1: DIRCLR,
    #[doc = "0x88 - Data Direction Set"]
    pub dirset1: DIRSET,
    #[doc = "0x8c - Data Direction Toggle"]
    pub dirtgl1: DIRTGL,
    #[doc = "0x90 - Data Output Value"]
    pub out1: OUT,
    #[doc = "0x94 - Data Output Value Clear"]
    pub outclr1: OUTCLR,
    #[doc = "0x98 - Data Output Value Set"]
    pub outset1: OUTSET,
    #[doc = "0x9c - Data Output Value Toggle"]
    pub outtgl1: OUTTGL,
    #[doc = "0xa0 - Data Input Value"]
    pub in1: IN,
    #[doc = "0xa4 - Control"]
    pub ctrl1: CTRL,
    #[doc = "0xa8 - Write Configuration"]
    pub wrconfig1: WRCONFIG,
    #[doc = "0xac - Event Input Control"]
    pub evctrl1: EVCTRL,
    #[doc = "0xb0 - Peripheral Multiplexing - Group 1"]
    pub pmux1_: [PMUX1_; 16],
    #[doc = "0xc0 - Pin Configuration - Group 1"]
    pub pincfg1_: [PINCFG1_; 32],
}
#[doc = "Data Direction"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction"]
pub mod dir;
#[doc = "Data Direction Clear"]
pub struct DIRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "Data Direction Set"]
pub struct DIRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "Data Direction Toggle"]
pub struct DIRTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "Data Output Value"]
pub struct OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value"]
pub mod out;
#[doc = "Data Output Value Clear"]
pub struct OUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value Clear"]
pub mod outclr;
#[doc = "Data Output Value Set"]
pub struct OUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value Set"]
pub mod outset;
#[doc = "Data Output Value Toggle"]
pub struct OUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Value Toggle"]
pub mod outtgl;
#[doc = "Data Input Value"]
pub struct IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input Value"]
pub mod in_;
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Write Configuration"]
pub struct WRCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Configuration"]
pub mod wrconfig;
#[doc = "Event Input Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Input Control"]
pub mod evctrl;
#[doc = "Peripheral Multiplexing - Group 0"]
pub struct PMUX0_ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral Multiplexing - Group 0"]
pub mod pmux0_;
#[doc = "Peripheral Multiplexing - Group 1"]
pub struct PMUX1_ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral Multiplexing - Group 1"]
pub mod pmux1_;
#[doc = "Pin Configuration - Group 0"]
pub struct PINCFG0_ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Pin Configuration - Group 0"]
pub mod pincfg0_;
#[doc = "Pin Configuration - Group 1"]
pub struct PINCFG1_ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Pin Configuration - Group 1"]
pub mod pincfg1_;
