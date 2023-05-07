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
#[doc = "Field `EOEN` reader - External Output Enable"]
pub type EOEN_R = crate::BitReader<bool>;
#[doc = "Field `EOEN` writer - External Output Enable"]
pub type EOEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `IOEN` reader - Internal Output Enable"]
pub type IOEN_R = crate::BitReader<bool>;
#[doc = "Field `IOEN` writer - Internal Output Enable"]
pub type IOEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `LEFTADJ` reader - Left Adjusted Data"]
pub type LEFTADJ_R = crate::BitReader<bool>;
#[doc = "Field `LEFTADJ` writer - Left Adjusted Data"]
pub type LEFTADJ_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `VPD` reader - Voltage Pump Disable"]
pub type VPD_R = crate::BitReader<bool>;
#[doc = "Field `VPD` writer - Voltage Pump Disable"]
pub type VPD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `BDWP` reader - Bypass DATABUF Write Protection"]
pub type BDWP_R = crate::BitReader<bool>;
#[doc = "Field `BDWP` writer - Bypass DATABUF Write Protection"]
pub type BDWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSELSELECT_A>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSELSELECT_A {
    #[doc = "0: Internal 1.0V reference"]
    INT1V = 0,
    #[doc = "1: AVCC"]
    AVCC = 1,
    #[doc = "2: External reference"]
    VREFP = 2,
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
    pub fn variant(&self) -> Option<REFSELSELECT_A> {
        match self.bits {
            0 => Some(REFSELSELECT_A::INT1V),
            1 => Some(REFSELSELECT_A::AVCC),
            2 => Some(REFSELSELECT_A::VREFP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INT1V`"]
    #[inline(always)]
    pub fn is_int1v(&self) -> bool {
        *self == REFSELSELECT_A::INT1V
    }
    #[doc = "Checks if the value of the field is `AVCC`"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        *self == REFSELSELECT_A::AVCC
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == REFSELSELECT_A::VREFP
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTRLB_SPEC, u8, REFSELSELECT_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Internal 1.0V reference"]
    #[inline(always)]
    pub fn int1v(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::INT1V)
    }
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::AVCC)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::VREFP)
    }
}
impl R {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&self) -> EOEN_R {
        EOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&self) -> VPD_R {
        VPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    pub fn bdwp(&self) -> BDWP_R {
        BDWP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoen(&mut self) -> EOEN_W<0> {
        EOEN_W::new(self)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ioen(&mut self) -> IOEN_W<1> {
        IOEN_W::new(self)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<2> {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vpd(&mut self) -> VPD_W<3> {
        VPD_W::new(self)
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn bdwp(&mut self) -> BDWP_W<4> {
        BDWP_W::new(self)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<6> {
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
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
