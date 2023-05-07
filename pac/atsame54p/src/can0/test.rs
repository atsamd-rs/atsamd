#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBCK` reader - Loop Back Mode"]
pub type LBCK_R = crate::BitReader<bool>;
#[doc = "Field `LBCK` writer - Loop Back Mode"]
pub type LBCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `TX` reader - Control of Transmit Pin"]
pub type TX_R = crate::FieldReader<u8, TXSELECT_A>;
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
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSELECT_A {
        match self.bits {
            0 => TXSELECT_A::CORE,
            1 => TXSELECT_A::SAMPLE,
            2 => TXSELECT_A::DOMINANT,
            3 => TXSELECT_A::RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == TXSELECT_A::CORE
    }
    #[doc = "Checks if the value of the field is `SAMPLE`"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == TXSELECT_A::SAMPLE
    }
    #[doc = "Checks if the value of the field is `DOMINANT`"]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == TXSELECT_A::DOMINANT
    }
    #[doc = "Checks if the value of the field is `RECESSIVE`"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == TXSELECT_A::RECESSIVE
    }
}
#[doc = "Field `TX` writer - Control of Transmit Pin"]
pub type TX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TEST_SPEC, u8, TXSELECT_A, 2, O>;
impl<'a, const O: u8> TX_W<'a, O> {
    #[doc = "TX controlled by CAN core"]
    #[inline(always)]
    pub fn core(self) -> &'a mut W {
        self.variant(TXSELECT_A::CORE)
    }
    #[doc = "TX monitoring sample point"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut W {
        self.variant(TXSELECT_A::SAMPLE)
    }
    #[doc = "Dominant (0) level at pin CAN_TX"]
    #[inline(always)]
    pub fn dominant(self) -> &'a mut W {
        self.variant(TXSELECT_A::DOMINANT)
    }
    #[doc = "Recessive (1) level at pin CAN_TX"]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut W {
        self.variant(TXSELECT_A::RECESSIVE)
    }
}
#[doc = "Field `RX` reader - Receive Pin"]
pub type RX_R = crate::BitReader<bool>;
#[doc = "Field `RX` writer - Receive Pin"]
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
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
    pub fn lbck(&mut self) -> LBCK_W<4> {
        LBCK_W::new(self)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<5> {
        TX_W::new(self)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<7> {
        RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
