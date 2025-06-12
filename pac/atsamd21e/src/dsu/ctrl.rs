#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` writer - 32-bit Cyclic Redundancy Check"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIST` writer - Memory Built-In Self-Test"]
pub type MbistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE` writer - Chip Erase"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CtrlSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 2 - 32-bit Cyclic Redundancy Check"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<CtrlSpec> {
        CrcW::new(self, 2)
    }
    #[doc = "Bit 3 - Memory Built-In Self-Test"]
    #[inline(always)]
    pub fn mbist(&mut self) -> MbistW<CtrlSpec> {
        MbistW::new(self, 3)
    }
    #[doc = "Bit 4 - Chip Erase"]
    #[inline(always)]
    pub fn ce(&mut self) -> CeW<CtrlSpec> {
        CeW::new(self, 4)
    }
}
#[doc = "Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
