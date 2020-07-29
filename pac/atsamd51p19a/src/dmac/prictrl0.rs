#[doc = "Reader of register PRICTRL0"]
pub type R = crate::R<u32, super::PRICTRL0>;
#[doc = "Writer for register PRICTRL0"]
pub type W = crate::W<u32, super::PRICTRL0>;
#[doc = "Register PRICTRL0 `reset()`'s with value 0x4040_4040"]
impl crate::ResetValue for super::PRICTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4040_4040
    }
}
#[doc = "Reader of field `LVLPRI0`"]
pub type LVLPRI0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVLPRI0`"]
pub struct LVLPRI0_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
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
#[doc = "Reader of field `QOS0`"]
pub type QOS0_R = crate::R<u8, QOS0_A>;
impl QOS0_R {
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
        *self == QOS0_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS0_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS0_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS0_A::CRITICAL
    }
}
#[doc = "Write proxy for field `QOS0`"]
pub struct QOS0_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RRLVLEN0`"]
pub type RRLVLEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRLVLEN0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `LVLPRI1`"]
pub type LVLPRI1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVLPRI1`"]
pub struct LVLPRI1_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
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
#[doc = "Reader of field `QOS1`"]
pub type QOS1_R = crate::R<u8, QOS1_A>;
impl QOS1_R {
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
        *self == QOS1_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS1_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS1_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS1_A::CRITICAL
    }
}
#[doc = "Write proxy for field `QOS1`"]
pub struct QOS1_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `RRLVLEN1`"]
pub type RRLVLEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRLVLEN1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LVLPRI2`"]
pub type LVLPRI2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVLPRI2`"]
pub struct LVLPRI2_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
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
#[doc = "Reader of field `QOS2`"]
pub type QOS2_R = crate::R<u8, QOS2_A>;
impl QOS2_R {
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
        *self == QOS2_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS2_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS2_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS2_A::CRITICAL
    }
}
#[doc = "Write proxy for field `QOS2`"]
pub struct QOS2_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `RRLVLEN2`"]
pub type RRLVLEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRLVLEN2`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `LVLPRI3`"]
pub type LVLPRI3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVLPRI3`"]
pub struct LVLPRI3_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLPRI3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
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
#[doc = "Reader of field `QOS3`"]
pub type QOS3_R = crate::R<u8, QOS3_A>;
impl QOS3_R {
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
        *self == QOS3_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS3_A::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS3_A::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS3_A::CRITICAL
    }
}
#[doc = "Write proxy for field `QOS3`"]
pub struct QOS3_W<'a> {
    w: &'a mut W,
}
impl<'a> QOS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QOS3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `RRLVLEN3`"]
pub type RRLVLEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRLVLEN3`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
