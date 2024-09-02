#[doc = "Register `BOD33` reader"]
pub type R = crate::R<Bod33Spec>;
#[doc = "Register `BOD33` writer"]
pub type W = crate::W<Bod33Spec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Hysteresis"]
pub type HystR = crate::BitReader;
#[doc = "Field `HYST` writer - Hysteresis"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "BOD33 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actionselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: The BOD33 generates a reset"]
    Reset = 1,
    #[doc = "2: The BOD33 generates an interrupt"]
    Interrupt = 2,
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
#[doc = "Field `ACTION` reader - BOD33 Action"]
pub type ActionR = crate::FieldReader<Actionselect>;
impl ActionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Actionselect> {
        match self.bits {
            0 => Some(Actionselect::None),
            1 => Some(Actionselect::Reset),
            2 => Some(Actionselect::Interrupt),
            _ => None,
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
    pub fn is_interrupt(&self) -> bool {
        *self == Actionselect::Interrupt
    }
}
#[doc = "Field `ACTION` writer - BOD33 Action"]
pub type ActionW<'a, REG> = crate::FieldWriter<'a, REG, 2, Actionselect>;
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
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::Interrupt)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Operation Mode"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Operation Mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - Clock Enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Clock Enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pselselect {
    #[doc = "0: Divide clock by 2"]
    Div2 = 0,
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
    #[doc = "8: Divide clock by 512"]
    Div512 = 8,
    #[doc = "9: Divide clock by 1024"]
    Div1k = 9,
    #[doc = "10: Divide clock by 2048"]
    Div2k = 10,
    #[doc = "11: Divide clock by 4096"]
    Div4k = 11,
    #[doc = "12: Divide clock by 8192"]
    Div8k = 12,
    #[doc = "13: Divide clock by 16384"]
    Div16k = 13,
    #[doc = "14: Divide clock by 32768"]
    Div32k = 14,
    #[doc = "15: Divide clock by 65536"]
    Div64k = 15,
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
            0 => Pselselect::Div2,
            1 => Pselselect::Div4,
            2 => Pselselect::Div8,
            3 => Pselselect::Div16,
            4 => Pselselect::Div32,
            5 => Pselselect::Div64,
            6 => Pselselect::Div128,
            7 => Pselselect::Div256,
            8 => Pselselect::Div512,
            9 => Pselselect::Div1k,
            10 => Pselselect::Div2k,
            11 => Pselselect::Div4k,
            12 => Pselselect::Div8k,
            13 => Pselselect::Div16k,
            14 => Pselselect::Div32k,
            15 => Pselselect::Div64k,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pselselect::Div2
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
    #[doc = "Divide clock by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Pselselect::Div512
    }
    #[doc = "Divide clock by 1024"]
    #[inline(always)]
    pub fn is_div1k(&self) -> bool {
        *self == Pselselect::Div1k
    }
    #[doc = "Divide clock by 2048"]
    #[inline(always)]
    pub fn is_div2k(&self) -> bool {
        *self == Pselselect::Div2k
    }
    #[doc = "Divide clock by 4096"]
    #[inline(always)]
    pub fn is_div4k(&self) -> bool {
        *self == Pselselect::Div4k
    }
    #[doc = "Divide clock by 8192"]
    #[inline(always)]
    pub fn is_div8k(&self) -> bool {
        *self == Pselselect::Div8k
    }
    #[doc = "Divide clock by 16384"]
    #[inline(always)]
    pub fn is_div16k(&self) -> bool {
        *self == Pselselect::Div16k
    }
    #[doc = "Divide clock by 32768"]
    #[inline(always)]
    pub fn is_div32k(&self) -> bool {
        *self == Pselselect::Div32k
    }
    #[doc = "Divide clock by 65536"]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        *self == Pselselect::Div64k
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pselselect, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div2)
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
    #[doc = "Divide clock by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div512)
    }
    #[doc = "Divide clock by 1024"]
    #[inline(always)]
    pub fn div1k(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div1k)
    }
    #[doc = "Divide clock by 2048"]
    #[inline(always)]
    pub fn div2k(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div2k)
    }
    #[doc = "Divide clock by 4096"]
    #[inline(always)]
    pub fn div4k(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div4k)
    }
    #[doc = "Divide clock by 8192"]
    #[inline(always)]
    pub fn div8k(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div8k)
    }
    #[doc = "Divide clock by 16384"]
    #[inline(always)]
    pub fn div16k(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div16k)
    }
    #[doc = "Divide clock by 32768"]
    #[inline(always)]
    pub fn div32k(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div32k)
    }
    #[doc = "Divide clock by 65536"]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div64k)
    }
}
#[doc = "Field `LEVEL` reader - BOD33 Threshold Level"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - BOD33 Threshold Level"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline(always)]
    pub fn action(&self) -> ActionR {
        ActionR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Bod33Spec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HystW<Bod33Spec> {
        HystW::new(self, 2)
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ActionW<Bod33Spec> {
        ActionW::new(self, 3)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<Bod33Spec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Bod33Spec> {
        ModeW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<Bod33Spec> {
        CenW::new(self, 9)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<Bod33Spec> {
        PselW::new(self, 12)
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LevelW<Bod33Spec> {
        LevelW::new(self, 16)
    }
}
#[doc = "3.3V Brown-Out Detector (BOD33) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bod33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
