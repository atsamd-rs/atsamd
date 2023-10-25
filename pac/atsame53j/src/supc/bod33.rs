#[doc = "Register `BOD33` reader"]
pub type R = crate::R<BOD33_SPEC>;
#[doc = "Register `BOD33` writer"]
pub type W = crate::W<BOD33_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACTION` reader - Action when Threshold Crossed"]
pub type ACTION_R = crate::FieldReader<ACTIONSELECT_A>;
#[doc = "Action when Threshold Crossed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIONSELECT_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: The BOD33 generates a reset"]
    RESET = 1,
    #[doc = "2: The BOD33 generates an interrupt"]
    INT = 2,
    #[doc = "3: The BOD33 puts the device in backup sleep mode"]
    BKUP = 3,
}
impl From<ACTIONSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIONSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTIONSELECT_A {
    type Ux = u8;
}
impl ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTIONSELECT_A {
        match self.bits {
            0 => ACTIONSELECT_A::NONE,
            1 => ACTIONSELECT_A::RESET,
            2 => ACTIONSELECT_A::INT,
            3 => ACTIONSELECT_A::BKUP,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTIONSELECT_A::NONE
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ACTIONSELECT_A::RESET
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == ACTIONSELECT_A::INT
    }
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    #[inline(always)]
    pub fn is_bkup(&self) -> bool {
        *self == ACTIONSELECT_A::BKUP
    }
}
#[doc = "Field `ACTION` writer - Action when Threshold Crossed"]
pub type ACTION_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ACTIONSELECT_A>;
impl<'a, REG, const O: u8> ACTION_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIONSELECT_A::NONE)
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIONSELECT_A::RESET)
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIONSELECT_A::INT)
    }
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    #[inline(always)]
    pub fn bkup(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIONSELECT_A::BKUP)
    }
}
#[doc = "Field `STDBYCFG` reader - Configuration in Standby mode"]
pub type STDBYCFG_R = crate::BitReader;
#[doc = "Field `STDBYCFG` writer - Configuration in Standby mode"]
pub type STDBYCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby mode"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby mode"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNHIB` reader - Run in Hibernate mode"]
pub type RUNHIB_R = crate::BitReader;
#[doc = "Field `RUNHIB` writer - Run in Hibernate mode"]
pub type RUNHIB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNBKUP` reader - Run in Backup mode"]
pub type RUNBKUP_R = crate::BitReader;
#[doc = "Field `RUNBKUP` writer - Run in Backup mode"]
pub type RUNBKUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HYST` reader - Hysteresis value"]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - Hysteresis value"]
pub type HYST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PSEL_R = crate::FieldReader<PSELSELECT_A>;
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSELSELECT_A {
    #[doc = "0: Not divided"]
    NODIV = 0,
    #[doc = "1: Divide clock by 4"]
    DIV4 = 1,
    #[doc = "2: Divide clock by 8"]
    DIV8 = 2,
    #[doc = "3: Divide clock by 16"]
    DIV16 = 3,
    #[doc = "4: Divide clock by 32"]
    DIV32 = 4,
    #[doc = "5: Divide clock by 64"]
    DIV64 = 5,
    #[doc = "6: Divide clock by 128"]
    DIV128 = 6,
    #[doc = "7: Divide clock by 256"]
    DIV256 = 7,
}
impl From<PSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSELSELECT_A {
    type Ux = u8;
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELSELECT_A {
        match self.bits {
            0 => PSELSELECT_A::NODIV,
            1 => PSELSELECT_A::DIV4,
            2 => PSELSELECT_A::DIV8,
            3 => PSELSELECT_A::DIV16,
            4 => PSELSELECT_A::DIV32,
            5 => PSELSELECT_A::DIV64,
            6 => PSELSELECT_A::DIV128,
            7 => PSELSELECT_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Not divided"]
    #[inline(always)]
    pub fn is_nodiv(&self) -> bool {
        *self == PSELSELECT_A::NODIV
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSELSELECT_A::DIV4
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSELSELECT_A::DIV8
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSELSELECT_A::DIV16
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PSELSELECT_A::DIV32
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PSELSELECT_A::DIV64
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PSELSELECT_A::DIV128
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PSELSELECT_A::DIV256
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, PSELSELECT_A>;
impl<'a, REG, const O: u8> PSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not divided"]
    #[inline(always)]
    pub fn nodiv(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::NODIV)
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::DIV4)
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::DIV8)
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::DIV16)
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::DIV32)
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::DIV64)
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::DIV128)
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PSELSELECT_A::DIV256)
    }
}
#[doc = "Field `LEVEL` reader - Threshold Level for VDD"]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level for VDD"]
pub type LEVEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VBATLEVEL` reader - Threshold Level in battery backup sleep mode for VBAT"]
pub type VBATLEVEL_R = crate::FieldReader;
#[doc = "Field `VBATLEVEL` writer - Threshold Level in battery backup sleep mode for VBAT"]
pub type VBATLEVEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline(always)]
    pub fn stdbycfg(&self) -> STDBYCFG_R {
        STDBYCFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline(always)]
    pub fn runhib(&self) -> RUNHIB_R {
        RUNHIB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&self) -> RUNBKUP_R {
        RUNBKUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline(always)]
    pub fn vbatlevel(&self) -> VBATLEVEL_R {
        VBATLEVEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<BOD33_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<BOD33_SPEC, 2> {
        ACTION_W::new(self)
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdbycfg(&mut self) -> STDBYCFG_W<BOD33_SPEC, 4> {
        STDBYCFG_W::new(self)
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<BOD33_SPEC, 5> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline(always)]
    #[must_use]
    pub fn runhib(&mut self) -> RUNHIB_W<BOD33_SPEC, 6> {
        RUNHIB_W::new(self)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    #[must_use]
    pub fn runbkup(&mut self) -> RUNBKUP_W<BOD33_SPEC, 7> {
        RUNBKUP_W::new(self)
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<BOD33_SPEC, 8> {
        HYST_W::new(self)
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<BOD33_SPEC, 12> {
        PSEL_W::new(self)
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<BOD33_SPEC, 16> {
        LEVEL_W::new(self)
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline(always)]
    #[must_use]
    pub fn vbatlevel(&mut self) -> VBATLEVEL_W<BOD33_SPEC, 24> {
        VBATLEVEL_W::new(self)
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
#[doc = "BOD33 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bod33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bod33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOD33_SPEC;
impl crate::RegisterSpec for BOD33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod33::R`](R) reader structure"]
impl crate::Readable for BOD33_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bod33::W`](W) writer structure"]
impl crate::Writable for BOD33_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD33 to value 0"]
impl crate::Resettable for BOD33_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
