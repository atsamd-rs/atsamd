#[doc = "SPI Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Control A"]
pub mod ctrla;
#[doc = "SPI Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Control B"]
pub mod ctrlb;
#[doc = "SPI Control C"]
pub struct CTRLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Control C"]
pub mod ctrlc;
#[doc = "SPI Baud Rate"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI Baud Rate"]
pub mod baud;
#[doc = "SPI Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "SPI Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI Interrupt Enable Set"]
pub mod intenset;
#[doc = "SPI Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "SPI Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPI Status"]
pub mod status;
#[doc = "SPI Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Synchronization Busy"]
pub mod syncbusy;
#[doc = "SPI Length"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPI Length"]
pub mod length;
#[doc = "SPI Address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Address"]
pub mod addr;
#[doc = "SPI Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Data"]
pub mod data;
#[doc = "SPI Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI Debug Control"]
pub mod dbgctrl;
