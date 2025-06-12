#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `RHC` reader - Region Hash Completed"]
pub type RhcR = crate::FieldReader;
#[doc = "Field `RDM` reader - Region Digest Mismatch"]
pub type RdmR = crate::FieldReader;
#[doc = "Field `RBE` reader - Region Bus Error"]
pub type RbeR = crate::FieldReader;
#[doc = "Field `RWC` reader - Region Wrap Condition Detected"]
pub type RwcR = crate::FieldReader;
#[doc = "Field `REC` reader - Region End bit Condition Detected"]
pub type RecR = crate::FieldReader;
#[doc = "Field `RSU` reader - Region Status Updated Detected"]
pub type RsuR = crate::FieldReader;
#[doc = "Field `URAD` reader - Undefined Register Access Detection Status"]
pub type UradR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Region Hash Completed"]
    #[inline(always)]
    pub fn rhc(&self) -> RhcR {
        RhcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch"]
    #[inline(always)]
    pub fn rdm(&self) -> RdmR {
        RdmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region Bus Error"]
    #[inline(always)]
    pub fn rbe(&self) -> RbeR {
        RbeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected"]
    #[inline(always)]
    pub fn rwc(&self) -> RwcR {
        RwcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Region Status Updated Detected"]
    #[inline(always)]
    pub fn rsu(&self) -> RsuR {
        RsuR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Status"]
    #[inline(always)]
    pub fn urad(&self) -> UradR {
        UradR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
