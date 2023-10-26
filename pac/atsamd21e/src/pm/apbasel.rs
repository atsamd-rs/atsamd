#[doc = "Register `APBASEL` reader"]
pub type R = crate::R<APBASEL_SPEC>;
#[doc = "Register `APBASEL` writer"]
pub type W = crate::W<APBASEL_SPEC>;
#[doc = "Field `APBADIV` reader - APBA Prescaler Selection"]
pub type APBADIV_R = crate::FieldReader<APBADIVSELECT_A>;
#[doc = "APBA Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APBADIVSELECT_A {
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
impl From<APBADIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: APBADIVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APBADIVSELECT_A {
    type Ux = u8;
}
impl APBADIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APBADIVSELECT_A {
        match self.bits {
            0 => APBADIVSELECT_A::DIV1,
            1 => APBADIVSELECT_A::DIV2,
            2 => APBADIVSELECT_A::DIV4,
            3 => APBADIVSELECT_A::DIV8,
            4 => APBADIVSELECT_A::DIV16,
            5 => APBADIVSELECT_A::DIV32,
            6 => APBADIVSELECT_A::DIV64,
            7 => APBADIVSELECT_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == APBADIVSELECT_A::DIV1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == APBADIVSELECT_A::DIV2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == APBADIVSELECT_A::DIV4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == APBADIVSELECT_A::DIV8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == APBADIVSELECT_A::DIV16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == APBADIVSELECT_A::DIV32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == APBADIVSELECT_A::DIV64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == APBADIVSELECT_A::DIV128
    }
}
#[doc = "Field `APBADIV` writer - APBA Prescaler Selection"]
pub type APBADIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, APBADIVSELECT_A>;
impl<'a, REG, const O: u8> APBADIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(APBADIVSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline(always)]
    pub fn apbadiv(&self) -> APBADIV_R {
        APBADIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn apbadiv(&mut self) -> APBADIV_W<APBASEL_SPEC, 0> {
        APBADIV_W::new(self)
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
#[doc = "APBA Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbasel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbasel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBASEL_SPEC;
impl crate::RegisterSpec for APBASEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbasel::R`](R) reader structure"]
impl crate::Readable for APBASEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbasel::W`](W) writer structure"]
impl crate::Writable for APBASEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBASEL to value 0"]
impl crate::Resettable for APBASEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
