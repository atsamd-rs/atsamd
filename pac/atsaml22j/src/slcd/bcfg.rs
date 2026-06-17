#[doc = "Register `BCFG` reader"]
pub struct R(crate::R<BCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCFG` writer"]
pub struct W(crate::W<BCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCFG_SPEC>;
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
impl From<crate::W<BCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Blinking Mode"]
pub type MODE_R = crate::BitReader<MODESELECT_A>;
#[doc = "Blinking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODESELECT_A {
    #[doc = "0: Blink all segments"]
    BLINKALL = 0,
    #[doc = "1: Blink selected segments"]
    BLINKSEL = 1,
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
            false => MODESELECT_A::BLINKALL,
            true => MODESELECT_A::BLINKSEL,
        }
    }
    #[doc = "Checks if the value of the field is `BLINKALL`"]
    #[inline(always)]
    pub fn is_blinkall(&self) -> bool {
        *self == MODESELECT_A::BLINKALL
    }
    #[doc = "Checks if the value of the field is `BLINKSEL`"]
    #[inline(always)]
    pub fn is_blinksel(&self) -> bool {
        *self == MODESELECT_A::BLINKSEL
    }
}
#[doc = "Field `MODE` writer - Blinking Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCFG_SPEC, MODESELECT_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Blink all segments"]
    #[inline(always)]
    pub fn blinkall(self) -> &'a mut W {
        self.variant(MODESELECT_A::BLINKALL)
    }
    #[doc = "Blink selected segments"]
    #[inline(always)]
    pub fn blinksel(self) -> &'a mut W {
        self.variant(MODESELECT_A::BLINKSEL)
    }
}
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
pub type FCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCFG_SPEC, u8, FCSSELECT_A, 2, O>;
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
#[doc = "Field `BSS0` reader - Blink Segment Selection 0"]
pub type BSS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSS0` writer - Blink Segment Selection 0"]
pub type BSS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `BSS1` reader - Blink Segment Selection 1"]
pub type BSS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSS1` writer - Blink Segment Selection 1"]
pub type BSS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Blink Segment Selection 0"]
    #[inline(always)]
    pub fn bss0(&self) -> BSS0_R {
        BSS0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Blink Segment Selection 1"]
    #[inline(always)]
    pub fn bss1(&self) -> BSS1_R {
        BSS1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<1> {
        FCS_W::new(self)
    }
    #[doc = "Bits 8:15 - Blink Segment Selection 0"]
    #[inline(always)]
    #[must_use]
    pub fn bss0(&mut self) -> BSS0_W<8> {
        BSS0_W::new(self)
    }
    #[doc = "Bits 16:23 - Blink Segment Selection 1"]
    #[inline(always)]
    #[must_use]
    pub fn bss1(&mut self) -> BSS1_W<16> {
        BSS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Blink Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfg](index.html) module"]
pub struct BCFG_SPEC;
impl crate::RegisterSpec for BCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcfg::R](R) reader structure"]
impl crate::Readable for BCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcfg::W](W) writer structure"]
impl crate::Writable for BCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCFG to value 0"]
impl crate::Resettable for BCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
