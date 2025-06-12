#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "AHB Maximum Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bmaxselect {
    #[doc = "0: `0`"]
    Incr16 = 0,
    #[doc = "1: `1`"]
    Incr8 = 1,
    #[doc = "2: `10`"]
    Incr4 = 2,
    #[doc = "3: `11`"]
    Single = 3,
}
impl From<Bmaxselect> for u8 {
    #[inline(always)]
    fn from(variant: Bmaxselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bmaxselect {
    type Ux = u8;
}
impl crate::IsEnum for Bmaxselect {}
#[doc = "Field `BMAX` reader - AHB Maximum Burst"]
pub type BmaxR = crate::FieldReader<Bmaxselect>;
impl BmaxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bmaxselect {
        match self.bits {
            0 => Bmaxselect::Incr16,
            1 => Bmaxselect::Incr8,
            2 => Bmaxselect::Incr4,
            3 => Bmaxselect::Single,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == Bmaxselect::Incr16
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == Bmaxselect::Incr8
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == Bmaxselect::Incr4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Bmaxselect::Single
    }
}
#[doc = "Field `BMAX` writer - AHB Maximum Burst"]
pub type BmaxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bmaxselect, crate::Safe>;
impl<'a, REG> BmaxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(Bmaxselect::Incr16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(Bmaxselect::Incr8)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(Bmaxselect::Incr4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Bmaxselect::Single)
    }
}
impl R {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&self) -> BmaxR {
        BmaxR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&mut self) -> BmaxW<AcrSpec> {
        BmaxW::new(self, 0)
    }
}
#[doc = "AHB Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for AcrSpec {}
