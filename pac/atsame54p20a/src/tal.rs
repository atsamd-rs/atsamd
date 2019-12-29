#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - External Break Control"]
    pub extctrl: EXTCTRL,
    _reserved2: [u8; 2usize],
    #[doc = "0x04 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved3: [u8; 2usize],
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0b - Global Break Requests Mask"]
    pub globmask: GLOBMASK,
    #[doc = "0x0c - Debug Halt Request"]
    pub halt: HALT,
    #[doc = "0x0d - Debug Restart Request"]
    pub restart: RESTART,
    #[doc = "0x0e - Break Request Status"]
    pub brkstatus: BRKSTATUS,
    #[doc = "0x10 - Cross-Trigger Interface n Control A"]
    pub ctictrla0: CTICTRLA,
    #[doc = "0x11 - Cross-Trigger Interface n Mask"]
    pub ctimask0: CTIMASK,
    #[doc = "0x12 - Cross-Trigger Interface n Control A"]
    pub ctictrla1: CTICTRLA,
    #[doc = "0x13 - Cross-Trigger Interface n Mask"]
    pub ctimask1: CTIMASK,
    #[doc = "0x14 - Cross-Trigger Interface n Control A"]
    pub ctictrla2: CTICTRLA,
    #[doc = "0x15 - Cross-Trigger Interface n Mask"]
    pub ctimask2: CTIMASK,
    #[doc = "0x16 - Cross-Trigger Interface n Control A"]
    pub ctictrla3: CTICTRLA,
    #[doc = "0x17 - Cross-Trigger Interface n Mask"]
    pub ctimask3: CTIMASK,
    _reserved18: [u8; 8usize],
    #[doc = "0x20 - Interrupt n Status"]
    pub intstatus: [INTSTATUS; 137],
    _reserved19: [u8; 103usize],
    #[doc = "0x110 - DMA Channel Interrupts CPU Select 0"]
    pub dmacpusel0: DMACPUSEL0,
    #[doc = "0x114 - DMA Channel Interrupts CPU Select 1"]
    pub dmacpusel1: DMACPUSEL1,
    #[doc = "0x118 - EVSYS Channel Interrupts CPU Select 0"]
    pub evcpusel0: EVCPUSEL0,
    _reserved22: [u8; 4usize],
    #[doc = "0x120 - EIC External Interrupts CPU Select 0"]
    pub eiccpusel0: EICCPUSEL0,
    _reserved23: [u8; 4usize],
    #[doc = "0x128 - Interrupts CPU Select 0"]
    pub intcpusel0: INTCPUSEL0,
    #[doc = "0x12c - Interrupts CPU Select 1"]
    pub intcpusel1: INTCPUSEL1,
    #[doc = "0x130 - Interrupts CPU Select 2"]
    pub intcpusel2: INTCPUSEL2,
    #[doc = "0x134 - Interrupts CPU Select 3"]
    pub intcpusel3: INTCPUSEL3,
    #[doc = "0x138 - Interrupts CPU Select 4"]
    pub intcpusel4: INTCPUSEL4,
    #[doc = "0x13c - Interrupts CPU Select 5"]
    pub intcpusel5: INTCPUSEL5,
    #[doc = "0x140 - Interrupts CPU Select 6"]
    pub intcpusel6: INTCPUSEL6,
    #[doc = "0x144 - Interrupts CPU Select 7"]
    pub intcpusel7: INTCPUSEL7,
    #[doc = "0x148 - Interrupts CPU Select 8"]
    pub intcpusel8: INTCPUSEL8,
    _reserved32: [u8; 24usize],
    #[doc = "0x164 - Interrupt Trigger"]
    pub irqtrig: IRQTRIG,
    #[doc = "0x168 - Interrupt Monitor Select"]
    pub irqmon: [IRQMON; 1],
    _reserved34: [u8; 22usize],
    #[doc = "0x180 - Interrupt Status m for CPU n - Group 0"]
    pub cpuirqs0_: [CPUIRQS0_; 5],
    _reserved35: [u8; 12usize],
    #[doc = "0x1a0 - Interrupt Status m for CPU n - Group 1"]
    pub cpuirqs1_: [CPUIRQS1_; 5],
    _reserved36: [u8; 76usize],
    #[doc = "0x200 - Inter-Process Signal Mask m for CPU n - Group 0"]
    pub smask0_: [SMASK0_; 2],
    #[doc = "0x208 - Inter-Process Signal Mask m for CPU n - Group 1"]
    pub smask1_: [SMASK1_; 2],
    _reserved38: [u8; 16usize],
    #[doc = "0x220 - Inter-Process Signal Flag Clear"]
    pub sflagclr: [SFLAGCLR; 2],
    #[doc = "0x228 - Inter-Process Signal Flag Set"]
    pub sflagset: [SFLAGSET; 2],
    #[doc = "0x230 - Inter-Process Signal Flag"]
    pub sflag: [SFLAG; 2],
    _reserved41: [u8; 200usize],
    #[doc = "0x300 - Inter-Process Signal Flag Bit n"]
    pub sflagclrr: [SFLAGCLRR; 64],
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "External Break Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extctrl](extctrl) module"]
pub type EXTCTRL = crate::Reg<u8, _EXTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTCTRL;
#[doc = "`read()` method returns [extctrl::R](extctrl::R) reader structure"]
impl crate::Readable for EXTCTRL {}
#[doc = "`write(|w| ..)` method takes [extctrl::W](extctrl::W) writer structure"]
impl crate::Writable for EXTCTRL {}
#[doc = "External Break Control"]
pub mod extctrl;
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u16, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Global Break Requests Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globmask](globmask) module"]
pub type GLOBMASK = crate::Reg<u8, _GLOBMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBMASK;
#[doc = "`read()` method returns [globmask::R](globmask::R) reader structure"]
impl crate::Readable for GLOBMASK {}
#[doc = "`write(|w| ..)` method takes [globmask::W](globmask::W) writer structure"]
impl crate::Writable for GLOBMASK {}
#[doc = "Global Break Requests Mask"]
pub mod globmask;
#[doc = "Debug Halt Request\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halt](halt) module"]
pub type HALT = crate::Reg<u8, _HALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HALT;
#[doc = "`write(|w| ..)` method takes [halt::W](halt::W) writer structure"]
impl crate::Writable for HALT {}
#[doc = "Debug Halt Request"]
pub mod halt;
#[doc = "Debug Restart Request\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [restart](restart) module"]
pub type RESTART = crate::Reg<u8, _RESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESTART;
#[doc = "`write(|w| ..)` method takes [restart::W](restart::W) writer structure"]
impl crate::Writable for RESTART {}
#[doc = "Debug Restart Request"]
pub mod restart;
#[doc = "Break Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brkstatus](brkstatus) module"]
pub type BRKSTATUS = crate::Reg<u16, _BRKSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRKSTATUS;
#[doc = "`read()` method returns [brkstatus::R](brkstatus::R) reader structure"]
impl crate::Readable for BRKSTATUS {}
#[doc = "Break Request Status"]
pub mod brkstatus;
#[doc = "Cross-Trigger Interface n Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctictrla](ctictrla) module"]
pub type CTICTRLA = crate::Reg<u8, _CTICTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTICTRLA;
#[doc = "`read()` method returns [ctictrla::R](ctictrla::R) reader structure"]
impl crate::Readable for CTICTRLA {}
#[doc = "`write(|w| ..)` method takes [ctictrla::W](ctictrla::W) writer structure"]
impl crate::Writable for CTICTRLA {}
#[doc = "Cross-Trigger Interface n Control A"]
pub mod ctictrla;
#[doc = "Cross-Trigger Interface n Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctimask](ctimask) module"]
pub type CTIMASK = crate::Reg<u8, _CTIMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMASK;
#[doc = "`read()` method returns [ctimask::R](ctimask::R) reader structure"]
impl crate::Readable for CTIMASK {}
#[doc = "`write(|w| ..)` method takes [ctimask::W](ctimask::W) writer structure"]
impl crate::Writable for CTIMASK {}
#[doc = "Cross-Trigger Interface n Mask"]
pub mod ctimask;
#[doc = "Interrupt n Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u8, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Interrupt n Status"]
pub mod intstatus;
#[doc = "DMA Channel Interrupts CPU Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacpusel0](dmacpusel0) module"]
pub type DMACPUSEL0 = crate::Reg<u32, _DMACPUSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACPUSEL0;
#[doc = "`read()` method returns [dmacpusel0::R](dmacpusel0::R) reader structure"]
impl crate::Readable for DMACPUSEL0 {}
#[doc = "`write(|w| ..)` method takes [dmacpusel0::W](dmacpusel0::W) writer structure"]
impl crate::Writable for DMACPUSEL0 {}
#[doc = "DMA Channel Interrupts CPU Select 0"]
pub mod dmacpusel0;
#[doc = "DMA Channel Interrupts CPU Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacpusel1](dmacpusel1) module"]
pub type DMACPUSEL1 = crate::Reg<u32, _DMACPUSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACPUSEL1;
#[doc = "`read()` method returns [dmacpusel1::R](dmacpusel1::R) reader structure"]
impl crate::Readable for DMACPUSEL1 {}
#[doc = "`write(|w| ..)` method takes [dmacpusel1::W](dmacpusel1::W) writer structure"]
impl crate::Writable for DMACPUSEL1 {}
#[doc = "DMA Channel Interrupts CPU Select 1"]
pub mod dmacpusel1;
#[doc = "EVSYS Channel Interrupts CPU Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evcpusel0](evcpusel0) module"]
pub type EVCPUSEL0 = crate::Reg<u32, _EVCPUSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCPUSEL0;
#[doc = "`read()` method returns [evcpusel0::R](evcpusel0::R) reader structure"]
impl crate::Readable for EVCPUSEL0 {}
#[doc = "`write(|w| ..)` method takes [evcpusel0::W](evcpusel0::W) writer structure"]
impl crate::Writable for EVCPUSEL0 {}
#[doc = "EVSYS Channel Interrupts CPU Select 0"]
pub mod evcpusel0;
#[doc = "EIC External Interrupts CPU Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eiccpusel0](eiccpusel0) module"]
pub type EICCPUSEL0 = crate::Reg<u32, _EICCPUSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EICCPUSEL0;
#[doc = "`read()` method returns [eiccpusel0::R](eiccpusel0::R) reader structure"]
impl crate::Readable for EICCPUSEL0 {}
#[doc = "`write(|w| ..)` method takes [eiccpusel0::W](eiccpusel0::W) writer structure"]
impl crate::Writable for EICCPUSEL0 {}
#[doc = "EIC External Interrupts CPU Select 0"]
pub mod eiccpusel0;
#[doc = "Interrupts CPU Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel0](intcpusel0) module"]
pub type INTCPUSEL0 = crate::Reg<u32, _INTCPUSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL0;
#[doc = "`read()` method returns [intcpusel0::R](intcpusel0::R) reader structure"]
impl crate::Readable for INTCPUSEL0 {}
#[doc = "`write(|w| ..)` method takes [intcpusel0::W](intcpusel0::W) writer structure"]
impl crate::Writable for INTCPUSEL0 {}
#[doc = "Interrupts CPU Select 0"]
pub mod intcpusel0;
#[doc = "Interrupts CPU Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel1](intcpusel1) module"]
pub type INTCPUSEL1 = crate::Reg<u32, _INTCPUSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL1;
#[doc = "`read()` method returns [intcpusel1::R](intcpusel1::R) reader structure"]
impl crate::Readable for INTCPUSEL1 {}
#[doc = "`write(|w| ..)` method takes [intcpusel1::W](intcpusel1::W) writer structure"]
impl crate::Writable for INTCPUSEL1 {}
#[doc = "Interrupts CPU Select 1"]
pub mod intcpusel1;
#[doc = "Interrupts CPU Select 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel2](intcpusel2) module"]
pub type INTCPUSEL2 = crate::Reg<u32, _INTCPUSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL2;
#[doc = "`read()` method returns [intcpusel2::R](intcpusel2::R) reader structure"]
impl crate::Readable for INTCPUSEL2 {}
#[doc = "`write(|w| ..)` method takes [intcpusel2::W](intcpusel2::W) writer structure"]
impl crate::Writable for INTCPUSEL2 {}
#[doc = "Interrupts CPU Select 2"]
pub mod intcpusel2;
#[doc = "Interrupts CPU Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel3](intcpusel3) module"]
pub type INTCPUSEL3 = crate::Reg<u32, _INTCPUSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL3;
#[doc = "`read()` method returns [intcpusel3::R](intcpusel3::R) reader structure"]
impl crate::Readable for INTCPUSEL3 {}
#[doc = "`write(|w| ..)` method takes [intcpusel3::W](intcpusel3::W) writer structure"]
impl crate::Writable for INTCPUSEL3 {}
#[doc = "Interrupts CPU Select 3"]
pub mod intcpusel3;
#[doc = "Interrupts CPU Select 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel4](intcpusel4) module"]
pub type INTCPUSEL4 = crate::Reg<u32, _INTCPUSEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL4;
#[doc = "`read()` method returns [intcpusel4::R](intcpusel4::R) reader structure"]
impl crate::Readable for INTCPUSEL4 {}
#[doc = "`write(|w| ..)` method takes [intcpusel4::W](intcpusel4::W) writer structure"]
impl crate::Writable for INTCPUSEL4 {}
#[doc = "Interrupts CPU Select 4"]
pub mod intcpusel4;
#[doc = "Interrupts CPU Select 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel5](intcpusel5) module"]
pub type INTCPUSEL5 = crate::Reg<u32, _INTCPUSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL5;
#[doc = "`read()` method returns [intcpusel5::R](intcpusel5::R) reader structure"]
impl crate::Readable for INTCPUSEL5 {}
#[doc = "`write(|w| ..)` method takes [intcpusel5::W](intcpusel5::W) writer structure"]
impl crate::Writable for INTCPUSEL5 {}
#[doc = "Interrupts CPU Select 5"]
pub mod intcpusel5;
#[doc = "Interrupts CPU Select 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel6](intcpusel6) module"]
pub type INTCPUSEL6 = crate::Reg<u32, _INTCPUSEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL6;
#[doc = "`read()` method returns [intcpusel6::R](intcpusel6::R) reader structure"]
impl crate::Readable for INTCPUSEL6 {}
#[doc = "`write(|w| ..)` method takes [intcpusel6::W](intcpusel6::W) writer structure"]
impl crate::Writable for INTCPUSEL6 {}
#[doc = "Interrupts CPU Select 6"]
pub mod intcpusel6;
#[doc = "Interrupts CPU Select 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel7](intcpusel7) module"]
pub type INTCPUSEL7 = crate::Reg<u32, _INTCPUSEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL7;
#[doc = "`read()` method returns [intcpusel7::R](intcpusel7::R) reader structure"]
impl crate::Readable for INTCPUSEL7 {}
#[doc = "`write(|w| ..)` method takes [intcpusel7::W](intcpusel7::W) writer structure"]
impl crate::Writable for INTCPUSEL7 {}
#[doc = "Interrupts CPU Select 7"]
pub mod intcpusel7;
#[doc = "Interrupts CPU Select 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcpusel8](intcpusel8) module"]
pub type INTCPUSEL8 = crate::Reg<u32, _INTCPUSEL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCPUSEL8;
#[doc = "`read()` method returns [intcpusel8::R](intcpusel8::R) reader structure"]
impl crate::Readable for INTCPUSEL8 {}
#[doc = "`write(|w| ..)` method takes [intcpusel8::W](intcpusel8::W) writer structure"]
impl crate::Writable for INTCPUSEL8 {}
#[doc = "Interrupts CPU Select 8"]
pub mod intcpusel8;
#[doc = "Interrupt Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqtrig](irqtrig) module"]
pub type IRQTRIG = crate::Reg<u32, _IRQTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQTRIG;
#[doc = "`read()` method returns [irqtrig::R](irqtrig::R) reader structure"]
impl crate::Readable for IRQTRIG {}
#[doc = "`write(|w| ..)` method takes [irqtrig::W](irqtrig::W) writer structure"]
impl crate::Writable for IRQTRIG {}
#[doc = "Interrupt Trigger"]
pub mod irqtrig;
#[doc = "Interrupt Monitor Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqmon](irqmon) module"]
pub type IRQMON = crate::Reg<u16, _IRQMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQMON;
#[doc = "`read()` method returns [irqmon::R](irqmon::R) reader structure"]
impl crate::Readable for IRQMON {}
#[doc = "`write(|w| ..)` method takes [irqmon::W](irqmon::W) writer structure"]
impl crate::Writable for IRQMON {}
#[doc = "Interrupt Monitor Select"]
pub mod irqmon;
#[doc = "Interrupt Status m for CPU n - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqs0_](cpuirqs0_) module"]
pub type CPUIRQS0_ = crate::Reg<u32, _CPUIRQS0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQS0_;
#[doc = "`read()` method returns [cpuirqs0_::R](cpuirqs0_::R) reader structure"]
impl crate::Readable for CPUIRQS0_ {}
#[doc = "Interrupt Status m for CPU n - Group 0"]
pub mod cpuirqs0_;
#[doc = "Interrupt Status m for CPU n - Group 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqs1_](cpuirqs1_) module"]
pub type CPUIRQS1_ = crate::Reg<u32, _CPUIRQS1_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQS1_;
#[doc = "`read()` method returns [cpuirqs1_::R](cpuirqs1_::R) reader structure"]
impl crate::Readable for CPUIRQS1_ {}
#[doc = "Interrupt Status m for CPU n - Group 1"]
pub mod cpuirqs1_;
#[doc = "Inter-Process Signal Mask m for CPU n - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smask0_](smask0_) module"]
pub type SMASK0_ = crate::Reg<u32, _SMASK0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMASK0_;
#[doc = "`read()` method returns [smask0_::R](smask0_::R) reader structure"]
impl crate::Readable for SMASK0_ {}
#[doc = "`write(|w| ..)` method takes [smask0_::W](smask0_::W) writer structure"]
impl crate::Writable for SMASK0_ {}
#[doc = "Inter-Process Signal Mask m for CPU n - Group 0"]
pub mod smask0_;
#[doc = "Inter-Process Signal Mask m for CPU n - Group 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smask1_](smask1_) module"]
pub type SMASK1_ = crate::Reg<u32, _SMASK1_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMASK1_;
#[doc = "`read()` method returns [smask1_::R](smask1_::R) reader structure"]
impl crate::Readable for SMASK1_ {}
#[doc = "`write(|w| ..)` method takes [smask1_::W](smask1_::W) writer structure"]
impl crate::Writable for SMASK1_ {}
#[doc = "Inter-Process Signal Mask m for CPU n - Group 1"]
pub mod smask1_;
#[doc = "Inter-Process Signal Flag Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sflagclr](sflagclr) module"]
pub type SFLAGCLR = crate::Reg<u32, _SFLAGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFLAGCLR;
#[doc = "`write(|w| ..)` method takes [sflagclr::W](sflagclr::W) writer structure"]
impl crate::Writable for SFLAGCLR {}
#[doc = "Inter-Process Signal Flag Clear"]
pub mod sflagclr;
#[doc = "Inter-Process Signal Flag Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sflagset](sflagset) module"]
pub type SFLAGSET = crate::Reg<u32, _SFLAGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFLAGSET;
#[doc = "`write(|w| ..)` method takes [sflagset::W](sflagset::W) writer structure"]
impl crate::Writable for SFLAGSET {}
#[doc = "Inter-Process Signal Flag Set"]
pub mod sflagset;
#[doc = "Inter-Process Signal Flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sflag](sflag) module"]
pub type SFLAG = crate::Reg<u32, _SFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFLAG;
#[doc = "`read()` method returns [sflag::R](sflag::R) reader structure"]
impl crate::Readable for SFLAG {}
#[doc = "Inter-Process Signal Flag"]
pub mod sflag;
#[doc = "Inter-Process Signal Flag Bit n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sflagclrr](sflagclrr) module"]
pub type SFLAGCLRR = crate::Reg<u8, _SFLAGCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFLAGCLRR;
#[doc = "`read()` method returns [sflagclrr::R](sflagclrr::R) reader structure"]
impl crate::Readable for SFLAGCLRR {}
#[doc = "`write(|w| ..)` method takes [sflagclrr::W](sflagclrr::W) writer structure"]
impl crate::Writable for SFLAGCLRR {}
#[doc = "Inter-Process Signal Flag Bit n"]
pub mod sflagclrr;
