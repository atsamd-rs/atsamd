#[doc = "Register `RWD` reader"]
pub struct R(crate::R<RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWD` writer"]
pub struct W(crate::W<RWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWD_SPEC>;
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
impl From<crate::W<RWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDC` reader - Watchdog Configuration"]
pub type WDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDC` writer - Watchdog Configuration"]
pub type WDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RWD_SPEC, u8, u8, 8, O>;
#[doc = "Field `WDV` reader - Watchdog Value"]
pub type WDV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDV` writer - Watchdog Value"]
pub type WDV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RWD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Watchdog Configuration"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog Value"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<0> {
        WDC_W::new(self)
    }
    #[doc = "Bits 8:15 - Watchdog Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WDV_W<8> {
        WDV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Watchdog\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwd](index.html) module"]
pub struct RWD_SPEC;
impl crate::RegisterSpec for RWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwd::R](R) reader structure"]
impl crate::Readable for RWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwd::W](W) writer structure"]
impl crate::Writable for RWD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RWD to value 0"]
impl crate::Resettable for RWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
