#[doc = "Register `XOSC32K` reader"]
pub struct R(crate::R<XOSC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC32K` writer"]
pub struct W(crate::W<XOSC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<XOSC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader<bool>;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type EN32K_R = crate::BitReader<bool>;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type EN32K_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type EN1K_R = crate::BitReader<bool>;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type EN1K_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<u8, STARTUPSELECT_A>;
#[doc = "Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUPSELECT_A {
    #[doc = "0: 62.6 ms"]
    CYCLE2048 = 0,
    #[doc = "1: 125 ms"]
    CYCLE4096 = 1,
    #[doc = "2: 500 ms"]
    CYCLE16384 = 2,
    #[doc = "3: 1000 ms"]
    CYCLE32768 = 3,
    #[doc = "4: 2000 ms"]
    CYCLE65536 = 4,
    #[doc = "5: 4000 ms"]
    CYCLE131072 = 5,
    #[doc = "6: 8000 ms"]
    CYCLE262144 = 6,
}
impl From<STARTUPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPSELECT_A) -> Self {
        variant as _
    }
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTUPSELECT_A> {
        match self.bits {
            0 => Some(STARTUPSELECT_A::CYCLE2048),
            1 => Some(STARTUPSELECT_A::CYCLE4096),
            2 => Some(STARTUPSELECT_A::CYCLE16384),
            3 => Some(STARTUPSELECT_A::CYCLE32768),
            4 => Some(STARTUPSELECT_A::CYCLE65536),
            5 => Some(STARTUPSELECT_A::CYCLE131072),
            6 => Some(STARTUPSELECT_A::CYCLE262144),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE2048`"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE2048
    }
    #[doc = "Checks if the value of the field is `CYCLE4096`"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE4096
    }
    #[doc = "Checks if the value of the field is `CYCLE16384`"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE16384
    }
    #[doc = "Checks if the value of the field is `CYCLE32768`"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE32768
    }
    #[doc = "Checks if the value of the field is `CYCLE65536`"]
    #[inline(always)]
    pub fn is_cycle65536(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE65536
    }
    #[doc = "Checks if the value of the field is `CYCLE131072`"]
    #[inline(always)]
    pub fn is_cycle131072(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE131072
    }
    #[doc = "Checks if the value of the field is `CYCLE262144`"]
    #[inline(always)]
    pub fn is_cycle262144(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE262144
    }
}
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type STARTUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, XOSC32K_SPEC, u8, STARTUPSELECT_A, 3, O>;
impl<'a, const O: u8> STARTUP_W<'a, O> {
    #[doc = "62.6 ms"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut W {
        self.variant(STARTUPSELECT_A::CYCLE2048)
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut W {
        self.variant(STARTUPSELECT_A::CYCLE4096)
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut W {
        self.variant(STARTUPSELECT_A::CYCLE16384)
    }
    #[doc = "1000 ms"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut W {
        self.variant(STARTUPSELECT_A::CYCLE32768)
    }
    #[doc = "2000 ms"]
    #[inline(always)]
    pub fn cycle65536(self) -> &'a mut W {
        self.variant(STARTUPSELECT_A::CYCLE65536)
    }
    #[doc = "4000 ms"]
    #[inline(always)]
    pub fn cycle131072(self) -> &'a mut W {
        self.variant(STARTUPSELECT_A::CYCLE131072)
    }
    #[doc = "8000 ms"]
    #[inline(always)]
    pub fn cycle262144(self) -> &'a mut W {
        self.variant(STARTUPSELECT_A::CYCLE262144)
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u16, XOSC32K_SPEC, bool, O>;
#[doc = "Field `CGM` reader - Control Gain Mode"]
pub type CGM_R = crate::FieldReader<u8, CGMSELECT_A>;
#[doc = "Control Gain Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CGMSELECT_A {
    #[doc = "1: Standard mode"]
    XT = 1,
    #[doc = "2: High Speed mode"]
    HS = 2,
}
impl From<CGMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CGMSELECT_A) -> Self {
        variant as _
    }
}
impl CGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CGMSELECT_A> {
        match self.bits {
            1 => Some(CGMSELECT_A::XT),
            2 => Some(CGMSELECT_A::HS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == CGMSELECT_A::XT
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == CGMSELECT_A::HS
    }
}
#[doc = "Field `CGM` writer - Control Gain Mode"]
pub type CGM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, XOSC32K_SPEC, u8, CGMSELECT_A, 2, O>;
impl<'a, const O: u8> CGM_W<'a, O> {
    #[doc = "Standard mode"]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(CGMSELECT_A::XT)
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut W {
        self.variant(CGMSELECT_A::HS)
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
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Control Gain Mode"]
    #[inline(always)]
    pub fn cgm(&self) -> CGM_R {
        CGM_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XTALEN_W<2> {
        XTALEN_W::new(self)
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<3> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<4> {
        EN1K_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<8> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<12> {
        WRTLOCK_W::new(self)
    }
    #[doc = "Bits 13:14 - Control Gain Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cgm(&mut self) -> CGM_W<13> {
        CGM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32k](index.html) module"]
pub struct XOSC32K_SPEC;
impl crate::RegisterSpec for XOSC32K_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xosc32k::R](R) reader structure"]
impl crate::Readable for XOSC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc32k::W](W) writer structure"]
impl crate::Writable for XOSC32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSC32K to value 0x2080"]
impl crate::Resettable for XOSC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x2080;
}
