#[doc = "Register `LUTCTRL[%s]` reader"]
pub struct R(crate::R<LUTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUTCTRL[%s]` writer"]
pub struct W(crate::W<LUTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUTCTRL_SPEC>;
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
impl From<crate::W<LUTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LUT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: LUT block is disabled"]
    DISABLE = 0,
    #[doc = "1: LUT block is enabled"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - LUT Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - LUT Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LUT block is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "LUT block is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTSEL_A {
    #[doc = "0: Filter disabled"]
    DISABLE = 0,
    #[doc = "1: Synchronizer enabled"]
    SYNCH = 1,
    #[doc = "2: Filter enabled"]
    FILTER = 2,
}
impl From<FILTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTSEL` reader - Filter Selection"]
pub struct FILTSEL_R(crate::FieldReader<u8, FILTSEL_A>);
impl FILTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTSEL_A> {
        match self.bits {
            0 => Some(FILTSEL_A::DISABLE),
            1 => Some(FILTSEL_A::SYNCH),
            2 => Some(FILTSEL_A::FILTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FILTSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SYNCH`"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        **self == FILTSEL_A::SYNCH
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        **self == FILTSEL_A::FILTER
    }
}
impl core::ops::Deref for FILTSEL_R {
    type Target = crate::FieldReader<u8, FILTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTSEL` writer - Filter Selection"]
pub struct FILTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILTSEL_A::DISABLE)
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn synch(self) -> &'a mut W {
        self.variant(FILTSEL_A::SYNCH)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(FILTSEL_A::FILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGESEL_A {
    #[doc = "0: Edge detector is disabled"]
    DISABLE = 0,
    #[doc = "1: Edge detector is enabled"]
    ENABLE = 1,
}
impl From<EDGESEL_A> for bool {
    #[inline(always)]
    fn from(variant: EDGESEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGESEL` reader - Edge Selection"]
pub struct EDGESEL_R(crate::FieldReader<bool, EDGESEL_A>);
impl EDGESEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDGESEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGESEL_A {
        match self.bits {
            false => EDGESEL_A::DISABLE,
            true => EDGESEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == EDGESEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == EDGESEL_A::ENABLE
    }
}
impl core::ops::Deref for EDGESEL_R {
    type Target = crate::FieldReader<bool, EDGESEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGESEL` writer - Edge Selection"]
pub struct EDGESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGESEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge detector is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDGESEL_A::DISABLE)
    }
    #[doc = "Edge detector is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDGESEL_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Input Selection 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INSEL0_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source"]
    EVENT = 3,
    #[doc = "4: I/O pin input source"]
    IO = 4,
    #[doc = "5: AC input source"]
    AC = 5,
    #[doc = "6: TC input source"]
    TC = 6,
    #[doc = "7: Alternate TC input source"]
    ALTTC = 7,
    #[doc = "8: TCC input source"]
    TCC = 8,
    #[doc = "9: SERCOM input source"]
    SERCOM = 9,
}
impl From<INSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INSEL0` reader - Input Selection 0"]
pub struct INSEL0_R(crate::FieldReader<u8, INSEL0_A>);
impl INSEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSEL0_A> {
        match self.bits {
            0 => Some(INSEL0_A::MASK),
            1 => Some(INSEL0_A::FEEDBACK),
            2 => Some(INSEL0_A::LINK),
            3 => Some(INSEL0_A::EVENT),
            4 => Some(INSEL0_A::IO),
            5 => Some(INSEL0_A::AC),
            6 => Some(INSEL0_A::TC),
            7 => Some(INSEL0_A::ALTTC),
            8 => Some(INSEL0_A::TCC),
            9 => Some(INSEL0_A::SERCOM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == INSEL0_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        **self == INSEL0_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        **self == INSEL0_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == INSEL0_A::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == INSEL0_A::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        **self == INSEL0_A::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        **self == INSEL0_A::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        **self == INSEL0_A::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        **self == INSEL0_A::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        **self == INSEL0_A::SERCOM
    }
}
impl core::ops::Deref for INSEL0_R {
    type Target = crate::FieldReader<u8, INSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL0` writer - Input Selection 0"]
pub struct INSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL0_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL0_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL0_A::LINK)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL0_A::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL0_A::IO)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL0_A::AC)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL0_A::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL0_A::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL0_A::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL0_A::SERCOM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Input Selection 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INSEL1_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source"]
    EVENT = 3,
    #[doc = "4: I/O pin input source"]
    IO = 4,
    #[doc = "5: AC input source"]
    AC = 5,
    #[doc = "6: TC input source"]
    TC = 6,
    #[doc = "7: Alternate TC input source"]
    ALTTC = 7,
    #[doc = "8: TCC input source"]
    TCC = 8,
    #[doc = "9: SERCOM input source"]
    SERCOM = 9,
}
impl From<INSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INSEL1` reader - Input Selection 1"]
pub struct INSEL1_R(crate::FieldReader<u8, INSEL1_A>);
impl INSEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSEL1_A> {
        match self.bits {
            0 => Some(INSEL1_A::MASK),
            1 => Some(INSEL1_A::FEEDBACK),
            2 => Some(INSEL1_A::LINK),
            3 => Some(INSEL1_A::EVENT),
            4 => Some(INSEL1_A::IO),
            5 => Some(INSEL1_A::AC),
            6 => Some(INSEL1_A::TC),
            7 => Some(INSEL1_A::ALTTC),
            8 => Some(INSEL1_A::TCC),
            9 => Some(INSEL1_A::SERCOM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == INSEL1_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        **self == INSEL1_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        **self == INSEL1_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == INSEL1_A::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == INSEL1_A::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        **self == INSEL1_A::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        **self == INSEL1_A::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        **self == INSEL1_A::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        **self == INSEL1_A::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        **self == INSEL1_A::SERCOM
    }
}
impl core::ops::Deref for INSEL1_R {
    type Target = crate::FieldReader<u8, INSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL1` writer - Input Selection 1"]
