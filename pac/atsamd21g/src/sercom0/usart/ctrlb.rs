#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type ChsizeR = crate::FieldReader;
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type ChsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub type SbmodeR = crate::BitReader;
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub type SbmodeW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PMODE` reader - Parity Mode"]
pub type PmodeR = crate::BitReader;
#[doc = "Field `PMODE` writer - Parity Mode"]
pub type PmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
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
}
#[doc = "USART Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
