use crate::gpio::{self, IntoFunction, Port};
pub use crate::pad::PadPin;

// sercom0[0]:  PA04:D   PA06:C   PA14:C
// sercom0[1]:  PA05:D   PA07:C   PA15:C
// sercom0[2]:  PA04:C   PA06:D   PA08:D   PA10:C
// sercom0[3]:  PA05:C   PA07:D   PA09:D   PA11:C

pad!(Sercom0Pad0 {
    Pa4(PfD),
    Pa6(PfC),
    Pa14(PfC),
});

pad!(Sercom0Pad1 {
    Pa5(PfD),
    Pa7(PfC),
    Pa15(PfC),
});

pad!(Sercom0Pad2 {
    Pa4(PfC),
    Pa6(PfD),
    Pa8(PfD),
    Pa10(PfC),
});

pad!(Sercom0Pad3 {
    Pa5(PfC),
    Pa7(PfD),
    Pa9(PfD),
    Pa11(PfC),
});

// sercom1[0]:  PA22:C   PA30:C
// sercom1[1]:  PA23:C   PA31:C
// sercom1[2]:  PA08:C   PA16:C   PA30:D   PA24:C
// sercom1[3]:  PA09:C   PA17:C   PA31:D   PA25:C

pad!(Sercom1Pad0 {
    Pa22(PfC),
    Pa30(PfC),
});

pad!(Sercom1Pad1 {
    Pa23(PfC),
    Pa31(PfC),
});

pad!(Sercom1Pad2 {
    Pa8(PfC),
    Pa16(PfC),
    Pa30(PfD),
    Pa24(PfC),
});

pad!(Sercom1Pad3 {
    Pa9(PfC),
    Pa17(PfC),
    Pa31(PfD),
    Pa25(PfC),
});
