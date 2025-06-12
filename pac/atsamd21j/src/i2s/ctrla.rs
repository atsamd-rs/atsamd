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
#[doc = "Field `CKEN0` reader - Clock Unit 0 Enable"]
pub type Cken0R = crate::BitReader;
#[doc = "Field `CKEN0` writer - Clock Unit 0 Enable"]
pub type Cken0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN1` reader - Clock Unit 1 Enable"]
pub type Cken1R = crate::BitReader;
#[doc = "Field `CKEN1` writer - Clock Unit 1 Enable"]
pub type Cken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEREN0` reader - Serializer 0 Enable"]
pub type Seren0R = crate::BitReader;
#[doc = "Field `SEREN0` writer - Serializer 0 Enable"]
pub type Seren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEREN1` reader - Serializer 1 Enable"]
pub type Seren1R = crate::BitReader;
#[doc = "Field `SEREN1` writer - Serializer 1 Enable"]
pub type Seren1W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Clock Unit 0 Enable"]
    #[inline(always)]
    pub fn cken0(&self) -> Cken0R {
        Cken0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable"]
    #[inline(always)]
    pub fn cken1(&self) -> Cken1R {
        Cken1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serializer 0 Enable"]
    #[inline(always)]
    pub fn seren0(&self) -> Seren0R {
        Seren0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serializer 1 Enable"]
    #[inline(always)]
    pub fn seren1(&self) -> Seren1R {
        Seren1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable"]
    #[inline(always)]
    pub fn cken0(&mut self) -> Cken0W<CtrlaSpec> {
        Cken0W::new(self, 2)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable"]
    #[inline(always)]
    pub fn cken1(&mut self) -> Cken1W<CtrlaSpec> {
        Cken1W::new(self, 3)
    }
    #[doc = "Bit 4 - Serializer 0 Enable"]
    #[inline(always)]
    pub fn seren0(&mut self) -> Seren0W<CtrlaSpec> {
        Seren0W::new(self, 4)
    }
    #[doc = "Bit 5 - Serializer 1 Enable"]
    #[inline(always)]
    pub fn seren1(&mut self) -> Seren1W<CtrlaSpec> {
        Seren1W::new(self, 5)
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
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
