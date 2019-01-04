#[doc = "I2CS Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2CS Control A"]
pub mod ctrla;
#[doc = "I2CS Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2CS Control B"]
pub mod ctrlb;
#[doc = "I2CS Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2CS Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "I2CS Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2CS Interrupt Enable Set"]
pub mod intenset;
#[doc = "I2CS Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2CS Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "I2CS Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2CS Status"]
pub mod status;
#[doc = "I2CS Syncbusy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2CS Syncbusy"]
pub mod syncbusy;
#[doc = "I2CS Address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2CS Address"]
pub mod addr;
#[doc = "I2CS Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2CS Data"]
pub mod data;
