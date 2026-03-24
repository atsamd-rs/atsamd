#[doc = "Register `EPCFG` reader"]
pub type R = crate::R<EpcfgSpec>;
#[doc = "Register `EPCFG` writer"]
pub type W = crate::W<EpcfgSpec>;
#[doc = "Field `EPTYPE0` reader - End Point Type0"]
pub type Eptype0R = crate::FieldReader;
#[doc = "Field `EPTYPE0` writer - End Point Type0"]
pub type Eptype0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EPTYPE1` reader - End Point Type1"]
pub type Eptype1R = crate::FieldReader;
#[doc = "Field `EPTYPE1` writer - End Point Type1"]
pub type Eptype1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&self) -> Eptype0R {
        Eptype0R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&self) -> Eptype1R {
        Eptype1R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    #[must_use]
    pub fn eptype0(&mut self) -> Eptype0W<EpcfgSpec> {
        Eptype0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    #[must_use]
    pub fn eptype1(&mut self) -> Eptype1W<EpcfgSpec> {
        Eptype1W::new(self, 4)
    }
}
#[doc = "DEVICE_ENDPOINT End Point Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`epcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpcfgSpec;
impl crate::RegisterSpec for EpcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epcfg::R`](R) reader structure"]
impl crate::Readable for EpcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`epcfg::W`](W) writer structure"]
impl crate::Writable for EpcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EPCFG to value 0"]
impl crate::Resettable for EpcfgSpec {
    const RESET_VALUE: u8 = 0;
}
