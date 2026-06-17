#[doc = "Register `TAMPID` reader"]
pub struct R(crate::R<TAMPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPID` writer"]
pub struct W(crate::W<TAMPID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPID_SPEC>;
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
impl From<crate::W<TAMPID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMPID0` reader - Tamper Input 0 Detected"]
pub type TAMPID0_R = crate::BitReader<bool>;
#[doc = "Field `TAMPID0` writer - Tamper Input 0 Detected"]
pub type TAMPID0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPID_SPEC, bool, O>;
#[doc = "Field `TAMPID1` reader - Tamper Input 1 Detected"]
pub type TAMPID1_R = crate::BitReader<bool>;
#[doc = "Field `TAMPID1` writer - Tamper Input 1 Detected"]
pub type TAMPID1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPID_SPEC, bool, O>;
#[doc = "Field `TAMPID2` reader - Tamper Input 2 Detected"]
pub type TAMPID2_R = crate::BitReader<bool>;
#[doc = "Field `TAMPID2` writer - Tamper Input 2 Detected"]
pub type TAMPID2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPID_SPEC, bool, O>;
#[doc = "Field `TAMPID3` reader - Tamper Input 3 Detected"]
pub type TAMPID3_R = crate::BitReader<bool>;
#[doc = "Field `TAMPID3` writer - Tamper Input 3 Detected"]
pub type TAMPID3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPID_SPEC, bool, O>;
#[doc = "Field `TAMPID4` reader - Tamper Input 4 Detected"]
pub type TAMPID4_R = crate::BitReader<bool>;
#[doc = "Field `TAMPID4` writer - Tamper Input 4 Detected"]
pub type TAMPID4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPID_SPEC, bool, O>;
#[doc = "Field `TAMPEVT` reader - Tamper Event Detected"]
pub type TAMPEVT_R = crate::BitReader<bool>;
#[doc = "Field `TAMPEVT` writer - Tamper Event Detected"]
pub type TAMPEVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPID_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&self) -> TAMPID0_R {
        TAMPID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&self) -> TAMPID1_R {
        TAMPID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&self) -> TAMPID2_R {
        TAMPID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&self) -> TAMPID3_R {
        TAMPID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&self) -> TAMPID4_R {
        TAMPID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&self) -> TAMPEVT_R {
        TAMPEVT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid0(&mut self) -> TAMPID0_W<0> {
        TAMPID0_W::new(self)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid1(&mut self) -> TAMPID1_W<1> {
        TAMPID1_W::new(self)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid2(&mut self) -> TAMPID2_W<2> {
        TAMPID2_W::new(self)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid3(&mut self) -> TAMPID3_W<3> {
        TAMPID3_W::new(self)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampid4(&mut self) -> TAMPID4_W<4> {
        TAMPID4_W::new(self)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tampevt(&mut self) -> TAMPEVT_W<31> {
        TAMPEVT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampid](index.html) module"]
pub struct TAMPID_SPEC;
impl crate::RegisterSpec for TAMPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampid::R](R) reader structure"]
impl crate::Readable for TAMPID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampid::W](W) writer structure"]
impl crate::Writable for TAMPID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMPID to value 0"]
impl crate::Resettable for TAMPID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
