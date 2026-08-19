#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<ChstatusSpec>;
#[doc = "Field `RDYUSR` reader - Ready User"]
pub type RdyusrR = crate::BitReader;
#[doc = "Field `BUSYCH` reader - Busy Channel"]
pub type BusychR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Ready User"]
    #[inline(always)]
    pub fn rdyusr(&self) -> RdyusrR {
        RdyusrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Channel"]
    #[inline(always)]
    pub fn busych(&self) -> BusychR {
        BusychR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Channel n Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`reset()` method sets CHSTATUS to value 0x01"]
impl crate::Resettable for ChstatusSpec {
    const RESET_VALUE: u8 = 0x01;
}
