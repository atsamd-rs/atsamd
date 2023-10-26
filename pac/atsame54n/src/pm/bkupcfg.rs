#[doc = "Register `BKUPCFG` reader"]
pub type R = crate::R<BKUPCFG_SPEC>;
#[doc = "Register `BKUPCFG` writer"]
pub type W = crate::W<BKUPCFG_SPEC>;
#[doc = "Field `BRAMCFG` reader - Ram Configuration"]
pub type BRAMCFG_R = crate::FieldReader<BRAMCFGSELECT_A>;
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRAMCFGSELECT_A {
    #[doc = "0: All the backup RAM is retained"]
    RET = 0,
    #[doc = "1: Only the first 4Kbytes of the backup RAM is retained"]
    PARTIAL = 1,
    #[doc = "2: All the backup RAM is turned OFF"]
    OFF = 2,
}
impl From<BRAMCFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BRAMCFGSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BRAMCFGSELECT_A {
    type Ux = u8;
}
impl BRAMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BRAMCFGSELECT_A> {
        match self.bits {
            0 => Some(BRAMCFGSELECT_A::RET),
            1 => Some(BRAMCFGSELECT_A::PARTIAL),
            2 => Some(BRAMCFGSELECT_A::OFF),
            _ => None,
        }
    }
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == BRAMCFGSELECT_A::RET
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == BRAMCFGSELECT_A::PARTIAL
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BRAMCFGSELECT_A::OFF
    }
}
#[doc = "Field `BRAMCFG` writer - Ram Configuration"]
pub type BRAMCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, BRAMCFGSELECT_A>;
impl<'a, REG, const O: u8> BRAMCFG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(BRAMCFGSELECT_A::RET)
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut crate::W<REG> {
        self.variant(BRAMCFGSELECT_A::PARTIAL)
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(BRAMCFGSELECT_A::OFF)
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&self) -> BRAMCFG_R {
        BRAMCFG_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bramcfg(&mut self) -> BRAMCFG_W<BKUPCFG_SPEC, 0> {
        BRAMCFG_W::new(self)
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
#[doc = "Backup Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkupcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkupcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKUPCFG_SPEC;
impl crate::RegisterSpec for BKUPCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkupcfg::R`](R) reader structure"]
impl crate::Readable for BKUPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bkupcfg::W`](W) writer structure"]
impl crate::Writable for BKUPCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKUPCFG to value 0"]
impl crate::Resettable for BKUPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
