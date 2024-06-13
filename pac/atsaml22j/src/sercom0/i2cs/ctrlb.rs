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
#[doc = "Field `GCMD` reader - PMBus Group Command"]
pub type GCMD_R = crate::BitReader<bool>;
#[doc = "Field `GCMD` writer - PMBus Group Command"]
pub type GCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `AACKEN` reader - Automatic Address Acknowledge"]
pub type AACKEN_R = crate::BitReader<bool>;
#[doc = "Field `AACKEN` writer - Automatic Address Acknowledge"]
pub type AACKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, u8, 2, O>;
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
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&self) -> GCMD_R {
        GCMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&self) -> AACKEN_R {
        AACKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 3) as u8)
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
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    #[must_use]
    pub fn gcmd(&mut self) -> GCMD_W<9> {
        GCMD_W::new(self)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn aacken(&mut self) -> AACKEN_W<10> {
        AACKEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AMODE_W<14> {
        AMODE_W::new(self)
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
#[doc = "I2CS Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
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
