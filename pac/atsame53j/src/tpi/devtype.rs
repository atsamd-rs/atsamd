#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DEVTYPE_SPEC>;
#[doc = "Field `SubType` reader - "]
pub type SUB_TYPE_R = crate::FieldReader;
#[doc = "Field `MajorType` reader - "]
pub type MAJOR_TYPE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUB_TYPE_R {
        SUB_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn major_type(&self) -> MAJOR_TYPE_R {
        MAJOR_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "TPIU_DEVTYPE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {}
#[doc = "`reset()` method sets DEVTYPE to value 0"]
impl crate::Resettable for DEVTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
