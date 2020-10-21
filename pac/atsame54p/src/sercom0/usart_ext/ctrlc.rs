#[doc = "Reader of register CTRLC"]
pub type R = crate::R<u32, super::CTRLC>;
#[doc = "Writer for register CTRLC"]
pub type W = crate::W<u32, super::CTRLC>;
#[doc = "Register CTRLC `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GTIME`"]
pub type GTIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GTIME`"]
pub struct GTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> GTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BRKLEN`"]
pub type BRKLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRKLEN`"]
pub struct BRKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `HDRDLY`"]
pub type HDRDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDRDLY`"]
pub struct HDRDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `INACK`"]
pub type INACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INACK`"]
pub struct INACK_W<'a> {
    w: &'a mut W,
}
impl<'a> INACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DSNACK`"]
pub type DSNACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSNACK`"]
pub struct DSNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSNACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MAXITER`"]
pub type MAXITER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXITER`"]
pub struct MAXITER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXITER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA32B_A {
    #[doc = "0: Data reads and writes according CTRLB.CHSIZE"]
    DATA_READ_WRITE_CHSIZE = 0,
    #[doc = "1: Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    DATA_READ_CHSIZE_WRITE_32BIT = 1,
    #[doc = "2: Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    DATA_READ_32BIT_WRITE_CHSIZE = 2,
    #[doc = "3: Data reads and writes according 32-bit extension"]
    DATA_READ_WRITE_32BIT = 3,
}
impl From<DATA32B_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA32B_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATA32B`"]
pub type DATA32B_R = crate::R<u8, DATA32B_A>;
impl DATA32B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA32B_A {
        match self.bits {
            0 => DATA32B_A::DATA_READ_WRITE_CHSIZE,
            1 => DATA32B_A::DATA_READ_CHSIZE_WRITE_32BIT,
            2 => DATA32B_A::DATA_READ_32BIT_WRITE_CHSIZE,
            3 => DATA32B_A::DATA_READ_WRITE_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA_READ_WRITE_CHSIZE`"]
    #[inline(always)]
    pub fn is_data_read_write_chsize(&self) -> bool {
        *self == DATA32B_A::DATA_READ_WRITE_CHSIZE
    }
    #[doc = "Checks if the value of the field is `DATA_READ_CHSIZE_WRITE_32BIT`"]
    #[inline(always)]
    pub fn is_data_read_chsize_write_32bit(&self) -> bool {
        *self == DATA32B_A::DATA_READ_CHSIZE_WRITE_32BIT
    }
    #[doc = "Checks if the value of the field is `DATA_READ_32BIT_WRITE_CHSIZE`"]
    #[inline(always)]
    pub fn is_data_read_32bit_write_chsize(&self) -> bool {
        *self == DATA32B_A::DATA_READ_32BIT_WRITE_CHSIZE
    }
    #[doc = "Checks if the value of the field is `DATA_READ_WRITE_32BIT`"]
    #[inline(always)]
    pub fn is_data_read_write_32bit(&self) -> bool {
        *self == DATA32B_A::DATA_READ_WRITE_32BIT
    }
}
#[doc = "Write proxy for field `DATA32B`"]
pub struct DATA32B_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA32B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA32B_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data reads and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_write_chsize(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_WRITE_CHSIZE)
    }
    #[doc = "Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_chsize_write_32bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_CHSIZE_WRITE_32BIT)
    }
    #[doc = "Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_32bit_write_chsize(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_32BIT_WRITE_CHSIZE)
    }
    #[doc = "Data reads and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_write_32bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_WRITE_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GTIME_R {
        GTIME_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BRKLEN_R {
        BRKLEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HDRDLY_R {
        HDRDLY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&self) -> MAXITER_R {
        MAXITER_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> DATA32B_R {
        DATA32B_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&mut self) -> GTIME_W {
        GTIME_W { w: self }
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&mut self) -> BRKLEN_W {
        BRKLEN_W { w: self }
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&mut self) -> HDRDLY_W {
        HDRDLY_W { w: self }
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&mut self) -> INACK_W {
        INACK_W { w: self }
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&mut self) -> DSNACK_W {
        DSNACK_W { w: self }
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&mut self) -> MAXITER_W {
        MAXITER_W { w: self }
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&mut self) -> DATA32B_W {
        DATA32B_W { w: self }
    }
}
