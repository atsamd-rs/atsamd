#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `INVEN0` reader - Output Waveform 0 Invert Enable"]
pub type Inven0R = crate::BitReader;
#[doc = "Field `INVEN0` writer - Output Waveform 0 Invert Enable"]
pub type Inven0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEN1` reader - Output Waveform 1 Invert Enable"]
pub type Inven1R = crate::BitReader;
#[doc = "Field `INVEN1` writer - Output Waveform 1 Invert Enable"]
pub type Inven1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN0` reader - Capture Channel 0 Enable"]
pub type Cpten0R = crate::BitReader;
#[doc = "Field `CPTEN0` writer - Capture Channel 0 Enable"]
pub type Cpten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN1` reader - Capture Channel 1 Enable"]
pub type Cpten1R = crate::BitReader;
#[doc = "Field `CPTEN1` writer - Capture Channel 1 Enable"]
pub type Cpten1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&self) -> Inven0R {
        Inven0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&self) -> Inven1R {
        Inven1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> Cpten0R {
        Cpten0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> Cpten1R {
        Cpten1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&mut self) -> Inven0W<CtrlcSpec> {
        Inven0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&mut self) -> Inven1W<CtrlcSpec> {
        Inven1W::new(self, 1)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&mut self) -> Cpten0W<CtrlcSpec> {
        Cpten0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&mut self) -> Cpten1W<CtrlcSpec> {
        Cpten1W::new(self, 5)
    }
}
#[doc = "Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}
