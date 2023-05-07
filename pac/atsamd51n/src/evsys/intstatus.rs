#[doc = "Register `INTSTATUS` reader"]
pub struct R(crate::R<INTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHINT0` reader - Channel 0 Pending Interrupt"]
pub type CHINT0_R = crate::BitReader<bool>;
#[doc = "Field `CHINT1` reader - Channel 1 Pending Interrupt"]
pub type CHINT1_R = crate::BitReader<bool>;
#[doc = "Field `CHINT2` reader - Channel 2 Pending Interrupt"]
pub type CHINT2_R = crate::BitReader<bool>;
#[doc = "Field `CHINT3` reader - Channel 3 Pending Interrupt"]
pub type CHINT3_R = crate::BitReader<bool>;
#[doc = "Field `CHINT4` reader - Channel 4 Pending Interrupt"]
pub type CHINT4_R = crate::BitReader<bool>;
#[doc = "Field `CHINT5` reader - Channel 5 Pending Interrupt"]
pub type CHINT5_R = crate::BitReader<bool>;
#[doc = "Field `CHINT6` reader - Channel 6 Pending Interrupt"]
pub type CHINT6_R = crate::BitReader<bool>;
#[doc = "Field `CHINT7` reader - Channel 7 Pending Interrupt"]
pub type CHINT7_R = crate::BitReader<bool>;
#[doc = "Field `CHINT8` reader - Channel 8 Pending Interrupt"]
pub type CHINT8_R = crate::BitReader<bool>;
#[doc = "Field `CHINT9` reader - Channel 9 Pending Interrupt"]
pub type CHINT9_R = crate::BitReader<bool>;
#[doc = "Field `CHINT10` reader - Channel 10 Pending Interrupt"]
pub type CHINT10_R = crate::BitReader<bool>;
#[doc = "Field `CHINT11` reader - Channel 11 Pending Interrupt"]
pub type CHINT11_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Pending Interrupt"]
    #[inline(always)]
    pub fn chint0(&self) -> CHINT0_R {
        CHINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Pending Interrupt"]
    #[inline(always)]
    pub fn chint1(&self) -> CHINT1_R {
        CHINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Pending Interrupt"]
    #[inline(always)]
    pub fn chint2(&self) -> CHINT2_R {
        CHINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Pending Interrupt"]
    #[inline(always)]
    pub fn chint3(&self) -> CHINT3_R {
        CHINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Pending Interrupt"]
    #[inline(always)]
    pub fn chint4(&self) -> CHINT4_R {
        CHINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Pending Interrupt"]
    #[inline(always)]
    pub fn chint5(&self) -> CHINT5_R {
        CHINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Pending Interrupt"]
    #[inline(always)]
    pub fn chint6(&self) -> CHINT6_R {
        CHINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Pending Interrupt"]
    #[inline(always)]
    pub fn chint7(&self) -> CHINT7_R {
        CHINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Pending Interrupt"]
    #[inline(always)]
    pub fn chint8(&self) -> CHINT8_R {
        CHINT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Pending Interrupt"]
    #[inline(always)]
    pub fn chint9(&self) -> CHINT9_R {
        CHINT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Pending Interrupt"]
    #[inline(always)]
    pub fn chint10(&self) -> CHINT10_R {
        CHINT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Pending Interrupt"]
    #[inline(always)]
    pub fn chint11(&self) -> CHINT11_R {
        CHINT11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](index.html) module"]
pub struct INTSTATUS_SPEC;
impl crate::RegisterSpec for INTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstatus::R](R) reader structure"]
impl crate::Readable for INTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for INTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
