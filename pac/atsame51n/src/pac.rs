#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: WRCTRL,
    #[doc = "0x04 - Event control"]
    pub evctrl: EVCTRL,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: INTFLAGAHB,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: INTFLAGA,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: INTFLAGB,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: INTFLAGC,
    #[doc = "0x20 - Peripheral interrupt flag status - Bridge D"]
    pub intflagd: INTFLAGD,
    _reserved9: [u8; 0x10],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: STATUSA,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: STATUSB,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: STATUSC,
    #[doc = "0x40 - Peripheral write protection status - Bridge D"]
    pub statusd: STATUSD,
}
#[doc = "WRCTRL (rw) register accessor: Write control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrctrl`]
module"]
pub type WRCTRL = crate::Reg<wrctrl::WRCTRL_SPEC>;
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "EVCTRL (rw) register accessor: Event control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: Interrupt enable clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt enable set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "INTFLAGAHB (rw) register accessor: Bridge interrupt flag status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagahb`]
module"]
pub type INTFLAGAHB = crate::Reg<intflagahb::INTFLAGAHB_SPEC>;
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "INTFLAGA (rw) register accessor: Peripheral interrupt flag status - Bridge A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflaga::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflaga::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflaga`]
module"]
pub type INTFLAGA = crate::Reg<intflaga::INTFLAGA_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "INTFLAGB (rw) register accessor: Peripheral interrupt flag status - Bridge B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagb`]
module"]
pub type INTFLAGB = crate::Reg<intflagb::INTFLAGB_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "INTFLAGC (rw) register accessor: Peripheral interrupt flag status - Bridge C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagc`]
module"]
pub type INTFLAGC = crate::Reg<intflagc::INTFLAGC_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "INTFLAGD (rw) register accessor: Peripheral interrupt flag status - Bridge D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagd`]
module"]
pub type INTFLAGD = crate::Reg<intflagd::INTFLAGD_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge D"]
pub mod intflagd;
#[doc = "STATUSA (r) register accessor: Peripheral write protection status - Bridge A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusa::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`]
module"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: Peripheral write protection status - Bridge B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusb`]
module"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "STATUSC (r) register accessor: Peripheral write protection status - Bridge C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusc`]
module"]
pub type STATUSC = crate::Reg<statusc::STATUSC_SPEC>;
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "STATUSD (r) register accessor: Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusd`]
module"]
pub type STATUSD = crate::Reg<statusd::STATUSD_SPEC>;
#[doc = "Peripheral write protection status - Bridge D"]
pub mod statusd;
