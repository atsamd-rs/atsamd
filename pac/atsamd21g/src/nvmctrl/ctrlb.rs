#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `RWS` reader - NVM Read Wait States"]
pub type RWS_R = crate::FieldReader<RWSSELECT_A>;
#[doc = "NVM Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RWSSELECT_A {
    #[doc = "0: Single Auto Wait State"]
    SINGLE = 0,
    #[doc = "1: Half Auto Wait State"]
    HALF = 1,
    #[doc = "2: Dual Auto Wait State"]
    DUAL = 2,
}
impl From<RWSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RWSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RWSSELECT_A {
    type Ux = u8;
}
impl RWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RWSSELECT_A> {
        match self.bits {
            0 => Some(RWSSELECT_A::SINGLE),
            1 => Some(RWSSELECT_A::HALF),
            2 => Some(RWSSELECT_A::DUAL),
            _ => None,
        }
    }
    #[doc = "Single Auto Wait State"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == RWSSELECT_A::SINGLE
    }
    #[doc = "Half Auto Wait State"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == RWSSELECT_A::HALF
    }
    #[doc = "Dual Auto Wait State"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == RWSSELECT_A::DUAL
    }
}
#[doc = "Field `RWS` writer - NVM Read Wait States"]
pub type RWS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, RWSSELECT_A>;
impl<'a, REG, const O: u8> RWS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single Auto Wait State"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(RWSSELECT_A::SINGLE)
    }
    #[doc = "Half Auto Wait State"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(RWSSELECT_A::HALF)
    }
    #[doc = "Dual Auto Wait State"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(RWSSELECT_A::DUAL)
    }
}
#[doc = "Field `MANW` reader - Manual Write"]
pub type MANW_R = crate::BitReader;
#[doc = "Field `MANW` writer - Manual Write"]
pub type MANW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLEEPPRM` reader - Power Reduction Mode during Sleep"]
pub type SLEEPPRM_R = crate::FieldReader<SLEEPPRMSELECT_A>;
#[doc = "Power Reduction Mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLEEPPRMSELECT_A {
    #[doc = "0: NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    WAKEONACCESS = 0,
    #[doc = "1: NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    WAKEUPINSTANT = 1,
    #[doc = "3: Auto power reduction disabled."]
    DISABLED = 3,
}
impl From<SLEEPPRMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEPPRMSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLEEPPRMSELECT_A {
    type Ux = u8;
}
impl SLEEPPRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SLEEPPRMSELECT_A> {
        match self.bits {
            0 => Some(SLEEPPRMSELECT_A::WAKEONACCESS),
            1 => Some(SLEEPPRMSELECT_A::WAKEUPINSTANT),
            3 => Some(SLEEPPRMSELECT_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn is_wakeonaccess(&self) -> bool {
        *self == SLEEPPRMSELECT_A::WAKEONACCESS
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline(always)]
    pub fn is_wakeupinstant(&self) -> bool {
        *self == SLEEPPRMSELECT_A::WAKEUPINSTANT
    }
    #[doc = "Auto power reduction disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPPRMSELECT_A::DISABLED
    }
}
#[doc = "Field `SLEEPPRM` writer - Power Reduction Mode during Sleep"]
pub type SLEEPPRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SLEEPPRMSELECT_A>;
impl<'a, REG, const O: u8> SLEEPPRM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn wakeonaccess(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPPRMSELECT_A::WAKEONACCESS)
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline(always)]
    pub fn wakeupinstant(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPPRMSELECT_A::WAKEUPINSTANT)
    }
    #[doc = "Auto power reduction disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPPRMSELECT_A::DISABLED)
    }
}
#[doc = "Field `READMODE` reader - NVMCTRL Read Mode"]
pub type READMODE_R = crate::FieldReader<READMODESELECT_A>;
#[doc = "NVMCTRL Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum READMODESELECT_A {
    #[doc = "0: The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NO_MISS_PENALTY = 0,
    #[doc = "1: Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LOW_POWER = 1,
    #[doc = "2: The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    DETERMINISTIC = 2,
}
impl From<READMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: READMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for READMODESELECT_A {
    type Ux = u8;
}
impl READMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<READMODESELECT_A> {
        match self.bits {
            0 => Some(READMODESELECT_A::NO_MISS_PENALTY),
            1 => Some(READMODESELECT_A::LOW_POWER),
            2 => Some(READMODESELECT_A::DETERMINISTIC),
            _ => None,
        }
    }
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline(always)]
    pub fn is_no_miss_penalty(&self) -> bool {
        *self == READMODESELECT_A::NO_MISS_PENALTY
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == READMODESELECT_A::LOW_POWER
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline(always)]
    pub fn is_deterministic(&self) -> bool {
        *self == READMODESELECT_A::DETERMINISTIC
    }
}
#[doc = "Field `READMODE` writer - NVMCTRL Read Mode"]
pub type READMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, READMODESELECT_A>;
impl<'a, REG, const O: u8> READMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline(always)]
    pub fn no_miss_penalty(self) -> &'a mut crate::W<REG> {
        self.variant(READMODESELECT_A::NO_MISS_PENALTY)
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(READMODESELECT_A::LOW_POWER)
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline(always)]
    pub fn deterministic(self) -> &'a mut crate::W<REG> {
        self.variant(READMODESELECT_A::DETERMINISTIC)
    }
}
#[doc = "Field `CACHEDIS` reader - Cache Disable"]
pub type CACHEDIS_R = crate::BitReader;
#[doc = "Field `CACHEDIS` writer - Cache Disable"]
pub type CACHEDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline(always)]
    pub fn manw(&self) -> MANW_R {
        MANW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn sleepprm(&self) -> SLEEPPRM_R {
        SLEEPPRM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    pub fn readmode(&self) -> READMODE_R {
        READMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&self) -> CACHEDIS_R {
        CACHEDIS_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    #[must_use]
    pub fn rws(&mut self) -> RWS_W<CTRLB_SPEC, 1> {
        RWS_W::new(self)
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline(always)]
    #[must_use]
    pub fn manw(&mut self) -> MANW_W<CTRLB_SPEC, 7> {
        MANW_W::new(self)
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn sleepprm(&mut self) -> SLEEPPRM_W<CTRLB_SPEC, 8> {
        SLEEPPRM_W::new(self)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn readmode(&mut self) -> READMODE_W<CTRLB_SPEC, 16> {
        READMODE_W::new(self)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cachedis(&mut self) -> CACHEDIS_W<CTRLB_SPEC, 18> {
        CACHEDIS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
