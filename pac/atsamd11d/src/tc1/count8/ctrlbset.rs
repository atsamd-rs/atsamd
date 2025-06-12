#[doc = "Register `CTRLBSET` reader"]
pub type R = crate::R<CtrlbsetSpec>;
#[doc = "Register `CTRLBSET` writer"]
pub type W = crate::W<CtrlbsetSpec>;
#[doc = "Field `DIR` reader - Counter Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Counter Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Force a start, restart or retrigger"]
    Retrigger = 1,
    #[doc = "2: Force a stop"]
    Stop = 2,
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
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::FieldReader<Cmdselect>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdselect> {
        match self.bits {
            0 => Some(Cmdselect::None),
            1 => Some(Cmdselect::Retrigger),
            2 => Some(Cmdselect::Stop),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cmdselect::None
    }
    #[doc = "Force a start, restart or retrigger"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == Cmdselect::Retrigger
    }
    #[doc = "Force a stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Cmdselect::Stop
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdselect>;
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
    #[doc = "Force a start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Retrigger)
    }
    #[doc = "Force a stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Stop)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<CtrlbsetSpec> {
        DirW::new(self, 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<CtrlbsetSpec> {
        OneshotW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<CtrlbsetSpec> {
        CmdW::new(self, 6)
    }
}
#[doc = "Control B Set\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbsetSpec;
impl crate::RegisterSpec for CtrlbsetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlbset::R`](R) reader structure"]
impl crate::Readable for CtrlbsetSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlbset::W`](W) writer structure"]
impl crate::Writable for CtrlbsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLBSET to value 0"]
impl crate::Resettable for CtrlbsetSpec {}
