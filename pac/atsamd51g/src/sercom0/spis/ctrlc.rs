#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICSPACE` reader - Inter-Character Spacing"]
pub type ICSPACE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICSPACE` writer - Inter-Character Spacing"]
pub type ICSPACE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLC_SPEC, u8, u8, 6, O>;
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub type DATA32B_R = crate::BitReader<DATA32BSELECT_A>;
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA32BSELECT_A {
    #[doc = "0: Transaction from and to DATA register are 8-bit"]
    DATA_TRANS_8BIT = 0,
    #[doc = "1: Transaction from and to DATA register are 32-bit"]
    DATA_TRANS_32BIT = 1,
}
impl From<DATA32BSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DATA32BSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA32B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA32BSELECT_A {
        match self.bits {
            false => DATA32BSELECT_A::DATA_TRANS_8BIT,
            true => DATA32BSELECT_A::DATA_TRANS_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_8BIT`"]
    #[inline(always)]
    pub fn is_data_trans_8bit(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_TRANS_8BIT
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_32BIT`"]
    #[inline(always)]
    pub fn is_data_trans_32bit(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_TRANS_32BIT
    }
}
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub type DATA32B_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLC_SPEC, DATA32BSELECT_A, O>;
impl<'a, const O: u8> DATA32B_W<'a, O> {
    #[doc = "Transaction from and to DATA register are 8-bit"]
    #[inline(always)]
    pub fn data_trans_8bit(self) -> &'a mut W {
        self.variant(DATA32BSELECT_A::DATA_TRANS_8BIT)
    }
    #[doc = "Transaction from and to DATA register are 32-bit"]
    #[inline(always)]
    pub fn data_trans_32bit(self) -> &'a mut W {
        self.variant(DATA32BSELECT_A::DATA_TRANS_32BIT)
    }
}
impl R {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&self) -> ICSPACE_R {
        ICSPACE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> DATA32B_R {
        DATA32B_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    #[must_use]
    pub fn icspace(&mut self) -> ICSPACE_W<0> {
        ICSPACE_W::new(self)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn data32b(&mut self) -> DATA32B_W<24> {
        DATA32B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIS Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
