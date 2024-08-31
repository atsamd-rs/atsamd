#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsizeselect {
    #[doc = "0: 8 Bits"]
    _8Bit = 0,
    #[doc = "1: 9 Bits"]
    _9Bit = 1,
    #[doc = "5: 5 Bits"]
    _5Bit = 5,
    #[doc = "6: 6 Bits"]
    _6Bit = 6,
    #[doc = "7: 7 Bits"]
    _7Bit = 7,
}
impl From<Chsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Chsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Chsizeselect {}
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type ChsizeR = crate::FieldReader<Chsizeselect>;
impl ChsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsizeselect> {
        match self.bits {
            0 => Some(Chsizeselect::_8Bit),
            1 => Some(Chsizeselect::_9Bit),
            5 => Some(Chsizeselect::_5Bit),
            6 => Some(Chsizeselect::_6Bit),
            7 => Some(Chsizeselect::_7Bit),
            _ => None,
        }
    }
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Chsizeselect::_8Bit
    }
    #[doc = "9 Bits"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Chsizeselect::_9Bit
    }
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Chsizeselect::_5Bit
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Chsizeselect::_6Bit
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Chsizeselect::_7Bit
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type ChsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chsizeselect>;
impl<'a, REG> ChsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_8Bit)
    }
    #[doc = "9 Bits"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_9Bit)
    }
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_5Bit)
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_6Bit)
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_7Bit)
    }
}
#[doc = "Stop Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbmodeselect {
    #[doc = "0: One Stop Bit"]
    _1Bit = 0,
    #[doc = "1: Two Stop Bits"]
    _2Bit = 1,
}
impl From<Sbmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Sbmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub type SbmodeR = crate::BitReader<Sbmodeselect>;
impl SbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbmodeselect {
        match self.bits {
            false => Sbmodeselect::_1Bit,
            true => Sbmodeselect::_2Bit,
        }
    }
    #[doc = "One Stop Bit"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == Sbmodeselect::_1Bit
    }
    #[doc = "Two Stop Bits"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == Sbmodeselect::_2Bit
    }
}
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub type SbmodeW<'a, REG> = crate::BitWriter<'a, REG, Sbmodeselect>;
impl<'a, REG> SbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One Stop Bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Sbmodeselect::_1Bit)
    }
    #[doc = "Two Stop Bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Sbmodeselect::_2Bit)
    }
}
#[doc = "Field `COLDEN` reader - Collision Detection Enable"]
pub type ColdenR = crate::BitReader;
#[doc = "Field `COLDEN` writer - Collision Detection Enable"]
pub type ColdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFDE` reader - Start of Frame Detection Enable"]
pub type SfdeR = crate::BitReader;
#[doc = "Field `SFDE` writer - Start of Frame Detection Enable"]
pub type SfdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - Encoding Format"]
pub type EncR = crate::BitReader;
#[doc = "Field `ENC` writer - Encoding Format"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmodeselect {
    #[doc = "0: Even Parity"]
    Even = 0,
    #[doc = "1: Odd Parity"]
    Odd = 1,
}
impl From<Pmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Pmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Parity Mode"]
pub type PmodeR = crate::BitReader<Pmodeselect>;
impl PmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmodeselect {
        match self.bits {
            false => Pmodeselect::Even,
            true => Pmodeselect::Odd,
        }
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Pmodeselect::Even
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Pmodeselect::Odd
    }
}
#[doc = "Field `PMODE` writer - Parity Mode"]
pub type PmodeW<'a, REG> = crate::BitWriter<'a, REG, Pmodeselect>;
impl<'a, REG> PmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Pmodeselect::Even)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Pmodeselect::Odd)
    }
}
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCMD` reader - LIN Command"]
pub type LincmdR = crate::FieldReader;
#[doc = "Field `LINCMD` writer - LIN Command"]
pub type LincmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> ChsizeR {
        ChsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&self) -> SbmodeR {
        SbmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&self) -> ColdenR {
        ColdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SfdeR {
        SfdeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PmodeR {
        PmodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline(always)]
    pub fn lincmd(&self) -> LincmdR {
        LincmdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn chsize(&mut self) -> ChsizeW<CtrlbSpec> {
        ChsizeW::new(self, 0)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sbmode(&mut self) -> SbmodeW<CtrlbSpec> {
        SbmodeW::new(self, 6)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn colden(&mut self) -> ColdenW<CtrlbSpec> {
        ColdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde(&mut self) -> SfdeW<CtrlbSpec> {
        SfdeW::new(self, 9)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> EncW<CtrlbSpec> {
        EncW::new(self, 10)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PmodeW<CtrlbSpec> {
        PmodeW::new(self, 13)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<CtrlbSpec> {
        TxenW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<CtrlbSpec> {
        RxenW::new(self, 17)
    }
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline(always)]
    #[must_use]
    pub fn lincmd(&mut self) -> LincmdW<CtrlbSpec> {
        LincmdW::new(self, 24)
    }
}
#[doc = "USART_INT Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u32 = 0;
}
