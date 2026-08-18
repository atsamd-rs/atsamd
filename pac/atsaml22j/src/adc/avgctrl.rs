#[doc = "Register `AVGCTRL` reader"]
pub struct R(crate::R<AVGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AVGCTRL` writer"]
pub struct W(crate::W<AVGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AVGCTRL_SPEC>;
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
impl From<crate::W<AVGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AVGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLENUM` reader - Number of Samples to be Collected"]
pub type SAMPLENUM_R = crate::FieldReader<u8, SAMPLENUMSELECT_A>;
#[doc = "Number of Samples to be Collected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPLENUMSELECT_A {
    #[doc = "0: 1 sample"]
    _1 = 0,
    #[doc = "1: 2 samples"]
    _2 = 1,
    #[doc = "2: 4 samples"]
    _4 = 2,
    #[doc = "3: 8 samples"]
    _8 = 3,
    #[doc = "4: 16 samples"]
    _16 = 4,
    #[doc = "5: 32 samples"]
    _32 = 5,
    #[doc = "6: 64 samples"]
    _64 = 6,
    #[doc = "7: 128 samples"]
    _128 = 7,
    #[doc = "8: 256 samples"]
    _256 = 8,
    #[doc = "9: 512 samples"]
    _512 = 9,
    #[doc = "10: 1024 samples"]
    _1024 = 10,
}
impl From<SAMPLENUMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLENUMSELECT_A) -> Self {
        variant as _
    }
}
impl SAMPLENUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAMPLENUMSELECT_A> {
        match self.bits {
            0 => Some(SAMPLENUMSELECT_A::_1),
            1 => Some(SAMPLENUMSELECT_A::_2),
            2 => Some(SAMPLENUMSELECT_A::_4),
            3 => Some(SAMPLENUMSELECT_A::_8),
            4 => Some(SAMPLENUMSELECT_A::_16),
            5 => Some(SAMPLENUMSELECT_A::_32),
            6 => Some(SAMPLENUMSELECT_A::_64),
            7 => Some(SAMPLENUMSELECT_A::_128),
            8 => Some(SAMPLENUMSELECT_A::_256),
            9 => Some(SAMPLENUMSELECT_A::_512),
            10 => Some(SAMPLENUMSELECT_A::_1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == SAMPLENUMSELECT_A::_1024
    }
}
#[doc = "Field `SAMPLENUM` writer - Number of Samples to be Collected"]
pub type SAMPLENUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, AVGCTRL_SPEC, u8, SAMPLENUMSELECT_A, 4, O>;
impl<'a, const O: u8> SAMPLENUM_W<'a, O> {
    #[doc = "1 sample"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_1)
    }
    #[doc = "2 samples"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_2)
    }
    #[doc = "4 samples"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_4)
    }
    #[doc = "8 samples"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_8)
    }
    #[doc = "16 samples"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_16)
    }
    #[doc = "32 samples"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_32)
    }
    #[doc = "64 samples"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_64)
    }
    #[doc = "128 samples"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_128)
    }
    #[doc = "256 samples"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_256)
    }
    #[doc = "512 samples"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_512)
    }
    #[doc = "1024 samples"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SAMPLENUMSELECT_A::_1024)
    }
}
#[doc = "Field `ADJRES` reader - Adjusting Result / Division Coefficient"]
pub type ADJRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADJRES` writer - Adjusting Result / Division Coefficient"]
pub type ADJRES_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AVGCTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline(always)]
    pub fn samplenum(&self) -> SAMPLENUM_R {
        SAMPLENUM_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline(always)]
    pub fn adjres(&self) -> ADJRES_R {
        ADJRES_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline(always)]
    #[must_use]
    pub fn samplenum(&mut self) -> SAMPLENUM_W<0> {
        SAMPLENUM_W::new(self)
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn adjres(&mut self) -> ADJRES_W<4> {
        ADJRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Average Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avgctrl](index.html) module"]
pub struct AVGCTRL_SPEC;
impl crate::RegisterSpec for AVGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [avgctrl::R](R) reader structure"]
impl crate::Readable for AVGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [avgctrl::W](W) writer structure"]
impl crate::Writable for AVGCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AVGCTRL to value 0"]
impl crate::Resettable for AVGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
