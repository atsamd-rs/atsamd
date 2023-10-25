#[doc = "Register `ASYNCH` reader"]
pub type R = crate::R<ASYNCH_SPEC>;
#[doc = "Register `ASYNCH` writer"]
pub type W = crate::W<ASYNCH_SPEC>;
#[doc = "Field `ASYNCH` reader - Asynchronous Edge Detection Mode"]
pub type ASYNCH_R = crate::FieldReader<ASYNCHSELECT_A>;
#[doc = "Asynchronous Edge Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ASYNCHSELECT_A {
    #[doc = "0: Edge detection is clock synchronously operated"]
    SYNC = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    ASYNC = 1,
}
impl From<ASYNCHSELECT_A> for u16 {
    #[inline(always)]
    fn from(variant: ASYNCHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ASYNCHSELECT_A {
    type Ux = u16;
}
impl ASYNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ASYNCHSELECT_A> {
        match self.bits {
            0 => Some(ASYNCHSELECT_A::SYNC),
            1 => Some(ASYNCHSELECT_A::ASYNC),
            _ => None,
        }
    }
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == ASYNCHSELECT_A::SYNC
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == ASYNCHSELECT_A::ASYNC
    }
}
#[doc = "Field `ASYNCH` writer - Asynchronous Edge Detection Mode"]
pub type ASYNCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, ASYNCHSELECT_A>;
impl<'a, REG, const O: u8> ASYNCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCHSELECT_A::SYNC)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCHSELECT_A::ASYNC)
    }
}
impl R {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn asynch(&self) -> ASYNCH_R {
        ASYNCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn asynch(&mut self) -> ASYNCH_W<ASYNCH_SPEC, 0> {
        ASYNCH_W::new(self)
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
#[doc = "External Interrupt Asynchronous Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asynch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asynch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCH_SPEC;
impl crate::RegisterSpec for ASYNCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asynch::R`](R) reader structure"]
impl crate::Readable for ASYNCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asynch::W`](W) writer structure"]
impl crate::Writable for ASYNCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCH to value 0"]
impl crate::Resettable for ASYNCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
