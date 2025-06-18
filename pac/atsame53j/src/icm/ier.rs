#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RHC` writer - Region Hash Completed Interrupt Enable"]
pub type RhcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDM` writer - Region Digest Mismatch Interrupt Enable"]
pub type RdmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RBE` writer - Region Bus Error Interrupt Enable"]
pub type RbeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWC` writer - Region Wrap Condition detected Interrupt Enable"]
pub type RwcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REC` writer - Region End bit Condition Detected Interrupt Enable"]
pub type RecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSU` writer - Region Status Updated Interrupt Disable"]
pub type RsuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `URAD` writer - Undefined Register Access Detection Interrupt Enable"]
pub type UradW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Enable"]
    #[inline(always)]
    pub fn rhc(&mut self) -> RhcW<IerSpec> {
        RhcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Enable"]
    #[inline(always)]
    pub fn rdm(&mut self) -> RdmW<IerSpec> {
        RdmW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn rbe(&mut self) -> RbeW<IerSpec> {
        RbeW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition detected Interrupt Enable"]
    #[inline(always)]
    pub fn rwc(&mut self) -> RwcW<IerSpec> {
        RwcW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Enable"]
    #[inline(always)]
    pub fn rec(&mut self) -> RecW<IerSpec> {
        RecW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    pub fn rsu(&mut self) -> RsuW<IerSpec> {
        RsuW::new(self, 20)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Enable"]
    #[inline(always)]
    pub fn urad(&mut self) -> UradW<IerSpec> {
        UradW::new(self, 24)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
