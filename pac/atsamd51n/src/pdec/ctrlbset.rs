#[doc = "Register `CTRLBSET` reader"]
pub struct R(crate::R<CTRLBSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLBSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLBSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLBSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLBSET` writer"]
pub struct W(crate::W<CTRLBSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLBSET_SPEC>;
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
impl From<crate::W<CTRLBSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLBSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUPD` reader - Lock Update"]
pub struct LUPD_R(crate::FieldReader<bool, bool>);
impl LUPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LUPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUPD` writer - Lock Update"]
pub struct LUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> LUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Force a counter restart or retrigger"]
    RETRIGGER = 1,
    #[doc = "2: Force update of double buffered registers"]
    UPDATE = 2,
    #[doc = "3: Force a read synchronization of COUNT"]
    READSYNC = 3,
    #[doc = "4: Start QDEC/HALL"]
    START = 4,
    #[doc = "5: Stop QDEC/HALL"]
    STOP = 5,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - Command"]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::NONE),
            1 => Some(CMD_A::RETRIGGER),
            2 => Some(CMD_A::UPDATE),
            3 => Some(CMD_A::READSYNC),
            4 => Some(CMD_A::START),
            5 => Some(CMD_A::STOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == CMD_A::NONE
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        **self == CMD_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == CMD_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `READSYNC`"]
    #[inline(always)]
    pub fn is_readsync(&self) -> bool {
        **self == CMD_A::READSYNC
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == CMD_A::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == CMD_A::STOP
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Command"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMD_A::NONE)
    }
    #[doc = "Force a counter restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMD_A::RETRIGGER)
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CMD_A::UPDATE)
    }
    #[doc = "Force a read synchronization of COUNT"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut W {
        self.variant(CMD_A::READSYNC)
    }
    #[doc = "Start QDEC/HALL"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CMD_A::START)
    }
    #[doc = "Stop QDEC/HALL"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMD_A::STOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u8 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&mut self) -> LUPD_W {
        LUPD_W { w: self }
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlbset](index.html) module"]
pub struct CTRLBSET_SPEC;
impl crate::RegisterSpec for CTRLBSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlbset::R](R) reader structure"]
impl crate::Readable for CTRLBSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlbset::W](W) writer structure"]
impl crate::Writable for CTRLBSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLBSET to value 0"]
impl crate::Resettable for CTRLBSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
