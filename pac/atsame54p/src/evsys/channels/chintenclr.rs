#[doc = "Register `CHINTENCLR` reader"]
pub struct R(crate::R<CHINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHINTENCLR` writer"]
pub struct W(crate::W<CHINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHINTENCLR_SPEC>;
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
impl From<crate::W<CHINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR` reader - Channel Overrun Interrupt Disable"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` writer - Channel Overrun Interrupt Disable"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHINTENCLR_SPEC, bool, O>;
#[doc = "Field `EVD` reader - Channel Event Detected Interrupt Disable"]
pub type EVD_R = crate::BitReader<bool>;
#[doc = "Field `EVD` writer - Channel Event Detected Interrupt Disable"]
pub type EVD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHINTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Disable"]
    #[inline(always)]
    pub fn evd(&self) -> EVD_R {
        EVD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<0> {
        OVR_W::new(self)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn evd(&mut self) -> EVD_W<1> {
        EVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintenclr](index.html) module"]
pub struct CHINTENCLR_SPEC;
impl crate::RegisterSpec for CHINTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chintenclr::R](R) reader structure"]
impl crate::Readable for CHINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chintenclr::W](W) writer structure"]
impl crate::Writable for CHINTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHINTENCLR to value 0"]
impl crate::Resettable for CHINTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
