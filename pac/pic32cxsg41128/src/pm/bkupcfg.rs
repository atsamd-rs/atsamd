#[doc = "Register `BKUPCFG` reader"]
pub type R = crate::R<BkupcfgSpec>;
#[doc = "Register `BKUPCFG` writer"]
pub type W = crate::W<BkupcfgSpec>;
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bramcfgselect {
    #[doc = "0: All the backup RAM is retained"]
    Ret = 0,
    #[doc = "1: Only the first 4Kbytes of the backup RAM is retained"]
    Partial = 1,
    #[doc = "2: All the backup RAM is turned OFF"]
    Off = 2,
}
impl From<Bramcfgselect> for u8 {
    #[inline(always)]
    fn from(variant: Bramcfgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bramcfgselect {
    type Ux = u8;
}
impl crate::IsEnum for Bramcfgselect {}
#[doc = "Field `BRAMCFG` reader - Ram Configuration"]
pub type BramcfgR = crate::FieldReader<Bramcfgselect>;
impl BramcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bramcfgselect> {
        match self.bits {
            0 => Some(Bramcfgselect::Ret),
            1 => Some(Bramcfgselect::Partial),
            2 => Some(Bramcfgselect::Off),
            _ => None,
        }
    }
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Bramcfgselect::Ret
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == Bramcfgselect::Partial
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Bramcfgselect::Off
    }
}
#[doc = "Field `BRAMCFG` writer - Ram Configuration"]
pub type BramcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bramcfgselect>;
impl<'a, REG> BramcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Bramcfgselect::Ret)
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut crate::W<REG> {
        self.variant(Bramcfgselect::Partial)
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Bramcfgselect::Off)
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&self) -> BramcfgR {
        BramcfgR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bramcfg(&mut self) -> BramcfgW<BkupcfgSpec> {
        BramcfgW::new(self, 0)
    }
}
#[doc = "Backup Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bkupcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkupcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkupcfgSpec;
impl crate::RegisterSpec for BkupcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkupcfg::R`](R) reader structure"]
impl crate::Readable for BkupcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bkupcfg::W`](W) writer structure"]
impl crate::Writable for BkupcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BKUPCFG to value 0"]
impl crate::Resettable for BkupcfgSpec {
    const RESET_VALUE: u8 = 0;
}
