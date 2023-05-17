#[doc = "Register `CHSTATUS` reader"]
pub struct R(crate::R<CHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDYUSR` reader - Ready User"]
pub type RDYUSR_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH` reader - Busy Channel"]
pub type BUSYCH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Ready User"]
    #[inline(always)]
    pub fn rdyusr(&self) -> RDYUSR_R {
        RDYUSR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Channel"]
    #[inline(always)]
    pub fn busych(&self) -> BUSYCH_R {
        BUSYCH_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Channel n Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus](index.html) module"]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chstatus::R](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSTATUS to value 0x01"]
impl crate::Resettable for CHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
