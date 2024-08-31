#[doc = "Register `FNUM` reader"]
pub type R = crate::R<FnumSpec>;
#[doc = "Register `FNUM` writer"]
pub type W = crate::W<FnumSpec>;
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MfnumR = crate::FieldReader;
#[doc = "Field `MFNUM` writer - Micro Frame Number"]
pub type MfnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FnumR = crate::FieldReader<u16>;
#[doc = "Field `FNUM` writer - Frame Number"]
pub type FnumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MfnumR {
        MfnumR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FnumR {
        FnumR::new((self.bits >> 3) & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn mfnum(&mut self) -> MfnumW<FnumSpec> {
        MfnumW::new(self, 0)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FnumW<FnumSpec> {
        FnumW::new(self, 3)
    }
}
#[doc = "HOST Host Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`fnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FnumSpec;
impl crate::RegisterSpec for FnumSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fnum::R`](R) reader structure"]
impl crate::Readable for FnumSpec {}
#[doc = "`write(|w| ..)` method takes [`fnum::W`](W) writer structure"]
impl crate::Writable for FnumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FnumSpec {
    const RESET_VALUE: u16 = 0;
}
