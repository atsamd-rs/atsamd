#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AircrSpec>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AircrSpec>;
#[doc = "Field `VECTRESET` reader - Must write 0"]
pub type VectresetR = crate::BitReader;
#[doc = "Field `VECTRESET` writer - Must write 0"]
pub type VectresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTCLRACTIVE` reader - Must write 0"]
pub type VectclractiveR = crate::BitReader;
#[doc = "Field `VECTCLRACTIVE` writer - Must write 0"]
pub type VectclractiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysresetreqselect {
    #[doc = "0: No system reset request"]
    Value0 = 0,
    #[doc = "1: Asserts a signal to the outer system that requests a reset"]
    Value1 = 1,
}
impl From<Sysresetreqselect> for bool {
    #[inline(always)]
    fn from(variant: Sysresetreqselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRESETREQ` reader - System Reset Request"]
pub type SysresetreqR = crate::BitReader<Sysresetreqselect>;
impl SysresetreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysresetreqselect {
        match self.bits {
            false => Sysresetreqselect::Value0,
            true => Sysresetreqselect::Value1,
        }
    }
    #[doc = "No system reset request"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Sysresetreqselect::Value0
    }
    #[doc = "Asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Sysresetreqselect::Value1
    }
}
#[doc = "Field `SYSRESETREQ` writer - System Reset Request"]
pub type SysresetreqW<'a, REG> = crate::BitWriter<'a, REG, Sysresetreqselect>;
impl<'a, REG> SysresetreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No system reset request"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sysresetreqselect::Value0)
    }
    #[doc = "Asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sysresetreqselect::Value1)
    }
}
#[doc = "Field `PRIGROUP` reader - Interrupt priority grouping"]
pub type PrigroupR = crate::FieldReader;
#[doc = "Field `PRIGROUP` writer - Interrupt priority grouping"]
pub type PrigroupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Data endianness, 0=little, 1=big\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endiannessselect {
    #[doc = "0: Little-endian"]
    Value0 = 0,
    #[doc = "1: Big-endian"]
    Value1 = 1,
}
impl From<Endiannessselect> for bool {
    #[inline(always)]
    fn from(variant: Endiannessselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIANNESS` reader - Data endianness, 0=little, 1=big"]
pub type EndiannessR = crate::BitReader<Endiannessselect>;
impl EndiannessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endiannessselect {
        match self.bits {
            false => Endiannessselect::Value0,
            true => Endiannessselect::Value1,
        }
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Endiannessselect::Value0
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Endiannessselect::Value1
    }
}
#[doc = "Field `ENDIANNESS` writer - Data endianness, 0=little, 1=big"]
pub type EndiannessW<'a, REG> = crate::BitWriter<'a, REG, Endiannessselect>;
impl<'a, REG> EndiannessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Endiannessselect::Value0)
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Endiannessselect::Value1)
    }
}
#[doc = "Field `VECTKEY` reader - Register key"]
pub type VectkeyR = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - Register key"]
pub type VectkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Must write 0"]
    #[inline(always)]
    pub fn vectreset(&self) -> VectresetR {
        VectresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Must write 0"]
    #[inline(always)]
    pub fn vectclractive(&self) -> VectclractiveR {
        VectclractiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Reset Request"]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SysresetreqR {
        SysresetreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping"]
    #[inline(always)]
    pub fn prigroup(&self) -> PrigroupR {
        PrigroupR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Data endianness, 0=little, 1=big"]
    #[inline(always)]
    pub fn endianness(&self) -> EndiannessR {
        EndiannessR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VectkeyR {
        VectkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Must write 0"]
    #[inline(always)]
    #[must_use]
    pub fn vectreset(&mut self) -> VectresetW<AircrSpec> {
        VectresetW::new(self, 0)
    }
    #[doc = "Bit 1 - Must write 0"]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VectclractiveW<AircrSpec> {
        VectclractiveW::new(self, 1)
    }
    #[doc = "Bit 2 - System Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SysresetreqW<AircrSpec> {
        SysresetreqW::new(self, 2)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping"]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PrigroupW<AircrSpec> {
        PrigroupW::new(self, 8)
    }
    #[doc = "Bit 15 - Data endianness, 0=little, 1=big"]
    #[inline(always)]
    #[must_use]
    pub fn endianness(&mut self) -> EndiannessW<AircrSpec> {
        EndiannessW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VectkeyW<AircrSpec> {
        VectkeyW::new(self, 16)
    }
}
#[doc = "Application Interrupt and Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aircr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aircr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AircrSpec;
impl crate::RegisterSpec for AircrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aircr::R`](R) reader structure"]
impl crate::Readable for AircrSpec {}
#[doc = "`write(|w| ..)` method takes [`aircr::W`](W) writer structure"]
impl crate::Writable for AircrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AircrSpec {
    const RESET_VALUE: u32 = 0xfa05_0000;
}
