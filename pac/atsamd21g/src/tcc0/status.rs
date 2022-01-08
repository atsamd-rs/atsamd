#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - Stop"]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX` reader - Ramp"]
pub struct IDX_R(crate::FieldReader<bool, bool>);
impl IDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault State"]
pub struct DFS_R(crate::FieldReader<bool, bool>);
impl DFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault State"]
pub struct DFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SLAVE` reader - Slave"]
pub struct SLAVE_R(crate::FieldReader<bool, bool>);
impl SLAVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATTBV` reader - Pattern Buffer Valid"]
pub struct PATTBV_R(crate::FieldReader<bool, bool>);
impl PATTBV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATTBV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATTBV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATTBV` writer - Pattern Buffer Valid"]
pub struct PATTBV_W<'a> {
    w: &'a mut W,
}
impl<'a> PATTBV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WAVEBV` reader - Wave Buffer Valid"]
pub struct WAVEBV_R(crate::FieldReader<bool, bool>);
impl WAVEBV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAVEBV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVEBV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVEBV` writer - Wave Buffer Valid"]
pub struct WAVEBV_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVEBV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PERBV` reader - Period Buffer Valid"]
pub struct PERBV_R(crate::FieldReader<bool, bool>);
impl PERBV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERBV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERBV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERBV` writer - Period Buffer Valid"]
pub struct PERBV_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FAULTAIN` reader - Recoverable Fault A Input"]
pub struct FAULTAIN_R(crate::FieldReader<bool, bool>);
impl FAULTAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULTAIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTBIN` reader - Recoverable Fault B Input"]
pub struct FAULTBIN_R(crate::FieldReader<bool, bool>);
impl FAULTBIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTBIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULTBIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0IN` reader - Non-Recoverable Fault0 Input"]
pub struct FAULT0IN_R(crate::FieldReader<bool, bool>);
impl FAULT0IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT0IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1IN` reader - Non-Recoverable Fault1 Input"]
pub struct FAULT1IN_R(crate::FieldReader<bool, bool>);
impl FAULT1IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTA` reader - Recoverable Fault A State"]
pub struct FAULTA_R(crate::FieldReader<bool, bool>);
impl FAULTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULTA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTA` writer - Recoverable Fault A State"]
pub struct FAULTA_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FAULTB` reader - Recoverable Fault B State"]
pub struct FAULTB_R(crate::FieldReader<bool, bool>);
impl FAULTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTB` writer - Recoverable Fault B State"]
pub struct FAULTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0 State"]
pub struct FAULT0_R(crate::FieldReader<bool, bool>);
impl FAULT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0 State"]
pub struct FAULT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1 State"]
pub struct FAULT1_R(crate::FieldReader<bool, bool>);
impl FAULT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1 State"]
pub struct FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CCBV0` reader - Compare Channel 0 Buffer Valid"]
pub struct CCBV0_R(crate::FieldReader<bool, bool>);
impl CCBV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBV0` writer - Compare Channel 0 Buffer Valid"]
pub struct CCBV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CCBV1` reader - Compare Channel 1 Buffer Valid"]
pub struct CCBV1_R(crate::FieldReader<bool, bool>);
impl CCBV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBV1` writer - Compare Channel 1 Buffer Valid"]
pub struct CCBV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CCBV2` reader - Compare Channel 2 Buffer Valid"]
pub struct CCBV2_R(crate::FieldReader<bool, bool>);
impl CCBV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBV2` writer - Compare Channel 2 Buffer Valid"]
pub struct CCBV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CCBV3` reader - Compare Channel 3 Buffer Valid"]
pub struct CCBV3_R(crate::FieldReader<bool, bool>);
impl CCBV3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBV3` writer - Compare Channel 3 Buffer Valid"]
pub struct CCBV3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CMP0` reader - Compare Channel 0 Value"]
pub struct CMP0_R(crate::FieldReader<bool, bool>);
impl CMP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1` reader - Compare Channel 1 Value"]
pub struct CMP1_R(crate::FieldReader<bool, bool>);
impl CMP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2` reader - Compare Channel 2 Value"]
pub struct CMP2_R(crate::FieldReader<bool, bool>);
impl CMP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3` reader - Compare Channel 3 Value"]
pub struct CMP3_R(crate::FieldReader<bool, bool>);
impl CMP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ramp"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbv(&self) -> PATTBV_R {
        PATTBV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    pub fn wavebv(&self) -> WAVEBV_R {
        WAVEBV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&self) -> PERBV_R {
        PERBV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Recoverable Fault A Input"]
    #[inline(always)]
    pub fn faultain(&self) -> FAULTAIN_R {
        FAULTAIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Recoverable Fault B Input"]
    #[inline(always)]
    pub fn faultbin(&self) -> FAULTBIN_R {
        FAULTBIN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
    #[inline(always)]
    pub fn fault0in(&self) -> FAULT0IN_R {
        FAULT0IN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
    #[inline(always)]
    pub fn fault1in(&self) -> FAULT1IN_R {
        FAULT1IN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    pub fn faulta(&self) -> FAULTA_R {
        FAULTA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    pub fn faultb(&self) -> FAULTB_R {
        FAULTB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv0(&self) -> CCBV0_R {
        CCBV0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv1(&self) -> CCBV1_R {
        CCBV1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv2(&self) -> CCBV2_R {
        CCBV2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv3(&self) -> CCBV3_R {
        CCBV3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Compare Channel 0 Value"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Compare Channel 1 Value"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Compare Channel 2 Value"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Compare Channel 3 Value"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W { w: self }
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbv(&mut self) -> PATTBV_W {
        PATTBV_W { w: self }
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    pub fn wavebv(&mut self) -> WAVEBV_W {
        WAVEBV_W { w: self }
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&mut self) -> PERBV_W {
        PERBV_W { w: self }
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    pub fn faulta(&mut self) -> FAULTA_W {
        FAULTA_W { w: self }
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    pub fn faultb(&mut self) -> FAULTB_W {
        FAULTB_W { w: self }
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    pub fn fault0(&mut self) -> FAULT0_W {
        FAULT0_W { w: self }
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv0(&mut self) -> CCBV0_W {
        CCBV0_W { w: self }
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv1(&mut self) -> CCBV1_W {
        CCBV1_W { w: self }
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv2(&mut self) -> CCBV2_W {
        CCBV2_W { w: self }
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv3(&mut self) -> CCBV3_W {
        CCBV3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
