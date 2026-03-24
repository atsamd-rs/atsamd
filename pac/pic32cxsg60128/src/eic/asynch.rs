#[doc = "Register `ASYNCH` reader"]
pub type R = crate::R<AsynchSpec>;
#[doc = "Register `ASYNCH` writer"]
pub type W = crate::W<AsynchSpec>;
#[doc = "Asynchronous Edge Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Asynchselect {
    #[doc = "0: Edge detection is clock synchronously operated"]
    Sync = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    Async = 1,
}
impl From<Asynchselect> for u16 {
    #[inline(always)]
    fn from(variant: Asynchselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Asynchselect {
    type Ux = u16;
}
impl crate::IsEnum for Asynchselect {}
#[doc = "Field `ASYNCH` reader - Asynchronous Edge Detection Mode"]
pub type AsynchR = crate::FieldReader<Asynchselect>;
impl AsynchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Asynchselect> {
        match self.bits {
            0 => Some(Asynchselect::Sync),
            1 => Some(Asynchselect::Async),
            _ => None,
        }
    }
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Asynchselect::Sync
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Asynchselect::Async
    }
}
#[doc = "Field `ASYNCH` writer - Asynchronous Edge Detection Mode"]
pub type AsynchW<'a, REG> = crate::FieldWriter<'a, REG, 16, Asynchselect>;
impl<'a, REG> AsynchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Asynchselect::Sync)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Asynchselect::Async)
    }
}
impl R {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn asynch(&self) -> AsynchR {
        AsynchR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn asynch(&mut self) -> AsynchW<AsynchSpec> {
        AsynchW::new(self, 0)
    }
}
#[doc = "External Interrupt Asynchronous Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`asynch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asynch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsynchSpec;
impl crate::RegisterSpec for AsynchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asynch::R`](R) reader structure"]
impl crate::Readable for AsynchSpec {}
#[doc = "`write(|w| ..)` method takes [`asynch::W`](W) writer structure"]
impl crate::Writable for AsynchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASYNCH to value 0"]
impl crate::Resettable for AsynchSpec {
    const RESET_VALUE: u32 = 0;
}
