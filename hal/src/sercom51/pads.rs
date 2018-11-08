use gpio::{self, IntoFunction, Port};

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
    ($(pub enum $PadType:ident {
        $( $PinType:ident ($new:ident, $Pf:ident),)+
    })+
    ) => {
$(
/// Represents a numbered pad for the associated sercom instance
pub enum $PadType {
    $(
        $PinType(gpio::$PinType<gpio::$Pf>),
    )+
}

impl $PadType {
    $(
    /// Construct pad from the appropriate pin in any mode.
    /// You may find it more convenient to use the `into_pad` trait
    /// and avoid referencing the pad type.
    pub fn $new<MODE>(pin: gpio::$PinType<MODE>, port: &mut Port) -> Self {
        $PadType::$PinType(pin.into_function(port))
    }

    )+
}

$(
impl<MODE> PadPin<$PadType> for gpio::$PinType<MODE> {
    fn into_pad(self, port: &mut Port) -> $PadType {
        $PadType::$new(self, port)
    }
}
)+

)+
    };
}

pad!(
// sercom0[0]:  PA04:D   PA08:C
// sercom0[1]:  PA05:D   PA09:C
// sercom0[2]:  PA06:D   PA10:C
// sercom0[3]:  PA07:D   PA11:C

pub enum Sercom0Pad0 {
    Pa4(pa4, PfD),
    Pa8(pa8, PfC),
}

pub enum Sercom0Pad1 {
    Pa5(pa5, PfD),
    Pa9(pa9, PfC),
}

pub enum Sercom0Pad2 {
    Pa6(pa6, PfD),
    Pa10(pa10, PfC),
}

pub enum Sercom0Pad3 {
    Pa7(pa7, PfD),
    Pa11(pa11, PfC),
}

// sercom1[0]:  PA16:C   PA00:D
// sercom1[1]:  PA17:C   PA01:D
// sercom1[2]:  PA18:C   PA30:D
// sercom1[3]:  PA19:C   PA31:D

pub enum Sercom1Pad0 {
    Pa0(pa0, PfD),
    Pa16(pa16, PfC),
}

pub enum Sercom1Pad1 {
    Pa1(pa1, PfD),
    Pa17(pa17, PfC),
}

pub enum Sercom1Pad2 {
    Pa18(pa18, PfC),
    Pa30(pa30, PfD),
}

pub enum Sercom1Pad3 {
    Pa19(pa19, PfC),
    Pa31(pa31, PfD),
}

// sercom2[0]:  PA12:C   PA08:D
// sercom2[1]:  PA13:C   PA09:D
// sercom2[2]:  PA14:C   PA10:D
// sercom2[3]:  PA15:C   PA11:D

pub enum Sercom2Pad0 {
    Pa8(pa8, PfD),
    Pa12(pa12, PfC),
}

pub enum Sercom2Pad1 {
    Pa9(pa9, PfD),
    Pa13(pa13, PfC),
}

pub enum Sercom2Pad2 {
    Pa10(pa10, PfD),
    Pa14(pa14, PfC),
}

pub enum Sercom2Pad3 {
    Pa11(pa11, PfD),
    Pa15(pa15, PfC),
}

// sercom3[0]:  PA16:D   PA22:C
// sercom3[1]:  PA17:D   PA23:C
// sercom3[2]:  PA18:D   PA24:C   PA20:D
// sercom3[3]:  PA19:D   PA25:C   PA21:D

pub enum Sercom3Pad0 {
    Pa16(pa16, PfD),
    Pa22(pa22, PfC),
}
pub enum Sercom3Pad1 {
    Pa17(pa17, PfD),
    Pa23(pa23, PfC),
}
pub enum Sercom3Pad2 {
    Pa18(pa18, PfD),
    Pa20(pa20, PfD),
    Pa24(pa24, PfC),
}
pub enum Sercom3Pad3 {
    Pa19(pa19, PfD),
    Pa21(pa21, PfD),
    Pa25(pa25, PfC),
}
);

#[cfg(feature = "samd21g18a")]
pad!(
// sercom4[0]:  PA12:D   PB08:D   PB12:C
// sercom4[1]:  PA13:D   PB09:D   PB13:C
// sercom4[2]:  PA14:D   PB10:D   PB14:C
// sercom4[3]:  PA15:D   PB11:D   PB15:C

pub enum Sercom4Pad0 {
    Pa12(pa12, PfD),
    Pb8(pb8, PfD),
    Pb12(pb12, PfC),
}

pub enum Sercom4Pad1 {
    Pa13(pa13, PfD),
    Pb9(pb9, PfD),
    Pb13(pb13, PfC),
}

pub enum Sercom4Pad2 {
    Pa14(pa14, PfD),
    Pb10(pb10, PfD),
    Pb14(pb14, PfC),
}

pub enum Sercom4Pad3 {
    Pa15(pa15, PfD),
    Pb11(pb11, PfD),
    Pb15(pb15, PfC),
}

// sercom5[0]:  PA22:D   PB02:D   PB16:C  PB30:D
// sercom5[1]:  PA23:D   PB03:D   PB17:C  PB31:D
// sercom5[2]:  PA24:D   PB00:D   PA20:C  PB22:D
// sercom5[3]:  PA25:D   PB01:D   PA21:C  PB23:D

pub enum Sercom5Pad0 {
    Pa22(pa22, PfD),
    Pb2(pb2, PfD),
    Pb16(pb16, PfC),
    Pb30(pb30, PfD),
}

pub enum Sercom5Pad1 {
    Pa23(pa23, PfD),
    Pb3(pb3, PfD),
    Pb17(pb17, PfC),
    Pb31(pb31, PfD),
}

pub enum Sercom5Pad2 {
    Pa24(pa24, PfD),
    Pb0(pb0, PfD),
    Pa20(pa20, PfC),
    Pb22(pb22, PfD),
}

pub enum Sercom5Pad3 {
    Pa25(pa25, PfD),
    Pb1(pb1, PfD),
    Pa21(pa21, PfC),
    Pb23(pb23, PfD),
}
);
