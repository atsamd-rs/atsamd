#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCEN` reader - Parallel Capture Enable"]
pub type PCEN_R = crate::BitReader<bool>;
#[doc = "Field `PCEN` writer - Parallel Capture Enable"]
pub type PCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `DSIZE` reader - Data size"]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - Data size"]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCALE` reader - Scale data"]
pub type SCALE_R = crate::BitReader<bool>;
#[doc = "Field `SCALE` writer - Scale data"]
pub type SCALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `ALWYS` reader - Always Sampling"]
pub type ALWYS_R = crate::BitReader<bool>;
#[doc = "Field `ALWYS` writer - Always Sampling"]
pub type ALWYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `HALFS` reader - Half Sampling"]
pub type HALFS_R = crate::BitReader<bool>;
#[doc = "Field `HALFS` writer - Half Sampling"]
pub type HALFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `FRSTS` reader - First sample"]
pub type FRSTS_R = crate::BitReader<bool>;
#[doc = "Field `FRSTS` writer - First sample"]
pub type FRSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `ISIZE` reader - Input Data Size"]
pub type ISIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISIZE` writer - Input Data Size"]
pub type ISIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CID` reader - Clear If Disabled"]
pub type CID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CID` writer - Clear If Disabled"]
pub type CID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> ALWYS_R {
        ALWYS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HALFS_R {
        HALFS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - First sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FRSTS_R {
        FRSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline(always)]
    pub fn cid(&self) -> CID_R {
        CID_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<0> {
        PCEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<4> {
        DSIZE_W::new(self)
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<8> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn alwys(&mut self) -> ALWYS_W<9> {
        ALWYS_W::new(self)
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn halfs(&mut self) -> HALFS_W<10> {
        HALFS_W::new(self)
    }
    #[doc = "Bit 11 - First sample"]
    #[inline(always)]
    #[must_use]
    pub fn frsts(&mut self) -> FRSTS_W<11> {
        FRSTS_W::new(self)
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn isize(&mut self) -> ISIZE_W<16> {
        ISIZE_W::new(self)
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cid(&mut self) -> CID_W<30> {
        CID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
