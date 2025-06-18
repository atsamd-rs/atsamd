#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Speed Configuration for Host\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spdconfselect {
    #[doc = "0: Normal mode:the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    Normal = 0,
    #[doc = "3: Full-speed:the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    Fs = 3,
}
impl From<Spdconfselect> for u8 {
    #[inline(always)]
    fn from(variant: Spdconfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spdconfselect {
    type Ux = u8;
}
impl crate::IsEnum for Spdconfselect {}
#[doc = "Field `SPDCONF` reader - Speed Configuration for Host"]
pub type SpdconfR = crate::FieldReader<Spdconfselect>;
impl SpdconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Spdconfselect> {
        match self.bits {
            0 => Some(Spdconfselect::Normal),
            3 => Some(Spdconfselect::Fs),
            _ => None,
        }
    }
    #[doc = "Normal mode:the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Spdconfselect::Normal
    }
    #[doc = "Full-speed:the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == Spdconfselect::Fs
    }
}
#[doc = "Field `SPDCONF` writer - Speed Configuration for Host"]
pub type SpdconfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spdconfselect>;
impl<'a, REG> SpdconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode:the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::Normal)
    }
    #[doc = "Full-speed:the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::Fs)
    }
}
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TstjR = crate::BitReader;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TstjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TstkR = crate::BitReader;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TstkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub type SofeR = crate::BitReader;
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub type SofeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSRESET` reader - Send USB Reset"]
pub type BusresetR = crate::BitReader;
#[doc = "Field `BUSRESET` writer - Send USB Reset"]
pub type BusresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSOK` reader - VBUS is OK"]
pub type VbusokR = crate::BitReader;
#[doc = "Field `VBUSOK` writer - VBUS is OK"]
pub type VbusokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1RESUME` reader - Send L1 Resume"]
pub type L1resumeR = crate::BitReader;
#[doc = "Field `L1RESUME` writer - Send L1 Resume"]
pub type L1resumeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    pub fn spdconf(&self) -> SpdconfR {
        SpdconfR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TstjR {
        TstjR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TstkR {
        TstkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SofeR {
        SofeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn busreset(&self) -> BusresetR {
        BusresetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    pub fn vbusok(&self) -> VbusokR {
        VbusokR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1resumeR {
        L1resumeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<CtrlbSpec> {
        ResumeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SpdconfW<CtrlbSpec> {
        SpdconfW::new(self, 2)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&mut self) -> TstjW<CtrlbSpec> {
        TstjW::new(self, 5)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&mut self) -> TstkW<CtrlbSpec> {
        TstkW::new(self, 6)
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&mut self) -> SofeW<CtrlbSpec> {
        SofeW::new(self, 8)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn busreset(&mut self) -> BusresetW<CtrlbSpec> {
        BusresetW::new(self, 9)
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    pub fn vbusok(&mut self) -> VbusokW<CtrlbSpec> {
        VbusokW::new(self, 10)
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1resumeW<CtrlbSpec> {
        L1resumeW::new(self, 11)
    }
}
#[doc = "HOST Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
