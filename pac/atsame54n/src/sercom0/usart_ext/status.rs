#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Frame Error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Frame Error"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BufovfR = crate::BitReader;
#[doc = "Field `BUFOVF` writer - Buffer Overflow"]
pub type BufovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - Clear To Send"]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - Clear To Send"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISF` reader - Inconsistent Sync Field"]
pub type IsfR = crate::BitReader;
#[doc = "Field `ISF` writer - Inconsistent Sync Field"]
pub type IsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLL` reader - Collision Detected"]
pub type CollR = crate::BitReader;
#[doc = "Field `COLL` writer - Collision Detected"]
pub type CollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - Transmitter Empty"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - Transmitter Empty"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` reader - Maximum Number of Repetitions Reached"]
pub type IterR = crate::BitReader;
#[doc = "Field `ITER` writer - Maximum Number of Repetitions Reached"]
pub type IterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BufovfR {
        BufovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Inconsistent Sync Field"]
    #[inline(always)]
    pub fn isf(&self) -> IsfR {
        IsfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Collision Detected"]
    #[inline(always)]
    pub fn coll(&self) -> CollR {
        CollR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Maximum Number of Repetitions Reached"]
    #[inline(always)]
    pub fn iter(&self) -> IterR {
        IterR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<StatusSpec> {
        PerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Error"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<StatusSpec> {
        FerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&mut self) -> BufovfW<StatusSpec> {
        BufovfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear To Send"]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<StatusSpec> {
        CtsW::new(self, 3)
    }
    #[doc = "Bit 4 - Inconsistent Sync Field"]
    #[inline(always)]
    pub fn isf(&mut self) -> IsfW<StatusSpec> {
        IsfW::new(self, 4)
    }
    #[doc = "Bit 5 - Collision Detected"]
    #[inline(always)]
    pub fn coll(&mut self) -> CollW<StatusSpec> {
        CollW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Empty"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<StatusSpec> {
        TxeW::new(self, 6)
    }
    #[doc = "Bit 7 - Maximum Number of Repetitions Reached"]
    #[inline(always)]
    pub fn iter(&mut self) -> IterW<StatusSpec> {
        IterW::new(self, 7)
    }
}
#[doc = "USART_EXT Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
