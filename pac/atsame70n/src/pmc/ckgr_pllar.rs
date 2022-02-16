#[doc = "Register `CKGR_PLLAR` reader"]
pub struct R(crate::R<CKGR_PLLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_PLLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_PLLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_PLLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_PLLAR` writer"]
pub struct W(crate::W<CKGR_PLLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_PLLAR_SPEC>;
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
impl From<crate::W<CKGR_PLLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_PLLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLLA Front End Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: Divider output is 0 and PLLA is disabled."]
    _0 = 0,
    #[doc = "1: Divider is bypassed (divide by 1) and PLLA is enabled."]
    BYPASS = 1,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVA` reader - PLLA Front End Divider"]
pub struct DIVA_R(crate::FieldReader<u8, DIVA_A>);
impl DIVA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVA_A> {
        match self.bits {
            0 => Some(DIVA_A::_0),
            1 => Some(DIVA_A::BYPASS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIVA_A::_0
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == DIVA_A::BYPASS
    }
}
impl core::ops::Deref for DIVA_R {
    type Target = crate::FieldReader<u8, DIVA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVA` writer - PLLA Front End Divider"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divider output is 0 and PLLA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIVA_A::_0)
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(DIVA_A::BYPASS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PLLACOUNT` reader - PLLA Counter"]
pub struct PLLACOUNT_R(crate::FieldReader<u8, u8>);
impl PLLACOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLLACOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLACOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLACOUNT` writer - PLLA Counter"]
pub struct PLLACOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLACOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `MULA` reader - PLLA Multiplier"]
pub struct MULA_R(crate::FieldReader<u16, u16>);
impl MULA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MULA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULA` writer - PLLA Multiplier"]
pub struct MULA_W<'a> {
    w: &'a mut W,
}
impl<'a> MULA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub struct ONE_R(crate::FieldReader<bool, bool>);
impl ONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub struct ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&self) -> PLLACOUNT_R {
        PLLACOUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&self) -> MULA_R {
        MULA_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&mut self) -> PLLACOUNT_W {
        PLLACOUNT_W { w: self }
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&mut self) -> MULA_W {
        MULA_W { w: self }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> ONE_W {
        ONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_pllar](index.html) module"]
pub struct CKGR_PLLAR_SPEC;
impl crate::RegisterSpec for CKGR_PLLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_pllar::R](R) reader structure"]
impl crate::Readable for CKGR_PLLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_pllar::W](W) writer structure"]
impl crate::Writable for CKGR_PLLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGR_PLLAR to value 0"]
impl crate::Resettable for CKGR_PLLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
