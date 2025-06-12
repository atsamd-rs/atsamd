#[doc = "Register `LFFE` reader"]
pub type R = crate::R<LffeSpec>;
#[doc = "Field `LFER` reader - Length Field Frame Errors"]
pub type LferR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Length Field Frame Errors"]
    #[inline(always)]
    pub fn lfer(&self) -> LferR {
        LferR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Length Field Frame Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lffe::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LffeSpec;
impl crate::RegisterSpec for LffeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lffe::R`](R) reader structure"]
impl crate::Readable for LffeSpec {}
#[doc = "`reset()` method sets LFFE to value 0"]
impl crate::Resettable for LffeSpec {}
