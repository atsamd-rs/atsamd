#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSCRDY` reader - XOSC Ready Interrupt Enable"]
pub struct XOSCRDY_R(crate::FieldReader<bool, bool>);
impl XOSCRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCRDY` writer - XOSC Ready Interrupt Enable"]
pub struct XOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY_W<'a> {
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
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub struct XOSC32KRDY_R(crate::FieldReader<bool, bool>);
impl XOSC32KRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSC32KRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC32KRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub struct XOSC32KRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC32KRDY_W<'a> {
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
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready Interrupt Enable"]
pub struct OSC32KRDY_R(crate::FieldReader<bool, bool>);
impl OSC32KRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC32KRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32KRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32KRDY` writer - OSC32K Ready Interrupt Enable"]
pub struct OSC32KRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KRDY_W<'a> {
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
#[doc = "Field `OSC8MRDY` reader - OSC8M Ready Interrupt Enable"]
pub struct OSC8MRDY_R(crate::FieldReader<bool, bool>);
impl OSC8MRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC8MRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC8MRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC8MRDY` writer - OSC8M Ready Interrupt Enable"]
pub struct OSC8MRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC8MRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BOD33RDY` reader - BOD33 Ready Interrupt Enable"]
pub struct BOD33RDY_R(crate::FieldReader<bool, bool>);
impl BOD33RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD33RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33RDY` writer - BOD33 Ready Interrupt Enable"]
pub struct BOD33RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD33RDY_W<'a> {
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
#[doc = "Field `BOD33DET` reader - BOD33 Detection Interrupt Enable"]
pub struct BOD33DET_R(crate::FieldReader<bool, bool>);
impl BOD33DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD33DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33DET` writer - BOD33 Detection Interrupt Enable"]
pub struct BOD33DET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD33DET_W<'a> {
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
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready Interrupt Enable"]
pub struct B33SRDY_R(crate::FieldReader<bool, bool>);
impl B33SRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        B33SRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B33SRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready Interrupt Enable"]
pub struct B33SRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> B33SRDY_W<'a> {
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
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise Interrupt Enable"]
pub struct DPLLLCKR_R(crate::FieldReader<bool, bool>);
impl DPLLLCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKR` writer - DPLL Lock Rise Interrupt Enable"]
pub struct DPLLLCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKR_W<'a> {
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
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall Interrupt Enable"]
pub struct DPLLLCKF_R(crate::FieldReader<bool, bool>);
impl DPLLLCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKF` writer - DPLL Lock Fall Interrupt Enable"]
pub struct DPLLLCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKF_W<'a> {
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
#[doc = "Field `DPLLLTO` reader - DPLL Lock Timeout Interrupt Enable"]
pub struct DPLLLTO_R(crate::FieldReader<bool, bool>);
impl DPLLLTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLTO` writer - DPLL Lock Timeout Interrupt Enable"]
pub struct DPLLLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLTO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> OSC32KRDY_R {
        OSC32KRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> OSC8MRDY_R {
        OSC8MRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&mut self) -> XOSCRDY_W {
        XOSCRDY_W { w: self }
    }
    #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&mut self) -> XOSC32KRDY_W {
        XOSC32KRDY_W { w: self }
    }
    #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc32krdy(&mut self) -> OSC32KRDY_W {
        OSC32KRDY_W { w: self }
    }
    #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc8mrdy(&mut self) -> OSC8MRDY_W {
        OSC8MRDY_W { w: self }
    }
    #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W {
        DFLLRDY_W { w: self }
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&mut self) -> DFLLOOB_W {
        DFLLOOB_W { w: self }
    }
    #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W {
        DFLLLCKF_W { w: self }
    }
    #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W {
        DFLLLCKC_W { w: self }
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W {
        DFLLRCS_W { w: self }
    }
    #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bod33rdy(&mut self) -> BOD33RDY_W {
        BOD33RDY_W { w: self }
    }
    #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bod33det(&mut self) -> BOD33DET_W {
        BOD33DET_W { w: self }
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn b33srdy(&mut self) -> B33SRDY_W {
        B33SRDY_W { w: self }
    }
    #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W {
        DPLLLCKR_W { w: self }
    }
    #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W {
        DPLLLCKF_W { w: self }
    }
    #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&mut self) -> DPLLLTO_W {
        DPLLLTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
