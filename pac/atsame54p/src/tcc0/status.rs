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
#[doc = "Field `STOP` writer - Stop"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
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
#[doc = "Field `IDX` writer - Ramp"]
pub struct IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UFS` reader - Non-recoverable Update Fault State"]
pub struct UFS_R(crate::FieldReader<bool, bool>);
impl UFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UFS` writer - Non-recoverable Update Fault State"]
pub struct UFS_W<'a> {
    w: &'a mut W,
}
impl<'a> UFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
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
#[doc = "Field `SLAVE` writer - Slave"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PATTBUFV` reader - Pattern Buffer Valid"]
pub struct PATTBUFV_R(crate::FieldReader<bool, bool>);
impl PATTBUFV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATTBUFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATTBUFV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATTBUFV` writer - Pattern Buffer Valid"]
pub struct PATTBUFV_W<'a> {
    w: &'a mut W,
}
impl<'a> PATTBUFV_W<'a> {
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
#[doc = "Field `PERBUFV` reader - Period Buffer Valid"]
pub struct PERBUFV_R(crate::FieldReader<bool, bool>);
impl PERBUFV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERBUFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERBUFV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERBUFV` writer - Period Buffer Valid"]
pub struct PERBUFV_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBUFV_W<'a> {
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
#[doc = "Field `FAULTAIN` writer - Recoverable Fault A Input"]
pub struct FAULTAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTAIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
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
#[doc = "Field `FAULTBIN` writer - Recoverable Fault B Input"]
pub struct FAULTBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTBIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
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
#[doc = "Field `FAULT0IN` writer - Non-Recoverable Fault0 Input"]
pub struct FAULT0IN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0IN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
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
#[doc = "Field `FAULT1IN` writer - Non-Recoverable Fault1 Input"]
pub struct FAULT1IN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1IN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
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
#[doc = "Field `CCBUFV0` reader - Compare Channel 0 Buffer Valid"]
pub struct CCBUFV0_R(crate::FieldReader<bool, bool>);
impl CCBUFV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV0` writer - Compare Channel 0 Buffer Valid"]
pub struct CCBUFV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV0_W<'a> {
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
#[doc = "Field `CCBUFV1` reader - Compare Channel 1 Buffer Valid"]
pub struct CCBUFV1_R(crate::FieldReader<bool, bool>);
impl CCBUFV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV1` writer - Compare Channel 1 Buffer Valid"]
pub struct CCBUFV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV1_W<'a> {
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
#[doc = "Field `CCBUFV2` reader - Compare Channel 2 Buffer Valid"]
pub struct CCBUFV2_R(crate::FieldReader<bool, bool>);
impl CCBUFV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV2` writer - Compare Channel 2 Buffer Valid"]
pub struct CCBUFV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV2_W<'a> {
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
#[doc = "Field `CCBUFV3` reader - Compare Channel 3 Buffer Valid"]
pub struct CCBUFV3_R(crate::FieldReader<bool, bool>);
impl CCBUFV3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV3` writer - Compare Channel 3 Buffer Valid"]
pub struct CCBUFV3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV3_W<'a> {
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
#[doc = "Field `CCBUFV4` reader - Compare Channel 4 Buffer Valid"]
pub struct CCBUFV4_R(crate::FieldReader<bool, bool>);
impl CCBUFV4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV4` writer - Compare Channel 4 Buffer Valid"]
pub struct CCBUFV4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CCBUFV5` reader - Compare Channel 5 Buffer Valid"]
pub struct CCBUFV5_R(crate::FieldReader<bool, bool>);
impl CCBUFV5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV5` writer - Compare Channel 5 Buffer Valid"]
pub struct CCBUFV5_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
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
#[doc = "Field `CMP0` writer - Compare Channel 0 Value"]
pub struct CMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
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
#[doc = "Field `CMP1` writer - Compare Channel 1 Value"]
pub struct CMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
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
#[doc = "Field `CMP2` writer - Compare Channel 2 Value"]
pub struct CMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
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
#[doc = "Field `CMP3` writer - Compare Channel 3 Value"]
pub struct CMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CMP4` reader - Compare Channel 4 Value"]
pub struct CMP4_R(crate::FieldReader<bool, bool>);
impl CMP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP4` writer - Compare Channel 4 Value"]
pub struct CMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CMP5` reader - Compare Channel 5 Value"]
pub struct CMP5_R(crate::FieldReader<bool, bool>);
impl CMP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP5` writer - Compare Channel 5 Value"]
pub struct CMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
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
    #[doc = "Bit 2 - Non-recoverable Update Fault State"]
    #[inline(always)]
    pub fn ufs(&self) -> UFS_R {
        UFS_R::new(((self.bits >> 2) & 0x01) != 0)
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
    pub fn pattbufv(&self) -> PATTBUFV_R {
        PATTBUFV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbufv(&self) -> PERBUFV_R {
        PERBUFV_R::new(((self.bits >> 7) & 0x01) != 0)
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
    pub fn ccbufv0(&self) -> CCBUFV0_R {
        CCBUFV0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> CCBUFV1_R {
        CCBUFV1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv2(&self) -> CCBUFV2_R {
        CCBUFV2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv3(&self) -> CCBUFV3_R {
        CCBUFV3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Compare Channel 4 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv4(&self) -> CCBUFV4_R {
        CCBUFV4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Compare Channel 5 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv5(&self) -> CCBUFV5_R {
        CCBUFV5_R::new(((self.bits >> 21) & 0x01) != 0)
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
    #[doc = "Bit 28 - Compare Channel 4 Value"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Compare Channel 5 Value"]
    #[inline(always)]
    pub fn cmp5(&self) -> CMP5_R {
        CMP5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 1 - Ramp"]
    #[inline(always)]
    pub fn idx(&mut self) -> IDX_W {
        IDX_W { w: self }
    }
    #[doc = "Bit 2 - Non-recoverable Update Fault State"]
    #[inline(always)]
    pub fn ufs(&mut self) -> UFS_W {
        UFS_W { w: self }
    }
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W { w: self }
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbufv(&mut self) -> PATTBUFV_W {
        PATTBUFV_W { w: self }
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbufv(&mut self) -> PERBUFV_W {
        PERBUFV_W { w: self }
    }
    #[doc = "Bit 8 - Recoverable Fault A Input"]
    #[inline(always)]
    pub fn faultain(&mut self) -> FAULTAIN_W {
        FAULTAIN_W { w: self }
    }
    #[doc = "Bit 9 - Recoverable Fault B Input"]
    #[inline(always)]
    pub fn faultbin(&mut self) -> FAULTBIN_W {
        FAULTBIN_W { w: self }
    }
    #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
    #[inline(always)]
    pub fn fault0in(&mut self) -> FAULT0IN_W {
        FAULT0IN_W { w: self }
    }
    #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
    #[inline(always)]
    pub fn fault1in(&mut self) -> FAULT1IN_W {
        FAULT1IN_W { w: self }
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
    pub fn ccbufv0(&mut self) -> CCBUFV0_W {
        CCBUFV0_W { w: self }
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W {
        CCBUFV1_W { w: self }
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv2(&mut self) -> CCBUFV2_W {
        CCBUFV2_W { w: self }
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv3(&mut self) -> CCBUFV3_W {
        CCBUFV3_W { w: self }
    }
    #[doc = "Bit 20 - Compare Channel 4 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv4(&mut self) -> CCBUFV4_W {
        CCBUFV4_W { w: self }
    }
    #[doc = "Bit 21 - Compare Channel 5 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv5(&mut self) -> CCBUFV5_W {
        CCBUFV5_W { w: self }
    }
    #[doc = "Bit 24 - Compare Channel 0 Value"]
    #[inline(always)]
    pub fn cmp0(&mut self) -> CMP0_W {
        CMP0_W { w: self }
    }
    #[doc = "Bit 25 - Compare Channel 1 Value"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W {
        CMP1_W { w: self }
    }
    #[doc = "Bit 26 - Compare Channel 2 Value"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W {
        CMP2_W { w: self }
    }
    #[doc = "Bit 27 - Compare Channel 3 Value"]
    #[inline(always)]
    pub fn cmp3(&mut self) -> CMP3_W {
        CMP3_W { w: self }
    }
    #[doc = "Bit 28 - Compare Channel 4 Value"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W {
        CMP4_W { w: self }
    }
    #[doc = "Bit 29 - Compare Channel 5 Value"]
    #[inline(always)]
    pub fn cmp5(&mut self) -> CMP5_W {
        CMP5_W { w: self }
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
