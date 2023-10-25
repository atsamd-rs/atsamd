#[doc = "Register `PCR` reader"]
pub type R = crate::R<PCR_SPEC>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PCR_SPEC>;
#[doc = "Field `SDBPWR` reader - SD Bus Power"]
pub type SDBPWR_R = crate::BitReader<SDBPWRSELECT_A>;
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDBPWRSELECT_A {
    #[doc = "0: Power off"]
    OFF = 0,
    #[doc = "1: Power on"]
    ON = 1,
}
impl From<SDBPWRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SDBPWRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDBPWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDBPWRSELECT_A {
        match self.bits {
            false => SDBPWRSELECT_A::OFF,
            true => SDBPWRSELECT_A::ON,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SDBPWRSELECT_A::OFF
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SDBPWRSELECT_A::ON
    }
}
#[doc = "Field `SDBPWR` writer - SD Bus Power"]
pub type SDBPWR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SDBPWRSELECT_A>;
impl<'a, REG, const O: u8> SDBPWR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SDBPWRSELECT_A::OFF)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(SDBPWRSELECT_A::ON)
    }
}
#[doc = "Field `SDBVSEL` reader - SD Bus Voltage Select"]
pub type SDBVSEL_R = crate::FieldReader<SDBVSELSELECT_A>;
#[doc = "SD Bus Voltage Select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDBVSELSELECT_A {
    #[doc = "5: 1.8V (Typ.)"]
    _1V8 = 5,
    #[doc = "6: 3.0V (Typ.)"]
    _3V0 = 6,
    #[doc = "7: 3.3V (Typ.)"]
    _3V3 = 7,
}
impl From<SDBVSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SDBVSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDBVSELSELECT_A {
    type Ux = u8;
}
impl SDBVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDBVSELSELECT_A> {
        match self.bits {
            5 => Some(SDBVSELSELECT_A::_1V8),
            6 => Some(SDBVSELSELECT_A::_3V0),
            7 => Some(SDBVSELSELECT_A::_3V3),
            _ => None,
        }
    }
    #[doc = "1.8V (Typ.)"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == SDBVSELSELECT_A::_1V8
    }
    #[doc = "3.0V (Typ.)"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        *self == SDBVSELSELECT_A::_3V0
    }
    #[doc = "3.3V (Typ.)"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == SDBVSELSELECT_A::_3V3
    }
}
#[doc = "Field `SDBVSEL` writer - SD Bus Voltage Select"]
pub type SDBVSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SDBVSELSELECT_A>;
impl<'a, REG, const O: u8> SDBVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.8V (Typ.)"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut crate::W<REG> {
        self.variant(SDBVSELSELECT_A::_1V8)
    }
    #[doc = "3.0V (Typ.)"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut crate::W<REG> {
        self.variant(SDBVSELSELECT_A::_3V0)
    }
    #[doc = "3.3V (Typ.)"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut crate::W<REG> {
        self.variant(SDBVSELSELECT_A::_3V3)
    }
}
impl R {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&self) -> SDBPWR_R {
        SDBPWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbvsel(&self) -> SDBVSEL_R {
        SDBVSEL_R::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    #[must_use]
    pub fn sdbpwr(&mut self) -> SDBPWR_W<PCR_SPEC, 0> {
        SDBPWR_W::new(self)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdbvsel(&mut self) -> SDBVSEL_W<PCR_SPEC, 1> {
        SDBVSEL_W::new(self)
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
#[doc = "Power Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR to value 0x0e"]
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
