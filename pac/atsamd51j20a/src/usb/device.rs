#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "USB Quality Of Service"]
pub struct QOSCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Quality Of Service"]
pub mod qosctrl;
#[doc = "DEVICE Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Control B"]
pub mod ctrlb;
#[doc = "DEVICE Device Address"]
pub struct DADD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE Device Address"]
pub mod dadd;
#[doc = "DEVICE Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE Status"]
pub mod status;
#[doc = "Finite State Machine Status"]
pub struct FSMSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "DEVICE Device Frame Number"]
pub struct FNUM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Frame Number"]
pub mod fnum;
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "DEVICE Device Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Interrupt Enable Set"]
pub mod intenset;
#[doc = "DEVICE Device Interrupt Flag"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Interrupt Flag"]
pub mod intflag;
#[doc = "DEVICE End Point Interrupt Summary"]
pub struct EPINTSMRY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE End Point Interrupt Summary"]
pub mod epintsmry;
#[doc = "Descriptor Address"]
pub struct DESCADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "USB PAD Calibration"]
pub struct PADCAL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = "DEVICE End Point Configuration"]
pub struct EPCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE End Point Configuration"]
pub mod epcfg;
#[doc = "DEVICE End Point Pipe Status Clear"]
pub struct EPSTATUSCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE End Point Pipe Status Clear"]
pub mod epstatusclr;
#[doc = "DEVICE End Point Pipe Status Set"]
pub struct EPSTATUSSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE End Point Pipe Status Set"]
pub mod epstatusset;
#[doc = "DEVICE End Point Pipe Status"]
pub struct EPSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE End Point Pipe Status"]
pub mod epstatus;
#[doc = "DEVICE End Point Interrupt Flag"]
pub struct EPINTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE End Point Interrupt Flag"]
pub mod epintflag;
#[doc = "DEVICE End Point Interrupt Clear Flag"]
pub struct EPINTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE End Point Interrupt Clear Flag"]
pub mod epintenclr;
#[doc = "DEVICE End Point Interrupt Set Flag"]
pub struct EPINTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE End Point Interrupt Set Flag"]
pub mod epintenset;
