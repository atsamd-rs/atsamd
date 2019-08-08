#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLA {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "USART mode with external clock"]
    USART_EXT_CLK,
    #[doc = "USART mode with internal clock"]
    USART_INT_CLK,
    #[doc = "SPI mode with external clock"]
    SPI_SLAVE,
    #[doc = "SPI mode with internal clock"]
    SPI_MASTER,
    #[doc = "I2C mode with external clock"]
    I2C_SLAVE,
    #[doc = "I2C mode with internal clock"]
    I2C_MASTER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::USART_EXT_CLK => 0,
            MODER::USART_INT_CLK => 1,
            MODER::SPI_SLAVE => 2,
            MODER::SPI_MASTER => 3,
            MODER::I2C_SLAVE => 4,
            MODER::I2C_MASTER => 5,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::USART_EXT_CLK,
            1 => MODER::USART_INT_CLK,
            2 => MODER::SPI_SLAVE,
            3 => MODER::SPI_MASTER,
            4 => MODER::I2C_SLAVE,
            5 => MODER::I2C_MASTER,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODER::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODER::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODER::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline]
    pub fn is_spi_master(&self) -> bool {
        *self == MODER::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODER::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODER::I2C_MASTER
    }
}
#[doc = r" Value of the field"]
pub struct RUNSTDBYR {
    bits: bool,
}
impl RUNSTDBYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PINOUTR {
    bits: bool,
}
impl PINOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SDAHOLDR {
    bits: u8,
}
impl SDAHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEXTTOENR {
    bits: bool,
}
impl MEXTTOENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SEXTTOENR {
    bits: bool,
}
impl SEXTTOENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SPEEDR {
    bits: u8,
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCLSMR {
    bits: bool,
}
impl SCLSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct INACTOUTR {
    bits: u8,
}
impl INACTOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOWTOUTENR {
    bits: bool,
}
impl LOWTOUTENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "USART mode with external clock"]
    USART_EXT_CLK,
    #[doc = "USART mode with internal clock"]
    USART_INT_CLK,
    #[doc = "SPI mode with external clock"]
    SPI_SLAVE,
    #[doc = "SPI mode with internal clock"]
    SPI_MASTER,
    #[doc = "I2C mode with external clock"]
    I2C_SLAVE,
    #[doc = "I2C mode with internal clock"]
    I2C_MASTER,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::USART_EXT_CLK => 0,
            MODEW::USART_INT_CLK => 1,
            MODEW::SPI_SLAVE => 2,
            MODEW::SPI_MASTER => 3,
            MODEW::I2C_SLAVE => 4,
            MODEW::I2C_MASTER => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "USART mode with external clock"]
    #[inline]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODEW::USART_EXT_CLK)
    }
    #[doc = "USART mode with internal clock"]
    #[inline]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODEW::USART_INT_CLK)
    }
    #[doc = "SPI mode with external clock"]
    #[inline]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODEW::SPI_SLAVE)
    }
    #[doc = "SPI mode with internal clock"]
    #[inline]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODEW::SPI_MASTER)
    }
    #[doc = "I2C mode with external clock"]
    #[inline]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODEW::I2C_SLAVE)
    }
    #[doc = "I2C mode with internal clock"]
    #[inline]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODEW::I2C_MASTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNSTDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNSTDBYW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINOUTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDAHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _SDAHOLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEXTTOENW<'a> {
    w: &'a mut W,
}
impl<'a> _MEXTTOENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEXTTOENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEXTTOENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCLSMW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLSMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INACTOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _INACTOUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOWTOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWTOUTENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRSTR { bits }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline]
    pub fn pinout(&self) -> PINOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINOUTR { bits }
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline]
    pub fn sdahold(&self) -> SDAHOLDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDAHOLDR { bits }
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline]
    pub fn mexttoen(&self) -> MEXTTOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEXTTOENR { bits }
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline]
    pub fn sexttoen(&self) -> SEXTTOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEXTTOENR { bits }
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPEEDR { bits }
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline]
    pub fn sclsm(&self) -> SCLSMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLSMR { bits }
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline]
    pub fn inactout(&self) -> INACTOUTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INACTOUTR { bits }
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline]
    pub fn lowtouten(&self) -> LOWTOUTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOWTOUTENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline]
    pub fn pinout(&mut self) -> _PINOUTW {
        _PINOUTW { w: self }
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline]
    pub fn sdahold(&mut self) -> _SDAHOLDW {
        _SDAHOLDW { w: self }
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline]
    pub fn mexttoen(&mut self) -> _MEXTTOENW {
        _MEXTTOENW { w: self }
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline]
    pub fn sexttoen(&mut self) -> _SEXTTOENW {
        _SEXTTOENW { w: self }
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline]
    pub fn sclsm(&mut self) -> _SCLSMW {
        _SCLSMW { w: self }
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline]
    pub fn inactout(&mut self) -> _INACTOUTW {
        _INACTOUTW { w: self }
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline]
    pub fn lowtouten(&mut self) -> _LOWTOUTENW {
        _LOWTOUTENW { w: self }
    }
}
