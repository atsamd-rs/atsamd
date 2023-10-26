#[doc = "Register `VREF` reader"]
pub type R = crate::R<VREF_SPEC>;
#[doc = "Register `VREF` writer"]
pub type W = crate::W<VREF_SPEC>;
#[doc = "Field `TSEN` reader - Temperature Sensor Output Enable"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature Sensor Output Enable"]
pub type TSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFOE` reader - Voltage Reference Output Enable"]
pub type VREFOE_R = crate::BitReader;
#[doc = "Field `VREFOE` writer - Voltage Reference Output Enable"]
pub type VREFOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSSEL` reader - Temperature Sensor Selection"]
pub type TSSEL_R = crate::BitReader;
#[doc = "Field `TSSEL` writer - Temperature Sensor Selection"]
pub type TSSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Contrl"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Contrl"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEL` reader - Voltage Reference Selection"]
pub type SEL_R = crate::FieldReader<SELSELECT_A>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELSELECT_A {
    #[doc = "0: 1.0V voltage reference typical value"]
    _1V0 = 0,
    #[doc = "1: 1.1V voltage reference typical value"]
    _1V1 = 1,
    #[doc = "2: 1.2V voltage reference typical value"]
    _1V2 = 2,
    #[doc = "3: 1.25V voltage reference typical value"]
    _1V25 = 3,
    #[doc = "4: 2.0V voltage reference typical value"]
    _2V0 = 4,
    #[doc = "5: 2.2V voltage reference typical value"]
    _2V2 = 5,
    #[doc = "6: 2.4V voltage reference typical value"]
    _2V4 = 6,
    #[doc = "7: 2.5V voltage reference typical value"]
    _2V5 = 7,
}
impl From<SELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELSELECT_A {
    type Ux = u8;
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SELSELECT_A> {
        match self.bits {
            0 => Some(SELSELECT_A::_1V0),
            1 => Some(SELSELECT_A::_1V1),
            2 => Some(SELSELECT_A::_1V2),
            3 => Some(SELSELECT_A::_1V25),
            4 => Some(SELSELECT_A::_2V0),
            5 => Some(SELSELECT_A::_2V2),
            6 => Some(SELSELECT_A::_2V4),
            7 => Some(SELSELECT_A::_2V5),
            _ => None,
        }
    }
    #[doc = "1.0V voltage reference typical value"]
    #[inline(always)]
    pub fn is_1v0(&self) -> bool {
        *self == SELSELECT_A::_1V0
    }
    #[doc = "1.1V voltage reference typical value"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        *self == SELSELECT_A::_1V1
    }
    #[doc = "1.2V voltage reference typical value"]
    #[inline(always)]
    pub fn is_1v2(&self) -> bool {
        *self == SELSELECT_A::_1V2
    }
    #[doc = "1.25V voltage reference typical value"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == SELSELECT_A::_1V25
    }
    #[doc = "2.0V voltage reference typical value"]
    #[inline(always)]
    pub fn is_2v0(&self) -> bool {
        *self == SELSELECT_A::_2V0
    }
    #[doc = "2.2V voltage reference typical value"]
    #[inline(always)]
    pub fn is_2v2(&self) -> bool {
        *self == SELSELECT_A::_2V2
    }
    #[doc = "2.4V voltage reference typical value"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == SELSELECT_A::_2V4
    }
    #[doc = "2.5V voltage reference typical value"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == SELSELECT_A::_2V5
    }
}
#[doc = "Field `SEL` writer - Voltage Reference Selection"]
pub type SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, SELSELECT_A>;
impl<'a, REG, const O: u8> SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v0(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_1V0)
    }
    #[doc = "1.1V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_1V1)
    }
    #[doc = "1.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v2(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_1V2)
    }
    #[doc = "1.25V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_1V25)
    }
    #[doc = "2.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v0(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_2V0)
    }
    #[doc = "2.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v2(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_2V2)
    }
    #[doc = "2.4V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_2V4)
    }
    #[doc = "2.5V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::_2V5)
    }
}
impl R {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn vrefoe(&self) -> VREFOE_R {
        VREFOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<VREF_SPEC, 1> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefoe(&mut self) -> VREFOE_W<VREF_SPEC, 2> {
        VREFOE_W::new(self)
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tssel(&mut self) -> TSSEL_W<VREF_SPEC, 3> {
        TSSEL_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<VREF_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<VREF_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<VREF_SPEC, 16> {
        SEL_W::new(self)
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
#[doc = "VREF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREF_SPEC;
impl crate::RegisterSpec for VREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref::R`](R) reader structure"]
impl crate::Readable for VREF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vref::W`](W) writer structure"]
impl crate::Writable for VREF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
