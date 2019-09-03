#[doc = "Reader of register CALIB"]
pub type R = crate::R<u16, super::CALIB>;
#[doc = "Writer for register CALIB"]
pub type W = crate::W<u16, super::CALIB>;
#[doc = "Register CALIB `reset()`'s with value 0x0101"]
impl crate::ResetValue for super::CALIB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101
    }
}
#[doc = "Reader of field `BIAS0`"]
pub type BIAS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIAS0`"]
pub struct BIAS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - COMP0/1 Bias Scaling"]
    #[inline(always)]
    pub fn bias0(&self) -> BIAS0_R {
        BIAS0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - COMP0/1 Bias Scaling"]
    #[inline(always)]
    pub fn bias0(&mut self) -> BIAS0_W {
        BIAS0_W { w: self }
    }
}
