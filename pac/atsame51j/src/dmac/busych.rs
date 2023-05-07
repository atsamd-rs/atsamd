#[doc = "Register `BUSYCH` reader"]
pub struct R(crate::R<BUSYCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSYCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSYCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSYCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSYCH0` reader - Busy Channel 0"]
pub type BUSYCH0_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH1` reader - Busy Channel 1"]
pub type BUSYCH1_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH2` reader - Busy Channel 2"]
pub type BUSYCH2_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH3` reader - Busy Channel 3"]
pub type BUSYCH3_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH4` reader - Busy Channel 4"]
pub type BUSYCH4_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH5` reader - Busy Channel 5"]
pub type BUSYCH5_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH6` reader - Busy Channel 6"]
pub type BUSYCH6_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH7` reader - Busy Channel 7"]
pub type BUSYCH7_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH8` reader - Busy Channel 8"]
pub type BUSYCH8_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH9` reader - Busy Channel 9"]
pub type BUSYCH9_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH10` reader - Busy Channel 10"]
pub type BUSYCH10_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH11` reader - Busy Channel 11"]
pub type BUSYCH11_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH12` reader - Busy Channel 12"]
pub type BUSYCH12_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH13` reader - Busy Channel 13"]
pub type BUSYCH13_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH14` reader - Busy Channel 14"]
pub type BUSYCH14_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH15` reader - Busy Channel 15"]
pub type BUSYCH15_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH16` reader - Busy Channel 16"]
pub type BUSYCH16_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH17` reader - Busy Channel 17"]
pub type BUSYCH17_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH18` reader - Busy Channel 18"]
pub type BUSYCH18_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH19` reader - Busy Channel 19"]
pub type BUSYCH19_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH20` reader - Busy Channel 20"]
pub type BUSYCH20_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH21` reader - Busy Channel 21"]
pub type BUSYCH21_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH22` reader - Busy Channel 22"]
pub type BUSYCH22_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH23` reader - Busy Channel 23"]
pub type BUSYCH23_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH24` reader - Busy Channel 24"]
pub type BUSYCH24_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH25` reader - Busy Channel 25"]
pub type BUSYCH25_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH26` reader - Busy Channel 26"]
pub type BUSYCH26_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH27` reader - Busy Channel 27"]
pub type BUSYCH27_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH28` reader - Busy Channel 28"]
pub type BUSYCH28_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH29` reader - Busy Channel 29"]
pub type BUSYCH29_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH30` reader - Busy Channel 30"]
pub type BUSYCH30_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCH31` reader - Busy Channel 31"]
pub type BUSYCH31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> BUSYCH0_R {
        BUSYCH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> BUSYCH1_R {
        BUSYCH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Channel 2"]
    #[inline(always)]
    pub fn busych2(&self) -> BUSYCH2_R {
        BUSYCH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy Channel 3"]
    #[inline(always)]
    pub fn busych3(&self) -> BUSYCH3_R {
        BUSYCH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy Channel 4"]
    #[inline(always)]
    pub fn busych4(&self) -> BUSYCH4_R {
        BUSYCH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Busy Channel 5"]
    #[inline(always)]
    pub fn busych5(&self) -> BUSYCH5_R {
        BUSYCH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Busy Channel 6"]
    #[inline(always)]
    pub fn busych6(&self) -> BUSYCH6_R {
        BUSYCH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy Channel 7"]
    #[inline(always)]
    pub fn busych7(&self) -> BUSYCH7_R {
        BUSYCH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Busy Channel 8"]
    #[inline(always)]
    pub fn busych8(&self) -> BUSYCH8_R {
        BUSYCH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Busy Channel 9"]
    #[inline(always)]
    pub fn busych9(&self) -> BUSYCH9_R {
        BUSYCH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Busy Channel 10"]
    #[inline(always)]
    pub fn busych10(&self) -> BUSYCH10_R {
        BUSYCH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Busy Channel 11"]
    #[inline(always)]
    pub fn busych11(&self) -> BUSYCH11_R {
        BUSYCH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Busy Channel 12"]
    #[inline(always)]
    pub fn busych12(&self) -> BUSYCH12_R {
        BUSYCH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Busy Channel 13"]
    #[inline(always)]
    pub fn busych13(&self) -> BUSYCH13_R {
        BUSYCH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Busy Channel 14"]
    #[inline(always)]
    pub fn busych14(&self) -> BUSYCH14_R {
        BUSYCH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy Channel 15"]
    #[inline(always)]
    pub fn busych15(&self) -> BUSYCH15_R {
        BUSYCH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy Channel 16"]
    #[inline(always)]
    pub fn busych16(&self) -> BUSYCH16_R {
        BUSYCH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Busy Channel 17"]
    #[inline(always)]
    pub fn busych17(&self) -> BUSYCH17_R {
        BUSYCH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Busy Channel 18"]
    #[inline(always)]
    pub fn busych18(&self) -> BUSYCH18_R {
        BUSYCH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Busy Channel 19"]
    #[inline(always)]
    pub fn busych19(&self) -> BUSYCH19_R {
        BUSYCH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Busy Channel 20"]
    #[inline(always)]
    pub fn busych20(&self) -> BUSYCH20_R {
        BUSYCH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Busy Channel 21"]
    #[inline(always)]
    pub fn busych21(&self) -> BUSYCH21_R {
        BUSYCH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Busy Channel 22"]
    #[inline(always)]
    pub fn busych22(&self) -> BUSYCH22_R {
        BUSYCH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Busy Channel 23"]
    #[inline(always)]
    pub fn busych23(&self) -> BUSYCH23_R {
        BUSYCH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Busy Channel 24"]
    #[inline(always)]
    pub fn busych24(&self) -> BUSYCH24_R {
        BUSYCH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Busy Channel 25"]
    #[inline(always)]
    pub fn busych25(&self) -> BUSYCH25_R {
        BUSYCH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Busy Channel 26"]
    #[inline(always)]
    pub fn busych26(&self) -> BUSYCH26_R {
        BUSYCH26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Busy Channel 27"]
    #[inline(always)]
    pub fn busych27(&self) -> BUSYCH27_R {
        BUSYCH27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Busy Channel 28"]
    #[inline(always)]
    pub fn busych28(&self) -> BUSYCH28_R {
        BUSYCH28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Busy Channel 29"]
    #[inline(always)]
    pub fn busych29(&self) -> BUSYCH29_R {
        BUSYCH29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Busy Channel 30"]
    #[inline(always)]
    pub fn busych30(&self) -> BUSYCH30_R {
        BUSYCH30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Busy Channel 31"]
    #[inline(always)]
    pub fn busych31(&self) -> BUSYCH31_R {
        BUSYCH31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Busy Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busych](index.html) module"]
pub struct BUSYCH_SPEC;
impl crate::RegisterSpec for BUSYCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busych::R](R) reader structure"]
impl crate::Readable for BUSYCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSYCH to value 0"]
impl crate::Resettable for BUSYCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
