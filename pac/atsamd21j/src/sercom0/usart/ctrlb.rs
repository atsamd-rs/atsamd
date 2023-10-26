#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type CHSIZE_R = crate::FieldReader;
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type CHSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub type SBMODE_R = crate::BitReader;
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub type SBMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
pub type PMODE_R = crate::BitReader;
#[doc = "Field `PMODE` writer - Parity Mode"]
pub type PMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TXEN_R = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
#[doc = "USART Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
