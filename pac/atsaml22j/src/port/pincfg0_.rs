#[doc = "Register `PINCFG0_%s` reader"]
pub struct R(crate::R<PINCFG0__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINCFG0__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINCFG0__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINCFG0__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINCFG0_%s` writer"]
pub struct W(crate::W<PINCFG0__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINCFG0__SPEC>;
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
impl From<crate::W<PINCFG0__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINCFG0__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMUXEN` reader - Select Peripheral Multiplexer"]
pub type PMUXEN_R = crate::BitReader<bool>;
#[doc = "Field `PMUXEN` writer - Select Peripheral Multiplexer"]
pub type PMUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINCFG0__SPEC, bool, O>;
#[doc = "Field `INEN` reader - Input Enable"]
pub type INEN_R = crate::BitReader<bool>;
#[doc = "Field `INEN` writer - Input Enable"]
pub type INEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINCFG0__SPEC, bool, O>;
#[doc = "Field `PULLEN` reader - Pull Enable"]
pub type PULLEN_R = crate::BitReader<bool>;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PULLEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINCFG0__SPEC, bool, O>;
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub type DRVSTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINCFG0__SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select Peripheral Multiplexer"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PMUXEN_R {
        PMUXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PULLEN_R {
        PULLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Peripheral Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxen(&mut self) -> PMUXEN_W<0> {
        PMUXEN_W::new(self)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inen(&mut self) -> INEN_W<1> {
        INEN_W::new(self)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pullen(&mut self) -> PULLEN_W<2> {
        PULLEN_W::new(self)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    #[must_use]
    pub fn drvstr(&mut self) -> DRVSTR_W<6> {
        DRVSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Configuration n - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg0_](index.html) module"]
pub struct PINCFG0__SPEC;
impl crate::RegisterSpec for PINCFG0__SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pincfg0_::R](R) reader structure"]
impl crate::Readable for PINCFG0__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pincfg0_::W](W) writer structure"]
impl crate::Writable for PINCFG0__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINCFG0_%s to value 0"]
impl crate::Resettable for PINCFG0__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
