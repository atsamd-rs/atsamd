#[doc = "Register `CHEVCTRL` reader"]
pub type R = crate::R<CHEVCTRL_SPEC>;
#[doc = "Register `CHEVCTRL` writer"]
pub type W = crate::W<CHEVCTRL_SPEC>;
#[doc = "Field `EVACT` reader - Channel Event Input Action"]
pub type EVACT_R = crate::FieldReader<EVACTSELECT_A>;
#[doc = "Channel Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACTSELECT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    TRIG = 1,
    #[doc = "2: Conditional transfer trigger"]
    CTRIG = 2,
    #[doc = "3: Conditional block transfer"]
    CBLOCK = 3,
    #[doc = "4: Channel suspend operation"]
    SUSPEND = 4,
    #[doc = "5: Channel resume operation"]
    RESUME = 5,
    #[doc = "6: Skip next block suspend action"]
    SSKIP = 6,
    #[doc = "7: Increase priority"]
    INCPRI = 7,
}
impl From<EVACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVACTSELECT_A {
    type Ux = u8;
}
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVACTSELECT_A {
        match self.bits {
            0 => EVACTSELECT_A::NOACT,
            1 => EVACTSELECT_A::TRIG,
            2 => EVACTSELECT_A::CTRIG,
            3 => EVACTSELECT_A::CBLOCK,
            4 => EVACTSELECT_A::SUSPEND,
            5 => EVACTSELECT_A::RESUME,
            6 => EVACTSELECT_A::SSKIP,
            7 => EVACTSELECT_A::INCPRI,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == EVACTSELECT_A::NOACT
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == EVACTSELECT_A::TRIG
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == EVACTSELECT_A::CTRIG
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == EVACTSELECT_A::CBLOCK
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == EVACTSELECT_A::SUSPEND
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == EVACTSELECT_A::RESUME
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == EVACTSELECT_A::SSKIP
    }
    #[doc = "Increase priority"]
    #[inline(always)]
    pub fn is_incpri(&self) -> bool {
        *self == EVACTSELECT_A::INCPRI
    }
}
#[doc = "Field `EVACT` writer - Channel Event Input Action"]
pub type EVACT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, EVACTSELECT_A>;
impl<'a, REG, const O: u8> EVACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::NOACT)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::TRIG)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::CTRIG)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::CBLOCK)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::RESUME)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::SSKIP)
    }
    #[doc = "Increase priority"]
    #[inline(always)]
    pub fn incpri(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::INCPRI)
    }
}
#[doc = "Field `EVOMODE` reader - Channel Event Output Mode"]
pub type EVOMODE_R = crate::FieldReader<EVOMODESELECT_A>;
#[doc = "Channel Event Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVOMODESELECT_A {
    #[doc = "0: Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    DEFAULT = 0,
    #[doc = "1: Ongoing trigger action"]
    TRIGACT = 1,
}
impl From<EVOMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVOMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVOMODESELECT_A {
    type Ux = u8;
}
impl EVOMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EVOMODESELECT_A> {
        match self.bits {
            0 => Some(EVOMODESELECT_A::DEFAULT),
            1 => Some(EVOMODESELECT_A::TRIGACT),
            _ => None,
        }
    }
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOMODESELECT_A::DEFAULT
    }
    #[doc = "Ongoing trigger action"]
    #[inline(always)]
    pub fn is_trigact(&self) -> bool {
        *self == EVOMODESELECT_A::TRIGACT
    }
}
#[doc = "Field `EVOMODE` writer - Channel Event Output Mode"]
pub type EVOMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, EVOMODESELECT_A>;
impl<'a, REG, const O: u8> EVOMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(EVOMODESELECT_A::DEFAULT)
    }
    #[doc = "Ongoing trigger action"]
    #[inline(always)]
    pub fn trigact(self) -> &'a mut crate::W<REG> {
        self.variant(EVOMODESELECT_A::TRIGACT)
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub type EVIE_R = crate::BitReader;
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub type EVIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub type EVOE_R = crate::BitReader;
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub type EVOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&self) -> EVOMODE_R {
        EVOMODE_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EVACT_W<CHEVCTRL_SPEC, 0> {
        EVACT_W::new(self)
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn evomode(&mut self) -> EVOMODE_W<CHEVCTRL_SPEC, 4> {
        EVOMODE_W::new(self)
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EVIE_W<CHEVCTRL_SPEC, 6> {
        EVIE_W::new(self)
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoe(&mut self) -> EVOE_W<CHEVCTRL_SPEC, 7> {
        EVOE_W::new(self)
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
#[doc = "Channel n Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chevctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chevctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHEVCTRL_SPEC;
impl crate::RegisterSpec for CHEVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chevctrl::R`](R) reader structure"]
impl crate::Readable for CHEVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chevctrl::W`](W) writer structure"]
impl crate::Writable for CHEVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHEVCTRL to value 0"]
impl crate::Resettable for CHEVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
