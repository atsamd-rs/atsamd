#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u32, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u32, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PID0`"]
pub type PID0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID0`"]
pub struct PID0_W<'a> {
    w: &'a mut W,
}
impl<'a> PID0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "PORT Event Action 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT0_A {
    #[doc = "0: Event output to pin"]
    OUT = 0,
    #[doc = "1: Set output register of pin on event"]
    SET = 1,
    #[doc = "2: Clear output register of pin on event"]
    CLR = 2,
    #[doc = "3: Toggle output register of pin on event"]
    TGL = 3,
}
impl From<EVACT0_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVACT0`"]
pub type EVACT0_R = crate::R<u8, EVACT0_A>;
impl EVACT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT0_A {
        match self.bits {
            0 => EVACT0_A::OUT,
            1 => EVACT0_A::SET,
            2 => EVACT0_A::CLR,
            3 => EVACT0_A::TGL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EVACT0_A::OUT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EVACT0_A::SET
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == EVACT0_A::CLR
    }
    #[doc = "Checks if the value of the field is `TGL`"]
    #[inline(always)]
    pub fn is_tgl(&self) -> bool {
        *self == EVACT0_A::TGL
    }
}
#[doc = "Write proxy for field `EVACT0`"]
pub struct EVACT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Event output to pin"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EVACT0_A::OUT)
    }
    #[doc = "Set output register of pin on event"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EVACT0_A::SET)
    }
    #[doc = "Clear output register of pin on event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(EVACT0_A::CLR)
    }
    #[doc = "Toggle output register of pin on event"]
    #[inline(always)]
    pub fn tgl(self) -> &'a mut W {
        self.variant(EVACT0_A::TGL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `PORTEI0`"]
pub type PORTEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTEI0`"]
pub struct PORTEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTEI0_W<'a> {
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
#[doc = "Reader of field `PID1`"]
pub type PID1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID1`"]
pub struct PID1_W<'a> {
    w: &'a mut W,
}
impl<'a> PID1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EVACT1`"]
pub type EVACT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVACT1`"]
pub struct EVACT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `PORTEI1`"]
pub type PORTEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTEI1`"]
pub struct PORTEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTEI1_W<'a> {
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
#[doc = "Reader of field `PID2`"]
pub type PID2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID2`"]
pub struct PID2_W<'a> {
    w: &'a mut W,
}
impl<'a> PID2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EVACT2`"]
pub type EVACT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVACT2`"]
pub struct EVACT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `PORTEI2`"]
pub type PORTEI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTEI2`"]
pub struct PORTEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTEI2_W<'a> {
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
#[doc = "Reader of field `PID3`"]
pub type PID3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID3`"]
pub struct PID3_W<'a> {
    w: &'a mut W,
}
impl<'a> PID3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `EVACT3`"]
pub type EVACT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVACT3`"]
pub struct EVACT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `PORTEI3`"]
pub type PORTEI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTEI3`"]
pub struct PORTEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTEI3_W<'a> {
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
    #[doc = "Bits 0:4 - PORT Event Pin Identifier 0"]
    #[inline(always)]
    pub fn pid0(&self) -> PID0_R {
        PID0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - PORT Event Action 0"]
    #[inline(always)]
    pub fn evact0(&self) -> EVACT0_R {
        EVACT0_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - PORT Event Input Enable 0"]
    #[inline(always)]
    pub fn portei0(&self) -> PORTEI0_R {
        PORTEI0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - PORT Event Pin Identifier 1"]
    #[inline(always)]
    pub fn pid1(&self) -> PID1_R {
        PID1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - PORT Event Action 1"]
    #[inline(always)]
    pub fn evact1(&self) -> EVACT1_R {
        EVACT1_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - PORT Event Input Enable 1"]
    #[inline(always)]
    pub fn portei1(&self) -> PORTEI1_R {
        PORTEI1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - PORT Event Pin Identifier 2"]
    #[inline(always)]
    pub fn pid2(&self) -> PID2_R {
        PID2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - PORT Event Action 2"]
    #[inline(always)]
    pub fn evact2(&self) -> EVACT2_R {
        EVACT2_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - PORT Event Input Enable 2"]
    #[inline(always)]
    pub fn portei2(&self) -> PORTEI2_R {
        PORTEI2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - PORT Event Pin Identifier 3"]
    #[inline(always)]
    pub fn pid3(&self) -> PID3_R {
        PID3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - PORT Event Action 3"]
    #[inline(always)]
    pub fn evact3(&self) -> EVACT3_R {
        EVACT3_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 31 - PORT Event Input Enable 3"]
    #[inline(always)]
    pub fn portei3(&self) -> PORTEI3_R {
        PORTEI3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PORT Event Pin Identifier 0"]
    #[inline(always)]
    pub fn pid0(&mut self) -> PID0_W {
        PID0_W { w: self }
    }
    #[doc = "Bits 5:6 - PORT Event Action 0"]
    #[inline(always)]
    pub fn evact0(&mut self) -> EVACT0_W {
        EVACT0_W { w: self }
    }
    #[doc = "Bit 7 - PORT Event Input Enable 0"]
    #[inline(always)]
    pub fn portei0(&mut self) -> PORTEI0_W {
        PORTEI0_W { w: self }
    }
    #[doc = "Bits 8:12 - PORT Event Pin Identifier 1"]
    #[inline(always)]
    pub fn pid1(&mut self) -> PID1_W {
        PID1_W { w: self }
    }
    #[doc = "Bits 13:14 - PORT Event Action 1"]
    #[inline(always)]
    pub fn evact1(&mut self) -> EVACT1_W {
        EVACT1_W { w: self }
    }
    #[doc = "Bit 15 - PORT Event Input Enable 1"]
    #[inline(always)]
    pub fn portei1(&mut self) -> PORTEI1_W {
        PORTEI1_W { w: self }
    }
    #[doc = "Bits 16:20 - PORT Event Pin Identifier 2"]
    #[inline(always)]
    pub fn pid2(&mut self) -> PID2_W {
        PID2_W { w: self }
    }
    #[doc = "Bits 21:22 - PORT Event Action 2"]
    #[inline(always)]
    pub fn evact2(&mut self) -> EVACT2_W {
        EVACT2_W { w: self }
    }
    #[doc = "Bit 23 - PORT Event Input Enable 2"]
    #[inline(always)]
    pub fn portei2(&mut self) -> PORTEI2_W {
        PORTEI2_W { w: self }
    }
    #[doc = "Bits 24:28 - PORT Event Pin Identifier 3"]
    #[inline(always)]
    pub fn pid3(&mut self) -> PID3_W {
        PID3_W { w: self }
    }
    #[doc = "Bits 29:30 - PORT Event Action 3"]
    #[inline(always)]
    pub fn evact3(&mut self) -> EVACT3_W {
        EVACT3_W { w: self }
    }
    #[doc = "Bit 31 - PORT Event Input Enable 3"]
    #[inline(always)]
    pub fn portei3(&mut self) -> PORTEI3_W {
        PORTEI3_W { w: self }
    }
}
