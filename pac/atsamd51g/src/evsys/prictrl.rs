#[doc = "Register `PRICTRL` reader"]
pub struct R(crate::R<PRICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRICTRL` writer"]
pub struct W(crate::W<PRICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRICTRL_SPEC>;
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
impl From<crate::W<PRICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI` reader - Channel Priority Number"]
pub struct PRI_R(crate::FieldReader<u8, u8>);
impl PRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI` writer - Channel Priority Number"]
pub struct PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Field `RREN` reader - Round-Robin Scheduling Enable"]
pub struct RREN_R(crate::FieldReader<bool, bool>);
impl RREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RREN` writer - Round-Robin Scheduling Enable"]
pub struct RREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rren(&self) -> RREN_R {
        RREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    pub fn pri(&mut self) -> PRI_W {
        PRI_W { w: self }
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rren(&mut self) -> RREN_W {
        RREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prictrl](index.html) module"]
pub struct PRICTRL_SPEC;
impl crate::RegisterSpec for PRICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prictrl::R](R) reader structure"]
impl crate::Readable for PRICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prictrl::W](W) writer structure"]
impl crate::Writable for PRICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRICTRL to value 0"]
impl crate::Resettable for PRICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
