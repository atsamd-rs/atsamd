#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_count8: [u8; 0x38],
}
impl RegisterBlock {
    #[doc = "0x00..0x38 - 32-bit Counter Mode"]
    #[inline(always)]
    pub const fn count32(&self) -> &COUNT32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x34 - 16-bit Counter Mode"]
    #[inline(always)]
    pub const fn count16(&self) -> &COUNT16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x32 - 8-bit Counter Mode"]
    #[inline(always)]
    pub const fn count8(&self) -> &COUNT8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "8-bit Counter Mode"]
pub use self::count8::COUNT8;
#[doc = r"Cluster"]
#[doc = "8-bit Counter Mode"]
pub mod count8;
#[doc = "16-bit Counter Mode"]
pub use self::count16::COUNT16;
#[doc = r"Cluster"]
#[doc = "16-bit Counter Mode"]
pub mod count16;
#[doc = "32-bit Counter Mode"]
pub use self::count32::COUNT32;
#[doc = r"Cluster"]
#[doc = "32-bit Counter Mode"]
pub mod count32;
