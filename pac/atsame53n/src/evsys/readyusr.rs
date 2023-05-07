#[doc = "Register `READYUSR` reader"]
pub struct R(crate::R<READYUSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READYUSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READYUSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READYUSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READYUSR0` reader - Ready User for Channel 0"]
pub type READYUSR0_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR1` reader - Ready User for Channel 1"]
pub type READYUSR1_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR2` reader - Ready User for Channel 2"]
pub type READYUSR2_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR3` reader - Ready User for Channel 3"]
pub type READYUSR3_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR4` reader - Ready User for Channel 4"]
pub type READYUSR4_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR5` reader - Ready User for Channel 5"]
pub type READYUSR5_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR6` reader - Ready User for Channel 6"]
pub type READYUSR6_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR7` reader - Ready User for Channel 7"]
pub type READYUSR7_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR8` reader - Ready User for Channel 8"]
pub type READYUSR8_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR9` reader - Ready User for Channel 9"]
pub type READYUSR9_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR10` reader - Ready User for Channel 10"]
pub type READYUSR10_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR11` reader - Ready User for Channel 11"]
pub type READYUSR11_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Ready User for Channel 0"]
    #[inline(always)]
    pub fn readyusr0(&self) -> READYUSR0_R {
        READYUSR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ready User for Channel 1"]
    #[inline(always)]
    pub fn readyusr1(&self) -> READYUSR1_R {
        READYUSR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ready User for Channel 2"]
    #[inline(always)]
    pub fn readyusr2(&self) -> READYUSR2_R {
        READYUSR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ready User for Channel 3"]
    #[inline(always)]
    pub fn readyusr3(&self) -> READYUSR3_R {
        READYUSR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ready User for Channel 4"]
    #[inline(always)]
    pub fn readyusr4(&self) -> READYUSR4_R {
        READYUSR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ready User for Channel 5"]
    #[inline(always)]
    pub fn readyusr5(&self) -> READYUSR5_R {
        READYUSR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ready User for Channel 6"]
    #[inline(always)]
    pub fn readyusr6(&self) -> READYUSR6_R {
        READYUSR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ready User for Channel 7"]
    #[inline(always)]
    pub fn readyusr7(&self) -> READYUSR7_R {
        READYUSR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ready User for Channel 8"]
    #[inline(always)]
    pub fn readyusr8(&self) -> READYUSR8_R {
        READYUSR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ready User for Channel 9"]
    #[inline(always)]
    pub fn readyusr9(&self) -> READYUSR9_R {
        READYUSR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ready User for Channel 10"]
    #[inline(always)]
    pub fn readyusr10(&self) -> READYUSR10_R {
        READYUSR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ready User for Channel 11"]
    #[inline(always)]
    pub fn readyusr11(&self) -> READYUSR11_R {
        READYUSR11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Ready Users\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readyusr](index.html) module"]
pub struct READYUSR_SPEC;
impl crate::RegisterSpec for READYUSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readyusr::R](R) reader structure"]
impl crate::Readable for READYUSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets READYUSR to value 0xffff_ffff"]
impl crate::Resettable for READYUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
