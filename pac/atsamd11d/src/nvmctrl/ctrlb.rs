#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWS` reader - NVM Read Wait States"]
pub type RWS_R = crate::FieldReader<u8, RWSSELECT_A>;
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
impl RWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RWSSELECT_A> {
        match self.bits {
            0 => Some(RWSSELECT_A::SINGLE),
            1 => Some(RWSSELECT_A::HALF),
            2 => Some(RWSSELECT_A::DUAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == RWSSELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == RWSSELECT_A::HALF
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == RWSSELECT_A::DUAL
    }
}
#[doc = "Field `RWS` writer - NVM Read Wait States"]
pub type RWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, RWSSELECT_A, 4, O>;
impl<'a, const O: u8> RWS_W<'a, O> {
    #[doc = "Single Auto Wait State"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(RWSSELECT_A::SINGLE)
    }
    #[doc = "Half Auto Wait State"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(RWSSELECT_A::HALF)
    }
    #[doc = "Dual Auto Wait State"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(RWSSELECT_A::DUAL)
    }
}
#[doc = "Field `MANW` reader - Manual Write"]
pub type MANW_R = crate::BitReader<bool>;
#[doc = "Field `MANW` writer - Manual Write"]
pub type MANW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
#[doc = "Field `SLEEPPRM` reader - Power Reduction Mode during Sleep"]
pub type SLEEPPRM_R = crate::FieldReader<u8, SLEEPPRMSELECT_A>;
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
impl SLEEPPRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLEEPPRMSELECT_A> {
        match self.bits {
            0 => Some(SLEEPPRMSELECT_A::WAKEONACCESS),
            1 => Some(SLEEPPRMSELECT_A::WAKEUPINSTANT),
            3 => Some(SLEEPPRMSELECT_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEONACCESS`"]
    #[inline(always)]
    pub fn is_wakeonaccess(&self) -> bool {
        *self == SLEEPPRMSELECT_A::WAKEONACCESS
    }
    #[doc = "Checks if the value of the field is `WAKEUPINSTANT`"]
    #[inline(always)]
    pub fn is_wakeupinstant(&self) -> bool {
        *self == SLEEPPRMSELECT_A::WAKEUPINSTANT
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPPRMSELECT_A::DISABLED
    }
}
#[doc = "Field `SLEEPPRM` writer - Power Reduction Mode during Sleep"]
pub type SLEEPPRM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, SLEEPPRMSELECT_A, 2, O>;
impl<'a, const O: u8> SLEEPPRM_W<'a, O> {
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn wakeonaccess(self) -> &'a mut W {
        self.variant(SLEEPPRMSELECT_A::WAKEONACCESS)
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline(always)]
    pub fn wakeupinstant(self) -> &'a mut W {
        self.variant(SLEEPPRMSELECT_A::WAKEUPINSTANT)
    }
    #[doc = "Auto power reduction disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEPPRMSELECT_A::DISABLED)
    }
}
#[doc = "Field `READMODE` reader - NVMCTRL Read Mode"]
pub type READMODE_R = crate::FieldReader<u8, READMODESELECT_A>;
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
impl READMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<READMODESELECT_A> {
        match self.bits {
            0 => Some(READMODESELECT_A::NO_MISS_PENALTY),
            1 => Some(READMODESELECT_A::LOW_POWER),
            2 => Some(READMODESELECT_A::DETERMINISTIC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MISS_PENALTY`"]
    #[inline(always)]
    pub fn is_no_miss_penalty(&self) -> bool {
        *self == READMODESELECT_A::NO_MISS_PENALTY
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == READMODESELECT_A::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `DETERMINISTIC`"]
    #[inline(always)]
    pub fn is_deterministic(&self) -> bool {
        *self == READMODESELECT_A::DETERMINISTIC
    }
}
#[doc = "Field `READMODE` writer - NVMCTRL Read Mode"]
pub type READMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, READMODESELECT_A, 2, O>;
impl<'a, const O: u8> READMODE_W<'a, O> {
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline(always)]
    pub fn no_miss_penalty(self) -> &'a mut W {
        self.variant(READMODESELECT_A::NO_MISS_PENALTY)
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(READMODESELECT_A::LOW_POWER)
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline(always)]
    pub fn deterministic(self) -> &'a mut W {
        self.variant(READMODESELECT_A::DETERMINISTIC)
    }
}
#[doc = "Field `CACHEDIS` reader - Cache Disable"]
pub type CACHEDIS_R = crate::BitReader<bool>;
#[doc = "Field `CACHEDIS` writer - Cache Disable"]
pub type CACHEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, O>;
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
    pub fn rws(&mut self) -> RWS_W<1> {
        RWS_W::new(self)
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline(always)]
    #[must_use]
    pub fn manw(&mut self) -> MANW_W<7> {
        MANW_W::new(self)
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn sleepprm(&mut self) -> SLEEPPRM_W<8> {
        SLEEPPRM_W::new(self)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn readmode(&mut self) -> READMODE_W<16> {
        READMODE_W::new(self)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cachedis(&mut self) -> CACHEDIS_W<18> {
        CACHEDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
