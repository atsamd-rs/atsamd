#[doc = "Register `EWCTRL` reader"]
pub struct R(crate::R<EWCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EWCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EWCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EWCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EWCTRL` writer"]
pub struct W(crate::W<EWCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EWCTRL_SPEC>;
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
impl From<crate::W<EWCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EWCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Early Warning Interrupt Time Offset\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EWOFFSET_A {
    #[doc = "0: 8 clock cycles"]
    _8 = 0,
    #[doc = "1: 16 clock cycles"]
    _16 = 1,
    #[doc = "2: 32 clock cycles"]
    _32 = 2,
    #[doc = "3: 64 clock cycles"]
    _64 = 3,
    #[doc = "4: 128 clock cycles"]
    _128 = 4,
    #[doc = "5: 256 clock cycles"]
    _256 = 5,
    #[doc = "6: 512 clock cycles"]
    _512 = 6,
    #[doc = "7: 1024 clock cycles"]
    _1K = 7,
    #[doc = "8: 2048 clock cycles"]
    _2K = 8,
    #[doc = "9: 4096 clock cycles"]
    _4K = 9,
    #[doc = "10: 8192 clock cycles"]
    _8K = 10,
    #[doc = "11: 16384 clock cycles"]
    _16K = 11,
}
impl From<EWOFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: EWOFFSET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EWOFFSET` reader - Early Warning Interrupt Time Offset"]
pub struct EWOFFSET_R(crate::FieldReader<u8, EWOFFSET_A>);
impl EWOFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EWOFFSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EWOFFSET_A> {
        match self.bits {
            0 => Some(EWOFFSET_A::_8),
            1 => Some(EWOFFSET_A::_16),
            2 => Some(EWOFFSET_A::_32),
            3 => Some(EWOFFSET_A::_64),
            4 => Some(EWOFFSET_A::_128),
            5 => Some(EWOFFSET_A::_256),
            6 => Some(EWOFFSET_A::_512),
            7 => Some(EWOFFSET_A::_1K),
            8 => Some(EWOFFSET_A::_2K),
            9 => Some(EWOFFSET_A::_4K),
            10 => Some(EWOFFSET_A::_8K),
            11 => Some(EWOFFSET_A::_16K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == EWOFFSET_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == EWOFFSET_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == EWOFFSET_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == EWOFFSET_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == EWOFFSET_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == EWOFFSET_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        **self == EWOFFSET_A::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        **self == EWOFFSET_A::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        **self == EWOFFSET_A::_2K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        **self == EWOFFSET_A::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        **self == EWOFFSET_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        **self == EWOFFSET_A::_16K
    }
}
impl core::ops::Deref for EWOFFSET_R {
    type Target = crate::FieldReader<u8, EWOFFSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWOFFSET` writer - Early Warning Interrupt Time Offset"]
pub struct EWOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> EWOFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWOFFSET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_1K)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_2K)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_4K)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_8K)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut W {
        self.variant(EWOFFSET_A::_16K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    pub fn ewoffset(&self) -> EWOFFSET_R {
        EWOFFSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    pub fn ewoffset(&mut self) -> EWOFFSET_W {
        EWOFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Early Warning Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ewctrl](index.html) module"]
pub struct EWCTRL_SPEC;
impl crate::RegisterSpec for EWCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ewctrl::R](R) reader structure"]
impl crate::Readable for EWCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ewctrl::W](W) writer structure"]
impl crate::Writable for EWCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EWCTRL to value 0x0b"]
impl crate::Resettable for EWCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
