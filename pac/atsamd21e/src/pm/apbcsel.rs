#[doc = "Register `APBCSEL` reader"]
pub type R = crate::R<APBCSEL_SPEC>;
#[doc = "Register `APBCSEL` writer"]
pub type W = crate::W<APBCSEL_SPEC>;
#[doc = "Field `APBCDIV` reader - APBC Prescaler Selection"]
pub type APBCDIV_R = crate::FieldReader<APBCDIVSELECT_A>;
#[doc = "APBC Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APBCDIVSELECT_A {
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
impl From<APBCDIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: APBCDIVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APBCDIVSELECT_A {
    type Ux = u8;
}
impl APBCDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APBCDIVSELECT_A {
        match self.bits {
            0 => APBCDIVSELECT_A::DIV1,
            1 => APBCDIVSELECT_A::DIV2,
            2 => APBCDIVSELECT_A::DIV4,
            3 => APBCDIVSELECT_A::DIV8,
            4 => APBCDIVSELECT_A::DIV16,
            5 => APBCDIVSELECT_A::DIV32,
            6 => APBCDIVSELECT_A::DIV64,
            7 => APBCDIVSELECT_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == APBCDIVSELECT_A::DIV128
    }
}
#[doc = "Field `APBCDIV` writer - APBC Prescaler Selection"]
pub type APBCDIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, APBCDIVSELECT_A>;
impl<'a, REG, const O: u8> APBCDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(APBCDIVSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:2 - APBC Prescaler Selection"]
    #[inline(always)]
    pub fn apbcdiv(&self) -> APBCDIV_R {
        APBCDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBC Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn apbcdiv(&mut self) -> APBCDIV_W<APBCSEL_SPEC, 0> {
        APBCDIV_W::new(self)
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
#[doc = "APBC Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBCSEL_SPEC;
impl crate::RegisterSpec for APBCSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbcsel::R`](R) reader structure"]
impl crate::Readable for APBCSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbcsel::W`](W) writer structure"]
impl crate::Writable for APBCSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCSEL to value 0"]
impl crate::Resettable for APBCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
