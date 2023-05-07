#[doc = "Register `BSR` reader"]
pub struct R(crate::R<BSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSR` writer"]
pub struct W(crate::W<BSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSR_SPEC>;
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
impl From<crate::W<BSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCKSIZE` reader - Transfer Block Size"]
pub type BLOCKSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLOCKSIZE` writer - Transfer Block Size"]
pub type BLOCKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BSR_SPEC, u16, u16, 10, O>;
#[doc = "Field `BOUNDARY` reader - SDMA Buffer Boundary"]
pub type BOUNDARY_R = crate::FieldReader<u8, BOUNDARYSELECT_A>;
#[doc = "SDMA Buffer Boundary\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOUNDARYSELECT_A {
    #[doc = "0: 4k bytes"]
    _4K = 0,
    #[doc = "1: 8k bytes"]
    _8K = 1,
    #[doc = "2: 16k bytes"]
    _16K = 2,
    #[doc = "3: 32k bytes"]
    _32K = 3,
    #[doc = "4: 64k bytes"]
    _64K = 4,
    #[doc = "5: 128k bytes"]
    _128K = 5,
    #[doc = "6: 256k bytes"]
    _256K = 6,
    #[doc = "7: 512k bytes"]
    _512K = 7,
}
impl From<BOUNDARYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BOUNDARYSELECT_A) -> Self {
        variant as _
    }
}
impl BOUNDARY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOUNDARYSELECT_A {
        match self.bits {
            0 => BOUNDARYSELECT_A::_4K,
            1 => BOUNDARYSELECT_A::_8K,
            2 => BOUNDARYSELECT_A::_16K,
            3 => BOUNDARYSELECT_A::_32K,
            4 => BOUNDARYSELECT_A::_64K,
            5 => BOUNDARYSELECT_A::_128K,
            6 => BOUNDARYSELECT_A::_256K,
            7 => BOUNDARYSELECT_A::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == BOUNDARYSELECT_A::_512K
    }
}
#[doc = "Field `BOUNDARY` writer - SDMA Buffer Boundary"]
pub type BOUNDARY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BSR_SPEC, u8, BOUNDARYSELECT_A, 3, O>;
impl<'a, const O: u8> BOUNDARY_W<'a, O> {
    #[doc = "4k bytes"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_4K)
    }
    #[doc = "8k bytes"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_8K)
    }
    #[doc = "16k bytes"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_16K)
    }
    #[doc = "32k bytes"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_32K)
    }
    #[doc = "64k bytes"]
    #[inline(always)]
    pub fn _64k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_64K)
    }
    #[doc = "128k bytes"]
    #[inline(always)]
    pub fn _128k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_128K)
    }
    #[doc = "256k bytes"]
    #[inline(always)]
    pub fn _256k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_256K)
    }
    #[doc = "512k bytes"]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut W {
        self.variant(BOUNDARYSELECT_A::_512K)
    }
}
impl R {
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn boundary(&self) -> BOUNDARY_R {
        BOUNDARY_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline(always)]
    #[must_use]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W<0> {
        BLOCKSIZE_W::new(self)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline(always)]
    #[must_use]
    pub fn boundary(&mut self) -> BOUNDARY_W<12> {
        BOUNDARY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsr](index.html) module"]
pub struct BSR_SPEC;
impl crate::RegisterSpec for BSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bsr::R](R) reader structure"]
impl crate::Readable for BSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsr::W](W) writer structure"]
impl crate::Writable for BSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSR to value 0"]
impl crate::Resettable for BSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
