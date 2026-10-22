#[doc = "Register `ACMCFG` reader"]
pub struct R(crate::R<ACMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMCFG` writer"]
pub struct W(crate::W<ACMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMCFG_SPEC>;
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
impl From<crate::W<ACMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCOM` reader - COM Lines per Row"]
pub type NCOM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NCOM` writer - COM Lines per Row"]
pub type NCOM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `NDIG` reader - Number of Digit"]
pub type NDIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDIG` writer - Number of Digit"]
pub type NDIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `STEPS` reader - Scrolling Steps"]
pub type STEPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEPS` writer - Scrolling Steps"]
pub type STEPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `NDROW` reader - Number of Digit per Row"]
pub type NDROW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDROW` writer - Number of Digit per Row"]
pub type NDROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `MODE` reader - Mode"]
pub type MODE_R = crate::BitReader<MODESELECT_A>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODESELECT_A {
    #[doc = "0: Sequential Display Mode"]
    SEQ = 0,
    #[doc = "1: Scrolling Display Mode"]
    SCROLL = 1,
}
impl From<MODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODESELECT_A {
        match self.bits {
            false => MODESELECT_A::SEQ,
            true => MODESELECT_A::SCROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SEQ`"]
    #[inline(always)]
    pub fn is_seq(&self) -> bool {
        *self == MODESELECT_A::SEQ
    }
    #[doc = "Checks if the value of the field is `SCROLL`"]
    #[inline(always)]
    pub fn is_scroll(&self) -> bool {
        *self == MODESELECT_A::SCROLL
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMCFG_SPEC, MODESELECT_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Sequential Display Mode"]
    #[inline(always)]
    pub fn seq(self) -> &'a mut W {
        self.variant(MODESELECT_A::SEQ)
    }
    #[doc = "Scrolling Display Mode"]
    #[inline(always)]
    pub fn scroll(self) -> &'a mut W {
        self.variant(MODESELECT_A::SCROLL)
    }
}
#[doc = "Field `STSEG` reader - Start SEG Line"]
pub type STSEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STSEG` writer - Start SEG Line"]
pub type STSEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 6, O>;
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
pub type FCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, FCSSELECT_A, 2, O>;
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
impl R {
    #[doc = "Bits 0:2 - COM Lines per Row"]
    #[inline(always)]
    pub fn ncom(&self) -> NCOM_R {
        NCOM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Number of Digit"]
    #[inline(always)]
    pub fn ndig(&self) -> NDIG_R {
        NDIG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Scrolling Steps"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - Number of Digit per Row"]
    #[inline(always)]
    pub fn ndrow(&self) -> NDROW_R {
        NDROW_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Start SEG Line"]
    #[inline(always)]
    pub fn stseg(&self) -> STSEG_R {
        STSEG_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - COM Lines per Row"]
    #[inline(always)]
    #[must_use]
    pub fn ncom(&mut self) -> NCOM_W<0> {
        NCOM_W::new(self)
    }
    #[doc = "Bits 4:7 - Number of Digit"]
    #[inline(always)]
    #[must_use]
    pub fn ndig(&mut self) -> NDIG_W<4> {
        NDIG_W::new(self)
    }
    #[doc = "Bits 8:15 - Scrolling Steps"]
    #[inline(always)]
    #[must_use]
    pub fn steps(&mut self) -> STEPS_W<8> {
        STEPS_W::new(self)
    }
    #[doc = "Bits 16:21 - Number of Digit per Row"]
    #[inline(always)]
    #[must_use]
    pub fn ndrow(&mut self) -> NDROW_W<16> {
        NDROW_W::new(self)
    }
    #[doc = "Bit 23 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<23> {
        MODE_W::new(self)
    }
    #[doc = "Bits 24:29 - Start SEG Line"]
    #[inline(always)]
    #[must_use]
    pub fn stseg(&mut self) -> STSEG_W<24> {
        STSEG_W::new(self)
    }
    #[doc = "Bits 30:31 - Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<30> {
        FCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automated Character Mapping Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmcfg](index.html) module"]
pub struct ACMCFG_SPEC;
impl crate::RegisterSpec for ACMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmcfg::R](R) reader structure"]
impl crate::Readable for ACMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmcfg::W](W) writer structure"]
impl crate::Writable for ACMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMCFG to value 0"]
impl crate::Resettable for ACMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
