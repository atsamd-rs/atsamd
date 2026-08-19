#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x03],
    clkctrl: [Clkctrl; 2],
    intenclr: Intenclr,
    _reserved3: [u8; 0x02],
    intenset: Intenset,
    _reserved4: [u8; 0x02],
    intflag: Intflag,
    _reserved5: [u8; 0x02],
    syncbusy: Syncbusy,
    _reserved6: [u8; 0x06],
    txctrl: Txctrl,
    rxctrl: Rxctrl,
    _reserved8: [u8; 0x08],
    txdata: Txdata,
    rxdata: Rxdata,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04..0x0c - Clock Unit n Control"]
    #[inline(always)]
    pub const fn clkctrl(&self, n: usize) -> &Clkctrl {
        &self.clkctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Clock Unit n Control"]
    #[inline(always)]
    pub fn clkctrl_iter(&self) -> impl Iterator<Item = &Clkctrl> {
        self.clkctrl.iter()
    }
    #[doc = "0x0c - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x18 - Synchronization Status"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x20 - Tx Serializer Control"]
    #[inline(always)]
    pub const fn txctrl(&self) -> &Txctrl {
        &self.txctrl
    }
    #[doc = "0x24 - Rx Serializer Control"]
    #[inline(always)]
    pub const fn rxctrl(&self) -> &Rxctrl {
        &self.rxctrl
    }
    #[doc = "0x30 - Tx Data"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x34 - Rx Data"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CLKCTRL (rw) register accessor: Clock Unit n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Clock Unit n Control"]
pub mod clkctrl;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Status"]
pub mod syncbusy;
#[doc = "TXCTRL (rw) register accessor: Tx Serializer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl`]
module"]
#[doc(alias = "TXCTRL")]
pub type Txctrl = crate::Reg<txctrl::TxctrlSpec>;
#[doc = "Tx Serializer Control"]
pub mod txctrl;
#[doc = "RXCTRL (rw) register accessor: Rx Serializer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrl`]
module"]
#[doc(alias = "RXCTRL")]
pub type Rxctrl = crate::Reg<rxctrl::RxctrlSpec>;
#[doc = "Rx Serializer Control"]
pub mod rxctrl;
#[doc = "TXDATA (w) register accessor: Tx Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "Tx Data"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: Rx Data\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "Rx Data"]
pub mod rxdata;
