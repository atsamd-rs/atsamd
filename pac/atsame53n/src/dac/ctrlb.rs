#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `DIFF` reader - Differential mode enable"]
pub type DiffR = crate::BitReader;
#[doc = "Field `DIFF` writer - Differential mode enable"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference Selection for DAC0/1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refselselect {
    #[doc = "0: External reference unbuffered"]
    Vrefpu = 0,
    #[doc = "1: Analog supply"]
    Vddana = 1,
    #[doc = "2: External reference buffered"]
    Vrefpb = 2,
    #[doc = "3: Internal bandgap reference"]
    Intref = 3,
}
impl From<Refselselect> for u8 {
    #[inline(always)]
    fn from(variant: Refselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refselselect {
    type Ux = u8;
}
impl crate::IsEnum for Refselselect {}
#[doc = "Field `REFSEL` reader - Reference Selection for DAC0/1"]
pub type RefselR = crate::FieldReader<Refselselect>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refselselect {
        match self.bits {
            0 => Refselselect::Vrefpu,
            1 => Refselselect::Vddana,
            2 => Refselselect::Vrefpb,
            3 => Refselselect::Intref,
            _ => unreachable!(),
        }
    }
    #[doc = "External reference unbuffered"]
    #[inline(always)]
    pub fn is_vrefpu(&self) -> bool {
        *self == Refselselect::Vrefpu
    }
    #[doc = "Analog supply"]
    #[inline(always)]
    pub fn is_vddana(&self) -> bool {
        *self == Refselselect::Vddana
    }
    #[doc = "External reference buffered"]
    #[inline(always)]
    pub fn is_vrefpb(&self) -> bool {
        *self == Refselselect::Vrefpb
    }
    #[doc = "Internal bandgap reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == Refselselect::Intref
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection for DAC0/1"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refselselect, crate::Safe>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External reference unbuffered"]
    #[inline(always)]
    pub fn vrefpu(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Vrefpu)
    }
    #[doc = "Analog supply"]
    #[inline(always)]
    pub fn vddana(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Vddana)
    }
    #[doc = "External reference buffered"]
    #[inline(always)]
    pub fn vrefpb(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Vrefpb)
    }
    #[doc = "Internal bandgap reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Intref)
    }
}
impl R {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<CtrlbSpec> {
        DiffW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<CtrlbSpec> {
        RefselW::new(self, 1)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0x02"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u8 = 0x02;
}
