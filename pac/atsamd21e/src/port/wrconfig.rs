#[doc = "Register `WRCONFIG%s` writer"]
pub type W = crate::W<WrconfigSpec>;
#[doc = "Field `PINMASK` writer - Pin Mask for Multiple Pin Configuration"]
pub type PinmaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PmuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN` writer - Input Enable"]
pub type InenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PullenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub type DrvstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMUX` writer - Peripheral Multiplexing"]
pub type PmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRPMUX` writer - Write PMUX"]
pub type WrpmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPINCFG` writer - Write PINCFG"]
pub type WrpincfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWSEL` writer - Half-Word Select"]
pub type HwselW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
    #[inline(always)]
    pub fn pinmask(&mut self) -> PinmaskW<WrconfigSpec> {
        PinmaskW::new(self, 0)
    }
    #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PmuxenW<WrconfigSpec> {
        PmuxenW::new(self, 16)
    }
    #[doc = "Bit 17 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> InenW<WrconfigSpec> {
        InenW::new(self, 17)
    }
    #[doc = "Bit 18 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PullenW<WrconfigSpec> {
        PullenW::new(self, 18)
    }
    #[doc = "Bit 22 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DrvstrW<WrconfigSpec> {
        DrvstrW::new(self, 22)
    }
    #[doc = "Bits 24:27 - Peripheral Multiplexing"]
    #[inline(always)]
    pub fn pmux(&mut self) -> PmuxW<WrconfigSpec> {
        PmuxW::new(self, 24)
    }
    #[doc = "Bit 28 - Write PMUX"]
    #[inline(always)]
    pub fn wrpmux(&mut self) -> WrpmuxW<WrconfigSpec> {
        WrpmuxW::new(self, 28)
    }
    #[doc = "Bit 30 - Write PINCFG"]
    #[inline(always)]
    pub fn wrpincfg(&mut self) -> WrpincfgW<WrconfigSpec> {
        WrpincfgW::new(self, 30)
    }
    #[doc = "Bit 31 - Half-Word Select"]
    #[inline(always)]
    pub fn hwsel(&mut self) -> HwselW<WrconfigSpec> {
        HwselW::new(self, 31)
    }
}
#[doc = "Write Configuration\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrconfig::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrconfigSpec;
impl crate::RegisterSpec for WrconfigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wrconfig::W`](W) writer structure"]
impl crate::Writable for WrconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRCONFIG%s to value 0"]
impl crate::Resettable for WrconfigSpec {}
