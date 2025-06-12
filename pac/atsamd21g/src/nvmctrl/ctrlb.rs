#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "NVM Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rwsselect {
    #[doc = "0: Single Auto Wait State"]
    Single = 0,
    #[doc = "1: Half Auto Wait State"]
    Half = 1,
    #[doc = "2: Dual Auto Wait State"]
    Dual = 2,
}
impl From<Rwsselect> for u8 {
    #[inline(always)]
    fn from(variant: Rwsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rwsselect {
    type Ux = u8;
}
impl crate::IsEnum for Rwsselect {}
#[doc = "Field `RWS` reader - NVM Read Wait States"]
pub type RwsR = crate::FieldReader<Rwsselect>;
impl RwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rwsselect> {
        match self.bits {
            0 => Some(Rwsselect::Single),
            1 => Some(Rwsselect::Half),
            2 => Some(Rwsselect::Dual),
            _ => None,
        }
    }
    #[doc = "Single Auto Wait State"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Rwsselect::Single
    }
    #[doc = "Half Auto Wait State"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Rwsselect::Half
    }
    #[doc = "Dual Auto Wait State"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Rwsselect::Dual
    }
}
#[doc = "Field `RWS` writer - NVM Read Wait States"]
pub type RwsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rwsselect>;
impl<'a, REG> RwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single Auto Wait State"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsselect::Single)
    }
    #[doc = "Half Auto Wait State"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsselect::Half)
    }
    #[doc = "Dual Auto Wait State"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsselect::Dual)
    }
}
#[doc = "Field `MANW` reader - Manual Write"]
pub type ManwR = crate::BitReader;
#[doc = "Field `MANW` writer - Manual Write"]
pub type ManwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Power Reduction Mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sleepprmselect {
    #[doc = "0: NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    Wakeonaccess = 0,
    #[doc = "1: NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    Wakeupinstant = 1,
    #[doc = "3: Auto power reduction disabled."]
    Disabled = 3,
}
impl From<Sleepprmselect> for u8 {
    #[inline(always)]
    fn from(variant: Sleepprmselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sleepprmselect {
    type Ux = u8;
}
impl crate::IsEnum for Sleepprmselect {}
#[doc = "Field `SLEEPPRM` reader - Power Reduction Mode during Sleep"]
pub type SleepprmR = crate::FieldReader<Sleepprmselect>;
impl SleepprmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sleepprmselect> {
        match self.bits {
            0 => Some(Sleepprmselect::Wakeonaccess),
            1 => Some(Sleepprmselect::Wakeupinstant),
            3 => Some(Sleepprmselect::Disabled),
            _ => None,
        }
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn is_wakeonaccess(&self) -> bool {
        *self == Sleepprmselect::Wakeonaccess
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline(always)]
    pub fn is_wakeupinstant(&self) -> bool {
        *self == Sleepprmselect::Wakeupinstant
    }
    #[doc = "Auto power reduction disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sleepprmselect::Disabled
    }
}
#[doc = "Field `SLEEPPRM` writer - Power Reduction Mode during Sleep"]
pub type SleepprmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sleepprmselect>;
impl<'a, REG> SleepprmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn wakeonaccess(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepprmselect::Wakeonaccess)
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline(always)]
    pub fn wakeupinstant(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepprmselect::Wakeupinstant)
    }
    #[doc = "Auto power reduction disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepprmselect::Disabled)
    }
}
#[doc = "NVMCTRL Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Readmodeselect {
    #[doc = "0: The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NoMissPenalty = 0,
    #[doc = "1: Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LowPower = 1,
    #[doc = "2: The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    Deterministic = 2,
}
impl From<Readmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Readmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Readmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Readmodeselect {}
#[doc = "Field `READMODE` reader - NVMCTRL Read Mode"]
pub type ReadmodeR = crate::FieldReader<Readmodeselect>;
impl ReadmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Readmodeselect> {
        match self.bits {
            0 => Some(Readmodeselect::NoMissPenalty),
            1 => Some(Readmodeselect::LowPower),
            2 => Some(Readmodeselect::Deterministic),
            _ => None,
        }
    }
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline(always)]
    pub fn is_no_miss_penalty(&self) -> bool {
        *self == Readmodeselect::NoMissPenalty
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Readmodeselect::LowPower
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline(always)]
    pub fn is_deterministic(&self) -> bool {
        *self == Readmodeselect::Deterministic
    }
}
#[doc = "Field `READMODE` writer - NVMCTRL Read Mode"]
pub type ReadmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Readmodeselect>;
impl<'a, REG> ReadmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline(always)]
    pub fn no_miss_penalty(self) -> &'a mut crate::W<REG> {
        self.variant(Readmodeselect::NoMissPenalty)
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Readmodeselect::LowPower)
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline(always)]
    pub fn deterministic(self) -> &'a mut crate::W<REG> {
        self.variant(Readmodeselect::Deterministic)
    }
}
#[doc = "Field `CACHEDIS` reader - Cache Disable"]
pub type CachedisR = crate::BitReader;
#[doc = "Field `CACHEDIS` writer - Cache Disable"]
pub type CachedisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RwsR {
        RwsR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline(always)]
    pub fn manw(&self) -> ManwR {
        ManwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn sleepprm(&self) -> SleepprmR {
        SleepprmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    pub fn readmode(&self) -> ReadmodeR {
        ReadmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&self) -> CachedisR {
        CachedisR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&mut self) -> RwsW<CtrlbSpec> {
        RwsW::new(self, 1)
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline(always)]
    pub fn manw(&mut self) -> ManwW<CtrlbSpec> {
        ManwW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn sleepprm(&mut self) -> SleepprmW<CtrlbSpec> {
        SleepprmW::new(self, 8)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    pub fn readmode(&mut self) -> ReadmodeW<CtrlbSpec> {
        ReadmodeW::new(self, 16)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&mut self) -> CachedisW<CtrlbSpec> {
        CachedisW::new(self, 18)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
