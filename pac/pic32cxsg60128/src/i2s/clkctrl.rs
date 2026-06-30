#[doc = "Register `CLKCTRL[%s]` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL[%s]` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Slot Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slotsizeselect {
    #[doc = "0: 8-bit Slot for Clock Unit n"]
    _8 = 0,
    #[doc = "1: 16-bit Slot for Clock Unit n"]
    _16 = 1,
    #[doc = "2: 24-bit Slot for Clock Unit n"]
    _24 = 2,
    #[doc = "3: 32-bit Slot for Clock Unit n"]
    _32 = 3,
}
impl From<Slotsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Slotsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slotsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Slotsizeselect {}
#[doc = "Field `SLOTSIZE` reader - Slot Size"]
pub type SlotsizeR = crate::FieldReader<Slotsizeselect>;
impl SlotsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slotsizeselect {
        match self.bits {
            0 => Slotsizeselect::_8,
            1 => Slotsizeselect::_16,
            2 => Slotsizeselect::_24,
            3 => Slotsizeselect::_32,
            _ => unreachable!(),
        }
    }
    #[doc = "8-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Slotsizeselect::_8
    }
    #[doc = "16-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Slotsizeselect::_16
    }
    #[doc = "24-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == Slotsizeselect::_24
    }
    #[doc = "32-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Slotsizeselect::_32
    }
}
#[doc = "Field `SLOTSIZE` writer - Slot Size"]
pub type SlotsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slotsizeselect, crate::Safe>;
impl<'a, REG> SlotsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Slotsizeselect::_8)
    }
    #[doc = "16-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Slotsizeselect::_16)
    }
    #[doc = "24-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut crate::W<REG> {
        self.variant(Slotsizeselect::_24)
    }
    #[doc = "32-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Slotsizeselect::_32)
    }
}
#[doc = "Field `NBSLOTS` reader - Number of Slots in Frame"]
pub type NbslotsR = crate::FieldReader;
#[doc = "Field `NBSLOTS` writer - Number of Slots in Frame"]
pub type NbslotsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Frame Sync Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fswidthselect {
    #[doc = "0: Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    Slot = 0,
    #[doc = "1: Frame Sync Pulse is half a Frame wide"]
    Half = 1,
    #[doc = "2: Frame Sync Pulse is 1 Bit wide"]
    Bit = 2,
    #[doc = "3: Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    Burst = 3,
}
impl From<Fswidthselect> for u8 {
    #[inline(always)]
    fn from(variant: Fswidthselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fswidthselect {
    type Ux = u8;
}
impl crate::IsEnum for Fswidthselect {}
#[doc = "Field `FSWIDTH` reader - Frame Sync Width"]
pub type FswidthR = crate::FieldReader<Fswidthselect>;
impl FswidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fswidthselect {
        match self.bits {
            0 => Fswidthselect::Slot,
            1 => Fswidthselect::Half,
            2 => Fswidthselect::Bit,
            3 => Fswidthselect::Burst,
            _ => unreachable!(),
        }
    }
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    #[inline(always)]
    pub fn is_slot(&self) -> bool {
        *self == Fswidthselect::Slot
    }
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Fswidthselect::Half
    }
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == Fswidthselect::Bit
    }
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Fswidthselect::Burst
    }
}
#[doc = "Field `FSWIDTH` writer - Frame Sync Width"]
pub type FswidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fswidthselect, crate::Safe>;
impl<'a, REG> FswidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    #[inline(always)]
    pub fn slot(self) -> &'a mut crate::W<REG> {
        self.variant(Fswidthselect::Slot)
    }
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Fswidthselect::Half)
    }
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(Fswidthselect::Bit)
    }
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Fswidthselect::Burst)
    }
}
#[doc = "Data Delay from Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bitdelayselect {
    #[doc = "0: Left Justified (0 Bit Delay)"]
    Lj = 0,
    #[doc = "1: I2S (1 Bit Delay)"]
    I2s = 1,
}
impl From<Bitdelayselect> for bool {
    #[inline(always)]
    fn from(variant: Bitdelayselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITDELAY` reader - Data Delay from Frame Sync"]
pub type BitdelayR = crate::BitReader<Bitdelayselect>;
impl BitdelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bitdelayselect {
        match self.bits {
            false => Bitdelayselect::Lj,
            true => Bitdelayselect::I2s,
        }
    }
    #[doc = "Left Justified (0 Bit Delay)"]
    #[inline(always)]
    pub fn is_lj(&self) -> bool {
        *self == Bitdelayselect::Lj
    }
    #[doc = "I2S (1 Bit Delay)"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == Bitdelayselect::I2s
    }
}
#[doc = "Field `BITDELAY` writer - Data Delay from Frame Sync"]
pub type BitdelayW<'a, REG> = crate::BitWriter<'a, REG, Bitdelayselect>;
impl<'a, REG> BitdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Left Justified (0 Bit Delay)"]
    #[inline(always)]
    pub fn lj(self) -> &'a mut crate::W<REG> {
        self.variant(Bitdelayselect::Lj)
    }
    #[doc = "I2S (1 Bit Delay)"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(Bitdelayselect::I2s)
    }
}
#[doc = "Frame Sync Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsselselect {
    #[doc = "0: Divided Serial Clock n is used as Frame Sync n source"]
    Sckdiv = 0,
    #[doc = "1: FSn input pin is used as Frame Sync n source"]
    Fspin = 1,
}
impl From<Fsselselect> for bool {
    #[inline(always)]
    fn from(variant: Fsselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSSEL` reader - Frame Sync Select"]
pub type FsselR = crate::BitReader<Fsselselect>;
impl FsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsselselect {
        match self.bits {
            false => Fsselselect::Sckdiv,
            true => Fsselselect::Fspin,
        }
    }
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    #[inline(always)]
    pub fn is_sckdiv(&self) -> bool {
        *self == Fsselselect::Sckdiv
    }
    #[doc = "FSn input pin is used as Frame Sync n source"]
    #[inline(always)]
    pub fn is_fspin(&self) -> bool {
        *self == Fsselselect::Fspin
    }
}
#[doc = "Field `FSSEL` writer - Frame Sync Select"]
pub type FsselW<'a, REG> = crate::BitWriter<'a, REG, Fsselselect>;
impl<'a, REG> FsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    #[inline(always)]
    pub fn sckdiv(self) -> &'a mut crate::W<REG> {
        self.variant(Fsselselect::Sckdiv)
    }
    #[doc = "FSn input pin is used as Frame Sync n source"]
    #[inline(always)]
    pub fn fspin(self) -> &'a mut crate::W<REG> {
        self.variant(Fsselselect::Fspin)
    }
}
#[doc = "Field `FSINV` reader - Frame Sync Invert"]
pub type FsinvR = crate::BitReader;
#[doc = "Field `FSINV` writer - Frame Sync Invert"]
pub type FsinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSOUTINV` reader - Frame Sync Output Invert"]
pub type FsoutinvR = crate::BitReader;
#[doc = "Field `FSOUTINV` writer - Frame Sync Output Invert"]
pub type FsoutinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Serial Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sckselselect {
    #[doc = "0: Divided Master Clock n is used as Serial Clock n source"]
    Mckdiv = 0,
    #[doc = "1: SCKn input pin is used as Serial Clock n source"]
    Sckpin = 1,
}
impl From<Sckselselect> for bool {
    #[inline(always)]
    fn from(variant: Sckselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKSEL` reader - Serial Clock Select"]
