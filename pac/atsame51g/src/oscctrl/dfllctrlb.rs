#[doc = "Register `DFLLCTRLB` reader"]
pub struct R(crate::R<DFLLCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLCTRLB` writer"]
pub struct W(crate::W<DFLLCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLCTRLB_SPEC>;
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
impl From<crate::W<DFLLCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Operating Mode Selection"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Operating Mode Selection"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
#[doc = "Field `STABLE` reader - Stable DFLL Frequency"]
pub type STABLE_R = crate::BitReader<bool>;
#[doc = "Field `STABLE` writer - Stable DFLL Frequency"]
pub type STABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
#[doc = "Field `LLAW` reader - Lose Lock After Wake"]
pub type LLAW_R = crate::BitReader<bool>;
#[doc = "Field `LLAW` writer - Lose Lock After Wake"]
pub type LLAW_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
#[doc = "Field `USBCRM` reader - USB Clock Recovery Mode"]
pub type USBCRM_R = crate::BitReader<bool>;
#[doc = "Field `USBCRM` writer - USB Clock Recovery Mode"]
pub type USBCRM_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
#[doc = "Field `CCDIS` reader - Chill Cycle Disable"]
pub type CCDIS_R = crate::BitReader<bool>;
#[doc = "Field `CCDIS` writer - Chill Cycle Disable"]
pub type CCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
#[doc = "Field `QLDIS` reader - Quick Lock Disable"]
pub type QLDIS_R = crate::BitReader<bool>;
#[doc = "Field `QLDIS` writer - Quick Lock Disable"]
pub type QLDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
#[doc = "Field `BPLCKC` reader - Bypass Coarse Lock"]
pub type BPLCKC_R = crate::BitReader<bool>;
#[doc = "Field `BPLCKC` writer - Bypass Coarse Lock"]
pub type BPLCKC_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
#[doc = "Field `WAITLOCK` reader - Wait Lock"]
pub type WAITLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WAITLOCK` writer - Wait Lock"]
pub type WAITLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLCTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LLAW_R {
        LLAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&self) -> USBCRM_R {
        USBCRM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CCDIS_R {
        CCDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QLDIS_R {
        QLDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&self) -> BPLCKC_R {
        BPLCKC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&self) -> WAITLOCK_R {
        WAITLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn stable(&mut self) -> STABLE_W<1> {
        STABLE_W::new(self)
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    #[must_use]
    pub fn llaw(&mut self) -> LLAW_W<2> {
        LLAW_W::new(self)
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbcrm(&mut self) -> USBCRM_W<3> {
        USBCRM_W::new(self)
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccdis(&mut self) -> CCDIS_W<4> {
        CCDIS_W::new(self)
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn qldis(&mut self) -> QLDIS_W<5> {
        QLDIS_W::new(self)
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    #[must_use]
    pub fn bplckc(&mut self) -> BPLCKC_W<6> {
        BPLCKC_W::new(self)
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    #[must_use]
    pub fn waitlock(&mut self) -> WAITLOCK_W<7> {
        WAITLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrlb](index.html) module"]
pub struct DFLLCTRLB_SPEC;
impl crate::RegisterSpec for DFLLCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dfllctrlb::R](R) reader structure"]
impl crate::Readable for DFLLCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllctrlb::W](W) writer structure"]
impl crate::Writable for DFLLCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLCTRLB to value 0"]
impl crate::Resettable for DFLLCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
