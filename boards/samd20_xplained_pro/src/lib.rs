#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;

pub use hal::pac;

hal::bsp_pins!(
    PA00 {
        name: pa00,
        aliases: { AlternateB: Xin32 }
    },
    PA01 {
        name: pa01,
        aliases: { AlternateB: Xout32 }
    },
    PA02 { name: pa02 },
    PA03 { name: pa03 },
    PA04 { name: pa04 },
    PA05 { name: pa05 },
    PA06 { name: pa06 },
    PA07 { name: pa07 },
    PA08 {
        name: pa08,
        aliases: { AlternateB: Sercom2Pad0SDA }
    },
    PA09 {
        name: pa09,
        aliases: { AlternateB: Sercom2Pad1Scl }
    },
    PA10 { name: pa10 },
    PA11 { name: pa11 },
    PA12 { name: pa12 },
    PA13 { name: pa13 },
    PA14 {
        name: pa14,
        aliases: { AlternateB: Led0 }
    },
    PA15 {
        name: pa15,
        aliases: { AlternateB: Sw0 }
    },
    PA16 { name: pa16 },
    PA17 { name: pa17 },
    PA18 { name: pa18 },
    PA19 { name: pa19 },
    PA20 {
        name: pa20,
        aliases: { AlternateB: EdbgGpio2 }
    },
    PA21 {
        name: pa21,
        aliases: { AlternateB: EdbgGpio3 }
    },
    PA22 { name: pa22 },
    PA23 { name: pa23 },
    PA24 {
        name: pa24,
        aliases: { AlternateB: Sercom3Pad2UartTX }
    },
    PA25 {
        name: pa25,
        aliases: { AlternateB: Sercom3Pad3UartRX }
    },
    PA27 {
        name: pa27,
        aliases: { AlternateB: EdbgGpio0 }
    },
    PA28 {
        name: pa28,
        aliases: { AlternateB: EdbgGpio1 }
    },
    PA30 {
        name: pa30,
        aliases: { AlternateB: SwdClk }
    },
    PA31 {
        name: pa31,
        aliases: { AlternateB: SwdData }
    },
    PB00 { name: pb00 },
    PB01 { name: pb01 },
    PB02 { name: pb02 },
    PB03 { name: pb03 },
    PB04 { name: pb04 },
    PB05 { name: pb05 },
    PB06 { name: pb06 },
    PB07 { name: pb07 },
    PB08 { name: pb08 },
    PB09 { name: pb09 },
    PB10 { name: pb10 },
    PB11 { name: pb11 },
    PB12 { name: pb12 },
    PB13 { name: pb13 },
    PB14 { name: pb14 },
    PB15 { name: pb15 },
    PB16 {
        name: pb16,
        aliases: { AlternateB: Sercom5Pad0SpiMISO }
    },
    PB17 { name: pb17 },
    PB22 {
        name: pb22,
        aliases: { AlternateB: Sercom5Pad2SpiMOSI }
    },
    PB23 {
        name: pb23,
        aliases: { AlternateB: Sercom5Pad3SpiSCK }
    },
    PB30 { name: pb30 },
    PB31 {
        name: pb31,
        aliases: { AlternateB: Sercom5Pad1SpiSS }
    },
);
