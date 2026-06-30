#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `GENCEN` reader - General Call Address Enable"]
pub type GencenR = crate::BitReader;
#[doc = "Field `GENCEN` writer - General Call Address Enable"]
pub type GencenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Address Value"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Address Value"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TENBITEN` reader - Ten Bit Addressing Enable"]
pub type TenbitenR = crate::BitReader;
#[doc = "Field `TENBITEN` writer - Ten Bit Addressing Enable"]
pub type TenbitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub type AddrmaskR = crate::FieldReader<u16>;
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub type AddrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    pub fn gencen(&self) -> GencenR {
        GencenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Address Value"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&self) -> TenbitenR {
        TenbitenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> AddrmaskR {
        AddrmaskR::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gencen(&mut self) -> GencenW<AddrSpec> {
        GencenW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<AddrSpec> {
        AddrW::new(self, 1)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tenbiten(&mut self) -> TenbitenW<AddrSpec> {
        TenbitenW::new(self, 15)
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn addrmask(&mut self) -> AddrmaskW<AddrSpec> {
        AddrmaskW::new(self, 17)
    }
}
#[doc = "I2CS Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
