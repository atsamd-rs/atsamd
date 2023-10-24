#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `LBCK` reader - Loop Back Mode"]
pub type LBCK_R = crate::BitReader;
#[doc = "Field `LBCK` writer - Loop Back Mode"]
pub type LBCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX` reader - Control of Transmit Pin"]
pub type TX_R = crate::FieldReader<TXSELECT_A>;
#[doc = "Control of Transmit Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXSELECT_A {
    #[doc = "0: TX controlled by CAN core"]
    CORE = 0,
    #[doc = "1: TX monitoring sample point"]
    SAMPLE = 1,
    #[doc = "2: Dominant (0) level at pin CAN_TX"]
    DOMINANT = 2,
    #[doc = "3: Recessive (1) level at pin CAN_TX"]
    RECESSIVE = 3,
}
impl From<TXSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TXSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXSELECT_A {
    type Ux = u8;
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXSELECT_A {
        match self.bits {
            0 => TXSELECT_A::CORE,
            1 => TXSELECT_A::SAMPLE,
            2 => TXSELECT_A::DOMINANT,
            3 => TXSELECT_A::RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "TX controlled by CAN core"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == TXSELECT_A::CORE
    }
    #[doc = "TX monitoring sample point"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == TXSELECT_A::SAMPLE
    }
    #[doc = "Dominant (0) level at pin CAN_TX"]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == TXSELECT_A::DOMINANT
    }
    #[doc = "Recessive (1) level at pin CAN_TX"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == TXSELECT_A::RECESSIVE
    }
}
#[doc = "Field `TX` writer - Control of Transmit Pin"]
pub type TX_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TXSELECT_A>;
impl<'a, REG, const O: u8> TX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX controlled by CAN core"]
    #[inline(always)]
    pub fn core(self) -> &'a mut crate::W<REG> {
        self.variant(TXSELECT_A::CORE)
    }
    #[doc = "TX monitoring sample point"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut crate::W<REG> {
        self.variant(TXSELECT_A::SAMPLE)
    }
    #[doc = "Dominant (0) level at pin CAN_TX"]
    #[inline(always)]
    pub fn dominant(self) -> &'a mut crate::W<REG> {
        self.variant(TXSELECT_A::DOMINANT)
    }
    #[doc = "Recessive (1) level at pin CAN_TX"]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut crate::W<REG> {
        self.variant(TXSELECT_A::RECESSIVE)
    }
}
#[doc = "Field `RX` reader - Receive Pin"]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - Receive Pin"]
pub type RX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<TEST_SPEC, 4> {
        LBCK_W::new(self)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<TEST_SPEC, 5> {
        TX_W::new(self)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<TEST_SPEC, 7> {
        RX_W::new(self)
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
#[doc = "Test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
