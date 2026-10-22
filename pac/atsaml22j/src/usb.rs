#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x111 - USB is Device"]
    pub device: DEVICE,
}
#[doc = "USB is Device"]
pub use self::device::DEVICE;
#[doc = r"Cluster"]
#[doc = "USB is Device"]
pub mod device;
