#[doc = "USART Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Control A"]
pub mod ctrla;
#[doc = "USART Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Control B"]
pub mod ctrlb;
#[doc = "USART Baud Rate"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART Baud Rate"]
pub mod baud;
#[doc = "USART Baud Rate"]
pub struct BAUD_FRAC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART Baud Rate"]
pub mod baud_frac_mode;
#[doc = "USART Baud Rate"]
pub struct BAUD_FRACFP_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART Baud Rate"]
pub mod baud_fracfp_mode;
#[doc = "USART Baud Rate"]
pub struct BAUD_USARTFP_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART Baud Rate"]
pub mod baud_usartfp_mode;
#[doc = "USART Receive Pulse Length"]
pub struct RXPL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART Receive Pulse Length"]
pub mod rxpl;
#[doc = "USART Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "USART Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART Interrupt Enable Set"]
pub mod intenset;
#[doc = "USART Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "USART Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART Status"]
pub mod status;
#[doc = "USART Syncbusy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Syncbusy"]
pub mod syncbusy;
#[doc = "USART Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART Data"]
pub mod data;
#[doc = "USART Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART Debug Control"]
pub mod dbgctrl;
