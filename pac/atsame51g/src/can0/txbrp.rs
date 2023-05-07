#[doc = "Register `TXBRP` reader"]
pub struct R(crate::R<TXBRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRP0` reader - Transmission Request Pending 0"]
pub type TRP0_R = crate::BitReader<bool>;
#[doc = "Field `TRP1` reader - Transmission Request Pending 1"]
pub type TRP1_R = crate::BitReader<bool>;
#[doc = "Field `TRP2` reader - Transmission Request Pending 2"]
pub type TRP2_R = crate::BitReader<bool>;
#[doc = "Field `TRP3` reader - Transmission Request Pending 3"]
pub type TRP3_R = crate::BitReader<bool>;
#[doc = "Field `TRP4` reader - Transmission Request Pending 4"]
pub type TRP4_R = crate::BitReader<bool>;
#[doc = "Field `TRP5` reader - Transmission Request Pending 5"]
pub type TRP5_R = crate::BitReader<bool>;
#[doc = "Field `TRP6` reader - Transmission Request Pending 6"]
pub type TRP6_R = crate::BitReader<bool>;
#[doc = "Field `TRP7` reader - Transmission Request Pending 7"]
pub type TRP7_R = crate::BitReader<bool>;
#[doc = "Field `TRP8` reader - Transmission Request Pending 8"]
pub type TRP8_R = crate::BitReader<bool>;
#[doc = "Field `TRP9` reader - Transmission Request Pending 9"]
pub type TRP9_R = crate::BitReader<bool>;
#[doc = "Field `TRP10` reader - Transmission Request Pending 10"]
pub type TRP10_R = crate::BitReader<bool>;
#[doc = "Field `TRP11` reader - Transmission Request Pending 11"]
pub type TRP11_R = crate::BitReader<bool>;
#[doc = "Field `TRP12` reader - Transmission Request Pending 12"]
pub type TRP12_R = crate::BitReader<bool>;
#[doc = "Field `TRP13` reader - Transmission Request Pending 13"]
pub type TRP13_R = crate::BitReader<bool>;
#[doc = "Field `TRP14` reader - Transmission Request Pending 14"]
pub type TRP14_R = crate::BitReader<bool>;
#[doc = "Field `TRP15` reader - Transmission Request Pending 15"]
pub type TRP15_R = crate::BitReader<bool>;
#[doc = "Field `TRP16` reader - Transmission Request Pending 16"]
pub type TRP16_R = crate::BitReader<bool>;
#[doc = "Field `TRP17` reader - Transmission Request Pending 17"]
pub type TRP17_R = crate::BitReader<bool>;
#[doc = "Field `TRP18` reader - Transmission Request Pending 18"]
pub type TRP18_R = crate::BitReader<bool>;
#[doc = "Field `TRP19` reader - Transmission Request Pending 19"]
pub type TRP19_R = crate::BitReader<bool>;
#[doc = "Field `TRP20` reader - Transmission Request Pending 20"]
pub type TRP20_R = crate::BitReader<bool>;
#[doc = "Field `TRP21` reader - Transmission Request Pending 21"]
pub type TRP21_R = crate::BitReader<bool>;
#[doc = "Field `TRP22` reader - Transmission Request Pending 22"]
pub type TRP22_R = crate::BitReader<bool>;
#[doc = "Field `TRP23` reader - Transmission Request Pending 23"]
pub type TRP23_R = crate::BitReader<bool>;
#[doc = "Field `TRP24` reader - Transmission Request Pending 24"]
pub type TRP24_R = crate::BitReader<bool>;
#[doc = "Field `TRP25` reader - Transmission Request Pending 25"]
pub type TRP25_R = crate::BitReader<bool>;
#[doc = "Field `TRP26` reader - Transmission Request Pending 26"]
pub type TRP26_R = crate::BitReader<bool>;
#[doc = "Field `TRP27` reader - Transmission Request Pending 27"]
pub type TRP27_R = crate::BitReader<bool>;
#[doc = "Field `TRP28` reader - Transmission Request Pending 28"]
pub type TRP28_R = crate::BitReader<bool>;
#[doc = "Field `TRP29` reader - Transmission Request Pending 29"]
pub type TRP29_R = crate::BitReader<bool>;
#[doc = "Field `TRP30` reader - Transmission Request Pending 30"]
pub type TRP30_R = crate::BitReader<bool>;
#[doc = "Field `TRP31` reader - Transmission Request Pending 31"]
pub type TRP31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmission Request Pending 0"]
    #[inline(always)]
    pub fn trp0(&self) -> TRP0_R {
        TRP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Request Pending 1"]
    #[inline(always)]
    pub fn trp1(&self) -> TRP1_R {
        TRP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Pending 2"]
    #[inline(always)]
    pub fn trp2(&self) -> TRP2_R {
        TRP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Request Pending 3"]
    #[inline(always)]
    pub fn trp3(&self) -> TRP3_R {
        TRP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Request Pending 4"]
    #[inline(always)]
    pub fn trp4(&self) -> TRP4_R {
        TRP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Request Pending 5"]
    #[inline(always)]
    pub fn trp5(&self) -> TRP5_R {
        TRP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Request Pending 6"]
    #[inline(always)]
    pub fn trp6(&self) -> TRP6_R {
        TRP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Request Pending 7"]
    #[inline(always)]
    pub fn trp7(&self) -> TRP7_R {
        TRP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Request Pending 8"]
    #[inline(always)]
    pub fn trp8(&self) -> TRP8_R {
        TRP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Request Pending 9"]
    #[inline(always)]
    pub fn trp9(&self) -> TRP9_R {
        TRP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Request Pending 10"]
    #[inline(always)]
    pub fn trp10(&self) -> TRP10_R {
        TRP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Request Pending 11"]
    #[inline(always)]
    pub fn trp11(&self) -> TRP11_R {
        TRP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmission Request Pending 12"]
    #[inline(always)]
    pub fn trp12(&self) -> TRP12_R {
        TRP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmission Request Pending 13"]
    #[inline(always)]
    pub fn trp13(&self) -> TRP13_R {
        TRP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmission Request Pending 14"]
    #[inline(always)]
    pub fn trp14(&self) -> TRP14_R {
        TRP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmission Request Pending 15"]
    #[inline(always)]
    pub fn trp15(&self) -> TRP15_R {
        TRP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Request Pending 16"]
    #[inline(always)]
    pub fn trp16(&self) -> TRP16_R {
        TRP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission Request Pending 17"]
    #[inline(always)]
    pub fn trp17(&self) -> TRP17_R {
        TRP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Request Pending 18"]
    #[inline(always)]
    pub fn trp18(&self) -> TRP18_R {
        TRP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Request Pending 19"]
    #[inline(always)]
    pub fn trp19(&self) -> TRP19_R {
        TRP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmission Request Pending 20"]
    #[inline(always)]
    pub fn trp20(&self) -> TRP20_R {
        TRP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmission Request Pending 21"]
    #[inline(always)]
    pub fn trp21(&self) -> TRP21_R {
        TRP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Request Pending 22"]
    #[inline(always)]
    pub fn trp22(&self) -> TRP22_R {
        TRP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmission Request Pending 23"]
    #[inline(always)]
    pub fn trp23(&self) -> TRP23_R {
        TRP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Request Pending 24"]
    #[inline(always)]
    pub fn trp24(&self) -> TRP24_R {
        TRP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission Request Pending 25"]
    #[inline(always)]
    pub fn trp25(&self) -> TRP25_R {
        TRP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Request Pending 26"]
    #[inline(always)]
    pub fn trp26(&self) -> TRP26_R {
        TRP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Request Pending 27"]
    #[inline(always)]
    pub fn trp27(&self) -> TRP27_R {
        TRP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Request Pending 28"]
    #[inline(always)]
    pub fn trp28(&self) -> TRP28_R {
        TRP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmission Request Pending 29"]
    #[inline(always)]
    pub fn trp29(&self) -> TRP29_R {
        TRP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmission Request Pending 30"]
    #[inline(always)]
    pub fn trp30(&self) -> TRP30_R {
        TRP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Request Pending 31"]
    #[inline(always)]
    pub fn trp31(&self) -> TRP31_R {
        TRP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx Buffer Request Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrp](index.html) module"]
pub struct TXBRP_SPEC;
impl crate::RegisterSpec for TXBRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbrp::R](R) reader structure"]
impl crate::Readable for TXBRP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TXBRP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
