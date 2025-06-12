#[doc = "Register `BKUPEXIT` reader"]
pub type R = crate::R<BkupexitSpec>;
#[doc = "Field `RTC` reader - Real Timer Counter Interrupt"]
pub type RtcR = crate::BitReader;
#[doc = "Field `BBPS` reader - Battery Backup Power Switch"]
pub type BbpsR = crate::BitReader;
#[doc = "Field `HIB` reader - Hibernate"]
pub type HibR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Real Timer Counter Interrupt"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Battery Backup Power Switch"]
    #[inline(always)]
    pub fn bbps(&self) -> BbpsR {
        BbpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Hibernate"]
    #[inline(always)]
    pub fn hib(&self) -> HibR {
        HibR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Backup Exit Source\n\nYou can [`read`](crate::Reg::read) this register and get [`bkupexit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkupexitSpec;
impl crate::RegisterSpec for BkupexitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkupexit::R`](R) reader structure"]
impl crate::Readable for BkupexitSpec {}
#[doc = "`reset()` method sets BKUPEXIT to value 0"]
impl crate::Resettable for BkupexitSpec {}
