#[doc = "Register `DPLLCTRLB` reader"]
pub type R = crate::R<DpllctrlbSpec>;
#[doc = "Register `DPLLCTRLB` writer"]
pub type W = crate::W<DpllctrlbSpec>;
#[doc = "Proportional Integral Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filterselect {
    #[doc = "0: Default filter mode"]
    Default = 0,
    #[doc = "1: Low bandwidth filter"]
    Lbfilt = 1,
    #[doc = "2: High bandwidth filter"]
    Hbfilt = 2,
    #[doc = "3: High damping filter"]
    Hdfilt = 3,
}
impl From<Filterselect> for u8 {
    #[inline(always)]
    fn from(variant: Filterselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filterselect {
    type Ux = u8;
}
impl crate::IsEnum for Filterselect {}
#[doc = "Field `FILTER` reader - Proportional Integral Filter Selection"]
pub type FilterR = crate::FieldReader<Filterselect>;
impl FilterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filterselect {
        match self.bits {
            0 => Filterselect::Default,
            1 => Filterselect::Lbfilt,
            2 => Filterselect::Hbfilt,
            3 => Filterselect::Hdfilt,
            _ => unreachable!(),
        }
    }
    #[doc = "Default filter mode"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Filterselect::Default
    }
    #[doc = "Low bandwidth filter"]
    #[inline(always)]
    pub fn is_lbfilt(&self) -> bool {
        *self == Filterselect::Lbfilt
    }
    #[doc = "High bandwidth filter"]
    #[inline(always)]
    pub fn is_hbfilt(&self) -> bool {
        *self == Filterselect::Hbfilt
    }
    #[doc = "High damping filter"]
    #[inline(always)]
    pub fn is_hdfilt(&self) -> bool {
        *self == Filterselect::Hdfilt
    }
}
#[doc = "Field `FILTER` writer - Proportional Integral Filter Selection"]
pub type FilterW<'a, REG> = crate::FieldWriter<'a, REG, 2, Filterselect, crate::Safe>;
impl<'a, REG> FilterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default filter mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Default)
    }
    #[doc = "Low bandwidth filter"]
    #[inline(always)]
    pub fn lbfilt(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Lbfilt)
    }
    #[doc = "High bandwidth filter"]
    #[inline(always)]
    pub fn hbfilt(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Hbfilt)
    }
    #[doc = "High damping filter"]
    #[inline(always)]
    pub fn hdfilt(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Hdfilt)
    }
}
#[doc = "Field `LPEN` reader - Low-Power Enable"]
pub type LpenR = crate::BitReader;
#[doc = "Field `LPEN` writer - Low-Power Enable"]
pub type LpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF` reader - Wake Up Fast"]
pub type WufR = crate::BitReader;
#[doc = "Field `WUF` writer - Wake Up Fast"]
pub type WufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refclkselect {
    #[doc = "0: CLK_DPLL_REF0 clock reference"]
    Ref0 = 0,
    #[doc = "1: CLK_DPLL_REF1 clock reference"]
    Ref1 = 1,
    #[doc = "2: GCLK_DPLL clock reference"]
    Gclk = 2,
}
impl From<Refclkselect> for u8 {
    #[inline(always)]
    fn from(variant: Refclkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refclkselect {
    type Ux = u8;
}
impl crate::IsEnum for Refclkselect {}
#[doc = "Field `REFCLK` reader - Reference Clock Selection"]
pub type RefclkR = crate::FieldReader<Refclkselect>;
impl RefclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refclkselect> {
        match self.bits {
            0 => Some(Refclkselect::Ref0),
            1 => Some(Refclkselect::Ref1),
            2 => Some(Refclkselect::Gclk),
            _ => None,
        }
    }
    #[doc = "CLK_DPLL_REF0 clock reference"]
    #[inline(always)]
    pub fn is_ref0(&self) -> bool {
        *self == Refclkselect::Ref0
    }
    #[doc = "CLK_DPLL_REF1 clock reference"]
    #[inline(always)]
    pub fn is_ref1(&self) -> bool {
        *self == Refclkselect::Ref1
    }
    #[doc = "GCLK_DPLL clock reference"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Refclkselect::Gclk
    }
}
#[doc = "Field `REFCLK` writer - Reference Clock Selection"]
pub type RefclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refclkselect>;
impl<'a, REG> RefclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_DPLL_REF0 clock reference"]
    #[inline(always)]
    pub fn ref0(self) -> &'a mut crate::W<REG> {
        self.variant(Refclkselect::Ref0)
    }
    #[doc = "CLK_DPLL_REF1 clock reference"]
    #[inline(always)]
    pub fn ref1(self) -> &'a mut crate::W<REG> {
        self.variant(Refclkselect::Ref1)
    }
    #[doc = "GCLK_DPLL clock reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Refclkselect::Gclk)
    }
}
#[doc = "Lock Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ltimeselect {
    #[doc = "0: Default No time-out"]
    None = 0,
    #[doc = "4: 8MS Time-out if no lock within 8 ms"]
    _8ms = 4,
    #[doc = "5: 9MS Time-out if no lock within 9 ms"]
    _9ms = 5,
    #[doc = "6: 10MS Time-out if no lock within 10 ms"]
    _10ms = 6,
    #[doc = "7: 11MS Time-out if no lock within 11 ms"]
    _11ms = 7,
}
impl From<Ltimeselect> for u8 {
    #[inline(always)]
    fn from(variant: Ltimeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ltimeselect {
    type Ux = u8;
}
impl crate::IsEnum for Ltimeselect {}
#[doc = "Field `LTIME` reader - Lock Time"]
pub type LtimeR = crate::FieldReader<Ltimeselect>;
impl LtimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ltimeselect> {
        match self.bits {
            0 => Some(Ltimeselect::None),
            4 => Some(Ltimeselect::_8ms),
            5 => Some(Ltimeselect::_9ms),
            6 => Some(Ltimeselect::_10ms),
            7 => Some(Ltimeselect::_11ms),
            _ => None,
        }
    }
    #[doc = "Default No time-out"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ltimeselect::None
    }
    #[doc = "8MS Time-out if no lock within 8 ms"]
    #[inline(always)]
    pub fn is_8ms(&self) -> bool {
        *self == Ltimeselect::_8ms
    }
    #[doc = "9MS Time-out if no lock within 9 ms"]
    #[inline(always)]
    pub fn is_9ms(&self) -> bool {
        *self == Ltimeselect::_9ms
    }
    #[doc = "10MS Time-out if no lock within 10 ms"]
    #[inline(always)]
    pub fn is_10ms(&self) -> bool {
        *self == Ltimeselect::_10ms
    }
    #[doc = "11MS Time-out if no lock within 11 ms"]
    #[inline(always)]
    pub fn is_11ms(&self) -> bool {
        *self == Ltimeselect::_11ms
    }
}
#[doc = "Field `LTIME` writer - Lock Time"]
pub type LtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ltimeselect>;
impl<'a, REG> LtimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default No time-out"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::None)
    }
    #[doc = "8MS Time-out if no lock within 8 ms"]
    #[inline(always)]
    pub fn _8ms(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_8ms)
    }
    #[doc = "9MS Time-out if no lock within 9 ms"]
    #[inline(always)]
    pub fn _9ms(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_9ms)
    }
    #[doc = "10MS Time-out if no lock within 10 ms"]
    #[inline(always)]
    pub fn _10ms(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_10ms)
    }
    #[doc = "11MS Time-out if no lock within 11 ms"]
    #[inline(always)]
    pub fn _11ms(self) -> &'a mut crate::W<REG> {
        self.variant(Ltimeselect::_11ms)
    }
}
#[doc = "Field `LBYPASS` reader - Lock Bypass"]
pub type LbypassR = crate::BitReader;
#[doc = "Field `LBYPASS` writer - Lock Bypass"]
pub type LbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LpenR {
        LpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> RefclkR {
        RefclkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LtimeR {
        LtimeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LbypassR {
        LbypassR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&mut self) -> FilterW<DpllctrlbSpec> {
        FilterW::new(self, 0)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LpenW<DpllctrlbSpec> {
        LpenW::new(self, 2)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&mut self) -> WufW<DpllctrlbSpec> {
        WufW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&mut self) -> RefclkW<DpllctrlbSpec> {
        RefclkW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&mut self) -> LtimeW<DpllctrlbSpec> {
        LtimeW::new(self, 8)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&mut self) -> LbypassW<DpllctrlbSpec> {
        LbypassW::new(self, 12)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<DpllctrlbSpec> {
        DivW::new(self, 16)
    }
}
#[doc = "DPLL Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllctrlbSpec;
impl crate::RegisterSpec for DpllctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllctrlb::R`](R) reader structure"]
impl crate::Readable for DpllctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`dpllctrlb::W`](W) writer structure"]
impl crate::Writable for DpllctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPLLCTRLB to value 0"]
impl crate::Resettable for DpllctrlbSpec {}
