#[doc = "Register `CHINTENSET` reader"]
pub type R = crate::R<CHINTENSET_SPEC>;
#[doc = "Register `CHINTENSET` writer"]
pub type W = crate::W<CHINTENSET_SPEC>;
#[doc = "Field `OVR` reader - Channel Overrun Interrupt Enable"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - Channel Overrun Interrupt Enable"]
pub type OVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD` reader - Channel Event Detected Interrupt Enable"]
pub type EVD_R = crate::BitReader;
#[doc = "Field `EVD` writer - Channel Event Detected Interrupt Enable"]
pub type EVD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Enable"]
    #[inline(always)]
    pub fn evd(&self) -> EVD_R {
        EVD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<CHINTENSET_SPEC, 0> {
        OVR_W::new(self)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd(&mut self) -> EVD_W<CHINTENSET_SPEC, 1> {
        EVD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel n Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chintenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chintenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHINTENSET_SPEC;
impl crate::RegisterSpec for CHINTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chintenset::R`](R) reader structure"]
impl crate::Readable for CHINTENSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chintenset::W`](W) writer structure"]
impl crate::Writable for CHINTENSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHINTENSET to value 0"]
impl crate::Resettable for CHINTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
