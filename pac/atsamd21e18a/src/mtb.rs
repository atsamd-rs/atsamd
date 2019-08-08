#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Position"]
    pub position: POSITION,
    #[doc = "0x04 - MTB Master"]
    pub master: MASTER,
    #[doc = "0x08 - MTB Flow"]
    pub flow: FLOW,
    #[doc = "0x0c - MTB Base"]
    pub base: BASE,
    _reserved4: [u8; 3824usize],
    #[doc = "0xf00 - MTB Integration Mode Control"]
    pub itctrl: ITCTRL,
    _reserved5: [u8; 156usize],
    #[doc = "0xfa0 - MTB Claim Set"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - MTB Claim Clear"]
    pub claimclr: CLAIMCLR,
    _reserved7: [u8; 8usize],
    #[doc = "0xfb0 - MTB Lock Access"]
    pub lockaccess: LOCKACCESS,
    #[doc = "0xfb4 - MTB Lock Status"]
    pub lockstatus: LOCKSTATUS,
    #[doc = "0xfb8 - MTB Authentication Status"]
    pub authstatus: AUTHSTATUS,
    #[doc = "0xfbc - MTB Device Architecture"]
    pub devarch: DEVARCH,
    _reserved11: [u8; 8usize],
    #[doc = "0xfc8 - MTB Device Configuration"]
    pub devid: DEVID,
    #[doc = "0xfcc - MTB Device Type"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - CoreSight"]
    pub pid4: PID4,
    #[doc = "0xfd4 - CoreSight"]
    pub pid5: PID5,
    #[doc = "0xfd8 - CoreSight"]
    pub pid6: PID6,
    #[doc = "0xfdc - CoreSight"]
    pub pid7: PID7,
    #[doc = "0xfe0 - CoreSight"]
    pub pid0: PID0,
    #[doc = "0xfe4 - CoreSight"]
    pub pid1: PID1,
    #[doc = "0xfe8 - CoreSight"]
    pub pid2: PID2,
    #[doc = "0xfec - CoreSight"]
    pub pid3: PID3,
    #[doc = "0xff0 - CoreSight"]
    pub cid0: CID0,
    #[doc = "0xff4 - CoreSight"]
    pub cid1: CID1,
    #[doc = "0xff8 - CoreSight"]
    pub cid2: CID2,
    #[doc = "0xffc - CoreSight"]
    pub cid3: CID3,
}
#[doc = "MTB Position"]
pub struct POSITION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Position"]
pub mod position;
#[doc = "MTB Master"]
pub struct MASTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Master"]
pub mod master;
#[doc = "MTB Flow"]
pub struct FLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Flow"]
pub mod flow;
#[doc = "MTB Base"]
pub struct BASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Base"]
pub mod base;
#[doc = "MTB Integration Mode Control"]
pub struct ITCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Integration Mode Control"]
pub mod itctrl;
#[doc = "MTB Claim Set"]
pub struct CLAIMSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Claim Set"]
pub mod claimset;
#[doc = "MTB Claim Clear"]
pub struct CLAIMCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Claim Clear"]
pub mod claimclr;
#[doc = "MTB Lock Access"]
pub struct LOCKACCESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Lock Access"]
pub mod lockaccess;
#[doc = "MTB Lock Status"]
pub struct LOCKSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Lock Status"]
pub mod lockstatus;
#[doc = "MTB Authentication Status"]
pub struct AUTHSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Authentication Status"]
pub mod authstatus;
#[doc = "MTB Device Architecture"]
pub struct DEVARCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Device Architecture"]
pub mod devarch;
#[doc = "MTB Device Configuration"]
pub struct DEVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Device Configuration"]
pub mod devid;
#[doc = "MTB Device Type"]
pub struct DEVTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Device Type"]
pub mod devtype;
#[doc = "CoreSight"]
pub struct PID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid4;
#[doc = "CoreSight"]
pub struct PID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid5;
#[doc = "CoreSight"]
pub struct PID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid6;
#[doc = "CoreSight"]
pub struct PID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid7;
#[doc = "CoreSight"]
pub struct PID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid0;
#[doc = "CoreSight"]
pub struct PID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid1;
#[doc = "CoreSight"]
pub struct PID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid2;
#[doc = "CoreSight"]
pub struct PID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid3;
#[doc = "CoreSight"]
pub struct CID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid0;
#[doc = "CoreSight"]
pub struct CID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid1;
#[doc = "CoreSight"]
pub struct CID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid2;
#[doc = "CoreSight"]
pub struct CID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid3;
