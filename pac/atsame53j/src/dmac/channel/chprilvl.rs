#[doc = "Register `CHPRILVL` reader"]
pub type R = crate::R<ChprilvlSpec>;
#[doc = "Register `CHPRILVL` writer"]
pub type W = crate::W<ChprilvlSpec>;
#[doc = "Channel Priority Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prilvlselect {
    #[doc = "0: Channel Priority Level 0 (Lowest Level)"]
    Lvl0 = 0,
    #[doc = "1: Channel Priority Level 1"]
    Lvl1 = 1,
    #[doc = "2: Channel Priority Level 2"]
    Lvl2 = 2,
    #[doc = "3: Channel Priority Level 3 (Highest Level)"]
    Lvl3 = 3,
}
impl From<Prilvlselect> for u8 {
    #[inline(always)]
    fn from(variant: Prilvlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prilvlselect {
    type Ux = u8;
}
impl crate::IsEnum for Prilvlselect {}
#[doc = "Field `PRILVL` reader - Channel Priority Level"]
pub type PrilvlR = crate::FieldReader<Prilvlselect>;
impl PrilvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prilvlselect {
        match self.bits {
            0 => Prilvlselect::Lvl0,
            1 => Prilvlselect::Lvl1,
            2 => Prilvlselect::Lvl2,
            3 => Prilvlselect::Lvl3,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        *self == Prilvlselect::Lvl0
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        *self == Prilvlselect::Lvl1
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        *self == Prilvlselect::Lvl2
    }
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        *self == Prilvlselect::Lvl3
    }
}
#[doc = "Field `PRILVL` writer - Channel Priority Level"]
pub type PrilvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prilvlselect, crate::Safe>;
impl<'a, REG> PrilvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut crate::W<REG> {
        self.variant(Prilvlselect::Lvl0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut crate::W<REG> {
        self.variant(Prilvlselect::Lvl1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut crate::W<REG> {
        self.variant(Prilvlselect::Lvl2)
    }
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut crate::W<REG> {
        self.variant(Prilvlselect::Lvl3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    pub fn prilvl(&self) -> PrilvlR {
        PrilvlR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    #[must_use]
    pub fn prilvl(&mut self) -> PrilvlW<ChprilvlSpec> {
        PrilvlW::new(self, 0)
    }
}
#[doc = "Channel n Priority Level\n\nYou can [`read`](crate::Reg::read) this register and get [`chprilvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chprilvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChprilvlSpec;
impl crate::RegisterSpec for ChprilvlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chprilvl::R`](R) reader structure"]
impl crate::Readable for ChprilvlSpec {}
#[doc = "`write(|w| ..)` method takes [`chprilvl::W`](W) writer structure"]
impl crate::Writable for ChprilvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHPRILVL to value 0"]
impl crate::Resettable for ChprilvlSpec {
    const RESET_VALUE: u8 = 0;
}
