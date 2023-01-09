#[doc = "Register `INTFLAGAHB` reader"]
pub struct R(crate::R<INTFLAGAHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGAHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGAHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGAHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGAHB` writer"]
pub struct W(crate::W<INTFLAGAHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGAHB_SPEC>;
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
impl From<crate::W<INTFLAGAHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGAHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_` reader - FLASH"]
pub type FLASH__R = crate::BitReader<bool>;
#[doc = "Field `FLASH_` writer - FLASH"]
pub type FLASH__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HSRAMCM0P_` reader - HSRAMCM0P"]
pub type HSRAMCM0P__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMCM0P_` writer - HSRAMCM0P"]
pub type HSRAMCM0P__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HSRAMDSU_` reader - HSRAMDSU"]
pub type HSRAMDSU__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMDSU_` writer - HSRAMDSU"]
pub type HSRAMDSU__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB1_` reader - HPB1"]
pub type HPB1__R = crate::BitReader<bool>;
#[doc = "Field `HPB1_` writer - HPB1"]
pub type HPB1__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB0_` reader - HPB0"]
pub type HPB0__R = crate::BitReader<bool>;
#[doc = "Field `HPB0_` writer - HPB0"]
pub type HPB0__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB2_` reader - HPB2"]
pub type HPB2__R = crate::BitReader<bool>;
#[doc = "Field `HPB2_` writer - HPB2"]
pub type HPB2__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HSRAMDMAC_` reader - HSRAMDMAC"]
pub type HSRAMDMAC__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMDMAC_` writer - HSRAMDMAC"]
pub type HSRAMDMAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> FLASH__R {
        FLASH__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&self) -> HSRAMCM0P__R {
        HSRAMCM0P__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&self) -> HSRAMDSU__R {
        HSRAMDSU__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSRAMDMAC"]
    #[inline(always)]
    pub fn hsramdmac_(&self) -> HSRAMDMAC__R {
        HSRAMDMAC__R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    #[must_use]
    pub fn flash_(&mut self) -> FLASH__W<0> {
        FLASH__W::new(self)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    #[must_use]
    pub fn hsramcm0p_(&mut self) -> HSRAMCM0P__W<1> {
        HSRAMCM0P__W::new(self)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    #[must_use]
    pub fn hsramdsu_(&mut self) -> HSRAMDSU__W<2> {
        HSRAMDSU__W::new(self)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> HPB1__W<3> {
        HPB1__W::new(self)
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> HPB0__W<4> {
        HPB0__W::new(self)
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> HPB2__W<5> {
        HPB2__W::new(self)
    }
    #[doc = "Bit 6 - HSRAMDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn hsramdmac_(&mut self) -> HSRAMDMAC__W<6> {
        HSRAMDMAC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bridge interrupt flag status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagahb](index.html) module"]
pub struct INTFLAGAHB_SPEC;
impl crate::RegisterSpec for INTFLAGAHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagahb::R](R) reader structure"]
impl crate::Readable for INTFLAGAHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagahb::W](W) writer structure"]
impl crate::Writable for INTFLAGAHB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for INTFLAGAHB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
