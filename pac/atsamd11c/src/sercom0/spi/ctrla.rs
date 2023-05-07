#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: USART mode with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART mode with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI mode with external clock"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI mode with internal clock"]
    SPI_MASTER = 3,
    #[doc = "4: I2C mode with external clock"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C mode with internal clock"]
    I2C_MASTER = 5,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::USART_EXT_CLK),
            1 => Some(MODESELECT_A::USART_INT_CLK),
            2 => Some(MODESELECT_A::SPI_SLAVE),
            3 => Some(MODESELECT_A::SPI_MASTER),
            4 => Some(MODESELECT_A::I2C_SLAVE),
            5 => Some(MODESELECT_A::I2C_MASTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODESELECT_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODESELECT_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODESELECT_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODESELECT_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODESELECT_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODESELECT_A::I2C_MASTER
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, MODESELECT_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "USART mode with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_EXT_CLK)
    }
    #[doc = "USART mode with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_INT_CLK)
    }
    #[doc = "SPI mode with external clock"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
    #[doc = "SPI mode with internal clock"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "I2C mode with external clock"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_SLAVE)
    }
    #[doc = "I2C mode with internal clock"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IBON_R = crate::BitReader<bool>;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IBON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `DOPO` reader - Data Out Pinout"]
pub type DOPO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOPO` writer - Data Out Pinout"]
pub type DOPO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIPO` reader - Data In Pinout"]
pub type DIPO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIPO` writer - Data In Pinout"]
pub type DIPO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 2, O>;
#[doc = "Field `FORM` reader - Frame Format"]
pub type FORM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORM` writer - Frame Format"]
pub type FORM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `DORD` reader - Data Order"]
pub type DORD_R = crate::BitReader<bool>;
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&self) -> DOPO_R {
        DOPO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&self) -> DIPO_R {
        DIPO_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    #[must_use]
    pub fn ibon(&mut self) -> IBON_W<8> {
        IBON_W::new(self)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dopo(&mut self) -> DOPO_W<16> {
        DOPO_W::new(self)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dipo(&mut self) -> DIPO_W<20> {
        DIPO_W::new(self)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FORM_W<24> {
        FORM_W::new(self)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<28> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<29> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<30> {
        DORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
