#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_spi: [u8; 0x31],
}
impl RegisterBlock {
    #[doc = "0x00..0x31 - USART Mode"]
    #[inline(always)]
    pub const fn usart(&self) -> &Usart {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00..0x31 - SPI Mode"]
    #[inline(always)]
    pub const fn spi(&self) -> &Spi {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00..0x29 - I2C Slave Mode"]
    #[inline(always)]
    pub const fn i2cs(&self) -> &I2cs {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00..0x31 - I2C Master Mode"]
    #[inline(always)]
    pub const fn i2cm(&self) -> &I2cm {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
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
#[doc = "SPI Mode"]
pub use self::spi::Spi;
#[doc = r"Cluster"]
#[doc = "SPI Mode"]
pub mod spi;
#[doc = "USART Mode"]
pub use self::usart::Usart;
#[doc = r"Cluster"]
#[doc = "USART Mode"]
pub mod usart;
