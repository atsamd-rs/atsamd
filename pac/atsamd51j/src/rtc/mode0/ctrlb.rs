#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `GP0EN` reader - General Purpose 0 Enable"]
pub type Gp0enR = crate::BitReader;
#[doc = "Field `GP0EN` writer - General Purpose 0 Enable"]
pub type Gp0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GP2EN` reader - General Purpose 2 Enable"]
pub type Gp2enR = crate::BitReader;
#[doc = "Field `GP2EN` writer - General Purpose 2 Enable"]
pub type Gp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBMAJ` reader - Debouncer Majority Enable"]
pub type DebmajR = crate::BitReader;
#[doc = "Field `DEBMAJ` writer - Debouncer Majority Enable"]
pub type DebmajW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBASYNC` reader - Debouncer Asynchronous Enable"]
pub type DebasyncR = crate::BitReader;
#[doc = "Field `DEBASYNC` writer - Debouncer Asynchronous Enable"]
pub type DebasyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCOUT` reader - RTC Output Enable"]
pub type RtcoutR = crate::BitReader;
#[doc = "Field `RTCOUT` writer - RTC Output Enable"]
pub type RtcoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Debounce Freqnuency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Debfselect {
    #[doc = "0: CLK_RTC_DEB = CLK_RTC/2"]
    Div2 = 0,
    #[doc = "1: CLK_RTC_DEB = CLK_RTC/4"]
    Div4 = 1,
    #[doc = "2: CLK_RTC_DEB = CLK_RTC/8"]
    Div8 = 2,
    #[doc = "3: CLK_RTC_DEB = CLK_RTC/16"]
    Div16 = 3,
    #[doc = "4: CLK_RTC_DEB = CLK_RTC/32"]
    Div32 = 4,
    #[doc = "5: CLK_RTC_DEB = CLK_RTC/64"]
    Div64 = 5,
    #[doc = "6: CLK_RTC_DEB = CLK_RTC/128"]
    Div128 = 6,
    #[doc = "7: CLK_RTC_DEB = CLK_RTC/256"]
    Div256 = 7,
}
impl From<Debfselect> for u8 {
    #[inline(always)]
    fn from(variant: Debfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Debfselect {
    type Ux = u8;
}
impl crate::IsEnum for Debfselect {}
#[doc = "Field `DEBF` reader - Debounce Freqnuency"]
pub type DebfR = crate::FieldReader<Debfselect>;
impl DebfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debfselect {
        match self.bits {
            0 => Debfselect::Div2,
            1 => Debfselect::Div4,
            2 => Debfselect::Div8,
            3 => Debfselect::Div16,
            4 => Debfselect::Div32,
            5 => Debfselect::Div64,
            6 => Debfselect::Div128,
            7 => Debfselect::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Debfselect::Div2
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Debfselect::Div4
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Debfselect::Div8
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Debfselect::Div16
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Debfselect::Div32
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Debfselect::Div64
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Debfselect::Div128
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Debfselect::Div256
    }
}
#[doc = "Field `DEBF` writer - Debounce Freqnuency"]
pub type DebfW<'a, REG> = crate::FieldWriter<'a, REG, 3, Debfselect, crate::Safe>;
impl<'a, REG> DebfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_RTC_DEB = CLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div2)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div4)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div8)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div16)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div32)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div64)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div128)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Debfselect::Div256)
    }
}
#[doc = "Active Layer Freqnuency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actfselect {
    #[doc = "0: CLK_RTC_OUT = CLK_RTC/2"]
    Div2 = 0,
    #[doc = "1: CLK_RTC_OUT = CLK_RTC/4"]
    Div4 = 1,
    #[doc = "2: CLK_RTC_OUT = CLK_RTC/8"]
    Div8 = 2,
    #[doc = "3: CLK_RTC_OUT = CLK_RTC/16"]
    Div16 = 3,
    #[doc = "4: CLK_RTC_OUT = CLK_RTC/32"]
    Div32 = 4,
    #[doc = "5: CLK_RTC_OUT = CLK_RTC/64"]
    Div64 = 5,
    #[doc = "6: CLK_RTC_OUT = CLK_RTC/128"]
    Div128 = 6,
    #[doc = "7: CLK_RTC_OUT = CLK_RTC/256"]
    Div256 = 7,
}
impl From<Actfselect> for u8 {
    #[inline(always)]
    fn from(variant: Actfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actfselect {
    type Ux = u8;
}
impl crate::IsEnum for Actfselect {}
#[doc = "Field `ACTF` reader - Active Layer Freqnuency"]
pub type ActfR = crate::FieldReader<Actfselect>;
impl ActfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actfselect {
        match self.bits {
            0 => Actfselect::Div2,
            1 => Actfselect::Div4,
            2 => Actfselect::Div8,
            3 => Actfselect::Div16,
            4 => Actfselect::Div32,
            5 => Actfselect::Div64,
            6 => Actfselect::Div128,
            7 => Actfselect::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Actfselect::Div2
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Actfselect::Div4
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Actfselect::Div8
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Actfselect::Div16
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Actfselect::Div32
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Actfselect::Div64
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Actfselect::Div128
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Actfselect::Div256
    }
}
#[doc = "Field `ACTF` writer - Active Layer Freqnuency"]
pub type ActfW<'a, REG> = crate::FieldWriter<'a, REG, 3, Actfselect, crate::Safe>;
impl<'a, REG> ActfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_RTC_OUT = CLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div2)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div4)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div8)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div16)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div32)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div64)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div128)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Actfselect::Div256)
    }
}
impl R {
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline(always)]
    pub fn gp0en(&self) -> Gp0enR {
        Gp0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General Purpose 2 Enable"]
    #[inline(always)]
    pub fn gp2en(&self) -> Gp2enR {
        Gp2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline(always)]
    pub fn debmaj(&self) -> DebmajR {
        DebmajR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline(always)]
    pub fn debasync(&self) -> DebasyncR {
        DebasyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline(always)]
    pub fn rtcout(&self) -> RtcoutR {
        RtcoutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline(always)]
    pub fn debf(&self) -> DebfR {
        DebfR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline(always)]
    pub fn actf(&self) -> ActfR {
        ActfR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gp0en(&mut self) -> Gp0enW<CtrlbSpec> {
        Gp0enW::new(self, 0)
    }
    #[doc = "Bit 1 - General Purpose 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gp2en(&mut self) -> Gp2enW<CtrlbSpec> {
        Gp2enW::new(self, 1)
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debmaj(&mut self) -> DebmajW<CtrlbSpec> {
        DebmajW::new(self, 4)
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debasync(&mut self) -> DebasyncW<CtrlbSpec> {
        DebasyncW::new(self, 5)
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcout(&mut self) -> RtcoutW<CtrlbSpec> {
        RtcoutW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<CtrlbSpec> {
        DmaenW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline(always)]
    #[must_use]
    pub fn debf(&mut self) -> DebfW<CtrlbSpec> {
        DebfW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline(always)]
    #[must_use]
    pub fn actf(&mut self) -> ActfW<CtrlbSpec> {
        ActfW::new(self, 12)
    }
}
#[doc = "MODE0 Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u16 = 0;
}
