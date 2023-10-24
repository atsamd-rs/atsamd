#[doc = "Register `TPSF` reader"]
pub type R = crate::R<TPSF_SPEC>;
#[doc = "Register `TPSF` writer"]
pub type W = crate::W<TPSF_SPEC>;
#[doc = "Field `TPB1ADR` reader - TX packet buffer address"]
pub type TPB1ADR_R = crate::FieldReader<u16>;
#[doc = "Field `TPB1ADR` writer - TX packet buffer address"]
pub type TPB1ADR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `ENTXP` reader - Enable TX partial store and forward operation"]
pub type ENTXP_R = crate::BitReader;
#[doc = "Field `ENTXP` writer - Enable TX partial store and forward operation"]
pub type ENTXP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    pub fn tpb1adr(&self) -> TPB1ADR_R {
        TPB1ADR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn entxp(&self) -> ENTXP_R {
        ENTXP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn tpb1adr(&mut self) -> TPB1ADR_W<TPSF_SPEC, 0> {
        TPB1ADR_W::new(self)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn entxp(&mut self) -> ENTXP_W<TPSF_SPEC, 31> {
        ENTXP_W::new(self)
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
#[doc = "TX partial store and forward Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpsf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpsf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPSF_SPEC;
impl crate::RegisterSpec for TPSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpsf::R`](R) reader structure"]
impl crate::Readable for TPSF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpsf::W`](W) writer structure"]
impl crate::Writable for TPSF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPSF to value 0x03ff"]
impl crate::Resettable for TPSF_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
