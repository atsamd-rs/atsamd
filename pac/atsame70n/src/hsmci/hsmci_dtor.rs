#[doc = "Register `HSMCI_DTOR` reader"]
pub struct R(crate::R<HSMCI_DTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSMCI_DTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSMCI_DTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSMCI_DTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSMCI_DTOR` writer"]
pub struct W(crate::W<HSMCI_DTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_DTOR_SPEC>;
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
impl From<crate::W<HSMCI_DTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_DTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTOCYC` reader - Data Timeout Cycle Number"]
pub struct DTOCYC_R(crate::FieldReader<u8, u8>);
impl DTOCYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTOCYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOCYC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOCYC` writer - Data Timeout Cycle Number"]
pub struct DTOCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Data Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTOMUL_A {
    #[doc = "0: DTOCYC"]
    _1 = 0,
    #[doc = "1: DTOCYC x 16"]
    _16 = 1,
    #[doc = "2: DTOCYC x 128"]
    _128 = 2,
    #[doc = "3: DTOCYC x 256"]
    _256 = 3,
    #[doc = "4: DTOCYC x 1024"]
    _1024 = 4,
    #[doc = "5: DTOCYC x 4096"]
    _4096 = 5,
    #[doc = "6: DTOCYC x 65536"]
    _65536 = 6,
    #[doc = "7: DTOCYC x 1048576"]
    _1048576 = 7,
}
impl From<DTOMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOMUL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTOMUL` reader - Data Timeout Multiplier"]
pub struct DTOMUL_R(crate::FieldReader<u8, DTOMUL_A>);
impl DTOMUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTOMUL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOMUL_A {
        match self.bits {
            0 => DTOMUL_A::_1,
            1 => DTOMUL_A::_16,
            2 => DTOMUL_A::_128,
            3 => DTOMUL_A::_256,
            4 => DTOMUL_A::_1024,
            5 => DTOMUL_A::_4096,
            6 => DTOMUL_A::_65536,
            7 => DTOMUL_A::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTOMUL_A::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == DTOMUL_A::_16
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == DTOMUL_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == DTOMUL_A::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == DTOMUL_A::_1024
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        **self == DTOMUL_A::_4096
    }
    #[doc = "Checks if the value of the field is `_65536`"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        **self == DTOMUL_A::_65536
    }
    #[doc = "Checks if the value of the field is `_1048576`"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        **self == DTOMUL_A::_1048576
    }
}
impl core::ops::Deref for DTOMUL_R {
    type Target = crate::FieldReader<u8, DTOMUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOMUL` writer - Data Timeout Multiplier"]
pub struct DTOMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOMUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOMUL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOMUL_A::_1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DTOMUL_A::_16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(DTOMUL_A::_128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(DTOMUL_A::_256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(DTOMUL_A::_1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(DTOMUL_A::_4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut W {
        self.variant(DTOMUL_A::_65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut W {
        self.variant(DTOMUL_A::_1048576)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DTOCYC_R {
        DTOCYC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DTOMUL_R {
        DTOMUL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&mut self) -> DTOCYC_W {
        DTOCYC_W { w: self }
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&mut self) -> DTOMUL_W {
        DTOMUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_dtor](index.html) module"]
pub struct HSMCI_DTOR_SPEC;
impl crate::RegisterSpec for HSMCI_DTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsmci_dtor::R](R) reader structure"]
impl crate::Readable for HSMCI_DTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsmci_dtor::W](W) writer structure"]
impl crate::Writable for HSMCI_DTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_DTOR to value 0"]
impl crate::Resettable for HSMCI_DTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
