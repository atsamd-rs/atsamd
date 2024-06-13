#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARAM` writer"]
pub struct W(crate::W<PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARAM_SPEC>;
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
impl From<crate::W<PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NVMP` reader - NVM Pages"]
pub struct NVMP_R(crate::FieldReader<u16, u16>);
impl NVMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        NVMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMP` writer - NVM Pages"]
pub struct NVMP_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSZ_A {
    #[doc = "0: 8 bytes"]
    _8 = 0,
    #[doc = "1: 16 bytes"]
    _16 = 1,
    #[doc = "2: 32 bytes"]
    _32 = 2,
    #[doc = "3: 64 bytes"]
    _64 = 3,
    #[doc = "4: 128 bytes"]
    _128 = 4,
    #[doc = "5: 256 bytes"]
    _256 = 5,
    #[doc = "6: 512 bytes"]
    _512 = 6,
    #[doc = "7: 1024 bytes"]
    _1024 = 7,
}
impl From<PSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: PSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSZ` reader - Page Size"]
pub struct PSZ_R(crate::FieldReader<u8, PSZ_A>);
impl PSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSZ_A {
        match self.bits {
            0 => PSZ_A::_8,
            1 => PSZ_A::_16,
            2 => PSZ_A::_32,
            3 => PSZ_A::_64,
            4 => PSZ_A::_128,
            5 => PSZ_A::_256,
            6 => PSZ_A::_512,
            7 => PSZ_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == PSZ_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == PSZ_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == PSZ_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == PSZ_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == PSZ_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == PSZ_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        **self == PSZ_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == PSZ_A::_1024
    }
}
impl core::ops::Deref for PSZ_R {
    type Target = crate::FieldReader<u8, PSZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSZ` writer - Page Size"]
pub struct PSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSZ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSZ_A::_8)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PSZ_A::_16)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PSZ_A::_32)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PSZ_A::_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(PSZ_A::_128)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PSZ_A::_256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(PSZ_A::_512)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(PSZ_A::_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `RWWEEP` reader - RWW EEPROM Pages"]
pub struct RWWEEP_R(crate::FieldReader<u16, u16>);
impl RWWEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RWWEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWWEEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWWEEP` writer - RWW EEPROM Pages"]
pub struct RWWEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWWEEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - NVM Pages"]
    #[inline(always)]
    pub fn nvmp(&self) -> NVMP_R {
        NVMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:31 - RWW EEPROM Pages"]
    #[inline(always)]
    pub fn rwweep(&self) -> RWWEEP_R {
        RWWEEP_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NVM Pages"]
    #[inline(always)]
    pub fn nvmp(&mut self) -> NVMP_W {
        NVMP_W { w: self }
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&mut self) -> PSZ_W {
        PSZ_W { w: self }
    }
    #[doc = "Bits 20:31 - RWW EEPROM Pages"]
    #[inline(always)]
    pub fn rwweep(&mut self) -> RWWEEP_W {
        RWWEEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NVM Parameter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [param::W](W) writer structure"]
impl crate::Writable for PARAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
