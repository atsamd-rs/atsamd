#[doc = "Register `INTPEND` reader"]
pub struct R(crate::R<INTPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPEND` writer"]
pub struct W(crate::W<INTPEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPEND_SPEC>;
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
impl From<crate::W<INTPEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Channel ID"]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` writer - Channel ID"]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u16, INTPEND_SPEC, u8, u8, 4, O>;
#[doc = "Field `TERR` reader - Transfer Error"]
pub type TERR_R = crate::BitReader<bool>;
#[doc = "Field `TERR` writer - Transfer Error"]
pub type TERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTPEND_SPEC, bool, O>;
#[doc = "Field `TCMPL` reader - Transfer Complete"]
pub type TCMPL_R = crate::BitReader<bool>;
#[doc = "Field `TCMPL` writer - Transfer Complete"]
pub type TCMPL_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTPEND_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTPEND_SPEC, bool, O>;
#[doc = "Field `FERR` reader - Fetch Error"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `PEND` reader - Pending"]
pub type PEND_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<8> {
        TERR_W::new(self)
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpl(&mut self) -> TCMPL_W<9> {
        TCMPL_W::new(self)
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<10> {
        SUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](index.html) module"]
pub struct INTPEND_SPEC;
impl crate::RegisterSpec for INTPEND_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intpend::R](R) reader structure"]
impl crate::Readable for INTPEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intpend::W](W) writer structure"]
impl crate::Writable for INTPEND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for INTPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
