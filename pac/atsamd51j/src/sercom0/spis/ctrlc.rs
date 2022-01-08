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
pub struct ICSPACE_R(crate::FieldReader<u8, u8>);
impl ICSPACE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ICSPACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICSPACE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICSPACE` writer - Inter-Character Spacing"]
pub struct ICSPACE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICSPACE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA32B_A {
    #[doc = "0: Transaction from and to DATA register are 8-bit"]
    DATA_TRANS_8BIT = 0,
    #[doc = "1: Transaction from and to DATA register are 32-bit"]
    DATA_TRANS_32BIT = 1,
}
impl From<DATA32B_A> for bool {
    #[inline(always)]
    fn from(variant: DATA32B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub struct DATA32B_R(crate::FieldReader<bool, DATA32B_A>);
impl DATA32B_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA32B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA32B_A {
        match self.bits {
            false => DATA32B_A::DATA_TRANS_8BIT,
            true => DATA32B_A::DATA_TRANS_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_8BIT`"]
    #[inline(always)]
    pub fn is_data_trans_8bit(&self) -> bool {
        **self == DATA32B_A::DATA_TRANS_8BIT
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_32BIT`"]
    #[inline(always)]
    pub fn is_data_trans_32bit(&self) -> bool {
        **self == DATA32B_A::DATA_TRANS_32BIT
    }
}
impl core::ops::Deref for DATA32B_R {
    type Target = crate::FieldReader<bool, DATA32B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub struct DATA32B_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA32B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA32B_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transaction from and to DATA register are 8-bit"]
    #[inline(always)]
    pub fn data_trans_8bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_TRANS_8BIT)
    }
    #[doc = "Transaction from and to DATA register are 32-bit"]
    #[inline(always)]
    pub fn data_trans_32bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_TRANS_32BIT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
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
        DATA32B_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&mut self) -> ICSPACE_W {
        ICSPACE_W { w: self }
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&mut self) -> DATA32B_W {
        DATA32B_W { w: self }
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
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
