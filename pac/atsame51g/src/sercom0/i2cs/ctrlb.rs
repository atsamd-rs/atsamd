#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SmenR = crate::BitReader;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCMD` reader - PMBus Group Command"]
pub type GcmdR = crate::BitReader;
#[doc = "Field `GCMD` writer - PMBus Group Command"]
pub type GcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AACKEN` reader - Automatic Address Acknowledge"]
pub type AackenR = crate::BitReader;
#[doc = "Field `AACKEN` writer - Automatic Address Acknowledge"]
pub type AackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amodeselect {
    #[doc = "0: I2C Address mask"]
    Mask = 0,
    #[doc = "1: Two unique Addressess"]
    _2Addresses = 1,
    #[doc = "2: Address Range"]
    Range = 2,
}
impl From<Amodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Amodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Amodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Amodeselect {}
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AmodeR = crate::FieldReader<Amodeselect>;
impl AmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Amodeselect> {
        match self.bits {
            0 => Some(Amodeselect::Mask),
            1 => Some(Amodeselect::_2Addresses),
            2 => Some(Amodeselect::Range),
            _ => None,
        }
    }
    #[doc = "I2C Address mask"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Amodeselect::Mask
    }
    #[doc = "Two unique Addressess"]
    #[inline(always)]
    pub fn is_2_addresses(&self) -> bool {
        *self == Amodeselect::_2Addresses
    }
    #[doc = "Address Range"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == Amodeselect::Range
    }
}
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Amodeselect>;
impl<'a, REG> AmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2C Address mask"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Amodeselect::Mask)
    }
    #[doc = "Two unique Addressess"]
    #[inline(always)]
    pub fn _2_addresses(self) -> &'a mut crate::W<REG> {
        self.variant(Amodeselect::_2Addresses)
    }
    #[doc = "Address Range"]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(Amodeselect::Range)
    }
}
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACKACT` reader - Acknowledge Action"]
pub type AckactR = crate::BitReader;
#[doc = "Field `ACKACT` writer - Acknowledge Action"]
pub type AckactW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SmenR {
        SmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&self) -> GcmdR {
        GcmdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&self) -> AackenR {
        AackenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AmodeR {
        AmodeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> AckactR {
        AckactR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SmenW<CtrlbSpec> {
        SmenW::new(self, 8)
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    #[must_use]
    pub fn gcmd(&mut self) -> GcmdW<CtrlbSpec> {
        GcmdW::new(self, 9)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn aacken(&mut self) -> AackenW<CtrlbSpec> {
        AackenW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AmodeW<CtrlbSpec> {
        AmodeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CtrlbSpec> {
        CmdW::new(self, 16)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn ackact(&mut self) -> AckactW<CtrlbSpec> {
        AckactW::new(self, 18)
    }
}
#[doc = "I2CS Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
