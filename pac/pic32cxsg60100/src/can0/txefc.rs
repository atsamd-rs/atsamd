#[doc = "Register `TXEFC` reader"]
pub type R = crate::R<TxefcSpec>;
#[doc = "Register `TXEFC` writer"]
pub type W = crate::W<TxefcSpec>;
#[doc = "Field `EFSA` reader - Event FIFO Start Address"]
pub type EfsaR = crate::FieldReader<u16>;
#[doc = "Field `EFSA` writer - Event FIFO Start Address"]
pub type EfsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EFS` reader - Event FIFO Size"]
pub type EfsR = crate::FieldReader;
#[doc = "Field `EFS` writer - Event FIFO Size"]
pub type EfsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFWM` reader - Event FIFO Watermark"]
pub type EfwmR = crate::FieldReader;
#[doc = "Field `EFWM` writer - Event FIFO Watermark"]
pub type EfwmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&self) -> EfsaR {
        EfsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&self) -> EfsR {
        EfsR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&self) -> EfwmR {
        EfwmR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Event FIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn efsa(&mut self) -> EfsaW<TxefcSpec> {
        EfsaW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    #[must_use]
    pub fn efs(&mut self) -> EfsW<TxefcSpec> {
        EfsW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn efwm(&mut self) -> EfwmW<TxefcSpec> {
        EfwmW::new(self, 24)
    }
}
#[doc = "Tx Event FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txefc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefcSpec;
impl crate::RegisterSpec for TxefcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefc::R`](R) reader structure"]
impl crate::Readable for TxefcSpec {}
#[doc = "`write(|w| ..)` method takes [`txefc::W`](W) writer structure"]
impl crate::Writable for TxefcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXEFC to value 0"]
impl crate::Resettable for TxefcSpec {
    const RESET_VALUE: u32 = 0;
}
