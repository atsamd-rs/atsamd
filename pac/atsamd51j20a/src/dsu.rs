#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status A"]
    pub statusa: STATUSA,
    #[doc = "0x02 - Status B"]
    pub statusb: STATUSB,
    _reserved3: [u8; 1usize],
    #[doc = "0x04 - Address"]
    pub addr: ADDR,
    #[doc = "0x08 - Length"]
    pub length: LENGTH,
    #[doc = "0x0c - Data"]
    pub data: DATA,
    #[doc = "0x10 - Debug Communication Channel n"]
    pub dcc: [DCC; 2],
    #[doc = "0x18 - Device Identification"]
    pub did: DID,
    #[doc = "0x1c - Configuration"]
    pub cfg: CFG,
    _reserved9: [u8; 208usize],
    #[doc = "0xf0 - Device Configuration"]
    pub dcfg: [DCFG; 2],
    _reserved10: [u8; 3848usize],
    #[doc = "0x1000 - CoreSight ROM Table Entry 0"]
    pub entry0: ENTRY0,
    #[doc = "0x1004 - CoreSight ROM Table Entry 1"]
    pub entry1: ENTRY1,
    #[doc = "0x1008 - CoreSight ROM Table End"]
    pub end: END,
    _reserved13: [u8; 4032usize],
    #[doc = "0x1fcc - CoreSight ROM Table Memory Type"]
    pub memtype: MEMTYPE,
    #[doc = "0x1fd0 - Peripheral Identification 4"]
    pub pid4: PID4,
    #[doc = "0x1fd4 - Peripheral Identification 5"]
    pub pid5: PID5,
    #[doc = "0x1fd8 - Peripheral Identification 6"]
    pub pid6: PID6,
    #[doc = "0x1fdc - Peripheral Identification 7"]
    pub pid7: PID7,
    #[doc = "0x1fe0 - Peripheral Identification 0"]
    pub pid0: PID0,
    #[doc = "0x1fe4 - Peripheral Identification 1"]
    pub pid1: PID1,
    #[doc = "0x1fe8 - Peripheral Identification 2"]
    pub pid2: PID2,
    #[doc = "0x1fec - Peripheral Identification 3"]
    pub pid3: PID3,
    #[doc = "0x1ff0 - Component Identification 0"]
    pub cid0: CID0,
    #[doc = "0x1ff4 - Component Identification 1"]
    pub cid1: CID1,
    #[doc = "0x1ff8 - Component Identification 2"]
    pub cid2: CID2,
    #[doc = "0x1ffc - Component Identification 3"]
    pub cid3: CID3,
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Status A"]
pub struct STATUSA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status A"]
pub mod statusa;
#[doc = "Status B"]
pub struct STATUSB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status B"]
pub mod statusb;
#[doc = "Address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address"]
pub mod addr;
#[doc = "Length"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length"]
pub mod length;
#[doc = "Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data"]
pub mod data;
#[doc = "Debug Communication Channel n"]
pub struct DCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Communication Channel n"]
pub mod dcc;
#[doc = "Device Identification"]
pub struct DID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Identification"]
pub mod did;
#[doc = "Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration"]
pub mod cfg;
#[doc = "Device Configuration"]
pub struct DCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Configuration"]
pub mod dcfg;
#[doc = "CoreSight ROM Table Entry 0"]
pub struct ENTRY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight ROM Table Entry 0"]
pub mod entry0;
#[doc = "CoreSight ROM Table Entry 1"]
pub struct ENTRY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight ROM Table Entry 1"]
pub mod entry1;
#[doc = "CoreSight ROM Table End"]
pub struct END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight ROM Table End"]
pub mod end;
#[doc = "CoreSight ROM Table Memory Type"]
pub struct MEMTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CoreSight ROM Table Memory Type"]
pub mod memtype;
#[doc = "Peripheral Identification 4"]
pub struct PID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 4"]
pub mod pid4;
#[doc = "Peripheral Identification 5"]
pub struct PID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 5"]
pub mod pid5;
#[doc = "Peripheral Identification 6"]
pub struct PID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 6"]
pub mod pid6;
#[doc = "Peripheral Identification 7"]
pub struct PID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 7"]
pub mod pid7;
#[doc = "Peripheral Identification 0"]
pub struct PID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 0"]
pub mod pid0;
#[doc = "Peripheral Identification 1"]
pub struct PID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 1"]
pub mod pid1;
#[doc = "Peripheral Identification 2"]
pub struct PID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 2"]
pub mod pid2;
#[doc = "Peripheral Identification 3"]
pub struct PID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification 3"]
pub mod pid3;
#[doc = "Component Identification 0"]
pub struct CID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Identification 0"]
pub mod cid0;
#[doc = "Component Identification 1"]
pub struct CID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Identification 1"]
pub mod cid1;
#[doc = "Component Identification 2"]
pub struct CID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Identification 2"]
pub mod cid2;
#[doc = "Component Identification 3"]
pub struct CID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Identification 3"]
pub mod cid3;
