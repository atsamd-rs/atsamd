#[doc = "Register `QOSCTRL` reader"]
pub type R = crate::R<QOSCTRL_SPEC>;
#[doc = "Register `QOSCTRL` writer"]
pub type W = crate::W<QOSCTRL_SPEC>;
#[doc = "Field `CQOS` reader - Configuration Quality of Service"]
pub type CQOS_R = crate::FieldReader<CQOSSELECT_A>;
#[doc = "Configuration Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CQOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<CQOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CQOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CQOSSELECT_A {
    type Ux = u8;
}
impl CQOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CQOSSELECT_A {
        match self.bits {
            0 => CQOSSELECT_A::DISABLE,
            1 => CQOSSELECT_A::LOW,
            2 => CQOSSELECT_A::MEDIUM,
            3 => CQOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CQOSSELECT_A::DISABLE
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CQOSSELECT_A::LOW
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == CQOSSELECT_A::MEDIUM
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CQOSSELECT_A::HIGH
    }
}
#[doc = "Field `CQOS` writer - Configuration Quality of Service"]
pub type CQOS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CQOSSELECT_A>;
impl<'a, REG, const O: u8> CQOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CQOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CQOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(CQOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CQOSSELECT_A::HIGH)
    }
}
#[doc = "Field `DQOS` reader - Data Quality of Service"]
pub type DQOS_R = crate::FieldReader<DQOSSELECT_A>;
#[doc = "Data Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DQOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<DQOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DQOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DQOSSELECT_A {
    type Ux = u8;
}
impl DQOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DQOSSELECT_A {
        match self.bits {
            0 => DQOSSELECT_A::DISABLE,
            1 => DQOSSELECT_A::LOW,
            2 => DQOSSELECT_A::MEDIUM,
            3 => DQOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DQOSSELECT_A::DISABLE
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DQOSSELECT_A::LOW
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == DQOSSELECT_A::MEDIUM
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DQOSSELECT_A::HIGH
    }
}
#[doc = "Field `DQOS` writer - Data Quality of Service"]
pub type DQOS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DQOSSELECT_A>;
impl<'a, REG, const O: u8> DQOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DQOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(DQOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(DQOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(DQOSSELECT_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&self) -> CQOS_R {
        CQOS_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DQOS_R {
        DQOS_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn cqos(&mut self) -> CQOS_W<QOSCTRL_SPEC, 0> {
        CQOS_W::new(self)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn dqos(&mut self) -> DQOS_W<QOSCTRL_SPEC, 2> {
        DQOS_W::new(self)
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
#[doc = "USB Quality Of Service\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qosctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qosctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QOSCTRL_SPEC;
impl crate::RegisterSpec for QOSCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`qosctrl::R`](R) reader structure"]
impl crate::Readable for QOSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qosctrl::W`](W) writer structure"]
impl crate::Writable for QOSCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QOSCTRL to value 0x05"]
impl crate::Resettable for QOSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
