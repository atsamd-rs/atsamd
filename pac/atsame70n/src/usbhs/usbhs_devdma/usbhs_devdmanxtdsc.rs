#[doc = "Register `USBHS_DEVDMANXTDSC` reader"]
pub struct R(crate::R<USBHS_DEVDMANXTDSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVDMANXTDSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVDMANXTDSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVDMANXTDSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_DEVDMANXTDSC` writer"]
pub struct W(crate::W<USBHS_DEVDMANXTDSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVDMANXTDSC_SPEC>;
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
impl From<crate::W<USBHS_DEVDMANXTDSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVDMANXTDSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NXT_DSC_ADD` reader - Next Descriptor Address"]
pub struct NXT_DSC_ADD_R(crate::FieldReader<u32, u32>);
impl NXT_DSC_ADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NXT_DSC_ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NXT_DSC_ADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NXT_DSC_ADD` writer - Next Descriptor Address"]
pub struct NXT_DSC_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> NXT_DSC_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&self) -> NXT_DSC_ADD_R {
        NXT_DSC_ADD_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&mut self) -> NXT_DSC_ADD_W {
        NXT_DSC_ADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device DMA Channel Next Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devdmanxtdsc](index.html) module"]
pub struct USBHS_DEVDMANXTDSC_SPEC;
impl crate::RegisterSpec for USBHS_DEVDMANXTDSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_devdmanxtdsc::R](R) reader structure"]
impl crate::Readable for USBHS_DEVDMANXTDSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devdmanxtdsc::W](W) writer structure"]
impl crate::Writable for USBHS_DEVDMANXTDSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVDMANXTDSC to value 0"]
impl crate::Resettable for USBHS_DEVDMANXTDSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
