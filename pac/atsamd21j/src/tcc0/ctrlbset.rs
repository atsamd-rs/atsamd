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
#[doc = "Field `DIR` reader - Counter Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Counter Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLBSET_SPEC, bool, O>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LUPD_R = crate::BitReader<bool>;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LUPD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLBSET_SPEC, bool, O>;
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub type ONESHOT_R = crate::BitReader<bool>;
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub type ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLBSET_SPEC, bool, O>;
#[doc = "Field `IDXCMD` reader - Ramp Index Command"]
pub type IDXCMD_R = crate::FieldReader<u8, IDXCMDSELECT_A>;
#[doc = "Ramp Index Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDXCMDSELECT_A {
    #[doc = "0: Command disabled: Index toggles between cycles A and B"]
    DISABLE = 0,
    #[doc = "1: Set index: cycle B will be forced in the next cycle"]
    SET = 1,
    #[doc = "2: Clear index: cycle A will be forced in the next cycle"]
    CLEAR = 2,
    #[doc = "3: Hold index: the next cycle will be the same as the current cycle"]
    HOLD = 3,
}
impl From<IDXCMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IDXCMDSELECT_A) -> Self {
        variant as _
    }
}
impl IDXCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDXCMDSELECT_A {
        match self.bits {
            0 => IDXCMDSELECT_A::DISABLE,
            1 => IDXCMDSELECT_A::SET,
            2 => IDXCMDSELECT_A::CLEAR,
            3 => IDXCMDSELECT_A::HOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDXCMDSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IDXCMDSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IDXCMDSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == IDXCMDSELECT_A::HOLD
    }
}
#[doc = "Field `IDXCMD` writer - Ramp Index Command"]
pub type IDXCMD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLBSET_SPEC, u8, IDXCMDSELECT_A, 2, O>;
impl<'a, const O: u8> IDXCMD_W<'a, O> {
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDXCMDSELECT_A::DISABLE)
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IDXCMDSELECT_A::SET)
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDXCMDSELECT_A::CLEAR)
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(IDXCMDSELECT_A::HOLD)
    }
}
#[doc = "Field `CMD` reader - TCC Command"]
pub type CMD_R = crate::FieldReader<u8, CMDSELECT_A>;
#[doc = "TCC Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Clear start, restart or retrigger"]
    RETRIGGER = 1,
    #[doc = "2: Force stop"]
    STOP = 2,
    #[doc = "3: Force update of double buffered registers"]
    UPDATE = 3,
    #[doc = "4: Force COUNT read synchronization"]
    READSYNC = 4,
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
            2 => Some(CMDSELECT_A::STOP),
            3 => Some(CMDSELECT_A::UPDATE),
            4 => Some(CMDSELECT_A::READSYNC),
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
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CMDSELECT_A::STOP
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
}
#[doc = "Field `CMD` writer - TCC Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLBSET_SPEC, u8, CMDSELECT_A, 3, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMDSELECT_A::NONE)
    }
    #[doc = "Clear start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMDSELECT_A::RETRIGGER)
    }
    #[doc = "Force stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMDSELECT_A::STOP)
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CMDSELECT_A::UPDATE)
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut W {
        self.variant(CMDSELECT_A::READSYNC)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&self) -> IDXCMD_R {
        IDXCMD_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LUPD_W<1> {
        LUPD_W::new(self)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> ONESHOT_W<2> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    #[must_use]
    pub fn idxcmd(&mut self) -> IDXCMD_W<3> {
        IDXCMD_W::new(self)
    }
    #[doc = "Bits 5:7 - TCC Command"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLBSET to value 0"]
impl crate::Resettable for CTRLBSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
