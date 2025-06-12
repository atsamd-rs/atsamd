#[doc = "Register `CPACR` reader"]
pub type R = crate::R<CpacrSpec>;
#[doc = "Register `CPACR` writer"]
pub type W = crate::W<CpacrSpec>;
#[doc = "Access privileges for coprocessor 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cp10select {
    #[doc = "0: Access denied"]
    Denied = 0,
    #[doc = "1: Privileged access only"]
    Priv = 1,
    #[doc = "3: Full access"]
    Full = 3,
}
impl From<Cp10select> for u8 {
    #[inline(always)]
    fn from(variant: Cp10select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cp10select {
    type Ux = u8;
}
impl crate::IsEnum for Cp10select {}
#[doc = "Field `CP10` reader - Access privileges for coprocessor 10"]
pub type Cp10R = crate::FieldReader<Cp10select>;
impl Cp10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cp10select> {
        match self.bits {
            0 => Some(Cp10select::Denied),
            1 => Some(Cp10select::Priv),
            3 => Some(Cp10select::Full),
            _ => None,
        }
    }
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == Cp10select::Denied
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn is_priv(&self) -> bool {
        *self == Cp10select::Priv
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Cp10select::Full
    }
}
#[doc = "Field `CP10` writer - Access privileges for coprocessor 10"]
pub type Cp10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cp10select>;
impl<'a, REG> Cp10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut crate::W<REG> {
        self.variant(Cp10select::Denied)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut crate::W<REG> {
        self.variant(Cp10select::Priv)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Cp10select::Full)
    }
}
#[doc = "Access privileges for coprocessor 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cp11select {
    #[doc = "0: Access denied"]
    Denied = 0,
    #[doc = "1: Privileged access only"]
    Priv = 1,
    #[doc = "3: Full access"]
    Full = 3,
}
impl From<Cp11select> for u8 {
    #[inline(always)]
    fn from(variant: Cp11select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cp11select {
    type Ux = u8;
}
impl crate::IsEnum for Cp11select {}
#[doc = "Field `CP11` reader - Access privileges for coprocessor 11"]
pub type Cp11R = crate::FieldReader<Cp11select>;
impl Cp11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cp11select> {
        match self.bits {
            0 => Some(Cp11select::Denied),
            1 => Some(Cp11select::Priv),
            3 => Some(Cp11select::Full),
            _ => None,
        }
    }
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == Cp11select::Denied
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn is_priv(&self) -> bool {
        *self == Cp11select::Priv
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Cp11select::Full
    }
}
#[doc = "Field `CP11` writer - Access privileges for coprocessor 11"]
pub type Cp11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cp11select>;
impl<'a, REG> Cp11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut crate::W<REG> {
        self.variant(Cp11select::Denied)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut crate::W<REG> {
        self.variant(Cp11select::Priv)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Cp11select::Full)
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&self) -> Cp10R {
        Cp10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&self) -> Cp11R {
        Cp11R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&mut self) -> Cp10W<CpacrSpec> {
        Cp10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&mut self) -> Cp11W<CpacrSpec> {
        Cp11W::new(self, 22)
    }
}
#[doc = "Coprocessor Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpacrSpec;
impl crate::RegisterSpec for CpacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpacr::R`](R) reader structure"]
impl crate::Readable for CpacrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpacr::W`](W) writer structure"]
impl crate::Writable for CpacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CpacrSpec {}
