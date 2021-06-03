#[doc = "Writer for register DFLLSYNC"]
pub type W = crate::W<u8, super::DFLLSYNC>;
#[doc = "Register DFLLSYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLSYNC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `READREQ`"]
pub struct READREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> READREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    pub fn readreq(&mut self) -> READREQ_W {
        READREQ_W { w: self }
    }
}
