#[doc = "Register `RXF1C` reader"]
pub type R = crate::R<Rxf1cSpec>;
#[doc = "Register `RXF1C` writer"]
pub type W = crate::W<Rxf1cSpec>;
#[doc = "Field `F1SA` reader - Rx FIFO 1 Start Address"]
pub type F1saR = crate::FieldReader<u16>;
#[doc = "Field `F1SA` writer - Rx FIFO 1 Start Address"]
pub type F1saW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `F1S` reader - Rx FIFO 1 Size"]
pub type F1sR = crate::FieldReader;
#[doc = "Field `F1S` writer - Rx FIFO 1 Size"]
pub type F1sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1WM` reader - Rx FIFO 1 Watermark"]
pub type F1wmR = crate::FieldReader;
#[doc = "Field `F1WM` writer - Rx FIFO 1 Watermark"]
pub type F1wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1OM` reader - FIFO 1 Operation Mode"]
pub type F1omR = crate::BitReader;
#[doc = "Field `F1OM` writer - FIFO 1 Operation Mode"]
pub type F1omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1saR {
        F1saR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&self) -> F1sR {
        F1sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1wmR {
        F1wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1omR {
        F1omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f1sa(&mut self) -> F1saW<Rxf1cSpec> {
        F1saW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1s(&mut self) -> F1sW<Rxf1cSpec> {
        F1sW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn f1wm(&mut self) -> F1wmW<Rxf1cSpec> {
        F1wmW::new(self, 24)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1omW<Rxf1cSpec> {
        F1omW::new(self, 31)
    }
}
#[doc = "Rx FIFO 1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1cSpec;
impl crate::RegisterSpec for Rxf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1c::R`](R) reader structure"]
impl crate::Readable for Rxf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf1c::W`](W) writer structure"]
impl crate::Writable for Rxf1cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF1C to value 0"]
impl crate::Resettable for Rxf1cSpec {
    const RESET_VALUE: u32 = 0;
}
