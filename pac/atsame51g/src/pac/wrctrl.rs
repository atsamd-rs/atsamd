#[doc = "Register `WRCTRL` reader"]
pub type R = crate::R<WRCTRL_SPEC>;
#[doc = "Register `WRCTRL` writer"]
pub type W = crate::W<WRCTRL_SPEC>;
#[doc = "Field `PERID` reader - Peripheral identifier"]
pub type PERID_R = crate::FieldReader<u16>;
#[doc = "Field `PERID` writer - Peripheral identifier"]
pub type PERID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `KEY` reader - Peripheral access control key"]
pub type KEY_R = crate::FieldReader<KEYSELECT_A>;
#[doc = "Peripheral access control key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_A {
    #[doc = "0: No action"]
    OFF = 0,
    #[doc = "1: Clear protection"]
    CLR = 1,
    #[doc = "2: Set protection"]
    SET = 2,
    #[doc = "3: Set and lock protection"]
    SETLCK = 3,
}
impl From<KEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYSELECT_A {
    type Ux = u8;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KEYSELECT_A> {
        match self.bits {
            0 => Some(KEYSELECT_A::OFF),
            1 => Some(KEYSELECT_A::CLR),
            2 => Some(KEYSELECT_A::SET),
            3 => Some(KEYSELECT_A::SETLCK),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == KEYSELECT_A::OFF
    }
    #[doc = "Clear protection"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == KEYSELECT_A::CLR
    }
    #[doc = "Set protection"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == KEYSELECT_A::SET
    }
    #[doc = "Set and lock protection"]
    #[inline(always)]
    pub fn is_setlck(&self) -> bool {
        *self == KEYSELECT_A::SETLCK
    }
}
#[doc = "Field `KEY` writer - Peripheral access control key"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, KEYSELECT_A>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSELECT_A::OFF)
    }
    #[doc = "Clear protection"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSELECT_A::CLR)
    }
    #[doc = "Set protection"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSELECT_A::SET)
    }
    #[doc = "Set and lock protection"]
    #[inline(always)]
    pub fn setlck(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSELECT_A::SETLCK)
    }
}
impl R {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    #[must_use]
    pub fn perid(&mut self) -> PERID_W<WRCTRL_SPEC, 0> {
        PERID_W::new(self)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<WRCTRL_SPEC, 16> {
        KEY_W::new(self)
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
#[doc = "Write control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRCTRL_SPEC;
impl crate::RegisterSpec for WRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrctrl::R`](R) reader structure"]
impl crate::Readable for WRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrctrl::W`](W) writer structure"]
impl crate::Writable for WRCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRCTRL to value 0"]
impl crate::Resettable for WRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
