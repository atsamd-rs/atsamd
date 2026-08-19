#[doc = "Register `HC1R` reader"]
pub type R = crate::R<Hc1rSpec>;
#[doc = "Register `HC1R` writer"]
pub type W = crate::W<Hc1rSpec>;
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledctrlselect {
    #[doc = "0: LED off"]
    Off = 0,
    #[doc = "1: LED on"]
    On = 1,
}
impl From<Ledctrlselect> for bool {
    #[inline(always)]
    fn from(variant: Ledctrlselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDCTRL` reader - LED Control"]
pub type LedctrlR = crate::BitReader<Ledctrlselect>;
impl LedctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ledctrlselect {
        match self.bits {
            false => Ledctrlselect::Off,
            true => Ledctrlselect::On,
        }
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ledctrlselect::Off
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ledctrlselect::On
    }
}
#[doc = "Field `LEDCTRL` writer - LED Control"]
pub type LedctrlW<'a, REG> = crate::BitWriter<'a, REG, Ledctrlselect>;
impl<'a, REG> LedctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LED off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ledctrlselect::Off)
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Ledctrlselect::On)
    }
}
#[doc = "Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dwselect {
    #[doc = "0: 1-bit mode"]
    _1bit = 0,
    #[doc = "1: 4-bit mode"]
    _4bit = 1,
}
impl From<Dwselect> for bool {
    #[inline(always)]
    fn from(variant: Dwselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DW` reader - Data Width"]
pub type DwR = crate::BitReader<Dwselect>;
impl DwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dwselect {
        match self.bits {
            false => Dwselect::_1bit,
            true => Dwselect::_4bit,
        }
    }
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == Dwselect::_1bit
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == Dwselect::_4bit
    }
}
#[doc = "Field `DW` writer - Data Width"]
pub type DwW<'a, REG> = crate::BitWriter<'a, REG, Dwselect>;
impl<'a, REG> DwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dwselect::_1bit)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dwselect::_4bit)
    }
}
#[doc = "High Speed Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsenselect {
    #[doc = "0: Normal Speed mode"]
    Normal = 0,
    #[doc = "1: High Speed mode"]
    High = 1,
}
impl From<Hsenselect> for bool {
    #[inline(always)]
    fn from(variant: Hsenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEN` reader - High Speed Enable"]
pub type HsenR = crate::BitReader<Hsenselect>;
impl HsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsenselect {
        match self.bits {
            false => Hsenselect::Normal,
            true => Hsenselect::High,
        }
    }
    #[doc = "Normal Speed mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Hsenselect::Normal
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Hsenselect::High
    }
}
#[doc = "Field `HSEN` writer - High Speed Enable"]
pub type HsenW<'a, REG> = crate::BitWriter<'a, REG, Hsenselect>;
impl<'a, REG> HsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Speed mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Hsenselect::Normal)
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Hsenselect::High)
    }
}
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaselselect {
    #[doc = "0: SDMA is selected"]
    Sdma = 0,
    #[doc = "2: 32-bit Address ADMA2 is selected"]
    _32bit = 2,
}
impl From<Dmaselselect> for u8 {
    #[inline(always)]
    fn from(variant: Dmaselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaselselect {
    type Ux = u8;
}
impl crate::IsEnum for Dmaselselect {}
#[doc = "Field `DMASEL` reader - DMA Select"]
pub type DmaselR = crate::FieldReader<Dmaselselect>;
impl DmaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmaselselect> {
        match self.bits {
            0 => Some(Dmaselselect::Sdma),
            2 => Some(Dmaselselect::_32bit),
            _ => None,
        }
    }
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == Dmaselselect::Sdma
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == Dmaselselect::_32bit
    }
}
#[doc = "Field `DMASEL` writer - DMA Select"]
pub type DmaselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmaselselect>;
impl<'a, REG> DmaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselselect::Sdma)
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselselect::_32bit)
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carddtlselect {
    #[doc = "0: No Card"]
    No = 0,
    #[doc = "1: Card Inserted"]
    Yes = 1,
}
impl From<Carddtlselect> for bool {
    #[inline(always)]
    fn from(variant: Carddtlselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDDTL` reader - Card Detect Test Level"]
pub type CarddtlR = crate::BitReader<Carddtlselect>;
impl CarddtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carddtlselect {
        match self.bits {
            false => Carddtlselect::No,
            true => Carddtlselect::Yes,
        }
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Carddtlselect::No
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Carddtlselect::Yes
    }
}
#[doc = "Field `CARDDTL` writer - Card Detect Test Level"]
pub type CarddtlW<'a, REG> = crate::BitWriter<'a, REG, Carddtlselect>;
impl<'a, REG> CarddtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Carddtlselect::No)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Carddtlselect::Yes)
    }
}
#[doc = "Card Detect Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carddselselect {
    #[doc = "0: SDCD# is selected (for normal use)"]
    Normal = 0,
    #[doc = "1: The Card Select Test Level is selected (for test purpose)"]
    Test = 1,
}
impl From<Carddselselect> for bool {
    #[inline(always)]
    fn from(variant: Carddselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDDSEL` reader - Card Detect Signal Selection"]
pub type CarddselR = crate::BitReader<Carddselselect>;
impl CarddselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carddselselect {
        match self.bits {
            false => Carddselselect::Normal,
            true => Carddselselect::Test,
        }
    }
    #[doc = "SDCD# is selected (for normal use)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Carddselselect::Normal
    }
    #[doc = "The Card Select Test Level is selected (for test purpose)"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == Carddselselect::Test
    }
}
#[doc = "Field `CARDDSEL` writer - Card Detect Signal Selection"]
pub type CarddselW<'a, REG> = crate::BitWriter<'a, REG, Carddselselect>;
impl<'a, REG> CarddselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDCD# is selected (for normal use)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Carddselselect::Normal)
    }
    #[doc = "The Card Select Test Level is selected (for test purpose)"]
    #[inline(always)]
    pub fn test(self) -> &'a mut crate::W<REG> {
        self.variant(Carddselselect::Test)
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&self) -> LedctrlR {
        LedctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&self) -> HsenR {
        HsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DmaselR {
        DmaselR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn carddtl(&self) -> CarddtlR {
        CarddtlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn carddsel(&self) -> CarddselR {
        CarddselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    #[must_use]
    pub fn ledctrl(&mut self) -> LedctrlW<Hc1rSpec> {
        LedctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn dw(&mut self) -> DwW<Hc1rSpec> {
        DwW::new(self, 1)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsen(&mut self) -> HsenW<Hc1rSpec> {
        HsenW::new(self, 2)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DmaselW<Hc1rSpec> {
        DmaselW::new(self, 3)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    #[must_use]
    pub fn carddtl(&mut self) -> CarddtlW<Hc1rSpec> {
        CarddtlW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn carddsel(&mut self) -> CarddselW<Hc1rSpec> {
        CarddselW::new(self, 7)
    }
}
#[doc = "Host Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc1rSpec;
impl crate::RegisterSpec for Hc1rSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hc1r::R`](R) reader structure"]
impl crate::Readable for Hc1rSpec {}
#[doc = "`write(|w| ..)` method takes [`hc1r::W`](W) writer structure"]
impl crate::Writable for Hc1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HC1R to value 0"]
impl crate::Resettable for Hc1rSpec {
    const RESET_VALUE: u8 = 0;
}
