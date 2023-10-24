#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SMEN_R = crate::BitReader;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCMD` reader - PMBus Group Command"]
pub type GCMD_R = crate::BitReader;
#[doc = "Field `GCMD` writer - PMBus Group Command"]
pub type GCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AACKEN` reader - Automatic Address Acknowledge"]
pub type AACKEN_R = crate::BitReader;
#[doc = "Field `AACKEN` writer - Automatic Address Acknowledge"]
pub type AACKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AMODE_R = crate::FieldReader;
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ACKACT` reader - Acknowledge Action"]
pub type ACKACT_R = crate::BitReader;
#[doc = "Field `ACKACT` writer - Acknowledge Action"]
pub type ACKACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn smen(&mut self) -> SMEN_W<CTRLB_SPEC, 8> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    #[must_use]
    pub fn gcmd(&mut self) -> GCMD_W<CTRLB_SPEC, 9> {
        GCMD_W::new(self)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn aacken(&mut self) -> AACKEN_W<CTRLB_SPEC, 10> {
        AACKEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AMODE_W<CTRLB_SPEC, 14> {
        AMODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CTRLB_SPEC, 16> {
        CMD_W::new(self)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn ackact(&mut self) -> ACKACT_W<CTRLB_SPEC, 18> {
        ACKACT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2CS Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
