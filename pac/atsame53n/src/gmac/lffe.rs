#[doc = "Register `LFFE` reader"]
pub type R = crate::R<LFFE_SPEC>;
#[doc = "Field `LFER` reader - Length Field Frame Errors"]
pub type LFER_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Length Field Frame Errors"]
    #[inline(always)]
    pub fn lfer(&self) -> LFER_R {
        LFER_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Length Field Frame Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lffe::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFFE_SPEC;
impl crate::RegisterSpec for LFFE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lffe::R`](R) reader structure"]
impl crate::Readable for LFFE_SPEC {}
#[doc = "`reset()` method sets LFFE to value 0"]
impl crate::Resettable for LFFE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
