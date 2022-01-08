#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSCRDY0` reader - XOSC 0 Ready Interrupt Enable"]
pub struct XOSCRDY0_R(crate::FieldReader<bool, bool>);
impl XOSCRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCRDY0` writer - XOSC 0 Ready Interrupt Enable"]
pub struct XOSCRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY0_W<'a> {
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
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready Interrupt Enable"]
pub struct XOSCRDY1_R(crate::FieldReader<bool, bool>);
impl XOSCRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCRDY1` writer - XOSC 1 Ready Interrupt Enable"]
pub struct XOSCRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY1_W<'a> {
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
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector Interrupt Enable"]
pub struct XOSCFAIL0_R(crate::FieldReader<bool, bool>);
impl XOSCFAIL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCFAIL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCFAIL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCFAIL0` writer - XOSC 0 Clock Failure Detector Interrupt Enable"]
pub struct XOSCFAIL0_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL0_W<'a> {
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
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector Interrupt Enable"]
pub struct XOSCFAIL1_R(crate::FieldReader<bool, bool>);
impl XOSCFAIL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCFAIL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCFAIL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCFAIL1` writer - XOSC 1 Clock Failure Detector Interrupt Enable"]
pub struct XOSCFAIL1_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL1_W<'a> {
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
#[doc = "Field `DFLLRDY` reader - DFLL Ready Interrupt Enable"]
pub struct DFLLRDY_R(crate::FieldReader<bool, bool>);
impl DFLLRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLRDY` writer - DFLL Ready Interrupt Enable"]
pub struct DFLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLRDY_W<'a> {
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
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds Interrupt Enable"]
pub struct DFLLOOB_R(crate::FieldReader<bool, bool>);
impl DFLLOOB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLOOB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLOOB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLOOB` writer - DFLL Out Of Bounds Interrupt Enable"]
pub struct DFLLOOB_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLOOB_W<'a> {
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
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine Interrupt Enable"]
pub struct DFLLLCKF_R(crate::FieldReader<bool, bool>);
impl DFLLLCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLLCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLLCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLLCKF` writer - DFLL Lock Fine Interrupt Enable"]
pub struct DFLLLCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLLCKF_W<'a> {
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
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse Interrupt Enable"]
pub struct DFLLLCKC_R(crate::FieldReader<bool, bool>);
impl DFLLLCKC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLLCKC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLLCKC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLLCKC` writer - DFLL Lock Coarse Interrupt Enable"]
pub struct DFLLLCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLLCKC_W<'a> {
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
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped Interrupt Enable"]
pub struct DFLLRCS_R(crate::FieldReader<bool, bool>);
impl DFLLRCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLRCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLRCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped Interrupt Enable"]
pub struct DFLLRCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLRCS_W<'a> {
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
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise Interrupt Enable"]
pub struct DPLL0LCKR_R(crate::FieldReader<bool, bool>);
impl DPLL0LCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0LCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0LCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0LCKR` writer - DPLL0 Lock Rise Interrupt Enable"]
pub struct DPLL0LCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LCKR_W<'a> {
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
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall Interrupt Enable"]
pub struct DPLL0LCKF_R(crate::FieldReader<bool, bool>);
impl DPLL0LCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0LCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0LCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0LCKF` writer - DPLL0 Lock Fall Interrupt Enable"]
pub struct DPLL0LCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LCKF_W<'a> {
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
#[doc = "Field `DPLL0LTO` reader - DPLL0 Lock Timeout Interrupt Enable"]
pub struct DPLL0LTO_R(crate::FieldReader<bool, bool>);
impl DPLL0LTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0LTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0LTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0LTO` writer - DPLL0 Lock Timeout Interrupt Enable"]
pub struct DPLL0LTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LTO_W<'a> {
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
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
pub struct DPLL0LDRTO_R(crate::FieldReader<bool, bool>);
impl DPLL0LDRTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0LDRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0LDRTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0LDRTO` writer - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
pub struct DPLL0LDRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LDRTO_W<'a> {
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
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise Interrupt Enable"]
pub struct DPLL1LCKR_R(crate::FieldReader<bool, bool>);
impl DPLL1LCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1LCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1LCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1LCKR` writer - DPLL1 Lock Rise Interrupt Enable"]
pub struct DPLL1LCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LCKR_W<'a> {
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
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall Interrupt Enable"]
pub struct DPLL1LCKF_R(crate::FieldReader<bool, bool>);
impl DPLL1LCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1LCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1LCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1LCKF` writer - DPLL1 Lock Fall Interrupt Enable"]
pub struct DPLL1LCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LCKF_W<'a> {
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
#[doc = "Field `DPLL1LTO` reader - DPLL1 Lock Timeout Interrupt Enable"]
pub struct DPLL1LTO_R(crate::FieldReader<bool, bool>);
impl DPLL1LTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1LTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1LTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1LTO` writer - DPLL1 Lock Timeout Interrupt Enable"]
pub struct DPLL1LTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LTO_W<'a> {
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
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
pub struct DPLL1LDRTO_R(crate::FieldReader<bool, bool>);
impl DPLL1LDRTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1LDRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1LDRTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1LDRTO` writer - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
pub struct DPLL1LDRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LDRTO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> XOSCRDY0_R {
        XOSCRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> XOSCRDY1_R {
        XOSCRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> XOSCFAIL0_R {
        XOSCFAIL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> XOSCFAIL1_R {
        XOSCFAIL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> DPLL0LCKR_R {
        DPLL0LCKR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> DPLL0LCKF_R {
        DPLL0LCKF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lto(&self) -> DPLL0LTO_R {
        DPLL0LTO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> DPLL0LDRTO_R {
        DPLL0LDRTO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> DPLL1LCKR_R {
        DPLL1LCKR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> DPLL1LCKF_R {
        DPLL1LCKF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lto(&self) -> DPLL1LTO_R {
        DPLL1LTO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> DPLL1LDRTO_R {
        DPLL1LDRTO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy0(&mut self) -> XOSCRDY0_W {
        XOSCRDY0_W { w: self }
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy1(&mut self) -> XOSCRDY1_W {
        XOSCRDY1_W { w: self }
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail0(&mut self) -> XOSCFAIL0_W {
        XOSCFAIL0_W { w: self }
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail1(&mut self) -> XOSCFAIL1_W {
        XOSCFAIL1_W { w: self }
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W {
        DFLLRDY_W { w: self }
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&mut self) -> DFLLOOB_W {
        DFLLOOB_W { w: self }
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W {
        DFLLLCKF_W { w: self }
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W {
        DFLLLCKC_W { w: self }
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W {
        DFLLRCS_W { w: self }
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckr(&mut self) -> DPLL0LCKR_W {
        DPLL0LCKR_W { w: self }
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckf(&mut self) -> DPLL0LCKF_W {
        DPLL0LCKF_W { w: self }
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lto(&mut self) -> DPLL0LTO_W {
        DPLL0LTO_W { w: self }
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0ldrto(&mut self) -> DPLL0LDRTO_W {
        DPLL0LDRTO_W { w: self }
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckr(&mut self) -> DPLL1LCKR_W {
        DPLL1LCKR_W { w: self }
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckf(&mut self) -> DPLL1LCKF_W {
        DPLL1LCKF_W { w: self }
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lto(&mut self) -> DPLL1LTO_W {
        DPLL1LTO_W { w: self }
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1ldrto(&mut self) -> DPLL1LDRTO_W {
        DPLL1LDRTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
