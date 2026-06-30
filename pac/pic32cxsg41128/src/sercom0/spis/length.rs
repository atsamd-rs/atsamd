#[doc = "Register `LENGTH` reader"]
pub type R = crate::R<LengthSpec>;
#[doc = "Register `LENGTH` writer"]
pub type W = crate::W<LengthSpec>;
#[doc = "Field `LEN` reader - Data Length"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - Data Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LENEN` reader - Data Length Enable"]
pub type LenenR = crate::BitReader;
#[doc = "Field `LENEN` writer - Data Length Enable"]
pub type LenenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LenenR {
        LenenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<LengthSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lenen(&mut self) -> LenenW<LengthSpec> {
        LenenW::new(self, 8)
    }
}
#[doc = "SPIS Length\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LengthSpec;
impl crate::RegisterSpec for LengthSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`length::R`](R) reader structure"]
impl crate::Readable for LengthSpec {}
#[doc = "`write(|w| ..)` method takes [`length::W`](W) writer structure"]
impl crate::Writable for LengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LengthSpec {
    const RESET_VALUE: u16 = 0;
}
