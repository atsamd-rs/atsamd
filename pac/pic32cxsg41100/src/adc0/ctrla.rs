#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Dual Mode Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dualselselect {
    #[doc = "0: Start event or software trigger will start a conversion on both ADCs"]
    Both = 0,
    #[doc = "1: START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    Interleave = 1,
}
impl From<Dualselselect> for u8 {
    #[inline(always)]
    fn from(variant: Dualselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dualselselect {
    type Ux = u8;
}
impl crate::IsEnum for Dualselselect {}
#[doc = "Field `DUALSEL` reader - Dual Mode Trigger Selection"]
pub type DualselR = crate::FieldReader<Dualselselect>;
impl DualselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dualselselect> {
        match self.bits {
            0 => Some(Dualselselect::Both),
            1 => Some(Dualselselect::Interleave),
            _ => None,
        }
    }
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Dualselselect::Both
    }
    #[doc = "START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn is_interleave(&self) -> bool {
        *self == Dualselselect::Interleave
    }
}
#[doc = "Field `DUALSEL` writer - Dual Mode Trigger Selection"]
pub type DualselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dualselselect>;
impl<'a, REG> DualselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Dualselselect::Both)
    }
    #[doc = "START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn interleave(self) -> &'a mut crate::W<REG> {
        self.variant(Dualselselect::Interleave)
    }
}
#[doc = "Field `SLAVEEN` reader - Slave Enable"]
pub type SlaveenR = crate::BitReader;
#[doc = "Field `SLAVEEN` writer - Slave Enable"]
pub type SlaveenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: Peripheral clock divided by 2"]
    Div2 = 0,
    #[doc = "1: Peripheral clock divided by 4"]
    Div4 = 1,
    #[doc = "2: Peripheral clock divided by 8"]
    Div8 = 2,
    #[doc = "3: Peripheral clock divided by 16"]
    Div16 = 3,
    #[doc = "4: Peripheral clock divided by 32"]
    Div32 = 4,
    #[doc = "5: Peripheral clock divided by 64"]
    Div64 = 5,
    #[doc = "6: Peripheral clock divided by 128"]
    Div128 = 6,
    #[doc = "7: Peripheral clock divided by 256"]
    Div256 = 7,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaler Configuration"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescalerselect {
        match self.bits {
            0 => Prescalerselect::Div2,
            1 => Prescalerselect::Div4,
            2 => Prescalerselect::Div8,
            3 => Prescalerselect::Div16,
            4 => Prescalerselect::Div32,
            5 => Prescalerselect::Div64,
            6 => Prescalerselect::Div128,
            7 => Prescalerselect::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "Peripheral clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescalerselect::Div32
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescalerselect::Div128
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescalerselect::Div256
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Configuration"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescalerselect, crate::Safe>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div32)
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div128)
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div256)
    }
}
#[doc = "Field `R2R` reader - Rail to Rail Operation Enable"]
pub type R2rR = crate::BitReader;
#[doc = "Field `R2R` writer - Rail to Rail Operation Enable"]
pub type R2rW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Dual Mode Trigger Selection"]
    #[inline(always)]
    pub fn dualsel(&self) -> DualselR {
        DualselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Slave Enable"]
    #[inline(always)]
    pub fn slaveen(&self) -> SlaveenR {
        SlaveenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Rail to Rail Operation Enable"]
    #[inline(always)]
    pub fn r2r(&self) -> R2rR {
        R2rR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Dual Mode Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dualsel(&mut self) -> DualselW<CtrlaSpec> {
        DualselW::new(self, 3)
    }
    #[doc = "Bit 5 - Slave Enable"]
    #[inline(always)]
    #[must_use]
    pub fn slaveen(&mut self) -> SlaveenW<CtrlaSpec> {
        SlaveenW::new(self, 5)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<CtrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<CtrlaSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<CtrlaSpec> {
        PrescalerW::new(self, 8)
    }
    #[doc = "Bit 15 - Rail to Rail Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn r2r(&mut self) -> R2rW<CtrlaSpec> {
        R2rW::new(self, 15)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u16 = 0;
}
