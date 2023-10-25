#[doc = "Register `CHINTFLAG` reader"]
pub type R = crate::R<CHINTFLAG_SPEC>;
#[doc = "Register `CHINTFLAG` writer"]
pub type W = crate::W<CHINTFLAG_SPEC>;
#[doc = "Field `TERR` reader - Channel Transfer Error"]
pub type TERR_R = crate::BitReader;
#[doc = "Field `TERR` writer - Channel Transfer Error"]
pub type TERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCMPL` reader - Channel Transfer Complete"]
pub type TCMPL_R = crate::BitReader;
#[doc = "Field `TCMPL` writer - Channel Transfer Complete"]
pub type TCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub type SUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<CHINTFLAG_SPEC, 0> {
        TERR_W::new(self)
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpl(&mut self) -> TCMPL_W<CHINTFLAG_SPEC, 1> {
        TCMPL_W::new(self)
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<CHINTFLAG_SPEC, 2> {
        SUSP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chintflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chintflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHINTFLAG_SPEC;
impl crate::RegisterSpec for CHINTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chintflag::R`](R) reader structure"]
impl crate::Readable for CHINTFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chintflag::W`](W) writer structure"]
impl crate::Writable for CHINTFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHINTFLAG to value 0"]
impl crate::Resettable for CHINTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
