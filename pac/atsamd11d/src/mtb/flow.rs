#[doc = "Register `FLOW` reader"]
pub type R = crate::R<FLOW_SPEC>;
#[doc = "Register `FLOW` writer"]
pub type W = crate::W<FLOW_SPEC>;
#[doc = "Field `AUTOSTOP` reader - Auto Stop Tracing"]
pub type AUTOSTOP_R = crate::BitReader;
#[doc = "Field `AUTOSTOP` writer - Auto Stop Tracing"]
pub type AUTOSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOHALT` reader - Auto Halt Request"]
pub type AUTOHALT_R = crate::BitReader;
#[doc = "Field `AUTOHALT` writer - Auto Halt Request"]
pub type AUTOHALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WATERMARK` reader - Watermark value"]
pub type WATERMARK_R = crate::FieldReader<u32>;
#[doc = "Field `WATERMARK` writer - Watermark value"]
pub type WATERMARK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 29, O, u32>;
impl R {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    pub fn autohalt(&self) -> AUTOHALT_R {
        AUTOHALT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    #[must_use]
    pub fn autostop(&mut self) -> AUTOSTOP_W<FLOW_SPEC, 0> {
        AUTOSTOP_W::new(self)
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    #[must_use]
    pub fn autohalt(&mut self) -> AUTOHALT_W<FLOW_SPEC, 1> {
        AUTOHALT_W::new(self)
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    #[must_use]
    pub fn watermark(&mut self) -> WATERMARK_W<FLOW_SPEC, 3> {
        WATERMARK_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MTB Flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLOW_SPEC;
impl crate::RegisterSpec for FLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flow::R`](R) reader structure"]
impl crate::Readable for FLOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flow::W`](W) writer structure"]
impl crate::Writable for FLOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLOW to value 0"]
impl crate::Resettable for FLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
