#[doc = "Register `PCFG%s` reader"]
pub struct R(crate::R<PCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG%s` writer"]
pub struct W(crate::W<PCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG_SPEC>;
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
impl From<crate::W<PCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PTOKEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PTOKEN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `BK` reader - Pipe Bank"]
pub type BK_R = crate::BitReader<bool>;
#[doc = "Field `BK` writer - Pipe Bank"]
pub type BK_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCFG_SPEC, bool, O>;
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PCFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    pub fn bk(&self) -> BK_R {
        BK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new((self.bits >> 3) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    #[must_use]
    pub fn ptoken(&mut self) -> PTOKEN_W<0> {
        PTOKEN_W::new(self)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    #[must_use]
    pub fn bk(&mut self) -> BK_W<2> {
        BK_W::new(self)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PTYPE_W<3> {
        PTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg](index.html) module"]
pub struct PCFG_SPEC;
impl crate::RegisterSpec for PCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcfg::R](R) reader structure"]
impl crate::Readable for PCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg::W](W) writer structure"]
impl crate::Writable for PCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFG%s to value 0"]
impl crate::Resettable for PCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
