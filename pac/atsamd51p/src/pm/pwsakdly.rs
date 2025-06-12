#[doc = "Register `PWSAKDLY` reader"]
pub type R = crate::R<PwsakdlySpec>;
#[doc = "Register `PWSAKDLY` writer"]
pub type W = crate::W<PwsakdlySpec>;
#[doc = "Field `DLYVAL` reader - Delay Value"]
pub type DlyvalR = crate::FieldReader;
#[doc = "Field `DLYVAL` writer - Delay Value"]
pub type DlyvalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IGNACK` reader - Ignore Acknowledge"]
pub type IgnackR = crate::BitReader;
#[doc = "Field `IGNACK` writer - Ignore Acknowledge"]
pub type IgnackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&self) -> DlyvalR {
        DlyvalR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&self) -> IgnackR {
        IgnackR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&mut self) -> DlyvalW<PwsakdlySpec> {
        DlyvalW::new(self, 0)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IgnackW<PwsakdlySpec> {
        IgnackW::new(self, 7)
    }
}
#[doc = "Power Switch Acknowledge Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`pwsakdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwsakdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwsakdlySpec;
impl crate::RegisterSpec for PwsakdlySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwsakdly::R`](R) reader structure"]
impl crate::Readable for PwsakdlySpec {}
#[doc = "`write(|w| ..)` method takes [`pwsakdly::W`](W) writer structure"]
impl crate::Writable for PwsakdlySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWSAKDLY to value 0"]
impl crate::Resettable for PwsakdlySpec {}
