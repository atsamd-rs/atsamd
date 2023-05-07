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
#[doc = "Field `DIFF` reader - Differential mode enable"]
pub type DIFF_R = crate::BitReader<bool>;
#[doc = "Field `DIFF` writer - Differential mode enable"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `REFSEL` reader - Reference Selection for DAC0/1"]
pub type REFSEL_R = crate::FieldReader<u8, REFSELSELECT_A>;
#[doc = "Reference Selection for DAC0/1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSELSELECT_A {
    #[doc = "0: External reference unbuffered"]
    VREFPU = 0,
    #[doc = "1: Analog supply"]
    VDDANA = 1,
    #[doc = "2: External reference buffered"]
    VREFPB = 2,
    #[doc = "3: Internal bandgap reference"]
    INTREF = 3,
}
impl From<REFSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSELSELECT_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSELSELECT_A {
        match self.bits {
            0 => REFSELSELECT_A::VREFPU,
            1 => REFSELSELECT_A::VDDANA,
            2 => REFSELSELECT_A::VREFPB,
            3 => REFSELSELECT_A::INTREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFPU`"]
    #[inline(always)]
    pub fn is_vrefpu(&self) -> bool {
        *self == REFSELSELECT_A::VREFPU
    }
    #[doc = "Checks if the value of the field is `VDDANA`"]
    #[inline(always)]
    pub fn is_vddana(&self) -> bool {
        *self == REFSELSELECT_A::VDDANA
    }
    #[doc = "Checks if the value of the field is `VREFPB`"]
    #[inline(always)]
    pub fn is_vrefpb(&self) -> bool {
        *self == REFSELSELECT_A::VREFPB
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == REFSELSELECT_A::INTREF
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection for DAC0/1"]
pub type REFSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLB_SPEC, u8, REFSELSELECT_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "External reference unbuffered"]
    #[inline(always)]
    pub fn vrefpu(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::VREFPU)
    }
    #[doc = "Analog supply"]
    #[inline(always)]
    pub fn vddana(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::VDDANA)
    }
    #[doc = "External reference buffered"]
    #[inline(always)]
    pub fn vrefpb(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::VREFPB)
    }
    #[doc = "Internal bandgap reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::INTREF)
    }
}
impl R {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<0> {
        DIFF_W::new(self)
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<1> {
        REFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
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
#[doc = "`reset()` method sets CTRLB to value 0x02"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