pub struct INSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL1_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL1_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL1_A::LINK)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL1_A::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL1_A::IO)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL1_A::AC)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL1_A::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL1_A::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL1_A::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL1_A::SERCOM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Input Selection 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INSEL2_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source"]
    EVENT = 3,
    #[doc = "4: I/O pin input source"]
    IO = 4,
    #[doc = "5: AC input source"]
    AC = 5,
    #[doc = "6: TC input source"]
    TC = 6,
    #[doc = "7: Alternate TC input source"]
    ALTTC = 7,
    #[doc = "8: TCC input source"]
    TCC = 8,
    #[doc = "9: SERCOM input source"]
    SERCOM = 9,
}
impl From<INSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INSEL2` reader - Input Selection 2"]
pub struct INSEL2_R(crate::FieldReader<u8, INSEL2_A>);
impl INSEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INSEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSEL2_A> {
        match self.bits {
            0 => Some(INSEL2_A::MASK),
            1 => Some(INSEL2_A::FEEDBACK),
            2 => Some(INSEL2_A::LINK),
            3 => Some(INSEL2_A::EVENT),
            4 => Some(INSEL2_A::IO),
            5 => Some(INSEL2_A::AC),
            6 => Some(INSEL2_A::TC),
            7 => Some(INSEL2_A::ALTTC),
            8 => Some(INSEL2_A::TCC),
            9 => Some(INSEL2_A::SERCOM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == INSEL2_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        **self == INSEL2_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        **self == INSEL2_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == INSEL2_A::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == INSEL2_A::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        **self == INSEL2_A::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        **self == INSEL2_A::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        **self == INSEL2_A::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        **self == INSEL2_A::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        **self == INSEL2_A::SERCOM
    }
}
impl core::ops::Deref for INSEL2_R {
    type Target = crate::FieldReader<u8, INSEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL2` writer - Input Selection 2"]
