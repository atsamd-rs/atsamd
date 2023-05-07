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
#[doc = "Field `DIFFMODE` reader - Differential Mode"]
pub type DIFFMODE_R = crate::BitReader<bool>;
#[doc = "Field `DIFFMODE` writer - Differential Mode"]
pub type DIFFMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
pub type LEFTADJ_R = crate::BitReader<bool>;
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
pub type LEFTADJ_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub type FREERUN_R = crate::BitReader<bool>;
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub type FREERUN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `CORREN` reader - Digital Correction Logic Enabled"]
pub type CORREN_R = crate::BitReader<bool>;
#[doc = "Field `CORREN` writer - Digital Correction Logic Enabled"]
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
#[doc = "Field `PRESCALER` reader - Prescaler Configuration"]
pub type PRESCALER_R = crate::FieldReader<u8, PRESCALERSELECT_A>;
#[doc = "Prescaler Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALERSELECT_A {
    #[doc = "0: Peripheral clock divided by 4"]
    DIV4 = 0,
    #[doc = "1: Peripheral clock divided by 8"]
    DIV8 = 1,
    #[doc = "2: Peripheral clock divided by 16"]
    DIV16 = 2,
    #[doc = "3: Peripheral clock divided by 32"]
    DIV32 = 3,
    #[doc = "4: Peripheral clock divided by 64"]
    DIV64 = 4,
    #[doc = "5: Peripheral clock divided by 128"]
    DIV128 = 5,
    #[doc = "6: Peripheral clock divided by 256"]
    DIV256 = 6,
    #[doc = "7: Peripheral clock divided by 512"]
    DIV512 = 7,
}
impl From<PRESCALERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALERSELECT_A) -> Self {
        variant as _
    }
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALERSELECT_A {
        match self.bits {
            0 => PRESCALERSELECT_A::DIV4,
            1 => PRESCALERSELECT_A::DIV8,
            2 => PRESCALERSELECT_A::DIV16,
            3 => PRESCALERSELECT_A::DIV32,
            4 => PRESCALERSELECT_A::DIV64,
            5 => PRESCALERSELECT_A::DIV128,
            6 => PRESCALERSELECT_A::DIV256,
            7 => PRESCALERSELECT_A::DIV512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCALERSELECT_A::DIV512
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Configuration"]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CTRLB_SPEC, u8, PRESCALERSELECT_A, 3, O>;
impl<'a, const O: u8> PRESCALER_W<'a, O> {
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV4)
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV8)
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV16)
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV32)
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV64)
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV128)
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV256)
    }
    #[doc = "Peripheral clock divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCALERSELECT_A::DIV512)
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
    #[doc = "Bit 3 - Digital Correction Logic Enabled"]
    #[inline(always)]
    pub fn corren(&self) -> CORREN_R {
        CORREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 7) as u8)
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
    #[doc = "Bit 3 - Digital Correction Logic Enabled"]
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
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<8> {
        PRESCALER_W::new(self)
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
