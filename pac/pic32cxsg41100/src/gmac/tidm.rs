#[doc = "Register `TIDM[%s]` reader"]
pub type R = crate::R<TidmSpec>;
#[doc = "Register `TIDM[%s]` writer"]
pub type W = crate::W<TidmSpec>;
#[doc = "Field `TID` reader - Type ID Match n"]
pub type TidR = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match n"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENID` reader - Enable Copying of TID n Matched Frames"]
pub type EnidR = crate::BitReader;
#[doc = "Field `ENID` writer - Enable Copying of TID n Matched Frames"]
pub type EnidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match n"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID n Matched Frames"]
    #[inline(always)]
    pub fn enid(&self) -> EnidR {
        EnidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match n"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TidW<TidmSpec> {
        TidW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable Copying of TID n Matched Frames"]
    #[inline(always)]
    #[must_use]
    pub fn enid(&mut self) -> EnidW<TidmSpec> {
        EnidW::new(self, 31)
    }
}
#[doc = "Type ID Match n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TidmSpec;
impl crate::RegisterSpec for TidmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm::R`](R) reader structure"]
impl crate::Readable for TidmSpec {}
#[doc = "`write(|w| ..)` method takes [`tidm::W`](W) writer structure"]
impl crate::Writable for TidmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIDM[%s]
to value 0"]
impl crate::Resettable for TidmSpec {
    const RESET_VALUE: u32 = 0;
}
