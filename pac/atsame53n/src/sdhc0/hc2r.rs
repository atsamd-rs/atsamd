#[doc = "Register `HC2R` reader"]
pub type R = crate::R<Hc2rSpec>;
#[doc = "Register `HC2R` writer"]
pub type W = crate::W<Hc2rSpec>;
#[doc = "UHS Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uhsmsselect {
    #[doc = "0: SDR12"]
    Sdr12 = 0,
    #[doc = "1: SDR25"]
    Sdr25 = 1,
    #[doc = "2: SDR50"]
    Sdr50 = 2,
    #[doc = "3: SDR104"]
    Sdr104 = 3,
    #[doc = "4: DDR50"]
    Ddr50 = 4,
}
impl From<Uhsmsselect> for u8 {
    #[inline(always)]
    fn from(variant: Uhsmsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uhsmsselect {
    type Ux = u8;
}
impl crate::IsEnum for Uhsmsselect {}
#[doc = "Field `UHSMS` reader - UHS Mode Select"]
pub type UhsmsR = crate::FieldReader<Uhsmsselect>;
impl UhsmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uhsmsselect> {
        match self.bits {
            0 => Some(Uhsmsselect::Sdr12),
            1 => Some(Uhsmsselect::Sdr25),
            2 => Some(Uhsmsselect::Sdr50),
            3 => Some(Uhsmsselect::Sdr104),
            4 => Some(Uhsmsselect::Ddr50),
            _ => None,
        }
    }
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == Uhsmsselect::Sdr12
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == Uhsmsselect::Sdr25
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == Uhsmsselect::Sdr50
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == Uhsmsselect::Sdr104
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == Uhsmsselect::Ddr50
    }
}
#[doc = "Field `UHSMS` writer - UHS Mode Select"]
pub type UhsmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Uhsmsselect>;
impl<'a, REG> UhsmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmsselect::Sdr12)
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmsselect::Sdr25)
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmsselect::Sdr50)
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmsselect::Sdr104)
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmsselect::Ddr50)
    }
}
#[doc = "1.8V Signaling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vs18enselect {
    #[doc = "0: 3.3V Signaling"]
    S33v = 0,
    #[doc = "1: 1.8V Signaling"]
    S18v = 1,
}
impl From<Vs18enselect> for bool {
    #[inline(always)]
    fn from(variant: Vs18enselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VS18EN` reader - 1.8V Signaling Enable"]
pub type Vs18enR = crate::BitReader<Vs18enselect>;
impl Vs18enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vs18enselect {
        match self.bits {
            false => Vs18enselect::S33v,
            true => Vs18enselect::S18v,
        }
    }
    #[doc = "3.3V Signaling"]
    #[inline(always)]
    pub fn is_s33v(&self) -> bool {
        *self == Vs18enselect::S33v
    }
    #[doc = "1.8V Signaling"]
    #[inline(always)]
    pub fn is_s18v(&self) -> bool {
        *self == Vs18enselect::S18v
    }
}
#[doc = "Field `VS18EN` writer - 1.8V Signaling Enable"]
pub type Vs18enW<'a, REG> = crate::BitWriter<'a, REG, Vs18enselect>;
impl<'a, REG> Vs18enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3.3V Signaling"]
    #[inline(always)]
    pub fn s33v(self) -> &'a mut crate::W<REG> {
        self.variant(Vs18enselect::S33v)
    }
    #[doc = "1.8V Signaling"]
    #[inline(always)]
    pub fn s18v(self) -> &'a mut crate::W<REG> {
        self.variant(Vs18enselect::S18v)
    }
}
#[doc = "Driver Strength Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drvselselect {
    #[doc = "0: Driver Type B is Selected (Default)"]
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
#[doc = "Field `DRVSEL` reader - Driver Strength Select"]
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
    #[doc = "Driver Type B is Selected (Default)"]
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
#[doc = "Field `DRVSEL` writer - Driver Strength Select"]
pub type DrvselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Drvselselect, crate::Safe>;
impl<'a, REG> DrvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type B is Selected (Default)"]
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
#[doc = "Execute Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extunselect {
    #[doc = "0: Not Tuned or Tuning Completed"]
    No = 0,
    #[doc = "1: Execute Tuning"]
    Requested = 1,
}
impl From<Extunselect> for bool {
    #[inline(always)]
    fn from(variant: Extunselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTUN` reader - Execute Tuning"]
pub type ExtunR = crate::BitReader<Extunselect>;
impl ExtunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extunselect {
        match self.bits {
            false => Extunselect::No,
            true => Extunselect::Requested,
        }
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Extunselect::No
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == Extunselect::Requested
    }
}
#[doc = "Field `EXTUN` writer - Execute Tuning"]
pub type ExtunW<'a, REG> = crate::BitWriter<'a, REG, Extunselect>;
impl<'a, REG> ExtunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Extunselect::No)
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(Extunselect::Requested)
    }
}
#[doc = "Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slckselselect {
    #[doc = "0: Fixed clock is used to sample data"]
    Fixed = 0,
    #[doc = "1: Tuned clock is used to sample data"]
    Tuned = 1,
}
impl From<Slckselselect> for bool {
    #[inline(always)]
    fn from(variant: Slckselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLCKSEL` reader - Sampling Clock Select"]
pub type SlckselR = crate::BitReader<Slckselselect>;
impl SlckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slckselselect {
        match self.bits {
            false => Slckselselect::Fixed,
            true => Slckselselect::Tuned,
        }
    }
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == Slckselselect::Fixed
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn is_tuned(&self) -> bool {
        *self == Slckselselect::Tuned
    }
}
#[doc = "Field `SLCKSEL` writer - Sampling Clock Select"]
pub type SlckselW<'a, REG> = crate::BitWriter<'a, REG, Slckselselect>;
impl<'a, REG> SlckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(Slckselselect::Fixed)
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn tuned(self) -> &'a mut crate::W<REG> {
        self.variant(Slckselselect::Tuned)
    }
}
#[doc = "Asynchronous Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asintenselect {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Asintenselect> for bool {
    #[inline(always)]
    fn from(variant: Asintenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASINTEN` reader - Asynchronous Interrupt Enable"]
pub type AsintenR = crate::BitReader<Asintenselect>;
impl AsintenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asintenselect {
        match self.bits {
            false => Asintenselect::Disabled,
            true => Asintenselect::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Asintenselect::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Asintenselect::Enabled
    }
}
#[doc = "Field `ASINTEN` writer - Asynchronous Interrupt Enable"]
pub type AsintenW<'a, REG> = crate::BitWriter<'a, REG, Asintenselect>;
impl<'a, REG> AsintenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Asintenselect::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Asintenselect::Enabled)
    }
}
#[doc = "Preset Value Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvalenselect {
    #[doc = "0: SDCLK and Driver Strength are controlled by Host Controller"]
    Host = 0,
    #[doc = "1: Automatic Selection by Preset Value is Enabled"]
    Auto = 1,
}
impl From<Pvalenselect> for bool {
    #[inline(always)]
    fn from(variant: Pvalenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVALEN` reader - Preset Value Enable"]
pub type PvalenR = crate::BitReader<Pvalenselect>;
impl PvalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvalenselect {
        match self.bits {
            false => Pvalenselect::Host,
            true => Pvalenselect::Auto,
        }
    }
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == Pvalenselect::Host
    }
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Pvalenselect::Auto
    }
}
#[doc = "Field `PVALEN` writer - Preset Value Enable"]
pub type PvalenW<'a, REG> = crate::BitWriter<'a, REG, Pvalenselect>;
impl<'a, REG> PvalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(Pvalenselect::Host)
    }
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Pvalenselect::Auto)
    }
}
impl R {
    #[doc = "Bits 0:2 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsms(&self) -> UhsmsR {
        UhsmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable"]
    #[inline(always)]
    pub fn vs18en(&self) -> Vs18enR {
        Vs18enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvsel(&self) -> DrvselR {
        DrvselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline(always)]
    pub fn extun(&self) -> ExtunR {
        ExtunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline(always)]
    pub fn slcksel(&self) -> SlckselR {
        SlckselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asinten(&self) -> AsintenR {
        AsintenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    pub fn pvalen(&self) -> PvalenR {
        PvalenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsms(&mut self) -> UhsmsW<Hc2rSpec> {
        UhsmsW::new(self, 0)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable"]
    #[inline(always)]
    pub fn vs18en(&mut self) -> Vs18enW<Hc2rSpec> {
        Vs18enW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvsel(&mut self) -> DrvselW<Hc2rSpec> {
        DrvselW::new(self, 4)
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline(always)]
    pub fn extun(&mut self) -> ExtunW<Hc2rSpec> {
        ExtunW::new(self, 6)
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline(always)]
    pub fn slcksel(&mut self) -> SlckselW<Hc2rSpec> {
        SlckselW::new(self, 7)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asinten(&mut self) -> AsintenW<Hc2rSpec> {
        AsintenW::new(self, 14)
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    pub fn pvalen(&mut self) -> PvalenW<Hc2rSpec> {
        PvalenW::new(self, 15)
    }
}
#[doc = "Host Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc2rSpec;
impl crate::RegisterSpec for Hc2rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hc2r::R`](R) reader structure"]
impl crate::Readable for Hc2rSpec {}
#[doc = "`write(|w| ..)` method takes [`hc2r::W`](W) writer structure"]
impl crate::Writable for Hc2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC2R to value 0"]
impl crate::Resettable for Hc2rSpec {}
