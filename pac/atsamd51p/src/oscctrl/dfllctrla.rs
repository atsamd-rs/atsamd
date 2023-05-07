#[doc = "Register `DFLLCTRLA` reader"]
pub struct R(crate::R<DFLLCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLCTRLA` writer"]
pub struct W(crate::W<DFLLCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLCTRLA_SPEC>;
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
impl From<crate::W<DFLLCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - DFLL Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - DFLL Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLA_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLA_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrla](index.html) module"]
pub struct DFLLCTRLA_SPEC;
impl crate::RegisterSpec for DFLLCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dfllctrla::R](R) reader structure"]
impl crate::Readable for DFLLCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllctrla::W](W) writer structure"]
impl crate::Writable for DFLLCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLCTRLA to value 0x82"]
impl crate::Resettable for DFLLCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0x82;
}
