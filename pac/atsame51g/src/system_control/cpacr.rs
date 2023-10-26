#[doc = "Register `CPACR` reader"]
pub type R = crate::R<CPACR_SPEC>;
#[doc = "Register `CPACR` writer"]
pub type W = crate::W<CPACR_SPEC>;
#[doc = "Field `CP10` reader - Access privileges for coprocessor 10"]
pub type CP10_R = crate::FieldReader<CP10SELECT_A>;
#[doc = "Access privileges for coprocessor 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP10SELECT_A {
    #[doc = "0: Access denied"]
    DENIED = 0,
    #[doc = "1: Privileged access only"]
    PRIV = 1,
    #[doc = "3: Full access"]
    FULL = 3,
}
impl From<CP10SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CP10SELECT_A {
    type Ux = u8;
}
impl CP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CP10SELECT_A> {
        match self.bits {
            0 => Some(CP10SELECT_A::DENIED),
            1 => Some(CP10SELECT_A::PRIV),
            3 => Some(CP10SELECT_A::FULL),
            _ => None,
        }
    }
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == CP10SELECT_A::DENIED
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn is_priv(&self) -> bool {
        *self == CP10SELECT_A::PRIV
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CP10SELECT_A::FULL
    }
}
#[doc = "Field `CP10` writer - Access privileges for coprocessor 10"]
pub type CP10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CP10SELECT_A>;
impl<'a, REG, const O: u8> CP10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut crate::W<REG> {
        self.variant(CP10SELECT_A::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut crate::W<REG> {
        self.variant(CP10SELECT_A::PRIV)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(CP10SELECT_A::FULL)
    }
}
#[doc = "Field `CP11` reader - Access privileges for coprocessor 11"]
pub type CP11_R = crate::FieldReader<CP11SELECT_A>;
#[doc = "Access privileges for coprocessor 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP11SELECT_A {
    #[doc = "0: Access denied"]
    DENIED = 0,
    #[doc = "1: Privileged access only"]
    PRIV = 1,
    #[doc = "3: Full access"]
    FULL = 3,
}
impl From<CP11SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CP11SELECT_A {
    type Ux = u8;
}
impl CP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CP11SELECT_A> {
        match self.bits {
            0 => Some(CP11SELECT_A::DENIED),
            1 => Some(CP11SELECT_A::PRIV),
            3 => Some(CP11SELECT_A::FULL),
            _ => None,
        }
    }
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == CP11SELECT_A::DENIED
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn is_priv(&self) -> bool {
        *self == CP11SELECT_A::PRIV
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CP11SELECT_A::FULL
    }
}
#[doc = "Field `CP11` writer - Access privileges for coprocessor 11"]
pub type CP11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CP11SELECT_A>;
impl<'a, REG, const O: u8> CP11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut crate::W<REG> {
        self.variant(CP11SELECT_A::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut crate::W<REG> {
        self.variant(CP11SELECT_A::PRIV)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(CP11SELECT_A::FULL)
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    #[must_use]
    pub fn cp10(&mut self) -> CP10_W<CPACR_SPEC, 20> {
        CP10_W::new(self)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    #[must_use]
    pub fn cp11(&mut self) -> CP11_W<CPACR_SPEC, 22> {
        CP11_W::new(self)
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
#[doc = "Coprocessor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPACR_SPEC;
impl crate::RegisterSpec for CPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpacr::R`](R) reader structure"]
impl crate::Readable for CPACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpacr::W`](W) writer structure"]
impl crate::Writable for CPACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CPACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
