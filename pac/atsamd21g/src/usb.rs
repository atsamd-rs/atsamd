#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_host: [u8; 0x0111],
}
impl RegisterBlock {
    #[doc = "0x00..0x111 - USB is Host"]
    #[inline(always)]
    pub const fn host(&self) -> &HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x111 - USB is Device"]
    #[inline(always)]
    pub const fn device(&self) -> &DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "USB is Device"]
pub use self::device::DEVICE;
#[doc = r"Cluster"]
#[doc = "USB is Device"]
pub mod device;
#[doc = "USB is Host"]
pub use self::host::HOST;
#[doc = r"Cluster"]
#[doc = "USB is Host"]
pub mod host;
