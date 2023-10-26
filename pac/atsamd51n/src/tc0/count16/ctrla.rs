#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - Timer Counter Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
#[doc = "Timer Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: Counter in 16-bit mode"]
    COUNT16 = 0,
    #[doc = "1: Counter in 8-bit mode"]
    COUNT8 = 1,
    #[doc = "2: Counter in 32-bit mode"]
    COUNT32 = 2,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODESELECT_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::COUNT16),
            1 => Some(MODESELECT_A::COUNT8),
            2 => Some(MODESELECT_A::COUNT32),
            _ => None,
        }
    }
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == MODESELECT_A::COUNT16
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn is_count8(&self) -> bool {
        *self == MODESELECT_A::COUNT8
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == MODESELECT_A::COUNT32
    }
}
#[doc = "Field `MODE` writer - Timer Counter Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MODESELECT_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::COUNT16)
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn count8(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::COUNT8)
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::COUNT32)
    }
}
#[doc = "Field `PRESCSYNC` reader - Prescaler and Counter Synchronization"]
pub type PRESCSYNC_R = crate::FieldReader<PRESCSYNCSELECT_A>;
#[doc = "Prescaler and Counter Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCSYNCSELECT_A {
    #[doc = "0: Reload or reset the counter on next generic clock"]
    GCLK = 0,
    #[doc = "1: Reload or reset the counter on next prescaler clock"]
    PRESC = 1,
    #[doc = "2: Reload or reset the counter on next generic clock and reset the prescaler counter"]
    RESYNC = 2,
}
impl From<PRESCSYNCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCSYNCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCSYNCSELECT_A {
    type Ux = u8;
}
impl PRESCSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESCSYNCSELECT_A> {
        match self.bits {
            0 => Some(PRESCSYNCSELECT_A::GCLK),
            1 => Some(PRESCSYNCSELECT_A::PRESC),
            2 => Some(PRESCSYNCSELECT_A::RESYNC),
            _ => None,
        }
    }
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == PRESCSYNCSELECT_A::GCLK
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn is_presc(&self) -> bool {
        *self == PRESCSYNCSELECT_A::PRESC
    }
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == PRESCSYNCSELECT_A::RESYNC
    }
}
#[doc = "Field `PRESCSYNC` writer - Prescaler and Counter Synchronization"]
pub type PRESCSYNC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PRESCSYNCSELECT_A>;
impl<'a, REG, const O: u8> PRESCSYNC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCSYNCSELECT_A::GCLK)
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCSYNCSELECT_A::PRESC)
    }
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCSYNCSELECT_A::RESYNC)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - Clock On Demand"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Clock On Demand"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PRESCALER_R = crate::FieldReader<PRESCALERSELECT_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALERSELECT_A {
    #[doc = "0: Prescaler: GCLK_TC"]
    DIV1 = 0,
    #[doc = "1: Prescaler: GCLK_TC/2"]
    DIV2 = 1,
    #[doc = "2: Prescaler: GCLK_TC/4"]
    DIV4 = 2,
    #[doc = "3: Prescaler: GCLK_TC/8"]
    DIV8 = 3,
    #[doc = "4: Prescaler: GCLK_TC/16"]
    DIV16 = 4,
    #[doc = "5: Prescaler: GCLK_TC/64"]
    DIV64 = 5,
    #[doc = "6: Prescaler: GCLK_TC/256"]
    DIV256 = 6,
    #[doc = "7: Prescaler: GCLK_TC/1024"]
    DIV1024 = 7,
}
impl From<PRESCALERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALERSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALERSELECT_A {
    type Ux = u8;
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESCALERSELECT_A {
        match self.bits {
            0 => PRESCALERSELECT_A::DIV1,
            1 => PRESCALERSELECT_A::DIV2,
            2 => PRESCALERSELECT_A::DIV4,
            3 => PRESCALERSELECT_A::DIV8,
            4 => PRESCALERSELECT_A::DIV16,
            5 => PRESCALERSELECT_A::DIV64,
            6 => PRESCALERSELECT_A::DIV256,
            7 => PRESCALERSELECT_A::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV1
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV2
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV4
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV8
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV16
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV64
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV256
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV1024
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PRESCALER_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 3, O, PRESCALERSELECT_A>;
impl<'a, REG, const O: u8> PRESCALER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV1)
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV2)
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV4)
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV8)
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV16)
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV64)
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV256)
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV1024)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type ALOCK_R = crate::BitReader;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type ALOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAOS` reader - DMA One-Shot Trigger Mode"]
pub type DMAOS_R = crate::BitReader;
#[doc = "Field `DMAOS` writer - DMA One-Shot Trigger Mode"]
pub type DMAOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAPTEN0` reader - Capture Channel 0 Enable"]
pub type CAPTEN0_R = crate::BitReader;
#[doc = "Field `CAPTEN0` writer - Capture Channel 0 Enable"]
pub type CAPTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAPTEN1` reader - Capture Channel 1 Enable"]
pub type CAPTEN1_R = crate::BitReader;
#[doc = "Field `CAPTEN1` writer - Capture Channel 1 Enable"]
pub type CAPTEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COPEN0` reader - Capture On Pin 0 Enable"]
pub type COPEN0_R = crate::BitReader;
#[doc = "Field `COPEN0` writer - Capture On Pin 0 Enable"]
pub type COPEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COPEN1` reader - Capture On Pin 1 Enable"]
pub type COPEN1_R = crate::BitReader;
#[doc = "Field `COPEN1` writer - Capture On Pin 1 Enable"]
pub type COPEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAPTMODE0` reader - Capture Mode Channel 0"]
pub type CAPTMODE0_R = crate::FieldReader<CAPTMODE0SELECT_A>;
#[doc = "Capture Mode Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTMODE0SELECT_A {
    #[doc = "0: Default capture"]
    DEFAULT = 0,
    #[doc = "1: Minimum capture"]
    CAPTMIN = 1,
    #[doc = "2: Maximum capture"]
    CAPTMAX = 2,
}
impl From<CAPTMODE0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTMODE0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAPTMODE0SELECT_A {
    type Ux = u8;
}
impl CAPTMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAPTMODE0SELECT_A> {
        match self.bits {
            0 => Some(CAPTMODE0SELECT_A::DEFAULT),
            1 => Some(CAPTMODE0SELECT_A::CAPTMIN),
            2 => Some(CAPTMODE0SELECT_A::CAPTMAX),
            _ => None,
        }
    }
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE0SELECT_A::DEFAULT
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE0SELECT_A::CAPTMIN
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE0SELECT_A::CAPTMAX
    }
}
#[doc = "Field `CAPTMODE0` writer - Capture Mode Channel 0"]
pub type CAPTMODE0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CAPTMODE0SELECT_A>;
impl<'a, REG, const O: u8> CAPTMODE0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTMODE0SELECT_A::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTMODE0SELECT_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTMODE0SELECT_A::CAPTMAX)
    }
}
#[doc = "Field `CAPTMODE1` reader - Capture mode Channel 1"]
pub type CAPTMODE1_R = crate::FieldReader<CAPTMODE1SELECT_A>;
#[doc = "Capture mode Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTMODE1SELECT_A {
    #[doc = "0: Default capture"]
    DEFAULT = 0,
    #[doc = "1: Minimum capture"]
    CAPTMIN = 1,
    #[doc = "2: Maximum capture"]
    CAPTMAX = 2,
}
impl From<CAPTMODE1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTMODE1SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAPTMODE1SELECT_A {
    type Ux = u8;
}
impl CAPTMODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAPTMODE1SELECT_A> {
        match self.bits {
            0 => Some(CAPTMODE1SELECT_A::DEFAULT),
            1 => Some(CAPTMODE1SELECT_A::CAPTMIN),
            2 => Some(CAPTMODE1SELECT_A::CAPTMAX),
            _ => None,
        }
    }
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE1SELECT_A::DEFAULT
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE1SELECT_A::CAPTMIN
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE1SELECT_A::CAPTMAX
    }
}
#[doc = "Field `CAPTMODE1` writer - Capture mode Channel 1"]
pub type CAPTMODE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CAPTMODE1SELECT_A>;
impl<'a, REG, const O: u8> CAPTMODE1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTMODE1SELECT_A::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTMODE1SELECT_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTMODE1SELECT_A::CAPTMAX)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&self) -> PRESCSYNC_R {
        PRESCSYNC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA One-Shot Trigger Mode"]
    #[inline(always)]
    pub fn dmaos(&self) -> DMAOS_R {
        DMAOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn capten0(&self) -> CAPTEN0_R {
        CAPTEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn capten1(&self) -> CAPTEN1_R {
        CAPTEN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    pub fn copen0(&self) -> COPEN0_R {
        COPEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    pub fn copen1(&self) -> COPEN1_R {
        COPEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    pub fn captmode0(&self) -> CAPTMODE0_R {
        CAPTMODE0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    pub fn captmode1(&self) -> CAPTMODE1_R {
        CAPTMODE1_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTRLA_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLA_SPEC, 2> {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn prescsync(&mut self) -> PRESCSYNC_W<CTRLA_SPEC, 4> {
        PRESCSYNC_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<CTRLA_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CTRLA_SPEC, 8> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    #[must_use]
    pub fn alock(&mut self) -> ALOCK_W<CTRLA_SPEC, 11> {
        ALOCK_W::new(self)
    }
    #[doc = "Bit 15 - DMA One-Shot Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmaos(&mut self) -> DMAOS_W<CTRLA_SPEC, 15> {
        DMAOS_W::new(self)
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn capten0(&mut self) -> CAPTEN0_W<CTRLA_SPEC, 16> {
        CAPTEN0_W::new(self)
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn capten1(&mut self) -> CAPTEN1_W<CTRLA_SPEC, 17> {
        CAPTEN1_W::new(self)
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn copen0(&mut self) -> COPEN0_W<CTRLA_SPEC, 20> {
        COPEN0_W::new(self)
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn copen1(&mut self) -> COPEN1_W<CTRLA_SPEC, 21> {
        COPEN1_W::new(self)
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn captmode0(&mut self) -> CAPTMODE0_W<CTRLA_SPEC, 24> {
        CAPTMODE0_W::new(self)
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn captmode1(&mut self) -> CAPTMODE1_W<CTRLA_SPEC, 27> {
        CAPTMODE1_W::new(self)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
