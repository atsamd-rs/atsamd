#[doc = "Writer for register DATABUF[%s]"]
pub type W = crate::W<u16, super::DATABUF>;
#[doc = "Register DATABUF[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DATABUF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATABUF`"]
pub struct DATABUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - DAC0 Data Buffer"]
    #[inline(always)]
    pub fn databuf(&mut self) -> DATABUF_W {
        DATABUF_W { w: self }
    }
}
