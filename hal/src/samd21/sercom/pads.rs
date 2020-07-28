use crate::gpio::{self, IntoFunction, Port};
pub use crate::pad::PadPin;

// sercom0[0]:  PA04:D   PA08:C
// sercom0[1]:  PA05:D   PA09:C
// sercom0[2]:  PA06:D   PA10:C
// sercom0[3]:  PA07:D   PA11:C

pad!(Sercom0Pad0 {
    Pa4(PfD),
    Pa8(PfC),
});

pad!(Sercom0Pad1 {
    Pa5(PfD),
    Pa9(PfC),
});

pad!(Sercom0Pad2 {
    Pa6(PfD),
    Pa10(PfC),
});

pad!(Sercom0Pad3 {
    Pa7(PfD),
    Pa11(PfC),
});

// sercom1[0]:  PA16:C   PA00:D
// sercom1[1]:  PA17:C   PA01:D
// sercom1[2]:  PA18:C   PA30:D
// sercom1[3]:  PA19:C   PA31:D

pad!(Sercom1Pad0 {
    Pa0(PfD),
    Pa16(PfC),
});

pad!(Sercom1Pad1 {
    Pa1(PfD),
    Pa17(PfC),
});

pad!(Sercom1Pad2 {
    Pa18(PfC),
    Pa30(PfD),
});

pad!(Sercom1Pad3 {
    Pa19(PfC),
    Pa31(PfD),
});

// sercom2[0]:  PA12:C   PA08:D
// sercom2[1]:  PA13:C   PA09:D
// sercom2[2]:  PA14:C   PA10:D
// sercom2[3]:  PA15:C   PA11:D

pad!(Sercom2Pad0 {
    Pa8(PfD),
    Pa12(PfC),
});

pad!(Sercom2Pad1 {
    Pa9(PfD),
    Pa13(PfC),
});

pad!(Sercom2Pad2 {
    Pa10(PfD),
    Pa14(PfC),
});

pad!(Sercom2Pad3 {
    Pa11(PfD),
    Pa15(PfC),
});

// sercom3[0]:  PA16:D   PA22:C
// sercom3[1]:  PA17:D   PA23:C
// sercom3[2]:  PA18:D   PA24:C   PA20:D
// sercom3[3]:  PA19:D   PA25:C   PA21:D

pad!(Sercom3Pad0 {
    Pa16(PfD),
    Pa22(PfC),
});

pad!(Sercom3Pad1 {
    Pa17(PfD),
    Pa23(PfC),
});

pad!(Sercom3Pad2 {
    Pa18(PfD),
    Pa20(PfD),
    Pa24(PfC),
});

pad!(Sercom3Pad3 {
    Pa19(PfD),
    Pa21(PfD),
    Pa25(PfC),
});

// sercom4[0]:  PA12:D   PB08:D   PB12:C
// sercom4[1]:  PA13:D   PB09:D   PB13:C
// sercom4[2]:  PA14:D   PB10:D   PB14:C
// sercom4[3]:  PA15:D   PB11:D   PB15:C

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom4Pad0 {
    Pa12(PfD),
    Pb8(PfD),
    Pb12(PfC),
});

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom4Pad1 {
    Pa13(PfD),
    Pb9(PfD),
    Pb13(PfC),
});

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom4Pad2 {
    Pa14(PfD),
    Pb10(PfD),
    Pb14(PfC),
});

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom4Pad3 {
    Pa15(PfD),
    Pb11(PfD),
    Pb15(PfC),
});

// sercom5[0]:  PA22:D   PB02:D   PB16:C  PB30:D
// sercom5[1]:  PA23:D   PB03:D   PB17:C  PB31:D
// sercom5[2]:  PA24:D   PB00:D   PA20:C  PB22:D
// sercom5[3]:  PA25:D   PB01:D   PA21:C  PB23:D

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom5Pad0 {
    Pa22(PfD),
    Pb2(PfD),
    Pb16(PfC),
    Pb30(PfD),
});

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom5Pad1 {
    Pa23(PfD),
    Pb3(PfD),
    Pb17(PfC),
    Pb31(PfD),
});

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom5Pad2 {
    Pa24(PfD),
    Pb0(PfD),
    Pa20(PfC),
    Pb22(PfD),
});

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
pad!(Sercom5Pad3 {
    Pa25(PfD),
    Pb1(PfD),
    Pa21(PfC),
    Pb23(PfD),
});
