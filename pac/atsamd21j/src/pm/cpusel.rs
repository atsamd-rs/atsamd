#[doc = "Register `CPUSEL` reader"]
pub type R = crate::R<CpuselSpec>;
#[doc = "Register `CPUSEL` writer"]
pub type W = crate::W<CpuselSpec>;
#[doc = "CPU Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpudivselect {
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
impl From<Cpudivselect> for u8 {
    #[inline(always)]
    fn from(variant: Cpudivselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpudivselect {
    type Ux = u8;
}
impl crate::IsEnum for Cpudivselect {}
#[doc = "Field `CPUDIV` reader - CPU Prescaler Selection"]
pub type CpudivR = crate::FieldReader<Cpudivselect>;
impl CpudivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpudivselect {
        match self.bits {
            0 => Cpudivselect::Div1,
            1 => Cpudivselect::Div2,
            2 => Cpudivselect::Div4,
            3 => Cpudivselect::Div8,
            4 => Cpudivselect::Div16,
            5 => Cpudivselect::Div32,
            6 => Cpudivselect::Div64,
            7 => Cpudivselect::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cpudivselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cpudivselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cpudivselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cpudivselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cpudivselect::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Cpudivselect::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Cpudivselect::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Cpudivselect::Div128
    }
}
#[doc = "Field `CPUDIV` writer - CPU Prescaler Selection"]
pub type CpudivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cpudivselect, crate::Safe>;
impl<'a, REG> CpudivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CpudivR {
        CpudivR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CpudivW<CpuselSpec> {
        CpudivW::new(self, 0)
    }
}
#[doc = "CPU Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`cpusel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpusel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuselSpec;
impl crate::RegisterSpec for CpuselSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpusel::R`](R) reader structure"]
impl crate::Readable for CpuselSpec {}
#[doc = "`write(|w| ..)` method takes [`cpusel::W`](W) writer structure"]
impl crate::Writable for CpuselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CPUSEL to value 0"]
impl crate::Resettable for CpuselSpec {
    const RESET_VALUE: u8 = 0;
}
