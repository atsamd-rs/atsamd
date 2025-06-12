#[doc = "Register `BBPS` reader"]
pub type R = crate::R<BbpsSpec>;
#[doc = "Register `BBPS` writer"]
pub type W = crate::W<BbpsSpec>;
#[doc = "Battery Backup Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Confselect {
    #[doc = "0: The power switch is handled by the BOD33"]
    Bod33 = 0,
    #[doc = "1: In Backup Domain, the backup domain is always supplied by battery backup power"]
    Forced = 1,
}
impl From<Confselect> for bool {
    #[inline(always)]
    fn from(variant: Confselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONF` reader - Battery Backup Configuration"]
pub type ConfR = crate::BitReader<Confselect>;
impl ConfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Confselect {
        match self.bits {
            false => Confselect::Bod33,
            true => Confselect::Forced,
        }
    }
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn is_bod33(&self) -> bool {
        *self == Confselect::Bod33
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == Confselect::Forced
    }
}
#[doc = "Field `CONF` writer - Battery Backup Configuration"]
pub type ConfW<'a, REG> = crate::BitWriter<'a, REG, Confselect>;
impl<'a, REG> ConfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn bod33(self) -> &'a mut crate::W<REG> {
        self.variant(Confselect::Bod33)
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(Confselect::Forced)
    }
}
#[doc = "Field `WAKEEN` reader - Wake Enable"]
pub type WakeenR = crate::BitReader;
#[doc = "Field `WAKEEN` writer - Wake Enable"]
pub type WakeenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> ConfR {
        ConfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&self) -> WakeenR {
        WakeenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&mut self) -> ConfW<BbpsSpec> {
        ConfW::new(self, 0)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WakeenW<BbpsSpec> {
        WakeenW::new(self, 2)
    }
}
#[doc = "Battery Backup Power Switch\n\nYou can [`read`](crate::Reg::read) this register and get [`bbps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BbpsSpec;
impl crate::RegisterSpec for BbpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bbps::R`](R) reader structure"]
impl crate::Readable for BbpsSpec {}
#[doc = "`write(|w| ..)` method takes [`bbps::W`](W) writer structure"]
impl crate::Writable for BbpsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BBPS to value 0"]
impl crate::Resettable for BbpsSpec {}
