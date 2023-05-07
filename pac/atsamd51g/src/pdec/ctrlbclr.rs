#[doc = "Register `CTRLBCLR` reader"]
pub struct R(crate::R<CTRLBCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLBCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLBCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLBCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLBCLR` writer"]
pub struct W(crate::W<CTRLBCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLBCLR_SPEC>;
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
impl From<crate::W<CTRLBCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLBCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LUPD_R = crate::BitReader<bool>;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LUPD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLBCLR_SPEC, bool, O>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<u8, CMDSELECT_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
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
impl From<CMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            0 => Some(CMDSELECT_A::NONE),
            1 => Some(CMDSELECT_A::RETRIGGER),
            2 => Some(CMDSELECT_A::UPDATE),
            3 => Some(CMDSELECT_A::READSYNC),
            4 => Some(CMDSELECT_A::START),
            5 => Some(CMDSELECT_A::STOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMDSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == CMDSELECT_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == CMDSELECT_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `READSYNC`"]
    #[inline(always)]
    pub fn is_readsync(&self) -> bool {
        *self == CMDSELECT_A::READSYNC
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CMDSELECT_A::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CMDSELECT_A::STOP
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLBCLR_SPEC, u8, CMDSELECT_A, 3, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMDSELECT_A::NONE)
    }
    #[doc = "Force a counter restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMDSELECT_A::RETRIGGER)
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CMDSELECT_A::UPDATE)
    }
    #[doc = "Force a read synchronization of COUNT"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut W {
        self.variant(CMDSELECT_A::READSYNC)
    }
    #[doc = "Start QDEC/HALL"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CMDSELECT_A::START)
    }
    #[doc = "Stop QDEC/HALL"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMDSELECT_A::STOP)
    }
}
impl R {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LUPD_W<1> {
        LUPD_W::new(self)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<5> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlbclr](index.html) module"]
pub struct CTRLBCLR_SPEC;
impl crate::RegisterSpec for CTRLBCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlbclr::R](R) reader structure"]
impl crate::Readable for CTRLBCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlbclr::W](W) writer structure"]
impl crate::Writable for CTRLBCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLBCLR to value 0"]
impl crate::Resettable for CTRLBCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
