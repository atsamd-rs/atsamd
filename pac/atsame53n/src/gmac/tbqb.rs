#[doc = "Register `TBQB` reader"]
pub struct R(crate::R<TBQB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBQB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBQB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBQB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBQB` writer"]
pub struct W(crate::W<TBQB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBQB_SPEC>;
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
impl From<crate::W<TBQB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBQB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Transmit Buffer Queue Base Address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Transmit Buffer Queue Base Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBQB_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<2> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Queue Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbqb](index.html) module"]
pub struct TBQB_SPEC;
impl crate::RegisterSpec for TBQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbqb::R](R) reader structure"]
impl crate::Readable for TBQB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbqb::W](W) writer structure"]
impl crate::Writable for TBQB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBQB to value 0"]
impl crate::Resettable for TBQB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
