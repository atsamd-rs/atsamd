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
#[doc = "Field `PENDCH6` reader - Pending Channel 6"]
pub type PENDCH6_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH7` reader - Pending Channel 7"]
pub type PENDCH7_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH8` reader - Pending Channel 8"]
pub type PENDCH8_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH9` reader - Pending Channel 9"]
pub type PENDCH9_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH10` reader - Pending Channel 10"]
pub type PENDCH10_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH11` reader - Pending Channel 11"]
pub type PENDCH11_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH12` reader - Pending Channel 12"]
pub type PENDCH12_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH13` reader - Pending Channel 13"]
pub type PENDCH13_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH14` reader - Pending Channel 14"]
pub type PENDCH14_R = crate::BitReader<bool>;
#[doc = "Field `PENDCH15` reader - Pending Channel 15"]
pub type PENDCH15_R = crate::BitReader<bool>;
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
    #[doc = "Bit 6 - Pending Channel 6"]
    #[inline(always)]
    pub fn pendch6(&self) -> PENDCH6_R {
        PENDCH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending Channel 7"]
    #[inline(always)]
    pub fn pendch7(&self) -> PENDCH7_R {
        PENDCH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending Channel 8"]
    #[inline(always)]
    pub fn pendch8(&self) -> PENDCH8_R {
        PENDCH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending Channel 9"]
    #[inline(always)]
    pub fn pendch9(&self) -> PENDCH9_R {
        PENDCH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending Channel 10"]
    #[inline(always)]
    pub fn pendch10(&self) -> PENDCH10_R {
        PENDCH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending Channel 11"]
    #[inline(always)]
    pub fn pendch11(&self) -> PENDCH11_R {
        PENDCH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending Channel 12"]
    #[inline(always)]
    pub fn pendch12(&self) -> PENDCH12_R {
        PENDCH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending Channel 13"]
    #[inline(always)]
    pub fn pendch13(&self) -> PENDCH13_R {
        PENDCH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending Channel 14"]
    #[inline(always)]
    pub fn pendch14(&self) -> PENDCH14_R {
        PENDCH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending Channel 15"]
    #[inline(always)]
    pub fn pendch15(&self) -> PENDCH15_R {
        PENDCH15_R::new(((self.bits >> 15) & 1) != 0)
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
