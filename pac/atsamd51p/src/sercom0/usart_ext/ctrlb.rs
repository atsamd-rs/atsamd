#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type CHSIZE_R = crate::FieldReader<CHSIZESELECT_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHSIZESELECT_A {
    #[doc = "0: 8 Bits"]
    _8_BIT = 0,
    #[doc = "1: 9 Bits"]
    _9_BIT = 1,
    #[doc = "5: 5 Bits"]
    _5_BIT = 5,
    #[doc = "6: 6 Bits"]
    _6_BIT = 6,
    #[doc = "7: 7 Bits"]
    _7_BIT = 7,
}
impl From<CHSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHSIZESELECT_A {
    type Ux = u8;
}
impl CHSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHSIZESELECT_A> {
        match self.bits {
            0 => Some(CHSIZESELECT_A::_8_BIT),
            1 => Some(CHSIZESELECT_A::_9_BIT),
            5 => Some(CHSIZESELECT_A::_5_BIT),
            6 => Some(CHSIZESELECT_A::_6_BIT),
            7 => Some(CHSIZESELECT_A::_7_BIT),
            _ => None,
        }
    }
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_8_BIT
    }
    #[doc = "9 Bits"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_9_BIT
    }
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_5_BIT
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_6_BIT
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == CHSIZESELECT_A::_7_BIT
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type CHSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CHSIZESELECT_A>;
impl<'a, REG, const O: u8> CHSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHSIZESELECT_A::_8_BIT)
    }
    #[doc = "9 Bits"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHSIZESELECT_A::_9_BIT)
    }
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHSIZESELECT_A::_5_BIT)
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHSIZESELECT_A::_6_BIT)
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHSIZESELECT_A::_7_BIT)
    }
}
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub type SBMODE_R = crate::BitReader<SBMODESELECT_A>;
#[doc = "Stop Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBMODESELECT_A {
    #[doc = "0: One Stop Bit"]
    _1_BIT = 0,
    #[doc = "1: Two Stop Bits"]
    _2_BIT = 1,
}
impl From<SBMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SBMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBMODESELECT_A {
        match self.bits {
            false => SBMODESELECT_A::_1_BIT,
            true => SBMODESELECT_A::_2_BIT,
        }
    }
    #[doc = "One Stop Bit"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == SBMODESELECT_A::_1_BIT
    }
    #[doc = "Two Stop Bits"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == SBMODESELECT_A::_2_BIT
    }
}
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub type SBMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SBMODESELECT_A>;
impl<'a, REG, const O: u8> SBMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One Stop Bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut crate::W<REG> {
        self.variant(SBMODESELECT_A::_1_BIT)
    }
    #[doc = "Two Stop Bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(SBMODESELECT_A::_2_BIT)
    }
}
#[doc = "Field `COLDEN` reader - Collision Detection Enable"]
pub type COLDEN_R = crate::BitReader;
#[doc = "Field `COLDEN` writer - Collision Detection Enable"]
pub type COLDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SFDE` reader - Start of Frame Detection Enable"]
pub type SFDE_R = crate::BitReader;
#[doc = "Field `SFDE` writer - Start of Frame Detection Enable"]
pub type SFDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENC` reader - Encoding Format"]
pub type ENC_R = crate::BitReader;
#[doc = "Field `ENC` writer - Encoding Format"]
pub type ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PMODE` reader - Parity Mode"]
pub type PMODE_R = crate::BitReader<PMODESELECT_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMODESELECT_A {
    #[doc = "0: Even Parity"]
    EVEN = 0,
    #[doc = "1: Odd Parity"]
    ODD = 1,
}
impl From<PMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMODESELECT_A {
        match self.bits {
            false => PMODESELECT_A::EVEN,
            true => PMODESELECT_A::ODD,
        }
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PMODESELECT_A::EVEN
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PMODESELECT_A::ODD
    }
}
#[doc = "Field `PMODE` writer - Parity Mode"]
pub type PMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PMODESELECT_A>;
impl<'a, REG, const O: u8> PMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PMODESELECT_A::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PMODESELECT_A::ODD)
    }
}
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TXEN_R = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINCMD` reader - LIN Command"]
pub type LINCMD_R = crate::FieldReader;
#[doc = "Field `LINCMD` writer - LIN Command"]
pub type LINCMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&self) -> SBMODE_R {
        SBMODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&self) -> COLDEN_R {
        COLDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SFDE_R {
        SFDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline(always)]
    pub fn lincmd(&self) -> LINCMD_R {
        LINCMD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn chsize(&mut self) -> CHSIZE_W<CTRLB_SPEC, 0> {
        CHSIZE_W::new(self)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sbmode(&mut self) -> SBMODE_W<CTRLB_SPEC, 6> {
        SBMODE_W::new(self)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn colden(&mut self) -> COLDEN_W<CTRLB_SPEC, 8> {
        COLDEN_W::new(self)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde(&mut self) -> SFDE_W<CTRLB_SPEC, 9> {
        SFDE_W::new(self)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<CTRLB_SPEC, 10> {
        ENC_W::new(self)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<CTRLB_SPEC, 13> {
        PMODE_W::new(self)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<CTRLB_SPEC, 16> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CTRLB_SPEC, 17> {
        RXEN_W::new(self)
    }
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline(always)]
    #[must_use]
    pub fn lincmd(&mut self) -> LINCMD_W<CTRLB_SPEC, 24> {
        LINCMD_W::new(self)
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
#[doc = "USART_EXT Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
