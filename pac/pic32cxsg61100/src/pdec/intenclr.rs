#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `OVF` reader - Overflow/Underflow Interrupt Disable"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow/Underflow Interrupt Disable"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error Interrupt Disable"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Error Interrupt Disable"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Direction Interrupt Disable"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Direction Interrupt Disable"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLC` reader - Velocity Interrupt Disable"]
pub type VlcR = crate::BitReader;
#[doc = "Field `VLC` writer - Velocity Interrupt Disable"]
pub type VlcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC0` reader - Channel 0 Compare Match Disable"]
pub type Mc0R = crate::BitReader;
#[doc = "Field `MC0` writer - Channel 0 Compare Match Disable"]
pub type Mc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC1` reader - Channel 1 Compare Match Disable"]
pub type Mc1R = crate::BitReader;
#[doc = "Field `MC1` writer - Channel 1 Compare Match Disable"]
pub type Mc1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow/Underflow Interrupt Disable"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Disable"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Interrupt Disable"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Velocity Interrupt Disable"]
    #[inline(always)]
    pub fn vlc(&self) -> VlcR {
        VlcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Compare Match Disable"]
    #[inline(always)]
    pub fn mc0(&self) -> Mc0R {
        Mc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Compare Match Disable"]
    #[inline(always)]
    pub fn mc1(&self) -> Mc1R {
        Mc1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow/Underflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntenclrSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<IntenclrSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<IntenclrSpec> {
        DirW::new(self, 2)
    }
    #[doc = "Bit 3 - Velocity Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VlcW<IntenclrSpec> {
        VlcW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 0 Compare Match Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mc0(&mut self) -> Mc0W<IntenclrSpec> {
        Mc0W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 1 Compare Match Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> Mc1W<IntenclrSpec> {
        Mc1W::new(self, 5)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u8 = 0;
}
