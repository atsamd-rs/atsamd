#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RHC` writer - Region Hash Completed Interrupt Enable"]
pub type RHC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RDM` writer - Region Digest Mismatch Interrupt Enable"]
pub type RDM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RBE` writer - Region Bus Error Interrupt Enable"]
pub type RBE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RWC` writer - Region Wrap Condition detected Interrupt Enable"]
pub type RWC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `REC` writer - Region End bit Condition Detected Interrupt Enable"]
pub type REC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RSU` writer - Region Status Updated Interrupt Disable"]
pub type RSU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `URAD` writer - Undefined Register Access Detection Interrupt Enable"]
pub type URAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rhc(&mut self) -> RHC_W<IER_SPEC, 0> {
        RHC_W::new(self)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdm(&mut self) -> RDM_W<IER_SPEC, 4> {
        RDM_W::new(self)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbe(&mut self) -> RBE_W<IER_SPEC, 8> {
        RBE_W::new(self)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwc(&mut self) -> RWC_W<IER_SPEC, 12> {
        RWC_W::new(self)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<IER_SPEC, 16> {
        REC_W::new(self)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rsu(&mut self) -> RSU_W<IER_SPEC, 20> {
        RSU_W::new(self)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn urad(&mut self) -> URAD_W<IER_SPEC, 24> {
        URAD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
