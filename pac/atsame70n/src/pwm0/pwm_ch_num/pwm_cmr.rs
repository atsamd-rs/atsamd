#[doc = "Register `PWM_CMR` reader"]
pub struct R(crate::R<PWM_CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CMR` writer"]
pub struct W(crate::W<PWM_CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CMR_SPEC>;
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
impl From<crate::W<PWM_CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPRE_A {
    #[doc = "0: Peripheral clock"]
    MCK = 0,
    #[doc = "1: Peripheral clock/2"]
    MCK_DIV_2 = 1,
    #[doc = "2: Peripheral clock/4"]
    MCK_DIV_4 = 2,
    #[doc = "3: Peripheral clock/8"]
    MCK_DIV_8 = 3,
    #[doc = "4: Peripheral clock/16"]
    MCK_DIV_16 = 4,
    #[doc = "5: Peripheral clock/32"]
    MCK_DIV_32 = 5,
    #[doc = "6: Peripheral clock/64"]
    MCK_DIV_64 = 6,
    #[doc = "7: Peripheral clock/128"]
    MCK_DIV_128 = 7,
    #[doc = "8: Peripheral clock/256"]
    MCK_DIV_256 = 8,
    #[doc = "9: Peripheral clock/512"]
    MCK_DIV_512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    MCK_DIV_1024 = 10,
    #[doc = "11: Clock A"]
    CLKA = 11,
    #[doc = "12: Clock B"]
    CLKB = 12,
}
impl From<CPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub struct CPRE_R(crate::FieldReader<u8, CPRE_A>);
impl CPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPRE_A> {
        match self.bits {
            0 => Some(CPRE_A::MCK),
            1 => Some(CPRE_A::MCK_DIV_2),
            2 => Some(CPRE_A::MCK_DIV_4),
            3 => Some(CPRE_A::MCK_DIV_8),
            4 => Some(CPRE_A::MCK_DIV_16),
            5 => Some(CPRE_A::MCK_DIV_32),
            6 => Some(CPRE_A::MCK_DIV_64),
            7 => Some(CPRE_A::MCK_DIV_128),
            8 => Some(CPRE_A::MCK_DIV_256),
            9 => Some(CPRE_A::MCK_DIV_512),
            10 => Some(CPRE_A::MCK_DIV_1024),
            11 => Some(CPRE_A::CLKA),
            12 => Some(CPRE_A::CLKB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        **self == CPRE_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_2`"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        **self == CPRE_A::MCK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_4`"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        **self == CPRE_A::MCK_DIV_4
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_8`"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        **self == CPRE_A::MCK_DIV_8
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_16`"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        **self == CPRE_A::MCK_DIV_16
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_32`"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        **self == CPRE_A::MCK_DIV_32
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_64`"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        **self == CPRE_A::MCK_DIV_64
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_128`"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        **self == CPRE_A::MCK_DIV_128
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_256`"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        **self == CPRE_A::MCK_DIV_256
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_512`"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        **self == CPRE_A::MCK_DIV_512
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_1024`"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        **self == CPRE_A::MCK_DIV_1024
    }
    #[doc = "Checks if the value of the field is `CLKA`"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        **self == CPRE_A::CLKA
    }
    #[doc = "Checks if the value of the field is `CLKB`"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        **self == CPRE_A::CLKB
    }
}
impl core::ops::Deref for CPRE_R {
    type Target = crate::FieldReader<u8, CPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub struct CPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPRE_A::MCK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut W {
        self.variant(CPRE_A::CLKA)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut W {
        self.variant(CPRE_A::CLKB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Channel Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALG_A {
    #[doc = "0: Left aligned"]
    LEFT_ALIGNED = 0,
    #[doc = "1: Center aligned"]
    CENTER_ALIGNED = 1,
}
impl From<CALG_A> for bool {
    #[inline(always)]
    fn from(variant: CALG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub struct CALG_R(crate::FieldReader<bool, CALG_A>);
impl CALG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALG_A {
        match self.bits {
            false => CALG_A::LEFT_ALIGNED,
            true => CALG_A::CENTER_ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_ALIGNED`"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        **self == CALG_A::LEFT_ALIGNED
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED`"]
    #[inline(always)]
    pub fn is_center_aligned(&self) -> bool {
        **self == CALG_A::CENTER_ALIGNED
    }
}
impl core::ops::Deref for CALG_R {
    type Target = crate::FieldReader<bool, CALG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALG` writer - Channel Alignment"]
pub struct CALG_W<'a> {
    w: &'a mut W,
}
impl<'a> CALG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Left aligned"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut W {
        self.variant(CALG_A::LEFT_ALIGNED)
    }
    #[doc = "Center aligned"]
    #[inline(always)]
    pub fn center_aligned(self) -> &'a mut W {
        self.variant(CALG_A::CENTER_ALIGNED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Channel Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Waveform starts at low level"]
    LOW_POLARITY = 0,
    #[doc = "1: Waveform starts at high level"]
    HIGH_POLARITY = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::LOW_POLARITY,
            true => CPOL_A::HIGH_POLARITY,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_POLARITY`"]
    #[inline(always)]
    pub fn is_low_polarity(&self) -> bool {
        **self == CPOL_A::LOW_POLARITY
    }
    #[doc = "Checks if the value of the field is `HIGH_POLARITY`"]
    #[inline(always)]
    pub fn is_high_polarity(&self) -> bool {
        **self == CPOL_A::HIGH_POLARITY
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Waveform starts at low level"]
    #[inline(always)]
    pub fn low_polarity(self) -> &'a mut W {
        self.variant(CPOL_A::LOW_POLARITY)
    }
    #[doc = "Waveform starts at high level"]
    #[inline(always)]
    pub fn high_polarity(self) -> &'a mut W {
        self.variant(CPOL_A::HIGH_POLARITY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Counter Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CES_A {
    #[doc = "0: At the end of PWM period"]
    SINGLE_EVENT = 0,
    #[doc = "1: At half of PWM period AND at the end of PWM period"]
    DOUBLE_EVENT = 1,
}
impl From<CES_A> for bool {
    #[inline(always)]
    fn from(variant: CES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CES` reader - Counter Event Selection"]
pub struct CES_R(crate::FieldReader<bool, CES_A>);
impl CES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CES_A {
        match self.bits {
            false => CES_A::SINGLE_EVENT,
            true => CES_A::DOUBLE_EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EVENT`"]
    #[inline(always)]
    pub fn is_single_event(&self) -> bool {
        **self == CES_A::SINGLE_EVENT
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EVENT`"]
    #[inline(always)]
    pub fn is_double_event(&self) -> bool {
        **self == CES_A::DOUBLE_EVENT
    }
}
impl core::ops::Deref for CES_R {
    type Target = crate::FieldReader<bool, CES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CES` writer - Counter Event Selection"]
pub struct CES_W<'a> {
    w: &'a mut W,
}
impl<'a> CES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "At the end of PWM period"]
    #[inline(always)]
    pub fn single_event(self) -> &'a mut W {
        self.variant(CES_A::SINGLE_EVENT)
    }
    #[doc = "At half of PWM period AND at the end of PWM period"]
    #[inline(always)]
    pub fn double_event(self) -> &'a mut W {
        self.variant(CES_A::DOUBLE_EVENT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Update Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDS_A {
    #[doc = "0: At the next end of PWM period"]
    UPDATE_AT_PERIOD = 0,
    #[doc = "1: At the next end of Half PWM period"]
    UPDATE_AT_HALF_PERIOD = 1,
}
impl From<UPDS_A> for bool {
    #[inline(always)]
    fn from(variant: UPDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDS` reader - Update Selection"]
pub struct UPDS_R(crate::FieldReader<bool, UPDS_A>);
impl UPDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDS_A {
        match self.bits {
            false => UPDS_A::UPDATE_AT_PERIOD,
            true => UPDS_A::UPDATE_AT_HALF_PERIOD,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE_AT_PERIOD`"]
    #[inline(always)]
    pub fn is_update_at_period(&self) -> bool {
        **self == UPDS_A::UPDATE_AT_PERIOD
    }
    #[doc = "Checks if the value of the field is `UPDATE_AT_HALF_PERIOD`"]
    #[inline(always)]
    pub fn is_update_at_half_period(&self) -> bool {
        **self == UPDS_A::UPDATE_AT_HALF_PERIOD
    }
}
impl core::ops::Deref for UPDS_R {
    type Target = crate::FieldReader<bool, UPDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDS` writer - Update Selection"]
pub struct UPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "At the next end of PWM period"]
    #[inline(always)]
    pub fn update_at_period(self) -> &'a mut W {
        self.variant(UPDS_A::UPDATE_AT_PERIOD)
    }
    #[doc = "At the next end of Half PWM period"]
    #[inline(always)]
    pub fn update_at_half_period(self) -> &'a mut W {
        self.variant(UPDS_A::UPDATE_AT_HALF_PERIOD)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DPOLI` reader - Disabled Polarity Inverted"]
pub struct DPOLI_R(crate::FieldReader<bool, bool>);
impl DPOLI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPOLI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPOLI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPOLI` writer - Disabled Polarity Inverted"]
pub struct DPOLI_W<'a> {
    w: &'a mut W,
}
impl<'a> DPOLI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TCTS` reader - Timer Counter Trigger Selection"]
pub struct TCTS_R(crate::FieldReader<bool, bool>);
impl TCTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCTS` writer - Timer Counter Trigger Selection"]
pub struct TCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DTE` reader - Dead-Time Generator Enable"]
pub struct DTE_R(crate::FieldReader<bool, bool>);
impl DTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTE` writer - Dead-Time Generator Enable"]
pub struct DTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `DTHI` reader - Dead-Time PWMHx Output Inverted"]
pub struct DTHI_R(crate::FieldReader<bool, bool>);
impl DTHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTHI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTHI` writer - Dead-Time PWMHx Output Inverted"]
pub struct DTHI_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DTLI` reader - Dead-Time PWMLx Output Inverted"]
pub struct DTLI_R(crate::FieldReader<bool, bool>);
impl DTLI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTLI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTLI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTLI` writer - Dead-Time PWMLx Output Inverted"]
pub struct DTLI_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PPM` reader - Push-Pull Mode"]
pub struct PPM_R(crate::FieldReader<bool, bool>);
impl PPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPM` writer - Push-Pull Mode"]
pub struct PPM_W<'a> {
    w: &'a mut W,
}
impl<'a> PPM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CALG_R {
        CALG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&self) -> UPDS_R {
        UPDS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&self) -> DPOLI_R {
        DPOLI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&self) -> TCTS_R {
        TCTS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DTHI_R {
        DTHI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DTLI_R {
        DTLI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&self) -> PPM_R {
        PPM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&mut self) -> CPRE_W {
        CPRE_W { w: self }
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&mut self) -> CALG_W {
        CALG_W { w: self }
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W { w: self }
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&mut self) -> UPDS_W {
        UPDS_W { w: self }
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&mut self) -> DPOLI_W {
        DPOLI_W { w: self }
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&mut self) -> TCTS_W {
        TCTS_W { w: self }
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> DTE_W {
        DTE_W { w: self }
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&mut self) -> DTHI_W {
        DTHI_W { w: self }
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&mut self) -> DTLI_W {
        DTLI_W { w: self }
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&mut self) -> PPM_W {
        PPM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmr](index.html) module"]
pub struct PWM_CMR_SPEC;
impl crate::RegisterSpec for PWM_CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cmr::R](R) reader structure"]
impl crate::Readable for PWM_CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cmr::W](W) writer structure"]
impl crate::Writable for PWM_CMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CMR to value 0"]
impl crate::Resettable for PWM_CMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
