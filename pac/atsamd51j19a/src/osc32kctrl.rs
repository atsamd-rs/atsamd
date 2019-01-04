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
    #[doc = "0x10 - RTC Clock Selection"]
    pub rtcctrl: RTCCTRL,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    pub xosc32k: XOSC32K,
    #[doc = "0x16 - Clock Failure Detector Control"]
    pub cfdctrl: CFDCTRL,
    #[doc = "0x17 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved8: [u8; 4usize],
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    pub osculp32k: OSCULP32K,
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
#[doc = "RTC Clock Selection"]
pub struct RTCCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RTC Clock Selection"]
pub mod rtcctrl;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub struct XOSC32K {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "Clock Failure Detector Control"]
pub struct CFDCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock Failure Detector Control"]
pub mod cfdctrl;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub struct OSCULP32K {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
