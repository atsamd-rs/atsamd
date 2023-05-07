#[doc = r"Register block"]
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
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    pub xosc32k: XOSC32K,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - 32kHz Internal Oscillator (OSC32K) Control"]
    pub osc32k: OSC32K,
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    pub osculp32k: OSCULP32K,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - 8MHz Internal Oscillator (OSC8M) Control"]
    pub osc8m: OSC8M,
    #[doc = "0x24 - DFLL48M Control"]
    pub dfllctrl: DFLLCTRL,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x2c - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x30 - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved13: [u8; 0x03],
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    pub bod33: BOD33,
    _reserved14: [u8; 0x04],
    #[doc = "0x3c - Voltage Regulator System (VREG) Control"]
    pub vreg: VREG,
    _reserved15: [u8; 0x02],
    #[doc = "0x40 - Voltage References System (VREF) Control"]
    pub vref: VREF,
    #[doc = "0x44 - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved17: [u8; 0x03],
    #[doc = "0x48 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x4c - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x50 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
}
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "PCLKSR (r) register accessor: an alias for `Reg<PCLKSR_SPEC>`"]
pub type PCLKSR = crate::Reg<pclksr::PCLKSR_SPEC>;
#[doc = "Power and Clocks Status"]
pub mod pclksr;
#[doc = "XOSC (rw) register accessor: an alias for `Reg<XOSC_SPEC>`"]
pub type XOSC = crate::Reg<xosc::XOSC_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xosc;
#[doc = "XOSC32K (rw) register accessor: an alias for `Reg<XOSC32K_SPEC>`"]
pub type XOSC32K = crate::Reg<xosc32k::XOSC32K_SPEC>;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "OSC32K (rw) register accessor: an alias for `Reg<OSC32K_SPEC>`"]
pub type OSC32K = crate::Reg<osc32k::OSC32K_SPEC>;
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub mod osc32k;
#[doc = "OSCULP32K (rw) register accessor: an alias for `Reg<OSCULP32K_SPEC>`"]
pub type OSCULP32K = crate::Reg<osculp32k::OSCULP32K_SPEC>;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
#[doc = "OSC8M (rw) register accessor: an alias for `Reg<OSC8M_SPEC>`"]
pub type OSC8M = crate::Reg<osc8m::OSC8M_SPEC>;
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub mod osc8m;
#[doc = "DFLLCTRL (rw) register accessor: an alias for `Reg<DFLLCTRL_SPEC>`"]
pub type DFLLCTRL = crate::Reg<dfllctrl::DFLLCTRL_SPEC>;
#[doc = "DFLL48M Control"]
pub mod dfllctrl;
#[doc = "DFLLVAL (rw) register accessor: an alias for `Reg<DFLLVAL_SPEC>`"]
pub type DFLLVAL = crate::Reg<dfllval::DFLLVAL_SPEC>;
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLLMUL (rw) register accessor: an alias for `Reg<DFLLMUL_SPEC>`"]
pub type DFLLMUL = crate::Reg<dfllmul::DFLLMUL_SPEC>;
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLLSYNC (rw) register accessor: an alias for `Reg<DFLLSYNC_SPEC>`"]
pub type DFLLSYNC = crate::Reg<dfllsync::DFLLSYNC_SPEC>;
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
#[doc = "BOD33 (rw) register accessor: an alias for `Reg<BOD33_SPEC>`"]
pub type BOD33 = crate::Reg<bod33::BOD33_SPEC>;
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub mod bod33;
#[doc = "VREG (rw) register accessor: an alias for `Reg<VREG_SPEC>`"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "Voltage Regulator System (VREG) Control"]
pub mod vreg;
#[doc = "VREF (rw) register accessor: an alias for `Reg<VREF_SPEC>`"]
pub type VREF = crate::Reg<vref::VREF_SPEC>;
#[doc = "Voltage References System (VREF) Control"]
pub mod vref;
#[doc = "DPLLCTRLA (rw) register accessor: an alias for `Reg<DPLLCTRLA_SPEC>`"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: an alias for `Reg<DPLLRATIO_SPEC>`"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: an alias for `Reg<DPLLCTRLB_SPEC>`"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLSTATUS (r) register accessor: an alias for `Reg<DPLLSTATUS_SPEC>`"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
