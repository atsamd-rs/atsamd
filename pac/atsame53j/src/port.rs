#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x60 - GROUP\\[%s\\]"]
    pub group0: GROUP,
    _reserved1: [u8; 0x20],
    #[doc = "0x80..0xe0 - GROUP\\[%s\\]"]
    pub group1: GROUP,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GROUP {
    #[doc = "0x00 - Data Direction"]
    pub dir: crate::Reg<self::group::dir::DIR_SPEC>,
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr: crate::Reg<self::group::dirclr::DIRCLR_SPEC>,
    #[doc = "0x08 - Data Direction Set"]
    pub dirset: crate::Reg<self::group::dirset::DIRSET_SPEC>,
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl: crate::Reg<self::group::dirtgl::DIRTGL_SPEC>,
    #[doc = "0x10 - Data Output Value"]
    pub out: crate::Reg<self::group::out::OUT_SPEC>,
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr: crate::Reg<self::group::outclr::OUTCLR_SPEC>,
    #[doc = "0x18 - Data Output Value Set"]
    pub outset: crate::Reg<self::group::outset::OUTSET_SPEC>,
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl: crate::Reg<self::group::outtgl::OUTTGL_SPEC>,
    #[doc = "0x20 - Data Input Value"]
    pub in_: crate::Reg<self::group::in_::IN_SPEC>,
    #[doc = "0x24 - Control"]
    pub ctrl: crate::Reg<self::group::ctrl::CTRL_SPEC>,
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig: crate::Reg<self::group::wrconfig::WRCONFIG_SPEC>,
    #[doc = "0x2c - Event Input Control"]
    pub evctrl: crate::Reg<self::group::evctrl::EVCTRL_SPEC>,
    #[doc = "0x30..0x40 - Peripheral Multiplexing"]
    pub pmux: [crate::Reg<self::group::pmux::PMUX_SPEC>; 16],
    #[doc = "0x40..0x60 - Pin Configuration"]
    pub pincfg: [crate::Reg<self::group::pincfg::PINCFG_SPEC>; 32],
}
#[doc = r"Register block"]
#[doc = "GROUP\\[%s\\]"]
pub mod group;
