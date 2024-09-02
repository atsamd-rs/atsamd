#[doc = "Register `FLOW` reader"]
pub type R = crate::R<FlowSpec>;
#[doc = "Register `FLOW` writer"]
pub type W = crate::W<FlowSpec>;
#[doc = "Field `AUTOSTOP` reader - Auto Stop Tracing"]
pub type AutostopR = crate::BitReader;
#[doc = "Field `AUTOSTOP` writer - Auto Stop Tracing"]
pub type AutostopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOHALT` reader - Auto Halt Request"]
pub type AutohaltR = crate::BitReader;
#[doc = "Field `AUTOHALT` writer - Auto Halt Request"]
pub type AutohaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATERMARK` reader - Watermark value"]
pub type WatermarkR = crate::FieldReader<u32>;
#[doc = "Field `WATERMARK` writer - Watermark value"]
pub type WatermarkW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    pub fn autostop(&self) -> AutostopR {
        AutostopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    pub fn autohalt(&self) -> AutohaltR {
        AutohaltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    pub fn watermark(&self) -> WatermarkR {
        WatermarkR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    #[must_use]
    pub fn autostop(&mut self) -> AutostopW<FlowSpec> {
        AutostopW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    #[must_use]
    pub fn autohalt(&mut self) -> AutohaltW<FlowSpec> {
        AutohaltW::new(self, 1)
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    #[must_use]
    pub fn watermark(&mut self) -> WatermarkW<FlowSpec> {
        WatermarkW::new(self, 3)
    }
}
#[doc = "MTB Flow\n\nYou can [`read`](crate::Reg::read) this register and get [`flow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlowSpec;
impl crate::RegisterSpec for FlowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flow::R`](R) reader structure"]
impl crate::Readable for FlowSpec {}
#[doc = "`write(|w| ..)` method takes [`flow::W`](W) writer structure"]
impl crate::Writable for FlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLOW to value 0"]
impl crate::Resettable for FlowSpec {
    const RESET_VALUE: u32 = 0;
}
