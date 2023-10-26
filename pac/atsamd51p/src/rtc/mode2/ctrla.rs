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
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: Mode 0: 32-bit Counter"]
    COUNT32 = 0,
    #[doc = "1: Mode 1: 16-bit Counter"]
    COUNT16 = 1,
    #[doc = "2: Mode 2: Clock/Calendar"]
    CLOCK = 2,
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
            0 => Some(MODESELECT_A::COUNT32),
            1 => Some(MODESELECT_A::COUNT16),
            2 => Some(MODESELECT_A::CLOCK),
            _ => None,
        }
    }
    #[doc = "Mode 0: 32-bit Counter"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == MODESELECT_A::COUNT32
    }
    #[doc = "Mode 1: 16-bit Counter"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == MODESELECT_A::COUNT16
    }
    #[doc = "Mode 2: Clock/Calendar"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == MODESELECT_A::CLOCK
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MODESELECT_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0: 32-bit Counter"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::COUNT32)
    }
    #[doc = "Mode 1: 16-bit Counter"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::COUNT16)
    }
    #[doc = "Mode 2: Clock/Calendar"]
    #[inline(always)]
    pub fn clock(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::CLOCK)
    }
}
#[doc = "Field `CLKREP` reader - Clock Representation"]
pub type CLKREP_R = crate::BitReader;
#[doc = "Field `CLKREP` writer - Clock Representation"]
pub type CLKREP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MATCHCLR` reader - Clear on Match"]
pub type MATCHCLR_R = crate::BitReader;
#[doc = "Field `MATCHCLR` writer - Clear on Match"]
pub type MATCHCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PRESCALER_R = crate::FieldReader<PRESCALERSELECT_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALERSELECT_A {
    #[doc = "0: CLK_RTC_CNT = GCLK_RTC/1"]
    OFF = 0,
    #[doc = "1: CLK_RTC_CNT = GCLK_RTC/1"]
    DIV1 = 1,
    #[doc = "2: CLK_RTC_CNT = GCLK_RTC/2"]
    DIV2 = 2,
    #[doc = "3: CLK_RTC_CNT = GCLK_RTC/4"]
    DIV4 = 3,
    #[doc = "4: CLK_RTC_CNT = GCLK_RTC/8"]
    DIV8 = 4,
    #[doc = "5: CLK_RTC_CNT = GCLK_RTC/16"]
    DIV16 = 5,
    #[doc = "6: CLK_RTC_CNT = GCLK_RTC/32"]
    DIV32 = 6,
    #[doc = "7: CLK_RTC_CNT = GCLK_RTC/64"]
    DIV64 = 7,
    #[doc = "8: CLK_RTC_CNT = GCLK_RTC/128"]
    DIV128 = 8,
    #[doc = "9: CLK_RTC_CNT = GCLK_RTC/256"]
    DIV256 = 9,
    #[doc = "10: CLK_RTC_CNT = GCLK_RTC/512"]
    DIV512 = 10,
    #[doc = "11: CLK_RTC_CNT = GCLK_RTC/1024"]
    DIV1024 = 11,
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
    pub const fn variant(&self) -> Option<PRESCALERSELECT_A> {
        match self.bits {
            0 => Some(PRESCALERSELECT_A::OFF),
            1 => Some(PRESCALERSELECT_A::DIV1),
            2 => Some(PRESCALERSELECT_A::DIV2),
            3 => Some(PRESCALERSELECT_A::DIV4),
            4 => Some(PRESCALERSELECT_A::DIV8),
            5 => Some(PRESCALERSELECT_A::DIV16),
            6 => Some(PRESCALERSELECT_A::DIV32),
            7 => Some(PRESCALERSELECT_A::DIV64),
            8 => Some(PRESCALERSELECT_A::DIV128),
            9 => Some(PRESCALERSELECT_A::DIV256),
            10 => Some(PRESCALERSELECT_A::DIV512),
            11 => Some(PRESCALERSELECT_A::DIV1024),
            _ => None,
        }
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PRESCALERSELECT_A::OFF
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV1
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV2
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV4
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV8
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV16
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV32
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV64
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV128
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV256
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV512
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV1024
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PRESCALER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PRESCALERSELECT_A>;
impl<'a, REG, const O: u8> PRESCALER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::OFF)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV1)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV2)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV4)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV8)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV16)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV32)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV64)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV128)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV256)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV512)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV1024)
    }
}
#[doc = "Field `BKTRST` reader - BKUP Registers Reset On Tamper Enable"]
pub type BKTRST_R = crate::BitReader;
#[doc = "Field `BKTRST` writer - BKUP Registers Reset On Tamper Enable"]
pub type BKTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPTRST` reader - GP Registers Reset On Tamper Enable"]
pub type GPTRST_R = crate::BitReader;
#[doc = "Field `GPTRST` writer - GP Registers Reset On Tamper Enable"]
pub type GPTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLOCKSYNC` reader - Clock Read Synchronization Enable"]
pub type CLOCKSYNC_R = crate::BitReader;
#[doc = "Field `CLOCKSYNC` writer - Clock Read Synchronization Enable"]
pub type CLOCKSYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Clock Representation"]
    #[inline(always)]
    pub fn clkrep(&self) -> CLKREP_R {
        CLKREP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline(always)]
    pub fn matchclr(&self) -> MATCHCLR_R {
        MATCHCLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - BKUP Registers Reset On Tamper Enable"]
    #[inline(always)]
    pub fn bktrst(&self) -> BKTRST_R {
        BKTRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GP Registers Reset On Tamper Enable"]
    #[inline(always)]
    pub fn gptrst(&self) -> GPTRST_R {
        GPTRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable"]
    #[inline(always)]
    pub fn clocksync(&self) -> CLOCKSYNC_R {
        CLOCKSYNC_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLA_SPEC, 2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 6 - Clock Representation"]
    #[inline(always)]
    #[must_use]
    pub fn clkrep(&mut self) -> CLKREP_W<CTRLA_SPEC, 6> {
        CLKREP_W::new(self)
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline(always)]
    #[must_use]
    pub fn matchclr(&mut self) -> MATCHCLR_W<CTRLA_SPEC, 7> {
        MATCHCLR_W::new(self)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CTRLA_SPEC, 8> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 13 - BKUP Registers Reset On Tamper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bktrst(&mut self) -> BKTRST_W<CTRLA_SPEC, 13> {
        BKTRST_W::new(self)
    }
    #[doc = "Bit 14 - GP Registers Reset On Tamper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gptrst(&mut self) -> GPTRST_W<CTRLA_SPEC, 14> {
        GPTRST_W::new(self)
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clocksync(&mut self) -> CLOCKSYNC_W<CTRLA_SPEC, 15> {
        CLOCKSYNC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MODE2 Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u16;
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
