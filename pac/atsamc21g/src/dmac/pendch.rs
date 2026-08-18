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
pub struct PENDCH0_R(crate::FieldReader<bool, bool>);
impl PENDCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH1` reader - Pending Channel 1"]
pub struct PENDCH1_R(crate::FieldReader<bool, bool>);
impl PENDCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH2` reader - Pending Channel 2"]
pub struct PENDCH2_R(crate::FieldReader<bool, bool>);
impl PENDCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH3` reader - Pending Channel 3"]
pub struct PENDCH3_R(crate::FieldReader<bool, bool>);
impl PENDCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH4` reader - Pending Channel 4"]
pub struct PENDCH4_R(crate::FieldReader<bool, bool>);
impl PENDCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH5` reader - Pending Channel 5"]
pub struct PENDCH5_R(crate::FieldReader<bool, bool>);
impl PENDCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH6` reader - Pending Channel 6"]
pub struct PENDCH6_R(crate::FieldReader<bool, bool>);
impl PENDCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH7` reader - Pending Channel 7"]
pub struct PENDCH7_R(crate::FieldReader<bool, bool>);
impl PENDCH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH8` reader - Pending Channel 8"]
pub struct PENDCH8_R(crate::FieldReader<bool, bool>);
impl PENDCH8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH9` reader - Pending Channel 9"]
pub struct PENDCH9_R(crate::FieldReader<bool, bool>);
impl PENDCH9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH10` reader - Pending Channel 10"]
pub struct PENDCH10_R(crate::FieldReader<bool, bool>);
impl PENDCH10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH11` reader - Pending Channel 11"]
pub struct PENDCH11_R(crate::FieldReader<bool, bool>);
impl PENDCH11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> PENDCH0_R {
        PENDCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> PENDCH1_R {
        PENDCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pending Channel 2"]
    #[inline(always)]
    pub fn pendch2(&self) -> PENDCH2_R {
        PENDCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pending Channel 3"]
    #[inline(always)]
    pub fn pendch3(&self) -> PENDCH3_R {
        PENDCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pending Channel 4"]
    #[inline(always)]
    pub fn pendch4(&self) -> PENDCH4_R {
        PENDCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending Channel 5"]
    #[inline(always)]
    pub fn pendch5(&self) -> PENDCH5_R {
        PENDCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pending Channel 6"]
    #[inline(always)]
    pub fn pendch6(&self) -> PENDCH6_R {
        PENDCH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pending Channel 7"]
    #[inline(always)]
    pub fn pendch7(&self) -> PENDCH7_R {
        PENDCH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pending Channel 8"]
    #[inline(always)]
    pub fn pendch8(&self) -> PENDCH8_R {
        PENDCH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pending Channel 9"]
    #[inline(always)]
    pub fn pendch9(&self) -> PENDCH9_R {
        PENDCH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pending Channel 10"]
    #[inline(always)]
    pub fn pendch10(&self) -> PENDCH10_R {
        PENDCH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pending Channel 11"]
    #[inline(always)]
    pub fn pendch11(&self) -> PENDCH11_R {
        PENDCH11_R::new(((self.bits >> 11) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
