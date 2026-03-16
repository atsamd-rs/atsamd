#[doc = "Register `TPSF` reader"]
pub type R = crate::R<TpsfSpec>;
#[doc = "Register `TPSF` writer"]
pub type W = crate::W<TpsfSpec>;
#[doc = "Field `TPB1ADR` reader - TX packet buffer address"]
pub type Tpb1adrR = crate::FieldReader<u16>;
#[doc = "Field `TPB1ADR` writer - TX packet buffer address"]
pub type Tpb1adrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ENTXP` reader - Enable TX partial store and forward operation"]
pub type EntxpR = crate::BitReader;
#[doc = "Field `ENTXP` writer - Enable TX partial store and forward operation"]
pub type EntxpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    pub fn tpb1adr(&self) -> Tpb1adrR {
        Tpb1adrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn entxp(&self) -> EntxpR {
        EntxpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn tpb1adr(&mut self) -> Tpb1adrW<TpsfSpec> {
        Tpb1adrW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn entxp(&mut self) -> EntxpW<TpsfSpec> {
        EntxpW::new(self, 31)
    }
}
#[doc = "TX partial store and forward Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpsf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpsf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpsfSpec;
impl crate::RegisterSpec for TpsfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpsf::R`](R) reader structure"]
impl crate::Readable for TpsfSpec {}
#[doc = "`write(|w| ..)` method takes [`tpsf::W`](W) writer structure"]
impl crate::Writable for TpsfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPSF to value 0x03ff"]
impl crate::Resettable for TpsfSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
