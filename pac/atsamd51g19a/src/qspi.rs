#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Baud Rate"]
    pub baud: BAUD,
    #[doc = "0x0c - Receive Data"]
    pub rxdata: RXDATA,
    #[doc = "0x10 - Transmit Data"]
    pub txdata: TXDATA,
    #[doc = "0x14 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x1c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x20 - Status Register"]
    pub status: STATUS,
    _reserved9: [u8; 12usize],
    #[doc = "0x30 - Instruction Address"]
    pub instraddr: INSTRADDR,
    #[doc = "0x34 - Instruction Code"]
    pub instrctrl: INSTRCTRL,
    #[doc = "0x38 - Instruction Frame"]
    pub instrframe: INSTRFRAME,
    _reserved12: [u8; 4usize],
    #[doc = "0x40 - Scrambling Mode"]
    pub scrambctrl: SCRAMBCTRL,
    #[doc = "0x44 - Scrambling Key"]
    pub scrambkey: SCRAMBKEY,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Baud Rate"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate"]
pub mod baud;
#[doc = "Receive Data"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data"]
pub mod rxdata;
#[doc = "Transmit Data"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data"]
pub mod txdata;
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
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Instruction Address"]
pub struct INSTRADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Address"]
pub mod instraddr;
#[doc = "Instruction Code"]
pub struct INSTRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Code"]
pub mod instrctrl;
#[doc = "Instruction Frame"]
pub struct INSTRFRAME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Frame"]
pub mod instrframe;
#[doc = "Scrambling Mode"]
pub struct SCRAMBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scrambling Mode"]
pub mod scrambctrl;
#[doc = "Scrambling Key"]
pub struct SCRAMBKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scrambling Key"]
pub mod scrambkey;
