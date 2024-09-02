#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type ChsizeR = crate::FieldReader;
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type ChsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLOADEN` reader - Data Preload Enable"]
pub type PloadenR = crate::BitReader;
#[doc = "Field `PLOADEN` writer - Data Preload Enable"]
pub type PloadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSDE` reader - Slave Select Low Detect Enable"]
pub type SsdeR = crate::BitReader;
#[doc = "Field `SSDE` writer - Slave Select Low Detect Enable"]
pub type SsdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSSEN` reader - Master Slave Select Enable"]
pub type MssenR = crate::BitReader;
#[doc = "Field `MSSEN` writer - Master Slave Select Enable"]
pub type MssenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AmodeR = crate::FieldReader;
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&self) -> PloadenR {
        PloadenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SsdeR {
        SsdeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&self) -> MssenR {
        MssenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AmodeR {
        AmodeR::new(((self.bits >> 14) & 3) as u8)
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
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ploaden(&mut self) -> PloadenW<CtrlbSpec> {
        PloadenW::new(self, 6)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssde(&mut self) -> SsdeW<CtrlbSpec> {
        SsdeW::new(self, 9)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mssen(&mut self) -> MssenW<CtrlbSpec> {
        MssenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AmodeW<CtrlbSpec> {
        AmodeW::new(self, 14)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<CtrlbSpec> {
        RxenW::new(self, 17)
    }
}
#[doc = "SPI Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
