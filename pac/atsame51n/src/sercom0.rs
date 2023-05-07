#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_i2cm: [u8; 0x31],
}
impl RegisterBlock {
    #[doc = "0x00..0x31 - USART INTERNAL CLOCK Mode"]
    #[inline(always)]
    pub const fn usart_int(&self) -> &USART_INT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x31 - USART EXTERNAL CLOCK Mode"]
    #[inline(always)]
    pub const fn usart_ext(&self) -> &USART_EXT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x31 - SPI Master Mode"]
    #[inline(always)]
    pub const fn spim(&self) -> &SPIM {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x31 - SPI Slave Mode"]
    #[inline(always)]
    pub const fn spis(&self) -> &SPIS {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x2c - I2C Slave Mode"]
    #[inline(always)]
    pub const fn i2cs(&self) -> &I2CS {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00..0x31 - I2C Master Mode"]
    #[inline(always)]
    pub const fn i2cm(&self) -> &I2CM {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "I2C Master Mode"]
pub use self::i2cm::I2CM;
#[doc = r"Cluster"]
#[doc = "I2C Master Mode"]
pub mod i2cm;
#[doc = "I2C Slave Mode"]
pub use self::i2cs::I2CS;
#[doc = r"Cluster"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = "SPI Slave Mode"]
pub use self::spis::SPIS;
#[doc = r"Cluster"]
#[doc = "SPI Slave Mode"]
pub mod spis;
#[doc = "SPI Master Mode"]
pub use self::spim::SPIM;
#[doc = r"Cluster"]
#[doc = "SPI Master Mode"]
pub mod spim;
#[doc = "USART EXTERNAL CLOCK Mode"]
pub use self::usart_ext::USART_EXT;
#[doc = r"Cluster"]
#[doc = "USART EXTERNAL CLOCK Mode"]
pub mod usart_ext;
#[doc = "USART INTERNAL CLOCK Mode"]
pub use self::usart_int::USART_INT;
#[doc = r"Cluster"]
#[doc = "USART INTERNAL CLOCK Mode"]
pub mod usart_int;
