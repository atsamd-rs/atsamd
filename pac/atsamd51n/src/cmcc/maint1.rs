#[doc = "Register `MAINT1` writer"]
pub type W = crate::W<Maint1Spec>;
#[doc = "Field `INDEX` writer - Invalidate Index"]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Invalidate Way\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wayselect {
    #[doc = "0: Way 0 is selection for index invalidation"]
    Way0 = 0,
    #[doc = "1: Way 1 is selection for index invalidation"]
    Way1 = 1,
    #[doc = "2: Way 2 is selection for index invalidation"]
    Way2 = 2,
    #[doc = "3: Way 3 is selection for index invalidation"]
    Way3 = 3,
}
impl From<Wayselect> for u8 {
    #[inline(always)]
    fn from(variant: Wayselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wayselect {
    type Ux = u8;
}
impl crate::IsEnum for Wayselect {}
#[doc = "Field `WAY` writer - Invalidate Way"]
pub type WayW<'a, REG> = crate::FieldWriter<'a, REG, 4, Wayselect>;
impl<'a, REG> WayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Way 0 is selection for index invalidation"]
    #[inline(always)]
    pub fn way0(self) -> &'a mut crate::W<REG> {
        self.variant(Wayselect::Way0)
    }
    #[doc = "Way 1 is selection for index invalidation"]
    #[inline(always)]
    pub fn way1(self) -> &'a mut crate::W<REG> {
        self.variant(Wayselect::Way1)
    }
    #[doc = "Way 2 is selection for index invalidation"]
    #[inline(always)]
    pub fn way2(self) -> &'a mut crate::W<REG> {
        self.variant(Wayselect::Way2)
    }
    #[doc = "Way 3 is selection for index invalidation"]
    #[inline(always)]
    pub fn way3(self) -> &'a mut crate::W<REG> {
        self.variant(Wayselect::Way3)
    }
}
impl W {
    #[doc = "Bits 4:11 - Invalidate Index"]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<Maint1Spec> {
        IndexW::new(self, 4)
    }
    #[doc = "Bits 28:31 - Invalidate Way"]
    #[inline(always)]
    pub fn way(&mut self) -> WayW<Maint1Spec> {
        WayW::new(self, 28)
    }
}
#[doc = "Cache Maintenance Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maint1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maint1Spec;
impl crate::RegisterSpec for Maint1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`maint1::W`](W) writer structure"]
impl crate::Writable for Maint1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAINT1 to value 0"]
impl crate::Resettable for Maint1Spec {}
