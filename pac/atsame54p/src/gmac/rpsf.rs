#[doc = "Register `RPSF` reader"]
pub type R = crate::R<RPSF_SPEC>;
#[doc = "Register `RPSF` writer"]
pub type W = crate::W<RPSF_SPEC>;
#[doc = "Field `RPB1ADR` reader - RX packet buffer address"]
pub type RPB1ADR_R = crate::FieldReader<u16>;
#[doc = "Field `RPB1ADR` writer - RX packet buffer address"]
pub type RPB1ADR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `ENRXP` reader - Enable RX partial store and forward operation"]
pub type ENRXP_R = crate::BitReader;
#[doc = "Field `ENRXP` writer - Enable RX partial store and forward operation"]
pub type ENRXP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    pub fn rpb1adr(&self) -> RPB1ADR_R {
        RPB1ADR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn enrxp(&self) -> ENRXP_R {
        ENRXP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn rpb1adr(&mut self) -> RPB1ADR_W<RPSF_SPEC, 0> {
        RPB1ADR_W::new(self)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn enrxp(&mut self) -> ENRXP_W<RPSF_SPEC, 31> {
        ENRXP_W::new(self)
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
#[doc = "RX partial store and forward Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpsf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpsf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPSF_SPEC;
impl crate::RegisterSpec for RPSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpsf::R`](R) reader structure"]
impl crate::Readable for RPSF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rpsf::W`](W) writer structure"]
impl crate::Writable for RPSF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPSF to value 0x03ff"]
impl crate::Resettable for RPSF_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
