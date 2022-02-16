#[doc = "Register `MCAN_NBTP` reader"]
pub struct R(crate::R<MCAN_NBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_NBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_NBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_NBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_NBTP` writer"]
pub struct W(crate::W<MCAN_NBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_NBTP_SPEC>;
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
impl From<crate::W<MCAN_NBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_NBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NTSEG2` reader - Nominal Time Segment After Sample Point"]
pub struct NTSEG2_R(crate::FieldReader<u8, u8>);
impl NTSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NTSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTSEG2` writer - Nominal Time Segment After Sample Point"]
pub struct NTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `NTSEG1` reader - Nominal Time Segment Before Sample Point"]
pub struct NTSEG1_R(crate::FieldReader<u8, u8>);
impl NTSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NTSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTSEG1` writer - Nominal Time Segment Before Sample Point"]
pub struct NTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `NBRP` reader - Nominal Bit Rate Prescaler"]
pub struct NBRP_R(crate::FieldReader<u16, u16>);
impl NBRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        NBRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBRP` writer - Nominal Bit Rate Prescaler"]
pub struct NBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> NBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `NSJW` reader - Nominal (Re) Synchronization Jump Width"]
pub struct NSJW_R(crate::FieldReader<u8, u8>);
impl NSJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NSJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSJW` writer - Nominal (Re) Synchronization Jump Width"]
pub struct NSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> NSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Nominal Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ntseg2(&mut self) -> NTSEG2_W {
        NTSEG2_W { w: self }
    }
    #[doc = "Bits 8:15 - Nominal Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ntseg1(&mut self) -> NTSEG1_W {
        NTSEG1_W { w: self }
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&mut self) -> NBRP_W {
        NBRP_W { w: self }
    }
    #[doc = "Bits 25:31 - Nominal (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&mut self) -> NSJW_W {
        NSJW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Nominal Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_nbtp](index.html) module"]
pub struct MCAN_NBTP_SPEC;
impl crate::RegisterSpec for MCAN_NBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_nbtp::R](R) reader structure"]
impl crate::Readable for MCAN_NBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_nbtp::W](W) writer structure"]
impl crate::Writable for MCAN_NBTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_NBTP to value 0"]
impl crate::Resettable for MCAN_NBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
