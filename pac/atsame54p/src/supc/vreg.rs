#[doc = "Register `VREG` reader"]
pub type R = crate::R<VregSpec>;
#[doc = "Register `VREG` writer"]
pub type W = crate::W<VregSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Voltage Regulator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selselect {
    #[doc = "0: LDO selection"]
    Ldo = 0,
    #[doc = "1: Buck selection"]
    Buck = 1,
}
impl From<Selselect> for bool {
    #[inline(always)]
    fn from(variant: Selselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Voltage Regulator Selection"]
pub type SelR = crate::BitReader<Selselect>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selselect {
        match self.bits {
            false => Selselect::Ldo,
            true => Selselect::Buck,
        }
    }
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == Selselect::Ldo
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == Selselect::Buck
    }
}
#[doc = "Field `SEL` writer - Voltage Regulator Selection"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Selselect>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Ldo)
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn buck(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Buck)
    }
}
#[doc = "Field `RUNBKUP` reader - Run in Backup mode"]
pub type RunbkupR = crate::BitReader;
#[doc = "Field `RUNBKUP` writer - Run in Backup mode"]
pub type RunbkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEN` reader - Voltage Scaling Enable"]
pub type VsenR = crate::BitReader;
#[doc = "Field `VSEN` writer - Voltage Scaling Enable"]
pub type VsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPER` reader - Voltage Scaling Period"]
pub type VsperR = crate::FieldReader;
#[doc = "Field `VSPER` writer - Voltage Scaling Period"]
pub type VsperW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&self) -> RunbkupR {
        RunbkupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VsenR {
        VsenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&self) -> VsperR {
        VsperR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<VregSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<VregSpec> {
        SelW::new(self, 2)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&mut self) -> RunbkupW<VregSpec> {
        RunbkupW::new(self, 7)
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    pub fn vsen(&mut self) -> VsenW<VregSpec> {
        VsenW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&mut self) -> VsperW<VregSpec> {
        VsperW::new(self, 24)
    }
}
#[doc = "VREG Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregSpec;
impl crate::RegisterSpec for VregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg::R`](R) reader structure"]
impl crate::Readable for VregSpec {}
#[doc = "`write(|w| ..)` method takes [`vreg::W`](W) writer structure"]
impl crate::Writable for VregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREG to value 0x02"]
impl crate::Resettable for VregSpec {
    const RESET_VALUE: u32 = 0x02;
}
