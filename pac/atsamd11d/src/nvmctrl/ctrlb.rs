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
#[doc = "NVM Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RWS_A {
    #[doc = "0: Single Auto Wait State"]
    SINGLE = 0,
    #[doc = "1: Half Auto Wait State"]
    HALF = 1,
    #[doc = "2: Dual Auto Wait State"]
    DUAL = 2,
}
impl From<RWS_A> for u8 {
    #[inline(always)]
    fn from(variant: RWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RWS` reader - NVM Read Wait States"]
pub struct RWS_R(crate::FieldReader<u8, RWS_A>);
impl RWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RWS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RWS_A> {
        match self.bits {
            0 => Some(RWS_A::SINGLE),
            1 => Some(RWS_A::HALF),
            2 => Some(RWS_A::DUAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == RWS_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == RWS_A::HALF
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == RWS_A::DUAL
    }
}
impl core::ops::Deref for RWS_R {
    type Target = crate::FieldReader<u8, RWS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWS` writer - NVM Read Wait States"]
pub struct RWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single Auto Wait State"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(RWS_A::SINGLE)
    }
    #[doc = "Half Auto Wait State"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(RWS_A::HALF)
    }
    #[doc = "Dual Auto Wait State"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(RWS_A::DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `MANW` reader - Manual Write"]
pub struct MANW_R(crate::FieldReader<bool, bool>);
impl MANW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MANW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MANW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MANW` writer - Manual Write"]
pub struct MANW_W<'a> {
    w: &'a mut W,
}
impl<'a> MANW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Power Reduction Mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLEEPPRM_A {
    #[doc = "0: NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    WAKEONACCESS = 0,
    #[doc = "1: NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    WAKEUPINSTANT = 1,
    #[doc = "3: Auto power reduction disabled."]
    DISABLED = 3,
}
impl From<SLEEPPRM_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEPPRM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SLEEPPRM` reader - Power Reduction Mode during Sleep"]
pub struct SLEEPPRM_R(crate::FieldReader<u8, SLEEPPRM_A>);
impl SLEEPPRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLEEPPRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLEEPPRM_A> {
        match self.bits {
            0 => Some(SLEEPPRM_A::WAKEONACCESS),
            1 => Some(SLEEPPRM_A::WAKEUPINSTANT),
            3 => Some(SLEEPPRM_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEONACCESS`"]
    #[inline(always)]
    pub fn is_wakeonaccess(&self) -> bool {
        **self == SLEEPPRM_A::WAKEONACCESS
    }
    #[doc = "Checks if the value of the field is `WAKEUPINSTANT`"]
    #[inline(always)]
    pub fn is_wakeupinstant(&self) -> bool {
        **self == SLEEPPRM_A::WAKEUPINSTANT
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SLEEPPRM_A::DISABLED
    }
}
impl core::ops::Deref for SLEEPPRM_R {
    type Target = crate::FieldReader<u8, SLEEPPRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPPRM` writer - Power Reduction Mode during Sleep"]
pub struct SLEEPPRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPPRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPPRM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn wakeonaccess(self) -> &'a mut W {
        self.variant(SLEEPPRM_A::WAKEONACCESS)
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline(always)]
    pub fn wakeupinstant(self) -> &'a mut W {
        self.variant(SLEEPPRM_A::WAKEUPINSTANT)
    }
    #[doc = "Auto power reduction disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEPPRM_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "NVMCTRL Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum READMODE_A {
    #[doc = "0: The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NO_MISS_PENALTY = 0,
    #[doc = "1: Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LOW_POWER = 1,
    #[doc = "2: The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    DETERMINISTIC = 2,
}
impl From<READMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: READMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `READMODE` reader - NVMCTRL Read Mode"]
pub struct READMODE_R(crate::FieldReader<u8, READMODE_A>);
impl READMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        READMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<READMODE_A> {
        match self.bits {
            0 => Some(READMODE_A::NO_MISS_PENALTY),
            1 => Some(READMODE_A::LOW_POWER),
            2 => Some(READMODE_A::DETERMINISTIC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MISS_PENALTY`"]
    #[inline(always)]
    pub fn is_no_miss_penalty(&self) -> bool {
        **self == READMODE_A::NO_MISS_PENALTY
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        **self == READMODE_A::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `DETERMINISTIC`"]
    #[inline(always)]
    pub fn is_deterministic(&self) -> bool {
        **self == READMODE_A::DETERMINISTIC
    }
}
impl core::ops::Deref for READMODE_R {
    type Target = crate::FieldReader<u8, READMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READMODE` writer - NVMCTRL Read Mode"]
pub struct READMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> READMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline(always)]
    pub fn no_miss_penalty(self) -> &'a mut W {
        self.variant(READMODE_A::NO_MISS_PENALTY)
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(READMODE_A::LOW_POWER)
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline(always)]
    pub fn deterministic(self) -> &'a mut W {
        self.variant(READMODE_A::DETERMINISTIC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CACHEDIS` reader - Cache Disable"]
pub struct CACHEDIS_R(crate::FieldReader<bool, bool>);
impl CACHEDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHEDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHEDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEDIS` writer - Cache Disable"]
pub struct CACHEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline(always)]
    pub fn manw(&self) -> MANW_R {
        MANW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn sleepprm(&self) -> SLEEPPRM_R {
        SLEEPPRM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    pub fn readmode(&self) -> READMODE_R {
        READMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&self) -> CACHEDIS_R {
        CACHEDIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&mut self) -> RWS_W {
        RWS_W { w: self }
    }
    #[doc = "Bit 7 - Manual Write"]
    #[inline(always)]
    pub fn manw(&mut self) -> MANW_W {
        MANW_W { w: self }
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn sleepprm(&mut self) -> SLEEPPRM_W {
        SLEEPPRM_W { w: self }
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    pub fn readmode(&mut self) -> READMODE_W {
        READMODE_W { w: self }
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&mut self) -> CACHEDIS_W {
        CACHEDIS_W { w: self }
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
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
