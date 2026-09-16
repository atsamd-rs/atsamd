#[doc = "Register `DRVCTRL` reader"]
pub type R = crate::R<DrvctrlSpec>;
#[doc = "Register `DRVCTRL` writer"]
pub type W = crate::W<DrvctrlSpec>;
#[doc = "Field `INVEN0` reader - Output Waveform Invert Enable 0"]
pub type Inven0R = crate::BitReader;
#[doc = "Field `INVEN0` writer - Output Waveform Invert Enable 0"]
pub type Inven0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEN1` reader - Output Waveform Invert Enable 1"]
pub type Inven1R = crate::BitReader;
#[doc = "Field `INVEN1` writer - Output Waveform Invert Enable 1"]
pub type Inven1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    pub fn inven0(&self) -> Inven0R {
        Inven0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    pub fn inven1(&self) -> Inven1R {
        Inven1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn inven0(&mut self) -> Inven0W<DrvctrlSpec> {
        Inven0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn inven1(&mut self) -> Inven1W<DrvctrlSpec> {
        Inven1W::new(self, 1)
    }
}
#[doc = "Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`drvctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drvctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrvctrlSpec;
impl crate::RegisterSpec for DrvctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`drvctrl::R`](R) reader structure"]
impl crate::Readable for DrvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`drvctrl::W`](W) writer structure"]
impl crate::Writable for DrvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DRVCTRL to value 0"]
impl crate::Resettable for DrvctrlSpec {
    const RESET_VALUE: u8 = 0;
}
