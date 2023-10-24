#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `RHC` writer - Region Hash Completed Interrupt Disable"]
pub type RHC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RDM` writer - Region Digest Mismatch Interrupt Disable"]
pub type RDM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RBE` writer - Region Bus Error Interrupt Disable"]
pub type RBE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RWC` writer - Region Wrap Condition Detected Interrupt Disable"]
pub type RWC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `REC` writer - Region End bit Condition detected Interrupt Disable"]
pub type REC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RSU` writer - Region Status Updated Interrupt Disable"]
pub type RSU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `URAD` writer - Undefined Register Access Detection Interrupt Disable"]
pub type URAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rhc(&mut self) -> RHC_W<IDR_SPEC, 0> {
        RHC_W::new(self)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rdm(&mut self) -> RDM_W<IDR_SPEC, 4> {
        RDM_W::new(self)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rbe(&mut self) -> RBE_W<IDR_SPEC, 8> {
        RBE_W::new(self)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rwc(&mut self) -> RWC_W<IDR_SPEC, 12> {
        RWC_W::new(self)
    }
    #[doc = "Bits 16:19 - Region End bit Condition detected Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<IDR_SPEC, 16> {
        REC_W::new(self)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rsu(&mut self) -> RSU_W<IDR_SPEC, 20> {
        RSU_W::new(self)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn urad(&mut self) -> URAD_W<IDR_SPEC, 24> {
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
#[doc = "Interrupt Disable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
