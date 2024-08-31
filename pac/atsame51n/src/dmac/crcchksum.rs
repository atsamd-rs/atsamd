#[doc = "Register `CRCCHKSUM` reader"]
pub type R = crate::R<CrcchksumSpec>;
#[doc = "Register `CRCCHKSUM` writer"]
pub type W = crate::W<CrcchksumSpec>;
#[doc = "Field `CRCCHKSUM` reader - CRC Checksum"]
pub type CrcchksumR = crate::FieldReader<u32>;
#[doc = "Field `CRCCHKSUM` writer - CRC Checksum"]
pub type CrcchksumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    pub fn crcchksum(&self) -> CrcchksumR {
        CrcchksumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    #[must_use]
    pub fn crcchksum(&mut self) -> CrcchksumW<CrcchksumSpec> {
        CrcchksumW::new(self, 0)
    }
}
#[doc = "CRC Checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`crcchksum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcchksum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcchksumSpec;
impl crate::RegisterSpec for CrcchksumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcchksum::R`](R) reader structure"]
impl crate::Readable for CrcchksumSpec {}
#[doc = "`write(|w| ..)` method takes [`crcchksum::W`](W) writer structure"]
impl crate::Writable for CrcchksumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCCHKSUM to value 0"]
impl crate::Resettable for CrcchksumSpec {
    const RESET_VALUE: u32 = 0;
}
