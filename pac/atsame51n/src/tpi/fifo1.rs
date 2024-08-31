#[doc = "Register `FIFO1` reader"]
pub type R = crate::R<Fifo1Spec>;
#[doc = "Field `ITM0` reader - "]
pub type Itm0R = crate::FieldReader;
#[doc = "Field `ITM1` reader - "]
pub type Itm1R = crate::FieldReader;
#[doc = "Field `ITM2` reader - "]
pub type Itm2R = crate::FieldReader;
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
    pub fn itm0(&self) -> Itm0R {
        Itm0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn itm1(&self) -> Itm1R {
        Itm1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn itm2(&self) -> Itm2R {
        Itm2R::new(((self.bits >> 16) & 0xff) as u8)
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
#[doc = "Integration ITM Data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo1Spec;
impl crate::RegisterSpec for Fifo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo1::R`](R) reader structure"]
impl crate::Readable for Fifo1Spec {}
#[doc = "`reset()` method sets FIFO1 to value 0"]
impl crate::Resettable for Fifo1Spec {
    const RESET_VALUE: u32 = 0;
}
