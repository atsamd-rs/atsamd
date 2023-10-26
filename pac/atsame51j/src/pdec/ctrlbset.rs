#[doc = "Register `CTRLBSET` reader"]
pub type R = crate::R<CTRLBSET_SPEC>;
#[doc = "Register `CTRLBSET` writer"]
pub type W = crate::W<CTRLBSET_SPEC>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LUPD_R = crate::BitReader;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<CMDSELECT_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Force a counter restart or retrigger"]
    RETRIGGER = 1,
    #[doc = "2: Force update of double buffered registers"]
    UPDATE = 2,
    #[doc = "3: Force a read synchronization of COUNT"]
    READSYNC = 3,
    #[doc = "4: Start QDEC/HALL"]
    START = 4,
    #[doc = "5: Stop QDEC/HALL"]
    STOP = 5,
}
impl From<CMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDSELECT_A {
    type Ux = u8;
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            0 => Some(CMDSELECT_A::NONE),
            1 => Some(CMDSELECT_A::RETRIGGER),
            2 => Some(CMDSELECT_A::UPDATE),
            3 => Some(CMDSELECT_A::READSYNC),
            4 => Some(CMDSELECT_A::START),
            5 => Some(CMDSELECT_A::STOP),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMDSELECT_A::NONE
    }
    #[doc = "Force a counter restart or retrigger"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == CMDSELECT_A::RETRIGGER
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == CMDSELECT_A::UPDATE
    }
    #[doc = "Force a read synchronization of COUNT"]
    #[inline(always)]
    pub fn is_readsync(&self) -> bool {
        *self == CMDSELECT_A::READSYNC
    }
    #[doc = "Start QDEC/HALL"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CMDSELECT_A::START
    }
    #[doc = "Stop QDEC/HALL"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CMDSELECT_A::STOP
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CMDSELECT_A>;
impl<'a, REG, const O: u8> CMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::NONE)
    }
    #[doc = "Force a counter restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::RETRIGGER)
    }
    #[doc = "Force update of double buffered registers"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::UPDATE)
    }
    #[doc = "Force a read synchronization of COUNT"]
    #[inline(always)]
    pub fn readsync(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::READSYNC)
    }
    #[doc = "Start QDEC/HALL"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::START)
    }
    #[doc = "Stop QDEC/HALL"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::STOP)
    }
}
impl R {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LUPD_W<CTRLBSET_SPEC, 1> {
        LUPD_W::new(self)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CTRLBSET_SPEC, 5> {
        CMD_W::new(self)
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
#[doc = "Control B Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlbset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlbset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLBSET_SPEC;
impl crate::RegisterSpec for CTRLBSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlbset::R`](R) reader structure"]
impl crate::Readable for CTRLBSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlbset::W`](W) writer structure"]
impl crate::Writable for CTRLBSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLBSET to value 0"]
impl crate::Resettable for CTRLBSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
