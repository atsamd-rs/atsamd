#[doc = "Register `CPUSEL` reader"]
pub type R = crate::R<CPUSEL_SPEC>;
#[doc = "Register `CPUSEL` writer"]
pub type W = crate::W<CPUSEL_SPEC>;
#[doc = "Field `CPUDIV` reader - CPU Prescaler Selection"]
pub type CPUDIV_R = crate::FieldReader<CPUDIVSELECT_A>;
#[doc = "CPU Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPUDIVSELECT_A {
    #[doc = "0: Divide by 1"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 4"]
    DIV4 = 2,
    #[doc = "3: Divide by 8"]
    DIV8 = 3,
    #[doc = "4: Divide by 16"]
    DIV16 = 4,
    #[doc = "5: Divide by 32"]
    DIV32 = 5,
    #[doc = "6: Divide by 64"]
    DIV64 = 6,
    #[doc = "7: Divide by 128"]
    DIV128 = 7,
}
impl From<CPUDIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUDIVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CPUDIVSELECT_A {
    type Ux = u8;
}
impl CPUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPUDIVSELECT_A {
        match self.bits {
            0 => CPUDIVSELECT_A::DIV1,
            1 => CPUDIVSELECT_A::DIV2,
            2 => CPUDIVSELECT_A::DIV4,
            3 => CPUDIVSELECT_A::DIV8,
            4 => CPUDIVSELECT_A::DIV16,
            5 => CPUDIVSELECT_A::DIV32,
            6 => CPUDIVSELECT_A::DIV64,
            7 => CPUDIVSELECT_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CPUDIVSELECT_A::DIV128
    }
}
#[doc = "Field `CPUDIV` writer - CPU Prescaler Selection"]
pub type CPUDIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CPUDIVSELECT_A>;
impl<'a, REG, const O: u8> CPUDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIVSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CPUDIV_W<CPUSEL_SPEC, 0> {
        CPUDIV_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CPU Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpusel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUSEL_SPEC;
impl crate::RegisterSpec for CPUSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpusel::R`](R) reader structure"]
impl crate::Readable for CPUSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpusel::W`](W) writer structure"]
impl crate::Writable for CPUSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUSEL to value 0"]
impl crate::Resettable for CPUSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
