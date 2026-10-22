#[doc = "Register `VREF` reader"]
pub struct R(crate::R<VREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF` writer"]
pub struct W(crate::W<VREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_SPEC>;
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
impl From<crate::W<VREF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Output Enable"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - Temperature Sensor Output Enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREF_SPEC, bool, O>;
#[doc = "Field `VREFOE` reader - Voltage Reference Output Enable"]
pub type VREFOE_R = crate::BitReader<bool>;
#[doc = "Field `VREFOE` writer - Voltage Reference Output Enable"]
pub type VREFOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREF_SPEC, bool, O>;
#[doc = "Field `TSSEL` reader - Temperature Sensor Selection"]
pub type TSSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSSEL` writer - Temperature Sensor Selection"]
pub type TSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREF_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREF_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Contrl"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Contrl"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREF_SPEC, bool, O>;
#[doc = "Field `SEL` reader - Voltage Reference Selection"]
pub type SEL_R = crate::FieldReader<u8, SELSELECT_A>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELSELECT_A {
    #[doc = "0: 1.0V voltage reference typical value"]
    _1V0 = 0,
    #[doc = "1: 1.1V voltage reference typical value"]
    _1V1 = 1,
    #[doc = "2: 1.2V voltage reference typical value"]
    _1V2 = 2,
    #[doc = "3: 1.25V voltage reference typical value"]
    _1V25 = 3,
    #[doc = "4: 2.0V voltage reference typical value"]
    _2V0 = 4,
    #[doc = "5: 2.2V voltage reference typical value"]
    _2V2 = 5,
    #[doc = "6: 2.4V voltage reference typical value"]
    _2V4 = 6,
    #[doc = "7: 2.5V voltage reference typical value"]
    _2V5 = 7,
}
impl From<SELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELSELECT_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELSELECT_A> {
        match self.bits {
            0 => Some(SELSELECT_A::_1V0),
            1 => Some(SELSELECT_A::_1V1),
            2 => Some(SELSELECT_A::_1V2),
            3 => Some(SELSELECT_A::_1V25),
            4 => Some(SELSELECT_A::_2V0),
            5 => Some(SELSELECT_A::_2V2),
            6 => Some(SELSELECT_A::_2V4),
            7 => Some(SELSELECT_A::_2V5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1V0`"]
    #[inline(always)]
    pub fn is_1v0(&self) -> bool {
        *self == SELSELECT_A::_1V0
    }
    #[doc = "Checks if the value of the field is `_1V1`"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        *self == SELSELECT_A::_1V1
    }
    #[doc = "Checks if the value of the field is `_1V2`"]
    #[inline(always)]
    pub fn is_1v2(&self) -> bool {
        *self == SELSELECT_A::_1V2
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == SELSELECT_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V0`"]
    #[inline(always)]
    pub fn is_2v0(&self) -> bool {
        *self == SELSELECT_A::_2V0
    }
    #[doc = "Checks if the value of the field is `_2V2`"]
    #[inline(always)]
    pub fn is_2v2(&self) -> bool {
        *self == SELSELECT_A::_2V2
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == SELSELECT_A::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == SELSELECT_A::_2V5
    }
}
#[doc = "Field `SEL` writer - Voltage Reference Selection"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_SPEC, u8, SELSELECT_A, 4, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "1.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v0(self) -> &'a mut W {
        self.variant(SELSELECT_A::_1V0)
    }
    #[doc = "1.1V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut W {
        self.variant(SELSELECT_A::_1V1)
    }
    #[doc = "1.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v2(self) -> &'a mut W {
        self.variant(SELSELECT_A::_1V2)
    }
    #[doc = "1.25V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(SELSELECT_A::_1V25)
    }
    #[doc = "2.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v0(self) -> &'a mut W {
        self.variant(SELSELECT_A::_2V0)
    }
    #[doc = "2.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v2(self) -> &'a mut W {
        self.variant(SELSELECT_A::_2V2)
    }
    #[doc = "2.4V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(SELSELECT_A::_2V4)
    }
    #[doc = "2.5V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(SELSELECT_A::_2V5)
    }
}
impl R {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn vrefoe(&self) -> VREFOE_R {
        VREFOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<1> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefoe(&mut self) -> VREFOE_W<2> {
        VREFOE_W::new(self)
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tssel(&mut self) -> TSSEL_W<3> {
        TSSEL_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<16> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref](index.html) module"]
pub struct VREF_SPEC;
impl crate::RegisterSpec for VREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref::R](R) reader structure"]
impl crate::Readable for VREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref::W](W) writer structure"]
impl crate::Writable for VREF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
