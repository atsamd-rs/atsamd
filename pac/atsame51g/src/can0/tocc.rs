#[doc = "Register `TOCC` reader"]
pub type R = crate::R<TOCC_SPEC>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<TOCC_SPEC>;
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub type ETOC_R = crate::BitReader;
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type ETOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TOS_R = crate::FieldReader<TOSSELECT_A>;
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOSSELECT_A {
    #[doc = "0: Continuout operation"]
    CONT = 0,
    #[doc = "1: Timeout controlled by TX Event FIFO"]
    TXEF = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    RXF0 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    RXF1 = 3,
}
impl From<TOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOSSELECT_A {
    type Ux = u8;
}
impl TOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOSSELECT_A {
        match self.bits {
            0 => TOSSELECT_A::CONT,
            1 => TOSSELECT_A::TXEF,
            2 => TOSSELECT_A::RXF0,
            3 => TOSSELECT_A::RXF1,
            _ => unreachable!(),
        }
    }
    #[doc = "Continuout operation"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == TOSSELECT_A::CONT
    }
    #[doc = "Timeout controlled by TX Event FIFO"]
    #[inline(always)]
    pub fn is_txef(&self) -> bool {
        *self == TOSSELECT_A::TXEF
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == TOSSELECT_A::RXF0
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == TOSSELECT_A::RXF1
    }
}
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TOS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TOSSELECT_A>;
impl<'a, REG, const O: u8> TOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuout operation"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(TOSSELECT_A::CONT)
    }
    #[doc = "Timeout controlled by TX Event FIFO"]
    #[inline(always)]
    pub fn txef(self) -> &'a mut crate::W<REG> {
        self.variant(TOSSELECT_A::TXEF)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut crate::W<REG> {
        self.variant(TOSSELECT_A::RXF0)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut crate::W<REG> {
        self.variant(TOSSELECT_A::RXF1)
    }
}
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TOP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<TOCC_SPEC, 0> {
        ETOC_W::new(self)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<TOCC_SPEC, 1> {
        TOS_W::new(self)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<TOCC_SPEC, 16> {
        TOP_W::new(self)
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
#[doc = "Timeout Counter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for TOCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for TOCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for TOCC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
