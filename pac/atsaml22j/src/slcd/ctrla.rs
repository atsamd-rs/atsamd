#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `DUTY` reader - Duty Ratio"]
pub type DUTY_R = crate::FieldReader<u8, DUTYSELECT_A>;
#[doc = "Duty Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUTYSELECT_A {
    #[doc = "0: Static duty"]
    STATIC = 0,
    #[doc = "1: 1/2 duty"]
    HALF = 1,
    #[doc = "2: 1/3 duty"]
    THIRD = 2,
    #[doc = "3: 1/4 duty"]
    FOURTH = 3,
    #[doc = "4: 1/6 duty"]
    SIXTH = 4,
    #[doc = "5: 1/8 duty"]
    EIGHT = 5,
}
impl From<DUTYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DUTYSELECT_A) -> Self {
        variant as _
    }
}
impl DUTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DUTYSELECT_A> {
        match self.bits {
            0 => Some(DUTYSELECT_A::STATIC),
            1 => Some(DUTYSELECT_A::HALF),
            2 => Some(DUTYSELECT_A::THIRD),
            3 => Some(DUTYSELECT_A::FOURTH),
            4 => Some(DUTYSELECT_A::SIXTH),
            5 => Some(DUTYSELECT_A::EIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == DUTYSELECT_A::STATIC
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == DUTYSELECT_A::HALF
    }
    #[doc = "Checks if the value of the field is `THIRD`"]
    #[inline(always)]
    pub fn is_third(&self) -> bool {
        *self == DUTYSELECT_A::THIRD
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == DUTYSELECT_A::FOURTH
    }
    #[doc = "Checks if the value of the field is `SIXTH`"]
    #[inline(always)]
    pub fn is_sixth(&self) -> bool {
        *self == DUTYSELECT_A::SIXTH
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DUTYSELECT_A::EIGHT
    }
}
#[doc = "Field `DUTY` writer - Duty Ratio"]
pub type DUTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, DUTYSELECT_A, 3, O>;
impl<'a, const O: u8> DUTY_W<'a, O> {
    #[doc = "Static duty"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(DUTYSELECT_A::STATIC)
    }
    #[doc = "1/2 duty"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(DUTYSELECT_A::HALF)
    }
    #[doc = "1/3 duty"]
    #[inline(always)]
    pub fn third(self) -> &'a mut W {
        self.variant(DUTYSELECT_A::THIRD)
    }
    #[doc = "1/4 duty"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(DUTYSELECT_A::FOURTH)
    }
    #[doc = "1/6 duty"]
    #[inline(always)]
    pub fn sixth(self) -> &'a mut W {
        self.variant(DUTYSELECT_A::SIXTH)
    }
    #[doc = "1/8 duty"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(DUTYSELECT_A::EIGHT)
    }
}
#[doc = "Field `WMOD` reader - Waveform Mode"]
pub type WMOD_R = crate::BitReader<WMODSELECT_A>;
#[doc = "Waveform Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WMODSELECT_A {
    #[doc = "0: Low Power Waveform Mode"]
    LP = 0,
    #[doc = "1: Standard Waveform Mode"]
    STD = 1,
}
impl From<WMODSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WMODSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMODSELECT_A {
        match self.bits {
            false => WMODSELECT_A::LP,
            true => WMODSELECT_A::STD,
        }
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == WMODSELECT_A::LP
    }
    #[doc = "Checks if the value of the field is `STD`"]
    #[inline(always)]
    pub fn is_std(&self) -> bool {
        *self == WMODSELECT_A::STD
    }
}
#[doc = "Field `WMOD` writer - Waveform Mode"]
pub type WMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, WMODSELECT_A, O>;
impl<'a, const O: u8> WMOD_W<'a, O> {
    #[doc = "Low Power Waveform Mode"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(WMODSELECT_A::LP)
    }
    #[doc = "Standard Waveform Mode"]
    #[inline(always)]
    pub fn std(self) -> &'a mut W {
        self.variant(WMODSELECT_A::STD)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PRESC` reader - Clock Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESCSELECT_A>;
#[doc = "Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCSELECT_A {
    #[doc = "0: 16"]
    PRESC16 = 0,
    #[doc = "1: 32"]
    PRESC32 = 1,
    #[doc = "2: 64"]
    PRESC64 = 2,
    #[doc = "3: 128"]
    PRESC128 = 3,
}
impl From<PRESCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCSELECT_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCSELECT_A {
        match self.bits {
            0 => PRESCSELECT_A::PRESC16,
            1 => PRESCSELECT_A::PRESC32,
            2 => PRESCSELECT_A::PRESC64,
            3 => PRESCSELECT_A::PRESC128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRESC16`"]
    #[inline(always)]
    pub fn is_presc16(&self) -> bool {
        *self == PRESCSELECT_A::PRESC16
    }
    #[doc = "Checks if the value of the field is `PRESC32`"]
    #[inline(always)]
    pub fn is_presc32(&self) -> bool {
        *self == PRESCSELECT_A::PRESC32
    }
    #[doc = "Checks if the value of the field is `PRESC64`"]
    #[inline(always)]
    pub fn is_presc64(&self) -> bool {
        *self == PRESCSELECT_A::PRESC64
    }
    #[doc = "Checks if the value of the field is `PRESC128`"]
    #[inline(always)]
    pub fn is_presc128(&self) -> bool {
        *self == PRESCSELECT_A::PRESC128
    }
}
#[doc = "Field `PRESC` writer - Clock Prescaler"]
pub type PRESC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, PRESCSELECT_A, 2, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "16"]
    #[inline(always)]
    pub fn presc16(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::PRESC16)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn presc32(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::PRESC32)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn presc64(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::PRESC64)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn presc128(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::PRESC128)
    }
}
#[doc = "Field `CKDIV` reader - Clock Divider"]
pub type CKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDIV` writer - Clock Divider"]
pub type CKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIAS` reader - Bias Setting"]
pub type BIAS_R = crate::FieldReader<u8, BIASSELECT_A>;
#[doc = "Bias Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIASSELECT_A {
    #[doc = "0: Static"]
    STATIC = 0,
    #[doc = "1: 1/2 bias"]
    HALF = 1,
    #[doc = "2: 1/3 bias"]
    THIRD = 2,
    #[doc = "3: 1/4 bias"]
    FOURTH = 3,
}
impl From<BIASSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BIASSELECT_A) -> Self {
        variant as _
    }
}
impl BIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIASSELECT_A {
        match self.bits {
            0 => BIASSELECT_A::STATIC,
            1 => BIASSELECT_A::HALF,
            2 => BIASSELECT_A::THIRD,
            3 => BIASSELECT_A::FOURTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == BIASSELECT_A::STATIC
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == BIASSELECT_A::HALF
    }
    #[doc = "Checks if the value of the field is `THIRD`"]
    #[inline(always)]
    pub fn is_third(&self) -> bool {
        *self == BIASSELECT_A::THIRD
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == BIASSELECT_A::FOURTH
    }
}
#[doc = "Field `BIAS` writer - Bias Setting"]
pub type BIAS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, BIASSELECT_A, 2, O>;
impl<'a, const O: u8> BIAS_W<'a, O> {
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(BIASSELECT_A::STATIC)
    }
    #[doc = "1/2 bias"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(BIASSELECT_A::HALF)
    }
    #[doc = "1/3 bias"]
    #[inline(always)]
    pub fn third(self) -> &'a mut W {
        self.variant(BIASSELECT_A::THIRD)
    }
    #[doc = "1/4 bias"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(BIASSELECT_A::FOURTH)
    }
}
#[doc = "Field `XVLCD` reader - External VLCD"]
pub type XVLCD_R = crate::BitReader<bool>;
#[doc = "Field `XVLCD` writer - External VLCD"]
pub type XVLCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PRF` reader - Power Refresh Frequency"]
pub type PRF_R = crate::FieldReader<u8, PRFSELECT_A>;
#[doc = "Power Refresh Frequency\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRFSELECT_A {
    #[doc = "0: 2kHz"]
    PR2000 = 0,
    #[doc = "1: 1kHz"]
    PR1000 = 1,
    #[doc = "2: 500Hz"]
    PR500 = 2,
    #[doc = "3: 250Hz"]
    PR250 = 3,
}
impl From<PRFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRFSELECT_A) -> Self {
        variant as _
    }
}
impl PRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFSELECT_A {
        match self.bits {
            0 => PRFSELECT_A::PR2000,
            1 => PRFSELECT_A::PR1000,
            2 => PRFSELECT_A::PR500,
            3 => PRFSELECT_A::PR250,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PR2000`"]
    #[inline(always)]
    pub fn is_pr2000(&self) -> bool {
        *self == PRFSELECT_A::PR2000
    }
    #[doc = "Checks if the value of the field is `PR1000`"]
    #[inline(always)]
    pub fn is_pr1000(&self) -> bool {
        *self == PRFSELECT_A::PR1000
    }
    #[doc = "Checks if the value of the field is `PR500`"]
    #[inline(always)]
    pub fn is_pr500(&self) -> bool {
        *self == PRFSELECT_A::PR500
    }
    #[doc = "Checks if the value of the field is `PR250`"]
    #[inline(always)]
    pub fn is_pr250(&self) -> bool {
        *self == PRFSELECT_A::PR250
    }
}
#[doc = "Field `PRF` writer - Power Refresh Frequency"]
pub type PRF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, PRFSELECT_A, 2, O>;
impl<'a, const O: u8> PRF_W<'a, O> {
    #[doc = "2kHz"]
    #[inline(always)]
    pub fn pr2000(self) -> &'a mut W {
        self.variant(PRFSELECT_A::PR2000)
    }
    #[doc = "1kHz"]
    #[inline(always)]
    pub fn pr1000(self) -> &'a mut W {
        self.variant(PRFSELECT_A::PR1000)
    }
    #[doc = "500Hz"]
    #[inline(always)]
    pub fn pr500(self) -> &'a mut W {
        self.variant(PRFSELECT_A::PR500)
    }
    #[doc = "250Hz"]
    #[inline(always)]
    pub fn pr250(self) -> &'a mut W {
        self.variant(PRFSELECT_A::PR250)
    }
}
#[doc = "Field `DMFCS` reader - Display Memory Update Frame Counter Selection"]
pub type DMFCS_R = crate::FieldReader<u8, DMFCSSELECT_A>;
#[doc = "Display Memory Update Frame Counter Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMFCSSELECT_A {
    #[doc = "0: Frame Counter 0"]
    FC0 = 0,
    #[doc = "1: Frame Counter 1"]
    FC1 = 1,
    #[doc = "2: Frame Counter 2"]
    FC2 = 2,
    #[doc = "3: Frame Counter event to DMU is forced to 0"]
    NFC = 3,
}
impl From<DMFCSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMFCSSELECT_A) -> Self {
        variant as _
    }
}
impl DMFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMFCSSELECT_A {
        match self.bits {
            0 => DMFCSSELECT_A::FC0,
            1 => DMFCSSELECT_A::FC1,
            2 => DMFCSSELECT_A::FC2,
            3 => DMFCSSELECT_A::NFC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FC0`"]
    #[inline(always)]
    pub fn is_fc0(&self) -> bool {
        *self == DMFCSSELECT_A::FC0
    }
    #[doc = "Checks if the value of the field is `FC1`"]
    #[inline(always)]
    pub fn is_fc1(&self) -> bool {
        *self == DMFCSSELECT_A::FC1
    }
    #[doc = "Checks if the value of the field is `FC2`"]
    #[inline(always)]
    pub fn is_fc2(&self) -> bool {
        *self == DMFCSSELECT_A::FC2
    }
    #[doc = "Checks if the value of the field is `NFC`"]
    #[inline(always)]
    pub fn is_nfc(&self) -> bool {
        *self == DMFCSSELECT_A::NFC
    }
}
#[doc = "Field `DMFCS` writer - Display Memory Update Frame Counter Selection"]
pub type DMFCS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, DMFCSSELECT_A, 2, O>;
impl<'a, const O: u8> DMFCS_W<'a, O> {
    #[doc = "Frame Counter 0"]
    #[inline(always)]
    pub fn fc0(self) -> &'a mut W {
        self.variant(DMFCSSELECT_A::FC0)
    }
    #[doc = "Frame Counter 1"]
    #[inline(always)]
    pub fn fc1(self) -> &'a mut W {
        self.variant(DMFCSSELECT_A::FC1)
    }
    #[doc = "Frame Counter 2"]
    #[inline(always)]
    pub fn fc2(self) -> &'a mut W {
        self.variant(DMFCSSELECT_A::FC2)
    }
    #[doc = "Frame Counter event to DMU is forced to 0"]
    #[inline(always)]
    pub fn nfc(self) -> &'a mut W {
        self.variant(DMFCSSELECT_A::NFC)
    }
}
#[doc = "Field `RRF` reader - Reference Refresh Frequency"]
pub type RRF_R = crate::FieldReader<u8, RRFSELECT_A>;
#[doc = "Reference Refresh Frequency\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RRFSELECT_A {
    #[doc = "0: 2kHz"]
    RR2000 = 0,
    #[doc = "1: 1kHz"]
    RR1000 = 1,
    #[doc = "2: 500Hz"]
    RR500 = 2,
    #[doc = "3: 250Hz"]
    RR250 = 3,
    #[doc = "4: 125Hz"]
    RR125 = 4,
    #[doc = "5: 62.5Hz"]
    RR62 = 5,
}
impl From<RRFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RRFSELECT_A) -> Self {
        variant as _
    }
}
impl RRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RRFSELECT_A> {
        match self.bits {
            0 => Some(RRFSELECT_A::RR2000),
            1 => Some(RRFSELECT_A::RR1000),
            2 => Some(RRFSELECT_A::RR500),
            3 => Some(RRFSELECT_A::RR250),
            4 => Some(RRFSELECT_A::RR125),
            5 => Some(RRFSELECT_A::RR62),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RR2000`"]
    #[inline(always)]
    pub fn is_rr2000(&self) -> bool {
        *self == RRFSELECT_A::RR2000
    }
    #[doc = "Checks if the value of the field is `RR1000`"]
    #[inline(always)]
    pub fn is_rr1000(&self) -> bool {
        *self == RRFSELECT_A::RR1000
    }
    #[doc = "Checks if the value of the field is `RR500`"]
    #[inline(always)]
    pub fn is_rr500(&self) -> bool {
        *self == RRFSELECT_A::RR500
    }
    #[doc = "Checks if the value of the field is `RR250`"]
    #[inline(always)]
    pub fn is_rr250(&self) -> bool {
        *self == RRFSELECT_A::RR250
    }
    #[doc = "Checks if the value of the field is `RR125`"]
    #[inline(always)]
    pub fn is_rr125(&self) -> bool {
        *self == RRFSELECT_A::RR125
    }
    #[doc = "Checks if the value of the field is `RR62`"]
    #[inline(always)]
    pub fn is_rr62(&self) -> bool {
        *self == RRFSELECT_A::RR62
    }
}
#[doc = "Field `RRF` writer - Reference Refresh Frequency"]
pub type RRF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, RRFSELECT_A, 3, O>;
impl<'a, const O: u8> RRF_W<'a, O> {
    #[doc = "2kHz"]
    #[inline(always)]
    pub fn rr2000(self) -> &'a mut W {
        self.variant(RRFSELECT_A::RR2000)
    }
    #[doc = "1kHz"]
    #[inline(always)]
    pub fn rr1000(self) -> &'a mut W {
        self.variant(RRFSELECT_A::RR1000)
    }
    #[doc = "500Hz"]
    #[inline(always)]
    pub fn rr500(self) -> &'a mut W {
        self.variant(RRFSELECT_A::RR500)
    }
    #[doc = "250Hz"]
    #[inline(always)]
    pub fn rr250(self) -> &'a mut W {
        self.variant(RRFSELECT_A::RR250)
    }
    #[doc = "125Hz"]
    #[inline(always)]
    pub fn rr125(self) -> &'a mut W {
        self.variant(RRFSELECT_A::RR125)
    }
    #[doc = "62.5Hz"]
    #[inline(always)]
    pub fn rr62(self) -> &'a mut W {
        self.variant(RRFSELECT_A::RR62)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Duty Ratio"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Waveform Mode"]
    #[inline(always)]
    pub fn wmod(&self) -> WMOD_R {
        WMOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Bias Setting"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - External VLCD"]
    #[inline(always)]
    pub fn xvlcd(&self) -> XVLCD_R {
        XVLCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Power Refresh Frequency"]
    #[inline(always)]
    pub fn prf(&self) -> PRF_R {
        PRF_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Display Memory Update Frame Counter Selection"]
    #[inline(always)]
    pub fn dmfcs(&self) -> DMFCS_R {
        DMFCS_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Reference Refresh Frequency"]
    #[inline(always)]
    pub fn rrf(&self) -> RRF_R {
        RRF_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Duty Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<2> {
        DUTY_W::new(self)
    }
    #[doc = "Bit 5 - Waveform Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wmod(&mut self) -> WMOD_W<5> {
        WMOD_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<8> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 12:14 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<12> {
        CKDIV_W::new(self)
    }
    #[doc = "Bits 16:17 - Bias Setting"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<16> {
        BIAS_W::new(self)
    }
    #[doc = "Bit 19 - External VLCD"]
    #[inline(always)]
    #[must_use]
    pub fn xvlcd(&mut self) -> XVLCD_W<19> {
        XVLCD_W::new(self)
    }
    #[doc = "Bits 20:21 - Power Refresh Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn prf(&mut self) -> PRF_W<20> {
        PRF_W::new(self)
    }
    #[doc = "Bits 22:23 - Display Memory Update Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmfcs(&mut self) -> DMFCS_W<22> {
        DMFCS_W::new(self)
    }
    #[doc = "Bits 24:26 - Reference Refresh Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn rrf(&mut self) -> RRF_W<24> {
        RRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0x03d8_0000"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03d8_0000;
}
