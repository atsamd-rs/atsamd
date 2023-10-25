#[doc = "Register `XOSCCTRL[%s]` reader"]
pub type R = crate::R<XOSCCTRL_SPEC>;
#[doc = "Register `XOSCCTRL[%s]` writer"]
pub type W = crate::W<XOSCCTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOWBUFGAIN` reader - Low Buffer Gain Enable"]
pub type LOWBUFGAIN_R = crate::BitReader;
#[doc = "Field `LOWBUFGAIN` writer - Low Buffer Gain Enable"]
pub type LOWBUFGAIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IPTAT` reader - Oscillator Current Reference"]
pub type IPTAT_R = crate::FieldReader;
#[doc = "Field `IPTAT` writer - Oscillator Current Reference"]
pub type IPTAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IMULT` reader - Oscillator Current Multiplier"]
pub type IMULT_R = crate::FieldReader;
#[doc = "Field `IMULT` writer - Oscillator Current Multiplier"]
pub type IMULT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ENALC` reader - Automatic Loop Control Enable"]
pub type ENALC_R = crate::BitReader;
#[doc = "Field `ENALC` writer - Automatic Loop Control Enable"]
pub type ENALC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CFDEN_R = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CFDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWBEN` reader - Xosc Clock Switch Enable"]
pub type SWBEN_R = crate::BitReader;
#[doc = "Field `SWBEN` writer - Xosc Clock Switch Enable"]
pub type SWBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<STARTUPSELECT_A>;
#[doc = "Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUPSELECT_A {
    #[doc = "0: 31 us"]
    CYCLE1 = 0,
    #[doc = "1: 61 us"]
    CYCLE2 = 1,
    #[doc = "2: 122 us"]
    CYCLE4 = 2,
    #[doc = "3: 244 us"]
    CYCLE8 = 3,
    #[doc = "4: 488 us"]
    CYCLE16 = 4,
    #[doc = "5: 977 us"]
    CYCLE32 = 5,
    #[doc = "6: 1953 us"]
    CYCLE64 = 6,
    #[doc = "7: 3906 us"]
    CYCLE128 = 7,
    #[doc = "8: 7813 us"]
    CYCLE256 = 8,
    #[doc = "9: 15625 us"]
    CYCLE512 = 9,
    #[doc = "10: 31250 us"]
    CYCLE1024 = 10,
    #[doc = "11: 62500 us"]
    CYCLE2048 = 11,
    #[doc = "12: 125000 us"]
    CYCLE4096 = 12,
    #[doc = "13: 250000 us"]
    CYCLE8192 = 13,
    #[doc = "14: 500000 us"]
    CYCLE16384 = 14,
    #[doc = "15: 1000000 us"]
    CYCLE32768 = 15,
}
impl From<STARTUPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STARTUPSELECT_A {
    type Ux = u8;
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STARTUPSELECT_A {
        match self.bits {
            0 => STARTUPSELECT_A::CYCLE1,
            1 => STARTUPSELECT_A::CYCLE2,
            2 => STARTUPSELECT_A::CYCLE4,
            3 => STARTUPSELECT_A::CYCLE8,
            4 => STARTUPSELECT_A::CYCLE16,
            5 => STARTUPSELECT_A::CYCLE32,
            6 => STARTUPSELECT_A::CYCLE64,
            7 => STARTUPSELECT_A::CYCLE128,
            8 => STARTUPSELECT_A::CYCLE256,
            9 => STARTUPSELECT_A::CYCLE512,
            10 => STARTUPSELECT_A::CYCLE1024,
            11 => STARTUPSELECT_A::CYCLE2048,
            12 => STARTUPSELECT_A::CYCLE4096,
            13 => STARTUPSELECT_A::CYCLE8192,
            14 => STARTUPSELECT_A::CYCLE16384,
            15 => STARTUPSELECT_A::CYCLE32768,
            _ => unreachable!(),
        }
    }
    #[doc = "31 us"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE1
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn is_cycle2(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE2
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE4
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE8
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE16
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE32
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE64
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE128
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE256
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE512
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE1024
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE2048
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE4096
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn is_cycle8192(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE8192
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE16384
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE32768
    }
}
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type STARTUP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, STARTUPSELECT_A>;
impl<'a, REG, const O: u8> STARTUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "31 us"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE1)
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn cycle2(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE2)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE4)
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE8)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE16)
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE32)
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE64)
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE128)
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE256)
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE512)
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE1024)
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE2048)
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE4096)
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn cycle8192(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE8192)
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE16384)
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE32768)
    }
}
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub type CFDPRESC_R = crate::FieldReader<CFDPRESCSELECT_A>;
#[doc = "Clock Failure Detector Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDPRESCSELECT_A {
    #[doc = "0: 48 MHz"]
    DIV1 = 0,
    #[doc = "1: 24 MHz"]
    DIV2 = 1,
    #[doc = "2: 12 MHz"]
    DIV4 = 2,
    #[doc = "3: 6 MHz"]
    DIV8 = 3,
    #[doc = "4: 3 MHz"]
    DIV16 = 4,
    #[doc = "5: 1.5 MHz"]
    DIV32 = 5,
    #[doc = "6: 0.75 MHz"]
    DIV64 = 6,
    #[doc = "7: 0.3125 MHz"]
    DIV128 = 7,
}
impl From<CFDPRESCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDPRESCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFDPRESCSELECT_A {
    type Ux = u8;
}
impl CFDPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CFDPRESCSELECT_A> {
        match self.bits {
            0 => Some(CFDPRESCSELECT_A::DIV1),
            1 => Some(CFDPRESCSELECT_A::DIV2),
            2 => Some(CFDPRESCSELECT_A::DIV4),
            3 => Some(CFDPRESCSELECT_A::DIV8),
            4 => Some(CFDPRESCSELECT_A::DIV16),
            5 => Some(CFDPRESCSELECT_A::DIV32),
            6 => Some(CFDPRESCSELECT_A::DIV64),
            7 => Some(CFDPRESCSELECT_A::DIV128),
            _ => None,
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV1
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV2
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV4
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV8
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV16
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV32
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV64
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CFDPRESCSELECT_A::DIV128
    }
}
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub type CFDPRESC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CFDPRESCSELECT_A>;
impl<'a, REG, const O: u8> CFDPRESC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV2)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV4)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV8)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV16)
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV32)
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV64)
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(CFDPRESCSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline(always)]
    pub fn lowbufgain(&self) -> LOWBUFGAIN_R {
        LOWBUFGAIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline(always)]
    pub fn iptat(&self) -> IPTAT_R {
        IPTAT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline(always)]
    pub fn imult(&self) -> IMULT_R {
        IMULT_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline(always)]
    pub fn enalc(&self) -> ENALC_R {
        ENALC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&self) -> SWBEN_R {
        SWBEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<XOSCCTRL_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XTALEN_W<XOSCCTRL_SPEC, 2> {
        XTALEN_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<XOSCCTRL_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<XOSCCTRL_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lowbufgain(&mut self) -> LOWBUFGAIN_W<XOSCCTRL_SPEC, 8> {
        LOWBUFGAIN_W::new(self)
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline(always)]
    #[must_use]
    pub fn iptat(&mut self) -> IPTAT_W<XOSCCTRL_SPEC, 9> {
        IPTAT_W::new(self)
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn imult(&mut self) -> IMULT_W<XOSCCTRL_SPEC, 11> {
        IMULT_W::new(self)
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enalc(&mut self) -> ENALC_W<XOSCCTRL_SPEC, 15> {
        ENALC_W::new(self)
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<XOSCCTRL_SPEC, 16> {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swben(&mut self) -> SWBEN_W<XOSCCTRL_SPEC, 17> {
        SWBEN_W::new(self)
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<XOSCCTRL_SPEC, 20> {
        STARTUP_W::new(self)
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W<XOSCCTRL_SPEC, 24> {
        CFDPRESC_W::new(self)
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
#[doc = "External Multipurpose Crystal Oscillator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xoscctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xoscctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOSCCTRL_SPEC;
impl crate::RegisterSpec for XOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xoscctrl::R`](R) reader structure"]
impl crate::Readable for XOSCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xoscctrl::W`](W) writer structure"]
impl crate::Writable for XOSCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSCCTRL[%s]
to value 0x80"]
impl crate::Resettable for XOSCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
