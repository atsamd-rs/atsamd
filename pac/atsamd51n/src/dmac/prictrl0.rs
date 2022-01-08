#[doc = "Register `PRICTRL0` reader"]
pub struct R(crate::R<PRICTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRICTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRICTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRICTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRICTRL0` writer"]
pub struct W(crate::W<PRICTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRICTRL0_SPEC>;
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
impl From<crate::W<PRICTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRICTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVLPRI0` reader - Level 0 Channel Priority Number"]
pub struct LVLPRI0_R(crate::FieldReader<u8, u8>);
impl LVLPRI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LVLPRI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLPRI0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLPRI0` writer - Level 0 Channel Priority Number"]
pub struct LVLPRI0_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Level 0 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QOS0_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS0_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QOS0` reader - Level 0 Quality of Service"]
pub struct QOS0_R(crate::FieldReader<u8, QOS0_A>);
impl QOS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QOS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QOS0_A {
        match self.bits {
            0 => QOS0_A::REGULAR,
            1 => QOS0_A::SHORTAGE,
            2 => QOS0_A::SENSITIVE,
            3 => QOS0_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        **self == QOS0_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        **self == QOS0_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        **self == QOS0_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        **self == QOS0_A::CRITICAL
    }
}
impl core::ops::Deref for QOS0_R {
    type Target = crate::FieldReader<u8, QOS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QOS0` writer - Level 0 Quality of Service"]
pub struct QOS0_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS0_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS0_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS0_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS0_A::CRITICAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `RRLVLEN0` reader - Level 0 Round-Robin Scheduling Enable"]
pub struct RRLVLEN0_R(crate::FieldReader<bool, bool>);
impl RRLVLEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRLVLEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRLVLEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRLVLEN0` writer - Level 0 Round-Robin Scheduling Enable"]
pub struct RRLVLEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RRLVLEN0_W<'a> {
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
#[doc = "Field `LVLPRI1` reader - Level 1 Channel Priority Number"]
pub struct LVLPRI1_R(crate::FieldReader<u8, u8>);
impl LVLPRI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LVLPRI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLPRI1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLPRI1` writer - Level 1 Channel Priority Number"]
pub struct LVLPRI1_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Level 1 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QOS1_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS1_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QOS1` reader - Level 1 Quality of Service"]
pub struct QOS1_R(crate::FieldReader<u8, QOS1_A>);
impl QOS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QOS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QOS1_A {
        match self.bits {
            0 => QOS1_A::REGULAR,
            1 => QOS1_A::SHORTAGE,
            2 => QOS1_A::SENSITIVE,
            3 => QOS1_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        **self == QOS1_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        **self == QOS1_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        **self == QOS1_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        **self == QOS1_A::CRITICAL
    }
}
impl core::ops::Deref for QOS1_R {
    type Target = crate::FieldReader<u8, QOS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QOS1` writer - Level 1 Quality of Service"]
pub struct QOS1_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS1_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS1_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS1_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS1_A::CRITICAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `RRLVLEN1` reader - Level 1 Round-Robin Scheduling Enable"]
pub struct RRLVLEN1_R(crate::FieldReader<bool, bool>);
impl RRLVLEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRLVLEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRLVLEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRLVLEN1` writer - Level 1 Round-Robin Scheduling Enable"]
pub struct RRLVLEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RRLVLEN1_W<'a> {
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
#[doc = "Field `LVLPRI2` reader - Level 2 Channel Priority Number"]
pub struct LVLPRI2_R(crate::FieldReader<u8, u8>);
impl LVLPRI2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LVLPRI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLPRI2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLPRI2` writer - Level 2 Channel Priority Number"]
pub struct LVLPRI2_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Level 2 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QOS2_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS2_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QOS2` reader - Level 2 Quality of Service"]
pub struct QOS2_R(crate::FieldReader<u8, QOS2_A>);
impl QOS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QOS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QOS2_A {
        match self.bits {
            0 => QOS2_A::REGULAR,
            1 => QOS2_A::SHORTAGE,
            2 => QOS2_A::SENSITIVE,
            3 => QOS2_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        **self == QOS2_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        **self == QOS2_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        **self == QOS2_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        **self == QOS2_A::CRITICAL
    }
}
impl core::ops::Deref for QOS2_R {
    type Target = crate::FieldReader<u8, QOS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QOS2` writer - Level 2 Quality of Service"]
pub struct QOS2_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS2_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS2_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS2_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS2_A::CRITICAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `RRLVLEN2` reader - Level 2 Round-Robin Scheduling Enable"]
pub struct RRLVLEN2_R(crate::FieldReader<bool, bool>);
impl RRLVLEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRLVLEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRLVLEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRLVLEN2` writer - Level 2 Round-Robin Scheduling Enable"]
pub struct RRLVLEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RRLVLEN2_W<'a> {
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
#[doc = "Field `LVLPRI3` reader - Level 3 Channel Priority Number"]
pub struct LVLPRI3_R(crate::FieldReader<u8, u8>);
impl LVLPRI3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LVLPRI3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLPRI3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLPRI3` writer - Level 3 Channel Priority Number"]
pub struct LVLPRI3_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Level 3 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QOS3_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS3_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QOS3` reader - Level 3 Quality of Service"]
pub struct QOS3_R(crate::FieldReader<u8, QOS3_A>);
impl QOS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QOS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QOS3_A {
        match self.bits {
            0 => QOS3_A::REGULAR,
            1 => QOS3_A::SHORTAGE,
            2 => QOS3_A::SENSITIVE,
            3 => QOS3_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        **self == QOS3_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        **self == QOS3_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        **self == QOS3_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        **self == QOS3_A::CRITICAL
    }
}
impl core::ops::Deref for QOS3_R {
    type Target = crate::FieldReader<u8, QOS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QOS3` writer - Level 3 Quality of Service"]
pub struct QOS3_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS3_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS3_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS3_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS3_A::CRITICAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `RRLVLEN3` reader - Level 3 Round-Robin Scheduling Enable"]
pub struct RRLVLEN3_R(crate::FieldReader<bool, bool>);
impl RRLVLEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRLVLEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRLVLEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRLVLEN3` writer - Level 3 Round-Robin Scheduling Enable"]
pub struct RRLVLEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> RRLVLEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&self) -> LVLPRI0_R {
        LVLPRI0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline(always)]
    pub fn qos0(&self) -> QOS0_R {
        QOS0_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&self) -> RRLVLEN0_R {
        RRLVLEN0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&self) -> LVLPRI1_R {
        LVLPRI1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline(always)]
    pub fn qos1(&self) -> QOS1_R {
        QOS1_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&self) -> RRLVLEN1_R {
        RRLVLEN1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&self) -> LVLPRI2_R {
        LVLPRI2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline(always)]
    pub fn qos2(&self) -> QOS2_R {
        QOS2_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&self) -> RRLVLEN2_R {
        RRLVLEN2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&self) -> LVLPRI3_R {
        LVLPRI3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline(always)]
    pub fn qos3(&self) -> QOS3_R {
        QOS3_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&self) -> RRLVLEN3_R {
        RRLVLEN3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&mut self) -> LVLPRI0_W {
        LVLPRI0_W { w: self }
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline(always)]
    pub fn qos0(&mut self) -> QOS0_W {
        QOS0_W { w: self }
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&mut self) -> RRLVLEN0_W {
        RRLVLEN0_W { w: self }
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&mut self) -> LVLPRI1_W {
        LVLPRI1_W { w: self }
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline(always)]
    pub fn qos1(&mut self) -> QOS1_W {
        QOS1_W { w: self }
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&mut self) -> RRLVLEN1_W {
        RRLVLEN1_W { w: self }
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&mut self) -> LVLPRI2_W {
        LVLPRI2_W { w: self }
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline(always)]
    pub fn qos2(&mut self) -> QOS2_W {
        QOS2_W { w: self }
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&mut self) -> RRLVLEN2_W {
        RRLVLEN2_W { w: self }
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&mut self) -> LVLPRI3_W {
        LVLPRI3_W { w: self }
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline(always)]
    pub fn qos3(&mut self) -> QOS3_W {
        QOS3_W { w: self }
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&mut self) -> RRLVLEN3_W {
        RRLVLEN3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prictrl0](index.html) module"]
pub struct PRICTRL0_SPEC;
impl crate::RegisterSpec for PRICTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prictrl0::R](R) reader structure"]
impl crate::Readable for PRICTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prictrl0::W](W) writer structure"]
impl crate::Writable for PRICTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRICTRL0 to value 0x4040_4040"]
impl crate::Resettable for PRICTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4040_4040
    }
}
