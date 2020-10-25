#[rustfmt::skip]
use atsamd_hal::gpio::{
    Floating, Input,
    Pa8, Pa9, Pa10, Pa11, Pb10, Pb11, Pc16, Pc17, Pc18, Pc19, Pd21,
};

/// QSPI Flash pins (uses `SERCOM4`)
pub struct QSPIFlash {
    /// QSPI Flash `sck` pin
    pub sck: Pb10<Input<Floating>>,

    /// QSPI Flash chip select pin
    pub cs: Pb11<Input<Floating>>,

    /// QSPI Flash `d0` pin
    pub d0: Pa8<Input<Floating>>,

    /// QSPI Flash `d1` pin
    pub d1: Pa9<Input<Floating>>,

    /// QSPI Flash `d2` pin
    pub d2: Pa10<Input<Floating>>,

    /// QSPI Flash `d3` pin
    pub d3: Pa11<Input<Floating>>,
}

/// SD Card pins (uses `SERCOM6`)
pub struct SDCard {
    /// SD Card chip select pin
    pub cs: Pc19<Input<Floating>>,

    /// SD Card `mosi` pin
    pub mosi: Pc16<Input<Floating>>,

    /// SD Card `sck` pin
    pub sck: Pc17<Input<Floating>>,

    /// SD Card `miso` pin
    pub miso: Pc18<Input<Floating>>,

    /// SD Card detect pin
    pub det: Pd21<Input<Floating>>,
}
