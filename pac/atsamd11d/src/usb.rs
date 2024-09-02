#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    device: Device,
}
impl RegisterBlock {
    #[doc = "0x00..0x109 - USB is Device"]
    #[inline(always)]
    pub const fn device(&self) -> &Device {
        &self.device
    }
}
#[doc = "USB is Device"]
pub use self::device::Device;
#[doc = r"Cluster"]
#[doc = "USB is Device"]
pub mod device;
