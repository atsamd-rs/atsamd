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
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
pub type LEFTADJ_R = crate::BitReader<bool>;
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
pub type LEFTADJ_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub type FREERUN_R = crate::BitReader<bool>;
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub type FREERUN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `CORREN` reader - Digital Correction Logic Enable"]
pub type CORREN_R = crate::BitReader<bool>;
#[doc = "Field `CORREN` writer - Digital Correction Logic Enable"]
pub type CORREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
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
    crate::FieldWriterSafe<'a, u16, CTRLB_SPEC, u8, RESSELSELECT_A, 2, O>;
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
    crate::FieldWriter<'a, u16, CTRLB_SPEC, u8, WINMODESELECT_A, 3, O>;
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
#[doc = "Field `WINSS` reader - Window Single Sample"]
pub type WINSS_R = crate::BitReader<bool>;
#[doc = "Field `WINSS` writer - Window Single Sample"]
pub type WINSS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&self) -> CORREN_R {
        CORREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    pub fn winss(&self) -> WINSS_R {
        WINSS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<0> {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<1> {
        FREERUN_W::new(self)
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    #[must_use]
    pub fn corren(&mut self) -> CORREN_W<2> {
        CORREN_W::new(self)
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> RESSEL_W<3> {
        RESSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<8> {
        WINMODE_W::new(self)
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    #[must_use]
    pub fn winss(&mut self) -> WINSS_W<11> {
        WINSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
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
