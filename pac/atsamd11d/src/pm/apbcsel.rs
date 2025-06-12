#[doc = "Register `APBCSEL` reader"]
pub type R = crate::R<ApbcselSpec>;
#[doc = "Register `APBCSEL` writer"]
pub type W = crate::W<ApbcselSpec>;
#[doc = "APBC Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apbcdivselect {
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
impl From<Apbcdivselect> for u8 {
    #[inline(always)]
    fn from(variant: Apbcdivselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apbcdivselect {
    type Ux = u8;
}
impl crate::IsEnum for Apbcdivselect {}
#[doc = "Field `APBCDIV` reader - APBC Prescaler Selection"]
pub type ApbcdivR = crate::FieldReader<Apbcdivselect>;
impl ApbcdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apbcdivselect {
        match self.bits {
            0 => Apbcdivselect::Div1,
            1 => Apbcdivselect::Div2,
            2 => Apbcdivselect::Div4,
            3 => Apbcdivselect::Div8,
            4 => Apbcdivselect::Div16,
            5 => Apbcdivselect::Div32,
            6 => Apbcdivselect::Div64,
            7 => Apbcdivselect::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Apbcdivselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Apbcdivselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Apbcdivselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Apbcdivselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Apbcdivselect::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Apbcdivselect::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Apbcdivselect::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Apbcdivselect::Div128
    }
}
#[doc = "Field `APBCDIV` writer - APBC Prescaler Selection"]
pub type ApbcdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Apbcdivselect, crate::Safe>;
impl<'a, REG> ApbcdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Apbcdivselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:2 - APBC Prescaler Selection"]
    #[inline(always)]
    pub fn apbcdiv(&self) -> ApbcdivR {
        ApbcdivR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBC Prescaler Selection"]
    #[inline(always)]
    pub fn apbcdiv(&mut self) -> ApbcdivW<ApbcselSpec> {
        ApbcdivW::new(self, 0)
    }
}
#[doc = "APBC Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbcselSpec;
impl crate::RegisterSpec for ApbcselSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbcsel::R`](R) reader structure"]
impl crate::Readable for ApbcselSpec {}
#[doc = "`write(|w| ..)` method takes [`apbcsel::W`](W) writer structure"]
impl crate::Writable for ApbcselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBCSEL to value 0"]
impl crate::Resettable for ApbcselSpec {}
