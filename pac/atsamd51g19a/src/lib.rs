#![doc = "Peripheral access API for ATSAMD51G19A microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn PM();
    fn MCLK();
    fn OSCCTRL_XOSC0();
    fn OSCCTRL_XOSC1();
    fn OSCCTRL_DFLL();
    fn OSCCTRL_DPLL0();
    fn OSCCTRL_DPLL1();
    fn OSC32KCTRL();
    fn SUPC_OTHER();
    fn SUPC_BODDET();
    fn WDT();
    fn RTC();
    fn EIC_EXTINT_0();
    fn EIC_EXTINT_1();
    fn EIC_EXTINT_2();
    fn EIC_EXTINT_3();
    fn EIC_EXTINT_4();
    fn EIC_EXTINT_5();
    fn EIC_EXTINT_6();
    fn EIC_EXTINT_7();
    fn EIC_EXTINT_8();
    fn EIC_EXTINT_9();
    fn EIC_EXTINT_10();
    fn EIC_EXTINT_11();
    fn EIC_EXTINT_12();
    fn EIC_EXTINT_13();
    fn EIC_EXTINT_14();
    fn EIC_EXTINT_15();
    fn FREQM();
    fn NVMCTRL_0();
    fn NVMCTRL_1();
    fn DMAC_0();
    fn DMAC_1();
    fn DMAC_2();
    fn DMAC_3();
    fn DMAC_OTHER();
    fn EVSYS_0();
    fn EVSYS_1();
    fn EVSYS_2();
    fn EVSYS_3();
    fn EVSYS_OTHER();
    fn PAC();
    fn RAMECC();
    fn SERCOM0_0();
    fn SERCOM0_1();
    fn SERCOM0_2();
    fn SERCOM0_OTHER();
    fn SERCOM1_0();
    fn SERCOM1_1();
    fn SERCOM1_2();
    fn SERCOM1_OTHER();
    fn SERCOM2_0();
    fn SERCOM2_1();
    fn SERCOM2_2();
    fn SERCOM2_OTHER();
    fn SERCOM3_0();
    fn SERCOM3_1();
    fn SERCOM3_2();
    fn SERCOM3_OTHER();
    fn SERCOM4_0();
    fn SERCOM4_1();
    fn SERCOM4_2();
    fn SERCOM4_OTHER();
    fn SERCOM5_0();
    fn SERCOM5_1();
    fn SERCOM5_2();
    fn SERCOM5_OTHER();
    fn USB_OTHER();
    fn USB_SOF_HSOF();
    fn USB_TRCPT0();
    fn USB_TRCPT1();
    fn TCC0_OTHER();
    fn TCC0_MC0();
    fn TCC0_MC1();
    fn TCC0_MC2();
    fn TCC0_MC3();
    fn TCC0_MC4();
    fn TCC0_MC5();
    fn TCC1_OTHER();
    fn TCC1_MC0();
    fn TCC1_MC1();
    fn TCC1_MC2();
    fn TCC1_MC3();
    fn TCC2_OTHER();
    fn TCC2_MC0();
    fn TCC2_MC1();
    fn TCC2_MC2();
    fn TC0();
    fn TC1();
    fn TC2();
    fn TC3();
    fn PDEC_OTHER();
    fn PDEC_MC0();
    fn PDEC_MC1();
    fn ADC0_OTHER();
    fn ADC0_RESRDY();
    fn ADC1_OTHER();
    fn ADC1_RESRDY();
    fn AC();
    fn DAC_OTHER();
    fn DAC_EMPTY_0();
    fn DAC_EMPTY_1();
    fn DAC_RESRDY_0();
    fn DAC_RESRDY_1();
    fn PCC();
    fn AES();
    fn TRNG();
    fn ICM();
    fn QSPI();
    fn SDHC0();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 136] = [
    Vector { _handler: PM },
    Vector { _handler: MCLK },
    Vector {
        _handler: OSCCTRL_XOSC0,
    },
    Vector {
        _handler: OSCCTRL_XOSC1,
    },
    Vector {
        _handler: OSCCTRL_DFLL,
    },
    Vector {
        _handler: OSCCTRL_DPLL0,
    },
    Vector {
        _handler: OSCCTRL_DPLL1,
    },
    Vector {
        _handler: OSC32KCTRL,
    },
    Vector {
        _handler: SUPC_OTHER,
    },
    Vector {
        _handler: SUPC_BODDET,
    },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector {
        _handler: EIC_EXTINT_0,
    },
    Vector {
        _handler: EIC_EXTINT_1,
    },
    Vector {
        _handler: EIC_EXTINT_2,
    },
    Vector {
        _handler: EIC_EXTINT_3,
    },
    Vector {
        _handler: EIC_EXTINT_4,
    },
    Vector {
        _handler: EIC_EXTINT_5,
    },
    Vector {
        _handler: EIC_EXTINT_6,
    },
    Vector {
        _handler: EIC_EXTINT_7,
    },
    Vector {
        _handler: EIC_EXTINT_8,
    },
    Vector {
        _handler: EIC_EXTINT_9,
    },
    Vector {
        _handler: EIC_EXTINT_10,
    },
    Vector {
        _handler: EIC_EXTINT_11,
    },
    Vector {
        _handler: EIC_EXTINT_12,
    },
    Vector {
        _handler: EIC_EXTINT_13,
    },
    Vector {
        _handler: EIC_EXTINT_14,
    },
    Vector {
        _handler: EIC_EXTINT_15,
    },
    Vector { _handler: FREQM },
    Vector {
        _handler: NVMCTRL_0,
    },
    Vector {
        _handler: NVMCTRL_1,
    },
    Vector { _handler: DMAC_0 },
    Vector { _handler: DMAC_1 },
    Vector { _handler: DMAC_2 },
    Vector { _handler: DMAC_3 },
    Vector {
        _handler: DMAC_OTHER,
    },
    Vector { _handler: EVSYS_0 },
    Vector { _handler: EVSYS_1 },
    Vector { _handler: EVSYS_2 },
    Vector { _handler: EVSYS_3 },
    Vector {
        _handler: EVSYS_OTHER,
    },
    Vector { _handler: PAC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RAMECC },
    Vector {
        _handler: SERCOM0_0,
    },
    Vector {
        _handler: SERCOM0_1,
    },
    Vector {
        _handler: SERCOM0_2,
    },
    Vector {
        _handler: SERCOM0_OTHER,
    },
    Vector {
        _handler: SERCOM1_0,
    },
    Vector {
        _handler: SERCOM1_1,
    },
    Vector {
        _handler: SERCOM1_2,
    },
    Vector {
        _handler: SERCOM1_OTHER,
    },
    Vector {
        _handler: SERCOM2_0,
    },
    Vector {
        _handler: SERCOM2_1,
    },
    Vector {
        _handler: SERCOM2_2,
    },
    Vector {
        _handler: SERCOM2_OTHER,
    },
    Vector {
        _handler: SERCOM3_0,
    },
    Vector {
        _handler: SERCOM3_1,
    },
    Vector {
        _handler: SERCOM3_2,
    },
    Vector {
        _handler: SERCOM3_OTHER,
    },
    Vector {
        _handler: SERCOM4_0,
    },
    Vector {
        _handler: SERCOM4_1,
    },
    Vector {
        _handler: SERCOM4_2,
    },
    Vector {
        _handler: SERCOM4_OTHER,
    },
    Vector {
        _handler: SERCOM5_0,
    },
    Vector {
        _handler: SERCOM5_1,
    },
    Vector {
        _handler: SERCOM5_2,
    },
    Vector {
        _handler: SERCOM5_OTHER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USB_OTHER,
    },
    Vector {
        _handler: USB_SOF_HSOF,
    },
    Vector {
        _handler: USB_TRCPT0,
    },
    Vector {
        _handler: USB_TRCPT1,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: TCC0_OTHER,
    },
    Vector { _handler: TCC0_MC0 },
    Vector { _handler: TCC0_MC1 },
    Vector { _handler: TCC0_MC2 },
    Vector { _handler: TCC0_MC3 },
    Vector { _handler: TCC0_MC4 },
    Vector { _handler: TCC0_MC5 },
    Vector {
        _handler: TCC1_OTHER,
    },
    Vector { _handler: TCC1_MC0 },
    Vector { _handler: TCC1_MC1 },
    Vector { _handler: TCC1_MC2 },
    Vector { _handler: TCC1_MC3 },
    Vector {
        _handler: TCC2_OTHER,
    },
    Vector { _handler: TCC2_MC0 },
    Vector { _handler: TCC2_MC1 },
    Vector { _handler: TCC2_MC2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector { _handler: TC3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: PDEC_OTHER,
    },
    Vector { _handler: PDEC_MC0 },
    Vector { _handler: PDEC_MC1 },
    Vector {
        _handler: ADC0_OTHER,
    },
    Vector {
        _handler: ADC0_RESRDY,
    },
    Vector {
        _handler: ADC1_OTHER,
    },
    Vector {
        _handler: ADC1_RESRDY,
    },
    Vector { _handler: AC },
    Vector {
        _handler: DAC_OTHER,
    },
    Vector {
        _handler: DAC_EMPTY_0,
    },
    Vector {
        _handler: DAC_EMPTY_1,
    },
    Vector {
        _handler: DAC_RESRDY_0,
    },
    Vector {
        _handler: DAC_RESRDY_1,
    },
    Vector { _reserved: 0 },
    Vector { _handler: PCC },
    Vector { _handler: AES },
    Vector { _handler: TRNG },
    Vector { _handler: ICM },
    Vector { _reserved: 0 },
    Vector { _handler: QSPI },
    Vector { _handler: SDHC0 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - PM"]
    PM,
    #[doc = "1 - MCLK"]
    MCLK,
    #[doc = "2 - OSCCTRL_XOSC0"]
    OSCCTRL_XOSC0,
    #[doc = "3 - OSCCTRL_XOSC1"]
    OSCCTRL_XOSC1,
    #[doc = "4 - OSCCTRL_DFLL"]
    OSCCTRL_DFLL,
    #[doc = "5 - OSCCTRL_DPLL0"]
    OSCCTRL_DPLL0,
    #[doc = "6 - OSCCTRL_DPLL1"]
    OSCCTRL_DPLL1,
    #[doc = "7 - OSC32KCTRL"]
    OSC32KCTRL,
    #[doc = "8 - SUPC_OTHER"]
    SUPC_OTHER,
    #[doc = "9 - SUPC_BODDET"]
    SUPC_BODDET,
    #[doc = "10 - WDT"]
    WDT,
    #[doc = "11 - RTC"]
    RTC,
    #[doc = "12 - EIC_EXTINT_0"]
    EIC_EXTINT_0,
    #[doc = "13 - EIC_EXTINT_1"]
    EIC_EXTINT_1,
    #[doc = "14 - EIC_EXTINT_2"]
    EIC_EXTINT_2,
    #[doc = "15 - EIC_EXTINT_3"]
    EIC_EXTINT_3,
    #[doc = "16 - EIC_EXTINT_4"]
    EIC_EXTINT_4,
    #[doc = "17 - EIC_EXTINT_5"]
    EIC_EXTINT_5,
    #[doc = "18 - EIC_EXTINT_6"]
    EIC_EXTINT_6,
    #[doc = "19 - EIC_EXTINT_7"]
    EIC_EXTINT_7,
    #[doc = "20 - EIC_EXTINT_8"]
    EIC_EXTINT_8,
    #[doc = "21 - EIC_EXTINT_9"]
    EIC_EXTINT_9,
    #[doc = "22 - EIC_EXTINT_10"]
    EIC_EXTINT_10,
    #[doc = "23 - EIC_EXTINT_11"]
    EIC_EXTINT_11,
    #[doc = "24 - EIC_EXTINT_12"]
    EIC_EXTINT_12,
    #[doc = "25 - EIC_EXTINT_13"]
    EIC_EXTINT_13,
    #[doc = "26 - EIC_EXTINT_14"]
    EIC_EXTINT_14,
    #[doc = "27 - EIC_EXTINT_15"]
    EIC_EXTINT_15,
    #[doc = "28 - FREQM"]
    FREQM,
    #[doc = "29 - NVMCTRL_0"]
    NVMCTRL_0,
    #[doc = "30 - NVMCTRL_1"]
    NVMCTRL_1,
    #[doc = "31 - DMAC_0"]
    DMAC_0,
    #[doc = "32 - DMAC_1"]
    DMAC_1,
    #[doc = "33 - DMAC_2"]
    DMAC_2,
    #[doc = "34 - DMAC_3"]
    DMAC_3,
    #[doc = "35 - DMAC_OTHER"]
    DMAC_OTHER,
    #[doc = "36 - EVSYS_0"]
    EVSYS_0,
    #[doc = "37 - EVSYS_1"]
    EVSYS_1,
    #[doc = "38 - EVSYS_2"]
    EVSYS_2,
    #[doc = "39 - EVSYS_3"]
    EVSYS_3,
    #[doc = "40 - EVSYS_OTHER"]
    EVSYS_OTHER,
    #[doc = "41 - PAC"]
    PAC,
    #[doc = "45 - RAMECC"]
    RAMECC,
    #[doc = "46 - SERCOM0_0"]
    SERCOM0_0,
    #[doc = "47 - SERCOM0_1"]
    SERCOM0_1,
    #[doc = "48 - SERCOM0_2"]
    SERCOM0_2,
    #[doc = "49 - SERCOM0_OTHER"]
    SERCOM0_OTHER,
    #[doc = "50 - SERCOM1_0"]
    SERCOM1_0,
    #[doc = "51 - SERCOM1_1"]
    SERCOM1_1,
    #[doc = "52 - SERCOM1_2"]
    SERCOM1_2,
    #[doc = "53 - SERCOM1_OTHER"]
    SERCOM1_OTHER,
    #[doc = "54 - SERCOM2_0"]
    SERCOM2_0,
    #[doc = "55 - SERCOM2_1"]
    SERCOM2_1,
    #[doc = "56 - SERCOM2_2"]
    SERCOM2_2,
    #[doc = "57 - SERCOM2_OTHER"]
    SERCOM2_OTHER,
    #[doc = "58 - SERCOM3_0"]
    SERCOM3_0,
    #[doc = "59 - SERCOM3_1"]
    SERCOM3_1,
    #[doc = "60 - SERCOM3_2"]
    SERCOM3_2,
    #[doc = "61 - SERCOM3_OTHER"]
    SERCOM3_OTHER,
    #[doc = "62 - SERCOM4_0"]
    SERCOM4_0,
    #[doc = "63 - SERCOM4_1"]
    SERCOM4_1,
    #[doc = "64 - SERCOM4_2"]
    SERCOM4_2,
    #[doc = "65 - SERCOM4_OTHER"]
    SERCOM4_OTHER,
    #[doc = "66 - SERCOM5_0"]
    SERCOM5_0,
    #[doc = "67 - SERCOM5_1"]
    SERCOM5_1,
    #[doc = "68 - SERCOM5_2"]
    SERCOM5_2,
    #[doc = "69 - SERCOM5_OTHER"]
    SERCOM5_OTHER,
    #[doc = "80 - USB_OTHER"]
    USB_OTHER,
    #[doc = "81 - USB_SOF_HSOF"]
    USB_SOF_HSOF,
    #[doc = "82 - USB_TRCPT0"]
    USB_TRCPT0,
    #[doc = "83 - USB_TRCPT1"]
    USB_TRCPT1,
    #[doc = "85 - TCC0_OTHER"]
    TCC0_OTHER,
    #[doc = "86 - TCC0_MC0"]
    TCC0_MC0,
    #[doc = "87 - TCC0_MC1"]
    TCC0_MC1,
    #[doc = "88 - TCC0_MC2"]
    TCC0_MC2,
    #[doc = "89 - TCC0_MC3"]
    TCC0_MC3,
    #[doc = "90 - TCC0_MC4"]
    TCC0_MC4,
    #[doc = "91 - TCC0_MC5"]
    TCC0_MC5,
    #[doc = "92 - TCC1_OTHER"]
    TCC1_OTHER,
    #[doc = "93 - TCC1_MC0"]
    TCC1_MC0,
    #[doc = "94 - TCC1_MC1"]
    TCC1_MC1,
    #[doc = "95 - TCC1_MC2"]
    TCC1_MC2,
    #[doc = "96 - TCC1_MC3"]
    TCC1_MC3,
    #[doc = "97 - TCC2_OTHER"]
    TCC2_OTHER,
    #[doc = "98 - TCC2_MC0"]
    TCC2_MC0,
    #[doc = "99 - TCC2_MC1"]
    TCC2_MC1,
    #[doc = "100 - TCC2_MC2"]
    TCC2_MC2,
    #[doc = "107 - TC0"]
    TC0,
    #[doc = "108 - TC1"]
    TC1,
    #[doc = "109 - TC2"]
    TC2,
    #[doc = "110 - TC3"]
    TC3,
    #[doc = "115 - PDEC_OTHER"]
    PDEC_OTHER,
    #[doc = "116 - PDEC_MC0"]
    PDEC_MC0,
    #[doc = "117 - PDEC_MC1"]
    PDEC_MC1,
    #[doc = "118 - ADC0_OTHER"]
    ADC0_OTHER,
    #[doc = "119 - ADC0_RESRDY"]
    ADC0_RESRDY,
    #[doc = "120 - ADC1_OTHER"]
    ADC1_OTHER,
    #[doc = "121 - ADC1_RESRDY"]
    ADC1_RESRDY,
    #[doc = "122 - AC"]
    AC,
    #[doc = "123 - DAC_OTHER"]
    DAC_OTHER,
    #[doc = "124 - DAC_EMPTY_0"]
    DAC_EMPTY_0,
    #[doc = "125 - DAC_EMPTY_1"]
    DAC_EMPTY_1,
    #[doc = "126 - DAC_RESRDY_0"]
    DAC_RESRDY_0,
    #[doc = "127 - DAC_RESRDY_1"]
    DAC_RESRDY_1,
    #[doc = "129 - PCC"]
    PCC,
    #[doc = "130 - AES"]
    AES,
    #[doc = "131 - TRNG"]
    TRNG,
    #[doc = "132 - ICM"]
    ICM,
    #[doc = "134 - QSPI"]
    QSPI,
    #[doc = "135 - SDHC0"]
    SDHC0,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PM => 0,
            Interrupt::MCLK => 1,
            Interrupt::OSCCTRL_XOSC0 => 2,
            Interrupt::OSCCTRL_XOSC1 => 3,
            Interrupt::OSCCTRL_DFLL => 4,
            Interrupt::OSCCTRL_DPLL0 => 5,
            Interrupt::OSCCTRL_DPLL1 => 6,
            Interrupt::OSC32KCTRL => 7,
            Interrupt::SUPC_OTHER => 8,
            Interrupt::SUPC_BODDET => 9,
            Interrupt::WDT => 10,
            Interrupt::RTC => 11,
            Interrupt::EIC_EXTINT_0 => 12,
            Interrupt::EIC_EXTINT_1 => 13,
            Interrupt::EIC_EXTINT_2 => 14,
            Interrupt::EIC_EXTINT_3 => 15,
            Interrupt::EIC_EXTINT_4 => 16,
            Interrupt::EIC_EXTINT_5 => 17,
            Interrupt::EIC_EXTINT_6 => 18,
            Interrupt::EIC_EXTINT_7 => 19,
            Interrupt::EIC_EXTINT_8 => 20,
            Interrupt::EIC_EXTINT_9 => 21,
            Interrupt::EIC_EXTINT_10 => 22,
            Interrupt::EIC_EXTINT_11 => 23,
            Interrupt::EIC_EXTINT_12 => 24,
            Interrupt::EIC_EXTINT_13 => 25,
            Interrupt::EIC_EXTINT_14 => 26,
            Interrupt::EIC_EXTINT_15 => 27,
            Interrupt::FREQM => 28,
            Interrupt::NVMCTRL_0 => 29,
            Interrupt::NVMCTRL_1 => 30,
            Interrupt::DMAC_0 => 31,
            Interrupt::DMAC_1 => 32,
            Interrupt::DMAC_2 => 33,
            Interrupt::DMAC_3 => 34,
            Interrupt::DMAC_OTHER => 35,
            Interrupt::EVSYS_0 => 36,
            Interrupt::EVSYS_1 => 37,
            Interrupt::EVSYS_2 => 38,
            Interrupt::EVSYS_3 => 39,
            Interrupt::EVSYS_OTHER => 40,
            Interrupt::PAC => 41,
            Interrupt::RAMECC => 45,
            Interrupt::SERCOM0_0 => 46,
            Interrupt::SERCOM0_1 => 47,
            Interrupt::SERCOM0_2 => 48,
            Interrupt::SERCOM0_OTHER => 49,
            Interrupt::SERCOM1_0 => 50,
            Interrupt::SERCOM1_1 => 51,
            Interrupt::SERCOM1_2 => 52,
            Interrupt::SERCOM1_OTHER => 53,
            Interrupt::SERCOM2_0 => 54,
            Interrupt::SERCOM2_1 => 55,
            Interrupt::SERCOM2_2 => 56,
            Interrupt::SERCOM2_OTHER => 57,
            Interrupt::SERCOM3_0 => 58,
            Interrupt::SERCOM3_1 => 59,
            Interrupt::SERCOM3_2 => 60,
            Interrupt::SERCOM3_OTHER => 61,
            Interrupt::SERCOM4_0 => 62,
            Interrupt::SERCOM4_1 => 63,
            Interrupt::SERCOM4_2 => 64,
            Interrupt::SERCOM4_OTHER => 65,
            Interrupt::SERCOM5_0 => 66,
            Interrupt::SERCOM5_1 => 67,
            Interrupt::SERCOM5_2 => 68,
            Interrupt::SERCOM5_OTHER => 69,
            Interrupt::USB_OTHER => 80,
            Interrupt::USB_SOF_HSOF => 81,
            Interrupt::USB_TRCPT0 => 82,
            Interrupt::USB_TRCPT1 => 83,
            Interrupt::TCC0_OTHER => 85,
            Interrupt::TCC0_MC0 => 86,
            Interrupt::TCC0_MC1 => 87,
            Interrupt::TCC0_MC2 => 88,
            Interrupt::TCC0_MC3 => 89,
            Interrupt::TCC0_MC4 => 90,
            Interrupt::TCC0_MC5 => 91,
            Interrupt::TCC1_OTHER => 92,
            Interrupt::TCC1_MC0 => 93,
            Interrupt::TCC1_MC1 => 94,
            Interrupt::TCC1_MC2 => 95,
            Interrupt::TCC1_MC3 => 96,
            Interrupt::TCC2_OTHER => 97,
            Interrupt::TCC2_MC0 => 98,
            Interrupt::TCC2_MC1 => 99,
            Interrupt::TCC2_MC2 => 100,
            Interrupt::TC0 => 107,
            Interrupt::TC1 => 108,
            Interrupt::TC2 => 109,
            Interrupt::TC3 => 110,
            Interrupt::PDEC_OTHER => 115,
            Interrupt::PDEC_MC0 => 116,
            Interrupt::PDEC_MC1 => 117,
            Interrupt::ADC0_OTHER => 118,
            Interrupt::ADC0_RESRDY => 119,
            Interrupt::ADC1_OTHER => 120,
            Interrupt::ADC1_RESRDY => 121,
            Interrupt::AC => 122,
            Interrupt::DAC_OTHER => 123,
            Interrupt::DAC_EMPTY_0 => 124,
            Interrupt::DAC_EMPTY_1 => 125,
            Interrupt::DAC_RESRDY_0 => 126,
            Interrupt::DAC_RESRDY_1 => 127,
            Interrupt::PCC => 129,
            Interrupt::AES => 130,
            Interrupt::TRNG => 131,
            Interrupt::ICM => 132,
            Interrupt::QSPI => 134,
            Interrupt::SDHC0 => 135,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog Comparators"]
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ac::RegisterBlock {
        0x4200_2000 as *const _
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AC::ptr() }
    }
}
#[doc = "Analog Comparators"]
pub mod ac;
#[doc = "Analog Digital Converter 0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4300_1c00 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog Digital Converter 0"]
pub mod adc0;
#[doc = "Analog Digital Converter 1"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4300_2000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4200_2400 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub mod aes;
#[doc = "Configurable Custom Logic"]
pub struct CCL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCL {}
impl CCL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccl::RegisterBlock {
        0x4200_3800 as *const _
    }
}
impl Deref for CCL {
    type Target = ccl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCL::ptr() }
    }
}
#[doc = "Configurable Custom Logic"]
pub mod ccl;
#[doc = "Cortex M Cache Controller"]
pub struct CMCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMCC {}
impl CMCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmcc::RegisterBlock {
        0x4100_6000 as *const _
    }
}
impl Deref for CMCC {
    type Target = cmcc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMCC::ptr() }
    }
}
#[doc = "Cortex M Cache Controller"]
pub mod cmcc;
#[doc = "Digital-to-Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4300_2400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter"]
pub mod dac;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        0x4100_a000 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Device Service Unit"]
pub struct DSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSU {}
impl DSU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsu::RegisterBlock {
        0x4100_2000 as *const _
    }
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSU::ptr() }
    }
}
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eic::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Event System Interface"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const evsys::RegisterBlock {
        0x4100_e000 as *const _
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EVSYS::ptr() }
    }
}
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "Frequency Meter"]
pub struct FREQM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FREQM {}
impl FREQM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const freqm::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for FREQM {
    type Target = freqm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FREQM::ptr() }
    }
}
#[doc = "Frequency Meter"]
pub mod freqm;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCLK {}
impl GCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gclk::RegisterBlock {
        0x4000_1c00 as *const _
    }
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GCLK::ptr() }
    }
}
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "HSB Matrix"]
pub struct HMATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIX {}
impl HMATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmatrix::RegisterBlock {
        0x4100_c000 as *const _
    }
}
impl Deref for HMATRIX {
    type Target = hmatrix::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*HMATRIX::ptr() }
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrix;
#[doc = "Integrity Check Monitor"]
pub struct ICM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICM {}
impl ICM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icm::RegisterBlock {
        0x4200_2c00 as *const _
    }
}
impl Deref for ICM {
    type Target = icm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICM::ptr() }
    }
}
#[doc = "Integrity Check Monitor"]
pub mod icm;
#[doc = "Main Clock"]
pub struct MCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCLK {}
impl MCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mclk::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for MCLK {
    type Target = mclk::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCLK::ptr() }
    }
}
#[doc = "Main Clock"]
pub mod mclk;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmctrl::RegisterBlock {
        0x4100_4000 as *const _
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMCTRL::ptr() }
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "Oscillators Control"]
pub struct OSCCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSCCTRL {}
impl OSCCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oscctrl::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for OSCCTRL {
    type Target = oscctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSCCTRL::ptr() }
    }
}
#[doc = "Oscillators Control"]
pub mod oscctrl;
#[doc = "32kHz Oscillators Control"]
pub struct OSC32KCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC32KCTRL {}
impl OSC32KCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const osc32kctrl::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for OSC32KCTRL {
    type Target = osc32kctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSC32KCTRL::ptr() }
    }
}
#[doc = "32kHz Oscillators Control"]
pub mod osc32kctrl;
#[doc = "Peripheral Access Controller"]
pub struct PAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC {}
impl PAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for PAC {
    type Target = pac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PAC::ptr() }
    }
}
#[doc = "Peripheral Access Controller"]
pub mod pac;
#[doc = "Parallel Capture Controller"]
pub struct PCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCC {}
impl PCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcc::RegisterBlock {
        0x4300_2c00 as *const _
    }
}
impl Deref for PCC {
    type Target = pcc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCC::ptr() }
    }
}
#[doc = "Parallel Capture Controller"]
pub mod pcc;
#[doc = "Quadrature Decodeur"]
pub struct PDEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDEC {}
impl PDEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdec::RegisterBlock {
        0x4200_1c00 as *const _
    }
}
impl Deref for PDEC {
    type Target = pdec::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDEC::ptr() }
    }
}
#[doc = "Quadrature Decodeur"]
pub mod pdec;
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pm::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Port Module"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        0x4100_8000 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Port Module"]
pub mod port;
#[doc = "Quad SPI interface"]
pub struct QSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI {}
impl QSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi::RegisterBlock {
        0x4200_3400 as *const _
    }
}
impl Deref for QSPI {
    type Target = qspi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI::ptr() }
    }
}
#[doc = "Quad SPI interface"]
pub mod qspi;
#[doc = "RAM ECC"]
pub struct RAMECC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RAMECC {}
impl RAMECC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ramecc::RegisterBlock {
        0x4102_0000 as *const _
    }
}
impl Deref for RAMECC {
    type Target = ramecc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RAMECC::ptr() }
    }
}
#[doc = "RAM ECC"]
pub mod ramecc;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2400 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "SD/MMC Host Controller"]
pub struct SDHC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDHC0 {}
impl SDHC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdhc0::RegisterBlock {
        0x4500_0000 as *const _
    }
}
impl Deref for SDHC0 {
    type Target = sdhc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDHC0::ptr() }
    }
}
#[doc = "SD/MMC Host Controller"]
pub mod sdhc0;
#[doc = "Serial Communication Interface 0"]
pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM0 {}
impl SERCOM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for SERCOM0 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM0::ptr() }
    }
}
#[doc = "Serial Communication Interface 0"]
pub mod sercom0;
#[doc = "Serial Communication Interface 1"]
pub struct SERCOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM1 {}
impl SERCOM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4000_3400 as *const _
    }
}
impl Deref for SERCOM1 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM1::ptr() }
    }
}
#[doc = "Serial Communication Interface 2"]
pub struct SERCOM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM2 {}
impl SERCOM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4101_2000 as *const _
    }
}
impl Deref for SERCOM2 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM2::ptr() }
    }
}
#[doc = "Serial Communication Interface 3"]
pub struct SERCOM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM3 {}
impl SERCOM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4101_4000 as *const _
    }
}
impl Deref for SERCOM3 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM3::ptr() }
    }
}
#[doc = "Serial Communication Interface 4"]
pub struct SERCOM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM4 {}
impl SERCOM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0000 as *const _
    }
}
impl Deref for SERCOM4 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM4::ptr() }
    }
}
#[doc = "Serial Communication Interface 5"]
pub struct SERCOM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM5 {}
impl SERCOM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0400 as *const _
    }
}
impl Deref for SERCOM5 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM5::ptr() }
    }
}
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x4000_1800 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Basic Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Basic Timer Counter 0"]
pub mod tc0;
#[doc = "Basic Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Basic Timer Counter 2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4101_a000 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Basic Timer Counter 3"]
pub struct TC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC3 {}
impl TC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4101_c000 as *const _
    }
}
impl Deref for TC3 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC3::ptr() }
    }
}
#[doc = "Timer Counter Control 0"]
pub struct TCC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC0 {}
impl TCC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4101_6000 as *const _
    }
}
impl Deref for TCC0 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC0::ptr() }
    }
}
#[doc = "Timer Counter Control 0"]
pub mod tcc0;
#[doc = "Timer Counter Control 1"]
pub struct TCC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC1 {}
impl TCC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4101_8000 as *const _
    }
}
impl Deref for TCC1 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC1::ptr() }
    }
}
#[doc = "Timer Counter Control 2"]
pub struct TCC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC2 {}
impl TCC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_0c00 as *const _
    }
}
impl Deref for TCC2 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC2::ptr() }
    }
}
#[doc = "True Random Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4200_2800 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Generator"]
pub mod trng;
#[doc = "Universal Serial Bus"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4100_0000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AC"]
    pub AC: AC,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "CCL"]
    pub CCL: CCL,
    #[doc = "CMCC"]
    pub CMCC: CMCC,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DSU"]
    pub DSU: DSU,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "FREQM"]
    pub FREQM: FREQM,
    #[doc = "GCLK"]
    pub GCLK: GCLK,
    #[doc = "HMATRIX"]
    pub HMATRIX: HMATRIX,
    #[doc = "ICM"]
    pub ICM: ICM,
    #[doc = "MCLK"]
    pub MCLK: MCLK,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "OSCCTRL"]
    pub OSCCTRL: OSCCTRL,
    #[doc = "OSC32KCTRL"]
    pub OSC32KCTRL: OSC32KCTRL,
    #[doc = "PAC"]
    pub PAC: PAC,
    #[doc = "PCC"]
    pub PCC: PCC,
    #[doc = "PDEC"]
    pub PDEC: PDEC,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "QSPI"]
    pub QSPI: QSPI,
    #[doc = "RAMECC"]
    pub RAMECC: RAMECC,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SDHC0"]
    pub SDHC0: SDHC0,
    #[doc = "SERCOM0"]
    pub SERCOM0: SERCOM0,
    #[doc = "SERCOM1"]
    pub SERCOM1: SERCOM1,
    #[doc = "SERCOM2"]
    pub SERCOM2: SERCOM2,
    #[doc = "SERCOM3"]
    pub SERCOM3: SERCOM3,
    #[doc = "SERCOM4"]
    pub SERCOM4: SERCOM4,
    #[doc = "SERCOM5"]
    pub SERCOM5: SERCOM5,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TC3"]
    pub TC3: TC3,
    #[doc = "TCC0"]
    pub TCC0: TCC0,
    #[doc = "TCC1"]
    pub TCC1: TCC1,
    #[doc = "TCC2"]
    pub TCC2: TCC2,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC: AC {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            CCL: CCL {
                _marker: PhantomData,
            },
            CMCC: CMCC {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DSU: DSU {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            FREQM: FREQM {
                _marker: PhantomData,
            },
            GCLK: GCLK {
                _marker: PhantomData,
            },
            HMATRIX: HMATRIX {
                _marker: PhantomData,
            },
            ICM: ICM {
                _marker: PhantomData,
            },
            MCLK: MCLK {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            OSCCTRL: OSCCTRL {
                _marker: PhantomData,
            },
            OSC32KCTRL: OSC32KCTRL {
                _marker: PhantomData,
            },
            PAC: PAC {
                _marker: PhantomData,
            },
            PCC: PCC {
                _marker: PhantomData,
            },
            PDEC: PDEC {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            QSPI: QSPI {
                _marker: PhantomData,
            },
            RAMECC: RAMECC {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SDHC0: SDHC0 {
                _marker: PhantomData,
            },
            SERCOM0: SERCOM0 {
                _marker: PhantomData,
            },
            SERCOM1: SERCOM1 {
                _marker: PhantomData,
            },
            SERCOM2: SERCOM2 {
                _marker: PhantomData,
            },
            SERCOM3: SERCOM3 {
                _marker: PhantomData,
            },
            SERCOM4: SERCOM4 {
                _marker: PhantomData,
            },
            SERCOM5: SERCOM5 {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TC3: TC3 {
                _marker: PhantomData,
            },
            TCC0: TCC0 {
                _marker: PhantomData,
            },
            TCC1: TCC1 {
                _marker: PhantomData,
            },
            TCC2: TCC2 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
