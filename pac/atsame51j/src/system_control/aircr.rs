#[doc = "Register `AIRCR` reader"]
pub struct R(crate::R<AIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIRCR` writer"]
pub struct W(crate::W<AIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTRESET` reader - Must write 0"]
pub type VECTRESET_R = crate::BitReader<bool>;
#[doc = "Field `VECTRESET` writer - Must write 0"]
pub type VECTRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `VECTCLRACTIVE` reader - Must write 0"]
pub type VECTCLRACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `VECTCLRACTIVE` writer - Must write 0"]
pub type VECTCLRACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
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
    pub fn variant(&self) -> SYSRESETREQSELECT_A {
        match self.bits {
            false => SYSRESETREQSELECT_A::VALUE_0,
            true => SYSRESETREQSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SYSRESETREQSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SYSRESETREQSELECT_A::VALUE_1
    }
}
#[doc = "Field `SYSRESETREQ` writer - System Reset Request"]
pub type SYSRESETREQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AIRCR_SPEC, SYSRESETREQSELECT_A, O>;
impl<'a, const O: u8> SYSRESETREQ_W<'a, O> {
    #[doc = "No system reset request"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SYSRESETREQSELECT_A::VALUE_0)
    }
    #[doc = "Asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SYSRESETREQSELECT_A::VALUE_1)
    }
}
#[doc = "Field `PRIGROUP` reader - Interrupt priority grouping"]
pub type PRIGROUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIGROUP` writer - Interrupt priority grouping"]
pub type PRIGROUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u8, u8, 3, O>;
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
    pub fn variant(&self) -> ENDIANNESSSELECT_A {
        match self.bits {
            false => ENDIANNESSSELECT_A::VALUE_0,
            true => ENDIANNESSSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == ENDIANNESSSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == ENDIANNESSSELECT_A::VALUE_1
    }
}
#[doc = "Field `ENDIANNESS` writer - Data endianness, 0=little, 1=big"]
pub type ENDIANNESS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AIRCR_SPEC, ENDIANNESSSELECT_A, O>;
impl<'a, const O: u8> ENDIANNESS_W<'a, O> {
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ENDIANNESSSELECT_A::VALUE_0)
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ENDIANNESSSELECT_A::VALUE_1)
    }
}
#[doc = "Field `VECTKEY` reader - Register key"]
pub type VECTKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VECTKEY` writer - Register key"]
pub type VECTKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u16, u16, 16, O>;
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
    pub fn vectreset(&mut self) -> VECTRESET_W<0> {
        VECTRESET_W::new(self)
    }
    #[doc = "Bit 1 - Must write 0"]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<1> {
        VECTCLRACTIVE_W::new(self)
    }
    #[doc = "Bit 2 - System Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<2> {
        SYSRESETREQ_W::new(self)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping"]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PRIGROUP_W<8> {
        PRIGROUP_W::new(self)
    }
    #[doc = "Bit 15 - Data endianness, 0=little, 1=big"]
    #[inline(always)]
    #[must_use]
    pub fn endianness(&mut self) -> ENDIANNESS_W<15> {
        ENDIANNESS_W::new(self)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<16> {
        VECTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](index.html) module"]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aircr::R](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aircr::W](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0000;
}
