#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACR_SPEC>;
#[doc = "Field `BMAX` reader - AHB Maximum Burst"]
pub type BMAX_R = crate::FieldReader<BMAXSELECT_A>;
#[doc = "AHB Maximum Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMAXSELECT_A {
    #[doc = "0: `0`"]
    INCR16 = 0,
    #[doc = "1: `1`"]
    INCR8 = 1,
    #[doc = "2: `10`"]
    INCR4 = 2,
    #[doc = "3: `11`"]
    SINGLE = 3,
}
impl From<BMAXSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BMAXSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BMAXSELECT_A {
    type Ux = u8;
}
impl BMAX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BMAXSELECT_A {
        match self.bits {
            0 => BMAXSELECT_A::INCR16,
            1 => BMAXSELECT_A::INCR8,
            2 => BMAXSELECT_A::INCR4,
            3 => BMAXSELECT_A::SINGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == BMAXSELECT_A::INCR16
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == BMAXSELECT_A::INCR8
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == BMAXSELECT_A::INCR4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == BMAXSELECT_A::SINGLE
    }
}
#[doc = "Field `BMAX` writer - AHB Maximum Burst"]
pub type BMAX_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BMAXSELECT_A>;
impl<'a, REG, const O: u8> BMAX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(BMAXSELECT_A::INCR16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(BMAXSELECT_A::INCR8)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(BMAXSELECT_A::INCR4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(BMAXSELECT_A::SINGLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&self) -> BMAX_R {
        BMAX_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    #[must_use]
    pub fn bmax(&mut self) -> BMAX_W<ACR_SPEC, 0> {
        BMAX_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
