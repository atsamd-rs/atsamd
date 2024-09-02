#[doc = "Register `BOD33` reader"]
pub type R = crate::R<Bod33Spec>;
#[doc = "Register `BOD33` writer"]
pub type W = crate::W<Bod33Spec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Action when Threshold Crossed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actionselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: The BOD33 generates a reset"]
    Reset = 1,
    #[doc = "2: The BOD33 generates an interrupt"]
    Int = 2,
    #[doc = "3: The BOD33 puts the device in backup sleep mode"]
    Bkup = 3,
}
impl From<Actionselect> for u8 {
    #[inline(always)]
    fn from(variant: Actionselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actionselect {
    type Ux = u8;
}
impl crate::IsEnum for Actionselect {}
#[doc = "Field `ACTION` reader - Action when Threshold Crossed"]
pub type ActionR = crate::FieldReader<Actionselect>;
impl ActionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actionselect {
        match self.bits {
            0 => Actionselect::None,
            1 => Actionselect::Reset,
            2 => Actionselect::Int,
            3 => Actionselect::Bkup,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Actionselect::None
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Actionselect::Reset
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Actionselect::Int
    }
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    #[inline(always)]
    pub fn is_bkup(&self) -> bool {
        *self == Actionselect::Bkup
    }
}
#[doc = "Field `ACTION` writer - Action when Threshold Crossed"]
pub type ActionW<'a, REG> = crate::FieldWriter<'a, REG, 2, Actionselect, crate::Safe>;
impl<'a, REG> ActionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::None)
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::Reset)
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::Int)
    }
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    #[inline(always)]
    pub fn bkup(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::Bkup)
    }
}
#[doc = "Field `STDBYCFG` reader - Configuration in Standby mode"]
pub type StdbycfgR = crate::BitReader;
#[doc = "Field `STDBYCFG` writer - Configuration in Standby mode"]
pub type StdbycfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby mode"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby mode"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNHIB` reader - Run in Hibernate mode"]
pub type RunhibR = crate::BitReader;
#[doc = "Field `RUNHIB` writer - Run in Hibernate mode"]
pub type RunhibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNBKUP` reader - Run in Backup mode"]
pub type RunbkupR = crate::BitReader;
#[doc = "Field `RUNBKUP` writer - Run in Backup mode"]
pub type RunbkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Hysteresis value"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - Hysteresis value"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pselselect {
    #[doc = "0: Not divided"]
    Nodiv = 0,
    #[doc = "1: Divide clock by 4"]
    Div4 = 1,
    #[doc = "2: Divide clock by 8"]
    Div8 = 2,
    #[doc = "3: Divide clock by 16"]
    Div16 = 3,
    #[doc = "4: Divide clock by 32"]
    Div32 = 4,
    #[doc = "5: Divide clock by 64"]
    Div64 = 5,
    #[doc = "6: Divide clock by 128"]
    Div128 = 6,
    #[doc = "7: Divide clock by 256"]
    Div256 = 7,
}
impl From<Pselselect> for u8 {
    #[inline(always)]
    fn from(variant: Pselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselselect {
    type Ux = u8;
}
impl crate::IsEnum for Pselselect {}
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PselR = crate::FieldReader<Pselselect>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselselect {
        match self.bits {
            0 => Pselselect::Nodiv,
            1 => Pselselect::Div4,
            2 => Pselselect::Div8,
            3 => Pselselect::Div16,
            4 => Pselselect::Div32,
            5 => Pselselect::Div64,
            6 => Pselselect::Div128,
            7 => Pselselect::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "Not divided"]
    #[inline(always)]
    pub fn is_nodiv(&self) -> bool {
        *self == Pselselect::Nodiv
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pselselect::Div4
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pselselect::Div8
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Pselselect::Div16
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Pselselect::Div32
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Pselselect::Div64
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Pselselect::Div128
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Pselselect::Div256
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pselselect, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not divided"]
    #[inline(always)]
    pub fn nodiv(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Nodiv)
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div4)
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div8)
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div16)
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div32)
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div64)
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div128)
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div256)
    }
}
#[doc = "Field `LEVEL` reader - Threshold Level for VDD"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level for VDD"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VBATLEVEL` reader - Threshold Level in battery backup sleep mode for VBAT"]
pub type VbatlevelR = crate::FieldReader;
#[doc = "Field `VBATLEVEL` writer - Threshold Level in battery backup sleep mode for VBAT"]
pub type VbatlevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&self) -> ActionR {
        ActionR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline(always)]
    pub fn stdbycfg(&self) -> StdbycfgR {
        StdbycfgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline(always)]
    pub fn runhib(&self) -> RunhibR {
        RunhibR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&self) -> RunbkupR {
        RunbkupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline(always)]
    pub fn vbatlevel(&self) -> VbatlevelR {
        VbatlevelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Bod33Spec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ActionW<Bod33Spec> {
        ActionW::new(self, 2)
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdbycfg(&mut self) -> StdbycfgW<Bod33Spec> {
        StdbycfgW::new(self, 4)
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<Bod33Spec> {
        RunstdbyW::new(self, 5)
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline(always)]
    #[must_use]
    pub fn runhib(&mut self) -> RunhibW<Bod33Spec> {
        RunhibW::new(self, 6)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    #[must_use]
    pub fn runbkup(&mut self) -> RunbkupW<Bod33Spec> {
        RunbkupW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HystW<Bod33Spec> {
        HystW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<Bod33Spec> {
        PselW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LevelW<Bod33Spec> {
        LevelW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline(always)]
    #[must_use]
    pub fn vbatlevel(&mut self) -> VbatlevelW<Bod33Spec> {
        VbatlevelW::new(self, 24)
    }
}
#[doc = "BOD33 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bod33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bod33Spec;
impl crate::RegisterSpec for Bod33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod33::R`](R) reader structure"]
impl crate::Readable for Bod33Spec {}
#[doc = "`write(|w| ..)` method takes [`bod33::W`](W) writer structure"]
impl crate::Writable for Bod33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOD33 to value 0"]
impl crate::Resettable for Bod33Spec {
    const RESET_VALUE: u32 = 0;
}
