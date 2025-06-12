#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Internal Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intclkenselect {
    #[doc = "0: Stop"]
    Off = 0,
    #[doc = "1: Oscillate"]
    On = 1,
}
impl From<Intclkenselect> for bool {
    #[inline(always)]
    fn from(variant: Intclkenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTCLKEN` reader - Internal Clock Enable"]
pub type IntclkenR = crate::BitReader<Intclkenselect>;
impl IntclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intclkenselect {
        match self.bits {
            false => Intclkenselect::Off,
            true => Intclkenselect::On,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Intclkenselect::Off
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Intclkenselect::On
    }
}
#[doc = "Field `INTCLKEN` writer - Internal Clock Enable"]
pub type IntclkenW<'a, REG> = crate::BitWriter<'a, REG, Intclkenselect>;
impl<'a, REG> IntclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Intclkenselect::Off)
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Intclkenselect::On)
    }
}
#[doc = "Internal Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intclksselect {
    #[doc = "0: Not Ready"]
    NotReady = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<Intclksselect> for bool {
    #[inline(always)]
    fn from(variant: Intclksselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTCLKS` reader - Internal Clock Stable"]
pub type IntclksR = crate::BitReader<Intclksselect>;
impl IntclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intclksselect {
        match self.bits {
            false => Intclksselect::NotReady,
            true => Intclksselect::Ready,
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Intclksselect::NotReady
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Intclksselect::Ready
    }
}
#[doc = "Field `INTCLKS` writer - Internal Clock Stable"]
pub type IntclksW<'a, REG> = crate::BitWriter<'a, REG, Intclksselect>;
impl<'a, REG> IntclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(Intclksselect::NotReady)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Intclksselect::Ready)
    }
}
#[doc = "SD Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdclkenselect {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Sdclkenselect> for bool {
    #[inline(always)]
    fn from(variant: Sdclkenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCLKEN` reader - SD Clock Enable"]
pub type SdclkenR = crate::BitReader<Sdclkenselect>;
impl SdclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdclkenselect {
        match self.bits {
            false => Sdclkenselect::Disable,
            true => Sdclkenselect::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sdclkenselect::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sdclkenselect::Enable
    }
}
#[doc = "Field `SDCLKEN` writer - SD Clock Enable"]
pub type SdclkenW<'a, REG> = crate::BitWriter<'a, REG, Sdclkenselect>;
impl<'a, REG> SdclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkenselect::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkenselect::Enable)
    }
}
#[doc = "Clock Generator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkgselselect {
    #[doc = "0: Divided Clock Mode"]
    Div = 0,
    #[doc = "1: Programmable Clock Mode"]
    Prog = 1,
}
impl From<Clkgselselect> for bool {
    #[inline(always)]
    fn from(variant: Clkgselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGSEL` reader - Clock Generator Select"]
pub type ClkgselR = crate::BitReader<Clkgselselect>;
impl ClkgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkgselselect {
        match self.bits {
            false => Clkgselselect::Div,
            true => Clkgselselect::Prog,
        }
    }
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == Clkgselselect::Div
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn is_prog(&self) -> bool {
        *self == Clkgselselect::Prog
    }
}
#[doc = "Field `CLKGSEL` writer - Clock Generator Select"]
pub type ClkgselW<'a, REG> = crate::BitWriter<'a, REG, Clkgselselect>;
impl<'a, REG> ClkgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgselselect::Div)
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn prog(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgselselect::Prog)
    }
}
#[doc = "Field `USDCLKFSEL` reader - Upper Bits of SDCLK Frequency Select"]
pub type UsdclkfselR = crate::FieldReader;
#[doc = "Field `USDCLKFSEL` writer - Upper Bits of SDCLK Frequency Select"]
pub type UsdclkfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLKFSEL` reader - SDCLK Frequency Select"]
pub type SdclkfselR = crate::FieldReader;
#[doc = "Field `SDCLKFSEL` writer - SDCLK Frequency Select"]
pub type SdclkfselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> IntclkenR {
        IntclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclks(&self) -> IntclksR {
        IntclksR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SdclkenR {
        SdclkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgsel(&self) -> ClkgselR {
        ClkgselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    pub fn usdclkfsel(&self) -> UsdclkfselR {
        UsdclkfselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfsel(&self) -> SdclkfselR {
        SdclkfselR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&mut self) -> IntclkenW<CcrSpec> {
        IntclkenW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclks(&mut self) -> IntclksW<CcrSpec> {
        IntclksW::new(self, 1)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&mut self) -> SdclkenW<CcrSpec> {
        SdclkenW::new(self, 2)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgsel(&mut self) -> ClkgselW<CcrSpec> {
        ClkgselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    pub fn usdclkfsel(&mut self) -> UsdclkfselW<CcrSpec> {
        UsdclkfselW::new(self, 6)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfsel(&mut self) -> SdclkfselW<CcrSpec> {
        SdclkfselW::new(self, 8)
    }
}
#[doc = "Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {}
