#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckselselect {
    #[doc = "0: Clocked by GCLK"]
    ClkGclk = 0,
    #[doc = "1: Clocked by ULP32K"]
    ClkUlp32k = 1,
}
impl From<Ckselselect> for bool {
    #[inline(always)]
    fn from(variant: Ckselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSEL` reader - Clock Selection"]
pub type CkselR = crate::BitReader<Ckselselect>;
impl CkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckselselect {
        match self.bits {
            false => Ckselselect::ClkGclk,
            true => Ckselselect::ClkUlp32k,
        }
    }
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn is_clk_gclk(&self) -> bool {
        *self == Ckselselect::ClkGclk
    }
    #[doc = "Clocked by ULP32K"]
    #[inline(always)]
    pub fn is_clk_ulp32k(&self) -> bool {
        *self == Ckselselect::ClkUlp32k
    }
}
#[doc = "Field `CKSEL` writer - Clock Selection"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG, Ckselselect>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn clk_gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckselselect::ClkGclk)
    }
    #[doc = "Clocked by ULP32K"]
    #[inline(always)]
    pub fn clk_ulp32k(self) -> &'a mut crate::W<REG> {
        self.variant(Ckselselect::ClkUlp32k)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 4 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CkselW<CtrlaSpec> {
        CkselW::new(self, 4)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u8 = 0;
}
