use crate::gpio::{self, IntoFunction, Port};
pub use crate::pad::PadPin;

// sercom0[0] PA08:C, PB24:C, PA04:D, PC17:D
// sercom0[1] PA09:C, PB25:C, PA05:D, PC16:D
// sercom0[2] PA10:C, PC24:C, PA06:D, PC18:D
// sercom0[3] PA11:C, PC25:C, PA07:D, PC19:D

pad!(Sercom0Pad0 {
    Pa8(PfC),
    Pb24(PfC),
    Pa4(PfD),
    Pc17(PfD),
});

pad!(Sercom0Pad1 {
    Pa9(PfC),
    Pb25(PfC),
    Pa5(PfD),
    Pc16(PfD),
});

pad!(Sercom0Pad2 {
    Pa10(PfC),
    Pc24(PfC),
    Pa6(PfD),
    Pc18(PfD),
});

pad!(Sercom0Pad3 {
    Pa11(PfC),
    Pc25(PfC),
    Pa7(PfD),
    Pc19(PfD),
});

// sercom1[0] PA16:C, PC22:C, PC27:C, PA00:D
// sercom1[1] PA17:C, PC23:C, PC28:C, PA01:D
// sercom1[2] PA18:C, PD20:C, PB22:C, PA30:D
// sercom1[3] PA19:C, PD21:C, PB23:C, PA31:D

pad!(Sercom1Pad0 {
    Pa16(PfC),
    Pc22(PfC),
    Pc27(PfC),
    Pa0(PfD),
});

pad!(Sercom1Pad1 {
    Pa17(PfC),
    Pc23(PfC),
    Pc28(PfC),
    Pa1(PfD),
});

pad!(Sercom1Pad2 {
    Pa18(PfC),
    Pd20(PfC),
    Pb22(PfC),
    Pa30(PfD),
});

pad!(Sercom1Pad3 {
    Pa19(PfC),
    Pd21(PfC),
    Pb23(PfC),
    Pa31(PfD),
});

// sercom2[0] PA12:C, PB26:C, PA09:D, PB25:D
// sercom2[1] PA13:C, PB27:C, PA08:D, PB24:D
// sercom2[2] PA14:C, PB28:C, PA10:D, PC24:D
// sercom2[3] PA15:C, PB29:C, PA11:D, PC25:D

pad!(Sercom2Pad0 {
    Pa12(PfC),
    Pb26(PfC),
    Pa9(PfD),
    Pb25(PfD),
});

pad!(Sercom2Pad1 {
    Pa13(PfC),
    Pb27(PfC),
    Pa8(PfD),
    Pb24(PfD),
});

pad!(Sercom2Pad2 {
    Pa14(PfC),
    Pb28(PfC),
    Pa10(PfD),
    Pc24(PfD),
});

pad!(Sercom2Pad3 {
    Pa15(PfC),
    Pb29(PfC),
    Pa11(PfD),
    Pc25(PfD),
});

// sercom3[0] PA22:C, PB20:C, PA17:D, PC23:D
// sercom3[1] PA23:C, PB21:C, PA16:D, PC22:D
// sercom3[2] PA24:C, PA20:C, PA18:D, PD20:D
// sercom3[3] PA25:C, PA21:C, PA19:D, PD21:D

pad!(Sercom3Pad0 {
    Pa22(PfC),
    Pb20(PfC),
    Pa17(PfD),
    Pc23(PfD),
});

pad!(Sercom3Pad1 {
    Pa23(PfC),
    Pb21(PfC),
    Pa16(PfD),
    Pc22(PfD),
});

pad!(Sercom3Pad2 {
    Pa24(PfC),
    Pa20(PfC),
    Pa18(PfD),
    Pd20(PfD),
});

pad!(Sercom3Pad3 {
    Pa25(PfC),
    Pa21(PfC),
    Pa19(PfD),
    Pd21(PfD),
});

// sercom4[0] PB12:C, PB08:D, PA13:D, PB27:D
// sercom4[1] PB13:C, PB09:D, PA12:D, PB26:D
// sercom4[2] PB14:C, PB10:D, PA14:D, PB28:D
// sercom4[3] PB15:c, PB11:D, PA15:D, PB29:D

