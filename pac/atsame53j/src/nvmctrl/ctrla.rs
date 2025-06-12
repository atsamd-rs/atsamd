#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `AUTOWS` reader - Auto Wait State Enable"]
pub type AutowsR = crate::BitReader;
#[doc = "Field `AUTOWS` writer - Auto Wait State Enable"]
pub type AutowsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEN` reader - Suspend Enable"]
pub type SuspenR = crate::BitReader;
#[doc = "Field `SUSPEN` writer - Suspend Enable"]
pub type SuspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wmodeselect {
    #[doc = "0: Manual Write"]
    Man = 0,
    #[doc = "1: Automatic Double Word Write"]
    Adw = 1,
    #[doc = "2: Automatic Quad Word"]
    Aqw = 2,
    #[doc = "3: Automatic Page Write"]
    Ap = 3,
}
impl From<Wmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Wmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Wmodeselect {}
#[doc = "Field `WMODE` reader - Write Mode"]
pub type WmodeR = crate::FieldReader<Wmodeselect>;
impl WmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wmodeselect {
        match self.bits {
            0 => Wmodeselect::Man,
            1 => Wmodeselect::Adw,
            2 => Wmodeselect::Aqw,
            3 => Wmodeselect::Ap,
            _ => unreachable!(),
        }
    }
    #[doc = "Manual Write"]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        *self == Wmodeselect::Man
    }
    #[doc = "Automatic Double Word Write"]
    #[inline(always)]
    pub fn is_adw(&self) -> bool {
        *self == Wmodeselect::Adw
    }
    #[doc = "Automatic Quad Word"]
    #[inline(always)]
    pub fn is_aqw(&self) -> bool {
        *self == Wmodeselect::Aqw
    }
    #[doc = "Automatic Page Write"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        *self == Wmodeselect::Ap
    }
}
#[doc = "Field `WMODE` writer - Write Mode"]
pub type WmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wmodeselect, crate::Safe>;
impl<'a, REG> WmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual Write"]
    #[inline(always)]
    pub fn man(self) -> &'a mut crate::W<REG> {
        self.variant(Wmodeselect::Man)
    }
    #[doc = "Automatic Double Word Write"]
    #[inline(always)]
    pub fn adw(self) -> &'a mut crate::W<REG> {
        self.variant(Wmodeselect::Adw)
    }
    #[doc = "Automatic Quad Word"]
    #[inline(always)]
    pub fn aqw(self) -> &'a mut crate::W<REG> {
        self.variant(Wmodeselect::Aqw)
    }
    #[doc = "Automatic Page Write"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut crate::W<REG> {
        self.variant(Wmodeselect::Ap)
    }
}
#[doc = "Power Reduction Mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmselect {
    #[doc = "0: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    Semiauto = 0,
    #[doc = "1: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    Fullauto = 1,
    #[doc = "3: NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    Manual = 3,
}
impl From<Prmselect> for u8 {
    #[inline(always)]
    fn from(variant: Prmselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmselect {
    type Ux = u8;
}
impl crate::IsEnum for Prmselect {}
#[doc = "Field `PRM` reader - Power Reduction Mode during Sleep"]
pub type PrmR = crate::FieldReader<Prmselect>;
impl PrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prmselect> {
        match self.bits {
            0 => Some(Prmselect::Semiauto),
            1 => Some(Prmselect::Fullauto),
            3 => Some(Prmselect::Manual),
            _ => None,
        }
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn is_semiauto(&self) -> bool {
        *self == Prmselect::Semiauto
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    #[inline(always)]
    pub fn is_fullauto(&self) -> bool {
        *self == Prmselect::Fullauto
    }
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Prmselect::Manual
    }
}
#[doc = "Field `PRM` writer - Power Reduction Mode during Sleep"]
pub type PrmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmselect>;
impl<'a, REG> PrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn semiauto(self) -> &'a mut crate::W<REG> {
        self.variant(Prmselect::Semiauto)
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    #[inline(always)]
    pub fn fullauto(self) -> &'a mut crate::W<REG> {
        self.variant(Prmselect::Fullauto)
    }
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Prmselect::Manual)
    }
}
#[doc = "Field `RWS` reader - NVM Read Wait States"]
pub type RwsR = crate::FieldReader;
#[doc = "Field `RWS` writer - NVM Read Wait States"]
pub type RwsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AHBNS0` reader - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type Ahbns0R = crate::BitReader;
#[doc = "Field `AHBNS0` writer - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type Ahbns0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBNS1` reader - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type Ahbns1R = crate::BitReader;
#[doc = "Field `AHBNS1` writer - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub type Ahbns1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHEDIS0` reader - AHB0 Cache Disable"]
pub type Cachedis0R = crate::BitReader;
#[doc = "Field `CACHEDIS0` writer - AHB0 Cache Disable"]
pub type Cachedis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHEDIS1` reader - AHB1 Cache Disable"]
pub type Cachedis1R = crate::BitReader;
#[doc = "Field `CACHEDIS1` writer - AHB1 Cache Disable"]
pub type Cachedis1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    pub fn autows(&self) -> AutowsR {
        AutowsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    pub fn suspen(&self) -> SuspenR {
        SuspenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WmodeR {
        WmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn prm(&self) -> PrmR {
        PrmR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RwsR {
        RwsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns0(&self) -> Ahbns0R {
        Ahbns0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns1(&self) -> Ahbns1R {
        Ahbns1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    pub fn cachedis0(&self) -> Cachedis0R {
        Cachedis0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    pub fn cachedis1(&self) -> Cachedis1R {
        Cachedis1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    pub fn autows(&mut self) -> AutowsW<CtrlaSpec> {
        AutowsW::new(self, 2)
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    pub fn suspen(&mut self) -> SuspenW<CtrlaSpec> {
        SuspenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&mut self) -> WmodeW<CtrlaSpec> {
        WmodeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn prm(&mut self) -> PrmW<CtrlaSpec> {
        PrmW::new(self, 6)
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&mut self) -> RwsW<CtrlaSpec> {
        RwsW::new(self, 8)
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns0(&mut self) -> Ahbns0W<CtrlaSpec> {
        Ahbns0W::new(self, 12)
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns1(&mut self) -> Ahbns1W<CtrlaSpec> {
        Ahbns1W::new(self, 13)
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    pub fn cachedis0(&mut self) -> Cachedis0W<CtrlaSpec> {
        Cachedis0W::new(self, 14)
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    pub fn cachedis1(&mut self) -> Cachedis1W<CtrlaSpec> {
        Cachedis1W::new(self, 15)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0x04"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u16 = 0x04;
}
