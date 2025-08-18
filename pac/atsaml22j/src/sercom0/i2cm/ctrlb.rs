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
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `QCEN` reader - Quick Command Enable"]
pub type QCEN_R = crate::BitReader<bool>;
#[doc = "Field `QCEN` writer - Quick Command Enable"]
pub type QCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `ACKACT` reader - Acknowledge Action"]
pub type ACKACT_R = crate::BitReader<bool>;
#[doc = "Field `ACKACT` writer - Acknowledge Action"]
pub type ACKACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quick Command Enable"]
    #[inline(always)]
    pub fn qcen(&self) -> QCEN_R {
        QCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> ACKACT_R {
        ACKACT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<8> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 9 - Quick Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qcen(&mut self) -> QCEN_W<9> {
        QCEN_W::new(self)
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<16> {
        CMD_W::new(self)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn ackact(&mut self) -> ACKACT_W<18> {
        ACKACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CM Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
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
