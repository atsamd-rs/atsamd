#[doc = "Register `SAMT1` reader"]
pub type R = crate::R<SAMT1_SPEC>;
#[doc = "Register `SAMT1` writer"]
pub type W = crate::W<SAMT1_SPEC>;
#[doc = "Field `ADDR` reader - Specific Address 1 Mask"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Specific Address 1 Mask"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Specific Address 1 Mask"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address 1 Mask"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SAMT1_SPEC, 0> {
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
#[doc = "Specific Address 1 Mask Top \\[47:32\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMT1_SPEC;
impl crate::RegisterSpec for SAMT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samt1::R`](R) reader structure"]
impl crate::Readable for SAMT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`samt1::W`](W) writer structure"]
impl crate::Writable for SAMT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMT1 to value 0"]
impl crate::Resettable for SAMT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
