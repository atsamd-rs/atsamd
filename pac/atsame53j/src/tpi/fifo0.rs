#[doc = "Register `FIFO0` reader"]
pub type R = crate::R<Fifo0Spec>;
#[doc = "Field `ETM0` reader - "]
pub type Etm0R = crate::FieldReader;
#[doc = "Field `ETM1` reader - "]
pub type Etm1R = crate::FieldReader;
#[doc = "Field `ETM2` reader - "]
pub type Etm2R = crate::FieldReader;
#[doc = "Field `ETM_bytecount` reader - "]
pub type EtmBytecountR = crate::FieldReader;
#[doc = "Field `ETM_ATVALID` reader - "]
pub type EtmAtvalidR = crate::BitReader;
#[doc = "Field `ITM_bytecount` reader - "]
pub type ItmBytecountR = crate::FieldReader;
#[doc = "Field `ITM_ATVALID` reader - "]
pub type ItmAtvalidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn etm0(&self) -> Etm0R {
        Etm0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn etm1(&self) -> Etm1R {
        Etm1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn etm2(&self) -> Etm2R {
        Etm2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn etm_bytecount(&self) -> EtmBytecountR {
        EtmBytecountR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn etm_atvalid(&self) -> EtmAtvalidR {
        EtmAtvalidR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn itm_bytecount(&self) -> ItmBytecountR {
        ItmBytecountR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn itm_atvalid(&self) -> ItmAtvalidR {
        ItmAtvalidR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Integration ETM Data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo0Spec;
impl crate::RegisterSpec for Fifo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo0::R`](R) reader structure"]
impl crate::Readable for Fifo0Spec {}
#[doc = "`reset()` method sets FIFO0 to value 0"]
impl crate::Resettable for Fifo0Spec {
    const RESET_VALUE: u32 = 0;
}
