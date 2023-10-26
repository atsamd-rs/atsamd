#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - TC Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
#[doc = "TC Mode\n\nValue on reset: 0"]
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
#[doc = "Field `MODE` writer - TC Mode"]
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
#[doc = "Field `WAVEGEN` reader - Waveform Generation Operation"]
pub type WAVEGEN_R = crate::FieldReader<WAVEGENSELECT_A>;
#[doc = "Waveform Generation Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVEGENSELECT_A {
    #[doc = "0: `0`"]
    NFRQ = 0,
    #[doc = "1: `1`"]
    MFRQ = 1,
    #[doc = "2: `10`"]
    NPWM = 2,
    #[doc = "3: `11`"]
    MPWM = 3,
}
impl From<WAVEGENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVEGENSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAVEGENSELECT_A {
    type Ux = u8;
}
impl WAVEGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAVEGENSELECT_A {
        match self.bits {
            0 => WAVEGENSELECT_A::NFRQ,
            1 => WAVEGENSELECT_A::MFRQ,
            2 => WAVEGENSELECT_A::NPWM,
            3 => WAVEGENSELECT_A::MPWM,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENSELECT_A::NFRQ
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENSELECT_A::MFRQ
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENSELECT_A::NPWM
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_mpwm(&self) -> bool {
        *self == WAVEGENSELECT_A::MPWM
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation Operation"]
pub type WAVEGEN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, WAVEGENSELECT_A>;
impl<'a, REG, const O: u8> WAVEGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::NFRQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::MFRQ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::NPWM)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn mpwm(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::MPWM)
    }
}
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
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "2: Reload or reset the counter on next generic clock. Reset the prescaler counter"]
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
    #[doc = "Reload or reset the counter on next generic clock. Reset the prescaler counter"]
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
    #[doc = "Reload or reset the counter on next generic clock. Reset the prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCSYNCSELECT_A::RESYNC)
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - TC Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Waveform Generation Operation"]
    #[inline(always)]
    pub fn wavegen(&self) -> WAVEGEN_R {
        WAVEGEN_R::new(((self.bits >> 5) & 3) as u8)
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
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&self) -> PRESCSYNC_R {
        PRESCSYNC_R::new(((self.bits >> 12) & 3) as u8)
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
    #[doc = "Bits 2:3 - TC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLA_SPEC, 2> {
        MODE_W::new(self)
    }
    #[doc = "Bits 5:6 - Waveform Generation Operation"]
    #[inline(always)]
    #[must_use]
    pub fn wavegen(&mut self) -> WAVEGEN_W<CTRLA_SPEC, 5> {
        WAVEGEN_W::new(self)
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
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn prescsync(&mut self) -> PRESCSYNC_W<CTRLA_SPEC, 12> {
        PRESCSYNC_W::new(self)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
