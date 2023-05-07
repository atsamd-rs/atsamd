#[doc = "Register `NBTP` reader"]
pub struct R(crate::R<NBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NBTP` writer"]
pub struct W(crate::W<NBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NBTP_SPEC>;
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
impl From<crate::W<NBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NTSEG2` reader - Nominal Time segment after sample point"]
pub type NTSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTSEG2` writer - Nominal Time segment after sample point"]
pub type NTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u8, u8, 7, O>;
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point"]
pub type NTSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTSEG1` writer - Nominal Time segment before sample point"]
pub type NTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u8, u8, 8, O>;
#[doc = "Field `NBRP` reader - Nominal Baud Rate Prescaler"]
pub type NBRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NBRP` writer - Nominal Baud Rate Prescaler"]
pub type NBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u16, u16, 9, O>;
#[doc = "Field `NSJW` reader - Nominal (Re)Synchronization Jump Width"]
pub type NSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSJW` writer - Nominal (Re)Synchronization Jump Width"]
pub type NSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal Baud Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<0> {
        NTSEG2_W::new(self)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<8> {
        NTSEG1_W::new(self)
    }
    #[doc = "Bits 16:24 - Nominal Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<16> {
        NBRP_W::new(self)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<25> {
        NSJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Nominal Bit Timing and Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbtp](index.html) module"]
pub struct NBTP_SPEC;
impl crate::RegisterSpec for NBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nbtp::R](R) reader structure"]
impl crate::Readable for NBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nbtp::W](W) writer structure"]
impl crate::Writable for NBTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NBTP to value 0x0600_0a03"]
impl crate::Resettable for NBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600_0a03;
}
