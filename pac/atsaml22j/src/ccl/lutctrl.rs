#[doc = "Register `LUTCTRL%s` reader"]
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
#[doc = "Register `LUTCTRL%s` writer"]
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
#[doc = "Field `ENABLE` reader - LUT Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - LUT Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCTRL_SPEC, bool, O>;
#[doc = "Field `FILTSEL` reader - Filter Selection"]
pub type FILTSEL_R = crate::FieldReader<u8, FILTSELSELECT_A>;
#[doc = "Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTSELSELECT_A {
    #[doc = "0: Filter disabled"]
    DISABLE = 0,
    #[doc = "1: Synchronizer enabled"]
    SYNCH = 1,
    #[doc = "2: Filter enabled"]
    FILTER = 2,
}
impl From<FILTSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSELSELECT_A) -> Self {
        variant as _
    }
}
impl FILTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTSELSELECT_A> {
        match self.bits {
            0 => Some(FILTSELSELECT_A::DISABLE),
            1 => Some(FILTSELSELECT_A::SYNCH),
            2 => Some(FILTSELSELECT_A::FILTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTSELSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SYNCH`"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        *self == FILTSELSELECT_A::SYNCH
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == FILTSELSELECT_A::FILTER
    }
}
#[doc = "Field `FILTSEL` writer - Filter Selection"]
pub type FILTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LUTCTRL_SPEC, u8, FILTSELSELECT_A, 2, O>;
impl<'a, const O: u8> FILTSEL_W<'a, O> {
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILTSELSELECT_A::DISABLE)
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn synch(self) -> &'a mut W {
        self.variant(FILTSELSELECT_A::SYNCH)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(FILTSELSELECT_A::FILTER)
    }
}
#[doc = "Field `EDGESEL` reader - Edge Selection"]
pub type EDGESEL_R = crate::BitReader<bool>;
#[doc = "Field `EDGESEL` writer - Edge Selection"]
pub type EDGESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCTRL_SPEC, bool, O>;
#[doc = "Field `INSEL0` reader - Input Selection 0"]
pub type INSEL0_R = crate::FieldReader<u8, INSEL0SELECT_A>;
#[doc = "Input Selection 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSEL0SELECT_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event in put source"]
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
    #[doc = "9: SERCOM inout source"]
    SERCOM = 9,
}
impl From<INSEL0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL0SELECT_A) -> Self {
        variant as _
    }
}
impl INSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSEL0SELECT_A> {
        match self.bits {
            0 => Some(INSEL0SELECT_A::MASK),
            1 => Some(INSEL0SELECT_A::FEEDBACK),
            2 => Some(INSEL0SELECT_A::LINK),
            3 => Some(INSEL0SELECT_A::EVENT),
            4 => Some(INSEL0SELECT_A::IO),
            5 => Some(INSEL0SELECT_A::AC),
            6 => Some(INSEL0SELECT_A::TC),
            7 => Some(INSEL0SELECT_A::ALTTC),
            8 => Some(INSEL0SELECT_A::TCC),
            9 => Some(INSEL0SELECT_A::SERCOM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL0SELECT_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL0SELECT_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL0SELECT_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == INSEL0SELECT_A::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL0SELECT_A::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == INSEL0SELECT_A::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == INSEL0SELECT_A::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == INSEL0SELECT_A::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == INSEL0SELECT_A::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == INSEL0SELECT_A::SERCOM
    }
}
#[doc = "Field `INSEL0` writer - Input Selection 0"]
pub type INSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LUTCTRL_SPEC, u8, INSEL0SELECT_A, 4, O>;
impl<'a, const O: u8> INSEL0_W<'a, O> {
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::LINK)
    }
    #[doc = "Event in put source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::IO)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::AC)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::TCC)
    }
    #[doc = "SERCOM inout source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL0SELECT_A::SERCOM)
    }
}
#[doc = "Field `INSEL1` reader - Input Selection 1"]
pub type INSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSEL1` writer - Input Selection 1"]
pub type INSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `INSEL2` reader - Input Selection 2"]
pub type INSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSEL2` writer - Input Selection 2"]
pub type INSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `INVEI` reader - Input Event Invert"]
pub type INVEI_R = crate::BitReader<bool>;
#[doc = "Field `INVEI` writer - Input Event Invert"]
pub type INVEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCTRL_SPEC, bool, O>;
#[doc = "Field `LUTEI` reader - Event Input Enable"]
pub type LUTEI_R = crate::BitReader<bool>;
#[doc = "Field `LUTEI` writer - Event Input Enable"]
pub type LUTEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCTRL_SPEC, bool, O>;
#[doc = "Field `LUTEO` reader - Event Output Enable"]
pub type LUTEO_R = crate::BitReader<bool>;
#[doc = "Field `LUTEO` writer - Event Output Enable"]
pub type LUTEO_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCTRL_SPEC, bool, O>;
#[doc = "Field `TRUTH` reader - Truth Value"]
pub type TRUTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRUTH` writer - Truth Value"]
pub type TRUTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 20 - Input Event Invert"]
    #[inline(always)]
    pub fn invei(&self) -> INVEI_R {
        INVEI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event Input Enable"]
    #[inline(always)]
    pub fn lutei(&self) -> LUTEI_R {
        LUTEI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event Output Enable"]
    #[inline(always)]
    pub fn luteo(&self) -> LUTEO_R {
        LUTEO_R::new(((self.bits >> 22) & 1) != 0)
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
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filtsel(&mut self) -> FILTSEL_W<4> {
        FILTSEL_W::new(self)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn edgesel(&mut self) -> EDGESEL_W<7> {
        EDGESEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    #[must_use]
    pub fn insel0(&mut self) -> INSEL0_W<8> {
        INSEL0_W::new(self)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    #[must_use]
    pub fn insel1(&mut self) -> INSEL1_W<12> {
        INSEL1_W::new(self)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    #[must_use]
    pub fn insel2(&mut self) -> INSEL2_W<16> {
        INSEL2_W::new(self)
    }
    #[doc = "Bit 20 - Input Event Invert"]
    #[inline(always)]
    #[must_use]
    pub fn invei(&mut self) -> INVEI_W<20> {
        INVEI_W::new(self)
    }
    #[doc = "Bit 21 - Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lutei(&mut self) -> LUTEI_W<21> {
        LUTEI_W::new(self)
    }
    #[doc = "Bit 22 - Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn luteo(&mut self) -> LUTEO_W<22> {
        LUTEO_W::new(self)
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    #[must_use]
    pub fn truth(&mut self) -> TRUTH_W<24> {
        TRUTH_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUTCTRL%s to value 0"]
impl crate::Resettable for LUTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
