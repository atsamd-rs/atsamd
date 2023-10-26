#[doc = "Register `SAB` reader"]
pub type R = crate::R<SAB_SPEC>;
#[doc = "Register `SAB` writer"]
pub type W = crate::W<SAB_SPEC>;
#[doc = "Field `ADDR` reader - Specific Address 1"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Specific Address 1"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Specific Address 1"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specific Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SAB_SPEC, 0> {
        ADDR_W::new(self)
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
#[doc = "Specific Address Bottom \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sab::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sab::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAB_SPEC;
impl crate::RegisterSpec for SAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sab::R`](R) reader structure"]
impl crate::Readable for SAB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sab::W`](W) writer structure"]
impl crate::Writable for SAB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAB to value 0"]
impl crate::Resettable for SAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
