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
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type CHSIZE_R = crate::FieldReader<u8, CHSIZESELECT_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHSIZESELECT_A {
    #[doc = "0: 8 bits"]
    _8_BIT = 0,
    #[doc = "1: 9 bits"]
    _9_BIT = 1,
}
impl From<CHSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSIZESELECT_A) -> Self {
        variant as _
    }
}
impl CHSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHSIZESELECT_A> {
        match self.bits {
            0 => Some(CHSIZESELECT_A::_8_BIT),
            1 => Some(CHSIZESELECT_A::_9_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_9_BIT
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type CHSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, CHSIZESELECT_A, 3, O>;
impl<'a, const O: u8> CHSIZE_W<'a, O> {
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHSIZESELECT_A::_8_BIT)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(CHSIZESELECT_A::_9_BIT)
    }
}
#[doc = "Field `PLOADEN` reader - Data Preload Enable"]
pub type PLOADEN_R = crate::BitReader<bool>;
#[doc = "Field `PLOADEN` writer - Data Preload Enable"]
pub type PLOADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `SSDE` reader - Slave Select Low Detect Enable"]
pub type SSDE_R = crate::BitReader<bool>;
#[doc = "Field `SSDE` writer - Slave Select Low Detect Enable"]
pub type SSDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `MSSEN` reader - Master Slave Select Enable"]
pub type MSSEN_R = crate::BitReader<bool>;
#[doc = "Field `MSSEN` writer - Master Slave Select Enable"]
pub type MSSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AMODE_R = crate::FieldReader<u8, AMODESELECT_A>;
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMODESELECT_A {
    #[doc = "0: SPI Address mask"]
    MASK = 0,
    #[doc = "1: Two unique Addressess"]
    _2_ADDRESSES = 1,
    #[doc = "2: Address Range"]
    RANGE = 2,
}
impl From<AMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: AMODESELECT_A) -> Self {
        variant as _
    }
}
impl AMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMODESELECT_A> {
        match self.bits {
            0 => Some(AMODESELECT_A::MASK),
            1 => Some(AMODESELECT_A::_2_ADDRESSES),
            2 => Some(AMODESELECT_A::RANGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == AMODESELECT_A::MASK
    }
    #[doc = "Checks if the value of the field is `_2_ADDRESSES`"]
    #[inline(always)]
    pub fn is_2_addresses(&self) -> bool {
        *self == AMODESELECT_A::_2_ADDRESSES
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == AMODESELECT_A::RANGE
    }
}
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, AMODESELECT_A, 2, O>;
impl<'a, const O: u8> AMODE_W<'a, O> {
    #[doc = "SPI Address mask"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AMODESELECT_A::MASK)
    }
    #[doc = "Two unique Addressess"]
    #[inline(always)]
    pub fn _2_addresses(self) -> &'a mut W {
        self.variant(AMODESELECT_A::_2_ADDRESSES)
    }
    #[doc = "Address Range"]
    #[inline(always)]
    pub fn range(self) -> &'a mut W {
        self.variant(AMODESELECT_A::RANGE)
    }
}
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&self) -> PLOADEN_R {
        PLOADEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&self) -> MSSEN_R {
        MSSEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn chsize(&mut self) -> CHSIZE_W<0> {
        CHSIZE_W::new(self)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ploaden(&mut self) -> PLOADEN_W<6> {
        PLOADEN_W::new(self)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssde(&mut self) -> SSDE_W<9> {
        SSDE_W::new(self)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mssen(&mut self) -> MSSEN_W<13> {
        MSSEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AMODE_W<14> {
        AMODE_W::new(self)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<17> {
        RXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIS Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
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
