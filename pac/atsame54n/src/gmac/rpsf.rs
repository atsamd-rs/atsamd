#[doc = "Register `RPSF` reader"]
pub struct R(crate::R<RPSF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPSF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPSF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPSF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPSF` writer"]
pub struct W(crate::W<RPSF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPSF_SPEC>;
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
impl From<crate::W<RPSF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPSF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPB1ADR` reader - RX packet buffer address"]
pub type RPB1ADR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RPB1ADR` writer - RX packet buffer address"]
pub type RPB1ADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RPSF_SPEC, u16, u16, 10, O>;
#[doc = "Field `ENRXP` reader - Enable RX partial store and forward operation"]
pub type ENRXP_R = crate::BitReader<bool>;
#[doc = "Field `ENRXP` writer - Enable RX partial store and forward operation"]
pub type ENRXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPSF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    pub fn rpb1adr(&self) -> RPB1ADR_R {
        RPB1ADR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn enrxp(&self) -> ENRXP_R {
        ENRXP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn rpb1adr(&mut self) -> RPB1ADR_W<0> {
        RPB1ADR_W::new(self)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn enrxp(&mut self) -> ENRXP_W<31> {
        ENRXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX partial store and forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpsf](index.html) module"]
pub struct RPSF_SPEC;
impl crate::RegisterSpec for RPSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpsf::R](R) reader structure"]
impl crate::Readable for RPSF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpsf::W](W) writer structure"]
impl crate::Writable for RPSF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPSF to value 0x03ff"]
impl crate::Resettable for RPSF_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
