// Note: section 7.2.3 shows which pins support I2C Hs mode
use gpio;

// sercom0[0]:  PA04:D   PA08:C
// sercom0[1]:  PA05:D   PA09:C
// sercom0[2]:  PA06:D   PA10:C
// sercom0[3]:  PA07:D   PA11:C

pub enum Sercom0Pad0 {
    Pa4(gpio::Pa4<gpio::PfD>),
    Pa8(gpio::Pa8<gpio::PfC>),
}

pub enum Sercom0Pad1 {
    Pa5(gpio::Pa5<gpio::PfD>),
    Pa9(gpio::Pa9<gpio::PfC>),
}

pub enum Sercom0Pad2 {
    Pa6(gpio::Pa6<gpio::PfD>),
    Pa10(gpio::Pa10<gpio::PfC>),
}

pub enum Sercom0Pad3 {
    Pa7(gpio::Pa7<gpio::PfD>),
    Pa11(gpio::Pa11<gpio::PfC>),
}

// sercom1[0]:  PA16:C   PA00:D
// sercom1[1]:  PA17:C   PA01:D
// sercom1[2]:  PA18:C   PA30:D
// sercom1[3]:  PA19:C   PA31:D

pub enum Sercom1Pad0 {
    Pa0(gpio::Pa0<gpio::PfD>),
    Pa16(gpio::Pa16<gpio::PfC>),
}

pub enum Sercom1Pad1 {
    Pa1(gpio::Pa1<gpio::PfD>),
    Pa17(gpio::Pa17<gpio::PfC>),
}

pub enum Sercom1Pad2 {
    Pa18(gpio::Pa18<gpio::PfC>),
    Pa30(gpio::Pa30<gpio::PfD>),
}

pub enum Sercom1Pad3 {
    Pa19(gpio::Pa19<gpio::PfC>),
    Pa31(gpio::Pa31<gpio::PfD>),
}

// sercom2[0]:  PA12:C   PA08:D
// sercom2[1]:  PA13:C   PA09:D
// sercom2[2]:  PA14:C   PA10:D
// sercom2[3]:  PA15:C   PA11:D

pub enum Sercom2Pad0 {
    Pa8(gpio::Pa8<gpio::PfD>),
    Pa12(gpio::Pa12<gpio::PfC>),
}

pub enum Sercom2Pad1 {
    Pa9(gpio::Pa9<gpio::PfD>),
    Pa13(gpio::Pa13<gpio::PfC>),
}

pub enum Sercom2Pad2 {
    Pa10(gpio::Pa10<gpio::PfD>),
    Pa14(gpio::Pa14<gpio::PfC>),
}

pub enum Sercom2Pad3 {
    Pa11(gpio::Pa11<gpio::PfD>),
    Pa15(gpio::Pa15<gpio::PfC>),
}

// sercom3[0]:  PA16:D   PA22:C
// sercom3[1]:  PA17:D   PA23:C
// sercom3[2]:  PA18:D   PA24:C   PA20:D
// sercom3[3]:  PA19:D   PA25:C   PA21:D

pub enum Sercom3Pad0 {
    Pa16(gpio::Pa16<gpio::PfD>),
    Pa22(gpio::Pa22<gpio::PfC>),
}

pub enum Sercom3Pad1 {
    Pa17(gpio::Pa17<gpio::PfD>),
    Pa23(gpio::Pa23<gpio::PfC>),
}

pub enum Sercom3Pad2 {
    Pa18(gpio::Pa18<gpio::PfD>),
    Pa20(gpio::Pa20<gpio::PfD>),
    Pa24(gpio::Pa24<gpio::PfC>),
}

pub enum Sercom3Pad3 {
    Pa19(gpio::Pa19<gpio::PfD>),
    Pa21(gpio::Pa21<gpio::PfD>),
    Pa25(gpio::Pa25<gpio::PfC>),
}

// sercom4[0]:  PA12:D   PB08:D   PB12:C
// sercom4[1]:  PA13:D   PB09:D   PB13:C
// sercom4[2]:  PA14:D   PB10:D   PB14:C
// sercom4[3]:  PA15:D   PB11:D   PB15:C

pub enum Sercom4Pad0 {
    Pa12(gpio::Pa12<gpio::PfD>),
    Pb8(gpio::Pb8<gpio::PfD>),
    Pb12(gpio::Pb12<gpio::PfC>),
}

pub enum Sercom4Pad1 {
    Pa13(gpio::Pa13<gpio::PfD>),
    Pb9(gpio::Pb9<gpio::PfD>),
    Pb13(gpio::Pb13<gpio::PfC>),
}

pub enum Sercom4Pad2 {
    Pa14(gpio::Pa14<gpio::PfD>),
    Pb10(gpio::Pb10<gpio::PfD>),
    Pb14(gpio::Pb14<gpio::PfC>),
}

pub enum Sercom4Pad3 {
    Pa15(gpio::Pa15<gpio::PfD>),
    Pb11(gpio::Pb11<gpio::PfD>),
    Pb15(gpio::Pb15<gpio::PfC>),
}

// sercom5[0]:  PA22:D   PB02:D   PB16:C  PB30:D
// sercom5[1]:  PA23:D   PB03:D   PB17:C  PB31:D
// sercom5[2]:  PA24:D   PB00:D   PA20:C  PB22:D
// sercom5[3]:  PA25:D   PB01:D   PA21:C  PB23:D

pub enum Sercom5Pad0 {
    Pa22(gpio::Pa22<gpio::PfD>),
    Pb2(gpio::Pb2<gpio::PfD>),
    Pb16(gpio::Pb16<gpio::PfC>),
    Pb30(gpio::Pb30<gpio::PfD>),
}

pub enum Sercom5Pad1 {
    Pa23(gpio::Pa23<gpio::PfD>),
    Pb3(gpio::Pb3<gpio::PfD>),
    Pb17(gpio::Pb17<gpio::PfC>),
    Pb31(gpio::Pb31<gpio::PfD>),
}

pub enum Sercom5Pad2 {
    Pa24(gpio::Pa24<gpio::PfD>),
    Pb0(gpio::Pb0<gpio::PfD>),
    Pa20(gpio::Pa20<gpio::PfC>),
    Pb22(gpio::Pb22<gpio::PfD>),
}

pub enum Sercom5Pad3 {
    Pa25(gpio::Pa25<gpio::PfD>),
    Pb1(gpio::Pb1<gpio::PfD>),
    Pa21(gpio::Pa21<gpio::PfC>),
    Pb23(gpio::Pb23<gpio::PfD>),
}
