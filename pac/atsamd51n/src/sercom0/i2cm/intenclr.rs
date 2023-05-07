#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB` reader - Master On Bus Interrupt Disable"]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - Master On Bus Interrupt Disable"]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `SB` reader - Slave On Bus Interrupt Disable"]
pub type SB_R = crate::BitReader<bool>;
#[doc = "Field `SB` writer - Slave On Bus Interrupt Disable"]
pub type SB_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Disable"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Disable"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Master On Bus Interrupt Disable"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave On Bus Interrupt Disable"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master On Bus Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<0> {
        MB_W::new(self)
    }
    #[doc = "Bit 1 - Slave On Bus Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SB_W<1> {
        SB_W::new(self)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<7> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CM Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
