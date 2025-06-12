#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_host: [u8; 0x01ea],
}
impl RegisterBlock {
    #[doc = "0x00..0x1ea - USB is Host"]
    #[inline(always)]
    pub const fn host(&self) -> &Host {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x1ea - USB is Device"]
    #[inline(always)]
    pub const fn device(&self) -> &Device {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
}
#[doc = "USB is Device"]
pub use self::device::Device;
#[doc = r"Cluster"]
#[doc = "USB is Device"]
pub mod device;
#[doc = "USB is Host"]
pub use self::host::Host;
#[doc = r"Cluster"]
#[doc = "USB is Host"]
pub mod host;
