#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    cfga: Cfga,
    _reserved3: [u8; 0x04],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    syncbusy: Syncbusy,
    value: Value,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x01 - Control B Register"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x02 - Config A register"]
    #[inline(always)]
    pub const fn cfga(&self) -> &Cfga {
        &self.cfga
    }
    #[doc = "0x08 - Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x09 - Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0a - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0b - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x10 - Count Value Register"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "CTRLA (rw) register accessor: Control A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A Register"]
pub mod ctrla;
#[doc = "CTRLB (w) register accessor: Control B Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B Register"]
pub mod ctrlb;
#[doc = "CFGA (rw) register accessor: Config A register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfga`]
module"]
#[doc(alias = "CFGA")]
pub type Cfga = crate::Reg<cfga::CfgaSpec>;
#[doc = "Config A register"]
pub mod cfga;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Register"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "VALUE (r) register accessor: Count Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Count Value Register"]
pub mod value;