pub struct INSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL2_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL2_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL2_A::LINK)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL2_A::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL2_A::IO)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL2_A::AC)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL2_A::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL2_A::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL2_A::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL2_A::SERCOM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Inverted Event Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVEI_A {
    #[doc = "0: Incoming event is not inverted"]
    DISABLE = 0,
    #[doc = "1: Incoming event is inverted"]
    ENABLE = 1,
}
impl From<INVEI_A> for bool {
    #[inline(always)]
    fn from(variant: INVEI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVEI` reader - Inverted Event Input Enable"]
pub struct INVEI_R(crate::FieldReader<bool, INVEI_A>);
impl INVEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVEI_A {
        match self.bits {
            false => INVEI_A::DISABLE,
            true => INVEI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == INVEI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == INVEI_A::ENABLE
    }
}
impl core::ops::Deref for INVEI_R {
    type Target = crate::FieldReader<bool, INVEI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEI` writer - Inverted Event Input Enable"]
pub struct INVEI_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVEI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Incoming event is not inverted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INVEI_A::DISABLE)
    }
    #[doc = "Incoming event is inverted"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INVEI_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "LUT Event Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUTEI_A {
    #[doc = "0: LUT incoming event is disabled"]
    DISABLE = 0,
    #[doc = "1: LUT incoming event is enabled"]
    ENABLE = 1,
}
impl From<LUTEI_A> for bool {
    #[inline(always)]
    fn from(variant: LUTEI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUTEI` reader - LUT Event Input Enable"]
pub struct LUTEI_R(crate::FieldReader<bool, LUTEI_A>);
impl LUTEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LUTEI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUTEI_A {
        match self.bits {
            false => LUTEI_A::DISABLE,
            true => LUTEI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LUTEI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LUTEI_A::ENABLE
    }
}
impl core::ops::Deref for LUTEI_R {
    type Target = crate::FieldReader<bool, LUTEI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUTEI` writer - LUT Event Input Enable"]
pub struct LUTEI_W<'a> {
    w: &'a mut W,
}
impl<'a> LUTEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LUTEI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LUT incoming event is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LUTEI_A::DISABLE)
    }
    #[doc = "LUT incoming event is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LUTEI_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "LUT Event Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUTEO_A {
    #[doc = "0: LUT event output is disabled"]
    DISABLE = 0,
    #[doc = "1: LUT event output is enabled"]
    ENABLE = 1,
}
impl From<LUTEO_A> for bool {
    #[inline(always)]
    fn from(variant: LUTEO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUTEO` reader - LUT Event Output Enable"]
pub struct LUTEO_R(crate::FieldReader<bool, LUTEO_A>);
impl LUTEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LUTEO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUTEO_A {
        match self.bits {
            false => LUTEO_A::DISABLE,
            true => LUTEO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LUTEO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LUTEO_A::ENABLE
    }
}
impl core::ops::Deref for LUTEO_R {
    type Target = crate::FieldReader<bool, LUTEO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUTEO` writer - LUT Event Output Enable"]
pub struct LUTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> LUTEO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LUTEO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LUT event output is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LUTEO_A::DISABLE)
    }
    #[doc = "LUT event output is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LUTEO_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TRUTH` reader - Truth Value"]
pub struct TRUTH_R(crate::FieldReader<u8, u8>);
impl TRUTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRUTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRUTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRUTH` writer - Truth Value"]
pub struct TRUTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    pub fn insel0(&self) -> INSEL0_R {
        INSEL0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    pub fn insel1(&self) -> INSEL1_R {
        INSEL1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    pub fn insel2(&self) -> INSEL2_R {
        INSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn invei(&self) -> INVEI_R {
        INVEI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline(always)]
    pub fn lutei(&self) -> LUTEI_R {
        LUTEI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline(always)]
    pub fn luteo(&self) -> LUTEO_R {
        LUTEO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    pub fn truth(&self) -> TRUTH_R {
        TRUTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&mut self) -> FILTSEL_W {
        FILTSEL_W { w: self }
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    pub fn edgesel(&mut self) -> EDGESEL_W {
        EDGESEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    pub fn insel0(&mut self) -> INSEL0_W {
        INSEL0_W { w: self }
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    pub fn insel1(&mut self) -> INSEL1_W {
        INSEL1_W { w: self }
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    pub fn insel2(&mut self) -> INSEL2_W {
        INSEL2_W { w: self }
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn invei(&mut self) -> INVEI_W {
        INVEI_W { w: self }
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline(always)]
    pub fn lutei(&mut self) -> LUTEI_W {
        LUTEI_W { w: self }
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline(always)]
    pub fn luteo(&mut self) -> LUTEO_W {
        LUTEO_W { w: self }
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    pub fn truth(&mut self) -> TRUTH_W {
        TRUTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT Control x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutctrl](index.html) module"]
pub struct LUTCTRL_SPEC;
impl crate::RegisterSpec for LUTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lutctrl::R](R) reader structure"]
impl crate::Readable for LUTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lutctrl::W](W) writer structure"]
impl crate::Writable for LUTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUTCTRL[%s]
to value 0"]
impl crate::Resettable for LUTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
