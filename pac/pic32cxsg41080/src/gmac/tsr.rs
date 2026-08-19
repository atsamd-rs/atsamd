#[doc = "Register `TSR` reader"]
pub type R = crate::R<TsrSpec>;
#[doc = "Register `TSR` writer"]
pub type W = crate::W<TsrSpec>;
#[doc = "Field `UBR` reader - Used Bit Read"]
pub type UbrR = crate::BitReader;
#[doc = "Field `UBR` writer - Used Bit Read"]
pub type UbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COL` reader - Collision Occurred"]
pub type ColR = crate::BitReader;
#[doc = "Field `COL` writer - Collision Occurred"]
pub type ColW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLE` reader - Retry Limit Exceeded"]
pub type RleR = crate::BitReader;
#[doc = "Field `RLE` writer - Retry Limit Exceeded"]
pub type RleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGO` reader - Transmit Go"]
pub type TxgoR = crate::BitReader;
#[doc = "Field `TXGO` writer - Transmit Go"]
pub type TxgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub type TfcR = crate::BitReader;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCOMP` reader - Transmit Complete"]
pub type TxcompR = crate::BitReader;
#[doc = "Field `TXCOMP` writer - Transmit Complete"]
pub type TxcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UND` reader - Transmit Underrun"]
pub type UndR = crate::BitReader;
#[doc = "Field `UND` writer - Transmit Underrun"]
pub type UndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HrespR = crate::BitReader;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HrespW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&self) -> UbrR {
        UbrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&self) -> ColR {
        ColR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rle(&self) -> RleR {
        RleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn txgo(&self) -> TxgoR {
        TxgoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TfcR {
        TfcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn txcomp(&self) -> TxcompR {
        TxcompR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&self) -> UndR {
        UndR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HrespR {
        HrespR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn ubr(&mut self) -> UbrW<TsrSpec> {
        UbrW::new(self, 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> ColW<TsrSpec> {
        ColW::new(self, 1)
    }
    #[doc = "Bit 2 - Retry Limit Exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rle(&mut self) -> RleW<TsrSpec> {
        RleW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    #[must_use]
    pub fn txgo(&mut self) -> TxgoW<TsrSpec> {
        TxgoW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TfcW<TsrSpec> {
        TfcW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txcomp(&mut self) -> TxcompW<TsrSpec> {
        TxcompW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn und(&mut self) -> UndW<TsrSpec> {
        UndW::new(self, 6)
    }
    #[doc = "Bit 8 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HrespW<TsrSpec> {
        HrespW::new(self, 8)
    }
}
#[doc = "Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsrSpec;
impl crate::RegisterSpec for TsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsr::W`](W) writer structure"]
impl crate::Writable for TsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TsrSpec {
    const RESET_VALUE: u32 = 0;
}
