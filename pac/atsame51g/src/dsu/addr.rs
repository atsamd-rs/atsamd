#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `AMOD` reader - Access Mode"]
pub type AMOD_R = crate::FieldReader;
#[doc = "Field `AMOD` writer - Access Mode"]
pub type AMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 0:1 - Access Mode"]
    #[inline(always)]
    pub fn amod(&self) -> AMOD_R {
        AMOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Access Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amod(&mut self) -> AMOD_W<ADDR_SPEC, 0> {
        AMOD_W::new(self)
    }
    #[doc = "Bits 2:31 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<ADDR_SPEC, 2> {
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
#[doc = "Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
