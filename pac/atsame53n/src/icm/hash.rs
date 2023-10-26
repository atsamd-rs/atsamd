#[doc = "Register `HASH` reader"]
pub type R = crate::R<HASH_SPEC>;
#[doc = "Register `HASH` writer"]
pub type W = crate::W<HASH_SPEC>;
#[doc = "Field `HASA` reader - Hash Area Start Address"]
pub type HASA_R = crate::FieldReader<u32>;
#[doc = "Field `HASA` writer - Hash Area Start Address"]
pub type HASA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 25, O, u32>;
impl R {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    pub fn hasa(&self) -> HASA_R {
        HASA_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn hasa(&mut self) -> HASA_W<HASH_SPEC, 7> {
        HASA_W::new(self)
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
#[doc = "Region Hash Area Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_SPEC;
impl crate::RegisterSpec for HASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash::R`](R) reader structure"]
impl crate::Readable for HASH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hash::W`](W) writer structure"]
impl crate::Writable for HASH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH to value 0"]
impl crate::Resettable for HASH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
