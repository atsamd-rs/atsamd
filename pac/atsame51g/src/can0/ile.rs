#[doc = "Register `ILE` reader"]
pub type R = crate::R<ILE_SPEC>;
#[doc = "Register `ILE` writer"]
pub type W = crate::W<ILE_SPEC>;
#[doc = "Field `EINT0` reader - Enable Interrupt Line 0"]
pub type EINT0_R = crate::BitReader;
#[doc = "Field `EINT0` writer - Enable Interrupt Line 0"]
pub type EINT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EINT1` reader - Enable Interrupt Line 1"]
pub type EINT1_R = crate::BitReader;
#[doc = "Field `EINT1` writer - Enable Interrupt Line 1"]
pub type EINT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> EINT0_W<ILE_SPEC, 0> {
        EINT0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> EINT1_W<ILE_SPEC, 1> {
        EINT1_W::new(self)
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
#[doc = "Interrupt Line Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILE_SPEC;
impl crate::RegisterSpec for ILE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ile::R`](R) reader structure"]
impl crate::Readable for ILE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ile::W`](W) writer structure"]
impl crate::Writable for ILE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ILE to value 0"]
impl crate::Resettable for ILE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
