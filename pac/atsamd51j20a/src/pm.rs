#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    _reserved2: [u8; 2usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 1usize],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
    #[doc = "0x09 - Hibernate Configuration"]
    pub hibcfg: HIBCFG,
    #[doc = "0x0a - Backup Configuration"]
    pub bkupcfg: BKUPCFG,
    _reserved8: [u8; 7usize],
    #[doc = "0x12 - Power Switch Acknowledge Delay"]
    pub pwsakdly: PWSAKDLY,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Sleep Configuration"]
pub struct SLEEPCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Standby Configuration"]
pub struct STDBYCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Standby Configuration"]
pub mod stdbycfg;
#[doc = "Hibernate Configuration"]
pub struct HIBCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Hibernate Configuration"]
pub mod hibcfg;
#[doc = "Backup Configuration"]
pub struct BKUPCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backup Configuration"]
pub mod bkupcfg;
#[doc = "Power Switch Acknowledge Delay"]
pub struct PWSAKDLY {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Switch Acknowledge Delay"]
pub mod pwsakdly;
