#[doc = "Register `CHEVCTRL` reader"]
pub struct R(crate::R<CHEVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHEVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHEVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHEVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHEVCTRL` writer"]
pub struct W(crate::W<CHEVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHEVCTRL_SPEC>;
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
impl From<crate::W<CHEVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHEVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    TRIG = 1,
    #[doc = "2: Conditional transfer trigger"]
    CTRIG = 2,
    #[doc = "3: Conditional block transfer"]
    CBLOCK = 3,
    #[doc = "4: Channel suspend operation"]
    SUSPEND = 4,
    #[doc = "5: Channel resume operation"]
    RESUME = 5,
    #[doc = "6: Skip next block suspend action"]
    SSKIP = 6,
    #[doc = "7: Increase priority"]
    INCPRI = 7,
}
impl From<EVACT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVACT` reader - Channel Event Input Action"]
pub struct EVACT_R(crate::FieldReader<u8, EVACT_A>);
impl EVACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT_A {
        match self.bits {
            0 => EVACT_A::NOACT,
            1 => EVACT_A::TRIG,
            2 => EVACT_A::CTRIG,
            3 => EVACT_A::CBLOCK,
            4 => EVACT_A::SUSPEND,
            5 => EVACT_A::RESUME,
            6 => EVACT_A::SSKIP,
            7 => EVACT_A::INCPRI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        **self == EVACT_A::NOACT
    }
    #[doc = "Checks if the value of the field is `TRIG`"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        **self == EVACT_A::TRIG
    }
    #[doc = "Checks if the value of the field is `CTRIG`"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        **self == EVACT_A::CTRIG
    }
    #[doc = "Checks if the value of the field is `CBLOCK`"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        **self == EVACT_A::CBLOCK
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        **self == EVACT_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        **self == EVACT_A::RESUME
    }
    #[doc = "Checks if the value of the field is `SSKIP`"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        **self == EVACT_A::SSKIP
    }
    #[doc = "Checks if the value of the field is `INCPRI`"]
    #[inline(always)]
    pub fn is_incpri(&self) -> bool {
        **self == EVACT_A::INCPRI
    }
}
impl core::ops::Deref for EVACT_R {
    type Target = crate::FieldReader<u8, EVACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVACT` writer - Channel Event Input Action"]
pub struct EVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(EVACT_A::NOACT)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut W {
        self.variant(EVACT_A::TRIG)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut W {
        self.variant(EVACT_A::CTRIG)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut W {
        self.variant(EVACT_A::CBLOCK)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(EVACT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(EVACT_A::RESUME)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut W {
        self.variant(EVACT_A::SSKIP)
    }
    #[doc = "Increase priority"]
    #[inline(always)]
    pub fn incpri(self) -> &'a mut W {
        self.variant(EVACT_A::INCPRI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Channel Event Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVOMODE_A {
    #[doc = "0: Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    DEFAULT = 0,
    #[doc = "1: Ongoing trigger action"]
    TRIGACT = 1,
}
impl From<EVOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EVOMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVOMODE` reader - Channel Event Output Mode"]
pub struct EVOMODE_R(crate::FieldReader<u8, EVOMODE_A>);
impl EVOMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVOMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVOMODE_A> {
        match self.bits {
            0 => Some(EVOMODE_A::DEFAULT),
            1 => Some(EVOMODE_A::TRIGACT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == EVOMODE_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `TRIGACT`"]
    #[inline(always)]
    pub fn is_trigact(&self) -> bool {
        **self == EVOMODE_A::TRIGACT
    }
}
impl core::ops::Deref for EVOMODE_R {
    type Target = crate::FieldReader<u8, EVOMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVOMODE` writer - Channel Event Output Mode"]
pub struct EVOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVOMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(EVOMODE_A::DEFAULT)
    }
    #[doc = "Ongoing trigger action"]
    #[inline(always)]
    pub fn trigact(self) -> &'a mut W {
        self.variant(EVOMODE_A::TRIGACT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub struct EVIE_R(crate::FieldReader<bool, bool>);
impl EVIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub struct EVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub struct EVOE_R(crate::FieldReader<bool, bool>);
impl EVOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub struct EVOE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&self) -> EVOMODE_R {
        EVOMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EVACT_W {
        EVACT_W { w: self }
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&mut self) -> EVOMODE_W {
        EVOMODE_W { w: self }
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EVIE_W {
        EVIE_W { w: self }
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EVOE_W {
        EVOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chevctrl](index.html) module"]
pub struct CHEVCTRL_SPEC;
impl crate::RegisterSpec for CHEVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chevctrl::R](R) reader structure"]
impl crate::Readable for CHEVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chevctrl::W](W) writer structure"]
impl crate::Writable for CHEVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHEVCTRL to value 0"]
impl crate::Resettable for CHEVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
