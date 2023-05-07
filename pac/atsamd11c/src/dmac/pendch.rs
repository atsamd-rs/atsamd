#[doc = "Register `PENDCH` reader"]
pub struct R(crate::R<PENDCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PENDCH0` reader - Pending Channel 0"]
pub type PENDCH0_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH1` reader - Pending Channel 1"]
pub type PENDCH1_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH2` reader - Pending Channel 2"]
pub type PENDCH2_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH3` reader - Pending Channel 3"]
pub type PENDCH3_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH4` reader - Pending Channel 4"]
pub type PENDCH4_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH5` reader - Pending Channel 5"]
pub type PENDCH5_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> PENDCH0_R {
        PENDCH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> PENDCH1_R {
        PENDCH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending Channel 2"]
    #[inline(always)]
    pub fn pendch2(&self) -> PENDCH2_R {
        PENDCH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending Channel 3"]
    #[inline(always)]
    pub fn pendch3(&self) -> PENDCH3_R {
        PENDCH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending Channel 4"]
    #[inline(always)]
    pub fn pendch4(&self) -> PENDCH4_R {
        PENDCH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending Channel 5"]
    #[inline(always)]
    pub fn pendch5(&self) -> PENDCH5_R {
        PENDCH5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Pending Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pendch](index.html) module"]
pub struct PENDCH_SPEC;
impl crate::RegisterSpec for PENDCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pendch::R](R) reader structure"]
impl crate::Readable for PENDCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PENDCH to value 0"]
impl crate::Resettable for PENDCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
