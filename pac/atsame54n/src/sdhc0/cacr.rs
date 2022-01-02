#[doc = "Register `CACR` reader"]
pub struct R(crate::R<CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACR` writer"]
pub struct W(crate::W<CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACR_SPEC>;
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
impl From<crate::W<CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPWREN` reader - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub struct CAPWREN_R(crate::FieldReader<bool, bool>);
impl CAPWREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPWREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPWREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPWREN` writer - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub struct CAPWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPWREN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `KEY` reader - Key (0x46)"]
pub struct KEY_R(crate::FieldReader<u8, u8>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Key (0x46)"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    pub fn capwren(&self) -> CAPWREN_R {
        CAPWREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    pub fn capwren(&mut self) -> CAPWREN_W {
        CAPWREN_W { w: self }
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capabilities Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](index.html) module"]
pub struct CACR_SPEC;
impl crate::RegisterSpec for CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacr::R](R) reader structure"]
impl crate::Readable for CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacr::W](W) writer structure"]
impl crate::Writable for CACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
