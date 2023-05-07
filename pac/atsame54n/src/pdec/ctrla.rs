#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Operation Mode"]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: QDEC operating mode"]
    QDEC = 0,
    #[doc = "1: HALL operating mode"]
    HALL = 1,
    #[doc = "2: COUNTER operating mode"]
    COUNTER = 2,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::QDEC),
            1 => Some(MODESELECT_A::HALL),
            2 => Some(MODESELECT_A::COUNTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `QDEC`"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == MODESELECT_A::QDEC
    }
    #[doc = "Checks if the value of the field is `HALL`"]
    #[inline(always)]
    pub fn is_hall(&self) -> bool {
        *self == MODESELECT_A::HALL
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == MODESELECT_A::COUNTER
    }
}
#[doc = "Field `MODE` writer - Operation Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, MODESELECT_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "QDEC operating mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut W {
        self.variant(MODESELECT_A::QDEC)
    }
    #[doc = "HALL operating mode"]
    #[inline(always)]
    pub fn hall(self) -> &'a mut W {
        self.variant(MODESELECT_A::HALL)
    }
    #[doc = "COUNTER operating mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut W {
        self.variant(MODESELECT_A::COUNTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `CONF` reader - PDEC Configuration"]
pub type CONF_R = crate::FieldReader<u8, CONFSELECT_A>;
#[doc = "PDEC Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONFSELECT_A {
    #[doc = "0: Quadrature decoder direction"]
    X4 = 0,
    #[doc = "1: Secure Quadrature decoder direction"]
    X4S = 1,
    #[doc = "2: Decoder direction"]
    X2 = 2,
    #[doc = "3: Secure decoder direction"]
    X2S = 3,
    #[doc = "4: Auto correction mode"]
    AUTOC = 4,
}
impl From<CONFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFSELECT_A) -> Self {
        variant as _
    }
}
impl CONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CONFSELECT_A> {
        match self.bits {
            0 => Some(CONFSELECT_A::X4),
            1 => Some(CONFSELECT_A::X4S),
            2 => Some(CONFSELECT_A::X2),
            3 => Some(CONFSELECT_A::X2S),
            4 => Some(CONFSELECT_A::AUTOC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == CONFSELECT_A::X4
    }
    #[doc = "Checks if the value of the field is `X4S`"]
    #[inline(always)]
    pub fn is_x4s(&self) -> bool {
        *self == CONFSELECT_A::X4S
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == CONFSELECT_A::X2
    }
    #[doc = "Checks if the value of the field is `X2S`"]
    #[inline(always)]
    pub fn is_x2s(&self) -> bool {
        *self == CONFSELECT_A::X2S
    }
    #[doc = "Checks if the value of the field is `AUTOC`"]
    #[inline(always)]
    pub fn is_autoc(&self) -> bool {
        *self == CONFSELECT_A::AUTOC
    }
}
#[doc = "Field `CONF` writer - PDEC Configuration"]
pub type CONF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, CONFSELECT_A, 3, O>;
impl<'a, const O: u8> CONF_W<'a, O> {
    #[doc = "Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(CONFSELECT_A::X4)
    }
    #[doc = "Secure Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4s(self) -> &'a mut W {
        self.variant(CONFSELECT_A::X4S)
    }
    #[doc = "Decoder direction"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(CONFSELECT_A::X2)
    }
    #[doc = "Secure decoder direction"]
    #[inline(always)]
    pub fn x2s(self) -> &'a mut W {
        self.variant(CONFSELECT_A::X2S)
    }
    #[doc = "Auto correction mode"]
    #[inline(always)]
    pub fn autoc(self) -> &'a mut W {
        self.variant(CONFSELECT_A::AUTOC)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type ALOCK_R = crate::BitReader<bool>;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type ALOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `SWAP` reader - PDEC Phase A and B Swap"]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - PDEC Phase A and B Swap"]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PEREN` reader - Period Enable"]
pub type PEREN_R = crate::BitReader<bool>;
#[doc = "Field `PEREN` writer - Period Enable"]
pub type PEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PINEN0` reader - PDEC Input From Pin 0 Enable"]
pub type PINEN0_R = crate::BitReader<bool>;
#[doc = "Field `PINEN0` writer - PDEC Input From Pin 0 Enable"]
pub type PINEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PINEN1` reader - PDEC Input From Pin 1 Enable"]
pub type PINEN1_R = crate::BitReader<bool>;
#[doc = "Field `PINEN1` writer - PDEC Input From Pin 1 Enable"]
pub type PINEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PINEN2` reader - PDEC Input From Pin 2 Enable"]
pub type PINEN2_R = crate::BitReader<bool>;
#[doc = "Field `PINEN2` writer - PDEC Input From Pin 2 Enable"]
pub type PINEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PINVEN0` reader - IO Pin 0 Invert Enable"]
pub type PINVEN0_R = crate::BitReader<bool>;
#[doc = "Field `PINVEN0` writer - IO Pin 0 Invert Enable"]
pub type PINVEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PINVEN1` reader - IO Pin 1 Invert Enable"]
pub type PINVEN1_R = crate::BitReader<bool>;
#[doc = "Field `PINVEN1` writer - IO Pin 1 Invert Enable"]
pub type PINVEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PINVEN2` reader - IO Pin 2 Invert Enable"]
pub type PINVEN2_R = crate::BitReader<bool>;
#[doc = "Field `PINVEN2` writer - IO Pin 2 Invert Enable"]
pub type PINVEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `ANGULAR` reader - Angular Counter Length"]
pub type ANGULAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANGULAR` writer - Angular Counter Length"]
pub type ANGULAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 3, O>;
#[doc = "Field `MAXCMP` reader - Maximum Consecutive Missing Pulses"]
pub type MAXCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXCMP` writer - Maximum Consecutive Missing Pulses"]
pub type MAXCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    pub fn pinen0(&self) -> PINEN0_R {
        PINEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    pub fn pinen1(&self) -> PINEN1_R {
        PINEN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    pub fn pinen2(&self) -> PINEN2_R {
        PINEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    pub fn pinven0(&self) -> PINVEN0_R {
        PINVEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    pub fn pinven1(&self) -> PINVEN1_R {
        PINVEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    pub fn pinven2(&self) -> PINVEN2_R {
        PINVEN2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    pub fn angular(&self) -> ANGULAR_R {
        ANGULAR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&self) -> MAXCMP_R {
        MAXCMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn conf(&mut self) -> CONF_W<8> {
        CONF_W::new(self)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    #[must_use]
    pub fn alock(&mut self) -> ALOCK_W<11> {
        ALOCK_W::new(self)
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<14> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peren(&mut self) -> PEREN_W<15> {
        PEREN_W::new(self)
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinen0(&mut self) -> PINEN0_W<16> {
        PINEN0_W::new(self)
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinen1(&mut self) -> PINEN1_W<17> {
        PINEN1_W::new(self)
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinen2(&mut self) -> PINEN2_W<18> {
        PINEN2_W::new(self)
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinven0(&mut self) -> PINVEN0_W<20> {
        PINVEN0_W::new(self)
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinven1(&mut self) -> PINVEN1_W<21> {
        PINVEN1_W::new(self)
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinven2(&mut self) -> PINVEN2_W<22> {
        PINVEN2_W::new(self)
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    #[must_use]
    pub fn angular(&mut self) -> ANGULAR_W<24> {
        ANGULAR_W::new(self)
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    #[must_use]
    pub fn maxcmp(&mut self) -> MAXCMP_W<28> {
        MAXCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
