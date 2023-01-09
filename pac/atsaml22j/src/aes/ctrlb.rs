#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Manual Start"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Manual Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `NEWMSG` reader - New message"]
pub type NEWMSG_R = crate::BitReader<bool>;
#[doc = "Field `NEWMSG` writer - New message"]
pub type NEWMSG_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `EOM` reader - End of message"]
pub type EOM_R = crate::BitReader<bool>;
#[doc = "Field `EOM` writer - End of message"]
pub type EOM_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `GFMUL` reader - GF Multiplication"]
pub type GFMUL_R = crate::BitReader<bool>;
#[doc = "Field `GFMUL` writer - GF Multiplication"]
pub type GFMUL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Manual Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&self) -> NEWMSG_R {
        NEWMSG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&self) -> EOM_R {
        EOM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&self) -> GFMUL_R {
        GFMUL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Manual Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    #[must_use]
    pub fn newmsg(&mut self) -> NEWMSG_W<1> {
        NEWMSG_W::new(self)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn eom(&mut self) -> EOM_W<2> {
        EOM_W::new(self)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    #[must_use]
    pub fn gfmul(&mut self) -> GFMUL_W<3> {
        GFMUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
