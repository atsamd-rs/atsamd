#[doc = "Register `CHCTRLB` reader"]
pub type R = crate::R<CHCTRLB_SPEC>;
#[doc = "Register `CHCTRLB` writer"]
pub type W = crate::W<CHCTRLB_SPEC>;
#[doc = "Field `CMD` reader - Software Command"]
pub type CMD_R = crate::FieldReader<CMDSELECT_A>;
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Channel suspend operation"]
    SUSPEND = 1,
    #[doc = "2: Channel resume operation"]
    RESUME = 2,
}
impl From<CMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDSELECT_A {
    type Ux = u8;
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            0 => Some(CMDSELECT_A::NOACT),
            1 => Some(CMDSELECT_A::SUSPEND),
            2 => Some(CMDSELECT_A::RESUME),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == CMDSELECT_A::NOACT
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDSELECT_A::SUSPEND
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDSELECT_A::RESUME
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CMDSELECT_A>;
impl<'a, REG, const O: u8> CMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::NOACT)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::RESUME)
    }
}
impl R {
    #[doc = "Bits 0:1 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Software Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CHCTRLB_SPEC, 0> {
        CMD_W::new(self)
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
#[doc = "Channel n Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTRLB_SPEC;
impl crate::RegisterSpec for CHCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chctrlb::R`](R) reader structure"]
impl crate::Readable for CHCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctrlb::W`](W) writer structure"]
impl crate::Writable for CHCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for CHCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
