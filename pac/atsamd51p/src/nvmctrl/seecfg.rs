#[doc = "Register `SEECFG` reader"]
pub type R = crate::R<SeecfgSpec>;
#[doc = "Register `SEECFG` writer"]
pub type W = crate::W<SeecfgSpec>;
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wmodeselect {
    #[doc = "0: A NVM write command is issued after each write in the pagebuffer"]
    Unbuffered = 0,
    #[doc = "1: A NVM write command is issued when a write to a new page is requested"]
    Buffered = 1,
}
impl From<Wmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Wmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WMODE` reader - Write Mode"]
pub type WmodeR = crate::BitReader<Wmodeselect>;
impl WmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wmodeselect {
        match self.bits {
            false => Wmodeselect::Unbuffered,
            true => Wmodeselect::Buffered,
        }
    }
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline(always)]
    pub fn is_unbuffered(&self) -> bool {
        *self == Wmodeselect::Unbuffered
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == Wmodeselect::Buffered
    }
}
#[doc = "Field `WMODE` writer - Write Mode"]
pub type WmodeW<'a, REG> = crate::BitWriter<'a, REG, Wmodeselect>;
impl<'a, REG> WmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline(always)]
    pub fn unbuffered(self) -> &'a mut crate::W<REG> {
        self.variant(Wmodeselect::Unbuffered)
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut crate::W<REG> {
        self.variant(Wmodeselect::Buffered)
    }
}
#[doc = "Field `APRDIS` reader - Automatic Page Reallocation Disable"]
pub type AprdisR = crate::BitReader;
#[doc = "Field `APRDIS` writer - Automatic Page Reallocation Disable"]
pub type AprdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WmodeR {
        WmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&self) -> AprdisR {
        AprdisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&mut self) -> WmodeW<SeecfgSpec> {
        WmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&mut self) -> AprdisW<SeecfgSpec> {
        AprdisW::new(self, 1)
    }
}
#[doc = "SmartEEPROM Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`seecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeecfgSpec;
impl crate::RegisterSpec for SeecfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seecfg::R`](R) reader structure"]
impl crate::Readable for SeecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`seecfg::W`](W) writer structure"]
impl crate::Writable for SeecfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEECFG to value 0"]
impl crate::Resettable for SeecfgSpec {}
