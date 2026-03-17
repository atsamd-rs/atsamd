#[doc = "Register `CTRLBCLR` reader"]
pub type R = crate::R<CtrlbclrSpec>;
#[doc = "Register `CTRLBCLR` writer"]
pub type W = crate::W<CtrlbclrSpec>;
#[doc = "Field `DIR` reader - Counter Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Counter Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LupdR = crate::BitReader;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Ramp Index Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idxcmdselect {
    #[doc = "0: Command disabled: Index toggles between cycles A and B"]
    Disable = 0,
    #[doc = "1: Set index: cycle B will be forced in the next cycle"]
    Set = 1,
    #[doc = "2: Clear index: cycle A will be forced in the next cycle"]
    Clear = 2,
    #[doc = "3: Hold index: the next cycle will be the same as the current cycle"]
    Hold = 3,
}
impl From<Idxcmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Idxcmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idxcmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Idxcmdselect {}
#[doc = "Field `IDXCMD` reader - Ramp Index Command"]
pub type IdxcmdR = crate::FieldReader<Idxcmdselect>;
impl IdxcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idxcmdselect {
        match self.bits {
            0 => Idxcmdselect::Disable,
            1 => Idxcmdselect::Set,
            2 => Idxcmdselect::Clear,
            3 => Idxcmdselect::Hold,
            _ => unreachable!(),
        }
    }
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idxcmdselect::Disable
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Idxcmdselect::Set
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Idxcmdselect::Clear
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == Idxcmdselect::Hold
    }
}
#[doc = "Field `IDXCMD` writer - Ramp Index Command"]
pub type IdxcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Idxcmdselect, crate::Safe>;
impl<'a, REG> IdxcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Command disabled: Index toggles between cycles A and B"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idxcmdselect::Disable)
    }
    #[doc = "Set index: cycle B will be forced in the next cycle"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Idxcmdselect::Set)
    }
    #[doc = "Clear index: cycle A will be forced in the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Idxcmdselect::Clear)
    }
    #[doc = "Hold index: the next cycle will be the same as the current cycle"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(Idxcmdselect::Hold)
    }
}
#[doc = "TCC Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Clear start, restart or retrigger"]
    Retrigger = 1,
    #[doc = "2: Force stop"]
    Stop = 2,
    #[doc = "3: Force update or double buffered registers"]
    Update = 3,
    #[doc = "4: Force COUNT read synchronization"]
    Readsync = 4,
    #[doc = "5: One-shot DMA trigger"]
    Dmaos = 5,
}
impl From<Cmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` reader - TCC Command"]
pub type CmdR = crate::FieldReader<Cmdselect>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdselect> {
        match self.bits {
            0 => Some(Cmdselect::None),
            1 => Some(Cmdselect::Retrigger),
            2 => Some(Cmdselect::Stop),
            3 => Some(Cmdselect::Update),
            4 => Some(Cmdselect::Readsync),
            5 => Some(Cmdselect::Dmaos),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cmdselect::None
    }
    #[doc = "Clear start, restart or retrigger"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == Cmdselect::Retrigger
    }
    #[doc = "Force stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Cmdselect::Stop
    }
    #[doc = "Force update or double buffered registers"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Cmdselect::Update
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline(always)]
    pub fn is_readsync(&self) -> bool {
        *self == Cmdselect::Readsync
    }
    #[doc = "One-shot DMA trigger"]
    #[inline(always)]
    pub fn is_dmaos(&self) -> bool {
        *self == Cmdselect::Dmaos
    }
}
#[doc = "Field `CMD` writer - TCC Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::None)
    }
    #[doc = "Clear start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Retrigger)
    }
    #[doc = "Force stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Stop)
    }
    #[doc = "Force update or double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Update)
    }
    #[doc = "Force COUNT read synchronization"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Readsync)
    }
    #[doc = "One-shot DMA trigger"]
    #[inline(always)]
    pub fn dmaos(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Dmaos)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LupdR {
        LupdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    pub fn idxcmd(&self) -> IdxcmdR {
        IdxcmdR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<CtrlbclrSpec> {
        DirW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LupdW<CtrlbclrSpec> {
        LupdW::new(self, 1)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> OneshotW<CtrlbclrSpec> {
        OneshotW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Ramp Index Command"]
    #[inline(always)]
    #[must_use]
    pub fn idxcmd(&mut self) -> IdxcmdW<CtrlbclrSpec> {
        IdxcmdW::new(self, 3)
    }
    #[doc = "Bits 5:7 - TCC Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CtrlbclrSpec> {
        CmdW::new(self, 5)
    }
}
#[doc = "Control B Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbclrSpec;
impl crate::RegisterSpec for CtrlbclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlbclr::R`](R) reader structure"]
impl crate::Readable for CtrlbclrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlbclr::W`](W) writer structure"]
impl crate::Writable for CtrlbclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLBCLR to value 0"]
impl crate::Resettable for CtrlbclrSpec {
    const RESET_VALUE: u8 = 0;
}
