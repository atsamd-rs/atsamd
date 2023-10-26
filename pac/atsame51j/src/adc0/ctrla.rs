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
#[doc = "Field `DUALSEL` reader - Dual Mode Trigger Selection"]
pub type DUALSEL_R = crate::FieldReader<DUALSELSELECT_A>;
#[doc = "Dual Mode Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUALSELSELECT_A {
    #[doc = "0: Start event or software trigger will start a conversion on both ADCs"]
    BOTH = 0,
    #[doc = "1: START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    INTERLEAVE = 1,
}
impl From<DUALSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DUALSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DUALSELSELECT_A {
    type Ux = u8;
}
impl DUALSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DUALSELSELECT_A> {
        match self.bits {
            0 => Some(DUALSELSELECT_A::BOTH),
            1 => Some(DUALSELSELECT_A::INTERLEAVE),
            _ => None,
        }
    }
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == DUALSELSELECT_A::BOTH
    }
    #[doc = "START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn is_interleave(&self) -> bool {
        *self == DUALSELSELECT_A::INTERLEAVE
    }
}
#[doc = "Field `DUALSEL` writer - Dual Mode Trigger Selection"]
pub type DUALSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DUALSELSELECT_A>;
impl<'a, REG, const O: u8> DUALSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(DUALSELSELECT_A::BOTH)
    }
    #[doc = "START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn interleave(self) -> &'a mut crate::W<REG> {
        self.variant(DUALSELSELECT_A::INTERLEAVE)
    }
}
#[doc = "Field `SLAVEEN` reader - Slave Enable"]
pub type SLAVEEN_R = crate::BitReader;
#[doc = "Field `SLAVEEN` writer - Slave Enable"]
pub type SLAVEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRESCALER` reader - Prescaler Configuration"]
pub type PRESCALER_R = crate::FieldReader<PRESCALERSELECT_A>;
#[doc = "Prescaler Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALERSELECT_A {
    #[doc = "0: Peripheral clock divided by 2"]
    DIV2 = 0,
    #[doc = "1: Peripheral clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: Peripheral clock divided by 8"]
    DIV8 = 2,
    #[doc = "3: Peripheral clock divided by 16"]
    DIV16 = 3,
    #[doc = "4: Peripheral clock divided by 32"]
    DIV32 = 4,
    #[doc = "5: Peripheral clock divided by 64"]
    DIV64 = 5,
    #[doc = "6: Peripheral clock divided by 128"]
    DIV128 = 6,
    #[doc = "7: Peripheral clock divided by 256"]
    DIV256 = 7,
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
            0 => PRESCALERSELECT_A::DIV2,
            1 => PRESCALERSELECT_A::DIV4,
            2 => PRESCALERSELECT_A::DIV8,
            3 => PRESCALERSELECT_A::DIV16,
            4 => PRESCALERSELECT_A::DIV32,
            5 => PRESCALERSELECT_A::DIV64,
            6 => PRESCALERSELECT_A::DIV128,
            7 => PRESCALERSELECT_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Peripheral clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV2
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV4
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV8
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV16
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV32
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV64
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV128
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV256
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Configuration"]
pub type PRESCALER_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 3, O, PRESCALERSELECT_A>;
impl<'a, REG, const O: u8> PRESCALER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV2)
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV4)
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV8)
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV16)
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV32)
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV64)
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV128)
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALERSELECT_A::DIV256)
    }
}
#[doc = "Field `R2R` reader - Rail to Rail Operation Enable"]
pub type R2R_R = crate::BitReader;
#[doc = "Field `R2R` writer - Rail to Rail Operation Enable"]
pub type R2R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bits 3:4 - Dual Mode Trigger Selection"]
    #[inline(always)]
    pub fn dualsel(&self) -> DUALSEL_R {
        DUALSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Slave Enable"]
    #[inline(always)]
    pub fn slaveen(&self) -> SLAVEEN_R {
        SLAVEEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Rail to Rail Operation Enable"]
    #[inline(always)]
    pub fn r2r(&self) -> R2R_R {
        R2R_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bits 3:4 - Dual Mode Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dualsel(&mut self) -> DUALSEL_W<CTRLA_SPEC, 3> {
        DUALSEL_W::new(self)
    }
    #[doc = "Bit 5 - Slave Enable"]
    #[inline(always)]
    #[must_use]
    pub fn slaveen(&mut self) -> SLAVEEN_W<CTRLA_SPEC, 5> {
        SLAVEEN_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<CTRLA_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CTRLA_SPEC, 8> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 15 - Rail to Rail Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn r2r(&mut self) -> R2R_W<CTRLA_SPEC, 15> {
        R2R_W::new(self)
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
