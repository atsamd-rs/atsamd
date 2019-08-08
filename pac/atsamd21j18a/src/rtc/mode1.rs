#[doc = "MODE1 Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Control"]
pub mod ctrl;
#[doc = "Read Request"]
pub struct READREQ {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Read Request"]
pub mod readreq;
#[doc = "MODE1 Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Event Control"]
pub mod evctrl;
#[doc = "MODE1 Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MODE1 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "MODE1 Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MODE1 Interrupt Enable Set"]
pub mod intenset;
#[doc = "MODE1 Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MODE1 Interrupt Flag Status and Clear"]
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
#[doc = "MODE1 Counter Value"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Counter Value"]
pub mod count;
#[doc = "MODE1 Counter Period"]
pub struct PER {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Counter Period"]
pub mod per;
#[doc = "MODE1 Compare n Value"]
pub struct COMP {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Compare n Value"]
pub mod comp;
