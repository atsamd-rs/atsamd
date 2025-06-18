#[doc = "Register `XOSCCTRL[%s]` reader"]
pub type R = crate::R<XoscctrlSpec>;
#[doc = "Register `XOSCCTRL[%s]` writer"]
pub type W = crate::W<XoscctrlSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XtalenR = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWBUFGAIN` reader - Low Buffer Gain Enable"]
pub type LowbufgainR = crate::BitReader;
#[doc = "Field `LOWBUFGAIN` writer - Low Buffer Gain Enable"]
pub type LowbufgainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPTAT` reader - Oscillator Current Reference"]
pub type IptatR = crate::FieldReader;
#[doc = "Field `IPTAT` writer - Oscillator Current Reference"]
pub type IptatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IMULT` reader - Oscillator Current Multiplier"]
pub type ImultR = crate::FieldReader;
#[doc = "Field `IMULT` writer - Oscillator Current Multiplier"]
pub type ImultW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENALC` reader - Automatic Loop Control Enable"]
pub type EnalcR = crate::BitReader;
#[doc = "Field `ENALC` writer - Automatic Loop Control Enable"]
pub type EnalcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CfdenR = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWBEN` reader - Xosc Clock Switch Enable"]
pub type SwbenR = crate::BitReader;
#[doc = "Field `SWBEN` writer - Xosc Clock Switch Enable"]
pub type SwbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startupselect {
    #[doc = "0: 31 us"]
    Cycle1 = 0,
    #[doc = "1: 61 us"]
    Cycle2 = 1,
    #[doc = "2: 122 us"]
    Cycle4 = 2,
    #[doc = "3: 244 us"]
    Cycle8 = 3,
    #[doc = "4: 488 us"]
    Cycle16 = 4,
    #[doc = "5: 977 us"]
    Cycle32 = 5,
    #[doc = "6: 1953 us"]
    Cycle64 = 6,
    #[doc = "7: 3906 us"]
    Cycle128 = 7,
    #[doc = "8: 7813 us"]
    Cycle256 = 8,
    #[doc = "9: 15625 us"]
    Cycle512 = 9,
    #[doc = "10: 31250 us"]
    Cycle1024 = 10,
    #[doc = "11: 62500 us"]
    Cycle2048 = 11,
    #[doc = "12: 125000 us"]
    Cycle4096 = 12,
    #[doc = "13: 250000 us"]
    Cycle8192 = 13,
    #[doc = "14: 500000 us"]
    Cycle16384 = 14,
    #[doc = "15: 1000000 us"]
    Cycle32768 = 15,
}
impl From<Startupselect> for u8 {
    #[inline(always)]
    fn from(variant: Startupselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startupselect {
    type Ux = u8;
}
impl crate::IsEnum for Startupselect {}
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type StartupR = crate::FieldReader<Startupselect>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startupselect {
        match self.bits {
            0 => Startupselect::Cycle1,
            1 => Startupselect::Cycle2,
            2 => Startupselect::Cycle4,
            3 => Startupselect::Cycle8,
            4 => Startupselect::Cycle16,
            5 => Startupselect::Cycle32,
            6 => Startupselect::Cycle64,
            7 => Startupselect::Cycle128,
            8 => Startupselect::Cycle256,
            9 => Startupselect::Cycle512,
            10 => Startupselect::Cycle1024,
            11 => Startupselect::Cycle2048,
            12 => Startupselect::Cycle4096,
            13 => Startupselect::Cycle8192,
            14 => Startupselect::Cycle16384,
            15 => Startupselect::Cycle32768,
            _ => unreachable!(),
        }
    }
    #[doc = "31 us"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == Startupselect::Cycle1
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn is_cycle2(&self) -> bool {
        *self == Startupselect::Cycle2
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == Startupselect::Cycle4
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        *self == Startupselect::Cycle8
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        *self == Startupselect::Cycle16
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == Startupselect::Cycle32
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        *self == Startupselect::Cycle64
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        *self == Startupselect::Cycle128
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        *self == Startupselect::Cycle256
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        *self == Startupselect::Cycle512
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        *self == Startupselect::Cycle1024
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == Startupselect::Cycle2048
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == Startupselect::Cycle4096
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn is_cycle8192(&self) -> bool {
        *self == Startupselect::Cycle8192
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == Startupselect::Cycle16384
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == Startupselect::Cycle32768
    }
}
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startupselect, crate::Safe>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "31 us"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle1)
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn cycle2(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle2)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle4)
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle8)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle16)
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle32)
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle64)
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle128)
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle256)
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle512)
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle1024)
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle2048)
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle4096)
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn cycle8192(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle8192)
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle16384)
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle32768)
    }
}
#[doc = "Clock Failure Detector Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfdprescselect {
    #[doc = "0: 48 MHz"]
    Div1 = 0,
    #[doc = "1: 24 MHz"]
    Div2 = 1,
    #[doc = "2: 12 MHz"]
    Div4 = 2,
    #[doc = "3: 6 MHz"]
    Div8 = 3,
    #[doc = "4: 3 MHz"]
    Div16 = 4,
    #[doc = "5: 1.5 MHz"]
    Div32 = 5,
    #[doc = "6: 0.75 MHz"]
    Div64 = 6,
    #[doc = "7: 0.3125 MHz"]
    Div128 = 7,
}
impl From<Cfdprescselect> for u8 {
    #[inline(always)]
    fn from(variant: Cfdprescselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfdprescselect {
    type Ux = u8;
}
impl crate::IsEnum for Cfdprescselect {}
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub type CfdprescR = crate::FieldReader<Cfdprescselect>;
impl CfdprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfdprescselect> {
        match self.bits {
            0 => Some(Cfdprescselect::Div1),
            1 => Some(Cfdprescselect::Div2),
            2 => Some(Cfdprescselect::Div4),
            3 => Some(Cfdprescselect::Div8),
            4 => Some(Cfdprescselect::Div16),
            5 => Some(Cfdprescselect::Div32),
            6 => Some(Cfdprescselect::Div64),
            7 => Some(Cfdprescselect::Div128),
            _ => None,
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cfdprescselect::Div1
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cfdprescselect::Div2
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cfdprescselect::Div4
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cfdprescselect::Div8
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cfdprescselect::Div16
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Cfdprescselect::Div32
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Cfdprescselect::Div64
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Cfdprescselect::Div128
    }
}
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub type CfdprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cfdprescselect>;
impl<'a, REG> CfdprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div2)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div4)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div8)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div16)
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div32)
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div64)
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div128)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XtalenR {
        XtalenR::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline(always)]
    pub fn lowbufgain(&self) -> LowbufgainR {
        LowbufgainR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline(always)]
    pub fn iptat(&self) -> IptatR {
        IptatR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline(always)]
    pub fn imult(&self) -> ImultR {
        ImultR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline(always)]
    pub fn enalc(&self) -> EnalcR {
        EnalcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CfdenR {
        CfdenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&self) -> SwbenR {
        SwbenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CfdprescR {
        CfdprescR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<XoscctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XtalenW<XoscctrlSpec> {
        XtalenW::new(self, 2)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<XoscctrlSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<XoscctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline(always)]
    pub fn lowbufgain(&mut self) -> LowbufgainW<XoscctrlSpec> {
        LowbufgainW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline(always)]
    pub fn iptat(&mut self) -> IptatW<XoscctrlSpec> {
        IptatW::new(self, 9)
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline(always)]
    pub fn imult(&mut self) -> ImultW<XoscctrlSpec> {
        ImultW::new(self, 11)
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline(always)]
    pub fn enalc(&mut self) -> EnalcW<XoscctrlSpec> {
        EnalcW::new(self, 15)
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CfdenW<XoscctrlSpec> {
        CfdenW::new(self, 16)
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&mut self) -> SwbenW<XoscctrlSpec> {
        SwbenW::new(self, 17)
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> StartupW<XoscctrlSpec> {
        StartupW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&mut self) -> CfdprescW<XoscctrlSpec> {
        CfdprescW::new(self, 24)
    }
}
#[doc = "External Multipurpose Crystal Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xoscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xoscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XoscctrlSpec;
impl crate::RegisterSpec for XoscctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xoscctrl::R`](R) reader structure"]
impl crate::Readable for XoscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xoscctrl::W`](W) writer structure"]
impl crate::Writable for XoscctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSCCTRL[%s] to value 0x80"]
impl crate::Resettable for XoscctrlSpec {
    const RESET_VALUE: u32 = 0x80;
}
