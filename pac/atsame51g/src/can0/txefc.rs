#[doc = "Register `TXEFC` reader"]
pub type R = crate::R<TXEFC_SPEC>;
#[doc = "Register `TXEFC` writer"]
pub type W = crate::W<TXEFC_SPEC>;
#[doc = "Field `EFSA` reader - Event FIFO Start Address"]
pub type EFSA_R = crate::FieldReader<u16>;
#[doc = "Field `EFSA` writer - Event FIFO Start Address"]
pub type EFSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `EFS` reader - Event FIFO Size"]
pub type EFS_R = crate::FieldReader;
#[doc = "Field `EFS` writer - Event FIFO Size"]
pub type EFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `EFWM` reader - Event FIFO Watermark"]
pub type EFWM_R = crate::FieldReader;
#[doc = "Field `EFWM` writer - Event FIFO Watermark"]
pub type EFWM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:15 - Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Event FIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn efsa(&mut self) -> EFSA_W<TXEFC_SPEC, 0> {
        EFSA_W::new(self)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    #[must_use]
    pub fn efs(&mut self) -> EFS_W<TXEFC_SPEC, 16> {
        EFS_W::new(self)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn efwm(&mut self) -> EFWM_W<TXEFC_SPEC, 24> {
        EFWM_W::new(self)
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
#[doc = "Tx Event FIFO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFC_SPEC;
impl crate::RegisterSpec for TXEFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefc::R`](R) reader structure"]
impl crate::Readable for TXEFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txefc::W`](W) writer structure"]
impl crate::Writable for TXEFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXEFC to value 0"]
impl crate::Resettable for TXEFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
