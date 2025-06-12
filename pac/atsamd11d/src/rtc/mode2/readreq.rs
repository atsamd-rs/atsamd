#[doc = "Register `READREQ` reader"]
pub type R = crate::R<ReadreqSpec>;
#[doc = "Register `READREQ` writer"]
pub type W = crate::W<ReadreqSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `RCONT` reader - Read Continuously"]
pub type RcontR = crate::BitReader;
#[doc = "Field `RCONT` writer - Read Continuously"]
pub type RcontW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RREQ` writer - Read Request"]
pub type RreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    pub fn rcont(&self) -> RcontR {
        RcontR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    pub fn rcont(&mut self) -> RcontW<ReadreqSpec> {
        RcontW::new(self, 14)
    }
    #[doc = "Bit 15 - Read Request"]
    #[inline(always)]
    pub fn rreq(&mut self) -> RreqW<ReadreqSpec> {
        RreqW::new(self, 15)
    }
}
#[doc = "Read Request\n\nYou can [`read`](crate::Reg::read) this register and get [`readreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadreqSpec;
impl crate::RegisterSpec for ReadreqSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`readreq::R`](R) reader structure"]
impl crate::Readable for ReadreqSpec {}
#[doc = "`write(|w| ..)` method takes [`readreq::W`](W) writer structure"]
impl crate::Writable for ReadreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READREQ to value 0x10"]
impl crate::Resettable for ReadreqSpec {
    const RESET_VALUE: u16 = 0x10;
}
