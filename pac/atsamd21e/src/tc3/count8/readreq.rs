#[doc = "Register `READREQ` reader"]
pub struct R(crate::R<READREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READREQ` writer"]
pub struct W(crate::W<READREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READREQ_SPEC>;
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
impl From<crate::W<READREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, READREQ_SPEC, u8, u8, 5, O>;
#[doc = "Field `RCONT` reader - Read Continuously"]
pub type RCONT_R = crate::BitReader<bool>;
#[doc = "Field `RCONT` writer - Read Continuously"]
pub type RCONT_W<'a, const O: u8> = crate::BitWriter<'a, u16, READREQ_SPEC, bool, O>;
#[doc = "Field `RREQ` reader - Read Request"]
pub type RREQ_R = crate::BitReader<bool>;
#[doc = "Field `RREQ` writer - Read Request"]
pub type RREQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, READREQ_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Read Request"]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<14> {
        RCONT_W::new(self)
    }
    #[doc = "Bit 15 - Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn rreq(&mut self) -> RREQ_W<15> {
        RREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readreq](index.html) module"]
pub struct READREQ_SPEC;
impl crate::RegisterSpec for READREQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [readreq::R](R) reader structure"]
impl crate::Readable for READREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readreq::W](W) writer structure"]
impl crate::Writable for READREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READREQ to value 0"]
impl crate::Resettable for READREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
