#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    pub pio_per: crate::Reg<pio_per::PIO_PER_SPEC>,
    #[doc = "0x04 - PIO Disable Register"]
    pub pio_pdr: crate::Reg<pio_pdr::PIO_PDR_SPEC>,
    #[doc = "0x08 - PIO Status Register"]
    pub pio_psr: crate::Reg<pio_psr::PIO_PSR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Output Enable Register"]
    pub pio_oer: crate::Reg<pio_oer::PIO_OER_SPEC>,
    #[doc = "0x14 - Output Disable Register"]
    pub pio_odr: crate::Reg<pio_odr::PIO_ODR_SPEC>,
    #[doc = "0x18 - Output Status Register"]
    pub pio_osr: crate::Reg<pio_osr::PIO_OSR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    pub pio_ifer: crate::Reg<pio_ifer::PIO_IFER_SPEC>,
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    pub pio_ifdr: crate::Reg<pio_ifdr::PIO_IFDR_SPEC>,
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    pub pio_ifsr: crate::Reg<pio_ifsr::PIO_IFSR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Set Output Data Register"]
    pub pio_sodr: crate::Reg<pio_sodr::PIO_SODR_SPEC>,
    #[doc = "0x34 - Clear Output Data Register"]
    pub pio_codr: crate::Reg<pio_codr::PIO_CODR_SPEC>,
    #[doc = "0x38 - Output Data Status Register"]
    pub pio_odsr: crate::Reg<pio_odsr::PIO_ODSR_SPEC>,
    #[doc = "0x3c - Pin Data Status Register"]
    pub pio_pdsr: crate::Reg<pio_pdsr::PIO_PDSR_SPEC>,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub pio_ier: crate::Reg<pio_ier::PIO_IER_SPEC>,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub pio_idr: crate::Reg<pio_idr::PIO_IDR_SPEC>,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub pio_imr: crate::Reg<pio_imr::PIO_IMR_SPEC>,
    #[doc = "0x4c - Interrupt Status Register"]
    pub pio_isr: crate::Reg<pio_isr::PIO_ISR_SPEC>,
    #[doc = "0x50 - Multi-driver Enable Register"]
    pub pio_mder: crate::Reg<pio_mder::PIO_MDER_SPEC>,
    #[doc = "0x54 - Multi-driver Disable Register"]
    pub pio_mddr: crate::Reg<pio_mddr::PIO_MDDR_SPEC>,
    #[doc = "0x58 - Multi-driver Status Register"]
    pub pio_mdsr: crate::Reg<pio_mdsr::PIO_MDSR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - Pull-up Disable Register"]
    pub pio_pudr: crate::Reg<pio_pudr::PIO_PUDR_SPEC>,
    #[doc = "0x64 - Pull-up Enable Register"]
    pub pio_puer: crate::Reg<pio_puer::PIO_PUER_SPEC>,
    #[doc = "0x68 - Pad Pull-up Status Register"]
    pub pio_pusr: crate::Reg<pio_pusr::PIO_PUSR_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x70..0x78 - Peripheral ABCD Select Register 0"]
    pub pio_abcdsr: [crate::Reg<pio_abcdsr::PIO_ABCDSR_SPEC>; 2],
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - Input Filter Slow Clock Disable Register"]
    pub pio_ifscdr: crate::Reg<pio_ifscdr::PIO_IFSCDR_SPEC>,
    #[doc = "0x84 - Input Filter Slow Clock Enable Register"]
    pub pio_ifscer: crate::Reg<pio_ifscer::PIO_IFSCER_SPEC>,
    #[doc = "0x88 - Input Filter Slow Clock Status Register"]
    pub pio_ifscsr: crate::Reg<pio_ifscsr::PIO_IFSCSR_SPEC>,
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    pub pio_scdr: crate::Reg<pio_scdr::PIO_SCDR_SPEC>,
    #[doc = "0x90 - Pad Pull-down Disable Register"]
    pub pio_ppddr: crate::Reg<pio_ppddr::PIO_PPDDR_SPEC>,
    #[doc = "0x94 - Pad Pull-down Enable Register"]
    pub pio_ppder: crate::Reg<pio_ppder::PIO_PPDER_SPEC>,
    #[doc = "0x98 - Pad Pull-down Status Register"]
    pub pio_ppdsr: crate::Reg<pio_ppdsr::PIO_PPDSR_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0xa0 - Output Write Enable"]
    pub pio_ower: crate::Reg<pio_ower::PIO_OWER_SPEC>,
    #[doc = "0xa4 - Output Write Disable"]
    pub pio_owdr: crate::Reg<pio_owdr::PIO_OWDR_SPEC>,
    #[doc = "0xa8 - Output Write Status Register"]
    pub pio_owsr: crate::Reg<pio_owsr::PIO_OWSR_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    pub pio_aimer: crate::Reg<pio_aimer::PIO_AIMER_SPEC>,
    #[doc = "0xb4 - Additional Interrupt Modes Disable Register"]
    pub pio_aimdr: crate::Reg<pio_aimdr::PIO_AIMDR_SPEC>,
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    pub pio_aimmr: crate::Reg<pio_aimmr::PIO_AIMMR_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0xc0 - Edge Select Register"]
    pub pio_esr: crate::Reg<pio_esr::PIO_ESR_SPEC>,
    #[doc = "0xc4 - Level Select Register"]
    pub pio_lsr: crate::Reg<pio_lsr::PIO_LSR_SPEC>,
    #[doc = "0xc8 - Edge/Level Status Register"]
    pub pio_elsr: crate::Reg<pio_elsr::PIO_ELSR_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0xd0 - Falling Edge/Low-Level Select Register"]
    pub pio_fellsr: crate::Reg<pio_fellsr::PIO_FELLSR_SPEC>,
    #[doc = "0xd4 - Rising Edge/High-Level Select Register"]
    pub pio_rehlsr: crate::Reg<pio_rehlsr::PIO_REHLSR_SPEC>,
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    pub pio_frlhsr: crate::Reg<pio_frlhsr::PIO_FRLHSR_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0xe0 - Lock Status"]
    pub pio_locksr: crate::Reg<pio_locksr::PIO_LOCKSR_SPEC>,
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub pio_wpmr: crate::Reg<pio_wpmr::PIO_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub pio_wpsr: crate::Reg<pio_wpsr::PIO_WPSR_SPEC>,
    _reserved46: [u8; 0x14],
    #[doc = "0x100 - Schmitt Trigger Register"]
    pub pio_schmitt: crate::Reg<pio_schmitt::PIO_SCHMITT_SPEC>,
    _reserved47: [u8; 0x14],
    #[doc = "0x118 - I/O Drive Register"]
    pub pio_driver: crate::Reg<pio_driver::PIO_DRIVER_SPEC>,
    _reserved48: [u8; 0x34],
    #[doc = "0x150 - Parallel Capture Mode Register"]
    pub pio_pcmr: crate::Reg<pio_pcmr::PIO_PCMR_SPEC>,
    #[doc = "0x154 - Parallel Capture Interrupt Enable Register"]
    pub pio_pcier: crate::Reg<pio_pcier::PIO_PCIER_SPEC>,
    #[doc = "0x158 - Parallel Capture Interrupt Disable Register"]
    pub pio_pcidr: crate::Reg<pio_pcidr::PIO_PCIDR_SPEC>,
    #[doc = "0x15c - Parallel Capture Interrupt Mask Register"]
    pub pio_pcimr: crate::Reg<pio_pcimr::PIO_PCIMR_SPEC>,
    #[doc = "0x160 - Parallel Capture Interrupt Status Register"]
    pub pio_pcisr: crate::Reg<pio_pcisr::PIO_PCISR_SPEC>,
    #[doc = "0x164 - Parallel Capture Reception Holding Register"]
    pub pio_pcrhr: crate::Reg<pio_pcrhr::PIO_PCRHR_SPEC>,
}
#[doc = "PIO_PER register accessor: an alias for `Reg<PIO_PER_SPEC>`"]
pub type PIO_PER = crate::Reg<pio_per::PIO_PER_SPEC>;
#[doc = "PIO Enable Register"]
pub mod pio_per;
#[doc = "PIO_PDR register accessor: an alias for `Reg<PIO_PDR_SPEC>`"]
pub type PIO_PDR = crate::Reg<pio_pdr::PIO_PDR_SPEC>;
#[doc = "PIO Disable Register"]
pub mod pio_pdr;
#[doc = "PIO_PSR register accessor: an alias for `Reg<PIO_PSR_SPEC>`"]
pub type PIO_PSR = crate::Reg<pio_psr::PIO_PSR_SPEC>;
#[doc = "PIO Status Register"]
pub mod pio_psr;
#[doc = "PIO_OER register accessor: an alias for `Reg<PIO_OER_SPEC>`"]
pub type PIO_OER = crate::Reg<pio_oer::PIO_OER_SPEC>;
#[doc = "Output Enable Register"]
pub mod pio_oer;
#[doc = "PIO_ODR register accessor: an alias for `Reg<PIO_ODR_SPEC>`"]
pub type PIO_ODR = crate::Reg<pio_odr::PIO_ODR_SPEC>;
#[doc = "Output Disable Register"]
pub mod pio_odr;
#[doc = "PIO_OSR register accessor: an alias for `Reg<PIO_OSR_SPEC>`"]
pub type PIO_OSR = crate::Reg<pio_osr::PIO_OSR_SPEC>;
#[doc = "Output Status Register"]
pub mod pio_osr;
#[doc = "PIO_IFER register accessor: an alias for `Reg<PIO_IFER_SPEC>`"]
pub type PIO_IFER = crate::Reg<pio_ifer::PIO_IFER_SPEC>;
#[doc = "Glitch Input Filter Enable Register"]
pub mod pio_ifer;
#[doc = "PIO_IFDR register accessor: an alias for `Reg<PIO_IFDR_SPEC>`"]
pub type PIO_IFDR = crate::Reg<pio_ifdr::PIO_IFDR_SPEC>;
#[doc = "Glitch Input Filter Disable Register"]
pub mod pio_ifdr;
#[doc = "PIO_IFSR register accessor: an alias for `Reg<PIO_IFSR_SPEC>`"]
pub type PIO_IFSR = crate::Reg<pio_ifsr::PIO_IFSR_SPEC>;
#[doc = "Glitch Input Filter Status Register"]
pub mod pio_ifsr;
#[doc = "PIO_SODR register accessor: an alias for `Reg<PIO_SODR_SPEC>`"]
pub type PIO_SODR = crate::Reg<pio_sodr::PIO_SODR_SPEC>;
#[doc = "Set Output Data Register"]
pub mod pio_sodr;
#[doc = "PIO_CODR register accessor: an alias for `Reg<PIO_CODR_SPEC>`"]
pub type PIO_CODR = crate::Reg<pio_codr::PIO_CODR_SPEC>;
#[doc = "Clear Output Data Register"]
pub mod pio_codr;
#[doc = "PIO_ODSR register accessor: an alias for `Reg<PIO_ODSR_SPEC>`"]
pub type PIO_ODSR = crate::Reg<pio_odsr::PIO_ODSR_SPEC>;
#[doc = "Output Data Status Register"]
pub mod pio_odsr;
#[doc = "PIO_PDSR register accessor: an alias for `Reg<PIO_PDSR_SPEC>`"]
pub type PIO_PDSR = crate::Reg<pio_pdsr::PIO_PDSR_SPEC>;
#[doc = "Pin Data Status Register"]
pub mod pio_pdsr;
#[doc = "PIO_IER register accessor: an alias for `Reg<PIO_IER_SPEC>`"]
pub type PIO_IER = crate::Reg<pio_ier::PIO_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pio_ier;
#[doc = "PIO_IDR register accessor: an alias for `Reg<PIO_IDR_SPEC>`"]
pub type PIO_IDR = crate::Reg<pio_idr::PIO_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod pio_idr;
#[doc = "PIO_IMR register accessor: an alias for `Reg<PIO_IMR_SPEC>`"]
pub type PIO_IMR = crate::Reg<pio_imr::PIO_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod pio_imr;
#[doc = "PIO_ISR register accessor: an alias for `Reg<PIO_ISR_SPEC>`"]
pub type PIO_ISR = crate::Reg<pio_isr::PIO_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod pio_isr;
#[doc = "PIO_MDER register accessor: an alias for `Reg<PIO_MDER_SPEC>`"]
pub type PIO_MDER = crate::Reg<pio_mder::PIO_MDER_SPEC>;
#[doc = "Multi-driver Enable Register"]
pub mod pio_mder;
#[doc = "PIO_MDDR register accessor: an alias for `Reg<PIO_MDDR_SPEC>`"]
pub type PIO_MDDR = crate::Reg<pio_mddr::PIO_MDDR_SPEC>;
#[doc = "Multi-driver Disable Register"]
pub mod pio_mddr;
#[doc = "PIO_MDSR register accessor: an alias for `Reg<PIO_MDSR_SPEC>`"]
pub type PIO_MDSR = crate::Reg<pio_mdsr::PIO_MDSR_SPEC>;
#[doc = "Multi-driver Status Register"]
pub mod pio_mdsr;
#[doc = "PIO_PUDR register accessor: an alias for `Reg<PIO_PUDR_SPEC>`"]
pub type PIO_PUDR = crate::Reg<pio_pudr::PIO_PUDR_SPEC>;
#[doc = "Pull-up Disable Register"]
pub mod pio_pudr;
#[doc = "PIO_PUER register accessor: an alias for `Reg<PIO_PUER_SPEC>`"]
pub type PIO_PUER = crate::Reg<pio_puer::PIO_PUER_SPEC>;
#[doc = "Pull-up Enable Register"]
pub mod pio_puer;
#[doc = "PIO_PUSR register accessor: an alias for `Reg<PIO_PUSR_SPEC>`"]
pub type PIO_PUSR = crate::Reg<pio_pusr::PIO_PUSR_SPEC>;
#[doc = "Pad Pull-up Status Register"]
pub mod pio_pusr;
#[doc = "PIO_ABCDSR register accessor: an alias for `Reg<PIO_ABCDSR_SPEC>`"]
pub type PIO_ABCDSR = crate::Reg<pio_abcdsr::PIO_ABCDSR_SPEC>;
#[doc = "Peripheral ABCD Select Register 0"]
pub mod pio_abcdsr;
#[doc = "PIO_IFSCDR register accessor: an alias for `Reg<PIO_IFSCDR_SPEC>`"]
pub type PIO_IFSCDR = crate::Reg<pio_ifscdr::PIO_IFSCDR_SPEC>;
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod pio_ifscdr;
#[doc = "PIO_IFSCER register accessor: an alias for `Reg<PIO_IFSCER_SPEC>`"]
pub type PIO_IFSCER = crate::Reg<pio_ifscer::PIO_IFSCER_SPEC>;
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod pio_ifscer;
#[doc = "PIO_IFSCSR register accessor: an alias for `Reg<PIO_IFSCSR_SPEC>`"]
pub type PIO_IFSCSR = crate::Reg<pio_ifscsr::PIO_IFSCSR_SPEC>;
#[doc = "Input Filter Slow Clock Status Register"]
pub mod pio_ifscsr;
#[doc = "PIO_SCDR register accessor: an alias for `Reg<PIO_SCDR_SPEC>`"]
pub type PIO_SCDR = crate::Reg<pio_scdr::PIO_SCDR_SPEC>;
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod pio_scdr;
#[doc = "PIO_PPDDR register accessor: an alias for `Reg<PIO_PPDDR_SPEC>`"]
pub type PIO_PPDDR = crate::Reg<pio_ppddr::PIO_PPDDR_SPEC>;
#[doc = "Pad Pull-down Disable Register"]
pub mod pio_ppddr;
#[doc = "PIO_PPDER register accessor: an alias for `Reg<PIO_PPDER_SPEC>`"]
pub type PIO_PPDER = crate::Reg<pio_ppder::PIO_PPDER_SPEC>;
#[doc = "Pad Pull-down Enable Register"]
pub mod pio_ppder;
#[doc = "PIO_PPDSR register accessor: an alias for `Reg<PIO_PPDSR_SPEC>`"]
pub type PIO_PPDSR = crate::Reg<pio_ppdsr::PIO_PPDSR_SPEC>;
#[doc = "Pad Pull-down Status Register"]
pub mod pio_ppdsr;
#[doc = "PIO_OWER register accessor: an alias for `Reg<PIO_OWER_SPEC>`"]
pub type PIO_OWER = crate::Reg<pio_ower::PIO_OWER_SPEC>;
#[doc = "Output Write Enable"]
pub mod pio_ower;
#[doc = "PIO_OWDR register accessor: an alias for `Reg<PIO_OWDR_SPEC>`"]
pub type PIO_OWDR = crate::Reg<pio_owdr::PIO_OWDR_SPEC>;
#[doc = "Output Write Disable"]
pub mod pio_owdr;
#[doc = "PIO_OWSR register accessor: an alias for `Reg<PIO_OWSR_SPEC>`"]
pub type PIO_OWSR = crate::Reg<pio_owsr::PIO_OWSR_SPEC>;
#[doc = "Output Write Status Register"]
pub mod pio_owsr;
#[doc = "PIO_AIMER register accessor: an alias for `Reg<PIO_AIMER_SPEC>`"]
pub type PIO_AIMER = crate::Reg<pio_aimer::PIO_AIMER_SPEC>;
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod pio_aimer;
#[doc = "PIO_AIMDR register accessor: an alias for `Reg<PIO_AIMDR_SPEC>`"]
pub type PIO_AIMDR = crate::Reg<pio_aimdr::PIO_AIMDR_SPEC>;
#[doc = "Additional Interrupt Modes Disable Register"]
pub mod pio_aimdr;
#[doc = "PIO_AIMMR register accessor: an alias for `Reg<PIO_AIMMR_SPEC>`"]
pub type PIO_AIMMR = crate::Reg<pio_aimmr::PIO_AIMMR_SPEC>;
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod pio_aimmr;
#[doc = "PIO_ESR register accessor: an alias for `Reg<PIO_ESR_SPEC>`"]
pub type PIO_ESR = crate::Reg<pio_esr::PIO_ESR_SPEC>;
#[doc = "Edge Select Register"]
pub mod pio_esr;
#[doc = "PIO_LSR register accessor: an alias for `Reg<PIO_LSR_SPEC>`"]
pub type PIO_LSR = crate::Reg<pio_lsr::PIO_LSR_SPEC>;
#[doc = "Level Select Register"]
pub mod pio_lsr;
#[doc = "PIO_ELSR register accessor: an alias for `Reg<PIO_ELSR_SPEC>`"]
pub type PIO_ELSR = crate::Reg<pio_elsr::PIO_ELSR_SPEC>;
#[doc = "Edge/Level Status Register"]
pub mod pio_elsr;
#[doc = "PIO_FELLSR register accessor: an alias for `Reg<PIO_FELLSR_SPEC>`"]
pub type PIO_FELLSR = crate::Reg<pio_fellsr::PIO_FELLSR_SPEC>;
#[doc = "Falling Edge/Low-Level Select Register"]
pub mod pio_fellsr;
#[doc = "PIO_REHLSR register accessor: an alias for `Reg<PIO_REHLSR_SPEC>`"]
pub type PIO_REHLSR = crate::Reg<pio_rehlsr::PIO_REHLSR_SPEC>;
#[doc = "Rising Edge/High-Level Select Register"]
pub mod pio_rehlsr;
#[doc = "PIO_FRLHSR register accessor: an alias for `Reg<PIO_FRLHSR_SPEC>`"]
pub type PIO_FRLHSR = crate::Reg<pio_frlhsr::PIO_FRLHSR_SPEC>;
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod pio_frlhsr;
#[doc = "PIO_LOCKSR register accessor: an alias for `Reg<PIO_LOCKSR_SPEC>`"]
pub type PIO_LOCKSR = crate::Reg<pio_locksr::PIO_LOCKSR_SPEC>;
#[doc = "Lock Status"]
pub mod pio_locksr;
#[doc = "PIO_WPMR register accessor: an alias for `Reg<PIO_WPMR_SPEC>`"]
pub type PIO_WPMR = crate::Reg<pio_wpmr::PIO_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod pio_wpmr;
#[doc = "PIO_WPSR register accessor: an alias for `Reg<PIO_WPSR_SPEC>`"]
pub type PIO_WPSR = crate::Reg<pio_wpsr::PIO_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod pio_wpsr;
#[doc = "PIO_SCHMITT register accessor: an alias for `Reg<PIO_SCHMITT_SPEC>`"]
pub type PIO_SCHMITT = crate::Reg<pio_schmitt::PIO_SCHMITT_SPEC>;
#[doc = "Schmitt Trigger Register"]
pub mod pio_schmitt;
#[doc = "PIO_DRIVER register accessor: an alias for `Reg<PIO_DRIVER_SPEC>`"]
pub type PIO_DRIVER = crate::Reg<pio_driver::PIO_DRIVER_SPEC>;
#[doc = "I/O Drive Register"]
pub mod pio_driver;
#[doc = "PIO_PCMR register accessor: an alias for `Reg<PIO_PCMR_SPEC>`"]
pub type PIO_PCMR = crate::Reg<pio_pcmr::PIO_PCMR_SPEC>;
#[doc = "Parallel Capture Mode Register"]
pub mod pio_pcmr;
#[doc = "PIO_PCIER register accessor: an alias for `Reg<PIO_PCIER_SPEC>`"]
pub type PIO_PCIER = crate::Reg<pio_pcier::PIO_PCIER_SPEC>;
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pio_pcier;
#[doc = "PIO_PCIDR register accessor: an alias for `Reg<PIO_PCIDR_SPEC>`"]
pub type PIO_PCIDR = crate::Reg<pio_pcidr::PIO_PCIDR_SPEC>;
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pio_pcidr;
#[doc = "PIO_PCIMR register accessor: an alias for `Reg<PIO_PCIMR_SPEC>`"]
pub type PIO_PCIMR = crate::Reg<pio_pcimr::PIO_PCIMR_SPEC>;
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pio_pcimr;
#[doc = "PIO_PCISR register accessor: an alias for `Reg<PIO_PCISR_SPEC>`"]
pub type PIO_PCISR = crate::Reg<pio_pcisr::PIO_PCISR_SPEC>;
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pio_pcisr;
#[doc = "PIO_PCRHR register accessor: an alias for `Reg<PIO_PCRHR_SPEC>`"]
pub type PIO_PCRHR = crate::Reg<pio_pcrhr::PIO_PCRHR_SPEC>;
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pio_pcrhr;
