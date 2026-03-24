#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<ClearSpec>;
#[doc = "Watchdog Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clearselect {
    #[doc = "165: Clear Key"]
    Key = 165,
}
impl From<Clearselect> for u8 {
    #[inline(always)]
    fn from(variant: Clearselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clearselect {
    type Ux = u8;
}
impl crate::IsEnum for Clearselect {}
#[doc = "Field `CLEAR` writer - Watchdog Clear"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 8, Clearselect>;
impl<'a, REG> ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clear Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Clearselect::Key)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<ClearSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for ClearSpec {
    const RESET_VALUE: u8 = 0;
}
