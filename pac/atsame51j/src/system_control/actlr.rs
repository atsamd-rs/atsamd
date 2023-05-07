#[doc = "Register `ACTLR` reader"]
pub struct R(crate::R<ACTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTLR` writer"]
pub struct W(crate::W<ACTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTLR_SPEC>;
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
impl From<crate::W<ACTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISMCYCINT` reader - Disable interruption of LDM/STM instructions"]
pub type DISMCYCINT_R = crate::BitReader<bool>;
#[doc = "Field `DISMCYCINT` writer - Disable interruption of LDM/STM instructions"]
pub type DISMCYCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISDEFWBUF` reader - Disable wruite buffer use during default memory map accesses"]
pub type DISDEFWBUF_R = crate::BitReader<bool>;
#[doc = "Field `DISDEFWBUF` writer - Disable wruite buffer use during default memory map accesses"]
pub type DISDEFWBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISFOLD` reader - Disable IT folding"]
pub type DISFOLD_R = crate::BitReader<bool>;
#[doc = "Field `DISFOLD` writer - Disable IT folding"]
pub type DISFOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISFPCA` reader - Disable automatic update of CONTROL.FPCA"]
pub type DISFPCA_R = crate::BitReader<bool>;
#[doc = "Field `DISFPCA` writer - Disable automatic update of CONTROL.FPCA"]
pub type DISFPCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISOOFP` reader - Disable out-of-order FP instructions"]
pub type DISOOFP_R = crate::BitReader<bool>;
#[doc = "Field `DISOOFP` writer - Disable out-of-order FP instructions"]
pub type DISOOFP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Disable interruption of LDM/STM instructions"]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable wruite buffer use during default memory map accesses"]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable IT folding"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable out-of-order FP instructions"]
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interruption of LDM/STM instructions"]
    #[inline(always)]
    #[must_use]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W<0> {
        DISMCYCINT_W::new(self)
    }
    #[doc = "Bit 1 - Disable wruite buffer use during default memory map accesses"]
    #[inline(always)]
    #[must_use]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W<1> {
        DISDEFWBUF_W::new(self)
    }
    #[doc = "Bit 2 - Disable IT folding"]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<2> {
        DISFOLD_W::new(self)
    }
    #[doc = "Bit 8 - Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    #[must_use]
    pub fn disfpca(&mut self) -> DISFPCA_W<8> {
        DISFPCA_W::new(self)
    }
    #[doc = "Bit 9 - Disable out-of-order FP instructions"]
    #[inline(always)]
    #[must_use]
    pub fn disoofp(&mut self) -> DISOOFP_W<9> {
        DISOOFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](index.html) module"]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actlr::R](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actlr::W](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
