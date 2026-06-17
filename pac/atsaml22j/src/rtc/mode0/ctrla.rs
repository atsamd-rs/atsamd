#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
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
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::COUNT32),
            1 => Some(MODESELECT_A::COUNT16),
            2 => Some(MODESELECT_A::CLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COUNT32`"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == MODESELECT_A::COUNT32
    }
    #[doc = "Checks if the value of the field is `COUNT16`"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == MODESELECT_A::COUNT16
    }
    #[doc = "Checks if the value of the field is `CLOCK`"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == MODESELECT_A::CLOCK
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLA_SPEC, u8, MODESELECT_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Mode 0: 32-bit Counter"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut W {
        self.variant(MODESELECT_A::COUNT32)
    }
    #[doc = "Mode 1: 16-bit Counter"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut W {
        self.variant(MODESELECT_A::COUNT16)
    }
    #[doc = "Mode 2: Clock/Calendar"]
    #[inline(always)]
    pub fn clock(self) -> &'a mut W {
        self.variant(MODESELECT_A::CLOCK)
    }
}
#[doc = "Field `MATCHCLR` reader - Clear on Match"]
pub type MATCHCLR_R = crate::BitReader<bool>;
#[doc = "Field `MATCHCLR` writer - Clear on Match"]
pub type MATCHCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PRESCALER_R = crate::FieldReader<u8, PRESCALERSELECT_A>;
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
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALERSELECT_A> {
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
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PRESCALERSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV1024
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CTRLA_SPEC, u8, PRESCALERSELECT_A, 4, O>;
impl<'a, const O: u8> PRESCALER_W<'a, O> {
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::OFF)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV1)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV2)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV4)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV8)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV16)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV32)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV64)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV128)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV256)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV512)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV1024)
    }
}
#[doc = "Field `BKTRST` reader - BKUP Registers Reset On Tamper Enable"]
pub type BKTRST_R = crate::BitReader<bool>;
#[doc = "Field `BKTRST` writer - BKUP Registers Reset On Tamper Enable"]
pub type BKTRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `GPTRST` reader - GP Registers Reset On Tamper Enable"]
pub type GPTRST_R = crate::BitReader<bool>;
#[doc = "Field `GPTRST` writer - GP Registers Reset On Tamper Enable"]
pub type GPTRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
#[doc = "Field `COUNTSYNC` reader - Count Read Synchronization Enable"]
pub type COUNTSYNC_R = crate::BitReader<bool>;
#[doc = "Field `COUNTSYNC` writer - Count Read Synchronization Enable"]
pub type COUNTSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLA_SPEC, bool, O>;
impl R {
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
    #[doc = "Bit 15 - Count Read Synchronization Enable"]
    #[inline(always)]
    pub fn countsync(&self) -> COUNTSYNC_R {
        COUNTSYNC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline(always)]
    #[must_use]
    pub fn matchclr(&mut self) -> MATCHCLR_W<7> {
        MATCHCLR_W::new(self)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<8> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 13 - BKUP Registers Reset On Tamper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bktrst(&mut self) -> BKTRST_W<13> {
        BKTRST_W::new(self)
    }
    #[doc = "Bit 14 - GP Registers Reset On Tamper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gptrst(&mut self) -> GPTRST_W<14> {
        GPTRST_W::new(self)
    }
    #[doc = "Bit 15 - Count Read Synchronization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn countsync(&mut self) -> COUNTSYNC_W<15> {
        COUNTSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE0 Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
