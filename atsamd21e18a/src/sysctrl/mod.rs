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
    pub pclksr: PCLKSR,
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xosc: XOSC,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    pub xosc32k: XOSC32K,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - 32kHz Internal Oscillator (OSC32K) Control"]
    pub osc32k: OSC32K,
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    pub osculp32k: OSCULP32K,
    _reserved8: [u8; 3usize],
    #[doc = "0x20 - 8MHz Internal Oscillator (OSC8M) Control"]
    pub osc8m: OSC8M,
    #[doc = "0x24 - DFLL48M Control"]
    pub dfllctrl: DFLLCTRL,
    _reserved10: [u8; 2usize],
    #[doc = "0x28 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x2c - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x30 - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved13: [u8; 3usize],
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    pub bod33: BOD33,
    _reserved14: [u8; 4usize],
    #[doc = "0x3c - Voltage Regulator System (VREG) Control"]
    pub vreg: VREG,
    _reserved15: [u8; 2usize],
    #[doc = "0x40 - Voltage References System (VREF) Control"]
    pub vref: VREF,
    #[doc = "0x44 - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved17: [u8; 3usize],
    #[doc = "0x48 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x4c - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x50 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
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
pub struct PCLKSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power and Clocks Status"]
pub mod pclksr;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub struct XOSC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xosc;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub struct XOSC32K {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub struct OSC32K {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub mod osc32k;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub struct OSCULP32K {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub struct OSC8M {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub mod osc8m;
#[doc = "DFLL48M Control"]
pub struct DFLLCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DFLL48M Control"]
pub mod dfllctrl;
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
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub struct BOD33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub mod bod33;
#[doc = "Voltage Regulator System (VREG) Control"]
pub struct VREG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Voltage Regulator System (VREG) Control"]
pub mod vreg;
#[doc = "Voltage References System (VREF) Control"]
pub struct VREF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage References System (VREF) Control"]
pub mod vref;
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
#[doc = "DPLL Status"]
pub struct DPLLSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DPLL Status"]
pub mod dpllstatus;
