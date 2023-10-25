#[doc = "Register `FIFO0` reader"]
pub type R = crate::R<FIFO0_SPEC>;
#[doc = "Field `ETM0` reader - "]
pub type ETM0_R = crate::FieldReader;
#[doc = "Field `ETM1` reader - "]
pub type ETM1_R = crate::FieldReader;
#[doc = "Field `ETM2` reader - "]
pub type ETM2_R = crate::FieldReader;
#[doc = "Field `ETM_bytecount` reader - "]
pub type ETM_BYTECOUNT_R = crate::FieldReader;
#[doc = "Field `ETM_ATVALID` reader - "]
pub type ETM_ATVALID_R = crate::BitReader;
#[doc = "Field `ITM_bytecount` reader - "]
pub type ITM_BYTECOUNT_R = crate::FieldReader;
#[doc = "Field `ITM_ATVALID` reader - "]
pub type ITM_ATVALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn etm0(&self) -> ETM0_R {
        ETM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn etm1(&self) -> ETM1_R {
        ETM1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn etm2(&self) -> ETM2_R {
        ETM2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn etm_bytecount(&self) -> ETM_BYTECOUNT_R {
        ETM_BYTECOUNT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn etm_atvalid(&self) -> ETM_ATVALID_R {
        ETM_ATVALID_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn itm_bytecount(&self) -> ITM_BYTECOUNT_R {
        ITM_BYTECOUNT_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn itm_atvalid(&self) -> ITM_ATVALID_R {
        ITM_ATVALID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Integration ETM Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO0_SPEC;
impl crate::RegisterSpec for FIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo0::R`](R) reader structure"]
impl crate::Readable for FIFO0_SPEC {}
#[doc = "`reset()` method sets FIFO0 to value 0"]
impl crate::Resettable for FIFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
