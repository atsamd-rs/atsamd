#[doc = "Register `MRCFG` reader"]
pub type R = crate::R<MRCFG_SPEC>;
#[doc = "Register `MRCFG` writer"]
pub type W = crate::W<MRCFG_SPEC>;
#[doc = "Field `QOS` reader - Quality of Service"]
pub type QOS_R = crate::FieldReader<QOSSELECT_A>;
#[doc = "Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QOSSELECT_A {
    #[doc = "0: Background (no sensitive operation)"]
    DISABLE = 0,
    #[doc = "1: Sensitive Bandwidth"]
    LOW = 1,
    #[doc = "2: Sensitive Latency"]
    MEDIUM = 2,
    #[doc = "3: Critical Latency"]
    HIGH = 3,
}
impl From<QOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: QOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QOSSELECT_A {
    type Ux = u8;
}
impl QOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QOSSELECT_A {
        match self.bits {
            0 => QOSSELECT_A::DISABLE,
            1 => QOSSELECT_A::LOW,
            2 => QOSSELECT_A::MEDIUM,
            3 => QOSSELECT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == QOSSELECT_A::DISABLE
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == QOSSELECT_A::LOW
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == QOSSELECT_A::MEDIUM
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == QOSSELECT_A::HIGH
    }
}
#[doc = "Field `QOS` writer - Quality of Service"]
pub type QOS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, QOSSELECT_A>;
impl<'a, REG, const O: u8> QOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(QOSSELECT_A::DISABLE)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(QOSSELECT_A::LOW)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(QOSSELECT_A::MEDIUM)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(QOSSELECT_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    pub fn qos(&self) -> QOS_R {
        QOS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn qos(&mut self) -> QOS_W<MRCFG_SPEC, 0> {
        QOS_W::new(self)
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
#[doc = "Message RAM Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MRCFG_SPEC;
impl crate::RegisterSpec for MRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcfg::R`](R) reader structure"]
impl crate::Readable for MRCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mrcfg::W`](W) writer structure"]
impl crate::Writable for MRCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRCFG to value 0x02"]
impl crate::Resettable for MRCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
