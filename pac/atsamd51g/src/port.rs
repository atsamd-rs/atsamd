#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GROUP\\[%s\\]"]
    pub group0: GROUP,
    _reserved1: [u8; 32usize],
    #[doc = "0x80 - GROUP\\[%s\\]"]
    pub group1: GROUP,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GROUP {
    #[doc = "0x00 - Data Direction"]
    pub dir: self::group::DIR,
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr: self::group::DIRCLR,
    #[doc = "0x08 - Data Direction Set"]
    pub dirset: self::group::DIRSET,
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl: self::group::DIRTGL,
    #[doc = "0x10 - Data Output Value"]
    pub out: self::group::OUT,
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr: self::group::OUTCLR,
    #[doc = "0x18 - Data Output Value Set"]
    pub outset: self::group::OUTSET,
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl: self::group::OUTTGL,
    #[doc = "0x20 - Data Input Value"]
    pub in_: self::group::IN,
    #[doc = "0x24 - Control"]
    pub ctrl: self::group::CTRL,
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig: self::group::WRCONFIG,
    #[doc = "0x2c - Event Input Control"]
    pub evctrl: self::group::EVCTRL,
    #[doc = "0x30 - Peripheral Multiplexing"]
    pub pmux: [self::group::PMUX; 16],
    #[doc = "0x40 - Pin Configuration"]
    pub pincfg: [self::group::PINCFG; 32],
}
#[doc = r"Register block"]
#[doc = "GROUP\\[%s\\]"]
pub mod group;
