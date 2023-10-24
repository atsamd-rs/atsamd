#[doc = "Register `VREG` reader"]
pub type R = crate::R<VREG_SPEC>;
#[doc = "Register `VREG` writer"]
pub type W = crate::W<VREG_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEL` reader - Voltage Regulator Selection"]
pub type SEL_R = crate::BitReader<SELSELECT_A>;
#[doc = "Voltage Regulator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELSELECT_A {
    #[doc = "0: LDO selection"]
    LDO = 0,
    #[doc = "1: Buck selection"]
    BUCK = 1,
}
impl From<SELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELSELECT_A {
        match self.bits {
            false => SELSELECT_A::LDO,
            true => SELSELECT_A::BUCK,
        }
    }
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == SELSELECT_A::LDO
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == SELSELECT_A::BUCK
    }
}
#[doc = "Field `SEL` writer - Voltage Regulator Selection"]
pub type SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SELSELECT_A>;
impl<'a, REG, const O: u8> SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::LDO)
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn buck(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::BUCK)
    }
}
#[doc = "Field `RUNBKUP` reader - Run in Backup mode"]
pub type RUNBKUP_R = crate::BitReader;
#[doc = "Field `RUNBKUP` writer - Run in Backup mode"]
pub type RUNBKUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSEN` reader - Voltage Scaling Enable"]
pub type VSEN_R = crate::BitReader;
#[doc = "Field `VSEN` writer - Voltage Scaling Enable"]
pub type VSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSPER` reader - Voltage Scaling Period"]
pub type VSPER_R = crate::FieldReader;
#[doc = "Field `VSPER` writer - Voltage Scaling Period"]
pub type VSPER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&self) -> RUNBKUP_R {
        RUNBKUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VSEN_R {
        VSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&self) -> VSPER_R {
        VSPER_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<VREG_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<VREG_SPEC, 2> {
        SEL_W::new(self)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    #[must_use]
    pub fn runbkup(&mut self) -> RUNBKUP_W<VREG_SPEC, 7> {
        RUNBKUP_W::new(self)
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsen(&mut self) -> VSEN_W<VREG_SPEC, 16> {
        VSEN_W::new(self)
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    #[must_use]
    pub fn vsper(&mut self) -> VSPER_W<VREG_SPEC, 24> {
        VSPER_W::new(self)
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
#[doc = "VREG Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREG_SPEC;
impl crate::RegisterSpec for VREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg::R`](R) reader structure"]
impl crate::Readable for VREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vreg::W`](W) writer structure"]
impl crate::Writable for VREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREG to value 0x02"]
impl crate::Resettable for VREG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
