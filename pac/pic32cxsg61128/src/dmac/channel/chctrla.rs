#[doc = "Register `CHCTRLA` reader"]
pub type R = crate::R<ChctrlaSpec>;
#[doc = "Register `CHCTRLA` writer"]
pub type W = crate::W<ChctrlaSpec>;
#[doc = "Field `SWRST` reader - Channel Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Channel Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Channel Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Channel Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Channel Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Channel Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrcselect {
    #[doc = "0: Only software/event triggers"]
    Disable = 0,
}
impl From<Trigsrcselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigsrcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsrcselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigsrcselect {}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub type TrigsrcR = crate::FieldReader<Trigsrcselect>;
impl TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsrcselect> {
        match self.bits {
            0 => Some(Trigsrcselect::Disable),
            _ => None,
        }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trigsrcselect::Disable
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 7, Trigsrcselect>;
impl<'a, REG> TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcselect::Disable)
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigactselect {
    #[doc = "0: One trigger required for each block transfer"]
    Block = 0,
    #[doc = "2: One trigger required for each burst transfer"]
    Burst = 2,
    #[doc = "3: One trigger required for each transaction"]
    Transaction = 3,
}
impl From<Trigactselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigactselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigactselect {}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub type TrigactR = crate::FieldReader<Trigactselect>;
impl TrigactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigactselect> {
        match self.bits {
            0 => Some(Trigactselect::Block),
            2 => Some(Trigactselect::Burst),
            3 => Some(Trigactselect::Transaction),
            _ => None,
        }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Trigactselect::Block
    }
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Trigactselect::Burst
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == Trigactselect::Transaction
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub type TrigactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigactselect>;
impl<'a, REG> TrigactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Block)
    }
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Burst)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(Trigactselect::Transaction)
    }
}
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burstlenselect {
    #[doc = "0: Single-beat burst length"]
    Single = 0,
    #[doc = "1: 2-beats burst length"]
    _2beat = 1,
    #[doc = "2: 3-beats burst length"]
    _3beat = 2,
    #[doc = "3: 4-beats burst length"]
    _4beat = 3,
    #[doc = "4: 5-beats burst length"]
    _5beat = 4,
    #[doc = "5: 6-beats burst length"]
    _6beat = 5,
    #[doc = "6: 7-beats burst length"]
    _7beat = 6,
    #[doc = "7: 8-beats burst length"]
    _8beat = 7,
    #[doc = "8: 9-beats burst length"]
    _9beat = 8,
    #[doc = "9: 10-beats burst length"]
    _10beat = 9,
    #[doc = "10: 11-beats burst length"]
    _11beat = 10,
    #[doc = "11: 12-beats burst length"]
    _12beat = 11,
    #[doc = "12: 13-beats burst length"]
    _13beat = 12,
    #[doc = "13: 14-beats burst length"]
    _14beat = 13,
    #[doc = "14: 15-beats burst length"]
    _15beat = 14,
    #[doc = "15: 16-beats burst length"]
    _16beat = 15,
}
impl From<Burstlenselect> for u8 {
    #[inline(always)]
    fn from(variant: Burstlenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burstlenselect {
    type Ux = u8;
}
impl crate::IsEnum for Burstlenselect {}
#[doc = "Field `BURSTLEN` reader - Burst Length"]
pub type BurstlenR = crate::FieldReader<Burstlenselect>;
impl BurstlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burstlenselect {
        match self.bits {
            0 => Burstlenselect::Single,
            1 => Burstlenselect::_2beat,
            2 => Burstlenselect::_3beat,
            3 => Burstlenselect::_4beat,
            4 => Burstlenselect::_5beat,
            5 => Burstlenselect::_6beat,
            6 => Burstlenselect::_7beat,
            7 => Burstlenselect::_8beat,
            8 => Burstlenselect::_9beat,
            9 => Burstlenselect::_10beat,
            10 => Burstlenselect::_11beat,
            11 => Burstlenselect::_12beat,
            12 => Burstlenselect::_13beat,
            13 => Burstlenselect::_14beat,
            14 => Burstlenselect::_15beat,
            15 => Burstlenselect::_16beat,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Burstlenselect::Single
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn is_2beat(&self) -> bool {
        *self == Burstlenselect::_2beat
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn is_3beat(&self) -> bool {
        *self == Burstlenselect::_3beat
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn is_4beat(&self) -> bool {
        *self == Burstlenselect::_4beat
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn is_5beat(&self) -> bool {
        *self == Burstlenselect::_5beat
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn is_6beat(&self) -> bool {
        *self == Burstlenselect::_6beat
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn is_7beat(&self) -> bool {
        *self == Burstlenselect::_7beat
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn is_8beat(&self) -> bool {
        *self == Burstlenselect::_8beat
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn is_9beat(&self) -> bool {
        *self == Burstlenselect::_9beat
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn is_10beat(&self) -> bool {
        *self == Burstlenselect::_10beat
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn is_11beat(&self) -> bool {
        *self == Burstlenselect::_11beat
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn is_12beat(&self) -> bool {
        *self == Burstlenselect::_12beat
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn is_13beat(&self) -> bool {
        *self == Burstlenselect::_13beat
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn is_14beat(&self) -> bool {
        *self == Burstlenselect::_14beat
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn is_15beat(&self) -> bool {
        *self == Burstlenselect::_15beat
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn is_16beat(&self) -> bool {
        *self == Burstlenselect::_16beat
    }
}
#[doc = "Field `BURSTLEN` writer - Burst Length"]
pub type BurstlenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Burstlenselect, crate::Safe>;
impl<'a, REG> BurstlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::Single)
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn _2beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_2beat)
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn _3beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_3beat)
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn _4beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_4beat)
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn _5beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_5beat)
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn _6beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_6beat)
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn _7beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_7beat)
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn _8beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_8beat)
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn _9beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_9beat)
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn _10beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_10beat)
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn _11beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_11beat)
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn _12beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_12beat)
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn _13beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_13beat)
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn _14beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_14beat)
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn _15beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_15beat)
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn _16beat(self) -> &'a mut crate::W<REG> {
        self.variant(Burstlenselect::_16beat)
    }
}
#[doc = "FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Thresholdselect {
    #[doc = "0: Destination write starts after each beat source address read"]
    _1beat = 0,
    #[doc = "1: Destination write starts after 2-beats source address read"]
    _2beats = 1,
    #[doc = "2: Destination write starts after 4-beats source address read"]
    _4beats = 2,
    #[doc = "3: Destination write starts after 8-beats source address read"]
    _8beats = 3,
}
impl From<Thresholdselect> for u8 {
    #[inline(always)]
    fn from(variant: Thresholdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Thresholdselect {
    type Ux = u8;
}
impl crate::IsEnum for Thresholdselect {}
#[doc = "Field `THRESHOLD` reader - FIFO Threshold"]
pub type ThresholdR = crate::FieldReader<Thresholdselect>;
impl ThresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thresholdselect {
        match self.bits {
            0 => Thresholdselect::_1beat,
            1 => Thresholdselect::_2beats,
            2 => Thresholdselect::_4beats,
            3 => Thresholdselect::_8beats,
            _ => unreachable!(),
        }
    }
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn is_1beat(&self) -> bool {
        *self == Thresholdselect::_1beat
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn is_2beats(&self) -> bool {
        *self == Thresholdselect::_2beats
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn is_4beats(&self) -> bool {
        *self == Thresholdselect::_4beats
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn is_8beats(&self) -> bool {
        *self == Thresholdselect::_8beats
    }
}
#[doc = "Field `THRESHOLD` writer - FIFO Threshold"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Thresholdselect, crate::Safe>;
impl<'a, REG> ThresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn _1beat(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_1beat)
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn _2beats(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_2beats)
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn _4beats(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_4beats)
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn _8beats(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdselect::_8beats)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TrigactR {
        TrigactR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&self) -> BurstlenR {
        BurstlenR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<ChctrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ChctrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<ChctrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrc(&mut self) -> TrigsrcW<ChctrlaSpec> {
        TrigsrcW::new(self, 8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    #[must_use]
    pub fn trigact(&mut self) -> TrigactW<ChctrlaSpec> {
        TrigactW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn burstlen(&mut self) -> BurstlenW<ChctrlaSpec> {
        BurstlenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> ThresholdW<ChctrlaSpec> {
        ThresholdW::new(self, 28)
    }
}
#[doc = "Channel n Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlaSpec;
impl crate::RegisterSpec for ChctrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrla::R`](R) reader structure"]
impl crate::Readable for ChctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrla::W`](W) writer structure"]
impl crate::Writable for ChctrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTRLA to value 0"]
impl crate::Resettable for ChctrlaSpec {
    const RESET_VALUE: u32 = 0;
}
