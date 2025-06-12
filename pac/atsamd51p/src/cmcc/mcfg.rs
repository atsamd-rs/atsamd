#[doc = "Register `MCFG` reader"]
pub type R = crate::R<McfgSpec>;
#[doc = "Register `MCFG` writer"]
pub type W = crate::W<McfgSpec>;
#[doc = "Cache Controller Monitor Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: Cycle counter"]
    CycleCount = 0,
    #[doc = "1: Instruction hit counter"]
    IhitCount = 1,
    #[doc = "2: Data hit counter"]
    DhitCount = 2,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Cache Controller Monitor Counter Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::CycleCount),
            1 => Some(Modeselect::IhitCount),
            2 => Some(Modeselect::DhitCount),
            _ => None,
        }
    }
    #[doc = "Cycle counter"]
    #[inline(always)]
    pub fn is_cycle_count(&self) -> bool {
        *self == Modeselect::CycleCount
    }
    #[doc = "Instruction hit counter"]
    #[inline(always)]
    pub fn is_ihit_count(&self) -> bool {
        *self == Modeselect::IhitCount
    }
    #[doc = "Data hit counter"]
    #[inline(always)]
    pub fn is_dhit_count(&self) -> bool {
        *self == Modeselect::DhitCount
    }
}
#[doc = "Field `MODE` writer - Cache Controller Monitor Counter Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cycle counter"]
    #[inline(always)]
    pub fn cycle_count(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::CycleCount)
    }
    #[doc = "Instruction hit counter"]
    #[inline(always)]
    pub fn ihit_count(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::IhitCount)
    }
    #[doc = "Data hit counter"]
    #[inline(always)]
    pub fn dhit_count(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::DhitCount)
    }
}
impl R {
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<McfgSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Cache Monitor Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McfgSpec;
impl crate::RegisterSpec for McfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg::R`](R) reader structure"]
impl crate::Readable for McfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mcfg::W`](W) writer structure"]
impl crate::Writable for McfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCFG to value 0"]
impl crate::Resettable for McfgSpec {}
