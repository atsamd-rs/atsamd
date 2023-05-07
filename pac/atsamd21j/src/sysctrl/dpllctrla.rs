#[doc = "Register `DPLLCTRLA` reader"]
pub struct R(crate::R<DPLLCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLCTRLA` writer"]
pub struct W(crate::W<DPLLCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLCTRLA_SPEC>;
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
impl From<crate::W<DPLLCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - DPLL Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - DPLL Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPLLCTRLA_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPLLCTRLA_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Clock Activation"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Clock Activation"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPLLCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - DPLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Clock Activation"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DPLL Enable"]
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
    #[doc = "Bit 7 - On Demand Clock Activation"]
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
#[doc = "DPLL Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrla](index.html) module"]
pub struct DPLLCTRLA_SPEC;
impl crate::RegisterSpec for DPLLCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpllctrla::R](R) reader structure"]
impl crate::Readable for DPLLCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllctrla::W](W) writer structure"]
impl crate::Writable for DPLLCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLCTRLA to value 0x80"]
impl crate::Resettable for DPLLCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
