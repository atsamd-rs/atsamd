#[doc = "Register `RPSF` reader"]
pub type R = crate::R<RpsfSpec>;
#[doc = "Register `RPSF` writer"]
pub type W = crate::W<RpsfSpec>;
#[doc = "Field `RPB1ADR` reader - RX packet buffer address"]
pub type Rpb1adrR = crate::FieldReader<u16>;
#[doc = "Field `RPB1ADR` writer - RX packet buffer address"]
pub type Rpb1adrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ENRXP` reader - Enable RX partial store and forward operation"]
pub type EnrxpR = crate::BitReader;
#[doc = "Field `ENRXP` writer - Enable RX partial store and forward operation"]
pub type EnrxpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    pub fn rpb1adr(&self) -> Rpb1adrR {
        Rpb1adrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn enrxp(&self) -> EnrxpR {
        EnrxpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    pub fn rpb1adr(&mut self) -> Rpb1adrW<RpsfSpec> {
        Rpb1adrW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn enrxp(&mut self) -> EnrxpW<RpsfSpec> {
        EnrxpW::new(self, 31)
    }
}
#[doc = "RX partial store and forward Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpsf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpsf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpsfSpec;
impl crate::RegisterSpec for RpsfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpsf::R`](R) reader structure"]
impl crate::Readable for RpsfSpec {}
#[doc = "`write(|w| ..)` method takes [`rpsf::W`](W) writer structure"]
impl crate::Writable for RpsfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RPSF to value 0x03ff"]
impl crate::Resettable for RpsfSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
