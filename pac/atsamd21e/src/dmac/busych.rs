#[doc = "Register `BUSYCH` reader"]
pub type R = crate::R<BUSYCH_SPEC>;
#[doc = "Field `BUSYCH0` reader - Busy Channel 0"]
pub type BUSYCH0_R = crate::BitReader;
#[doc = "Field `BUSYCH1` reader - Busy Channel 1"]
pub type BUSYCH1_R = crate::BitReader;
#[doc = "Field `BUSYCH2` reader - Busy Channel 2"]
pub type BUSYCH2_R = crate::BitReader;
#[doc = "Field `BUSYCH3` reader - Busy Channel 3"]
pub type BUSYCH3_R = crate::BitReader;
#[doc = "Field `BUSYCH4` reader - Busy Channel 4"]
pub type BUSYCH4_R = crate::BitReader;
#[doc = "Field `BUSYCH5` reader - Busy Channel 5"]
pub type BUSYCH5_R = crate::BitReader;
#[doc = "Field `BUSYCH6` reader - Busy Channel 6"]
pub type BUSYCH6_R = crate::BitReader;
#[doc = "Field `BUSYCH7` reader - Busy Channel 7"]
pub type BUSYCH7_R = crate::BitReader;
#[doc = "Field `BUSYCH8` reader - Busy Channel 8"]
pub type BUSYCH8_R = crate::BitReader;
#[doc = "Field `BUSYCH9` reader - Busy Channel 9"]
pub type BUSYCH9_R = crate::BitReader;
#[doc = "Field `BUSYCH10` reader - Busy Channel 10"]
pub type BUSYCH10_R = crate::BitReader;
#[doc = "Field `BUSYCH11` reader - Busy Channel 11"]
pub type BUSYCH11_R = crate::BitReader;
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
}
#[doc = "Busy Channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busych::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSYCH_SPEC;
impl crate::RegisterSpec for BUSYCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busych::R`](R) reader structure"]
impl crate::Readable for BUSYCH_SPEC {}
#[doc = "`reset()` method sets BUSYCH to value 0"]
impl crate::Resettable for BUSYCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
