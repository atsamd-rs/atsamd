#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - Operation Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: QDEC operating mode"]
    QDEC = 0,
    #[doc = "1: HALL operating mode"]
    HALL = 1,
    #[doc = "2: COUNTER operating mode"]
    COUNTER = 2,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODESELECT_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::QDEC),
            1 => Some(MODESELECT_A::HALL),
            2 => Some(MODESELECT_A::COUNTER),
            _ => None,
        }
    }
    #[doc = "QDEC operating mode"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == MODESELECT_A::QDEC
    }
    #[doc = "HALL operating mode"]
    #[inline(always)]
    pub fn is_hall(&self) -> bool {
        *self == MODESELECT_A::HALL
    }
    #[doc = "COUNTER operating mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == MODESELECT_A::COUNTER
    }
}
#[doc = "Field `MODE` writer - Operation Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MODESELECT_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QDEC operating mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::QDEC)
    }
    #[doc = "HALL operating mode"]
    #[inline(always)]
    pub fn hall(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::HALL)
    }
    #[doc = "COUNTER operating mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::COUNTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CONF` reader - PDEC Configuration"]
pub type CONF_R = crate::FieldReader<CONFSELECT_A>;
#[doc = "PDEC Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONFSELECT_A {
    #[doc = "0: Quadrature decoder direction"]
    X4 = 0,
    #[doc = "1: Secure Quadrature decoder direction"]
    X4S = 1,
    #[doc = "2: Decoder direction"]
    X2 = 2,
    #[doc = "3: Secure decoder direction"]
    X2S = 3,
    #[doc = "4: Auto correction mode"]
    AUTOC = 4,
}
impl From<CONFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CONFSELECT_A {
    type Ux = u8;
}
impl CONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CONFSELECT_A> {
        match self.bits {
            0 => Some(CONFSELECT_A::X4),
            1 => Some(CONFSELECT_A::X4S),
            2 => Some(CONFSELECT_A::X2),
            3 => Some(CONFSELECT_A::X2S),
            4 => Some(CONFSELECT_A::AUTOC),
            _ => None,
        }
    }
    #[doc = "Quadrature decoder direction"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == CONFSELECT_A::X4
    }
    #[doc = "Secure Quadrature decoder direction"]
    #[inline(always)]
    pub fn is_x4s(&self) -> bool {
        *self == CONFSELECT_A::X4S
    }
    #[doc = "Decoder direction"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == CONFSELECT_A::X2
    }
    #[doc = "Secure decoder direction"]
    #[inline(always)]
    pub fn is_x2s(&self) -> bool {
        *self == CONFSELECT_A::X2S
    }
    #[doc = "Auto correction mode"]
    #[inline(always)]
    pub fn is_autoc(&self) -> bool {
        *self == CONFSELECT_A::AUTOC
    }
}
#[doc = "Field `CONF` writer - PDEC Configuration"]
pub type CONF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CONFSELECT_A>;
impl<'a, REG, const O: u8> CONF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(CONFSELECT_A::X4)
    }
    #[doc = "Secure Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4s(self) -> &'a mut crate::W<REG> {
        self.variant(CONFSELECT_A::X4S)
    }
    #[doc = "Decoder direction"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(CONFSELECT_A::X2)
    }
    #[doc = "Secure decoder direction"]
    #[inline(always)]
    pub fn x2s(self) -> &'a mut crate::W<REG> {
        self.variant(CONFSELECT_A::X2S)
    }
    #[doc = "Auto correction mode"]
    #[inline(always)]
    pub fn autoc(self) -> &'a mut crate::W<REG> {
        self.variant(CONFSELECT_A::AUTOC)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type ALOCK_R = crate::BitReader;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type ALOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWAP` reader - PDEC Phase A and B Swap"]
pub type SWAP_R = crate::BitReader;
#[doc = "Field `SWAP` writer - PDEC Phase A and B Swap"]
pub type SWAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREN` reader - Period Enable"]
pub type PEREN_R = crate::BitReader;
#[doc = "Field `PEREN` writer - Period Enable"]
pub type PEREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINEN0` reader - PDEC Input From Pin 0 Enable"]
pub type PINEN0_R = crate::BitReader;
#[doc = "Field `PINEN0` writer - PDEC Input From Pin 0 Enable"]
pub type PINEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINEN1` reader - PDEC Input From Pin 1 Enable"]
pub type PINEN1_R = crate::BitReader;
#[doc = "Field `PINEN1` writer - PDEC Input From Pin 1 Enable"]
pub type PINEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINEN2` reader - PDEC Input From Pin 2 Enable"]
pub type PINEN2_R = crate::BitReader;
#[doc = "Field `PINEN2` writer - PDEC Input From Pin 2 Enable"]
pub type PINEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINVEN0` reader - IO Pin 0 Invert Enable"]
pub type PINVEN0_R = crate::BitReader;
#[doc = "Field `PINVEN0` writer - IO Pin 0 Invert Enable"]
pub type PINVEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINVEN1` reader - IO Pin 1 Invert Enable"]
pub type PINVEN1_R = crate::BitReader;
#[doc = "Field `PINVEN1` writer - IO Pin 1 Invert Enable"]
pub type PINVEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINVEN2` reader - IO Pin 2 Invert Enable"]
pub type PINVEN2_R = crate::BitReader;
#[doc = "Field `PINVEN2` writer - IO Pin 2 Invert Enable"]
pub type PINVEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ANGULAR` reader - Angular Counter Length"]
pub type ANGULAR_R = crate::FieldReader;
#[doc = "Field `ANGULAR` writer - Angular Counter Length"]
pub type ANGULAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MAXCMP` reader - Maximum Consecutive Missing Pulses"]
pub type MAXCMP_R = crate::FieldReader;
#[doc = "Field `MAXCMP` writer - Maximum Consecutive Missing Pulses"]
pub type MAXCMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    pub fn pinen0(&self) -> PINEN0_R {
        PINEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    pub fn pinen1(&self) -> PINEN1_R {
        PINEN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    pub fn pinen2(&self) -> PINEN2_R {
        PINEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    pub fn pinven0(&self) -> PINVEN0_R {
        PINVEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    pub fn pinven1(&self) -> PINVEN1_R {
        PINVEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    pub fn pinven2(&self) -> PINVEN2_R {
        PINVEN2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    pub fn angular(&self) -> ANGULAR_R {
        ANGULAR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&self) -> MAXCMP_R {
        MAXCMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTRLA_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLA_SPEC, 2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn conf(&mut self) -> CONF_W<CTRLA_SPEC, 8> {
        CONF_W::new(self)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    #[must_use]
    pub fn alock(&mut self) -> ALOCK_W<CTRLA_SPEC, 11> {
        ALOCK_W::new(self)
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<CTRLA_SPEC, 14> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peren(&mut self) -> PEREN_W<CTRLA_SPEC, 15> {
        PEREN_W::new(self)
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinen0(&mut self) -> PINEN0_W<CTRLA_SPEC, 16> {
        PINEN0_W::new(self)
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinen1(&mut self) -> PINEN1_W<CTRLA_SPEC, 17> {
        PINEN1_W::new(self)
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinen2(&mut self) -> PINEN2_W<CTRLA_SPEC, 18> {
        PINEN2_W::new(self)
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinven0(&mut self) -> PINVEN0_W<CTRLA_SPEC, 20> {
        PINVEN0_W::new(self)
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinven1(&mut self) -> PINVEN1_W<CTRLA_SPEC, 21> {
        PINVEN1_W::new(self)
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pinven2(&mut self) -> PINVEN2_W<CTRLA_SPEC, 22> {
        PINVEN2_W::new(self)
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    #[must_use]
    pub fn angular(&mut self) -> ANGULAR_W<CTRLA_SPEC, 24> {
        ANGULAR_W::new(self)
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    #[must_use]
    pub fn maxcmp(&mut self) -> MAXCMP_W<CTRLA_SPEC, 28> {
        MAXCMP_W::new(self)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
