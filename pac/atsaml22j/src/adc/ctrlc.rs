#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIFFMODE` reader - Differential Mode"]
pub type DIFFMODE_R = crate::BitReader<bool>;
#[doc = "Field `DIFFMODE` writer - Differential Mode"]
pub type DIFFMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
pub type LEFTADJ_R = crate::BitReader<bool>;
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
pub type LEFTADJ_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub type FREERUN_R = crate::BitReader<bool>;
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub type FREERUN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `CORREN` reader - Digital Correction Logic Enable"]
pub type CORREN_R = crate::BitReader<bool>;
#[doc = "Field `CORREN` writer - Digital Correction Logic Enable"]
pub type CORREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `RESSEL` reader - Conversion Result Resolution"]
pub type RESSEL_R = crate::FieldReader<u8, RESSELSELECT_A>;
#[doc = "Conversion Result Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESSELSELECT_A {
    #[doc = "0: 12-bit result"]
    _12BIT = 0,
    #[doc = "1: For averaging mode output"]
    _16BIT = 1,
    #[doc = "2: 10-bit result"]
    _10BIT = 2,
    #[doc = "3: 8-bit result"]
    _8BIT = 3,
}
impl From<RESSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSELSELECT_A) -> Self {
        variant as _
    }
}
impl RESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSELSELECT_A {
        match self.bits {
            0 => RESSELSELECT_A::_12BIT,
            1 => RESSELSELECT_A::_16BIT,
            2 => RESSELSELECT_A::_10BIT,
            3 => RESSELSELECT_A::_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RESSELSELECT_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == RESSELSELECT_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == RESSELSELECT_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RESSELSELECT_A::_8BIT
    }
}
#[doc = "Field `RESSEL` writer - Conversion Result Resolution"]
pub type RESSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CTRLC_SPEC, u8, RESSELSELECT_A, 2, O>;
impl<'a, const O: u8> RESSEL_W<'a, O> {
    #[doc = "12-bit result"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RESSELSELECT_A::_12BIT)
    }
    #[doc = "For averaging mode output"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(RESSELSELECT_A::_16BIT)
    }
    #[doc = "10-bit result"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(RESSELSELECT_A::_10BIT)
    }
    #[doc = "8-bit result"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESSELSELECT_A::_8BIT)
    }
}
#[doc = "Field `R2R` reader - Rail-to-Rail mode enable"]
pub type R2R_R = crate::BitReader<bool>;
#[doc = "Field `R2R` writer - Rail-to-Rail mode enable"]
pub type R2R_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub type WINMODE_R = crate::FieldReader<u8, WINMODESELECT_A>;
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINMODESELECT_A {
    #[doc = "0: No window mode (default)"]
    DISABLE = 0,
    #[doc = "1: RESULT > WINLT"]
    MODE1 = 1,
    #[doc = "2: RESULT < WINUT"]
    MODE2 = 2,
    #[doc = "3: WINLT < RESULT < WINUT"]
    MODE3 = 3,
    #[doc = "4: !(WINLT < RESULT < WINUT)"]
    MODE4 = 4,
}
impl From<WINMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINMODESELECT_A) -> Self {
        variant as _
    }
}
impl WINMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINMODESELECT_A> {
        match self.bits {
            0 => Some(WINMODESELECT_A::DISABLE),
            1 => Some(WINMODESELECT_A::MODE1),
            2 => Some(WINMODESELECT_A::MODE2),
            3 => Some(WINMODESELECT_A::MODE3),
            4 => Some(WINMODESELECT_A::MODE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WINMODESELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == WINMODESELECT_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == WINMODESELECT_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == WINMODESELECT_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == WINMODESELECT_A::MODE4
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub type WINMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CTRLC_SPEC, u8, WINMODESELECT_A, 3, O>;
impl<'a, const O: u8> WINMODE_W<'a, O> {
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WINMODESELECT_A::DISABLE)
    }
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WINMODESELECT_A::MODE1)
    }
    #[doc = "RESULT < WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WINMODESELECT_A::MODE2)
    }
    #[doc = "WINLT < RESULT < WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WINMODESELECT_A::MODE3)
    }
    #[doc = "!(WINLT < RESULT < WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(WINMODESELECT_A::MODE4)
    }
}
#[doc = "Field `DUALSEL` reader - Dual Mode Trigger Selection"]
pub type DUALSEL_R = crate::FieldReader<u8, DUALSELSELECT_A>;
#[doc = "Dual Mode Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUALSELSELECT_A {
    #[doc = "0: Start event or software trigger will start a conversion on both ADCs"]
    BOTH = 0,
    #[doc = "1: START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    INTERLEAVE = 1,
}
impl From<DUALSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DUALSELSELECT_A) -> Self {
        variant as _
    }
}
impl DUALSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DUALSELSELECT_A> {
        match self.bits {
            0 => Some(DUALSELSELECT_A::BOTH),
            1 => Some(DUALSELSELECT_A::INTERLEAVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == DUALSELSELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `INTERLEAVE`"]
    #[inline(always)]
    pub fn is_interleave(&self) -> bool {
        *self == DUALSELSELECT_A::INTERLEAVE
    }
}
#[doc = "Field `DUALSEL` writer - Dual Mode Trigger Selection"]
pub type DUALSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CTRLC_SPEC, u8, DUALSELSELECT_A, 2, O>;
impl<'a, const O: u8> DUALSEL_W<'a, O> {
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(DUALSELSELECT_A::BOTH)
    }
    #[doc = "START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn interleave(self) -> &'a mut W {
        self.variant(DUALSELSELECT_A::INTERLEAVE)
    }
}
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&self) -> DIFFMODE_R {
        DIFFMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&self) -> CORREN_R {
        CORREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Rail-to-Rail mode enable"]
    #[inline(always)]
    pub fn r2r(&self) -> R2R_R {
        R2R_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Dual Mode Trigger Selection"]
    #[inline(always)]
    pub fn dualsel(&self) -> DUALSEL_R {
        DUALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diffmode(&mut self) -> DIFFMODE_W<0> {
        DIFFMODE_W::new(self)
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<1> {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<2> {
        FREERUN_W::new(self)
    }
    #[doc = "Bit 3 - Digital Correction Logic Enable"]
    #[inline(always)]
    #[must_use]
    pub fn corren(&mut self) -> CORREN_W<3> {
        CORREN_W::new(self)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> RESSEL_W<4> {
        RESSEL_W::new(self)
    }
    #[doc = "Bit 7 - Rail-to-Rail mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn r2r(&mut self) -> R2R_W<7> {
        R2R_W::new(self)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<8> {
        WINMODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Dual Mode Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dualsel(&mut self) -> DUALSEL_W<12> {
        DUALSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
