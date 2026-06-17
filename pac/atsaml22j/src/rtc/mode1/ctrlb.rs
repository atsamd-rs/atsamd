#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP0EN` reader - General Purpose 0 Enable"]
pub type GP0EN_R = crate::BitReader<bool>;
#[doc = "Field `GP0EN` writer - General Purpose 0 Enable"]
pub type GP0EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `DEBMAJ` reader - Debouncer Majority Enable"]
pub type DEBMAJ_R = crate::BitReader<bool>;
#[doc = "Field `DEBMAJ` writer - Debouncer Majority Enable"]
pub type DEBMAJ_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `DEBASYNC` reader - Debouncer Asynchronous Enable"]
pub type DEBASYNC_R = crate::BitReader<bool>;
#[doc = "Field `DEBASYNC` writer - Debouncer Asynchronous Enable"]
pub type DEBASYNC_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `RTCOUT` reader - RTC Output Enable"]
pub type RTCOUT_R = crate::BitReader<bool>;
#[doc = "Field `RTCOUT` writer - RTC Output Enable"]
pub type RTCOUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `DEBF` reader - Debounce Freqnuency"]
pub type DEBF_R = crate::FieldReader<u8, DEBFSELECT_A>;
#[doc = "Debounce Freqnuency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEBFSELECT_A {
    #[doc = "0: CLK_RTC_DEB = CLK_RTC/2"]
    DIV2 = 0,
    #[doc = "1: CLK_RTC_DEB = CLK_RTC/4"]
    DIV4 = 1,
    #[doc = "2: CLK_RTC_DEB = CLK_RTC/8"]
    DIV8 = 2,
    #[doc = "3: CLK_RTC_DEB = CLK_RTC/16"]
    DIV16 = 3,
    #[doc = "4: CLK_RTC_DEB = CLK_RTC/32"]
    DIV32 = 4,
    #[doc = "5: CLK_RTC_DEB = CLK_RTC/64"]
    DIV64 = 5,
    #[doc = "6: CLK_RTC_DEB = CLK_RTC/128"]
    DIV128 = 6,
    #[doc = "7: CLK_RTC_DEB = CLK_RTC/256"]
    DIV256 = 7,
}
impl From<DEBFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBFSELECT_A) -> Self {
        variant as _
    }
}
impl DEBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBFSELECT_A {
        match self.bits {
            0 => DEBFSELECT_A::DIV2,
            1 => DEBFSELECT_A::DIV4,
            2 => DEBFSELECT_A::DIV8,
            3 => DEBFSELECT_A::DIV16,
            4 => DEBFSELECT_A::DIV32,
            5 => DEBFSELECT_A::DIV64,
            6 => DEBFSELECT_A::DIV128,
            7 => DEBFSELECT_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DEBFSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DEBFSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DEBFSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DEBFSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DEBFSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DEBFSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DEBFSELECT_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == DEBFSELECT_A::DIV256
    }
}
#[doc = "Field `DEBF` writer - Debounce Freqnuency"]
pub type DEBF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CTRLB_SPEC, u8, DEBFSELECT_A, 3, O>;
impl<'a, const O: u8> DEBF_W<'a, O> {
    #[doc = "CLK_RTC_DEB = CLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV2)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV4)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV8)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV16)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV32)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV64)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV128)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(DEBFSELECT_A::DIV256)
    }
}
#[doc = "Field `ACTF` reader - Active Layer Freqnuency"]
pub type ACTF_R = crate::FieldReader<u8, ACTFSELECT_A>;
#[doc = "Active Layer Freqnuency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTFSELECT_A {
    #[doc = "0: CLK_RTC_OUT = CLK_RTC/2"]
    DIV2 = 0,
    #[doc = "1: CLK_RTC_OUT = CLK_RTC/4"]
    DIV4 = 1,
    #[doc = "2: CLK_RTC_OUT = CLK_RTC/8"]
    DIV8 = 2,
    #[doc = "3: CLK_RTC_OUT = CLK_RTC/16"]
    DIV16 = 3,
    #[doc = "4: CLK_RTC_OUT = CLK_RTC/32"]
    DIV32 = 4,
    #[doc = "5: CLK_RTC_OUT = CLK_RTC/64"]
    DIV64 = 5,
    #[doc = "6: CLK_RTC_OUT = CLK_RTC/128"]
    DIV128 = 6,
    #[doc = "7: CLK_RTC_OUT = CLK_RTC/256"]
    DIV256 = 7,
}
impl From<ACTFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTFSELECT_A) -> Self {
        variant as _
    }
}
impl ACTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTFSELECT_A {
        match self.bits {
            0 => ACTFSELECT_A::DIV2,
            1 => ACTFSELECT_A::DIV4,
            2 => ACTFSELECT_A::DIV8,
            3 => ACTFSELECT_A::DIV16,
            4 => ACTFSELECT_A::DIV32,
            5 => ACTFSELECT_A::DIV64,
            6 => ACTFSELECT_A::DIV128,
            7 => ACTFSELECT_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ACTFSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ACTFSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ACTFSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == ACTFSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == ACTFSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == ACTFSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == ACTFSELECT_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == ACTFSELECT_A::DIV256
    }
}
#[doc = "Field `ACTF` writer - Active Layer Freqnuency"]
pub type ACTF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CTRLB_SPEC, u8, ACTFSELECT_A, 3, O>;
impl<'a, const O: u8> ACTF_W<'a, O> {
    #[doc = "CLK_RTC_OUT = CLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV2)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV4)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV8)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV16)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV32)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV64)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV128)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(ACTFSELECT_A::DIV256)
    }
}
impl R {
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline(always)]
    pub fn gp0en(&self) -> GP0EN_R {
        GP0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline(always)]
    pub fn debmaj(&self) -> DEBMAJ_R {
        DEBMAJ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline(always)]
    pub fn debasync(&self) -> DEBASYNC_R {
        DEBASYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline(always)]
    pub fn rtcout(&self) -> RTCOUT_R {
        RTCOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline(always)]
    pub fn debf(&self) -> DEBF_R {
        DEBF_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline(always)]
    pub fn actf(&self) -> ACTF_R {
        ACTF_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gp0en(&mut self) -> GP0EN_W<0> {
        GP0EN_W::new(self)
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debmaj(&mut self) -> DEBMAJ_W<4> {
        DEBMAJ_W::new(self)
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debasync(&mut self) -> DEBASYNC_W<5> {
        DEBASYNC_W::new(self)
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcout(&mut self) -> RTCOUT_W<6> {
        RTCOUT_W::new(self)
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<7> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline(always)]
    #[must_use]
    pub fn debf(&mut self) -> DEBF_W<8> {
        DEBF_W::new(self)
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline(always)]
    #[must_use]
    pub fn actf(&mut self) -> ACTF_W<12> {
        ACTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE1 Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
