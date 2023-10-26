#[doc = "Register `INSTRADDR` reader"]
pub type R = crate::R<INSTRADDR_SPEC>;
#[doc = "Register `INSTRADDR` writer"]
pub type W = crate::W<INSTRADDR_SPEC>;
#[doc = "Field `ADDR` reader - Instruction Address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Instruction Address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Instruction Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Instruction Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<INSTRADDR_SPEC, 0> {
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
#[doc = "Instruction Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instraddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instraddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTRADDR_SPEC;
impl crate::RegisterSpec for INSTRADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instraddr::R`](R) reader structure"]
impl crate::Readable for INSTRADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`instraddr::W`](W) writer structure"]
impl crate::Writable for INSTRADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTRADDR to value 0"]
impl crate::Resettable for INSTRADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
