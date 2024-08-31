#[doc = "Register `APBASEL` reader"]
pub type R = crate::R<ApbaselSpec>;
#[doc = "Register `APBASEL` writer"]
pub type W = crate::W<ApbaselSpec>;
#[doc = "APBA Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apbadivselect {
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
impl From<Apbadivselect> for u8 {
    #[inline(always)]
    fn from(variant: Apbadivselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apbadivselect {
    type Ux = u8;
}
impl crate::IsEnum for Apbadivselect {}
#[doc = "Field `APBADIV` reader - APBA Prescaler Selection"]
pub type ApbadivR = crate::FieldReader<Apbadivselect>;
impl ApbadivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apbadivselect {
        match self.bits {
            0 => Apbadivselect::Div1,
            1 => Apbadivselect::Div2,
            2 => Apbadivselect::Div4,
            3 => Apbadivselect::Div8,
            4 => Apbadivselect::Div16,
            5 => Apbadivselect::Div32,
            6 => Apbadivselect::Div64,
            7 => Apbadivselect::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Apbadivselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Apbadivselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Apbadivselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Apbadivselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Apbadivselect::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Apbadivselect::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Apbadivselect::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Apbadivselect::Div128
    }
}
#[doc = "Field `APBADIV` writer - APBA Prescaler Selection"]
pub type ApbadivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Apbadivselect, crate::Safe>;
impl<'a, REG> ApbadivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Apbadivselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline(always)]
    pub fn apbadiv(&self) -> ApbadivR {
        ApbadivR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn apbadiv(&mut self) -> ApbadivW<ApbaselSpec> {
        ApbadivW::new(self, 0)
    }
}
#[doc = "APBA Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`apbasel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbasel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaselSpec;
impl crate::RegisterSpec for ApbaselSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbasel::R`](R) reader structure"]
impl crate::Readable for ApbaselSpec {}
#[doc = "`write(|w| ..)` method takes [`apbasel::W`](W) writer structure"]
impl crate::Writable for ApbaselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets APBASEL to value 0"]
impl crate::Resettable for ApbaselSpec {
    const RESET_VALUE: u8 = 0;
}
