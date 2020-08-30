use crate::gpio::{self, IntoFunction, Port};
pub use crate::pad::PadPin;

// sercom0[0]:  PA04:D   PA08:C     PC17:D      PB24:C
// sercom0[1]:  PA05:D   PA09:C     PC16:D      PB25:C
// sercom0[2]:  PA06:D   PA10:C     PC18:D      PC24:C
// sercom0[3]:  PA07:D   PA11:C     PC19:D      PC25:C

pad!(Sercom0Pad0 {
    Pa4(PfD),
    Pa8(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc17(PfD),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb24(PfC),
});

pad!(Sercom0Pad1 {
    Pa5(PfD),
    Pa9(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc16(PfD),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb25(PfC),
});

pad!(Sercom0Pad2 {
    Pa6(PfD),
    Pa10(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc18(PfD),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc24(PfC),
});

pad!(Sercom0Pad3 {
    Pa7(PfD),
    Pa11(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc19(PfD),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc25(PfC),
});

// sercom1[0]:  PA00:D   PA16:C   PC22:C    PC27:C
// sercom1[1]:  PA01:D   PA17:C   PC23:C    PC28:C
// sercom1[2]:  PA18:C   PA30:D   PB22:C    PD20:C
// sercom1[3]:  PA19:C   PA31:D   PB23:C    PD21:C

pad!(Sercom1Pad0 {
    Pa0(PfD),
    Pa16(PfC),
    #[cfg(feature = "samd51p19a")]
    Pc22(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc27(PfC),
});

pad!(Sercom1Pad1 {
    Pa1(PfD),
    Pa17(PfC),
    #[cfg(feature = "samd51p19a")]
    Pc23(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc28(PfC),
});

pad!(Sercom1Pad2 {
    Pa18(PfC),
    Pa30(PfD),
    Pb22(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd20(PfC),
});

pad!(Sercom1Pad3 {
    Pa19(PfC),
    Pa31(PfD),
    Pb23(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd21(PfC),
});

// sercom2[0]:  PA09:D   PA12:C     PB25:D      PB26:C
// sercom2[1]:  PA08:D   PA13:C     PB24:D      PB27:C
// sercom2[2]:  PA10:D   PA14:C     PB28:C      PC24:D
// sercom2[3]:  PA11:D   PA15:C     PB29:C      PC25:D

pad!(Sercom2Pad0 {
    Pa9(PfD),
    Pa12(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb25(PfD),
    #[cfg(feature = "samd51p19a")]
    Pb26(PfC),
});

pad!(Sercom2Pad1 {
    Pa8(PfD),
    Pa13(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb24(PfD),
    #[cfg(feature = "samd51p19a")]
    Pb27(PfC),
});

pad!(Sercom2Pad2 {
    Pa10(PfD),
    Pa14(PfC),
    #[cfg(feature = "samd51p19a")]
    Pb28(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc24(PfD),
});

pad!(Sercom2Pad3 {
    Pa11(PfD),
    Pa15(PfC),
    #[cfg(feature = "samd51p19a")]
    Pb29(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pc25(PfD),
});

// sercom3[0]:  PA17:D   PA22:C   PB20:C    PC23:D
// sercom3[1]:  PA16:D   PA23:C   PB21:C    PC22:D
// sercom3[2]:  PA18:D   PA20:D   PA24:C    PD20:D
// sercom3[3]:  PA19:D   PA21:D   PA25:C    PD21:D

pad!(Sercom3Pad0 {
    Pa17(PfD),
    Pa22(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb20(PfC),
    #[cfg(feature = "samd51p19a")]
    Pc23(PfD),
});

pad!(Sercom3Pad1 {
    Pa16(PfD),
    Pa23(PfC),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb21(PfC),
    #[cfg(feature = "samd51p19a")]
    Pc22(PfD),
});

pad!(Sercom3Pad2 {
    Pa18(PfD),
    Pa20(PfD),
    Pa24(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd20(PfD),
});

pad!(Sercom3Pad3 {
    Pa19(PfD),
    Pa21(PfD),
    Pa25(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd21(PfD),
});

// sercom4[0]:  PA13:D   PB08:D   PB12:C    PB27:D
// sercom4[1]:  PA12:D   PB09:D   PB13:C    PB26:D
// sercom4[2]:  PA14:D   PB10:D   PB14:C    PB28:D
// sercom4[3]:  PA15:D   PB11:D   PB15:C    PB29:D

pad!(Sercom4Pad0 {
    Pa13(PfD),
    Pb8(PfD),
    Pb12(PfC),
    #[cfg(feature = "samd51p19a")]
    Pb27(PfD),
});

pad!(Sercom4Pad1 {
    Pa12(PfD),
    Pb9(PfD),
    Pb13(PfC),
    #[cfg(feature = "samd51p19a")]
    Pb26(PfD),
});

pad!(Sercom4Pad2 {
    Pa14(PfD),
    Pb10(PfD),
    Pb14(PfC),
    #[cfg(feature = "samd51p19a")]
    Pb28(PfD),
});

pad!(Sercom4Pad3 {
    Pa15(PfD),
    Pb11(PfD),
    Pb15(PfC),
    #[cfg(feature = "samd51p19a")]
    Pb29(PfD),
});

// sercom5[0]:  PA23:D   PB02:D   PB16:C   PB31:D
// sercom5[1]:  PA22:D   PB03:D   PB17:C   PB30:D
// sercom5[2]:  PA20:C   PA24:D   PB00:D   PB18:C   PB22:D
// sercom5[3]:  PA21:C   PA25:D   PB01:D   PB19:C   PB23:D

pad!(Sercom5Pad0 {
    Pa23(PfD),
    Pb2(PfD),
    Pb16(PfC),
    Pb31(PfD),
});

pad!(Sercom5Pad1 {
    Pa22(PfD),
    Pb3(PfD),
    Pb17(PfC),
    Pb30(PfD),
});

pad!(Sercom5Pad2 {
    Pa20(PfC),
    Pa24(PfD),
    Pb0(PfD),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb18(PfC),
    Pb22(PfD),
});

pad!(Sercom5Pad3 {
    Pa21(PfC),
    Pa25(PfD),
    Pb1(PfD),
    #[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
    Pb19(PfC),
    Pb23(PfD),
});

// sercom6[0]:  PC04:C   PC13:D   PC16:C   PD09:D
// sercom6[1]:  PC05:C   PC12:D   PC17:C   PD08:D
// sercom6[2]:  PC06:C   PC10:C   PC14:D   PC18:C   PD10:D
// sercom6[3]:  PC07:C   PC11:C   PC15:D   PC19:C   PD11:D

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom6Pad0 {
    #[cfg(feature = "samd51p19a")]
    Pc4(PfC),
    Pc13(PfD),
    Pc16(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd9(PfD),
});

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom6Pad1 {
    Pc5(PfC),
    Pc12(PfD),
    Pc17(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd8(PfD),
});

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom6Pad2 {
    Pc6(PfC),
    Pc10(PfC),
    Pc14(PfD),
    Pc18(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd10(PfD),
});

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom6Pad3 {
    Pc7(PfC),
    Pc11(PfC),
    Pc15(PfD),
    Pc19(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd11(PfD),
});

// sercom7[0]:  PB21:D   PC12:C   PD08:C
// sercom7[1]:  PB20:D   PC13:C   PD09:C
// sercom7[2]:  PB18:D   PC10:D   PC14:C   PD10:C
// sercom7[3]:  PC11:D   PC15:C   PD11:C   PB19:D

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom7Pad0 {
    Pb21(PfD),
    Pc12(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd8(PfC),
    Pb30(PfC),
});

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom7Pad1 {
    Pb20(PfD),
    Pc13(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd9(PfC),
    Pb31(PfC),
});

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom7Pad2 {
    Pb18(PfD),
    Pc10(PfD),
    Pc14(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd10(PfC),
});

#[cfg(any(feature = "samd51p19a", feature = "samd51n20a"))]
pad!(Sercom7Pad3 {
    Pc11(PfD),
    Pc15(PfC),
    #[cfg(feature = "samd51p19a")]
    Pd11(PfC),
    Pb19(PfD),
});
