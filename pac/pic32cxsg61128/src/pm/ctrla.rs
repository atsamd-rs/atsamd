#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `IORET` reader - I/O Retention"]
pub type IoretR = crate::BitReader;
#[doc = "Field `IORET` writer - I/O Retention"]
pub type IoretW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - I/O Retention"]
    #[inline(always)]
    pub fn ioret(&self) -> IoretR {
        IoretR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - I/O Retention"]
    #[inline(always)]
    #[must_use]
    pub fn ioret(&mut self) -> IoretW<CtrlaSpec> {
        IoretW::new(self, 2)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u8 = 0;
}
