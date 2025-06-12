#[doc = "Register `PCR` reader"]
pub type R = crate::R<PcrSpec>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PcrSpec>;
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdbpwrselect {
    #[doc = "0: Power off"]
    Off = 0,
    #[doc = "1: Power on"]
    On = 1,
}
impl From<Sdbpwrselect> for bool {
    #[inline(always)]
    fn from(variant: Sdbpwrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDBPWR` reader - SD Bus Power"]
pub type SdbpwrR = crate::BitReader<Sdbpwrselect>;
impl SdbpwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdbpwrselect {
        match self.bits {
            false => Sdbpwrselect::Off,
            true => Sdbpwrselect::On,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sdbpwrselect::Off
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sdbpwrselect::On
    }
}
#[doc = "Field `SDBPWR` writer - SD Bus Power"]
pub type SdbpwrW<'a, REG> = crate::BitWriter<'a, REG, Sdbpwrselect>;
impl<'a, REG> SdbpwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbpwrselect::Off)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbpwrselect::On)
    }
}
#[doc = "SD Bus Voltage Select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdbvselselect {
    #[doc = "5: 1.8V (Typ.)"]
    _1v8 = 5,
    #[doc = "6: 3.0V (Typ.)"]
    _3v0 = 6,
    #[doc = "7: 3.3V (Typ.)"]
    _3v3 = 7,
}
impl From<Sdbvselselect> for u8 {
    #[inline(always)]
    fn from(variant: Sdbvselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdbvselselect {
    type Ux = u8;
}
impl crate::IsEnum for Sdbvselselect {}
#[doc = "Field `SDBVSEL` reader - SD Bus Voltage Select"]
pub type SdbvselR = crate::FieldReader<Sdbvselselect>;
impl SdbvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdbvselselect> {
        match self.bits {
            5 => Some(Sdbvselselect::_1v8),
            6 => Some(Sdbvselselect::_3v0),
            7 => Some(Sdbvselselect::_3v3),
            _ => None,
        }
    }
    #[doc = "1.8V (Typ.)"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == Sdbvselselect::_1v8
    }
    #[doc = "3.0V (Typ.)"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        *self == Sdbvselselect::_3v0
    }
    #[doc = "3.3V (Typ.)"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == Sdbvselselect::_3v3
    }
}
#[doc = "Field `SDBVSEL` writer - SD Bus Voltage Select"]
pub type SdbvselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sdbvselselect>;
impl<'a, REG> SdbvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.8V (Typ.)"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbvselselect::_1v8)
    }
    #[doc = "3.0V (Typ.)"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbvselselect::_3v0)
    }
    #[doc = "3.3V (Typ.)"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbvselselect::_3v3)
    }
}
impl R {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&self) -> SdbpwrR {
        SdbpwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbvsel(&self) -> SdbvselR {
        SdbvselR::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&mut self) -> SdbpwrW<PcrSpec> {
        SdbpwrW::new(self, 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbvsel(&mut self) -> SdbvselW<PcrSpec> {
        SdbvselW::new(self, 1)
    }
}
#[doc = "Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCR to value 0x0e"]
impl crate::Resettable for PcrSpec {
    const RESET_VALUE: u8 = 0x0e;
}
