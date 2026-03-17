#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `ADDR` reader - Address Value"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Address Value"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `LENEN` reader - Length Enable"]
pub type LenenR = crate::BitReader;
#[doc = "Field `LENEN` writer - Length Enable"]
pub type LenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS` reader - High Speed Mode"]
pub type HsR = crate::BitReader;
#[doc = "Field `HS` writer - High Speed Mode"]
pub type HsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TENBITEN` reader - Ten Bit Addressing Enable"]
pub type TenbitenR = crate::BitReader;
#[doc = "Field `TENBITEN` writer - Ten Bit Addressing Enable"]
pub type TenbitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN` reader - Length"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - Address Value"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LenenR {
        LenenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline(always)]
    pub fn hs(&self) -> HsR {
        HsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&self) -> TenbitenR {
        TenbitenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<AddrSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lenen(&mut self) -> LenenW<AddrSpec> {
        LenenW::new(self, 13)
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HsW<AddrSpec> {
        HsW::new(self, 14)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tenbiten(&mut self) -> TenbitenW<AddrSpec> {
        TenbitenW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<AddrSpec> {
        LenW::new(self, 16)
    }
}
#[doc = "I2CM Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}
