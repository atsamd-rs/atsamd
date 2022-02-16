#[doc = "Register `HSMCI_DMA` reader"]
pub struct R(crate::R<HSMCI_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSMCI_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSMCI_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSMCI_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSMCI_DMA` writer"]
pub struct W(crate::W<HSMCI_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_DMA_SPEC>;
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
impl From<crate::W<HSMCI_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Channel Read and Write Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHKSIZE_A {
    #[doc = "0: 1 data available"]
    _1 = 0,
    #[doc = "1: 2 data available"]
    _2 = 1,
    #[doc = "2: 4 data available"]
    _4 = 2,
    #[doc = "3: 8 data available"]
    _8 = 3,
    #[doc = "4: 16 data available"]
    _16 = 4,
}
impl From<CHKSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHKSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHKSIZE` reader - DMA Channel Read and Write Chunk Size"]
pub struct CHKSIZE_R(crate::FieldReader<u8, CHKSIZE_A>);
impl CHKSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHKSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHKSIZE_A> {
        match self.bits {
            0 => Some(CHKSIZE_A::_1),
            1 => Some(CHKSIZE_A::_2),
            2 => Some(CHKSIZE_A::_4),
            3 => Some(CHKSIZE_A::_8),
            4 => Some(CHKSIZE_A::_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHKSIZE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CHKSIZE_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == CHKSIZE_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == CHKSIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == CHKSIZE_A::_16
    }
}
impl core::ops::Deref for CHKSIZE_R {
    type Target = crate::FieldReader<u8, CHKSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHKSIZE` writer - DMA Channel Read and Write Chunk Size"]
pub struct CHKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHKSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_1)
    }
    #[doc = "2 data available"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_2)
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_4)
    }
    #[doc = "8 data available"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_8)
    }
    #[doc = "16 data available"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DMAEN` reader - DMA Hardware Handshaking Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Hardware Handshaking Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> CHKSIZE_R {
        CHKSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&mut self) -> CHKSIZE_W {
        CHKSIZE_W { w: self }
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_dma](index.html) module"]
pub struct HSMCI_DMA_SPEC;
impl crate::RegisterSpec for HSMCI_DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsmci_dma::R](R) reader structure"]
impl crate::Readable for HSMCI_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsmci_dma::W](W) writer structure"]
impl crate::Writable for HSMCI_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_DMA to value 0"]
impl crate::Resettable for HSMCI_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
