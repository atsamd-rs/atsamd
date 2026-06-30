#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: Counter in 16-bit mode"]
    Count16 = 0,
    #[doc = "1: Counter in 8-bit mode"]
    Count8 = 1,
    #[doc = "2: Counter in 32-bit mode"]
    Count32 = 2,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Timer Counter Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::Count16),
            1 => Some(Modeselect::Count8),
            2 => Some(Modeselect::Count32),
            _ => None,
        }
    }
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == Modeselect::Count16
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn is_count8(&self) -> bool {
        *self == Modeselect::Count8
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == Modeselect::Count32
    }
}
#[doc = "Field `MODE` writer - Timer Counter Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count16)
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn count8(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count8)
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count32)
    }
}
#[doc = "Prescaler and Counter Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescsyncselect {
    #[doc = "0: Reload or reset the counter on next generic clock"]
    Gclk = 0,
    #[doc = "1: Reload or reset the counter on next prescaler clock"]
    Presc = 1,
    #[doc = "2: Reload or reset the counter on next generic clock and reset the prescaler counter"]
    Resync = 2,
}
impl From<Prescsyncselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescsyncselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescsyncselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescsyncselect {}
#[doc = "Field `PRESCSYNC` reader - Prescaler and Counter Synchronization"]
pub type PrescsyncR = crate::FieldReader<Prescsyncselect>;
impl PrescsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescsyncselect> {
        match self.bits {
            0 => Some(Prescsyncselect::Gclk),
            1 => Some(Prescsyncselect::Presc),
            2 => Some(Prescsyncselect::Resync),
            _ => None,
        }
    }
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Prescsyncselect::Gclk
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn is_presc(&self) -> bool {
        *self == Prescsyncselect::Presc
    }
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == Prescsyncselect::Resync
    }
}
#[doc = "Field `PRESCSYNC` writer - Prescaler and Counter Synchronization"]
pub type PrescsyncW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prescsyncselect>;
impl<'a, REG> PrescsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Gclk)
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Presc)
    }
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Resync)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - Clock On Demand"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Clock On Demand"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: Prescaler: GCLK_TC"]
    Div1 = 0,
    #[doc = "1: Prescaler: GCLK_TC/2"]
    Div2 = 1,
    #[doc = "2: Prescaler: GCLK_TC/4"]
    Div4 = 2,
    #[doc = "3: Prescaler: GCLK_TC/8"]
    Div8 = 3,
    #[doc = "4: Prescaler: GCLK_TC/16"]
    Div16 = 4,
    #[doc = "5: Prescaler: GCLK_TC/64"]
    Div64 = 5,
    #[doc = "6: Prescaler: GCLK_TC/256"]
    Div256 = 6,
    #[doc = "7: Prescaler: GCLK_TC/1024"]
    Div1024 = 7,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescalerselect {
        match self.bits {
            0 => Prescalerselect::Div1,
            1 => Prescalerselect::Div2,
            2 => Prescalerselect::Div4,
            3 => Prescalerselect::Div8,
            4 => Prescalerselect::Div16,
            5 => Prescalerselect::Div64,
            6 => Prescalerselect::Div256,
            7 => Prescalerselect::Div1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescalerselect::Div1
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescalerselect::Div256
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Prescalerselect::Div1024
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescalerselect, crate::Safe>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1)
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div256)
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1024)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type AlockR = crate::BitReader;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type AlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTEN0` reader - Capture Channel 0 Enable"]
pub type Capten0R = crate::BitReader;
#[doc = "Field `CAPTEN0` writer - Capture Channel 0 Enable"]
pub type Capten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTEN1` reader - Capture Channel 1 Enable"]
pub type Capten1R = crate::BitReader;
#[doc = "Field `CAPTEN1` writer - Capture Channel 1 Enable"]
pub type Capten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPEN0` reader - Capture On Pin 0 Enable"]
pub type Copen0R = crate::BitReader;
#[doc = "Field `COPEN0` writer - Capture On Pin 0 Enable"]
pub type Copen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPEN1` reader - Capture On Pin 1 Enable"]
pub type Copen1R = crate::BitReader;
#[doc = "Field `COPEN1` writer - Capture On Pin 1 Enable"]
pub type Copen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture Mode Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captmode0select {
    #[doc = "0: Default capture"]
    Default = 0,
    #[doc = "1: Minimum capture"]
    Captmin = 1,
    #[doc = "2: Maximum capture"]
    Captmax = 2,
}
impl From<Captmode0select> for u8 {
    #[inline(always)]
    fn from(variant: Captmode0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captmode0select {
    type Ux = u8;
}
impl crate::IsEnum for Captmode0select {}
#[doc = "Field `CAPTMODE0` reader - Capture Mode Channel 0"]
pub type Captmode0R = crate::FieldReader<Captmode0select>;
impl Captmode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Captmode0select> {
        match self.bits {
            0 => Some(Captmode0select::Default),
            1 => Some(Captmode0select::Captmin),
            2 => Some(Captmode0select::Captmax),
            _ => None,
        }
    }
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Captmode0select::Default
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == Captmode0select::Captmin
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == Captmode0select::Captmax
    }
}
#[doc = "Field `CAPTMODE0` writer - Capture Mode Channel 0"]
pub type Captmode0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Captmode0select>;
impl<'a, REG> Captmode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Captmode0select::Default)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut crate::W<REG> {
        self.variant(Captmode0select::Captmin)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut crate::W<REG> {
        self.variant(Captmode0select::Captmax)
    }
}
#[doc = "Capture mode Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captmode1select {
    #[doc = "0: Default capture"]
    Default = 0,
    #[doc = "1: Minimum capture"]
    Captmin = 1,
    #[doc = "2: Maximum capture"]
    Captmax = 2,
}
impl From<Captmode1select> for u8 {
    #[inline(always)]
    fn from(variant: Captmode1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captmode1select {
    type Ux = u8;
}
impl crate::IsEnum for Captmode1select {}
#[doc = "Field `CAPTMODE1` reader - Capture mode Channel 1"]
pub type Captmode1R = crate::FieldReader<Captmode1select>;
impl Captmode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Captmode1select> {
        match self.bits {
            0 => Some(Captmode1select::Default),
            1 => Some(Captmode1select::Captmin),
            2 => Some(Captmode1select::Captmax),
            _ => None,
        }
    }
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Captmode1select::Default
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == Captmode1select::Captmin
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == Captmode1select::Captmax
    }
}
#[doc = "Field `CAPTMODE1` writer - Capture mode Channel 1"]
pub type Captmode1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Captmode1select>;
impl<'a, REG> Captmode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Captmode1select::Default)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut crate::W<REG> {
        self.variant(Captmode1select::Captmin)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut crate::W<REG> {
        self.variant(Captmode1select::Captmax)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&self) -> PrescsyncR {
        PrescsyncR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> AlockR {
        AlockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn capten0(&self) -> Capten0R {
        Capten0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn capten1(&self) -> Capten1R {
        Capten1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    pub fn copen0(&self) -> Copen0R {
        Copen0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    pub fn copen1(&self) -> Copen1R {
        Copen1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    pub fn captmode0(&self) -> Captmode0R {
        Captmode0R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    pub fn captmode1(&self) -> Captmode1R {
        Captmode1R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn prescsync(&mut self) -> PrescsyncW<CtrlaSpec> {
        PrescsyncW::new(self, 4)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<CtrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<CtrlaSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<CtrlaSpec> {
        PrescalerW::new(self, 8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    #[must_use]
    pub fn alock(&mut self) -> AlockW<CtrlaSpec> {
        AlockW::new(self, 11)
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn capten0(&mut self) -> Capten0W<CtrlaSpec> {
        Capten0W::new(self, 16)
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn capten1(&mut self) -> Capten1W<CtrlaSpec> {
        Capten1W::new(self, 17)
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn copen0(&mut self) -> Copen0W<CtrlaSpec> {
        Copen0W::new(self, 20)
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn copen1(&mut self) -> Copen1W<CtrlaSpec> {
        Copen1W::new(self, 21)
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn captmode0(&mut self) -> Captmode0W<CtrlaSpec> {
        Captmode0W::new(self, 24)
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn captmode1(&mut self) -> Captmode1W<CtrlaSpec> {
        Captmode1W::new(self, 27)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0;
}
