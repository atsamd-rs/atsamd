#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - External Multipurpose Crystal Oscillator Control"]
    pub xoscctrl: [XOSCCTRL; 2],
    #[doc = "0x1c - DFLL48M Control A"]
    pub dfllctrla: DFLLCTRLA,
    _reserved7: [u8; 3usize],
    #[doc = "0x20 - DFLL48M Control B"]
    pub dfllctrlb: DFLLCTRLB,
    _reserved8: [u8; 3usize],
    #[doc = "0x24 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x28 - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x2c - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved11: [u8; 3usize],
    #[doc = "0x30 - DPLL Control A"]
    pub dpllctrla0: DPLLCTRLA,
    _reserved12: [u8; 3usize],
    #[doc = "0x34 - DPLL Ratio Control"]
    pub dpllratio0: DPLLRATIO,
    #[doc = "0x38 - DPLL Control B"]
    pub dpllctrlb0: DPLLCTRLB,
    #[doc = "0x3c - DPLL Synchronization Busy"]
    pub dpllsyncbusy0: DPLLSYNCBUSY,
    #[doc = "0x40 - DPLL Status"]
    pub dpllstatus0: DPLLSTATUS,
    #[doc = "0x44 - DPLL Control A"]
    pub dpllctrla1: DPLLCTRLA,
    _reserved17: [u8; 3usize],
    #[doc = "0x48 - DPLL Ratio Control"]
    pub dpllratio1: DPLLRATIO,
    #[doc = "0x4c - DPLL Control B"]
    pub dpllctrlb1: DPLLCTRLB,
    #[doc = "0x50 - DPLL Synchronization Busy"]
    pub dpllsyncbusy1: DPLLSYNCBUSY,
    #[doc = "0x54 - DPLL Status"]
    pub dpllstatus1: DPLLSTATUS,
}
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "External Multipurpose Crystal Oscillator Control"]
pub struct XOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Multipurpose Crystal Oscillator Control"]
pub mod xoscctrl;
#[doc = "DFLL48M Control A"]
pub struct DFLLCTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DFLL48M Control A"]
pub mod dfllctrla;
#[doc = "DFLL48M Control B"]
pub struct DFLLCTRLB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DFLL48M Control B"]
pub mod dfllctrlb;
#[doc = "DFLL48M Value"]
pub struct DFLLVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLL48M Multiplier"]
pub struct DFLLMUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLL48M Synchronization"]
pub struct DFLLSYNC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
#[doc = "DPLL Control A"]
pub struct DPLLCTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLL Ratio Control"]
pub struct DPLLRATIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLL Control B"]
pub struct DPLLCTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLL Synchronization Busy"]
pub struct DPLLSYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLL Status"]
pub struct DPLLSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Status"]
pub mod dpllstatus;
