#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x02 - CRC Control"]
    pub crcctrl: CRCCTRL,
    #[doc = "0x04 - CRC Data Input"]
    pub crcdatain: CRCDATAIN,
    #[doc = "0x08 - CRC Checksum"]
    pub crcchksum: CRCCHKSUM,
    #[doc = "0x0c - CRC Status"]
    pub crcstatus: CRCSTATUS,
    #[doc = "0x0d - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved6: [u8; 2usize],
    #[doc = "0x10 - Software Trigger Control"]
    pub swtrigctrl: SWTRIGCTRL,
    #[doc = "0x14 - Priority Control 0"]
    pub prictrl0: PRICTRL0,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - Interrupt Pending"]
    pub intpend: INTPEND,
    _reserved9: [u8; 2usize],
    #[doc = "0x24 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x28 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x2c - Pending Channels"]
    pub pendch: PENDCH,
    #[doc = "0x30 - Active Channel and Levels"]
    pub active: ACTIVE,
    #[doc = "0x34 - Descriptor Memory Section Base Address"]
    pub baseaddr: BASEADDR,
    #[doc = "0x38 - Write-Back Memory Section Base Address"]
    pub wrbaddr: WRBADDR,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - CHANNEL\\[%s\\]"]
    pub channel: [CHANNEL; 32],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control A"]
    pub chctrla: self::channel::CHCTRLA,
    #[doc = "0x04 - Channel n Control B"]
    pub chctrlb: self::channel::CHCTRLB,
    #[doc = "0x05 - Channel n Priority Level"]
    pub chprilvl: self::channel::CHPRILVL,
    #[doc = "0x06 - Channel n Event Control"]
    pub chevctrl: self::channel::CHEVCTRL,
    _reserved4: [u8; 5usize],
    #[doc = "0x0c - Channel n Interrupt Enable Clear"]
    pub chintenclr: self::channel::CHINTENCLR,
    #[doc = "0x0d - Channel n Interrupt Enable Set"]
    pub chintenset: self::channel::CHINTENSET,
    #[doc = "0x0e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: self::channel::CHINTFLAG,
    #[doc = "0x0f - Channel n Status"]
    pub chstatus: self::channel::CHSTATUS,
}
#[doc = r"Register block"]
#[doc = "CHANNEL\\[%s\\]"]
pub mod channel;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "CRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcctrl](crcctrl) module"]
pub type CRCCTRL = crate::Reg<u16, _CRCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCCTRL;
#[doc = "`read()` method returns [crcctrl::R](crcctrl::R) reader structure"]
impl crate::Readable for CRCCTRL {}
#[doc = "`write(|w| ..)` method takes [crcctrl::W](crcctrl::W) writer structure"]
impl crate::Writable for CRCCTRL {}
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdatain](crcdatain) module"]
pub type CRCDATAIN = crate::Reg<u32, _CRCDATAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCDATAIN;
#[doc = "`read()` method returns [crcdatain::R](crcdatain::R) reader structure"]
impl crate::Readable for CRCDATAIN {}
#[doc = "`write(|w| ..)` method takes [crcdatain::W](crcdatain::W) writer structure"]
impl crate::Writable for CRCDATAIN {}
#[doc = "CRC Data Input"]
pub mod crcdatain;
#[doc = "CRC Checksum\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcchksum](crcchksum) module"]
pub type CRCCHKSUM = crate::Reg<u32, _CRCCHKSUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCCHKSUM;
#[doc = "`read()` method returns [crcchksum::R](crcchksum::R) reader structure"]
impl crate::Readable for CRCCHKSUM {}
#[doc = "`write(|w| ..)` method takes [crcchksum::W](crcchksum::W) writer structure"]
impl crate::Writable for CRCCHKSUM {}
#[doc = "CRC Checksum"]
pub mod crcchksum;
#[doc = "CRC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcstatus](crcstatus) module"]
pub type CRCSTATUS = crate::Reg<u8, _CRCSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCSTATUS;
#[doc = "`read()` method returns [crcstatus::R](crcstatus::R) reader structure"]
impl crate::Readable for CRCSTATUS {}
#[doc = "`write(|w| ..)` method takes [crcstatus::W](crcstatus::W) writer structure"]
impl crate::Writable for CRCSTATUS {}
#[doc = "CRC Status"]
pub mod crcstatus;
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Software Trigger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigctrl](swtrigctrl) module"]
pub type SWTRIGCTRL = crate::Reg<u32, _SWTRIGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWTRIGCTRL;
#[doc = "`read()` method returns [swtrigctrl::R](swtrigctrl::R) reader structure"]
impl crate::Readable for SWTRIGCTRL {}
#[doc = "`write(|w| ..)` method takes [swtrigctrl::W](swtrigctrl::W) writer structure"]
impl crate::Writable for SWTRIGCTRL {}
#[doc = "Software Trigger Control"]
pub mod swtrigctrl;
#[doc = "Priority Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prictrl0](prictrl0) module"]
pub type PRICTRL0 = crate::Reg<u32, _PRICTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRICTRL0;
#[doc = "`read()` method returns [prictrl0::R](prictrl0::R) reader structure"]
impl crate::Readable for PRICTRL0 {}
#[doc = "`write(|w| ..)` method takes [prictrl0::W](prictrl0::W) writer structure"]
impl crate::Writable for PRICTRL0 {}
#[doc = "Priority Control 0"]
pub mod prictrl0;
#[doc = "Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](intpend) module"]
pub type INTPEND = crate::Reg<u16, _INTPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPEND;
#[doc = "`read()` method returns [intpend::R](intpend::R) reader structure"]
impl crate::Readable for INTPEND {}
#[doc = "`write(|w| ..)` method takes [intpend::W](intpend::W) writer structure"]
impl crate::Writable for INTPEND {}
#[doc = "Interrupt Pending"]
pub mod intpend;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busych](busych) module"]
pub type BUSYCH = crate::Reg<u32, _BUSYCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSYCH;
#[doc = "`read()` method returns [busych::R](busych::R) reader structure"]
impl crate::Readable for BUSYCH {}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Pending Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pendch](pendch) module"]
pub type PENDCH = crate::Reg<u32, _PENDCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDCH;
#[doc = "`read()` method returns [pendch::R](pendch::R) reader structure"]
impl crate::Readable for PENDCH {}
#[doc = "Pending Channels"]
pub mod pendch;
#[doc = "Active Channel and Levels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active](active) module"]
pub type ACTIVE = crate::Reg<u32, _ACTIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTIVE;
#[doc = "`read()` method returns [active::R](active::R) reader structure"]
impl crate::Readable for ACTIVE {}
#[doc = "Active Channel and Levels"]
pub mod active;
#[doc = "Descriptor Memory Section Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baseaddr](baseaddr) module"]
pub type BASEADDR = crate::Reg<u32, _BASEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASEADDR;
#[doc = "`read()` method returns [baseaddr::R](baseaddr::R) reader structure"]
impl crate::Readable for BASEADDR {}
#[doc = "`write(|w| ..)` method takes [baseaddr::W](baseaddr::W) writer structure"]
impl crate::Writable for BASEADDR {}
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr;
#[doc = "Write-Back Memory Section Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrbaddr](wrbaddr) module"]
pub type WRBADDR = crate::Reg<u32, _WRBADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRBADDR;
#[doc = "`read()` method returns [wrbaddr::R](wrbaddr::R) reader structure"]
impl crate::Readable for WRBADDR {}
#[doc = "`write(|w| ..)` method takes [wrbaddr::W](wrbaddr::W) writer structure"]
impl crate::Writable for WRBADDR {}
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr;
