#[doc = "Register `GFC` reader"]
pub struct R(crate::R<GFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GFC` writer"]
pub struct W(crate::W<GFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GFC_SPEC>;
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
impl From<crate::W<GFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended"]
pub type RRFE_R = crate::BitReader<bool>;
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended"]
pub type RRFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFC_SPEC, bool, O>;
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard"]
pub type RRFS_R = crate::BitReader<bool>;
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard"]
pub type RRFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFC_SPEC, bool, O>;
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended"]
pub type ANFE_R = crate::FieldReader<u8, ANFESELECT_A>;
#[doc = "Accept Non-matching Frames Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFESELECT_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    RXF0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RXF1 = 1,
    #[doc = "2: Reject"]
    REJECT = 2,
}
impl From<ANFESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFESELECT_A) -> Self {
        variant as _
    }
}
impl ANFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANFESELECT_A> {
        match self.bits {
            0 => Some(ANFESELECT_A::RXF0),
            1 => Some(ANFESELECT_A::RXF1),
            2 => Some(ANFESELECT_A::REJECT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RXF0`"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == ANFESELECT_A::RXF0
    }
    #[doc = "Checks if the value of the field is `RXF1`"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == ANFESELECT_A::RXF1
    }
    #[doc = "Checks if the value of the field is `REJECT`"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == ANFESELECT_A::REJECT
    }
}
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended"]
pub type ANFE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GFC_SPEC, u8, ANFESELECT_A, 2, O>;
impl<'a, const O: u8> ANFE_W<'a, O> {
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut W {
        self.variant(ANFESELECT_A::RXF0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut W {
        self.variant(ANFESELECT_A::RXF1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut W {
        self.variant(ANFESELECT_A::REJECT)
    }
}
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard"]
pub type ANFS_R = crate::FieldReader<u8, ANFSSELECT_A>;
#[doc = "Accept Non-matching Frames Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFSSELECT_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    RXF0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RXF1 = 1,
    #[doc = "2: Reject"]
    REJECT = 2,
}
impl From<ANFSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFSSELECT_A) -> Self {
        variant as _
    }
}
impl ANFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANFSSELECT_A> {
        match self.bits {
            0 => Some(ANFSSELECT_A::RXF0),
            1 => Some(ANFSSELECT_A::RXF1),
            2 => Some(ANFSSELECT_A::REJECT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RXF0`"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == ANFSSELECT_A::RXF0
    }
    #[doc = "Checks if the value of the field is `RXF1`"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == ANFSSELECT_A::RXF1
    }
    #[doc = "Checks if the value of the field is `REJECT`"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == ANFSSELECT_A::REJECT
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard"]
pub type ANFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GFC_SPEC, u8, ANFSSELECT_A, 2, O>;
impl<'a, const O: u8> ANFS_W<'a, O> {
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut W {
        self.variant(ANFSSELECT_A::RXF0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut W {
        self.variant(ANFSSELECT_A::RXF1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut W {
        self.variant(ANFSSELECT_A::REJECT)
    }
}
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<0> {
        RRFE_W::new(self)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<1> {
        RRFS_W::new(self)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<2> {
        ANFE_W::new(self)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<4> {
        ANFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfc](index.html) module"]
pub struct GFC_SPEC;
impl crate::RegisterSpec for GFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gfc::R](R) reader structure"]
impl crate::Readable for GFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gfc::W](W) writer structure"]
impl crate::Writable for GFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
