#[doc = "Register `MMFAR` reader"]
pub type R = crate::R<MMFAR_SPEC>;
#[doc = "Register `MMFAR` writer"]
pub type W = crate::W<MMFAR_SPEC>;
#[doc = "Field `ADDRESS` reader - Address that generated the MemManage fault"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - Address that generated the MemManage fault"]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Address that generated the MemManage fault"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address that generated the MemManage fault"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<MMFAR_SPEC, 0> {
        ADDRESS_W::new(self)
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
#[doc = "MemManage Fault Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmfar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmfar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMFAR_SPEC;
impl crate::RegisterSpec for MMFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmfar::R`](R) reader structure"]
impl crate::Readable for MMFAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmfar::W`](W) writer structure"]
impl crate::Writable for MMFAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMFAR to value 0"]
impl crate::Resettable for MMFAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
