#[doc = "Register `MCAN_RXESC` reader"]
pub struct R(crate::R<MCAN_RXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_RXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_RXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_RXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_RXESC` writer"]
pub struct W(crate::W<MCAN_RXESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_RXESC_SPEC>;
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
impl From<crate::W<MCAN_RXESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_RXESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO 0 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum F0DS_A {
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
impl From<F0DS_A> for u8 {
    #[inline(always)]
    fn from(variant: F0DS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `F0DS` reader - Receive FIFO 0 Data Field Size"]
pub struct F0DS_R(crate::FieldReader<u8, F0DS_A>);
impl F0DS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0DS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F0DS_A {
        match self.bits {
            0 => F0DS_A::_8_BYTE,
            1 => F0DS_A::_12_BYTE,
            2 => F0DS_A::_16_BYTE,
            3 => F0DS_A::_20_BYTE,
            4 => F0DS_A::_24_BYTE,
            5 => F0DS_A::_32_BYTE,
            6 => F0DS_A::_48_BYTE,
            7 => F0DS_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == F0DS_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_12_BYTE`"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        **self == F0DS_A::_12_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == F0DS_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_20_BYTE`"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        **self == F0DS_A::_20_BYTE
    }
    #[doc = "Checks if the value of the field is `_24_BYTE`"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        **self == F0DS_A::_24_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == F0DS_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_48_BYTE`"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        **self == F0DS_A::_48_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        **self == F0DS_A::_64_BYTE
    }
}
impl core::ops::Deref for F0DS_R {
    type Target = crate::FieldReader<u8, F0DS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0DS` writer - Receive FIFO 0 Data Field Size"]
pub struct F0DS_W<'a> {
    w: &'a mut W,
}
impl<'a> F0DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: F0DS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(F0DS_A::_64_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Receive FIFO 1 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum F1DS_A {
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
impl From<F1DS_A> for u8 {
    #[inline(always)]
    fn from(variant: F1DS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `F1DS` reader - Receive FIFO 1 Data Field Size"]
pub struct F1DS_R(crate::FieldReader<u8, F1DS_A>);
impl F1DS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F1DS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F1DS_A {
        match self.bits {
            0 => F1DS_A::_8_BYTE,
            1 => F1DS_A::_12_BYTE,
            2 => F1DS_A::_16_BYTE,
            3 => F1DS_A::_20_BYTE,
            4 => F1DS_A::_24_BYTE,
            5 => F1DS_A::_32_BYTE,
            6 => F1DS_A::_48_BYTE,
            7 => F1DS_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == F1DS_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_12_BYTE`"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        **self == F1DS_A::_12_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == F1DS_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_20_BYTE`"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        **self == F1DS_A::_20_BYTE
    }
    #[doc = "Checks if the value of the field is `_24_BYTE`"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        **self == F1DS_A::_24_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == F1DS_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_48_BYTE`"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        **self == F1DS_A::_48_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        **self == F1DS_A::_64_BYTE
    }
}
impl core::ops::Deref for F1DS_R {
    type Target = crate::FieldReader<u8, F1DS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1DS` writer - Receive FIFO 1 Data Field Size"]
pub struct F1DS_W<'a> {
    w: &'a mut W,
}
impl<'a> F1DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: F1DS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(F1DS_A::_64_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Receive Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RBDS_A {
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
impl From<RBDS_A> for u8 {
    #[inline(always)]
    fn from(variant: RBDS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RBDS` reader - Receive Buffer Data Field Size"]
pub struct RBDS_R(crate::FieldReader<u8, RBDS_A>);
impl RBDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBDS_A {
        match self.bits {
            0 => RBDS_A::_8_BYTE,
            1 => RBDS_A::_12_BYTE,
            2 => RBDS_A::_16_BYTE,
            3 => RBDS_A::_20_BYTE,
            4 => RBDS_A::_24_BYTE,
            5 => RBDS_A::_32_BYTE,
            6 => RBDS_A::_48_BYTE,
            7 => RBDS_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == RBDS_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_12_BYTE`"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        **self == RBDS_A::_12_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == RBDS_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_20_BYTE`"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        **self == RBDS_A::_20_BYTE
    }
    #[doc = "Checks if the value of the field is `_24_BYTE`"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        **self == RBDS_A::_24_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == RBDS_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_48_BYTE`"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        **self == RBDS_A::_48_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        **self == RBDS_A::_64_BYTE
    }
}
impl core::ops::Deref for RBDS_R {
    type Target = crate::FieldReader<u8, RBDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBDS` writer - Receive Buffer Data Field Size"]
pub struct RBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBDS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(RBDS_A::_64_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Receive FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Receive Buffer Data Field Size"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&mut self) -> F0DS_W {
        F0DS_W { w: self }
    }
    #[doc = "Bits 4:6 - Receive FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&mut self) -> F1DS_W {
        F1DS_W { w: self }
    }
    #[doc = "Bits 8:10 - Receive Buffer Data Field Size"]
    #[inline(always)]
    pub fn rbds(&mut self) -> RBDS_W {
        RBDS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer / FIFO Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxesc](index.html) module"]
pub struct MCAN_RXESC_SPEC;
impl crate::RegisterSpec for MCAN_RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_rxesc::R](R) reader structure"]
impl crate::Readable for MCAN_RXESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_rxesc::W](W) writer structure"]
impl crate::Writable for MCAN_RXESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_RXESC to value 0"]
impl crate::Resettable for MCAN_RXESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
