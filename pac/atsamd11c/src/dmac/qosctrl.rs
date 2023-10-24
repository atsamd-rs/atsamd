#[doc = "Register `QOSCTRL` reader"]
pub type R = crate::R<QOSCTRL_SPEC>;
#[doc = "Register `QOSCTRL` writer"]
pub type W = crate::W<QOSCTRL_SPEC>;
#[doc = "Field `WRBQOS` reader - Write-Back Quality of Service"]
pub type WRBQOS_R = crate::FieldReader<WRBQOSSELECT_A>;
#[doc = "Write-Back Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRBQOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<WRBQOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WRBQOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRBQOSSELECT_A {
    type Ux = u8;
}
impl WRBQOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRBQOSSELECT_A {
        match self.bits {
            0 => WRBQOSSELECT_A::DISABLE,
            1 => WRBQOSSELECT_A::LOW,
            2 => WRBQOSSELECT_A::MEDIUM,
            3 => WRBQOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WRBQOSSELECT_A::DISABLE
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WRBQOSSELECT_A::LOW
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == WRBQOSSELECT_A::MEDIUM
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WRBQOSSELECT_A::HIGH
    }
}
#[doc = "Field `WRBQOS` writer - Write-Back Quality of Service"]
pub type WRBQOS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, WRBQOSSELECT_A>;
impl<'a, REG, const O: u8> WRBQOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WRBQOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(WRBQOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(WRBQOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(WRBQOSSELECT_A::HIGH)
    }
}
#[doc = "Field `FQOS` reader - Fetch Quality of Service"]
pub type FQOS_R = crate::FieldReader<FQOSSELECT_A>;
#[doc = "Fetch Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FQOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<FQOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FQOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FQOSSELECT_A {
    type Ux = u8;
}
impl FQOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FQOSSELECT_A {
        match self.bits {
            0 => FQOSSELECT_A::DISABLE,
            1 => FQOSSELECT_A::LOW,
            2 => FQOSSELECT_A::MEDIUM,
            3 => FQOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FQOSSELECT_A::DISABLE
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FQOSSELECT_A::LOW
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == FQOSSELECT_A::MEDIUM
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FQOSSELECT_A::HIGH
    }
}
#[doc = "Field `FQOS` writer - Fetch Quality of Service"]
pub type FQOS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, FQOSSELECT_A>;
impl<'a, REG, const O: u8> FQOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FQOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(FQOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(FQOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(FQOSSELECT_A::HIGH)
    }
}
#[doc = "Field `DQOS` reader - Data Transfer Quality of Service"]
pub type DQOS_R = crate::FieldReader<DQOSSELECT_A>;
#[doc = "Data Transfer Quality of Service\n\nValue on reset: 1"]
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
#[doc = "Field `DQOS` writer - Data Transfer Quality of Service"]
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
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline(always)]
    pub fn wrbqos(&self) -> WRBQOS_R {
        WRBQOS_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline(always)]
    pub fn fqos(&self) -> FQOS_R {
        FQOS_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DQOS_R {
        DQOS_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn wrbqos(&mut self) -> WRBQOS_W<QOSCTRL_SPEC, 0> {
        WRBQOS_W::new(self)
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn fqos(&mut self) -> FQOS_W<QOSCTRL_SPEC, 2> {
        FQOS_W::new(self)
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn dqos(&mut self) -> DQOS_W<QOSCTRL_SPEC, 4> {
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
#[doc = "QOS Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qosctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qosctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets QOSCTRL to value 0x15"]
impl crate::Resettable for QOSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x15;
}
