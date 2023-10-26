#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Field `RHC` reader - Region Hash Completed Interrupt Mask"]
pub type RHC_R = crate::FieldReader;
#[doc = "Field `RDM` reader - Region Digest Mismatch Interrupt Mask"]
pub type RDM_R = crate::FieldReader;
#[doc = "Field `RBE` reader - Region Bus Error Interrupt Mask"]
pub type RBE_R = crate::FieldReader;
#[doc = "Field `RWC` reader - Region Wrap Condition Detected Interrupt Mask"]
pub type RWC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Region End bit Condition Detected Interrupt Mask"]
pub type REC_R = crate::FieldReader;
#[doc = "Field `RSU` reader - Region Status Updated Interrupt Mask"]
pub type RSU_R = crate::FieldReader;
#[doc = "Field `URAD` reader - Undefined Register Access Detection Interrupt Mask"]
pub type URAD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Mask"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn rdm(&self) -> RDM_R {
        RDM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn rbe(&self) -> RBE_R {
        RBE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Mask"]
    #[inline(always)]
    pub fn rsu(&self) -> RSU_R {
        RSU_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
