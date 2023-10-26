#[doc = "Register `CTRLBCLR` reader"]
pub type R = crate::R<CTRLBCLR_SPEC>;
#[doc = "Register `CTRLBCLR` writer"]
pub type W = crate::W<CTRLBCLR_SPEC>;
#[doc = "Field `DIR` reader - Counter Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Counter Direction"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LUPD_R = crate::BitReader;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub type ONESHOT_R = crate::BitReader;
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub type ONESHOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDXCMD` reader - Ramp Index Command"]
pub type IDXCMD_R = crate::FieldReader<IDXCMDSELECT_A>;
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
impl crate::FieldSpec for IDXCMDSELECT_A {
    type Ux = u8;
}
impl IDXCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDXCMDSELECT_A {
        match self.bits {
            0 => IDXCMDSELECT_A::DISABLE,
            1 => IDXCMDSELECT_A::SET,
            2 => IDXCMDSELECT_A::CLEAR,
            3 => IDXCMDSELECT_A::HOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDXCMDSELECT_A::DISABLE
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IDXCMDSELECT_A::SET
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IDXCMDSELECT_A::CLEAR
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == IDXCMDSELECT_A::HOLD
    }
}
#[doc = "Field `IDXCMD` writer - Ramp Index Command"]
pub type IDXCMD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, IDXCMDSELECT_A>;
impl<'a, REG, const O: u8> IDXCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDXCMDSELECT_A::DISABLE)
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(IDXCMDSELECT_A::SET)
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IDXCMDSELECT_A::CLEAR)
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(IDXCMDSELECT_A::HOLD)
    }
}
#[doc = "Field `CMD` reader - TCC Command"]
pub type CMD_R = crate::FieldReader<CMDSELECT_A>;
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
impl crate::FieldSpec for CMDSELECT_A {
    type Ux = u8;
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            0 => Some(CMDSELECT_A::NONE),
            1 => Some(CMDSELECT_A::RETRIGGER),
            2 => Some(CMDSELECT_A::STOP),
            3 => Some(CMDSELECT_A::UPDATE),
            4 => Some(CMDSELECT_A::READSYNC),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMDSELECT_A::NONE
    }
    #[doc = "Clear start, restart or retrigger"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == CMDSELECT_A::RETRIGGER
    }
    #[doc = "Force stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CMDSELECT_A::STOP
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == CMDSELECT_A::UPDATE
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline(always)]
    pub fn is_readsync(&self) -> bool {
        *self == CMDSELECT_A::READSYNC
    }
}
#[doc = "Field `CMD` writer - TCC Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CMDSELECT_A>;
impl<'a, REG, const O: u8> CMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::NONE)
    }
    #[doc = "Clear start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::RETRIGGER)
    }
    #[doc = "Force stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::STOP)
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::UPDATE)
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut crate::W<REG> {
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
    pub fn dir(&mut self) -> DIR_W<CTRLBCLR_SPEC, 0> {
        DIR_W::new(self)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LUPD_W<CTRLBCLR_SPEC, 1> {
        LUPD_W::new(self)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> ONESHOT_W<CTRLBCLR_SPEC, 2> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    #[must_use]
    pub fn idxcmd(&mut self) -> IDXCMD_W<CTRLBCLR_SPEC, 3> {
        IDXCMD_W::new(self)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CTRLBCLR_SPEC, 5> {
        CMD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control B Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlbclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlbclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLBCLR_SPEC;
impl crate::RegisterSpec for CTRLBCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlbclr::R`](R) reader structure"]
impl crate::Readable for CTRLBCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlbclr::W`](W) writer structure"]
impl crate::Writable for CTRLBCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLBCLR to value 0"]
impl crate::Resettable for CTRLBCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
