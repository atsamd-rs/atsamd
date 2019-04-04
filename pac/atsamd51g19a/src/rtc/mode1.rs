#[doc = "MODE1 Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Control A"]
pub mod ctrla;
#[doc = "MODE1 Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Control B"]
pub mod ctrlb;
#[doc = "MODE1 Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE1 Event Control"]
pub mod evctrl;
#[doc = "MODE1 Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "MODE1 Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Interrupt Enable Set"]
pub mod intenset;
#[doc = "MODE1 Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MODE1 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "MODE1 Synchronization Busy Status"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE1 Synchronization Busy Status"]
pub mod syncbusy;
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
#[doc = "General Purpose"]
pub struct GP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose"]
pub mod gp;
#[doc = "Tamper Control"]
pub struct TAMPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "MODE1 Timestamp"]
pub struct TIMESTAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MODE1 Timestamp"]
pub mod timestamp;
#[doc = "Tamper ID"]
pub struct TAMPID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "Backup"]
pub struct BKUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup"]
pub mod bkup;
