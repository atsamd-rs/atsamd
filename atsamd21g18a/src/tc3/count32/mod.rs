#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Read Request"]
pub struct READREQ {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Read Request"]
pub mod readreq;
#[doc = "Control B Clear"]
pub struct CTRLBCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "Control B Set"]
pub struct CTRLBSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "Control C"]
pub struct CTRLC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "COUNT32 Counter Value"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "COUNT32 Counter Value"]
pub mod count;
#[doc = "COUNT32 Compare/Capture"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "COUNT32 Compare/Capture"]
pub mod cc;
