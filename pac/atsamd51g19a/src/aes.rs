#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x05 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x06 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x07 - Interrupt Flag Status"]
    pub intflag: INTFLAG,
    #[doc = "0x08 - Data buffer pointer"]
    pub databufptr: DATABUFPTR,
    #[doc = "0x09 - Debug control"]
    pub dbgctrl: DBGCTRL,
    _reserved7: [u8; 2usize],
    #[doc = "0x0c - Keyword n"]
    pub keyword: [KEYWORD; 8],
    _reserved8: [u8; 12usize],
    #[doc = "0x38 - Indata"]
    pub indata: INDATA,
    #[doc = "0x3c - Initialisation Vector n"]
    pub intvectv: [INTVECTV; 4],
    _reserved10: [u8; 16usize],
    #[doc = "0x5c - Hash key n"]
    pub hashkey: [HASHKEY; 4],
    #[doc = "0x6c - Galois Hash n"]
    pub ghash: [GHASH; 4],
    _reserved12: [u8; 4usize],
    #[doc = "0x80 - Cipher Length"]
    pub ciplen: CIPLEN,
    #[doc = "0x84 - Random Seed"]
    pub randseed: RANDSEED,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B"]
pub mod ctrlb;
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
#[doc = "Interrupt Flag Status"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag Status"]
pub mod intflag;
#[doc = "Data buffer pointer"]
pub struct DATABUFPTR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Data buffer pointer"]
pub mod databufptr;
#[doc = "Debug control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug control"]
pub mod dbgctrl;
#[doc = "Keyword n"]
pub struct KEYWORD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Keyword n"]
pub mod keyword;
#[doc = "Indata"]
pub struct INDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indata"]
pub mod indata;
#[doc = "Initialisation Vector n"]
pub struct INTVECTV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initialisation Vector n"]
pub mod intvectv;
#[doc = "Hash key n"]
pub struct HASHKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash key n"]
pub mod hashkey;
#[doc = "Galois Hash n"]
pub struct GHASH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Galois Hash n"]
pub mod ghash;
#[doc = "Cipher Length"]
pub struct CIPLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cipher Length"]
pub mod ciplen;
#[doc = "Random Seed"]
pub struct RANDSEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Random Seed"]
pub mod randseed;
