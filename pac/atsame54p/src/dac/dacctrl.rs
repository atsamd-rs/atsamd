#[doc = "Register `DACCTRL[%s]` reader"]
pub type R = crate::R<DACCTRL_SPEC>;
#[doc = "Register `DACCTRL[%s]` writer"]
pub type W = crate::W<DACCTRL_SPEC>;
#[doc = "Field `LEFTADJ` reader - Left Adjusted Data"]
pub type LEFTADJ_R = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left Adjusted Data"]
pub type LEFTADJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable DAC0"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable DAC0"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCTRL` reader - Current Control"]
pub type CCTRL_R = crate::FieldReader<CCTRLSELECT_A>;
#[doc = "Current Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCTRLSELECT_A {
    #[doc = "0: 100kSPS"]
    CC100K = 0,
    #[doc = "1: 500kSPS"]
    CC1M = 1,
    #[doc = "2: 1MSPS"]
    CC12M = 2,
}
impl From<CCTRLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CCTRLSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCTRLSELECT_A {
    type Ux = u8;
}
impl CCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCTRLSELECT_A> {
        match self.bits {
            0 => Some(CCTRLSELECT_A::CC100K),
            1 => Some(CCTRLSELECT_A::CC1M),
            2 => Some(CCTRLSELECT_A::CC12M),
            _ => None,
        }
    }
    #[doc = "100kSPS"]
    #[inline(always)]
    pub fn is_cc100k(&self) -> bool {
        *self == CCTRLSELECT_A::CC100K
    }
    #[doc = "500kSPS"]
    #[inline(always)]
    pub fn is_cc1m(&self) -> bool {
        *self == CCTRLSELECT_A::CC1M
    }
    #[doc = "1MSPS"]
    #[inline(always)]
    pub fn is_cc12m(&self) -> bool {
        *self == CCTRLSELECT_A::CC12M
    }
}
#[doc = "Field `CCTRL` writer - Current Control"]
pub type CCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CCTRLSELECT_A>;
impl<'a, REG, const O: u8> CCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "100kSPS"]
    #[inline(always)]
    pub fn cc100k(self) -> &'a mut crate::W<REG> {
        self.variant(CCTRLSELECT_A::CC100K)
    }
    #[doc = "500kSPS"]
    #[inline(always)]
    pub fn cc1m(self) -> &'a mut crate::W<REG> {
        self.variant(CCTRLSELECT_A::CC1M)
    }
    #[doc = "1MSPS"]
    #[inline(always)]
    pub fn cc12m(self) -> &'a mut crate::W<REG> {
        self.variant(CCTRLSELECT_A::CC12M)
    }
}
#[doc = "Field `FEXT` reader - Standalone Filter"]
pub type FEXT_R = crate::BitReader;
#[doc = "Field `FEXT` writer - Standalone Filter"]
pub type FEXT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DITHER` reader - Dithering Mode"]
pub type DITHER_R = crate::BitReader;
#[doc = "Field `DITHER` writer - Dithering Mode"]
pub type DITHER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFRESH` reader - Refresh period"]
pub type REFRESH_R = crate::FieldReader<REFRESHSELECT_A>;
#[doc = "Refresh period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFRESHSELECT_A {
    #[doc = "0: Do not Refresh"]
    REFRESH_0 = 0,
    #[doc = "1: Refresh every 30 us"]
    REFRESH_1 = 1,
    #[doc = "2: Refresh every 60 us"]
    REFRESH_2 = 2,
    #[doc = "3: Refresh every 90 us"]
    REFRESH_3 = 3,
    #[doc = "4: Refresh every 120 us"]
    REFRESH_4 = 4,
    #[doc = "5: Refresh every 150 us"]
    REFRESH_5 = 5,
    #[doc = "6: Refresh every 180 us"]
    REFRESH_6 = 6,
    #[doc = "7: Refresh every 210 us"]
    REFRESH_7 = 7,
    #[doc = "8: Refresh every 240 us"]
    REFRESH_8 = 8,
    #[doc = "9: Refresh every 270 us"]
    REFRESH_9 = 9,
    #[doc = "10: Refresh every 300 us"]
    REFRESH_10 = 10,
    #[doc = "11: Refresh every 330 us"]
    REFRESH_11 = 11,
    #[doc = "12: Refresh every 360 us"]
    REFRESH_12 = 12,
    #[doc = "13: Refresh every 390 us"]
    REFRESH_13 = 13,
    #[doc = "14: Refresh every 420 us"]
    REFRESH_14 = 14,
    #[doc = "15: Refresh every 450 us"]
    REFRESH_15 = 15,
}
impl From<REFRESHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRESHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFRESHSELECT_A {
    type Ux = u8;
}
impl REFRESH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFRESHSELECT_A {
        match self.bits {
            0 => REFRESHSELECT_A::REFRESH_0,
            1 => REFRESHSELECT_A::REFRESH_1,
            2 => REFRESHSELECT_A::REFRESH_2,
            3 => REFRESHSELECT_A::REFRESH_3,
            4 => REFRESHSELECT_A::REFRESH_4,
            5 => REFRESHSELECT_A::REFRESH_5,
            6 => REFRESHSELECT_A::REFRESH_6,
            7 => REFRESHSELECT_A::REFRESH_7,
            8 => REFRESHSELECT_A::REFRESH_8,
            9 => REFRESHSELECT_A::REFRESH_9,
            10 => REFRESHSELECT_A::REFRESH_10,
            11 => REFRESHSELECT_A::REFRESH_11,
            12 => REFRESHSELECT_A::REFRESH_12,
            13 => REFRESHSELECT_A::REFRESH_13,
            14 => REFRESHSELECT_A::REFRESH_14,
            15 => REFRESHSELECT_A::REFRESH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not Refresh"]
    #[inline(always)]
    pub fn is_refresh_0(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_0
    }
    #[doc = "Refresh every 30 us"]
    #[inline(always)]
    pub fn is_refresh_1(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_1
    }
    #[doc = "Refresh every 60 us"]
    #[inline(always)]
    pub fn is_refresh_2(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_2
    }
    #[doc = "Refresh every 90 us"]
    #[inline(always)]
    pub fn is_refresh_3(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_3
    }
    #[doc = "Refresh every 120 us"]
    #[inline(always)]
    pub fn is_refresh_4(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_4
    }
    #[doc = "Refresh every 150 us"]
    #[inline(always)]
    pub fn is_refresh_5(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_5
    }
    #[doc = "Refresh every 180 us"]
    #[inline(always)]
    pub fn is_refresh_6(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_6
    }
    #[doc = "Refresh every 210 us"]
    #[inline(always)]
    pub fn is_refresh_7(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_7
    }
    #[doc = "Refresh every 240 us"]
    #[inline(always)]
    pub fn is_refresh_8(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_8
    }
    #[doc = "Refresh every 270 us"]
    #[inline(always)]
    pub fn is_refresh_9(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_9
    }
    #[doc = "Refresh every 300 us"]
    #[inline(always)]
    pub fn is_refresh_10(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_10
    }
    #[doc = "Refresh every 330 us"]
    #[inline(always)]
    pub fn is_refresh_11(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_11
    }
    #[doc = "Refresh every 360 us"]
    #[inline(always)]
    pub fn is_refresh_12(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_12
    }
    #[doc = "Refresh every 390 us"]
    #[inline(always)]
    pub fn is_refresh_13(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_13
    }
    #[doc = "Refresh every 420 us"]
    #[inline(always)]
    pub fn is_refresh_14(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_14
    }
    #[doc = "Refresh every 450 us"]
    #[inline(always)]
    pub fn is_refresh_15(&self) -> bool {
        *self == REFRESHSELECT_A::REFRESH_15
    }
}
#[doc = "Field `REFRESH` writer - Refresh period"]
pub type REFRESH_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, REFRESHSELECT_A>;
impl<'a, REG, const O: u8> REFRESH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not Refresh"]
    #[inline(always)]
    pub fn refresh_0(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_0)
    }
    #[doc = "Refresh every 30 us"]
    #[inline(always)]
    pub fn refresh_1(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_1)
    }
    #[doc = "Refresh every 60 us"]
    #[inline(always)]
    pub fn refresh_2(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_2)
    }
    #[doc = "Refresh every 90 us"]
    #[inline(always)]
    pub fn refresh_3(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_3)
    }
    #[doc = "Refresh every 120 us"]
    #[inline(always)]
    pub fn refresh_4(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_4)
    }
    #[doc = "Refresh every 150 us"]
    #[inline(always)]
    pub fn refresh_5(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_5)
    }
    #[doc = "Refresh every 180 us"]
    #[inline(always)]
    pub fn refresh_6(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_6)
    }
    #[doc = "Refresh every 210 us"]
    #[inline(always)]
    pub fn refresh_7(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_7)
    }
    #[doc = "Refresh every 240 us"]
    #[inline(always)]
    pub fn refresh_8(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_8)
    }
    #[doc = "Refresh every 270 us"]
    #[inline(always)]
    pub fn refresh_9(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_9)
    }
    #[doc = "Refresh every 300 us"]
    #[inline(always)]
    pub fn refresh_10(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_10)
    }
    #[doc = "Refresh every 330 us"]
    #[inline(always)]
    pub fn refresh_11(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_11)
    }
    #[doc = "Refresh every 360 us"]
    #[inline(always)]
    pub fn refresh_12(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_12)
    }
    #[doc = "Refresh every 390 us"]
    #[inline(always)]
    pub fn refresh_13(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_13)
    }
    #[doc = "Refresh every 420 us"]
    #[inline(always)]
    pub fn refresh_14(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_14)
    }
    #[doc = "Refresh every 450 us"]
    #[inline(always)]
    pub fn refresh_15(self) -> &'a mut crate::W<REG> {
        self.variant(REFRESHSELECT_A::REFRESH_15)
    }
}
#[doc = "Field `OSR` reader - Sampling Rate"]
pub type OSR_R = crate::FieldReader<OSRSELECT_A>;
#[doc = "Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSRSELECT_A {
    #[doc = "0: No Over Sampling"]
    OSR_1 = 0,
    #[doc = "1: 2x Over Sampling Ratio"]
    OSR_2 = 1,
    #[doc = "2: 4x Over Sampling Ratio"]
    OSR_4 = 2,
    #[doc = "3: 8x Over Sampling Ratio"]
    OSR_8 = 3,
    #[doc = "4: 16x Over Sampling Ratio"]
    OSR_16 = 4,
    #[doc = "5: 32x Over Sampling Ratio"]
    OSR_32 = 5,
}
impl From<OSRSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OSRSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSRSELECT_A {
    type Ux = u8;
}
impl OSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OSRSELECT_A> {
        match self.bits {
            0 => Some(OSRSELECT_A::OSR_1),
            1 => Some(OSRSELECT_A::OSR_2),
            2 => Some(OSRSELECT_A::OSR_4),
            3 => Some(OSRSELECT_A::OSR_8),
            4 => Some(OSRSELECT_A::OSR_16),
            5 => Some(OSRSELECT_A::OSR_32),
            _ => None,
        }
    }
    #[doc = "No Over Sampling"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == OSRSELECT_A::OSR_1
    }
    #[doc = "2x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == OSRSELECT_A::OSR_2
    }
    #[doc = "4x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSRSELECT_A::OSR_4
    }
    #[doc = "8x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSRSELECT_A::OSR_8
    }
    #[doc = "16x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSRSELECT_A::OSR_16
    }
    #[doc = "32x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == OSRSELECT_A::OSR_32
    }
}
#[doc = "Field `OSR` writer - Sampling Rate"]
pub type OSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, OSRSELECT_A>;
impl<'a, REG, const O: u8> OSR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Over Sampling"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut crate::W<REG> {
        self.variant(OSRSELECT_A::OSR_1)
    }
    #[doc = "2x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut crate::W<REG> {
        self.variant(OSRSELECT_A::OSR_2)
    }
    #[doc = "4x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut crate::W<REG> {
        self.variant(OSRSELECT_A::OSR_4)
    }
    #[doc = "8x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut crate::W<REG> {
        self.variant(OSRSELECT_A::OSR_8)
    }
    #[doc = "16x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut crate::W<REG> {
        self.variant(OSRSELECT_A::OSR_16)
    }
    #[doc = "32x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut crate::W<REG> {
        self.variant(OSRSELECT_A::OSR_32)
    }
}
impl R {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    pub fn cctrl(&self) -> CCTRL_R {
        CCTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline(always)]
    pub fn fext(&self) -> FEXT_R {
        FEXT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<DACCTRL_SPEC, 0> {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<DACCTRL_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn cctrl(&mut self) -> CCTRL_W<DACCTRL_SPEC, 2> {
        CCTRL_W::new(self)
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline(always)]
    #[must_use]
    pub fn fext(&mut self) -> FEXT_W<DACCTRL_SPEC, 5> {
        FEXT_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<DACCTRL_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<DACCTRL_SPEC, 7> {
        DITHER_W::new(self)
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<DACCTRL_SPEC, 8> {
        REFRESH_W::new(self)
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<DACCTRL_SPEC, 13> {
        OSR_W::new(self)
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
#[doc = "DAC n Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACCTRL_SPEC;
impl crate::RegisterSpec for DACCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dacctrl::R`](R) reader structure"]
impl crate::Readable for DACCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dacctrl::W`](W) writer structure"]
impl crate::Writable for DACCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACCTRL[%s]
to value 0"]
impl crate::Resettable for DACCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
