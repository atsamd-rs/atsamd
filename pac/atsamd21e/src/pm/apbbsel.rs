#[doc = "Register `APBBSEL` reader"]
pub type R = crate::R<APBBSEL_SPEC>;
#[doc = "Register `APBBSEL` writer"]
pub type W = crate::W<APBBSEL_SPEC>;
#[doc = "Field `APBBDIV` reader - APBB Prescaler Selection"]
pub type APBBDIV_R = crate::FieldReader<APBBDIVSELECT_A>;
#[doc = "APBB Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APBBDIVSELECT_A {
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
impl From<APBBDIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: APBBDIVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APBBDIVSELECT_A {
    type Ux = u8;
}
impl APBBDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APBBDIVSELECT_A {
        match self.bits {
            0 => APBBDIVSELECT_A::DIV1,
            1 => APBBDIVSELECT_A::DIV2,
            2 => APBBDIVSELECT_A::DIV4,
            3 => APBBDIVSELECT_A::DIV8,
            4 => APBBDIVSELECT_A::DIV16,
            5 => APBBDIVSELECT_A::DIV32,
            6 => APBBDIVSELECT_A::DIV64,
            7 => APBBDIVSELECT_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == APBBDIVSELECT_A::DIV128
    }
}
#[doc = "Field `APBBDIV` writer - APBB Prescaler Selection"]
pub type APBBDIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, APBBDIVSELECT_A>;
impl<'a, REG, const O: u8> APBBDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(APBBDIVSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline(always)]
    pub fn apbbdiv(&self) -> APBBDIV_R {
        APBBDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBB Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn apbbdiv(&mut self) -> APBBDIV_W<APBBSEL_SPEC, 0> {
        APBBDIV_W::new(self)
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
#[doc = "APBB Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbbsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbbsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBBSEL_SPEC;
impl crate::RegisterSpec for APBBSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbbsel::R`](R) reader structure"]
impl crate::Readable for APBBSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbbsel::W`](W) writer structure"]
impl crate::Writable for APBBSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBBSEL to value 0"]
impl crate::Resettable for APBBSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
