#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `RESRDY` reader - Result Ready Interrupt Disable"]
pub type ResrdyR = crate::BitReader;
#[doc = "Field `RESRDY` writer - Result Ready Interrupt Disable"]
pub type ResrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - Overrun Interrupt Disable"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Overrun Interrupt Disable"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINMON` reader - Window Monitor Interrupt Disable"]
pub type WinmonR = crate::BitReader;
#[doc = "Field `WINMON` writer - Window Monitor Interrupt Disable"]
pub type WinmonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Result Ready Interrupt Disable"]
    #[inline(always)]
    pub fn resrdy(&self) -> ResrdyR {
        ResrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Monitor Interrupt Disable"]
    #[inline(always)]
    pub fn winmon(&self) -> WinmonR {
        WinmonR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy(&mut self) -> ResrdyW<IntenclrSpec> {
        ResrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OverrunW<IntenclrSpec> {
        OverrunW::new(self, 1)
    }
    #[doc = "Bit 2 - Window Monitor Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn winmon(&mut self) -> WinmonW<IntenclrSpec> {
        WinmonW::new(self, 2)
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
