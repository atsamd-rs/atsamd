#[doc = "Register `INPUTCTRL` reader"]
pub struct R(crate::R<INPUTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTCTRL` writer"]
pub struct W(crate::W<INPUTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTCTRL_SPEC>;
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
impl From<crate::W<INPUTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Mux Input Selection"]
pub type MUXPOS_R = crate::FieldReader<u8, MUXPOSSELECT_A>;
#[doc = "Positive Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOSSELECT_A {
    #[doc = "0: ADC AIN0 Pin"]
    AIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    AIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    AIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    AIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    AIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    AIN5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    AIN6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    AIN7 = 7,
    #[doc = "8: ADC AIN8 Pin"]
    AIN8 = 8,
    #[doc = "9: ADC AIN9 Pin"]
    AIN9 = 9,
    #[doc = "10: ADC AIN10 Pin"]
    AIN10 = 10,
    #[doc = "11: ADC AIN11 Pin"]
    AIN11 = 11,
    #[doc = "12: ADC AIN12 Pin"]
    AIN12 = 12,
    #[doc = "13: ADC AIN13 Pin"]
    AIN13 = 13,
    #[doc = "14: ADC AIN14 Pin"]
    AIN14 = 14,
    #[doc = "15: ADC AIN15 Pin"]
    AIN15 = 15,
    #[doc = "16: ADC AIN16 Pin"]
    AIN16 = 16,
    #[doc = "17: ADC AIN17 Pin"]
    AIN17 = 17,
    #[doc = "18: ADC AIN18 Pin"]
    AIN18 = 18,
    #[doc = "19: ADC AIN19 Pin"]
    AIN19 = 19,
    #[doc = "24: Temperature Sensor"]
    TEMP = 24,
    #[doc = "25: Bandgap Voltage"]
    BANDGAP = 25,
    #[doc = "26: 1/4 Scaled Core Supply"]
    SCALEDCOREVCC = 26,
    #[doc = "27: 1/4 Scaled I/O Supply"]
    SCALEDIOVCC = 27,
    #[doc = "29: 1/4 Scaled VBAT Supply"]
    SCALEDVBAT = 29,
    #[doc = "30: CTAT output"]
    CTAT = 30,
}
impl From<MUXPOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOSSELECT_A) -> Self {
        variant as _
    }
}
impl MUXPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXPOSSELECT_A> {
        match self.bits {
            0 => Some(MUXPOSSELECT_A::AIN0),
            1 => Some(MUXPOSSELECT_A::AIN1),
            2 => Some(MUXPOSSELECT_A::AIN2),
            3 => Some(MUXPOSSELECT_A::AIN3),
            4 => Some(MUXPOSSELECT_A::AIN4),
            5 => Some(MUXPOSSELECT_A::AIN5),
            6 => Some(MUXPOSSELECT_A::AIN6),
            7 => Some(MUXPOSSELECT_A::AIN7),
            8 => Some(MUXPOSSELECT_A::AIN8),
            9 => Some(MUXPOSSELECT_A::AIN9),
            10 => Some(MUXPOSSELECT_A::AIN10),
            11 => Some(MUXPOSSELECT_A::AIN11),
            12 => Some(MUXPOSSELECT_A::AIN12),
            13 => Some(MUXPOSSELECT_A::AIN13),
            14 => Some(MUXPOSSELECT_A::AIN14),
            15 => Some(MUXPOSSELECT_A::AIN15),
            16 => Some(MUXPOSSELECT_A::AIN16),
            17 => Some(MUXPOSSELECT_A::AIN17),
            18 => Some(MUXPOSSELECT_A::AIN18),
            19 => Some(MUXPOSSELECT_A::AIN19),
            24 => Some(MUXPOSSELECT_A::TEMP),
            25 => Some(MUXPOSSELECT_A::BANDGAP),
            26 => Some(MUXPOSSELECT_A::SCALEDCOREVCC),
            27 => Some(MUXPOSSELECT_A::SCALEDIOVCC),
            29 => Some(MUXPOSSELECT_A::SCALEDVBAT),
            30 => Some(MUXPOSSELECT_A::CTAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN7
    }
    #[doc = "Checks if the value of the field is `AIN8`"]
    #[inline(always)]
    pub fn is_ain8(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN8
    }
    #[doc = "Checks if the value of the field is `AIN9`"]
    #[inline(always)]
    pub fn is_ain9(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN9
    }
    #[doc = "Checks if the value of the field is `AIN10`"]
    #[inline(always)]
    pub fn is_ain10(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN10
    }
    #[doc = "Checks if the value of the field is `AIN11`"]
    #[inline(always)]
    pub fn is_ain11(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN11
    }
    #[doc = "Checks if the value of the field is `AIN12`"]
    #[inline(always)]
    pub fn is_ain12(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN12
    }
    #[doc = "Checks if the value of the field is `AIN13`"]
    #[inline(always)]
    pub fn is_ain13(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN13
    }
    #[doc = "Checks if the value of the field is `AIN14`"]
    #[inline(always)]
    pub fn is_ain14(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN14
    }
    #[doc = "Checks if the value of the field is `AIN15`"]
    #[inline(always)]
    pub fn is_ain15(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN15
    }
    #[doc = "Checks if the value of the field is `AIN16`"]
    #[inline(always)]
    pub fn is_ain16(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN16
    }
    #[doc = "Checks if the value of the field is `AIN17`"]
    #[inline(always)]
    pub fn is_ain17(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN17
    }
    #[doc = "Checks if the value of the field is `AIN18`"]
    #[inline(always)]
    pub fn is_ain18(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN18
    }
    #[doc = "Checks if the value of the field is `AIN19`"]
    #[inline(always)]
    pub fn is_ain19(&self) -> bool {
        *self == MUXPOSSELECT_A::AIN19
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == MUXPOSSELECT_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOSSELECT_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDCOREVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDIOVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDVBAT`"]
    #[inline(always)]
    pub fn is_scaledvbat(&self) -> bool {
        *self == MUXPOSSELECT_A::SCALEDVBAT
    }
    #[doc = "Checks if the value of the field is `CTAT`"]
    #[inline(always)]
    pub fn is_ctat(&self) -> bool {
        *self == MUXPOSSELECT_A::CTAT
    }
}
#[doc = "Field `MUXPOS` writer - Positive Mux Input Selection"]
pub type MUXPOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, INPUTCTRL_SPEC, u8, MUXPOSSELECT_A, 5, O>;
impl<'a, const O: u8> MUXPOS_W<'a, O> {
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn ain8(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn ain9(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn ain10(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn ain11(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn ain12(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn ain13(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn ain14(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn ain15(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn ain16(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn ain17(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn ain18(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn ain19(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::AIN19)
    }
    #[doc = "Temperature Sensor"]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::TEMP)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::BANDGAP)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::SCALEDIOVCC)
    }
    #[doc = "1/4 Scaled VBAT Supply"]
    #[inline(always)]
    pub fn scaledvbat(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::SCALEDVBAT)
    }
    #[doc = "CTAT output"]
    #[inline(always)]
    pub fn ctat(self) -> &'a mut W {
        self.variant(MUXPOSSELECT_A::CTAT)
    }
}
#[doc = "Field `MUXNEG` reader - Negative Mux Input Selection"]
pub type MUXNEG_R = crate::FieldReader<u8, MUXNEGSELECT_A>;
#[doc = "Negative Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEGSELECT_A {
    #[doc = "0: ADC AIN0 Pin"]
    AIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    AIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    AIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    AIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    AIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    AIN5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    AIN6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    AIN7 = 7,
}
impl From<MUXNEGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEGSELECT_A) -> Self {
        variant as _
    }
}
impl MUXNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXNEGSELECT_A> {
        match self.bits {
            0 => Some(MUXNEGSELECT_A::AIN0),
            1 => Some(MUXNEGSELECT_A::AIN1),
            2 => Some(MUXNEGSELECT_A::AIN2),
            3 => Some(MUXNEGSELECT_A::AIN3),
            4 => Some(MUXNEGSELECT_A::AIN4),
            5 => Some(MUXNEGSELECT_A::AIN5),
            6 => Some(MUXNEGSELECT_A::AIN6),
            7 => Some(MUXNEGSELECT_A::AIN7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXNEGSELECT_A::AIN7
    }
}
#[doc = "Field `MUXNEG` writer - Negative Mux Input Selection"]
pub type MUXNEG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, INPUTCTRL_SPEC, u8, MUXNEGSELECT_A, 5, O>;
impl<'a, const O: u8> MUXNEG_W<'a, O> {
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXNEGSELECT_A::AIN7)
    }
}
impl R {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<0> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<8> {
        MUXNEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputctrl](index.html) module"]
pub struct INPUTCTRL_SPEC;
impl crate::RegisterSpec for INPUTCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [inputctrl::R](R) reader structure"]
impl crate::Readable for INPUTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputctrl::W](W) writer structure"]
impl crate::Writable for INPUTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for INPUTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
