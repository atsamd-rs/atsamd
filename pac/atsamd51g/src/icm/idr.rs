#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `RHC` writer - Region Hash Completed Interrupt Disable"]
pub type RhcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDM` writer - Region Digest Mismatch Interrupt Disable"]
pub type RdmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RBE` writer - Region Bus Error Interrupt Disable"]
pub type RbeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWC` writer - Region Wrap Condition Detected Interrupt Disable"]
pub type RwcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REC` writer - Region End bit Condition detected Interrupt Disable"]
pub type RecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSU` writer - Region Status Updated Interrupt Disable"]
pub type RsuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `URAD` writer - Undefined Register Access Detection Interrupt Disable"]
pub type UradW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Disable"]
    #[inline(always)]
    pub fn rhc(&mut self) -> RhcW<IdrSpec> {
        RhcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Disable"]
    #[inline(always)]
    pub fn rdm(&mut self) -> RdmW<IdrSpec> {
        RdmW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Disable"]
    #[inline(always)]
    pub fn rbe(&mut self) -> RbeW<IdrSpec> {
        RbeW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Disable"]
    #[inline(always)]
    pub fn rwc(&mut self) -> RwcW<IdrSpec> {
        RwcW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Region End bit Condition detected Interrupt Disable"]
    #[inline(always)]
    pub fn rec(&mut self) -> RecW<IdrSpec> {
        RecW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    pub fn rsu(&mut self) -> RsuW<IdrSpec> {
        RsuW::new(self, 20)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Disable"]
    #[inline(always)]
    pub fn urad(&mut self) -> UradW<IdrSpec> {
        UradW::new(self, 24)
    }
}
#[doc = "Interrupt Disable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
