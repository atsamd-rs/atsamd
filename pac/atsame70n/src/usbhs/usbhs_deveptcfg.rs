#[doc = "Register `USBHS_DEVEPTCFG[%s]` reader"]
pub struct R(crate::R<USBHS_DEVEPTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVEPTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVEPTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVEPTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_DEVEPTCFG[%s]` writer"]
pub struct W(crate::W<USBHS_DEVEPTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVEPTCFG_SPEC>;
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
impl From<crate::W<USBHS_DEVEPTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVEPTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOC` reader - Endpoint Memory Allocate"]
pub struct ALLOC_R(crate::FieldReader<bool, bool>);
impl ALLOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALLOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOC` writer - Endpoint Memory Allocate"]
pub struct ALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOC_W<'a> {
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
#[doc = "Endpoint Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPBK_A {
    #[doc = "0: Single-bank endpoint"]
    _1_BANK = 0,
    #[doc = "1: Double-bank endpoint"]
    _2_BANK = 1,
    #[doc = "2: Triple-bank endpoint"]
    _3_BANK = 2,
}
impl From<EPBK_A> for u8 {
    #[inline(always)]
    fn from(variant: EPBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPBK` reader - Endpoint Banks"]
pub struct EPBK_R(crate::FieldReader<u8, EPBK_A>);
impl EPBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPBK_A> {
        match self.bits {
            0 => Some(EPBK_A::_1_BANK),
            1 => Some(EPBK_A::_2_BANK),
            2 => Some(EPBK_A::_3_BANK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BANK`"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        **self == EPBK_A::_1_BANK
    }
    #[doc = "Checks if the value of the field is `_2_BANK`"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        **self == EPBK_A::_2_BANK
    }
    #[doc = "Checks if the value of the field is `_3_BANK`"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        **self == EPBK_A::_3_BANK
    }
}
impl core::ops::Deref for EPBK_R {
    type Target = crate::FieldReader<u8, EPBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPBK` writer - Endpoint Banks"]
pub struct EPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPBK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut W {
        self.variant(EPBK_A::_1_BANK)
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut W {
        self.variant(EPBK_A::_2_BANK)
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut W {
        self.variant(EPBK_A::_3_BANK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Endpoint Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSIZE_A {
    #[doc = "0: 8 bytes"]
    _8_BYTE = 0,
    #[doc = "1: 16 bytes"]
    _16_BYTE = 1,
    #[doc = "2: 32 bytes"]
    _32_BYTE = 2,
    #[doc = "3: 64 bytes"]
    _64_BYTE = 3,
    #[doc = "4: 128 bytes"]
    _128_BYTE = 4,
    #[doc = "5: 256 bytes"]
    _256_BYTE = 5,
    #[doc = "6: 512 bytes"]
    _512_BYTE = 6,
    #[doc = "7: 1024 bytes"]
    _1024_BYTE = 7,
}
impl From<EPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSIZE` reader - Endpoint Size"]
pub struct EPSIZE_R(crate::FieldReader<u8, EPSIZE_A>);
impl EPSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPSIZE_A {
        match self.bits {
            0 => EPSIZE_A::_8_BYTE,
            1 => EPSIZE_A::_16_BYTE,
            2 => EPSIZE_A::_32_BYTE,
            3 => EPSIZE_A::_64_BYTE,
            4 => EPSIZE_A::_128_BYTE,
            5 => EPSIZE_A::_256_BYTE,
            6 => EPSIZE_A::_512_BYTE,
            7 => EPSIZE_A::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == EPSIZE_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == EPSIZE_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == EPSIZE_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        **self == EPSIZE_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_128_BYTE`"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        **self == EPSIZE_A::_128_BYTE
    }
    #[doc = "Checks if the value of the field is `_256_BYTE`"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        **self == EPSIZE_A::_256_BYTE
    }
    #[doc = "Checks if the value of the field is `_512_BYTE`"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        **self == EPSIZE_A::_512_BYTE
    }
    #[doc = "Checks if the value of the field is `_1024_BYTE`"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        **self == EPSIZE_A::_1024_BYTE
    }
}
impl core::ops::Deref for EPSIZE_R {
    type Target = crate::FieldReader<u8, EPSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPSIZE` writer - Endpoint Size"]
pub struct EPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut W {
        self.variant(EPSIZE_A::_1024_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDIR_A {
    #[doc = "0: The endpoint direction is OUT."]
    OUT = 0,
    #[doc = "1: The endpoint direction is IN (nor for control endpoints)."]
    IN = 1,
}
impl From<EPDIR_A> for bool {
    #[inline(always)]
    fn from(variant: EPDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub struct EPDIR_R(crate::FieldReader<bool, EPDIR_A>);
impl EPDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDIR_A {
        match self.bits {
            false => EPDIR_A::OUT,
            true => EPDIR_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == EPDIR_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == EPDIR_A::IN
    }
}
impl core::ops::Deref for EPDIR_R {
    type Target = crate::FieldReader<bool, EPDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub struct EPDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EPDIR_A::OUT)
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EPDIR_A::IN)
    }
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
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub struct AUTOSW_R(crate::FieldReader<bool, bool>);
impl AUTOSW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub struct AUTOSW_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSW_W<'a> {
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
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "1: Isochronous"]
    ISO = 1,
    #[doc = "2: Bulk"]
    BLK = 2,
    #[doc = "3: Interrupt"]
    INTRPT = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub struct EPTYPE_R(crate::FieldReader<u8, EPTYPE_A>);
impl EPTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::CTRL,
            1 => EPTYPE_A::ISO,
            2 => EPTYPE_A::BLK,
            3 => EPTYPE_A::INTRPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        **self == EPTYPE_A::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        **self == EPTYPE_A::ISO
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        **self == EPTYPE_A::BLK
    }
    #[doc = "Checks if the value of the field is `INTRPT`"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        **self == EPTYPE_A::INTRPT
    }
}
impl core::ops::Deref for EPTYPE_R {
    type Target = crate::FieldReader<u8, EPTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub struct EPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPTYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(EPTYPE_A::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut W {
        self.variant(EPTYPE_A::BLK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut W {
        self.variant(EPTYPE_A::INTRPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Number of transactions per microframe for isochronous endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBTRANS_A {
    #[doc = "0: Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    _0_TRANS = 0,
    #[doc = "1: Default value: one transaction per microframe."]
    _1_TRANS = 1,
    #[doc = "2: Two transactions per microframe. This endpoint should be configured as double-bank."]
    _2_TRANS = 2,
    #[doc = "3: Three transactions per microframe. This endpoint should be configured as triple-bank."]
    _3_TRANS = 3,
}
impl From<NBTRANS_A> for u8 {
    #[inline(always)]
    fn from(variant: NBTRANS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NBTRANS` reader - Number of transactions per microframe for isochronous endpoint"]
pub struct NBTRANS_R(crate::FieldReader<u8, NBTRANS_A>);
impl NBTRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBTRANS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBTRANS_A {
        match self.bits {
            0 => NBTRANS_A::_0_TRANS,
            1 => NBTRANS_A::_1_TRANS,
            2 => NBTRANS_A::_2_TRANS,
            3 => NBTRANS_A::_3_TRANS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_TRANS`"]
    #[inline(always)]
    pub fn is_0_trans(&self) -> bool {
        **self == NBTRANS_A::_0_TRANS
    }
    #[doc = "Checks if the value of the field is `_1_TRANS`"]
    #[inline(always)]
    pub fn is_1_trans(&self) -> bool {
        **self == NBTRANS_A::_1_TRANS
    }
    #[doc = "Checks if the value of the field is `_2_TRANS`"]
    #[inline(always)]
    pub fn is_2_trans(&self) -> bool {
        **self == NBTRANS_A::_2_TRANS
    }
    #[doc = "Checks if the value of the field is `_3_TRANS`"]
    #[inline(always)]
    pub fn is_3_trans(&self) -> bool {
        **self == NBTRANS_A::_3_TRANS
    }
}
impl core::ops::Deref for NBTRANS_R {
    type Target = crate::FieldReader<u8, NBTRANS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBTRANS` writer - Number of transactions per microframe for isochronous endpoint"]
pub struct NBTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTRANS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBTRANS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn _0_trans(self) -> &'a mut W {
        self.variant(NBTRANS_A::_0_TRANS)
    }
    #[doc = "Default value: one transaction per microframe."]
    #[inline(always)]
    pub fn _1_trans(self) -> &'a mut W {
        self.variant(NBTRANS_A::_1_TRANS)
    }
    #[doc = "Two transactions per microframe. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn _2_trans(self) -> &'a mut W {
        self.variant(NBTRANS_A::_2_TRANS)
    }
    #[doc = "Three transactions per microframe. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn _3_trans(self) -> &'a mut W {
        self.variant(NBTRANS_A::_3_TRANS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    pub fn epbk(&self) -> EPBK_R {
        EPBK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&self) -> EPSIZE_R {
        EPSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AUTOSW_R {
        AUTOSW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline(always)]
    pub fn nbtrans(&self) -> NBTRANS_R {
        NBTRANS_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&mut self) -> ALLOC_W {
        ALLOC_W { w: self }
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    pub fn epbk(&mut self) -> EPBK_W {
        EPBK_W { w: self }
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&mut self) -> EPSIZE_W {
        EPSIZE_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W {
        EPDIR_W { w: self }
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&mut self) -> AUTOSW_W {
        AUTOSW_W { w: self }
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline(always)]
    pub fn nbtrans(&mut self) -> NBTRANS_W {
        NBTRANS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptcfg](index.html) module"]
pub struct USBHS_DEVEPTCFG_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_deveptcfg::R](R) reader structure"]
impl crate::Readable for USBHS_DEVEPTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_deveptcfg::W](W) writer structure"]
impl crate::Writable for USBHS_DEVEPTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVEPTCFG[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
