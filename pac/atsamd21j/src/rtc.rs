#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_mode0: [u8; 0x1d],
}
impl RegisterBlock {
    #[doc = "0x00..0x1d - Clock/Calendar with Alarm"]
    #[inline(always)]
    pub const fn mode2(&self) -> &Mode2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00..0x1c - 16-bit Counter with Two 16-bit Compares"]
    #[inline(always)]
    pub const fn mode1(&self) -> &Mode1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00..0x1c - 32-bit Counter with Single 32-bit Compare"]
    #[inline(always)]
    pub const fn mode0(&self) -> &Mode0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
}
#[doc = "32-bit Counter with Single 32-bit Compare"]
pub use self::mode0::Mode0;
#[doc = r"Cluster"]
#[doc = "32-bit Counter with Single 32-bit Compare"]
pub mod mode0;
#[doc = "16-bit Counter with Two 16-bit Compares"]
pub use self::mode1::Mode1;
#[doc = r"Cluster"]
#[doc = "16-bit Counter with Two 16-bit Compares"]
pub mod mode1;
#[doc = "Clock/Calendar with Alarm"]
pub use self::mode2::Mode2;
#[doc = r"Cluster"]
#[doc = "Clock/Calendar with Alarm"]
pub mod mode2;
