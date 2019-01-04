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
#[doc = "HOST Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Control B"]
pub mod ctrlb;
#[doc = "HOST Host Start Of Frame Control"]
pub struct HSOFC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Host Start Of Frame Control"]
pub mod hsofc;
#[doc = "HOST Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Status"]
pub mod status;
#[doc = "Finite State Machine Status"]
pub struct FSMSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "HOST Host Frame Number"]
pub struct FNUM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Frame Number"]
pub mod fnum;
#[doc = "HOST Host Frame Length"]
pub struct FLENHIGH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Host Frame Length"]
pub mod flenhigh;
#[doc = "HOST Host Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "HOST Host Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Interrupt Enable Set"]
pub mod intenset;
#[doc = "HOST Host Interrupt Flag"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Interrupt Flag"]
pub mod intflag;
#[doc = "HOST Pipe Interrupt Summary"]
pub struct PINTSMRY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Pipe Interrupt Summary"]
pub mod pintsmry;
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
#[doc = "HOST End Point Configuration"]
pub struct PCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST End Point Configuration"]
pub mod pcfg;
#[doc = "HOST Bus Access Period of Pipe"]
pub struct BINTERVAL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "HOST End Point Pipe Status Clear"]
pub struct PSTATUSCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "HOST End Point Pipe Status Set"]
pub struct PSTATUSSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "HOST End Point Pipe Status"]
pub struct PSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST End Point Pipe Status"]
pub mod pstatus;
#[doc = "HOST Pipe Interrupt Flag"]
pub struct PINTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "HOST Pipe Interrupt Flag Clear"]
pub struct PINTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "HOST Pipe Interrupt Flag Set"]
pub struct PINTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Pipe Interrupt Flag Set"]
pub mod pintenset;