pub type SckselR = crate::BitReader<Sckselselect>;
impl SckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sckselselect {
        match self.bits {
            false => Sckselselect::Mckdiv,
            true => Sckselselect::Sckpin,
        }
    }
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    #[inline(always)]
    pub fn is_mckdiv(&self) -> bool {
        *self == Sckselselect::Mckdiv
    }
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    #[inline(always)]
    pub fn is_sckpin(&self) -> bool {
        *self == Sckselselect::Sckpin
    }
}
#[doc = "Field `SCKSEL` writer - Serial Clock Select"]
pub type SckselW<'a, REG> = crate::BitWriter<'a, REG, Sckselselect>;
impl<'a, REG> SckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    #[inline(always)]
    pub fn mckdiv(self) -> &'a mut crate::W<REG> {
        self.variant(Sckselselect::Mckdiv)
    }
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    #[inline(always)]
    pub fn sckpin(self) -> &'a mut crate::W<REG> {
        self.variant(Sckselselect::Sckpin)
    }
}
#[doc = "Field `SCKOUTINV` reader - Serial Clock Output Invert"]
pub type SckoutinvR = crate::BitReader;
#[doc = "Field `SCKOUTINV` writer - Serial Clock Output Invert"]
pub type SckoutinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Master Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mckselselect {
    #[doc = "0: GCLK_I2S_n is used as Master Clock n source"]
    Gclk = 0,
    #[doc = "1: MCKn input pin is used as Master Clock n source"]
    Mckpin = 1,
}
impl From<Mckselselect> for bool {
    #[inline(always)]
    fn from(variant: Mckselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKSEL` reader - Master Clock Select"]
pub type MckselR = crate::BitReader<Mckselselect>;
impl MckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mckselselect {
        match self.bits {
            false => Mckselselect::Gclk,
            true => Mckselselect::Mckpin,
        }
    }
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Mckselselect::Gclk
    }
    #[doc = "MCKn input pin is used as Master Clock n source"]
    #[inline(always)]
    pub fn is_mckpin(&self) -> bool {
        *self == Mckselselect::Mckpin
    }
}
#[doc = "Field `MCKSEL` writer - Master Clock Select"]
pub type MckselW<'a, REG> = crate::BitWriter<'a, REG, Mckselselect>;
impl<'a, REG> MckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mckselselect::Gclk)
    }
    #[doc = "MCKn input pin is used as Master Clock n source"]
    #[inline(always)]
    pub fn mckpin(self) -> &'a mut crate::W<REG> {
        self.variant(Mckselselect::Mckpin)
    }
}
#[doc = "Field `MCKEN` reader - Master Clock Enable"]
pub type MckenR = crate::BitReader;
#[doc = "Field `MCKEN` writer - Master Clock Enable"]
pub type MckenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOUTINV` reader - Master Clock Output Invert"]
pub type MckoutinvR = crate::BitReader;
#[doc = "Field `MCKOUTINV` writer - Master Clock Output Invert"]
pub type MckoutinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKDIV` reader - Master Clock Division Factor"]
pub type MckdivR = crate::FieldReader;
#[doc = "Field `MCKDIV` writer - Master Clock Division Factor"]
pub type MckdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MCKOUTDIV` reader - Master Clock Output Division Factor"]
pub type MckoutdivR = crate::FieldReader;
#[doc = "Field `MCKOUTDIV` writer - Master Clock Output Division Factor"]
pub type MckoutdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    pub fn slotsize(&self) -> SlotsizeR {
        SlotsizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    pub fn nbslots(&self) -> NbslotsR {
        NbslotsR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    pub fn fswidth(&self) -> FswidthR {
        FswidthR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    pub fn bitdelay(&self) -> BitdelayR {
        BitdelayR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    pub fn fssel(&self) -> FsselR {
        FsselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Sync Invert"]
    #[inline(always)]
    pub fn fsinv(&self) -> FsinvR {
        FsinvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Frame Sync Output Invert"]
    #[inline(always)]
    pub fn fsoutinv(&self) -> FsoutinvR {
        FsoutinvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Serial Clock Select"]
    #[inline(always)]
    pub fn scksel(&self) -> SckselR {
        SckselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Serial Clock Output Invert"]
    #[inline(always)]
    pub fn sckoutinv(&self) -> SckoutinvR {
        SckoutinvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Clock Select"]
    #[inline(always)]
    pub fn mcksel(&self) -> MckselR {
        MckselR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Master Clock Enable"]
    #[inline(always)]
    pub fn mcken(&self) -> MckenR {
        MckenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Master Clock Output Invert"]
    #[inline(always)]
    pub fn mckoutinv(&self) -> MckoutinvR {
        MckoutinvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Master Clock Division Factor"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MckdivR {
        MckdivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Master Clock Output Division Factor"]
    #[inline(always)]
    pub fn mckoutdiv(&self) -> MckoutdivR {
        MckoutdivR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    #[must_use]
    pub fn slotsize(&mut self) -> SlotsizeW<ClkctrlSpec> {
        SlotsizeW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    #[must_use]
    pub fn nbslots(&mut self) -> NbslotsW<ClkctrlSpec> {
        NbslotsW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    #[must_use]
    pub fn fswidth(&mut self) -> FswidthW<ClkctrlSpec> {
        FswidthW::new(self, 5)
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    #[must_use]
    pub fn bitdelay(&mut self) -> BitdelayW<ClkctrlSpec> {
        BitdelayW::new(self, 7)
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    #[must_use]
    pub fn fssel(&mut self) -> FsselW<ClkctrlSpec> {
        FsselW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Sync Invert"]
    #[inline(always)]
    #[must_use]
    pub fn fsinv(&mut self) -> FsinvW<ClkctrlSpec> {
        FsinvW::new(self, 9)
    }
    #[doc = "Bit 10 - Frame Sync Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn fsoutinv(&mut self) -> FsoutinvW<ClkctrlSpec> {
        FsoutinvW::new(self, 10)
    }
    #[doc = "Bit 11 - Serial Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn scksel(&mut self) -> SckselW<ClkctrlSpec> {
        SckselW::new(self, 11)
    }
    #[doc = "Bit 12 - Serial Clock Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn sckoutinv(&mut self) -> SckoutinvW<ClkctrlSpec> {
        SckoutinvW::new(self, 12)
    }
    #[doc = "Bit 13 - Master Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcksel(&mut self) -> MckselW<ClkctrlSpec> {
        MckselW::new(self, 13)
    }
    #[doc = "Bit 14 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MckenW<ClkctrlSpec> {
        MckenW::new(self, 14)
    }
    #[doc = "Bit 15 - Master Clock Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn mckoutinv(&mut self) -> MckoutinvW<ClkctrlSpec> {
        MckoutinvW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Master Clock Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MckdivW<ClkctrlSpec> {
        MckdivW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Master Clock Output Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mckoutdiv(&mut self) -> MckoutdivW<ClkctrlSpec> {
        MckoutdivW::new(self, 24)
    }
}
#[doc = "Clock Unit n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL[%s]
to value 0"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
