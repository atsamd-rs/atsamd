#[doc = "Register `WAVEB` reader"]
pub struct R(crate::R<WAVEB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAVEB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAVEB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAVEB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAVEB` writer"]
pub struct W(crate::W<WAVEB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAVEB_SPEC>;
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
impl From<crate::W<WAVEB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAVEB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAVEGENB` reader - Waveform Generation Buffer"]
pub type WAVEGENB_R = crate::FieldReader<u8, WAVEGENBSELECT_A>;
#[doc = "Waveform Generation Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVEGENBSELECT_A {
    #[doc = "0: Normal frequency"]
    NFRQ = 0,
    #[doc = "1: Match frequency"]
    MFRQ = 1,
    #[doc = "2: Normal PWM"]
    NPWM = 2,
    #[doc = "4: Dual-slope critical"]
    DSCRITICAL = 4,
    #[doc = "5: Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    DSBOTTOM = 5,
    #[doc = "6: Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    DSBOTH = 6,
    #[doc = "7: Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    DSTOP = 7,
}
impl From<WAVEGENBSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVEGENBSELECT_A) -> Self {
        variant as _
    }
}
impl WAVEGENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVEGENBSELECT_A> {
        match self.bits {
            0 => Some(WAVEGENBSELECT_A::NFRQ),
            1 => Some(WAVEGENBSELECT_A::MFRQ),
            2 => Some(WAVEGENBSELECT_A::NPWM),
            4 => Some(WAVEGENBSELECT_A::DSCRITICAL),
            5 => Some(WAVEGENBSELECT_A::DSBOTTOM),
            6 => Some(WAVEGENBSELECT_A::DSBOTH),
            7 => Some(WAVEGENBSELECT_A::DSTOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENBSELECT_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENBSELECT_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENBSELECT_A::NPWM
    }
    #[doc = "Checks if the value of the field is `DSCRITICAL`"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        *self == WAVEGENBSELECT_A::DSCRITICAL
    }
    #[doc = "Checks if the value of the field is `DSBOTTOM`"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == WAVEGENBSELECT_A::DSBOTTOM
    }
    #[doc = "Checks if the value of the field is `DSBOTH`"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == WAVEGENBSELECT_A::DSBOTH
    }
    #[doc = "Checks if the value of the field is `DSTOP`"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == WAVEGENBSELECT_A::DSTOP
    }
}
#[doc = "Field `WAVEGENB` writer - Waveform Generation Buffer"]
pub type WAVEGENB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAVEB_SPEC, u8, WAVEGENBSELECT_A, 3, O>;
impl<'a, const O: u8> WAVEGENB_W<'a, O> {
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGENBSELECT_A::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGENBSELECT_A::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGENBSELECT_A::NPWM)
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn dscritical(self) -> &'a mut W {
        self.variant(WAVEGENBSELECT_A::DSCRITICAL)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut W {
        self.variant(WAVEGENBSELECT_A::DSBOTTOM)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut W {
        self.variant(WAVEGENBSELECT_A::DSBOTH)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut W {
        self.variant(WAVEGENBSELECT_A::DSTOP)
    }
}
#[doc = "Field `RAMPB` reader - Ramp Mode Buffer"]
pub type RAMPB_R = crate::FieldReader<u8, RAMPBSELECT_A>;
#[doc = "Ramp Mode Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPBSELECT_A {
    #[doc = "0: RAMP1 operation"]
    RAMP1 = 0,
    #[doc = "1: Alternative RAMP2 operation"]
    RAMP2A = 1,
    #[doc = "2: RAMP2 operation"]
    RAMP2 = 2,
}
impl From<RAMPBSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPBSELECT_A) -> Self {
        variant as _
    }
}
impl RAMPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMPBSELECT_A> {
        match self.bits {
            0 => Some(RAMPBSELECT_A::RAMP1),
            1 => Some(RAMPBSELECT_A::RAMP2A),
            2 => Some(RAMPBSELECT_A::RAMP2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RAMP1`"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == RAMPBSELECT_A::RAMP1
    }
    #[doc = "Checks if the value of the field is `RAMP2A`"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        *self == RAMPBSELECT_A::RAMP2A
    }
    #[doc = "Checks if the value of the field is `RAMP2`"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        *self == RAMPBSELECT_A::RAMP2
    }
}
#[doc = "Field `RAMPB` writer - Ramp Mode Buffer"]
pub type RAMPB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAVEB_SPEC, u8, RAMPBSELECT_A, 2, O>;
impl<'a, const O: u8> RAMPB_W<'a, O> {
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn ramp1(self) -> &'a mut W {
        self.variant(RAMPBSELECT_A::RAMP1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2a(self) -> &'a mut W {
        self.variant(RAMPBSELECT_A::RAMP2A)
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2(self) -> &'a mut W {
        self.variant(RAMPBSELECT_A::RAMP2)
    }
}
#[doc = "Field `CIPERENB` reader - Circular Period Enable Buffer"]
pub type CIPERENB_R = crate::BitReader<bool>;
#[doc = "Field `CIPERENB` writer - Circular Period Enable Buffer"]
pub type CIPERENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `CICCENB0` reader - Circular Channel 0 Enable Buffer"]
pub type CICCENB0_R = crate::BitReader<bool>;
#[doc = "Field `CICCENB0` writer - Circular Channel 0 Enable Buffer"]
pub type CICCENB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `CICCENB1` reader - Circular Channel 1 Enable Buffer"]
pub type CICCENB1_R = crate::BitReader<bool>;
#[doc = "Field `CICCENB1` writer - Circular Channel 1 Enable Buffer"]
pub type CICCENB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `CICCENB2` reader - Circular Channel 2 Enable Buffer"]
pub type CICCENB2_R = crate::BitReader<bool>;
#[doc = "Field `CICCENB2` writer - Circular Channel 2 Enable Buffer"]
pub type CICCENB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `CICCENB3` reader - Circular Channel 3 Enable Buffer"]
pub type CICCENB3_R = crate::BitReader<bool>;
#[doc = "Field `CICCENB3` writer - Circular Channel 3 Enable Buffer"]
pub type CICCENB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `POLB0` reader - Channel 0 Polarity Buffer"]
pub type POLB0_R = crate::BitReader<bool>;
#[doc = "Field `POLB0` writer - Channel 0 Polarity Buffer"]
pub type POLB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `POLB1` reader - Channel 1 Polarity Buffer"]
pub type POLB1_R = crate::BitReader<bool>;
#[doc = "Field `POLB1` writer - Channel 1 Polarity Buffer"]
pub type POLB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `POLB2` reader - Channel 2 Polarity Buffer"]
pub type POLB2_R = crate::BitReader<bool>;
#[doc = "Field `POLB2` writer - Channel 2 Polarity Buffer"]
pub type POLB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `POLB3` reader - Channel 3 Polarity Buffer"]
pub type POLB3_R = crate::BitReader<bool>;
#[doc = "Field `POLB3` writer - Channel 3 Polarity Buffer"]
pub type POLB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `SWAPB0` reader - Swap DTI Output Pair 0 Buffer"]
pub type SWAPB0_R = crate::BitReader<bool>;
#[doc = "Field `SWAPB0` writer - Swap DTI Output Pair 0 Buffer"]
pub type SWAPB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `SWAPB1` reader - Swap DTI Output Pair 1 Buffer"]
pub type SWAPB1_R = crate::BitReader<bool>;
#[doc = "Field `SWAPB1` writer - Swap DTI Output Pair 1 Buffer"]
pub type SWAPB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `SWAPB2` reader - Swap DTI Output Pair 2 Buffer"]
pub type SWAPB2_R = crate::BitReader<bool>;
#[doc = "Field `SWAPB2` writer - Swap DTI Output Pair 2 Buffer"]
pub type SWAPB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
#[doc = "Field `SWAPB3` reader - Swap DTI Output Pair 3 Buffer"]
pub type SWAPB3_R = crate::BitReader<bool>;
#[doc = "Field `SWAPB3` writer - Swap DTI Output Pair 3 Buffer"]
pub type SWAPB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAVEB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline(always)]
    pub fn wavegenb(&self) -> WAVEGENB_R {
        WAVEGENB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline(always)]
    pub fn rampb(&self) -> RAMPB_R {
        RAMPB_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline(always)]
    pub fn ciperenb(&self) -> CIPERENB_R {
        CIPERENB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb0(&self) -> CICCENB0_R {
        CICCENB0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb1(&self) -> CICCENB1_R {
        CICCENB1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb2(&self) -> CICCENB2_R {
        CICCENB2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb3(&self) -> CICCENB3_R {
        CICCENB3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline(always)]
    pub fn polb0(&self) -> POLB0_R {
        POLB0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline(always)]
    pub fn polb1(&self) -> POLB1_R {
        POLB1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline(always)]
    pub fn polb2(&self) -> POLB2_R {
        POLB2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline(always)]
    pub fn polb3(&self) -> POLB3_R {
        POLB3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline(always)]
    pub fn swapb0(&self) -> SWAPB0_R {
        SWAPB0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline(always)]
    pub fn swapb1(&self) -> SWAPB1_R {
        SWAPB1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline(always)]
    pub fn swapb2(&self) -> SWAPB2_R {
        SWAPB2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline(always)]
    pub fn swapb3(&self) -> SWAPB3_R {
        SWAPB3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn wavegenb(&mut self) -> WAVEGENB_W<0> {
        WAVEGENB_W::new(self)
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn rampb(&mut self) -> RAMPB_W<4> {
        RAMPB_W::new(self)
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciperenb(&mut self) -> CIPERENB_W<7> {
        CIPERENB_W::new(self)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb0(&mut self) -> CICCENB0_W<8> {
        CICCENB0_W::new(self)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb1(&mut self) -> CICCENB1_W<9> {
        CICCENB1_W::new(self)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb2(&mut self) -> CICCENB2_W<10> {
        CICCENB2_W::new(self)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb3(&mut self) -> CICCENB3_W<11> {
        CICCENB3_W::new(self)
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb0(&mut self) -> POLB0_W<16> {
        POLB0_W::new(self)
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb1(&mut self) -> POLB1_W<17> {
        POLB1_W::new(self)
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb2(&mut self) -> POLB2_W<18> {
        POLB2_W::new(self)
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb3(&mut self) -> POLB3_W<19> {
        POLB3_W::new(self)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb0(&mut self) -> SWAPB0_W<24> {
        SWAPB0_W::new(self)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb1(&mut self) -> SWAPB1_W<25> {
        SWAPB1_W::new(self)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb2(&mut self) -> SWAPB2_W<26> {
        SWAPB2_W::new(self)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb3(&mut self) -> SWAPB3_W<27> {
        SWAPB3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Waveform Control Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waveb](index.html) module"]
pub struct WAVEB_SPEC;
impl crate::RegisterSpec for WAVEB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waveb::R](R) reader structure"]
impl crate::Readable for WAVEB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [waveb::W](W) writer structure"]
impl crate::Writable for WAVEB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAVEB to value 0"]
impl crate::Resettable for WAVEB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
