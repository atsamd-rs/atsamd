#[doc = "Register `PINCFG[%s]` reader"]
pub type R = crate::R<PINCFG_SPEC>;
#[doc = "Register `PINCFG[%s]` writer"]
pub type W = crate::W<PINCFG_SPEC>;
#[doc = "Field `PMUXEN` reader - Peripheral Multiplexer Enable"]
pub type PMUXEN_R = crate::BitReader;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PMUXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEN` reader - Input Enable"]
pub type INEN_R = crate::BitReader;
#[doc = "Field `INEN` writer - Input Enable"]
pub type INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PULLEN` reader - Pull Enable"]
pub type PULLEN_R = crate::BitReader;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PULLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRVSTR` reader - Output Driver Strength Selection"]
pub type DRVSTR_R = crate::BitReader;
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub type DRVSTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PMUXEN_R {
        PMUXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PULLEN_R {
        PULLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&self) -> DRVSTR_R {
        DRVSTR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxen(&mut self) -> PMUXEN_W<PINCFG_SPEC, 0> {
        PMUXEN_W::new(self)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inen(&mut self) -> INEN_W<PINCFG_SPEC, 1> {
        INEN_W::new(self)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pullen(&mut self) -> PULLEN_W<PINCFG_SPEC, 2> {
        PULLEN_W::new(self)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    #[must_use]
    pub fn drvstr(&mut self) -> DRVSTR_W<PINCFG_SPEC, 6> {
        DRVSTR_W::new(self)
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
#[doc = "Pin Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pincfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pincfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINCFG_SPEC;
impl crate::RegisterSpec for PINCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pincfg::R`](R) reader structure"]
impl crate::Readable for PINCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pincfg::W`](W) writer structure"]
impl crate::Writable for PINCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINCFG[%s]
to value 0"]
impl crate::Resettable for PINCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
