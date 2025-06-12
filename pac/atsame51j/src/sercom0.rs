#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_i2cm: [u8; 0x31],
}
impl RegisterBlock {
    #[doc = "0x00..0x31 - USART INTERNAL CLOCK Mode"]
    #[inline(always)]
    pub const fn usart_int(&self) -> &UsartInt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x31 - USART EXTERNAL CLOCK Mode"]
    #[inline(always)]
    pub const fn usart_ext(&self) -> &UsartExt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x31 - SPI Master Mode"]
    #[inline(always)]
    pub const fn spim(&self) -> &Spim {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x31 - SPI Slave Mode"]
    #[inline(always)]
    pub const fn spis(&self) -> &Spis {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x2c - I2C Slave Mode"]
    #[inline(always)]
    pub const fn i2cs(&self) -> &I2cs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x31 - I2C Master Mode"]
    #[inline(always)]
    pub const fn i2cm(&self) -> &I2cm {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
}
#[doc = "I2C Master Mode"]
pub use self::i2cm::I2cm;
#[doc = r"Cluster"]
#[doc = "I2C Master Mode"]
pub mod i2cm;
#[doc = "I2C Slave Mode"]
pub use self::i2cs::I2cs;
#[doc = r"Cluster"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = "SPI Slave Mode"]
pub use self::spis::Spis;
#[doc = r"Cluster"]
#[doc = "SPI Slave Mode"]
pub mod spis;
#[doc = "SPI Master Mode"]
pub use self::spim::Spim;
#[doc = r"Cluster"]
#[doc = "SPI Master Mode"]
pub mod spim;
#[doc = "USART EXTERNAL CLOCK Mode"]
pub use self::usart_ext::UsartExt;
#[doc = r"Cluster"]
#[doc = "USART EXTERNAL CLOCK Mode"]
pub mod usart_ext;
#[doc = "USART INTERNAL CLOCK Mode"]
pub use self::usart_int::UsartInt;
#[doc = r"Cluster"]
#[doc = "USART INTERNAL CLOCK Mode"]
pub mod usart_int;
