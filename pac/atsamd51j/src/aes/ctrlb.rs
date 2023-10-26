#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `START` reader - Start Encryption/Decryption"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start Encryption/Decryption"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NEWMSG` reader - New message"]
pub type NEWMSG_R = crate::BitReader;
#[doc = "Field `NEWMSG` writer - New message"]
pub type NEWMSG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOM` reader - End of message"]
pub type EOM_R = crate::BitReader;
#[doc = "Field `EOM` writer - End of message"]
pub type EOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GFMUL` reader - GF Multiplication"]
pub type GFMUL_R = crate::BitReader;
#[doc = "Field `GFMUL` writer - GF Multiplication"]
pub type GFMUL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&self) -> NEWMSG_R {
        NEWMSG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&self) -> EOM_R {
        EOM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&self) -> GFMUL_R {
        GFMUL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTRLB_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    #[must_use]
    pub fn newmsg(&mut self) -> NEWMSG_W<CTRLB_SPEC, 1> {
        NEWMSG_W::new(self)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn eom(&mut self) -> EOM_W<CTRLB_SPEC, 2> {
        EOM_W::new(self)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    #[must_use]
    pub fn gfmul(&mut self) -> GFMUL_W<CTRLB_SPEC, 3> {
        GFMUL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
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