pad!(Sercom4Pad0 {
    Pb12(PfC),
    Pb8(PfD),
    Pa13(PfD),
    Pb27(PfD),
});

pad!(Sercom4Pad1 {
    Pb13(PfC),
    Pb9(PfD),
    Pa12(PfD),
    Pb26(PfD),
});

pad!(Sercom4Pad2 {
    Pb14(PfC),
    Pb10(PfD),
    Pa14(PfD),
    Pb28(PfD),
});

pad!(Sercom4Pad3 {
    Pb15(PfC),
    Pb11(PfD),
    Pa15(PfD),
    Pb29(PfD),
});

// sercom5[0] PB16:C, PA23:D, PA23:D, PA23:D, PB31:D, PB02:D
// sercom5[1] PB17:C, PA22:D, PA22:D, PA22:D, PB30:D, PB03:D
// sercom5[2] PB18:C, PA20:D, PA24:D, PB22:D, PB00:D, PB00:D
// sercom5[3] PB19:C, PA21:D, PA25:D, PB23:D, PB01:D, PB01:D

pad!(Sercom5Pad0 {
    Pb16(PfC),
    Pa23(PfD),
//    Pa23(PfD),
//    Pa23(PfD),
    Pb31(PfD),
    Pb2(PfD),
});

pad!(Sercom5Pad1 {
    Pb17(PfC),
//    Pa22(PfD),
//    Pa22(PfD),
    Pa22(PfD),
    Pb30(PfD),
    Pb3(PfD),
});

pad!(Sercom5Pad2 {
    Pb18(PfC),
    Pa20(PfD),
    Pa24(PfD),
    Pb22(PfD),
    Pb0(PfD),
//    Pb0(PfD),
});

pad!(Sercom5Pad3 {
    Pb19(PfC),
    Pa21(PfD),
    Pa25(PfD),
    Pb23(PfD),
    Pb1(PfD),
//    Pb1(PfD),
});

// sercom6[0] PC16:C, PC04:C, PD09:D, PC13:D, PC13:D
// sercom6[1] PC17:C, PC05:C, PD08:D, PC12:D, PC12:D
// sercom6[2] PC18:C, PC06:C, PD10:D, PC14:D, PC10:C
// sercom6[3] PC19:C, PC07:C, PD11:D, PC15:D, PC11:C

pad!(Sercom6Pad0 {
    Pc16(PfC),
    Pc4(PfC),
    Pd9(PfD),
    Pc13(PfD),
//    Pc13(PfD),
});

pad!(Sercom6Pad1 {
    Pc17(PfC),
    Pc5(PfC),
    Pd8(PfD),
    Pc12(PfD),
//    Pc12(PfD),
});

pad!(Sercom6Pad2 {
    Pc18(PfC),
    Pc6(PfC),
    Pd10(PfD),
    Pc14(PfD),
    Pc10(PfC),
});

pad!(Sercom6Pad3 {
    Pc19(PfC),
    Pc7(PfC),
    Pd11(PfD),
    Pc15(PfD),
    Pc11(PfC),
});

// sercom7[0] PC12:C, PD08:C, PC12:C, PB21:D, PB30:D
// sercom7[1] PC13:C, PD09:C, PC13:C, PB20:D, PB31:D
// sercom7[2] PC14:C, PD10:C, PC10:D, PB18:D, PA30:D
// sercom7[3] PC15:C, PD11:C, PC11:D, PB19:D, PA31:D

pad!(Sercom7Pad0 {
    Pc12(PfC),
    Pd8(PfC),
//    Pc12(PfC),
    Pb21(PfD),
    Pb30(PfD),
});

pad!(Sercom7Pad1 {
    Pc13(PfC),
    Pd9(PfC),
//    Pc13(PfC),
    Pb20(PfD),
    Pb31(PfD),
});

pad!(Sercom7Pad2 {
    Pc14(PfC),
    Pd10(PfC),
    Pc10(PfD),
    Pb18(PfD),
    Pa30(PfD),
});

pad!(Sercom7Pad3 {
    Pc15(PfC),
    Pd11(PfC),
    Pc11(PfD),
    Pb19(PfD),
    Pa31(PfD),
});
