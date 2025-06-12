#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `START` reader - Start Encryption/Decryption"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start Encryption/Decryption"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSG` reader - New message"]
pub type NewmsgR = crate::BitReader;
#[doc = "Field `NEWMSG` writer - New message"]
pub type NewmsgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOM` reader - End of message"]
pub type EomR = crate::BitReader;
#[doc = "Field `EOM` writer - End of message"]
pub type EomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFMUL` reader - GF Multiplication"]
pub type GfmulR = crate::BitReader;
#[doc = "Field `GFMUL` writer - GF Multiplication"]
pub type GfmulW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&self) -> NewmsgR {
        NewmsgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&self) -> EomR {
        EomR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&self) -> GfmulR {
        GfmulR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CtrlbSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&mut self) -> NewmsgW<CtrlbSpec> {
        NewmsgW::new(self, 1)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&mut self) -> EomW<CtrlbSpec> {
        EomW::new(self, 2)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&mut self) -> GfmulW<CtrlbSpec> {
        GfmulW::new(self, 3)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
