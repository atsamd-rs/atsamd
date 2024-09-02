#[doc = "Register `GENCTRL` reader"]
pub type R = crate::R<GenctrlSpec>;
#[doc = "Register `GENCTRL` writer"]
pub type W = crate::W<GenctrlSpec>;
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcselect {
    #[doc = "0: XOSC oscillator output"]
    Xosc = 0,
    #[doc = "1: Generator input pad"]
    Gclkin = 1,
    #[doc = "2: Generic clock generator 1 output"]
    Gclkgen1 = 2,
    #[doc = "3: OSCULP32K oscillator output"]
    Osculp32k = 3,
    #[doc = "4: OSC32K oscillator output"]
    Osc32k = 4,
    #[doc = "5: XOSC32K oscillator output"]
    Xosc32k = 5,
    #[doc = "6: OSC8M oscillator output"]
    Osc8m = 6,
    #[doc = "7: DFLL48M output"]
    Dfll48m = 7,
    #[doc = "8: DPLL96M output"]
    Dpll96m = 8,
}
impl From<Srcselect> for u8 {
    #[inline(always)]
    fn from(variant: Srcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcselect {
    type Ux = u8;
}
impl crate::IsEnum for Srcselect {}
#[doc = "Field `SRC` reader - Source Select"]
pub type SrcR = crate::FieldReader<Srcselect>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srcselect> {
        match self.bits {
            0 => Some(Srcselect::Xosc),
            1 => Some(Srcselect::Gclkin),
            2 => Some(Srcselect::Gclkgen1),
            3 => Some(Srcselect::Osculp32k),
            4 => Some(Srcselect::Osc32k),
            5 => Some(Srcselect::Xosc32k),
            6 => Some(Srcselect::Osc8m),
            7 => Some(Srcselect::Dfll48m),
            8 => Some(Srcselect::Dpll96m),
            _ => None,
        }
    }
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == Srcselect::Xosc
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == Srcselect::Gclkin
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == Srcselect::Gclkgen1
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == Srcselect::Osculp32k
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == Srcselect::Osc32k
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == Srcselect::Xosc32k
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn is_osc8m(&self) -> bool {
        *self == Srcselect::Osc8m
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == Srcselect::Dfll48m
    }
    #[doc = "DPLL96M output"]
    #[inline(always)]
    pub fn is_dpll96m(&self) -> bool {
        *self == Srcselect::Dpll96m
    }
}
#[doc = "Field `SRC` writer - Source Select"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 5, Srcselect>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Xosc)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Gclkin)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Gclkgen1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Osculp32k)
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Osc32k)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Xosc32k)
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn osc8m(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Osc8m)
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Dfll48m)
    }
    #[doc = "DPLL96M output"]
    #[inline(always)]
    pub fn dpll96m(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Dpll96m)
    }
}
#[doc = "Field `GENEN` reader - Generic Clock Generator Enable"]
pub type GenenR = crate::BitReader;
#[doc = "Field `GENEN` writer - Generic Clock Generator Enable"]
pub type GenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDC` reader - Improve Duty Cycle"]
pub type IdcR = crate::BitReader;
#[doc = "Field `IDC` writer - Improve Duty Cycle"]
pub type IdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOV` reader - Output Off Value"]
pub type OovR = crate::BitReader;
#[doc = "Field `OOV` writer - Output Off Value"]
pub type OovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - Output Enable"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - Output Enable"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVSEL` reader - Divide Selection"]
pub type DivselR = crate::BitReader;
#[doc = "Field `DIVSEL` writer - Divide Selection"]
pub type DivselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GenenR {
        GenenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IdcR {
        IdcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OovR {
        OovR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DivselR {
        DivselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<GenctrlSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<GenctrlSpec> {
        SrcW::new(self, 8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn genen(&mut self) -> GenenW<GenctrlSpec> {
        GenenW::new(self, 16)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IdcW<GenctrlSpec> {
        IdcW::new(self, 17)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    #[must_use]
    pub fn oov(&mut self) -> OovW<GenctrlSpec> {
        OovW::new(self, 18)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<GenctrlSpec> {
        OeW::new(self, 19)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    #[must_use]
    pub fn divsel(&mut self) -> DivselW<GenctrlSpec> {
        DivselW::new(self, 20)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<GenctrlSpec> {
        RunstdbyW::new(self, 21)
    }
}
#[doc = "Generic Clock Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`genctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenctrlSpec;
impl crate::RegisterSpec for GenctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genctrl::R`](R) reader structure"]
impl crate::Readable for GenctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`genctrl::W`](W) writer structure"]
impl crate::Writable for GenctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GENCTRL to value 0"]
impl crate::Resettable for GenctrlSpec {
    const RESET_VALUE: u32 = 0;
}
