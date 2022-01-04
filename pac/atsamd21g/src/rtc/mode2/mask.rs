#[doc = "Register `MASK%s` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK%s` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alarm Mask Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Alarm Disabled"]
    OFF = 0,
    #[doc = "1: Match seconds only"]
    SS = 1,
    #[doc = "2: Match seconds and minutes only"]
    MMSS = 2,
    #[doc = "3: Match seconds, minutes, and hours only"]
    HHMMSS = 3,
    #[doc = "4: Match seconds, minutes, hours, and days only"]
    DDHHMMSS = 4,
    #[doc = "5: Match seconds, minutes, hours, days, and months only"]
    MMDDHHMMSS = 5,
    #[doc = "6: Match seconds, minutes, hours, days, months, and years"]
    YYMMDDHHMMSS = 6,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Alarm Mask Selection"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::OFF),
            1 => Some(SEL_A::SS),
            2 => Some(SEL_A::MMSS),
            3 => Some(SEL_A::HHMMSS),
            4 => Some(SEL_A::DDHHMMSS),
            5 => Some(SEL_A::MMDDHHMMSS),
            6 => Some(SEL_A::YYMMDDHHMMSS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `SS`"]
    #[inline(always)]
    pub fn is_ss(&self) -> bool {
        **self == SEL_A::SS
    }
    #[doc = "Checks if the value of the field is `MMSS`"]
    #[inline(always)]
    pub fn is_mmss(&self) -> bool {
        **self == SEL_A::MMSS
    }
    #[doc = "Checks if the value of the field is `HHMMSS`"]
    #[inline(always)]
    pub fn is_hhmmss(&self) -> bool {
        **self == SEL_A::HHMMSS
    }
    #[doc = "Checks if the value of the field is `DDHHMMSS`"]
    #[inline(always)]
    pub fn is_ddhhmmss(&self) -> bool {
        **self == SEL_A::DDHHMMSS
    }
    #[doc = "Checks if the value of the field is `MMDDHHMMSS`"]
    #[inline(always)]
    pub fn is_mmddhhmmss(&self) -> bool {
        **self == SEL_A::MMDDHHMMSS
    }
    #[doc = "Checks if the value of the field is `YYMMDDHHMMSS`"]
    #[inline(always)]
    pub fn is_yymmddhhmmss(&self) -> bool {
        **self == SEL_A::YYMMDDHHMMSS
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Alarm Mask Selection"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Alarm Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SEL_A::OFF)
    }
    #[doc = "Match seconds only"]
    #[inline(always)]
    pub fn ss(self) -> &'a mut W {
        self.variant(SEL_A::SS)
    }
    #[doc = "Match seconds and minutes only"]
    #[inline(always)]
    pub fn mmss(self) -> &'a mut W {
        self.variant(SEL_A::MMSS)
    }
    #[doc = "Match seconds, minutes, and hours only"]
    #[inline(always)]
    pub fn hhmmss(self) -> &'a mut W {
        self.variant(SEL_A::HHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, and days only"]
    #[inline(always)]
    pub fn ddhhmmss(self) -> &'a mut W {
        self.variant(SEL_A::DDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    #[inline(always)]
    pub fn mmddhhmmss(self) -> &'a mut W {
        self.variant(SEL_A::MMDDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    #[inline(always)]
    pub fn yymmddhhmmss(self) -> &'a mut W {
        self.variant(SEL_A::YYMMDDHHMMSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE2 Alarm n Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK%s to value 0"]
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
