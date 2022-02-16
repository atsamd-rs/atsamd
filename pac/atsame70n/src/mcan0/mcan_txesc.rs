#[doc = "Register `MCAN_TXESC` reader"]
pub struct R(crate::R<MCAN_TXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_TXESC` writer"]
pub struct W(crate::W<MCAN_TXESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_TXESC_SPEC>;
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
impl From<crate::W<MCAN_TXESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_TXESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Tx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBDS_A {
    #[doc = "0: 8-byte data field"]
    _8_BYTE = 0,
    #[doc = "1: 12-byte data field"]
    _12_BYTE = 1,
    #[doc = "2: 16-byte data field"]
    _16_BYTE = 2,
    #[doc = "3: 20-byte data field"]
    _20_BYTE = 3,
    #[doc = "4: 24-byte data field"]
    _24_BYTE = 4,
    #[doc = "5: 32-byte data field"]
    _32_BYTE = 5,
    #[doc = "6: 48-byte data field"]
    _48_BYTE = 6,
    #[doc = "7: 64-byte data field"]
    _64_BYTE = 7,
}
impl From<TBDS_A> for u8 {
    #[inline(always)]
    fn from(variant: TBDS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub struct TBDS_R(crate::FieldReader<u8, TBDS_A>);
impl TBDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TBDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBDS_A {
        match self.bits {
            0 => TBDS_A::_8_BYTE,
            1 => TBDS_A::_12_BYTE,
            2 => TBDS_A::_16_BYTE,
            3 => TBDS_A::_20_BYTE,
            4 => TBDS_A::_24_BYTE,
            5 => TBDS_A::_32_BYTE,
            6 => TBDS_A::_48_BYTE,
            7 => TBDS_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == TBDS_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_12_BYTE`"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        **self == TBDS_A::_12_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == TBDS_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_20_BYTE`"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        **self == TBDS_A::_20_BYTE
    }
    #[doc = "Checks if the value of the field is `_24_BYTE`"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        **self == TBDS_A::_24_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == TBDS_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_48_BYTE`"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        **self == TBDS_A::_48_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        **self == TBDS_A::_64_BYTE
    }
}
impl core::ops::Deref for TBDS_R {
    type Target = crate::FieldReader<u8, TBDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size"]
pub struct TBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBDS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_64_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&mut self) -> TBDS_W {
        TBDS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txesc](index.html) module"]
pub struct MCAN_TXESC_SPEC;
impl crate::RegisterSpec for MCAN_TXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_txesc::R](R) reader structure"]
impl crate::Readable for MCAN_TXESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_txesc::W](W) writer structure"]
impl crate::Writable for MCAN_TXESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_TXESC to value 0"]
impl crate::Resettable for MCAN_TXESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
