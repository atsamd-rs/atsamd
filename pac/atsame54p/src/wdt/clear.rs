#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<CLEAR_SPEC>;
#[doc = "Watchdog Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLEARSELECT_AW {
    #[doc = "165: Clear Key"]
    KEY = 165,
}
impl From<CLEARSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: CLEARSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLEARSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `CLEAR` writer - Watchdog Clear"]
pub type CLEAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CLEARSELECT_AW>;
impl<'a, REG, const O: u8> CLEAR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clear Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(CLEARSELECT_AW::KEY)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<CLEAR_SPEC, 0> {
        CLEAR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEAR_SPEC;
impl crate::RegisterSpec for CLEAR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
