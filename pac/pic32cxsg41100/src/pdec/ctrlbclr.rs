#[doc = "Register `CTRLBCLR` reader"]
pub type R = crate::R<CtrlbclrSpec>;
#[doc = "Register `CTRLBCLR` writer"]
pub type W = crate::W<CtrlbclrSpec>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LupdR = crate::BitReader;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Force a counter restart or retrigger"]
    Retrigger = 1,
    #[doc = "2: Force update of double buffered registers"]
    Update = 2,
    #[doc = "3: Force a read synchronization of COUNT"]
    Readsync = 3,
    #[doc = "4: Start QDEC/HALL"]
    Start = 4,
    #[doc = "5: Stop QDEC/HALL"]
    Stop = 5,
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
            2 => Some(Cmdselect::Update),
            3 => Some(Cmdselect::Readsync),
            4 => Some(Cmdselect::Start),
            5 => Some(Cmdselect::Stop),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cmdselect::None
    }
    #[doc = "Force a counter restart or retrigger"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == Cmdselect::Retrigger
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Cmdselect::Update
    }
    #[doc = "Force a read synchronization of COUNT"]
    #[inline(always)]
    pub fn is_readsync(&self) -> bool {
        *self == Cmdselect::Readsync
    }
    #[doc = "Start QDEC/HALL"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Cmdselect::Start
    }
    #[doc = "Stop QDEC/HALL"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Cmdselect::Stop
    }
}
#[doc = "Field `CMD` writer - Command"]
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
    #[doc = "Force a counter restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Retrigger)
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Update)
    }
    #[doc = "Force a read synchronization of COUNT"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Readsync)
    }
    #[doc = "Start QDEC/HALL"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Start)
    }
    #[doc = "Stop QDEC/HALL"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Stop)
    }
}
impl R {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LupdR {
        LupdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LupdW<CtrlbclrSpec> {
        LupdW::new(self, 1)
    }
    #[doc = "Bits 5:7 - Command"]
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
