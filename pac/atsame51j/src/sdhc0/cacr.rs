#[doc = "Register `CACR` reader"]
pub type R = crate::R<CacrSpec>;
#[doc = "Register `CACR` writer"]
pub type W = crate::W<CacrSpec>;
#[doc = "Field `CAPWREN` reader - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CapwrenR = crate::BitReader;
#[doc = "Field `CAPWREN` writer - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CapwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - Key (0x46)"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - Key (0x46)"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    pub fn capwren(&self) -> CapwrenR {
        CapwrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    #[must_use]
    pub fn capwren(&mut self) -> CapwrenW<CacrSpec> {
        CapwrenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CacrSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Capabilities Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacrSpec;
impl crate::RegisterSpec for CacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacr::R`](R) reader structure"]
impl crate::Readable for CacrSpec {}
#[doc = "`write(|w| ..)` method takes [`cacr::W`](W) writer structure"]
impl crate::Writable for CacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CacrSpec {
    const RESET_VALUE: u32 = 0;
}
