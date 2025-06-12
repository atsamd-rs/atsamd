#[doc = "Register `PVR[%s]` reader"]
pub type R = crate::R<PvrSpec>;
#[doc = "Register `PVR[%s]` writer"]
pub type W = crate::W<PvrSpec>;
#[doc = "Field `SDCLKFSEL` reader - SDCLK Frequency Select Value for Initialization"]
pub type SdclkfselR = crate::FieldReader<u16>;
#[doc = "Field `SDCLKFSEL` writer - SDCLK Frequency Select Value for Initialization"]
pub type SdclkfselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Clock Generator Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkgselselect {
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    Div = 0,
    #[doc = "1: Programmable Clock Generator"]
    Prog = 1,
}
impl From<Clkgselselect> for bool {
    #[inline(always)]
    fn from(variant: Clkgselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGSEL` reader - Clock Generator Select Value for Initialization"]
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
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == Clkgselselect::Div
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_prog(&self) -> bool {
        *self == Clkgselselect::Prog
    }
}
#[doc = "Field `CLKGSEL` writer - Clock Generator Select Value for Initialization"]
pub type ClkgselW<'a, REG> = crate::BitWriter<'a, REG, Clkgselselect>;
impl<'a, REG> ClkgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgselselect::Div)
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn prog(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgselselect::Prog)
    }
}
#[doc = "Driver Strength Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drvselselect {
    #[doc = "0: Driver Type B is Selected"]
    B = 0,
    #[doc = "1: Driver Type A is Selected"]
    A = 1,
    #[doc = "2: Driver Type C is Selected"]
    C = 2,
    #[doc = "3: Driver Type D is Selected"]
    D = 3,
}
impl From<Drvselselect> for u8 {
    #[inline(always)]
    fn from(variant: Drvselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drvselselect {
    type Ux = u8;
}
impl crate::IsEnum for Drvselselect {}
#[doc = "Field `DRVSEL` reader - Driver Strength Select Value for Initialization"]
pub type DrvselR = crate::FieldReader<Drvselselect>;
impl DrvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drvselselect {
        match self.bits {
            0 => Drvselselect::B,
            1 => Drvselselect::A,
            2 => Drvselselect::C,
            3 => Drvselselect::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Drvselselect::B
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Drvselselect::A
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Drvselselect::C
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == Drvselselect::D
    }
}
#[doc = "Field `DRVSEL` writer - Driver Strength Select Value for Initialization"]
pub type DrvselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Drvselselect, crate::Safe>;
impl<'a, REG> DrvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Drvselselect::B)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Drvselselect::A)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Drvselselect::C)
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Drvselselect::D)
    }
}
impl R {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline(always)]
    pub fn sdclkfsel(&self) -> SdclkfselR {
        SdclkfselR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    pub fn clkgsel(&self) -> ClkgselR {
        ClkgselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    pub fn drvsel(&self) -> DrvselR {
        DrvselR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline(always)]
    pub fn sdclkfsel(&mut self) -> SdclkfselW<PvrSpec> {
        SdclkfselW::new(self, 0)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    pub fn clkgsel(&mut self) -> ClkgselW<PvrSpec> {
        ClkgselW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    pub fn drvsel(&mut self) -> DrvselW<PvrSpec> {
        DrvselW::new(self, 14)
    }
}
#[doc = "Preset Value n\n\nYou can [`read`](crate::Reg::read) this register and get [`pvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvrSpec;
impl crate::RegisterSpec for PvrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pvr::R`](R) reader structure"]
impl crate::Readable for PvrSpec {}
#[doc = "`write(|w| ..)` method takes [`pvr::W`](W) writer structure"]
impl crate::Writable for PvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVR[%s] to value 0"]
impl crate::Resettable for PvrSpec {}
