#[doc = "Register `CRCSTATUS` reader"]
pub struct R(crate::R<CRCSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCSTATUS` writer"]
pub struct W(crate::W<CRCSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCSTATUS_SPEC>;
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
impl From<crate::W<CRCSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCBUSY` reader - CRC Module Busy"]
pub type CRCBUSY_R = crate::BitReader<bool>;
#[doc = "Field `CRCBUSY` writer - CRC Module Busy"]
pub type CRCBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CRCSTATUS_SPEC, bool, O>;
#[doc = "Field `CRCZERO` reader - CRC Zero"]
pub type CRCZERO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&self) -> CRCBUSY_R {
        CRCBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Zero"]
    #[inline(always)]
    pub fn crczero(&self) -> CRCZERO_R {
        CRCZERO_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    #[must_use]
    pub fn crcbusy(&mut self) -> CRCBUSY_W<0> {
        CRCBUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcstatus](index.html) module"]
pub struct CRCSTATUS_SPEC;
impl crate::RegisterSpec for CRCSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crcstatus::R](R) reader structure"]
impl crate::Readable for CRCSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcstatus::W](W) writer structure"]
impl crate::Writable for CRCSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCSTATUS to value 0"]
impl crate::Resettable for CRCSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
