#[repr(C)]
#[doc = "USART EXTERNAL CLOCK Mode"]
#[doc(alias = "USART_EXT")]
pub struct UsartExt {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    ctrlc: Ctrlc,
    _reserved_3_baud: [u8; 0x02],
    rxpl: Rxpl,
    _reserved5: [u8; 0x05],
    intenclr: Intenclr,
    _reserved6: [u8; 0x01],
    intenset: Intenset,
    _reserved7: [u8; 0x01],
    intflag: Intflag,
    _reserved8: [u8; 0x01],
    status: Status,
    syncbusy: Syncbusy,
    rxerrcnt: Rxerrcnt,
    _reserved11: [u8; 0x01],
    length: Length,
    _reserved12: [u8; 0x04],
    data: Data,
    _reserved13: [u8; 0x04],
    dbgctrl: Dbgctrl,
}
impl UsartExt {
    #[doc = "0x00 - USART_EXT Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - USART_EXT Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - USART_EXT Control C"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &Ctrlc {
        &self.ctrlc
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub const fn baud_usartfp_mode(&self) -> &BaudUsartfpMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub const fn baud_fracfp_mode(&self) -> &BaudFracfpMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub const fn baud_frac_mode(&self) -> &BaudFracMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub const fn baud(&self) -> &Baud {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - USART_EXT Receive Pulse Length"]
    #[inline(always)]
    pub const fn rxpl(&self) -> &Rxpl {
        &self.rxpl
    }
    #[doc = "0x14 - USART_EXT Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x16 - USART_EXT Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x18 - USART_EXT Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x1a - USART_EXT Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1c - USART_EXT Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x20 - USART_EXT Receive Error Count"]
    #[inline(always)]
    pub const fn rxerrcnt(&self) -> &Rxerrcnt {
        &self.rxerrcnt
    }
    #[doc = "0x22 - USART_EXT Length"]
    #[inline(always)]
    pub const fn length(&self) -> &Length {
        &self.length
    }
    #[doc = "0x28 - USART_EXT Data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x30 - USART_EXT Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
}
#[doc = "CTRLA (rw) register accessor: USART_EXT Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "USART_EXT Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: USART_EXT Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "USART_EXT Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: USART_EXT Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`] module"]
#[doc(alias = "CTRLC")]
pub type Ctrlc = crate::Reg<ctrlc::CtrlcSpec>;
#[doc = "USART_EXT Control C"]
pub mod ctrlc;
#[doc = "BAUD (rw) register accessor: USART_EXT Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`] module"]
#[doc(alias = "BAUD")]
pub type Baud = crate::Reg<baud::BaudSpec>;
#[doc = "USART_EXT Baud Rate"]
pub mod baud;
#[doc = "BAUD_FRAC_MODE (rw) register accessor: USART_EXT Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_frac_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_frac_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud_frac_mode`] module"]
#[doc(alias = "BAUD_FRAC_MODE")]
pub type BaudFracMode = crate::Reg<baud_frac_mode::BaudFracModeSpec>;
#[doc = "USART_EXT Baud Rate"]
pub mod baud_frac_mode;
#[doc = "BAUD_FRACFP_MODE (rw) register accessor: USART_EXT Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_fracfp_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_fracfp_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud_fracfp_mode`] module"]
#[doc(alias = "BAUD_FRACFP_MODE")]
pub type BaudFracfpMode = crate::Reg<baud_fracfp_mode::BaudFracfpModeSpec>;
#[doc = "USART_EXT Baud Rate"]
pub mod baud_fracfp_mode;
#[doc = "BAUD_USARTFP_MODE (rw) register accessor: USART_EXT Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_usartfp_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_usartfp_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud_usartfp_mode`] module"]
#[doc(alias = "BAUD_USARTFP_MODE")]
pub type BaudUsartfpMode = crate::Reg<baud_usartfp_mode::BaudUsartfpModeSpec>;
#[doc = "USART_EXT Baud Rate"]
pub mod baud_usartfp_mode;
#[doc = "RXPL (rw) register accessor: USART_EXT Receive Pulse Length\n\nYou can [`read`](crate::Reg::read) this register and get [`rxpl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxpl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxpl`] module"]
#[doc(alias = "RXPL")]
pub type Rxpl = crate::Reg<rxpl::RxplSpec>;
#[doc = "USART_EXT Receive Pulse Length"]
pub mod rxpl;
#[doc = "INTENCLR (rw) register accessor: USART_EXT Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "USART_EXT Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: USART_EXT Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "USART_EXT Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: USART_EXT Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "USART_EXT Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: USART_EXT Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "USART_EXT Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: USART_EXT Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "USART_EXT Synchronization Busy"]
pub mod syncbusy;
#[doc = "RXERRCNT (r) register accessor: USART_EXT Receive Error Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rxerrcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxerrcnt`] module"]
#[doc(alias = "RXERRCNT")]
pub type Rxerrcnt = crate::Reg<rxerrcnt::RxerrcntSpec>;
#[doc = "USART_EXT Receive Error Count"]
pub mod rxerrcnt;
#[doc = "LENGTH (rw) register accessor: USART_EXT Length\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@length`] module"]
#[doc(alias = "LENGTH")]
pub type Length = crate::Reg<length::LengthSpec>;
#[doc = "USART_EXT Length"]
pub mod length;
#[doc = "DATA (rw) register accessor: USART_EXT Data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "USART_EXT Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: USART_EXT Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "USART_EXT Debug Control"]
pub mod dbgctrl;
