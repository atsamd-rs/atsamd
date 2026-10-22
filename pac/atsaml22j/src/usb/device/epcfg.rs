#[doc = "Register `EPCFG%s` reader"]
pub struct R(crate::R<EPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPCFG%s` writer"]
pub struct W(crate::W<EPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPCFG_SPEC>;
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
impl From<crate::W<EPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPTYPE0` reader - End Point Type0"]
pub type EPTYPE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTYPE0` writer - End Point Type0"]
pub type EPTYPE0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, EPCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `EPTYPE1` reader - End Point Type1"]
pub type EPTYPE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTYPE1` writer - End Point Type1"]
pub type EPTYPE1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, EPCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `NYETDIS` reader - NYET Token Disable"]
pub type NYETDIS_R = crate::BitReader<bool>;
#[doc = "Field `NYETDIS` writer - NYET Token Disable"]
pub type NYETDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&self) -> EPTYPE0_R {
        EPTYPE0_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&self) -> EPTYPE1_R {
        EPTYPE1_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    #[must_use]
    pub fn eptype0(&mut self) -> EPTYPE0_W<0> {
        EPTYPE0_W::new(self)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    #[must_use]
    pub fn eptype1(&mut self) -> EPTYPE1_W<4> {
        EPTYPE1_W::new(self)
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdis(&mut self) -> NYETDIS_W<7> {
        NYETDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcfg](index.html) module"]
pub struct EPCFG_SPEC;
impl crate::RegisterSpec for EPCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epcfg::R](R) reader structure"]
impl crate::Readable for EPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epcfg::W](W) writer structure"]
impl crate::Writable for EPCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPCFG%s to value 0"]
impl crate::Resettable for EPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
