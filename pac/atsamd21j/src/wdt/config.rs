#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER` reader - Time-Out Period"]
pub type PER_R = crate::FieldReader<u8, PERSELECT_A>;
#[doc = "Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERSELECT_A {
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
impl From<PERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSELECT_A) -> Self {
        variant as _
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERSELECT_A> {
        match self.bits {
            0 => Some(PERSELECT_A::_8),
            1 => Some(PERSELECT_A::_16),
            2 => Some(PERSELECT_A::_32),
            3 => Some(PERSELECT_A::_64),
            4 => Some(PERSELECT_A::_128),
            5 => Some(PERSELECT_A::_256),
            6 => Some(PERSELECT_A::_512),
            7 => Some(PERSELECT_A::_1K),
            8 => Some(PERSELECT_A::_2K),
            9 => Some(PERSELECT_A::_4K),
            10 => Some(PERSELECT_A::_8K),
            11 => Some(PERSELECT_A::_16K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PERSELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PERSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PERSELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PERSELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PERSELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PERSELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == PERSELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == PERSELECT_A::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == PERSELECT_A::_2K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == PERSELECT_A::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == PERSELECT_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == PERSELECT_A::_16K
    }
}
#[doc = "Field `PER` writer - Time-Out Period"]
pub type PER_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CONFIG_SPEC, u8, PERSELECT_A, 4, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PERSELECT_A::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PERSELECT_A::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PERSELECT_A::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PERSELECT_A::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(PERSELECT_A::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PERSELECT_A::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(PERSELECT_A::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut W {
        self.variant(PERSELECT_A::_1K)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut W {
        self.variant(PERSELECT_A::_2K)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut W {
        self.variant(PERSELECT_A::_4K)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut W {
        self.variant(PERSELECT_A::_8K)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut W {
        self.variant(PERSELECT_A::_16K)
    }
}
#[doc = "Field `WINDOW` reader - Window Mode Time-Out Period"]
pub type WINDOW_R = crate::FieldReader<u8, WINDOWSELECT_A>;
#[doc = "Window Mode Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINDOWSELECT_A {
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
impl From<WINDOWSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINDOWSELECT_A) -> Self {
        variant as _
    }
}
impl WINDOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINDOWSELECT_A> {
        match self.bits {
            0 => Some(WINDOWSELECT_A::_8),
            1 => Some(WINDOWSELECT_A::_16),
            2 => Some(WINDOWSELECT_A::_32),
            3 => Some(WINDOWSELECT_A::_64),
            4 => Some(WINDOWSELECT_A::_128),
            5 => Some(WINDOWSELECT_A::_256),
            6 => Some(WINDOWSELECT_A::_512),
            7 => Some(WINDOWSELECT_A::_1K),
            8 => Some(WINDOWSELECT_A::_2K),
            9 => Some(WINDOWSELECT_A::_4K),
            10 => Some(WINDOWSELECT_A::_8K),
            11 => Some(WINDOWSELECT_A::_16K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == WINDOWSELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == WINDOWSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == WINDOWSELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == WINDOWSELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == WINDOWSELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == WINDOWSELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == WINDOWSELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == WINDOWSELECT_A::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == WINDOWSELECT_A::_2K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == WINDOWSELECT_A::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == WINDOWSELECT_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == WINDOWSELECT_A::_16K
    }
}
#[doc = "Field `WINDOW` writer - Window Mode Time-Out Period"]
pub type WINDOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CONFIG_SPEC, u8, WINDOWSELECT_A, 4, O>;
impl<'a, const O: u8> WINDOW_W<'a, O> {
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_1K)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_2K)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_4K)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_8K)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut W {
        self.variant(WINDOWSELECT_A::_16K)
    }
}
impl R {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<0> {
        PER_W::new(self)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WINDOW_W<4> {
        WINDOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0xbb"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0xbb;
}
