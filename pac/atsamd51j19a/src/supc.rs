#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub status: STATUS,
    #[doc = "0x10 - BOD33 Control"]
    pub bod33: BOD33,
    #[doc = "0x14 - BOD12 Control"]
    pub bod12: BOD12,
    #[doc = "0x18 - VREG Control"]
    pub vreg: VREG,
    #[doc = "0x1c - VREF Control"]
    pub vref: VREF,
    #[doc = "0x20 - Battery Backup Power Switch"]
    pub bbps: BBPS,
    #[doc = "0x24 - Backup Output Control"]
    pub bkout: BKOUT,
    #[doc = "0x28 - Backup Input Control"]
    pub bkin: BKIN,
}
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Power and Clocks Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "BOD33 Control"]
pub struct BOD33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD33 Control"]
pub mod bod33;
#[doc = "BOD12 Control"]
pub struct BOD12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD12 Control"]
pub mod bod12;
#[doc = "VREG Control"]
pub struct VREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF Control"]
pub struct VREF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VREF Control"]
pub mod vref;
#[doc = "Battery Backup Power Switch"]
pub struct BBPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Battery Backup Power Switch"]
pub mod bbps;
#[doc = "Backup Output Control"]
pub struct BKOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Output Control"]
pub mod bkout;
#[doc = "Backup Input Control"]
pub struct BKIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Input Control"]
pub mod bkin;
