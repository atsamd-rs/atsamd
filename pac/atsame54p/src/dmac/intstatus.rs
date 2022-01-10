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
pub struct CHINT0_R(crate::FieldReader<bool, bool>);
impl CHINT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT1` reader - Channel 1 Pending Interrupt"]
pub struct CHINT1_R(crate::FieldReader<bool, bool>);
impl CHINT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT2` reader - Channel 2 Pending Interrupt"]
pub struct CHINT2_R(crate::FieldReader<bool, bool>);
impl CHINT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT3` reader - Channel 3 Pending Interrupt"]
pub struct CHINT3_R(crate::FieldReader<bool, bool>);
impl CHINT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT4` reader - Channel 4 Pending Interrupt"]
pub struct CHINT4_R(crate::FieldReader<bool, bool>);
impl CHINT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT5` reader - Channel 5 Pending Interrupt"]
pub struct CHINT5_R(crate::FieldReader<bool, bool>);
impl CHINT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT6` reader - Channel 6 Pending Interrupt"]
pub struct CHINT6_R(crate::FieldReader<bool, bool>);
impl CHINT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT7` reader - Channel 7 Pending Interrupt"]
pub struct CHINT7_R(crate::FieldReader<bool, bool>);
impl CHINT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT8` reader - Channel 8 Pending Interrupt"]
pub struct CHINT8_R(crate::FieldReader<bool, bool>);
impl CHINT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT9` reader - Channel 9 Pending Interrupt"]
pub struct CHINT9_R(crate::FieldReader<bool, bool>);
impl CHINT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT10` reader - Channel 10 Pending Interrupt"]
pub struct CHINT10_R(crate::FieldReader<bool, bool>);
impl CHINT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT11` reader - Channel 11 Pending Interrupt"]
pub struct CHINT11_R(crate::FieldReader<bool, bool>);
impl CHINT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT12` reader - Channel 12 Pending Interrupt"]
pub struct CHINT12_R(crate::FieldReader<bool, bool>);
impl CHINT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT13` reader - Channel 13 Pending Interrupt"]
pub struct CHINT13_R(crate::FieldReader<bool, bool>);
impl CHINT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT14` reader - Channel 14 Pending Interrupt"]
pub struct CHINT14_R(crate::FieldReader<bool, bool>);
impl CHINT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT15` reader - Channel 15 Pending Interrupt"]
pub struct CHINT15_R(crate::FieldReader<bool, bool>);
impl CHINT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT16` reader - Channel 16 Pending Interrupt"]
pub struct CHINT16_R(crate::FieldReader<bool, bool>);
impl CHINT16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT17` reader - Channel 17 Pending Interrupt"]
pub struct CHINT17_R(crate::FieldReader<bool, bool>);
impl CHINT17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT18` reader - Channel 18 Pending Interrupt"]
pub struct CHINT18_R(crate::FieldReader<bool, bool>);
impl CHINT18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT19` reader - Channel 19 Pending Interrupt"]
pub struct CHINT19_R(crate::FieldReader<bool, bool>);
impl CHINT19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT20` reader - Channel 20 Pending Interrupt"]
pub struct CHINT20_R(crate::FieldReader<bool, bool>);
impl CHINT20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT21` reader - Channel 21 Pending Interrupt"]
pub struct CHINT21_R(crate::FieldReader<bool, bool>);
impl CHINT21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT22` reader - Channel 22 Pending Interrupt"]
pub struct CHINT22_R(crate::FieldReader<bool, bool>);
impl CHINT22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT23` reader - Channel 23 Pending Interrupt"]
pub struct CHINT23_R(crate::FieldReader<bool, bool>);
impl CHINT23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT24` reader - Channel 24 Pending Interrupt"]
pub struct CHINT24_R(crate::FieldReader<bool, bool>);
impl CHINT24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT25` reader - Channel 25 Pending Interrupt"]
pub struct CHINT25_R(crate::FieldReader<bool, bool>);
impl CHINT25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT26` reader - Channel 26 Pending Interrupt"]
pub struct CHINT26_R(crate::FieldReader<bool, bool>);
impl CHINT26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT27` reader - Channel 27 Pending Interrupt"]
pub struct CHINT27_R(crate::FieldReader<bool, bool>);
impl CHINT27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT28` reader - Channel 28 Pending Interrupt"]
pub struct CHINT28_R(crate::FieldReader<bool, bool>);
impl CHINT28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT29` reader - Channel 29 Pending Interrupt"]
pub struct CHINT29_R(crate::FieldReader<bool, bool>);
impl CHINT29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT30` reader - Channel 30 Pending Interrupt"]
pub struct CHINT30_R(crate::FieldReader<bool, bool>);
impl CHINT30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINT31` reader - Channel 31 Pending Interrupt"]
pub struct CHINT31_R(crate::FieldReader<bool, bool>);
impl CHINT31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHINT31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINT31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Pending Interrupt"]
    #[inline(always)]
    pub fn chint0(&self) -> CHINT0_R {
        CHINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Pending Interrupt"]
    #[inline(always)]
    pub fn chint1(&self) -> CHINT1_R {
        CHINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Pending Interrupt"]
    #[inline(always)]
    pub fn chint2(&self) -> CHINT2_R {
        CHINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Pending Interrupt"]
    #[inline(always)]
    pub fn chint3(&self) -> CHINT3_R {
        CHINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Pending Interrupt"]
    #[inline(always)]
    pub fn chint4(&self) -> CHINT4_R {
        CHINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Pending Interrupt"]
    #[inline(always)]
    pub fn chint5(&self) -> CHINT5_R {
        CHINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Pending Interrupt"]
    #[inline(always)]
    pub fn chint6(&self) -> CHINT6_R {
        CHINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Pending Interrupt"]
    #[inline(always)]
    pub fn chint7(&self) -> CHINT7_R {
        CHINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Pending Interrupt"]
    #[inline(always)]
    pub fn chint8(&self) -> CHINT8_R {
        CHINT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Pending Interrupt"]
    #[inline(always)]
    pub fn chint9(&self) -> CHINT9_R {
        CHINT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Pending Interrupt"]
    #[inline(always)]
    pub fn chint10(&self) -> CHINT10_R {
        CHINT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Pending Interrupt"]
    #[inline(always)]
    pub fn chint11(&self) -> CHINT11_R {
        CHINT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Pending Interrupt"]
    #[inline(always)]
    pub fn chint12(&self) -> CHINT12_R {
        CHINT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Pending Interrupt"]
    #[inline(always)]
    pub fn chint13(&self) -> CHINT13_R {
        CHINT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Pending Interrupt"]
    #[inline(always)]
    pub fn chint14(&self) -> CHINT14_R {
        CHINT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Pending Interrupt"]
    #[inline(always)]
    pub fn chint15(&self) -> CHINT15_R {
        CHINT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Pending Interrupt"]
    #[inline(always)]
    pub fn chint16(&self) -> CHINT16_R {
        CHINT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Pending Interrupt"]
    #[inline(always)]
    pub fn chint17(&self) -> CHINT17_R {
        CHINT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Pending Interrupt"]
    #[inline(always)]
    pub fn chint18(&self) -> CHINT18_R {
        CHINT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Pending Interrupt"]
    #[inline(always)]
    pub fn chint19(&self) -> CHINT19_R {
        CHINT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Pending Interrupt"]
    #[inline(always)]
    pub fn chint20(&self) -> CHINT20_R {
        CHINT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Pending Interrupt"]
    #[inline(always)]
    pub fn chint21(&self) -> CHINT21_R {
        CHINT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Pending Interrupt"]
    #[inline(always)]
    pub fn chint22(&self) -> CHINT22_R {
        CHINT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Pending Interrupt"]
    #[inline(always)]
    pub fn chint23(&self) -> CHINT23_R {
        CHINT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 24 Pending Interrupt"]
    #[inline(always)]
    pub fn chint24(&self) -> CHINT24_R {
        CHINT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 25 Pending Interrupt"]
    #[inline(always)]
    pub fn chint25(&self) -> CHINT25_R {
        CHINT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 26 Pending Interrupt"]
    #[inline(always)]
    pub fn chint26(&self) -> CHINT26_R {
        CHINT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 27 Pending Interrupt"]
    #[inline(always)]
    pub fn chint27(&self) -> CHINT27_R {
        CHINT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Channel 28 Pending Interrupt"]
    #[inline(always)]
    pub fn chint28(&self) -> CHINT28_R {
        CHINT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 29 Pending Interrupt"]
    #[inline(always)]
    pub fn chint29(&self) -> CHINT29_R {
        CHINT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 30 Pending Interrupt"]
    #[inline(always)]
    pub fn chint30(&self) -> CHINT30_R {
        CHINT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 31 Pending Interrupt"]
    #[inline(always)]
    pub fn chint31(&self) -> CHINT31_R {
        CHINT31_R::new(((self.bits >> 31) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
