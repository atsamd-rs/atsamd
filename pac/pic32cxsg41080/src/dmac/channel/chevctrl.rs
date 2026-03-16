#[doc = "Register `CHEVCTRL` reader"]
pub type R = crate::R<ChevctrlSpec>;
#[doc = "Register `CHEVCTRL` writer"]
pub type W = crate::W<ChevctrlSpec>;
#[doc = "Channel Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evactselect {
    #[doc = "0: No action"]
    Noact = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    Trig = 1,
    #[doc = "2: Conditional transfer trigger"]
    Ctrig = 2,
    #[doc = "3: Conditional block transfer"]
    Cblock = 3,
    #[doc = "4: Channel suspend operation"]
    Suspend = 4,
    #[doc = "5: Channel resume operation"]
    Resume = 5,
    #[doc = "6: Skip next block suspend action"]
    Sskip = 6,
    #[doc = "7: Increase priority"]
    Incpri = 7,
}
impl From<Evactselect> for u8 {
    #[inline(always)]
    fn from(variant: Evactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evactselect {
    type Ux = u8;
}
impl crate::IsEnum for Evactselect {}
#[doc = "Field `EVACT` reader - Channel Event Input Action"]
pub type EvactR = crate::FieldReader<Evactselect>;
impl EvactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evactselect {
        match self.bits {
            0 => Evactselect::Noact,
            1 => Evactselect::Trig,
            2 => Evactselect::Ctrig,
            3 => Evactselect::Cblock,
            4 => Evactselect::Suspend,
            5 => Evactselect::Resume,
            6 => Evactselect::Sskip,
            7 => Evactselect::Incpri,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == Evactselect::Noact
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Evactselect::Trig
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == Evactselect::Ctrig
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == Evactselect::Cblock
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Evactselect::Suspend
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Evactselect::Resume
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == Evactselect::Sskip
    }
    #[doc = "Increase priority"]
    #[inline(always)]
    pub fn is_incpri(&self) -> bool {
        *self == Evactselect::Incpri
    }
}
#[doc = "Field `EVACT` writer - Channel Event Input Action"]
pub type EvactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Evactselect, crate::Safe>;
impl<'a, REG> EvactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Noact)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Trig)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Ctrig)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Cblock)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Suspend)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Resume)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Sskip)
    }
    #[doc = "Increase priority"]
    #[inline(always)]
    pub fn incpri(self) -> &'a mut crate::W<REG> {
        self.variant(Evactselect::Incpri)
    }
}
#[doc = "Channel Event Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evomodeselect {
    #[doc = "0: Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    Default = 0,
    #[doc = "1: Ongoing trigger action"]
    Trigact = 1,
}
impl From<Evomodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Evomodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evomodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Evomodeselect {}
#[doc = "Field `EVOMODE` reader - Channel Event Output Mode"]
pub type EvomodeR = crate::FieldReader<Evomodeselect>;
impl EvomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evomodeselect> {
        match self.bits {
            0 => Some(Evomodeselect::Default),
            1 => Some(Evomodeselect::Trigact),
            _ => None,
        }
    }
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Evomodeselect::Default
    }
    #[doc = "Ongoing trigger action"]
    #[inline(always)]
    pub fn is_trigact(&self) -> bool {
        *self == Evomodeselect::Trigact
    }
}
#[doc = "Field `EVOMODE` writer - Channel Event Output Mode"]
pub type EvomodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Evomodeselect>;
impl<'a, REG> EvomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Evomodeselect::Default)
    }
    #[doc = "Ongoing trigger action"]
    #[inline(always)]
    pub fn trigact(self) -> &'a mut crate::W<REG> {
        self.variant(Evomodeselect::Trigact)
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub type EvieR = crate::BitReader;
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub type EvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub type EvoeR = crate::BitReader;
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub type EvoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EvactR {
        EvactR::new(self.bits & 7)
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&self) -> EvomodeR {
        EvomodeR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EvieR {
        EvieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EvoeR {
        EvoeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EvactW<ChevctrlSpec> {
        EvactW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn evomode(&mut self) -> EvomodeW<ChevctrlSpec> {
        EvomodeW::new(self, 4)
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EvieW<ChevctrlSpec> {
        EvieW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoe(&mut self) -> EvoeW<ChevctrlSpec> {
        EvoeW::new(self, 7)
    }
}
#[doc = "Channel n Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`chevctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chevctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChevctrlSpec;
impl crate::RegisterSpec for ChevctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chevctrl::R`](R) reader structure"]
impl crate::Readable for ChevctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`chevctrl::W`](W) writer structure"]
impl crate::Writable for ChevctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHEVCTRL to value 0"]
impl crate::Resettable for ChevctrlSpec {
    const RESET_VALUE: u8 = 0;
}
