#[doc = "Register `SEECFG` reader"]
pub type R = crate::R<SEECFG_SPEC>;
#[doc = "Register `SEECFG` writer"]
pub type W = crate::W<SEECFG_SPEC>;
#[doc = "Field `WMODE` reader - Write Mode"]
pub type WMODE_R = crate::BitReader<WMODESELECT_A>;
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WMODESELECT_A {
    #[doc = "0: A NVM write command is issued after each write in the pagebuffer"]
    UNBUFFERED = 0,
    #[doc = "1: A NVM write command is issued when a write to a new page is requested"]
    BUFFERED = 1,
}
impl From<WMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WMODESELECT_A {
        match self.bits {
            false => WMODESELECT_A::UNBUFFERED,
            true => WMODESELECT_A::BUFFERED,
        }
    }
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline(always)]
    pub fn is_unbuffered(&self) -> bool {
        *self == WMODESELECT_A::UNBUFFERED
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == WMODESELECT_A::BUFFERED
    }
}
#[doc = "Field `WMODE` writer - Write Mode"]
pub type WMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WMODESELECT_A>;
impl<'a, REG, const O: u8> WMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline(always)]
    pub fn unbuffered(self) -> &'a mut crate::W<REG> {
        self.variant(WMODESELECT_A::UNBUFFERED)
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut crate::W<REG> {
        self.variant(WMODESELECT_A::BUFFERED)
    }
}
#[doc = "Field `APRDIS` reader - Automatic Page Reallocation Disable"]
pub type APRDIS_R = crate::BitReader;
#[doc = "Field `APRDIS` writer - Automatic Page Reallocation Disable"]
pub type APRDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WMODE_R {
        WMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&self) -> APRDIS_R {
        APRDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wmode(&mut self) -> WMODE_W<SEECFG_SPEC, 0> {
        WMODE_W::new(self)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aprdis(&mut self) -> APRDIS_W<SEECFG_SPEC, 1> {
        APRDIS_W::new(self)
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
#[doc = "SmartEEPROM Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seecfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seecfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEECFG_SPEC;
impl crate::RegisterSpec for SEECFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seecfg::R`](R) reader structure"]
impl crate::Readable for SEECFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seecfg::W`](W) writer structure"]
impl crate::Writable for SEECFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEECFG to value 0"]
impl crate::Resettable for SEECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
