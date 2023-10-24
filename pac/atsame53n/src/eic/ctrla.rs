#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKSEL` reader - Clock Selection"]
pub type CKSEL_R = crate::BitReader<CKSELSELECT_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSELSELECT_A {
    #[doc = "0: Clocked by GCLK"]
    CLK_GCLK = 0,
    #[doc = "1: Clocked by ULP32K"]
    CLK_ULP32K = 1,
}
impl From<CKSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CKSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKSELSELECT_A {
        match self.bits {
            false => CKSELSELECT_A::CLK_GCLK,
            true => CKSELSELECT_A::CLK_ULP32K,
        }
    }
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn is_clk_gclk(&self) -> bool {
        *self == CKSELSELECT_A::CLK_GCLK
    }
    #[doc = "Clocked by ULP32K"]
    #[inline(always)]
    pub fn is_clk_ulp32k(&self) -> bool {
        *self == CKSELSELECT_A::CLK_ULP32K
    }
}
#[doc = "Field `CKSEL` writer - Clock Selection"]
pub type CKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CKSELSELECT_A>;
impl<'a, REG, const O: u8> CKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn clk_gclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKSELSELECT_A::CLK_GCLK)
    }
    #[doc = "Clocked by ULP32K"]
    #[inline(always)]
    pub fn clk_ulp32k(self) -> &'a mut crate::W<REG> {
        self.variant(CKSELSELECT_A::CLK_ULP32K)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTRLA_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<CTRLA_SPEC, 4> {
        CKSEL_W::new(self)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
