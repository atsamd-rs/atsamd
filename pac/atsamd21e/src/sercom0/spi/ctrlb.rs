#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type CHSIZE_R = crate::FieldReader;
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type CHSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PLOADEN` reader - Data Preload Enable"]
pub type PLOADEN_R = crate::BitReader;
#[doc = "Field `PLOADEN` writer - Data Preload Enable"]
pub type PLOADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSDE` reader - Slave Select Low Detect Enable"]
pub type SSDE_R = crate::BitReader;
#[doc = "Field `SSDE` writer - Slave Select Low Detect Enable"]
pub type SSDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSSEN` reader - Master Slave Select Enable"]
pub type MSSEN_R = crate::BitReader;
#[doc = "Field `MSSEN` writer - Master Slave Select Enable"]
pub type MSSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AMODE_R = crate::FieldReader;
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&self) -> PLOADEN_R {
        PLOADEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&self) -> MSSEN_R {
        MSSEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn chsize(&mut self) -> CHSIZE_W<CTRLB_SPEC, 0> {
        CHSIZE_W::new(self)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ploaden(&mut self) -> PLOADEN_W<CTRLB_SPEC, 6> {
        PLOADEN_W::new(self)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssde(&mut self) -> SSDE_W<CTRLB_SPEC, 9> {
        SSDE_W::new(self)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mssen(&mut self) -> MSSEN_W<CTRLB_SPEC, 13> {
        MSSEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AMODE_W<CTRLB_SPEC, 14> {
        AMODE_W::new(self)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CTRLB_SPEC, 17> {
        RXEN_W::new(self)
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
#[doc = "SPI Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
