#[doc = "Register `SHPR3` reader"]
pub type R = crate::R<Shpr3Spec>;
#[doc = "Register `SHPR3` writer"]
pub type W = crate::W<Shpr3Spec>;
#[doc = "Field `PRI_14` reader - Priority of system handler 14, PendSV"]
pub type Pri14R = crate::FieldReader;
#[doc = "Field `PRI_14` writer - Priority of system handler 14, PendSV"]
pub type Pri14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_15` reader - Priority of system handler 15, SysTick exception"]
pub type Pri15R = crate::FieldReader;
#[doc = "Field `PRI_15` writer - Priority of system handler 15, SysTick exception"]
pub type Pri15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - Priority of system handler 14, PendSV"]
    #[inline(always)]
    pub fn pri_14(&self) -> Pri14R {
        Pri14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priority of system handler 15, SysTick exception"]
    #[inline(always)]
    pub fn pri_15(&self) -> Pri15R {
        Pri15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Priority of system handler 14, PendSV"]
    #[inline(always)]
    #[must_use]
    pub fn pri_14(&mut self) -> Pri14W<Shpr3Spec> {
        Pri14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Priority of system handler 15, SysTick exception"]
    #[inline(always)]
    #[must_use]
    pub fn pri_15(&mut self) -> Pri15W<Shpr3Spec> {
        Pri15W::new(self, 24)
    }
}
#[doc = "System Handler Priority Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`shpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shpr3Spec;
impl crate::RegisterSpec for Shpr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr3::R`](R) reader structure"]
impl crate::Readable for Shpr3Spec {}
#[doc = "`write(|w| ..)` method takes [`shpr3::W`](W) writer structure"]
impl crate::Writable for Shpr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR3 to value 0"]
impl crate::Resettable for Shpr3Spec {
    const RESET_VALUE: u32 = 0;
}
