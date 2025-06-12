#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x03],
    syncbusy: Syncbusy,
    _reserved2: [u8; 0x18],
    genctrl: [Genctrl; 12],
    _reserved3: [u8; 0x30],
    pchctrl: [Pchctrl; 48],
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x20..0x50 - Generic Clock Generator Control"]
    #[inline(always)]
    pub const fn genctrl(&self, n: usize) -> &Genctrl {
        &self.genctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x50 - Generic Clock Generator Control"]
    #[inline(always)]
    pub fn genctrl_iter(&self) -> impl Iterator<Item = &Genctrl> {
        self.genctrl.iter()
    }
    #[doc = "0x80..0x140 - Peripheral Clock Control"]
    #[inline(always)]
    pub const fn pchctrl(&self, n: usize) -> &Pchctrl {
        &self.pchctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x140 - Peripheral Clock Control"]
    #[inline(always)]
    pub fn pchctrl_iter(&self) -> impl Iterator<Item = &Pchctrl> {
        self.pchctrl.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "GENCTRL (rw) register accessor: Generic Clock Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`genctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genctrl`] module"]
#[doc(alias = "GENCTRL")]
pub type Genctrl = crate::Reg<genctrl::GenctrlSpec>;
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "PCHCTRL (rw) register accessor: Peripheral Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pchctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pchctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pchctrl`] module"]
#[doc(alias = "PCHCTRL")]
pub type Pchctrl = crate::Reg<pchctrl::PchctrlSpec>;
#[doc = "Peripheral Clock Control"]
pub mod pchctrl;
