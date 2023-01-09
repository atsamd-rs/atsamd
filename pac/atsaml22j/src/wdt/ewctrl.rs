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
#[doc = "Field `EWOFFSET` reader - Early Warning Interrupt Time Offset"]
pub type EWOFFSET_R = crate::FieldReader<u8, EWOFFSETSELECT_A>;
#[doc = "Early Warning Interrupt Time Offset\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EWOFFSETSELECT_A {
    #[doc = "0: 8 clock cycles"]
    CYC8 = 0,
    #[doc = "1: 16 clock cycles"]
    CYC16 = 1,
    #[doc = "2: 32 clock cycles"]
    CYC32 = 2,
    #[doc = "3: 64 clock cycles"]
    CYC64 = 3,
    #[doc = "4: 128 clock cycles"]
    CYC128 = 4,
    #[doc = "5: 256 clock cycles"]
    CYC256 = 5,
    #[doc = "6: 512 clock cycles"]
    CYC512 = 6,
    #[doc = "7: 1024 clock cycles"]
    CYC1024 = 7,
    #[doc = "8: 2048 clock cycles"]
    CYC2048 = 8,
    #[doc = "9: 4096 clock cycles"]
    CYC4096 = 9,
    #[doc = "10: 8192 clock cycles"]
    CYC8192 = 10,
    #[doc = "11: 16384 clock cycles"]
    CYC16384 = 11,
}
impl From<EWOFFSETSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EWOFFSETSELECT_A) -> Self {
        variant as _
    }
}
impl EWOFFSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EWOFFSETSELECT_A> {
        match self.bits {
            0 => Some(EWOFFSETSELECT_A::CYC8),
            1 => Some(EWOFFSETSELECT_A::CYC16),
            2 => Some(EWOFFSETSELECT_A::CYC32),
            3 => Some(EWOFFSETSELECT_A::CYC64),
            4 => Some(EWOFFSETSELECT_A::CYC128),
            5 => Some(EWOFFSETSELECT_A::CYC256),
            6 => Some(EWOFFSETSELECT_A::CYC512),
            7 => Some(EWOFFSETSELECT_A::CYC1024),
            8 => Some(EWOFFSETSELECT_A::CYC2048),
            9 => Some(EWOFFSETSELECT_A::CYC4096),
            10 => Some(EWOFFSETSELECT_A::CYC8192),
            11 => Some(EWOFFSETSELECT_A::CYC16384),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == EWOFFSETSELECT_A::CYC16384
    }
}
#[doc = "Field `EWOFFSET` writer - Early Warning Interrupt Time Offset"]
pub type EWOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, EWCTRL_SPEC, u8, EWOFFSETSELECT_A, 4, O>;
impl<'a, const O: u8> EWOFFSET_W<'a, O> {
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(EWOFFSETSELECT_A::CYC16384)
    }
}
impl R {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    pub fn ewoffset(&self) -> EWOFFSET_R {
        EWOFFSET_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    #[must_use]
    pub fn ewoffset(&mut self) -> EWOFFSET_W<0> {
        EWOFFSET_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EWCTRL to value 0x0b"]
impl crate::Resettable for EWCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
