#[doc = "Register `LENGTH` reader"]
pub type R = crate::R<LengthSpec>;
#[doc = "Register `LENGTH` writer"]
pub type W = crate::W<LengthSpec>;
#[doc = "Field `LENGTH` reader - Length"]
pub type LengthR = crate::FieldReader<u32>;
#[doc = "Field `LENGTH` writer - Length"]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Length"]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Length"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LengthW<LengthSpec> {
        LengthW::new(self, 2)
    }
}
#[doc = "Length\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LengthSpec;
impl crate::RegisterSpec for LengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`length::R`](R) reader structure"]
impl crate::Readable for LengthSpec {}
#[doc = "`write(|w| ..)` method takes [`length::W`](W) writer structure"]
impl crate::Writable for LengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LengthSpec {
    const RESET_VALUE: u32 = 0;
}
