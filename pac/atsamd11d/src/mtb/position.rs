#[doc = "Register `POSITION` reader"]
pub type R = crate::R<PositionSpec>;
#[doc = "Register `POSITION` writer"]
pub type W = crate::W<PositionSpec>;
#[doc = "Field `WRAP` reader - Pointer Value Wraps"]
pub type WrapR = crate::BitReader;
#[doc = "Field `WRAP` writer - Pointer Value Wraps"]
pub type WrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POINTER` reader - Trace Packet Location Pointer"]
pub type PointerR = crate::FieldReader<u32>;
#[doc = "Field `POINTER` writer - Trace Packet Location Pointer"]
pub type PointerW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 2 - Pointer Value Wraps"]
    #[inline(always)]
    pub fn wrap(&self) -> WrapR {
        WrapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
    #[inline(always)]
    pub fn pointer(&self) -> PointerR {
        PointerR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 2 - Pointer Value Wraps"]
    #[inline(always)]
    #[must_use]
    pub fn wrap(&mut self) -> WrapW<PositionSpec> {
        WrapW::new(self, 2)
    }
    #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn pointer(&mut self) -> PointerW<PositionSpec> {
        PointerW::new(self, 3)
    }
}
#[doc = "MTB Position\n\nYou can [`read`](crate::Reg::read) this register and get [`position::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`position::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PositionSpec;
impl crate::RegisterSpec for PositionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`position::R`](R) reader structure"]
impl crate::Readable for PositionSpec {}
#[doc = "`write(|w| ..)` method takes [`position::W`](W) writer structure"]
impl crate::Writable for PositionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POSITION to value 0"]
impl crate::Resettable for PositionSpec {
    const RESET_VALUE: u32 = 0;
}
