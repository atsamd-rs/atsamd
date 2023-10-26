#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `ENABLE` reader - DAC Enable Status"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `DATA0` reader - Data DAC 0"]
pub type DATA0_R = crate::BitReader;
#[doc = "Field `DATA1` reader - Data DAC 1"]
pub type DATA1_R = crate::BitReader;
#[doc = "Field `DATABUF0` reader - Data Buffer DAC 0"]
pub type DATABUF0_R = crate::BitReader;
#[doc = "Field `DATABUF1` reader - Data Buffer DAC 1"]
pub type DATABUF1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC Enable Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data DAC 0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data DAC 1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Buffer DAC 0"]
    #[inline(always)]
    pub fn databuf0(&self) -> DATABUF0_R {
        DATABUF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Buffer DAC 1"]
    #[inline(always)]
    pub fn databuf1(&self) -> DATABUF1_R {
        DATABUF1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
