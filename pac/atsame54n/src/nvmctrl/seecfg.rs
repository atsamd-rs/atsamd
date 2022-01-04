#[doc = "Register `SEECFG` reader"]
pub struct R(crate::R<SEECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEECFG` writer"]
pub struct W(crate::W<SEECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEECFG_SPEC>;
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
impl From<crate::W<SEECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMODE_A {
    #[doc = "0: A NVM write command is issued after each write in the pagebuffer"]
    UNBUFFERED = 0,
    #[doc = "1: A NVM write command is issued when a write to a new page is requested"]
    BUFFERED = 1,
}
impl From<WMODE_A> for bool {
    #[inline(always)]
    fn from(variant: WMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WMODE` reader - Write Mode"]
pub struct WMODE_R(crate::FieldReader<bool, WMODE_A>);
impl WMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMODE_A {
        match self.bits {
            false => WMODE_A::UNBUFFERED,
            true => WMODE_A::BUFFERED,
        }
    }
    #[doc = "Checks if the value of the field is `UNBUFFERED`"]
    #[inline(always)]
    pub fn is_unbuffered(&self) -> bool {
        **self == WMODE_A::UNBUFFERED
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        **self == WMODE_A::BUFFERED
    }
}
impl core::ops::Deref for WMODE_R {
    type Target = crate::FieldReader<bool, WMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WMODE` writer - Write Mode"]
pub struct WMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline(always)]
    pub fn unbuffered(self) -> &'a mut W {
        self.variant(WMODE_A::UNBUFFERED)
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut W {
        self.variant(WMODE_A::BUFFERED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `APRDIS` reader - Automatic Page Reallocation Disable"]
pub struct APRDIS_R(crate::FieldReader<bool, bool>);
impl APRDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APRDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APRDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APRDIS` writer - Automatic Page Reallocation Disable"]
pub struct APRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APRDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WMODE_R {
        WMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&self) -> APRDIS_R {
        APRDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&mut self) -> WMODE_W {
        WMODE_W { w: self }
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&mut self) -> APRDIS_W {
        APRDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SmartEEPROM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seecfg](index.html) module"]
pub struct SEECFG_SPEC;
impl crate::RegisterSpec for SEECFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seecfg::R](R) reader structure"]
impl crate::Readable for SEECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seecfg::W](W) writer structure"]
impl crate::Writable for SEECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEECFG to value 0"]
impl crate::Resettable for SEECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
