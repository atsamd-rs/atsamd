#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RHC` reader - Region Hash Completed Interrupt Mask"]
pub type RhcR = crate::FieldReader;
#[doc = "Field `RDM` reader - Region Digest Mismatch Interrupt Mask"]
pub type RdmR = crate::FieldReader;
#[doc = "Field `RBE` reader - Region Bus Error Interrupt Mask"]
pub type RbeR = crate::FieldReader;
#[doc = "Field `RWC` reader - Region Wrap Condition Detected Interrupt Mask"]
pub type RwcR = crate::FieldReader;
#[doc = "Field `REC` reader - Region End bit Condition Detected Interrupt Mask"]
pub type RecR = crate::FieldReader;
#[doc = "Field `RSU` reader - Region Status Updated Interrupt Mask"]
pub type RsuR = crate::FieldReader;
#[doc = "Field `URAD` reader - Undefined Register Access Detection Interrupt Mask"]
pub type UradR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Mask"]
    #[inline(always)]
    pub fn rhc(&self) -> RhcR {
        RhcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn rdm(&self) -> RdmR {
        RdmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn rbe(&self) -> RbeR {
        RbeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rwc(&self) -> RwcR {
        RwcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Mask"]
    #[inline(always)]
    pub fn rsu(&self) -> RsuR {
        RsuR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> UradR {
        UradR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
