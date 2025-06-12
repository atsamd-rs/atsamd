#[doc = "Register `ICTR` reader"]
pub type R = crate::R<IctrSpec>;
#[doc = "Field `INTLINESNUM` reader - "]
pub type IntlinesnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> IntlinesnumR {
        IntlinesnumR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Interrupt Controller Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ictr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IctrSpec;
impl crate::RegisterSpec for IctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ictr::R`](R) reader structure"]
impl crate::Readable for IctrSpec {}
#[doc = "`reset()` method sets ICTR to value 0"]
impl crate::Resettable for IctrSpec {}
