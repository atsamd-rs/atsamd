#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AIRCR_SPEC>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AIRCR_SPEC>;
#[doc = "Field `VECTRESET` reader - Must write 0"]
pub type VECTRESET_R = crate::BitReader;
#[doc = "Field `VECTRESET` writer - Must write 0"]
pub type VECTRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VECTCLRACTIVE` reader - Must write 0"]
pub type VECTCLRACTIVE_R = crate::BitReader;
#[doc = "Field `VECTCLRACTIVE` writer - Must write 0"]
pub type VECTCLRACTIVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSRESETREQ` reader - System Reset Request"]
pub type SYSRESETREQ_R = crate::BitReader<SYSRESETREQSELECT_A>;
#[doc = "System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRESETREQSELECT_A {
    #[doc = "0: No system reset request"]
    VALUE_0 = 0,
    #[doc = "1: Asserts a signal to the outer system that requests a reset"]
    VALUE_1 = 1,
}
impl From<SYSRESETREQSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRESETREQSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSRESETREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSRESETREQSELECT_A {
        match self.bits {
            false => SYSRESETREQSELECT_A::VALUE_0,
            true => SYSRESETREQSELECT_A::VALUE_1,
        }
    }
    #[doc = "No system reset request"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SYSRESETREQSELECT_A::VALUE_0
    }
    #[doc = "Asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SYSRESETREQSELECT_A::VALUE_1
    }
}
#[doc = "Field `SYSRESETREQ` writer - System Reset Request"]
pub type SYSRESETREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SYSRESETREQSELECT_A>;
impl<'a, REG, const O: u8> SYSRESETREQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No system reset request"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRESETREQSELECT_A::VALUE_0)
    }
    #[doc = "Asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRESETREQSELECT_A::VALUE_1)
    }
}
#[doc = "Field `PRIGROUP` reader - Interrupt priority grouping"]
pub type PRIGROUP_R = crate::FieldReader;
#[doc = "Field `PRIGROUP` writer - Interrupt priority grouping"]
pub type PRIGROUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ENDIANNESS` reader - Data endianness, 0=little, 1=big"]
pub type ENDIANNESS_R = crate::BitReader<ENDIANNESSSELECT_A>;
#[doc = "Data endianness, 0=little, 1=big\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIANNESSSELECT_A {
    #[doc = "0: Little-endian"]
    VALUE_0 = 0,
    #[doc = "1: Big-endian"]
    VALUE_1 = 1,
}
impl From<ENDIANNESSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIANNESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENDIANNESSSELECT_A {
        match self.bits {
            false => ENDIANNESSSELECT_A::VALUE_0,
            true => ENDIANNESSSELECT_A::VALUE_1,
        }
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == ENDIANNESSSELECT_A::VALUE_0
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == ENDIANNESSSELECT_A::VALUE_1
    }
}
#[doc = "Field `ENDIANNESS` writer - Data endianness, 0=little, 1=big"]
pub type ENDIANNESS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENDIANNESSSELECT_A>;
impl<'a, REG, const O: u8> ENDIANNESS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIANNESSSELECT_A::VALUE_0)
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIANNESSSELECT_A::VALUE_1)
    }
}
#[doc = "Field `VECTKEY` reader - Register key"]
pub type VECTKEY_R = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - Register key"]
pub type VECTKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Must write 0"]
    #[inline(always)]
    pub fn vectreset(&self) -> VECTRESET_R {
        VECTRESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Must write 0"]
    #[inline(always)]
    pub fn vectclractive(&self) -> VECTCLRACTIVE_R {
        VECTCLRACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Reset Request"]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping"]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Data endianness, 0=little, 1=big"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Must write 0"]
    #[inline(always)]
    #[must_use]
    pub fn vectreset(&mut self) -> VECTRESET_W<AIRCR_SPEC, 0> {
        VECTRESET_W::new(self)
    }
    #[doc = "Bit 1 - Must write 0"]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<AIRCR_SPEC, 1> {
        VECTCLRACTIVE_W::new(self)
    }
    #[doc = "Bit 2 - System Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<AIRCR_SPEC, 2> {
        SYSRESETREQ_W::new(self)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping"]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PRIGROUP_W<AIRCR_SPEC, 8> {
        PRIGROUP_W::new(self)
    }
    #[doc = "Bit 15 - Data endianness, 0=little, 1=big"]
    #[inline(always)]
    #[must_use]
    pub fn endianness(&mut self) -> ENDIANNESS_W<AIRCR_SPEC, 15> {
        ENDIANNESS_W::new(self)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<AIRCR_SPEC, 16> {
        VECTKEY_W::new(self)
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
#[doc = "Application Interrupt and Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aircr::R`](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aircr::W`](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0000;
}
