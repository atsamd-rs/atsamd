#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CFDEN_R = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CFDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKUPCLK` reader - Backup Clock Select"]
pub type BKUPCLK_R = crate::BitReader;
#[doc = "Field `BKUPCLK` writer - Backup Clock Select"]
pub type BKUPCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup Clock Select"]
    #[inline(always)]
    pub fn bkupclk(&self) -> BKUPCLK_R {
        BKUPCLK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<CTRL_SPEC, 2> {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 4 - Backup Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn bkupclk(&mut self) -> BKUPCLK_W<CTRL_SPEC, 4> {
        BKUPCLK_W::new(self)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
