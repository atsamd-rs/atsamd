#[doc = "Register `BKIN` reader"]
pub type R = crate::R<BkinSpec>;
#[doc = "Field `BKIN0` reader - Backup Input 0"]
pub type Bkin0R = crate::BitReader;
#[doc = "Field `BKIN1` reader - Backup Input 1"]
pub type Bkin1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Backup Input 0"]
    #[inline(always)]
    pub fn bkin0(&self) -> Bkin0R {
        Bkin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Backup Input 1"]
    #[inline(always)]
    pub fn bkin1(&self) -> Bkin1R {
        Bkin1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Backup Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkinSpec;
impl crate::RegisterSpec for BkinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkin::R`](R) reader structure"]
impl crate::Readable for BkinSpec {}
#[doc = "`reset()` method sets BKIN to value 0"]
impl crate::Resettable for BkinSpec {
    const RESET_VALUE: u32 = 0;
}
