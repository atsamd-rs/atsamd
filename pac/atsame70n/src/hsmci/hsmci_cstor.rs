#[doc = "Register `HSMCI_CSTOR` reader"]
pub struct R(crate::R<HSMCI_CSTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSMCI_CSTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSMCI_CSTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSMCI_CSTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSMCI_CSTOR` writer"]
pub struct W(crate::W<HSMCI_CSTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_CSTOR_SPEC>;
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
impl From<crate::W<HSMCI_CSTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_CSTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSTOCYC` reader - Completion Signal Timeout Cycle Number"]
pub struct CSTOCYC_R(crate::FieldReader<u8, u8>);
impl CSTOCYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSTOCYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTOCYC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTOCYC` writer - Completion Signal Timeout Cycle Number"]
pub struct CSTOCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTOCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Completion Signal Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSTOMUL_A {
    #[doc = "0: CSTOCYC x 1"]
    _1 = 0,
    #[doc = "1: CSTOCYC x 16"]
    _16 = 1,
    #[doc = "2: CSTOCYC x 128"]
    _128 = 2,
    #[doc = "3: CSTOCYC x 256"]
    _256 = 3,
    #[doc = "4: CSTOCYC x 1024"]
    _1024 = 4,
    #[doc = "5: CSTOCYC x 4096"]
    _4096 = 5,
    #[doc = "6: CSTOCYC x 65536"]
    _65536 = 6,
    #[doc = "7: CSTOCYC x 1048576"]
    _1048576 = 7,
}
impl From<CSTOMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSTOMUL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSTOMUL` reader - Completion Signal Timeout Multiplier"]
pub struct CSTOMUL_R(crate::FieldReader<u8, CSTOMUL_A>);
impl CSTOMUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSTOMUL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOMUL_A {
        match self.bits {
            0 => CSTOMUL_A::_1,
            1 => CSTOMUL_A::_16,
            2 => CSTOMUL_A::_128,
            3 => CSTOMUL_A::_256,
            4 => CSTOMUL_A::_1024,
            5 => CSTOMUL_A::_4096,
            6 => CSTOMUL_A::_65536,
            7 => CSTOMUL_A::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CSTOMUL_A::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == CSTOMUL_A::_16
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == CSTOMUL_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == CSTOMUL_A::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == CSTOMUL_A::_1024
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        **self == CSTOMUL_A::_4096
    }
    #[doc = "Checks if the value of the field is `_65536`"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        **self == CSTOMUL_A::_65536
    }
    #[doc = "Checks if the value of the field is `_1048576`"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        **self == CSTOMUL_A::_1048576
    }
}
impl core::ops::Deref for CSTOMUL_R {
    type Target = crate::FieldReader<u8, CSTOMUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTOMUL` writer - Completion Signal Timeout Multiplier"]
pub struct CSTOMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTOMUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSTOMUL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSTOCYC x 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_1)
    }
    #[doc = "CSTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_16)
    }
    #[doc = "CSTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_128)
    }
    #[doc = "CSTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_256)
    }
    #[doc = "CSTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_1024)
    }
    #[doc = "CSTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_4096)
    }
    #[doc = "CSTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_65536)
    }
    #[doc = "CSTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_1048576)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&self) -> CSTOCYC_R {
        CSTOCYC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&self) -> CSTOMUL_R {
        CSTOMUL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&mut self) -> CSTOCYC_W {
        CSTOCYC_W { w: self }
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&mut self) -> CSTOMUL_W {
        CSTOMUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Completion Signal Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_cstor](index.html) module"]
pub struct HSMCI_CSTOR_SPEC;
impl crate::RegisterSpec for HSMCI_CSTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsmci_cstor::R](R) reader structure"]
impl crate::Readable for HSMCI_CSTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsmci_cstor::W](W) writer structure"]
impl crate::Writable for HSMCI_CSTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_CSTOR to value 0"]
impl crate::Resettable for HSMCI_CSTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
