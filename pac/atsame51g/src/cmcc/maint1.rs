#[doc = "Register `MAINT1` writer"]
pub type W = crate::W<MAINT1_SPEC>;
#[doc = "Field `INDEX` writer - Invalidate Index"]
pub type INDEX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Invalidate Way\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAYSELECT_AW {
    #[doc = "0: Way 0 is selection for index invalidation"]
    WAY0 = 0,
    #[doc = "1: Way 1 is selection for index invalidation"]
    WAY1 = 1,
    #[doc = "2: Way 2 is selection for index invalidation"]
    WAY2 = 2,
    #[doc = "3: Way 3 is selection for index invalidation"]
    WAY3 = 3,
}
impl From<WAYSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: WAYSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAYSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `WAY` writer - Invalidate Way"]
pub type WAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, WAYSELECT_AW>;
impl<'a, REG, const O: u8> WAY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Way 0 is selection for index invalidation"]
    #[inline(always)]
    pub fn way0(self) -> &'a mut crate::W<REG> {
        self.variant(WAYSELECT_AW::WAY0)
    }
    #[doc = "Way 1 is selection for index invalidation"]
    #[inline(always)]
    pub fn way1(self) -> &'a mut crate::W<REG> {
        self.variant(WAYSELECT_AW::WAY1)
    }
    #[doc = "Way 2 is selection for index invalidation"]
    #[inline(always)]
    pub fn way2(self) -> &'a mut crate::W<REG> {
        self.variant(WAYSELECT_AW::WAY2)
    }
    #[doc = "Way 3 is selection for index invalidation"]
    #[inline(always)]
    pub fn way3(self) -> &'a mut crate::W<REG> {
        self.variant(WAYSELECT_AW::WAY3)
    }
}
impl W {
    #[doc = "Bits 4:11 - Invalidate Index"]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<MAINT1_SPEC, 4> {
        INDEX_W::new(self)
    }
    #[doc = "Bits 28:31 - Invalidate Way"]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WAY_W<MAINT1_SPEC, 28> {
        WAY_W::new(self)
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
#[doc = "Cache Maintenance Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maint1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAINT1_SPEC;
impl crate::RegisterSpec for MAINT1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`maint1::W`](W) writer structure"]
impl crate::Writable for MAINT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAINT1 to value 0"]
impl crate::Resettable for MAINT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
