#[doc = "MODE2 Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE2 Control"]
pub mod ctrl;
#[doc = "Read Request"]
pub struct READREQ {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Read Request"]
pub mod readreq;
#[doc = "MODE2 Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE2 Event Control"]
pub mod evctrl;
#[doc = "MODE2 Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MODE2 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "MODE2 Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MODE2 Interrupt Enable Set"]
pub mod intenset;
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Frequency Correction"]
pub struct FREQCORR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "MODE2 Clock Value"]
pub struct CLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "MODE2 Alarm n Value"]
pub struct ALARM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE2 Alarm n Value"]
pub mod alarm;
#[doc = "MODE2 Alarm n Mask"]
pub struct MASK {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MODE2 Alarm n Mask"]
pub mod mask;
