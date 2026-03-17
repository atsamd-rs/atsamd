#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BufovfR = crate::BitReader;
#[doc = "Field `BUFOVF` writer - Buffer Overflow"]
pub type BufovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LENERR` reader - Transaction Length Error"]
pub type LenerrR = crate::BitReader;
#[doc = "Field `LENERR` writer - Transaction Length Error"]
pub type LenerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BufovfR {
        BufovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    pub fn lenerr(&self) -> LenerrR {
        LenerrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn bufovf(&mut self) -> BufovfW<StatusSpec> {
        BufovfW::new(self, 2)
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    #[must_use]
    pub fn lenerr(&mut self) -> LenerrW<StatusSpec> {
        LenerrW::new(self, 11)
    }
}
#[doc = "SPIM Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u16 = 0;
}
