#[doc = "Register `DACC_ACR` reader"]
pub struct R(crate::R<DACC_ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACC_ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACC_ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACC_ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACC_ACR` writer"]
pub struct W(crate::W<DACC_ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACC_ACR_SPEC>;
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
impl From<crate::W<DACC_ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACC_ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBCTLCH0` reader - Analog Output Current Control"]
pub struct IBCTLCH0_R(crate::FieldReader<u8, u8>);
impl IBCTLCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IBCTLCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBCTLCH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBCTLCH0` writer - Analog Output Current Control"]
pub struct IBCTLCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IBCTLCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `IBCTLCH1` reader - Analog Output Current Control"]
pub struct IBCTLCH1_R(crate::FieldReader<u8, u8>);
impl IBCTLCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IBCTLCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBCTLCH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBCTLCH1` writer - Analog Output Current Control"]
pub struct IBCTLCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> IBCTLCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch0(&self) -> IBCTLCH0_R {
        IBCTLCH0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch1(&self) -> IBCTLCH1_R {
        IBCTLCH1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch0(&mut self) -> IBCTLCH0_W {
        IBCTLCH0_W { w: self }
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch1(&mut self) -> IBCTLCH1_W {
        IBCTLCH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Current Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_acr](index.html) module"]
pub struct DACC_ACR_SPEC;
impl crate::RegisterSpec for DACC_ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacc_acr::R](R) reader structure"]
impl crate::Readable for DACC_ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacc_acr::W](W) writer structure"]
impl crate::Writable for DACC_ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACC_ACR to value 0"]
impl crate::Resettable for DACC_ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
