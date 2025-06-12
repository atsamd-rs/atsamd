#[doc = "Register `SHPR1` reader"]
pub type R = crate::R<Shpr1Spec>;
#[doc = "Register `SHPR1` writer"]
pub type W = crate::W<Shpr1Spec>;
#[doc = "Field `PRI_4` reader - Priority of system handler 4, MemManage"]
pub type Pri4R = crate::FieldReader;
#[doc = "Field `PRI_4` writer - Priority of system handler 4, MemManage"]
pub type Pri4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_5` reader - Priority of system handler 5, BusFault"]
pub type Pri5R = crate::FieldReader;
#[doc = "Field `PRI_5` writer - Priority of system handler 5, BusFault"]
pub type Pri5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_6` reader - Priority of system handler 6, UsageFault"]
pub type Pri6R = crate::FieldReader;
#[doc = "Field `PRI_6` writer - Priority of system handler 6, UsageFault"]
pub type Pri6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub fn pri_4(&self) -> Pri4R {
        Pri4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&self) -> Pri5R {
        Pri5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&self) -> Pri6R {
        Pri6R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub fn pri_4(&mut self) -> Pri4W<Shpr1Spec> {
        Pri4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&mut self) -> Pri5W<Shpr1Spec> {
        Pri5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&mut self) -> Pri6W<Shpr1Spec> {
        Pri6W::new(self, 16)
    }
}
#[doc = "System Handler Priority Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`shpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shpr1Spec;
impl crate::RegisterSpec for Shpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr1::R`](R) reader structure"]
impl crate::Readable for Shpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`shpr1::W`](W) writer structure"]
impl crate::Writable for Shpr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHPR1 to value 0"]
impl crate::Resettable for Shpr1Spec {}
