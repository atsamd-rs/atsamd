#[doc = "Register `BBPS` reader"]
pub type R = crate::R<BBPS_SPEC>;
#[doc = "Register `BBPS` writer"]
pub type W = crate::W<BBPS_SPEC>;
#[doc = "Field `CONF` reader - Battery Backup Configuration"]
pub type CONF_R = crate::BitReader<CONFSELECT_A>;
#[doc = "Battery Backup Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFSELECT_A {
    #[doc = "0: The power switch is handled by the BOD33"]
    BOD33 = 0,
    #[doc = "1: In Backup Domain, the backup domain is always supplied by battery backup power"]
    FORCED = 1,
}
impl From<CONFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONFSELECT_A {
        match self.bits {
            false => CONFSELECT_A::BOD33,
            true => CONFSELECT_A::FORCED,
        }
    }
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn is_bod33(&self) -> bool {
        *self == CONFSELECT_A::BOD33
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == CONFSELECT_A::FORCED
    }
}
#[doc = "Field `CONF` writer - Battery Backup Configuration"]
pub type CONF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CONFSELECT_A>;
impl<'a, REG, const O: u8> CONF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn bod33(self) -> &'a mut crate::W<REG> {
        self.variant(CONFSELECT_A::BOD33)
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(CONFSELECT_A::FORCED)
    }
}
#[doc = "Field `WAKEEN` reader - Wake Enable"]
pub type WAKEEN_R = crate::BitReader;
#[doc = "Field `WAKEEN` writer - Wake Enable"]
pub type WAKEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn conf(&mut self) -> CONF_W<BBPS_SPEC, 0> {
        CONF_W::new(self)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeen(&mut self) -> WAKEEN_W<BBPS_SPEC, 2> {
        WAKEEN_W::new(self)
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
#[doc = "Battery Backup Power Switch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bbps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bbps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BBPS_SPEC;
impl crate::RegisterSpec for BBPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bbps::R`](R) reader structure"]
impl crate::Readable for BBPS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bbps::W`](W) writer structure"]
impl crate::Writable for BBPS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BBPS to value 0"]
impl crate::Resettable for BBPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
