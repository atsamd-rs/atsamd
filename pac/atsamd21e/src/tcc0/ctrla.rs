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
#[doc = "Field `RESOLUTION` reader - Enhanced Resolution"]
pub type RESOLUTION_R = crate::FieldReader<RESOLUTIONSELECT_A>;
#[doc = "Enhanced Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESOLUTIONSELECT_A {
    #[doc = "0: Dithering is disabled"]
    NONE = 0,
    #[doc = "1: Dithering is done every 16 PWM frames"]
    DITH4 = 1,
    #[doc = "2: Dithering is done every 32 PWM frames"]
    DITH5 = 2,
    #[doc = "3: Dithering is done every 64 PWM frames"]
    DITH6 = 3,
}
impl From<RESOLUTIONSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RESOLUTIONSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESOLUTIONSELECT_A {
    type Ux = u8;
}
impl RESOLUTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESOLUTIONSELECT_A {
        match self.bits {
            0 => RESOLUTIONSELECT_A::NONE,
            1 => RESOLUTIONSELECT_A::DITH4,
            2 => RESOLUTIONSELECT_A::DITH5,
            3 => RESOLUTIONSELECT_A::DITH6,
            _ => unreachable!(),
        }
    }
    #[doc = "Dithering is disabled"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RESOLUTIONSELECT_A::NONE
    }
    #[doc = "Dithering is done every 16 PWM frames"]
    #[inline(always)]
    pub fn is_dith4(&self) -> bool {
        *self == RESOLUTIONSELECT_A::DITH4
    }
    #[doc = "Dithering is done every 32 PWM frames"]
    #[inline(always)]
    pub fn is_dith5(&self) -> bool {
        *self == RESOLUTIONSELECT_A::DITH5
    }
    #[doc = "Dithering is done every 64 PWM frames"]
    #[inline(always)]
    pub fn is_dith6(&self) -> bool {
        *self == RESOLUTIONSELECT_A::DITH6
    }
}
#[doc = "Field `RESOLUTION` writer - Enhanced Resolution"]
pub type RESOLUTION_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, RESOLUTIONSELECT_A>;
impl<'a, REG, const O: u8> RESOLUTION_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dithering is disabled"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RESOLUTIONSELECT_A::NONE)
    }
    #[doc = "Dithering is done every 16 PWM frames"]
    #[inline(always)]
    pub fn dith4(self) -> &'a mut crate::W<REG> {
        self.variant(RESOLUTIONSELECT_A::DITH4)
    }
    #[doc = "Dithering is done every 32 PWM frames"]
    #[inline(always)]
    pub fn dith5(self) -> &'a mut crate::W<REG> {
        self.variant(RESOLUTIONSELECT_A::DITH5)
    }
    #[doc = "Dithering is done every 64 PWM frames"]
    #[inline(always)]
    pub fn dith6(self) -> &'a mut crate::W<REG> {
        self.variant(RESOLUTIONSELECT_A::DITH6)
    }
}
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PRESCALER_R = crate::FieldReader<PRESCALERSELECT_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALERSELECT_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 4"]
    DIV4 = 2,
    #[doc = "3: Divide by 8"]
    DIV8 = 3,
    #[doc = "4: Divide by 16"]
    DIV16 = 4,
    #[doc = "5: Divide by 64"]
    DIV64 = 5,
    #[doc = "6: Divide by 256"]
    DIV256 = 6,
    #[doc = "7: Divide by 1024"]
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
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV16
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV64
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV256
    }
    #[doc = "Divide by 1024"]
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
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV16)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV64)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV256)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV1024)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRESCSYNC` reader - Prescaler and Counter Synchronization Selection"]
pub type PRESCSYNC_R = crate::FieldReader<PRESCSYNCSELECT_A>;
#[doc = "Prescaler and Counter Synchronization Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCSYNCSELECT_A {
    #[doc = "0: Reload or reset counter on next GCLK"]
    GCLK = 0,
    #[doc = "1: Reload or reset counter on next prescaler clock"]
    PRESC = 1,
    #[doc = "2: Reload or reset counter on next GCLK and reset prescaler counter"]
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
    #[doc = "Reload or reset counter on next GCLK"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == PRESCSYNCSELECT_A::GCLK
    }
    #[doc = "Reload or reset counter on next prescaler clock"]
    #[inline(always)]
    pub fn is_presc(&self) -> bool {
        *self == PRESCSYNCSELECT_A::PRESC
    }
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == PRESCSYNCSELECT_A::RESYNC
    }
}
#[doc = "Field `PRESCSYNC` writer - Prescaler and Counter Synchronization Selection"]
pub type PRESCSYNC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PRESCSYNCSELECT_A>;
impl<'a, REG, const O: u8> PRESCSYNC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reload or reset counter on next GCLK"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCSYNCSELECT_A::GCLK)
    }
    #[doc = "Reload or reset counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCSYNCSELECT_A::PRESC)
    }
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCSYNCSELECT_A::RESYNC)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type ALOCK_R = crate::BitReader;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type ALOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPTEN0` reader - Capture Channel 0 Enable"]
pub type CPTEN0_R = crate::BitReader;
#[doc = "Field `CPTEN0` writer - Capture Channel 0 Enable"]
pub type CPTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPTEN1` reader - Capture Channel 1 Enable"]
pub type CPTEN1_R = crate::BitReader;
#[doc = "Field `CPTEN1` writer - Capture Channel 1 Enable"]
pub type CPTEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPTEN2` reader - Capture Channel 2 Enable"]
pub type CPTEN2_R = crate::BitReader;
#[doc = "Field `CPTEN2` writer - Capture Channel 2 Enable"]
pub type CPTEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPTEN3` reader - Capture Channel 3 Enable"]
pub type CPTEN3_R = crate::BitReader;
#[doc = "Field `CPTEN3` writer - Capture Channel 3 Enable"]
pub type CPTEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline(always)]
    pub fn resolution(&self) -> RESOLUTION_R {
        RESOLUTION_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline(always)]
    pub fn prescsync(&self) -> PRESCSYNC_R {
        PRESCSYNC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> CPTEN0_R {
        CPTEN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> CPTEN1_R {
        CPTEN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline(always)]
    pub fn cpten2(&self) -> CPTEN2_R {
        CPTEN2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline(always)]
    pub fn cpten3(&self) -> CPTEN3_R {
        CPTEN3_R::new(((self.bits >> 27) & 1) != 0)
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
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn resolution(&mut self) -> RESOLUTION_W<CTRLA_SPEC, 5> {
        RESOLUTION_W::new(self)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CTRLA_SPEC, 8> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC, 11> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prescsync(&mut self) -> PRESCSYNC_W<CTRLA_SPEC, 12> {
        PRESCSYNC_W::new(self)
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline(always)]
    #[must_use]
    pub fn alock(&mut self) -> ALOCK_W<CTRLA_SPEC, 14> {
        ALOCK_W::new(self)
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten0(&mut self) -> CPTEN0_W<CTRLA_SPEC, 24> {
        CPTEN0_W::new(self)
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten1(&mut self) -> CPTEN1_W<CTRLA_SPEC, 25> {
        CPTEN1_W::new(self)
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten2(&mut self) -> CPTEN2_W<CTRLA_SPEC, 26> {
        CPTEN2_W::new(self)
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten3(&mut self) -> CPTEN3_W<CTRLA_SPEC, 27> {
        CPTEN3_W::new(self)
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
