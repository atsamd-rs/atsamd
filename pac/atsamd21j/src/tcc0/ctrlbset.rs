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
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Counter Direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
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
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub struct ONESHOT_R(crate::FieldReader<bool, bool>);
impl ONESHOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONESHOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONESHOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub struct ONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Ramp Index Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDXCMD_A {
    #[doc = "0: Command disabled: Index toggles between cycles A and B"]
    DISABLE = 0,
    #[doc = "1: Set index: cycle B will be forced in the next cycle"]
    SET = 1,
    #[doc = "2: Clear index: cycle A will be forced in the next cycle"]
    CLEAR = 2,
    #[doc = "3: Hold index: the next cycle will be the same as the current cycle"]
    HOLD = 3,
}
impl From<IDXCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: IDXCMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDXCMD` reader - Ramp Index Command"]
pub struct IDXCMD_R(crate::FieldReader<u8, IDXCMD_A>);
impl IDXCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDXCMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDXCMD_A {
        match self.bits {
            0 => IDXCMD_A::DISABLE,
            1 => IDXCMD_A::SET,
            2 => IDXCMD_A::CLEAR,
            3 => IDXCMD_A::HOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IDXCMD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == IDXCMD_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == IDXCMD_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        **self == IDXCMD_A::HOLD
    }
}
impl core::ops::Deref for IDXCMD_R {
    type Target = crate::FieldReader<u8, IDXCMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDXCMD` writer - Ramp Index Command"]
pub struct IDXCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> IDXCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDXCMD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDXCMD_A::DISABLE)
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IDXCMD_A::SET)
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDXCMD_A::CLEAR)
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(IDXCMD_A::HOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u8 & 0x03) << 3);
        self.w
    }
}
#[doc = "TCC Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
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
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - TCC Command"]
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
            2 => Some(CMD_A::STOP),
            3 => Some(CMD_A::UPDATE),
            4 => Some(CMD_A::READSYNC),
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
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == CMD_A::STOP
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
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - TCC Command"]
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
    #[doc = "Clear start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMD_A::RETRIGGER)
    }
    #[doc = "Force stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMD_A::STOP)
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CMD_A::UPDATE)
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut W {
        self.variant(CMD_A::READSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u8 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&self) -> IDXCMD_R {
        IDXCMD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&mut self) -> LUPD_W {
        LUPD_W { w: self }
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W {
        ONESHOT_W { w: self }
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&mut self) -> IDXCMD_W {
        IDXCMD_W { w: self }
    }
    #[doc = "Bits 5:7 - TCC Command"]
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
