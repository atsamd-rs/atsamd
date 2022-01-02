#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x0c - Power and Clocks Status"]
    pub pclksr: crate::Reg<pclksr::PCLKSR_SPEC>,
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xosc: crate::Reg<xosc::XOSC_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    pub xosc32k: crate::Reg<xosc32k::XOSC32K_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - 32kHz Internal Oscillator (OSC32K) Control"]
    pub osc32k: crate::Reg<osc32k::OSC32K_SPEC>,
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    pub osculp32k: crate::Reg<osculp32k::OSCULP32K_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - 8MHz Internal Oscillator (OSC8M) Control"]
    pub osc8m: crate::Reg<osc8m::OSC8M_SPEC>,
    #[doc = "0x24 - DFLL48M Control"]
    pub dfllctrl: crate::Reg<dfllctrl::DFLLCTRL_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - DFLL48M Value"]
    pub dfllval: crate::Reg<dfllval::DFLLVAL_SPEC>,
    #[doc = "0x2c - DFLL48M Multiplier"]
    pub dfllmul: crate::Reg<dfllmul::DFLLMUL_SPEC>,
    #[doc = "0x30 - DFLL48M Synchronization"]
    pub dfllsync: crate::Reg<dfllsync::DFLLSYNC_SPEC>,
    _reserved13: [u8; 0x03],
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    pub bod33: crate::Reg<bod33::BOD33_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x40 - Voltage References System (VREF) Control"]
    pub vref: crate::Reg<vref::VREF_SPEC>,
    #[doc = "0x44 - DPLL Control A"]
    pub dpllctrla: crate::Reg<dpllctrla::DPLLCTRLA_SPEC>,
    _reserved16: [u8; 0x03],
    #[doc = "0x48 - DPLL Ratio Control"]
    pub dpllratio: crate::Reg<dpllratio::DPLLRATIO_SPEC>,
    #[doc = "0x4c - DPLL Control B"]
    pub dpllctrlb: crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>,
    #[doc = "0x50 - DPLL Status"]
    pub dpllstatus: crate::Reg<dpllstatus::DPLLSTATUS_SPEC>,
}
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "PCLKSR register accessor: an alias for `Reg<PCLKSR_SPEC>`"]
pub type PCLKSR = crate::Reg<pclksr::PCLKSR_SPEC>;
#[doc = "Power and Clocks Status"]
pub mod pclksr;
#[doc = "XOSC register accessor: an alias for `Reg<XOSC_SPEC>`"]
pub type XOSC = crate::Reg<xosc::XOSC_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xosc;
#[doc = "XOSC32K register accessor: an alias for `Reg<XOSC32K_SPEC>`"]
pub type XOSC32K = crate::Reg<xosc32k::XOSC32K_SPEC>;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "OSC32K register accessor: an alias for `Reg<OSC32K_SPEC>`"]
pub type OSC32K = crate::Reg<osc32k::OSC32K_SPEC>;
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub mod osc32k;
#[doc = "OSCULP32K register accessor: an alias for `Reg<OSCULP32K_SPEC>`"]
pub type OSCULP32K = crate::Reg<osculp32k::OSCULP32K_SPEC>;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
#[doc = "OSC8M register accessor: an alias for `Reg<OSC8M_SPEC>`"]
pub type OSC8M = crate::Reg<osc8m::OSC8M_SPEC>;
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub mod osc8m;
#[doc = "DFLLCTRL register accessor: an alias for `Reg<DFLLCTRL_SPEC>`"]
pub type DFLLCTRL = crate::Reg<dfllctrl::DFLLCTRL_SPEC>;
#[doc = "DFLL48M Control"]
pub mod dfllctrl;
#[doc = "DFLLVAL register accessor: an alias for `Reg<DFLLVAL_SPEC>`"]
pub type DFLLVAL = crate::Reg<dfllval::DFLLVAL_SPEC>;
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLLMUL register accessor: an alias for `Reg<DFLLMUL_SPEC>`"]
pub type DFLLMUL = crate::Reg<dfllmul::DFLLMUL_SPEC>;
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLLSYNC register accessor: an alias for `Reg<DFLLSYNC_SPEC>`"]
pub type DFLLSYNC = crate::Reg<dfllsync::DFLLSYNC_SPEC>;
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
#[doc = "BOD33 register accessor: an alias for `Reg<BOD33_SPEC>`"]
pub type BOD33 = crate::Reg<bod33::BOD33_SPEC>;
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub mod bod33;
#[doc = "VREF register accessor: an alias for `Reg<VREF_SPEC>`"]
pub type VREF = crate::Reg<vref::VREF_SPEC>;
#[doc = "Voltage References System (VREF) Control"]
pub mod vref;
#[doc = "DPLLCTRLA register accessor: an alias for `Reg<DPLLCTRLA_SPEC>`"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO register accessor: an alias for `Reg<DPLLRATIO_SPEC>`"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB register accessor: an alias for `Reg<DPLLCTRLB_SPEC>`"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLSTATUS register accessor: an alias for `Reg<DPLLSTATUS_SPEC>`"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
