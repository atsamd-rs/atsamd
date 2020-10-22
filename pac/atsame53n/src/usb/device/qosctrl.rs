#[doc = "Reader of register QOSCTRL"]
pub type R = crate::R<u8, super::QOSCTRL>;
#[doc = "Writer for register QOSCTRL"]
pub type W = crate::W<u8, super::QOSCTRL>;
#[doc = "Register QOSCTRL `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::QOSCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `CQOS`"]
pub type CQOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQOS`"]
pub struct CQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> CQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DQOS`"]
pub type DQOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQOS`"]
pub struct DQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&self) -> CQOS_R {
        CQOS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DQOS_R {
        DQOS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&mut self) -> CQOS_W {
        CQOS_W { w: self }
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&mut self) -> DQOS_W {
        DQOS_W { w: self }
    }
}
