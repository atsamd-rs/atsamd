#[doc = "Register `USBHS_HSTPIP` reader"]
pub struct R(crate::R<USBHS_HSTPIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTPIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTPIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTPIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTPIP` writer"]
pub struct W(crate::W<USBHS_HSTPIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTPIP_SPEC>;
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
impl From<crate::W<USBHS_HSTPIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTPIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEN0` reader - Pipe 0 Enable"]
pub struct PEN0_R(crate::FieldReader<bool, bool>);
impl PEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN0` writer - Pipe 0 Enable"]
pub struct PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN0_W<'a> {
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
#[doc = "Field `PEN1` reader - Pipe 1 Enable"]
pub struct PEN1_R(crate::FieldReader<bool, bool>);
impl PEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN1` writer - Pipe 1 Enable"]
pub struct PEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN1_W<'a> {
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
#[doc = "Field `PEN2` reader - Pipe 2 Enable"]
pub struct PEN2_R(crate::FieldReader<bool, bool>);
impl PEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN2` writer - Pipe 2 Enable"]
pub struct PEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN2_W<'a> {
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
#[doc = "Field `PEN3` reader - Pipe 3 Enable"]
pub struct PEN3_R(crate::FieldReader<bool, bool>);
impl PEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN3` writer - Pipe 3 Enable"]
pub struct PEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN3_W<'a> {
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
#[doc = "Field `PEN4` reader - Pipe 4 Enable"]
pub struct PEN4_R(crate::FieldReader<bool, bool>);
impl PEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN4` writer - Pipe 4 Enable"]
pub struct PEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN4_W<'a> {
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
#[doc = "Field `PEN5` reader - Pipe 5 Enable"]
pub struct PEN5_R(crate::FieldReader<bool, bool>);
impl PEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN5` writer - Pipe 5 Enable"]
pub struct PEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN5_W<'a> {
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
#[doc = "Field `PEN6` reader - Pipe 6 Enable"]
pub struct PEN6_R(crate::FieldReader<bool, bool>);
impl PEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN6` writer - Pipe 6 Enable"]
pub struct PEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN6_W<'a> {
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
#[doc = "Field `PEN7` reader - Pipe 7 Enable"]
pub struct PEN7_R(crate::FieldReader<bool, bool>);
impl PEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN7` writer - Pipe 7 Enable"]
pub struct PEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN7_W<'a> {
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
#[doc = "Field `PEN8` reader - Pipe 8 Enable"]
pub struct PEN8_R(crate::FieldReader<bool, bool>);
impl PEN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN8` writer - Pipe 8 Enable"]
pub struct PEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN8_W<'a> {
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
#[doc = "Field `PRST0` reader - Pipe 0 Reset"]
pub struct PRST0_R(crate::FieldReader<bool, bool>);
impl PRST0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST0` writer - Pipe 0 Reset"]
pub struct PRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST0_W<'a> {
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
#[doc = "Field `PRST1` reader - Pipe 1 Reset"]
pub struct PRST1_R(crate::FieldReader<bool, bool>);
impl PRST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST1` writer - Pipe 1 Reset"]
pub struct PRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST1_W<'a> {
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
#[doc = "Field `PRST2` reader - Pipe 2 Reset"]
pub struct PRST2_R(crate::FieldReader<bool, bool>);
impl PRST2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST2` writer - Pipe 2 Reset"]
pub struct PRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST2_W<'a> {
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
#[doc = "Field `PRST3` reader - Pipe 3 Reset"]
pub struct PRST3_R(crate::FieldReader<bool, bool>);
impl PRST3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST3` writer - Pipe 3 Reset"]
pub struct PRST3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST3_W<'a> {
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
#[doc = "Field `PRST4` reader - Pipe 4 Reset"]
pub struct PRST4_R(crate::FieldReader<bool, bool>);
impl PRST4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST4` writer - Pipe 4 Reset"]
pub struct PRST4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST4_W<'a> {
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
#[doc = "Field `PRST5` reader - Pipe 5 Reset"]
pub struct PRST5_R(crate::FieldReader<bool, bool>);
impl PRST5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST5` writer - Pipe 5 Reset"]
pub struct PRST5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST5_W<'a> {
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
#[doc = "Field `PRST6` reader - Pipe 6 Reset"]
pub struct PRST6_R(crate::FieldReader<bool, bool>);
impl PRST6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST6` writer - Pipe 6 Reset"]
pub struct PRST6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PRST7` reader - Pipe 7 Reset"]
pub struct PRST7_R(crate::FieldReader<bool, bool>);
impl PRST7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST7` writer - Pipe 7 Reset"]
pub struct PRST7_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PRST8` reader - Pipe 8 Reset"]
pub struct PRST8_R(crate::FieldReader<bool, bool>);
impl PRST8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST8` writer - Pipe 8 Reset"]
pub struct PRST8_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST8_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    pub fn pen0(&self) -> PEN0_R {
        PEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    pub fn pen1(&self) -> PEN1_R {
        PEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    pub fn pen2(&self) -> PEN2_R {
        PEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    pub fn pen3(&self) -> PEN3_R {
        PEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    pub fn pen4(&self) -> PEN4_R {
        PEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    pub fn pen5(&self) -> PEN5_R {
        PEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    pub fn pen6(&self) -> PEN6_R {
        PEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    pub fn pen7(&self) -> PEN7_R {
        PEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    pub fn pen8(&self) -> PEN8_R {
        PEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    pub fn prst0(&self) -> PRST0_R {
        PRST0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    pub fn prst1(&self) -> PRST1_R {
        PRST1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    pub fn prst2(&self) -> PRST2_R {
        PRST2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    pub fn prst3(&self) -> PRST3_R {
        PRST3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    pub fn prst4(&self) -> PRST4_R {
        PRST4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    pub fn prst5(&self) -> PRST5_R {
        PRST5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    pub fn prst6(&self) -> PRST6_R {
        PRST6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    pub fn prst7(&self) -> PRST7_R {
        PRST7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    pub fn prst8(&self) -> PRST8_R {
        PRST8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    pub fn pen0(&mut self) -> PEN0_W {
        PEN0_W { w: self }
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    pub fn pen1(&mut self) -> PEN1_W {
        PEN1_W { w: self }
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    pub fn pen2(&mut self) -> PEN2_W {
        PEN2_W { w: self }
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    pub fn pen3(&mut self) -> PEN3_W {
        PEN3_W { w: self }
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    pub fn pen4(&mut self) -> PEN4_W {
        PEN4_W { w: self }
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    pub fn pen5(&mut self) -> PEN5_W {
        PEN5_W { w: self }
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    pub fn pen6(&mut self) -> PEN6_W {
        PEN6_W { w: self }
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    pub fn pen7(&mut self) -> PEN7_W {
        PEN7_W { w: self }
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    pub fn pen8(&mut self) -> PEN8_W {
        PEN8_W { w: self }
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    pub fn prst0(&mut self) -> PRST0_W {
        PRST0_W { w: self }
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    pub fn prst1(&mut self) -> PRST1_W {
        PRST1_W { w: self }
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    pub fn prst2(&mut self) -> PRST2_W {
        PRST2_W { w: self }
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    pub fn prst3(&mut self) -> PRST3_W {
        PRST3_W { w: self }
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    pub fn prst4(&mut self) -> PRST4_W {
        PRST4_W { w: self }
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    pub fn prst5(&mut self) -> PRST5_W {
        PRST5_W { w: self }
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    pub fn prst6(&mut self) -> PRST6_W {
        PRST6_W { w: self }
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    pub fn prst7(&mut self) -> PRST7_W {
        PRST7_W { w: self }
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    pub fn prst8(&mut self) -> PRST8_W {
        PRST8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpip](index.html) module"]
pub struct USBHS_HSTPIP_SPEC;
impl crate::RegisterSpec for USBHS_HSTPIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstpip::R](R) reader structure"]
impl crate::Readable for USBHS_HSTPIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpip::W](W) writer structure"]
impl crate::Writable for USBHS_HSTPIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTPIP to value 0"]
impl crate::Resettable for USBHS_HSTPIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
