#[doc = "Register `CHID` reader"]
pub struct R(crate::R<CHID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHID` writer"]
pub struct W(crate::W<CHID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHID_SPEC>;
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
impl From<crate::W<CHID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Channel ID"]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` writer - Channel ID"]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CHID_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chid](index.html) module"]
pub struct CHID_SPEC;
impl crate::RegisterSpec for CHID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chid::R](R) reader structure"]
impl crate::Readable for CHID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chid::W](W) writer structure"]
impl crate::Writable for CHID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHID to value 0"]
impl crate::Resettable for CHID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
