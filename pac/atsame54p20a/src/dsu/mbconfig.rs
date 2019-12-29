#[doc = "Reader of register MBCONFIG"]
pub type R = crate::R<u32, super::MBCONFIG>;
#[doc = "Writer for register MBCONFIG"]
pub type W = crate::W<u32, super::MBCONFIG>;
#[doc = "Register MBCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::MBCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MBIST Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALGO_A {
    #[doc = "0: Memory Clear (1n)"]
    MEMCLEAR = 0,
    #[doc = "1: Memory Verify (1n)"]
    VERIFY = 1,
    #[doc = "2: Memory Clear and Verify (2n)"]
    CLEARVER = 2,
    #[doc = "3: Address Decoder (2n)"]
    ADDR_DEC = 3,
    #[doc = "4: March LR (14n)"]
    MARCH_LR = 4,
    #[doc = "5: March SR (14n)"]
    MARCH_SR = 5,
    #[doc = "6: March SS (22n)"]
    MARCH_SS = 6,
    #[doc = "8: CRC increasing address (1n)"]
    CRC_UP = 8,
    #[doc = "9: CRC decreasing address (1n)"]
    CRC_DOWN = 9,
}
impl From<ALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: ALGO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALGO`"]
pub type ALGO_R = crate::R<u8, ALGO_A>;
impl ALGO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALGO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ALGO_A::MEMCLEAR),
            1 => Val(ALGO_A::VERIFY),
            2 => Val(ALGO_A::CLEARVER),
            3 => Val(ALGO_A::ADDR_DEC),
            4 => Val(ALGO_A::MARCH_LR),
            5 => Val(ALGO_A::MARCH_SR),
            6 => Val(ALGO_A::MARCH_SS),
            8 => Val(ALGO_A::CRC_UP),
            9 => Val(ALGO_A::CRC_DOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MEMCLEAR`"]
    #[inline(always)]
    pub fn is_memclear(&self) -> bool {
        *self == ALGO_A::MEMCLEAR
    }
    #[doc = "Checks if the value of the field is `VERIFY`"]
    #[inline(always)]
    pub fn is_verify(&self) -> bool {
        *self == ALGO_A::VERIFY
    }
    #[doc = "Checks if the value of the field is `CLEARVER`"]
    #[inline(always)]
    pub fn is_clearver(&self) -> bool {
        *self == ALGO_A::CLEARVER
    }
    #[doc = "Checks if the value of the field is `ADDR_DEC`"]
    #[inline(always)]
    pub fn is_addr_dec(&self) -> bool {
        *self == ALGO_A::ADDR_DEC
    }
    #[doc = "Checks if the value of the field is `MARCH_LR`"]
    #[inline(always)]
    pub fn is_march_lr(&self) -> bool {
        *self == ALGO_A::MARCH_LR
    }
    #[doc = "Checks if the value of the field is `MARCH_SR`"]
    #[inline(always)]
    pub fn is_march_sr(&self) -> bool {
        *self == ALGO_A::MARCH_SR
    }
    #[doc = "Checks if the value of the field is `MARCH_SS`"]
    #[inline(always)]
    pub fn is_march_ss(&self) -> bool {
        *self == ALGO_A::MARCH_SS
    }
    #[doc = "Checks if the value of the field is `CRC_UP`"]
    #[inline(always)]
    pub fn is_crc_up(&self) -> bool {
        *self == ALGO_A::CRC_UP
    }
    #[doc = "Checks if the value of the field is `CRC_DOWN`"]
    #[inline(always)]
    pub fn is_crc_down(&self) -> bool {
        *self == ALGO_A::CRC_DOWN
    }
}
#[doc = "Write proxy for field `ALGO`"]
pub struct ALGO_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALGO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Memory Clear (1n)"]
    #[inline(always)]
    pub fn memclear(self) -> &'a mut W {
        self.variant(ALGO_A::MEMCLEAR)
    }
    #[doc = "Memory Verify (1n)"]
    #[inline(always)]
    pub fn verify(self) -> &'a mut W {
        self.variant(ALGO_A::VERIFY)
    }
    #[doc = "Memory Clear and Verify (2n)"]
    #[inline(always)]
    pub fn clearver(self) -> &'a mut W {
        self.variant(ALGO_A::CLEARVER)
    }
    #[doc = "Address Decoder (2n)"]
    #[inline(always)]
    pub fn addr_dec(self) -> &'a mut W {
        self.variant(ALGO_A::ADDR_DEC)
    }
    #[doc = "March LR (14n)"]
    #[inline(always)]
    pub fn march_lr(self) -> &'a mut W {
        self.variant(ALGO_A::MARCH_LR)
    }
    #[doc = "March SR (14n)"]
    #[inline(always)]
    pub fn march_sr(self) -> &'a mut W {
        self.variant(ALGO_A::MARCH_SR)
    }
    #[doc = "March SS (22n)"]
    #[inline(always)]
    pub fn march_ss(self) -> &'a mut W {
        self.variant(ALGO_A::MARCH_SS)
    }
    #[doc = "CRC increasing address (1n)"]
    #[inline(always)]
    pub fn crc_up(self) -> &'a mut W {
        self.variant(ALGO_A::CRC_UP)
    }
    #[doc = "CRC decreasing address (1n)"]
    #[inline(always)]
    pub fn crc_down(self) -> &'a mut W {
        self.variant(ALGO_A::CRC_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DEFRDMARGIN`"]
pub type DEFRDMARGIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEFRDMARGIN`"]
pub struct DEFRDMARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFRDMARGIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DBG`"]
pub type DBG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG`"]
pub struct DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - MBIST Algorithm"]
    #[inline(always)]
    pub fn algo(&self) -> ALGO_R {
        ALGO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Force Default Read Margin"]
    #[inline(always)]
    pub fn defrdmargin(&self) -> DEFRDMARGIN_R {
        DEFRDMARGIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Debug Mode"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - MBIST Algorithm"]
    #[inline(always)]
    pub fn algo(&mut self) -> ALGO_W {
        ALGO_W { w: self }
    }
    #[doc = "Bit 6 - Force Default Read Margin"]
    #[inline(always)]
    pub fn defrdmargin(&mut self) -> DEFRDMARGIN_W {
        DEFRDMARGIN_W { w: self }
    }
    #[doc = "Bit 7 - Enable Debug Mode"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DBG_W {
        DBG_W { w: self }
    }
}
