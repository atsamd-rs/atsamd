#[doc = "Register `FERACES` writer"]
pub struct W(crate::W<FERACES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FERACES_SPEC>;
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
impl From<crate::W<FERACES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FERACES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force Event for Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD12NE_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMD12NE_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMD12NE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD12NE` writer - Force Event for Auto CMD12 Not Executed"]
pub struct ACMD12NE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMD12NE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMD12NE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMD12NE_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMD12NE_AW::YES)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Force Event for Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDTEO_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDTEO_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDTEO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDTEO` writer - Force Event for Auto CMD Timeout Error"]
pub struct ACMDTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMDTEO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMDTEO_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDTEO_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDTEO_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Force Event for Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDCRC_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDCRC_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDCRC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDCRC` writer - Force Event for Auto CMD CRC Error"]
pub struct ACMDCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMDCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMDCRC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDCRC_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDCRC_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Force Event for Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDEND_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDEND_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDEND` writer - Force Event for Auto CMD End Bit Error"]
pub struct ACMDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMDEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMDEND_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDEND_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDEND_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Force Event for Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDIDX_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDIDX_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDIDX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDIDX` writer - Force Event for Auto CMD Index Error"]
pub struct ACMDIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMDIDX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMDIDX_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDIDX_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDIDX_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Force Event for Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDNI_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDNI_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDNI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDNI` writer - Force Event for Command Not Issued By Auto CMD12 Error"]
pub struct CMDNI_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDNI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDNI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDNI_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDNI_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn acmd12ne(&mut self) -> ACMD12NE_W {
        ACMD12NE_W { w: self }
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn acmdteo(&mut self) -> ACMDTEO_W {
        ACMDTEO_W { w: self }
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn acmdcrc(&mut self) -> ACMDCRC_W {
        ACMDCRC_W { w: self }
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn acmdend(&mut self) -> ACMDEND_W {
        ACMDEND_W { w: self }
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn acmdidx(&mut self) -> ACMDIDX_W {
        ACMDIDX_W { w: self }
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cmdni(&mut self) -> CMDNI_W {
        CMDNI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event for Auto CMD Error Status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feraces](index.html) module"]
pub struct FERACES_SPEC;
impl crate::RegisterSpec for FERACES_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [feraces::W](W) writer structure"]
impl crate::Writable for FERACES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FERACES to value 0"]
impl crate::Resettable for FERACES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
