#[doc = "Register `BOD33` reader"]
pub struct R(crate::R<BOD33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33` writer"]
pub struct W(crate::W<BOD33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33_SPEC>;
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
impl From<crate::W<BOD33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33_SPEC, bool, O>;
#[doc = "Field `HYST` reader - Hysteresis"]
pub type HYST_R = crate::BitReader<bool>;
#[doc = "Field `HYST` writer - Hysteresis"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33_SPEC, bool, O>;
#[doc = "Field `ACTION` reader - BOD33 Action"]
pub type ACTION_R = crate::FieldReader<u8, ACTIONSELECT_A>;
#[doc = "BOD33 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIONSELECT_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: The BOD33 generates a reset"]
    RESET = 1,
    #[doc = "2: The BOD33 generates an interrupt"]
    INTERRUPT = 2,
}
impl From<ACTIONSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIONSELECT_A) -> Self {
        variant as _
    }
}
impl ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACTIONSELECT_A> {
        match self.bits {
            0 => Some(ACTIONSELECT_A::NONE),
            1 => Some(ACTIONSELECT_A::RESET),
            2 => Some(ACTIONSELECT_A::INTERRUPT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTIONSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ACTIONSELECT_A::RESET
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == ACTIONSELECT_A::INTERRUPT
    }
}
#[doc = "Field `ACTION` writer - BOD33 Action"]
pub type ACTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOD33_SPEC, u8, ACTIONSELECT_A, 2, O>;
impl<'a, const O: u8> ACTION_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTIONSELECT_A::NONE)
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ACTIONSELECT_A::RESET)
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ACTIONSELECT_A::INTERRUPT)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Operation Mode"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Operation Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33_SPEC, bool, O>;
#[doc = "Field `CEN` reader - Clock Enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Clock Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33_SPEC, bool, O>;
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PSEL_R = crate::FieldReader<u8, PSELSELECT_A>;
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSELSELECT_A {
    #[doc = "0: Divide clock by 2"]
    DIV2 = 0,
    #[doc = "1: Divide clock by 4"]
    DIV4 = 1,
    #[doc = "2: Divide clock by 8"]
    DIV8 = 2,
    #[doc = "3: Divide clock by 16"]
    DIV16 = 3,
    #[doc = "4: Divide clock by 32"]
    DIV32 = 4,
    #[doc = "5: Divide clock by 64"]
    DIV64 = 5,
    #[doc = "6: Divide clock by 128"]
    DIV128 = 6,
    #[doc = "7: Divide clock by 256"]
    DIV256 = 7,
    #[doc = "8: Divide clock by 512"]
    DIV512 = 8,
    #[doc = "9: Divide clock by 1024"]
    DIV1K = 9,
    #[doc = "10: Divide clock by 2048"]
    DIV2K = 10,
    #[doc = "11: Divide clock by 4096"]
    DIV4K = 11,
    #[doc = "12: Divide clock by 8192"]
    DIV8K = 12,
    #[doc = "13: Divide clock by 16384"]
    DIV16K = 13,
    #[doc = "14: Divide clock by 32768"]
    DIV32K = 14,
    #[doc = "15: Divide clock by 65536"]
    DIV64K = 15,
}
impl From<PSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSELSELECT_A) -> Self {
        variant as _
    }
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSELSELECT_A {
        match self.bits {
            0 => PSELSELECT_A::DIV2,
            1 => PSELSELECT_A::DIV4,
            2 => PSELSELECT_A::DIV8,
            3 => PSELSELECT_A::DIV16,
            4 => PSELSELECT_A::DIV32,
            5 => PSELSELECT_A::DIV64,
            6 => PSELSELECT_A::DIV128,
            7 => PSELSELECT_A::DIV256,
            8 => PSELSELECT_A::DIV512,
            9 => PSELSELECT_A::DIV1K,
            10 => PSELSELECT_A::DIV2K,
            11 => PSELSELECT_A::DIV4K,
            12 => PSELSELECT_A::DIV8K,
            13 => PSELSELECT_A::DIV16K,
            14 => PSELSELECT_A::DIV32K,
            15 => PSELSELECT_A::DIV64K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSELSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSELSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSELSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSELSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PSELSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PSELSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PSELSELECT_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PSELSELECT_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PSELSELECT_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1K`"]
    #[inline(always)]
    pub fn is_div1k(&self) -> bool {
        *self == PSELSELECT_A::DIV1K
    }
    #[doc = "Checks if the value of the field is `DIV2K`"]
    #[inline(always)]
    pub fn is_div2k(&self) -> bool {
        *self == PSELSELECT_A::DIV2K
    }
    #[doc = "Checks if the value of the field is `DIV4K`"]
    #[inline(always)]
    pub fn is_div4k(&self) -> bool {
        *self == PSELSELECT_A::DIV4K
    }
    #[doc = "Checks if the value of the field is `DIV8K`"]
    #[inline(always)]
    pub fn is_div8k(&self) -> bool {
        *self == PSELSELECT_A::DIV8K
    }
    #[doc = "Checks if the value of the field is `DIV16K`"]
    #[inline(always)]
    pub fn is_div16k(&self) -> bool {
        *self == PSELSELECT_A::DIV16K
    }
    #[doc = "Checks if the value of the field is `DIV32K`"]
    #[inline(always)]
    pub fn is_div32k(&self) -> bool {
        *self == PSELSELECT_A::DIV32K
    }
    #[doc = "Checks if the value of the field is `DIV64K`"]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        *self == PSELSELECT_A::DIV64K
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BOD33_SPEC, u8, PSELSELECT_A, 4, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV2)
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV4)
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV8)
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV16)
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV32)
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV64)
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV128)
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV256)
    }
    #[doc = "Divide clock by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV512)
    }
    #[doc = "Divide clock by 1024"]
    #[inline(always)]
    pub fn div1k(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV1K)
    }
    #[doc = "Divide clock by 2048"]
    #[inline(always)]
    pub fn div2k(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV2K)
    }
    #[doc = "Divide clock by 4096"]
    #[inline(always)]
    pub fn div4k(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV4K)
    }
    #[doc = "Divide clock by 8192"]
    #[inline(always)]
    pub fn div8k(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV8K)
    }
    #[doc = "Divide clock by 16384"]
    #[inline(always)]
    pub fn div16k(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV16K)
    }
    #[doc = "Divide clock by 32768"]
    #[inline(always)]
    pub fn div32k(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV32K)
    }
    #[doc = "Divide clock by 65536"]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV64K)
    }
}
#[doc = "Field `LEVEL` reader - BOD33 Threshold Level"]
pub type LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVEL` writer - BOD33 Threshold Level"]
pub type LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD33_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<2> {
        HYST_W::new(self)
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<3> {
        ACTION_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<9> {
        CEN_W::new(self)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<12> {
        PSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<16> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "3.3V Brown-Out Detector (BOD33) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33](index.html) module"]
pub struct BOD33_SPEC;
impl crate::RegisterSpec for BOD33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33::R](R) reader structure"]
impl crate::Readable for BOD33_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33::W](W) writer structure"]
impl crate::Writable for BOD33_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD33 to value 0"]
impl crate::Resettable for BOD33_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
