#[doc = "Register `APBBSEL` reader"]
pub type R = crate::R<ApbbselSpec>;
#[doc = "Register `APBBSEL` writer"]
pub type W = crate::W<ApbbselSpec>;
#[doc = "APBB Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apbbdivselect {
    #[doc = "0: Divide by 1"]
    Div1 = 0,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "2: Divide by 4"]
    Div4 = 2,
    #[doc = "3: Divide by 8"]
    Div8 = 3,
    #[doc = "4: Divide by 16"]
    Div16 = 4,
    #[doc = "5: Divide by 32"]
    Div32 = 5,
    #[doc = "6: Divide by 64"]
    Div64 = 6,
    #[doc = "7: Divide by 128"]
    Div128 = 7,
}
impl From<Apbbdivselect> for u8 {
    #[inline(always)]
    fn from(variant: Apbbdivselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apbbdivselect {
    type Ux = u8;
}
impl crate::IsEnum for Apbbdivselect {}
#[doc = "Field `APBBDIV` reader - APBB Prescaler Selection"]
pub type ApbbdivR = crate::FieldReader<Apbbdivselect>;
impl ApbbdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apbbdivselect {
        match self.bits {
            0 => Apbbdivselect::Div1,
            1 => Apbbdivselect::Div2,
            2 => Apbbdivselect::Div4,
            3 => Apbbdivselect::Div8,
            4 => Apbbdivselect::Div16,
            5 => Apbbdivselect::Div32,
            6 => Apbbdivselect::Div64,
            7 => Apbbdivselect::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Apbbdivselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Apbbdivselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Apbbdivselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Apbbdivselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Apbbdivselect::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Apbbdivselect::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Apbbdivselect::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Apbbdivselect::Div128
    }
}
#[doc = "Field `APBBDIV` writer - APBB Prescaler Selection"]
pub type ApbbdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Apbbdivselect, crate::Safe>;
impl<'a, REG> ApbbdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbdivselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline(always)]
    pub fn apbbdiv(&self) -> ApbbdivR {
        ApbbdivR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline(always)]
    pub fn apbbdiv(&mut self) -> ApbbdivW<ApbbselSpec> {
        ApbbdivW::new(self, 0)
    }
}
#[doc = "APBB Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`apbbsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbbsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbbselSpec;
impl crate::RegisterSpec for ApbbselSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbbsel::R`](R) reader structure"]
impl crate::Readable for ApbbselSpec {}
#[doc = "`write(|w| ..)` method takes [`apbbsel::W`](W) writer structure"]
impl crate::Writable for ApbbselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBBSEL to value 0"]
impl crate::Resettable for ApbbselSpec {}
