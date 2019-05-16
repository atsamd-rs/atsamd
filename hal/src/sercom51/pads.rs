use crate::gpio::{self, IntoFunction, Port};

/// The PadPin trait makes it more ergonomic to convert a
/// pin into a Sercom pad.  You should not implement this
/// trait for yourself; only the implementations in the
/// sercom module make sense.
pub trait PadPin<T> {
    fn into_pad(self, port: &mut Port) -> T;
}

/// The pad macro helps to define enums for pads and makes it
/// a little more convenient to initialize them.
macro_rules! pad {
    ($PadType:ident {
        $($PinType:ident ($Pf:ident),)+
    }
    ) => {
/// Represents a numbered pad for the associated sercom instance. The pad is
/// generic over any pin, only the PadPin implementations in this the sercom
/// module make sense.
pub struct $PadType<PIN>(PIN);

impl<PIN> $PadType<PIN> {
    /// Construct pad from the appropriate pin in any mode.
    /// You may find it more convenient to use the `into_pad` trait
    /// and avoid referencing the pad type.
    pub fn new(pin: PIN) -> Self {
        $PadType(pin)
    }
}

$(
    impl<MODE> PadPin<$PadType<gpio::$PinType<gpio::$Pf>>> for gpio::$PinType<MODE> {
        fn into_pad(self, port: &mut Port) -> $PadType<gpio::$PinType<gpio::$Pf>> {
            $PadType::new(self.into_function(port))
        }
    }
)+

    };
}

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
// sercom1[3]:  PA19:C   PA31:D   PB23:C

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
    Pb22(PfC),
});

pad!(Sercom1Pad3 {
    Pa19(PfC),
    Pa31(PfD),
    Pb23(PfC),
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

pad!(Sercom4Pad0 {
    Pa12(PfD),
    Pb8(PfD),
    Pb12(PfC),
});

pad!(Sercom4Pad1 {
    Pa13(PfD),
    Pb9(PfD),
    Pb13(PfC),
});

pad!(Sercom4Pad2 {
    Pa14(PfD),
    Pb10(PfD),
    Pb14(PfC),
});

pad!(Sercom4Pad3 {
    Pa15(PfD),
    Pb11(PfD),
    Pb15(PfC),
});

// sercom5[0]:  PA23:D   PB02:D   PB31:D   PB16:C
// sercom5[1]:  PA22:D   PB03:D   PB30:D   PB17:C
// sercom5[2]:  PA24:D   PB00:D   PA20:C   PB22:D
// sercom5[3]:  PA25:D   PB01:D   PA21:C   PB23:D

pad!(Sercom5Pad0 {
    Pa23(PfD),
    Pb2(PfD),
    Pb31(PfD),
    Pb16(PfC),
});

pad!(Sercom5Pad1 {
    Pa22(PfD),
    Pb3(PfD),
    Pb30(PfD),
    Pb17(PfC),
});

pad!(Sercom5Pad2 {
    Pa24(PfD),
    Pb0(PfD),
    Pa20(PfC),
    Pb22(PfD),
});

pad!(Sercom5Pad3 {
    Pa25(PfD),
    Pb1(PfD),
    Pa21(PfC),
    Pb23(PfD),
});
