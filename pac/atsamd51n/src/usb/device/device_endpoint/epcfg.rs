#[doc = "Register `EPCFG` reader"]
pub type R = crate::R<EPCFG_SPEC>;
#[doc = "Register `EPCFG` writer"]
pub type W = crate::W<EPCFG_SPEC>;
#[doc = "Field `EPTYPE0` reader - End Point Type0"]
pub type EPTYPE0_R = crate::FieldReader;
#[doc = "Field `EPTYPE0` writer - End Point Type0"]
pub type EPTYPE0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `EPTYPE1` reader - End Point Type1"]
pub type EPTYPE1_R = crate::FieldReader;
#[doc = "Field `EPTYPE1` writer - End Point Type1"]
pub type EPTYPE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `NYETDIS` reader - NYET Token Disable"]
pub type NYETDIS_R = crate::BitReader;
#[doc = "Field `NYETDIS` writer - NYET Token Disable"]
pub type NYETDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&self) -> EPTYPE0_R {
        EPTYPE0_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&self) -> EPTYPE1_R {
        EPTYPE1_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    #[must_use]
    pub fn eptype0(&mut self) -> EPTYPE0_W<EPCFG_SPEC, 0> {
        EPTYPE0_W::new(self)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    #[must_use]
    pub fn eptype1(&mut self) -> EPTYPE1_W<EPCFG_SPEC, 4> {
        EPTYPE1_W::new(self)
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdis(&mut self) -> NYETDIS_W<EPCFG_SPEC, 7> {
        NYETDIS_W::new(self)
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
#[doc = "DEVICE_ENDPOINT End Point Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPCFG_SPEC;
impl crate::RegisterSpec for EPCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epcfg::R`](R) reader structure"]
impl crate::Readable for EPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epcfg::W`](W) writer structure"]
impl crate::Writable for EPCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPCFG to value 0"]
impl crate::Resettable for EPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
