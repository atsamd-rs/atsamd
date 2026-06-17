#[doc = "Register `CSRCFG` reader"]
pub struct R(crate::R<CSRCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSRCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSRCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSRCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSRCFG` writer"]
pub struct W(crate::W<CSRCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSRCFG_SPEC>;
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
impl From<crate::W<CSRCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSRCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSRCFG_SPEC, bool, O>;
#[doc = "Field `FCS` reader - Frame Counter Selection"]
pub type FCS_R = crate::FieldReader<u8, FCSSELECT_A>;
#[doc = "Frame Counter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCSSELECT_A {
    #[doc = "0: Frame Counter 0"]
    FC0 = 0,
    #[doc = "1: Frame Counter 1"]
    FC1 = 1,
    #[doc = "2: Frame Counter 2"]
    FC2 = 2,
}
impl From<FCSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FCSSELECT_A) -> Self {
        variant as _
    }
}
impl FCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FCSSELECT_A> {
        match self.bits {
            0 => Some(FCSSELECT_A::FC0),
            1 => Some(FCSSELECT_A::FC1),
            2 => Some(FCSSELECT_A::FC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FC0`"]
    #[inline(always)]
    pub fn is_fc0(&self) -> bool {
        *self == FCSSELECT_A::FC0
    }
    #[doc = "Checks if the value of the field is `FC1`"]
    #[inline(always)]
    pub fn is_fc1(&self) -> bool {
        *self == FCSSELECT_A::FC1
    }
    #[doc = "Checks if the value of the field is `FC2`"]
    #[inline(always)]
    pub fn is_fc2(&self) -> bool {
        *self == FCSSELECT_A::FC2
    }
}
#[doc = "Field `FCS` writer - Frame Counter Selection"]
pub type FCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSRCFG_SPEC, u8, FCSSELECT_A, 2, O>;
impl<'a, const O: u8> FCS_W<'a, O> {
    #[doc = "Frame Counter 0"]
    #[inline(always)]
    pub fn fc0(self) -> &'a mut W {
        self.variant(FCSSELECT_A::FC0)
    }
    #[doc = "Frame Counter 1"]
    #[inline(always)]
    pub fn fc1(self) -> &'a mut W {
        self.variant(FCSSELECT_A::FC1)
    }
    #[doc = "Frame Counter 2"]
    #[inline(always)]
    pub fn fc2(self) -> &'a mut W {
        self.variant(FCSSELECT_A::FC2)
    }
}
#[doc = "Field `SIZE` reader - Circular Shift Register Size"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - Circular Shift Register Size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSRCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA` reader - Circular Shift Register Value"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Circular Shift Register Value"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSRCFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Circular Shift Register Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Circular Shift Register Value"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<1> {
        FCS_W::new(self)
    }
    #[doc = "Bits 4:7 - Circular Shift Register Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<4> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 8:23 - Circular Shift Register Value"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<8> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Circular Shift Register Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrcfg](index.html) module"]
pub struct CSRCFG_SPEC;
impl crate::RegisterSpec for CSRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csrcfg::R](R) reader structure"]
impl crate::Readable for CSRCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csrcfg::W](W) writer structure"]
impl crate::Writable for CSRCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSRCFG to value 0"]
impl crate::Resettable for CSRCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
