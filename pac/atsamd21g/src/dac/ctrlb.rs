#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `EOEN` reader - External Output Enable"]
pub type EoenR = crate::BitReader;
#[doc = "Field `EOEN` writer - External Output Enable"]
pub type EoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOEN` reader - Internal Output Enable"]
pub type IoenR = crate::BitReader;
#[doc = "Field `IOEN` writer - Internal Output Enable"]
pub type IoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEFTADJ` reader - Left Adjusted Data"]
pub type LeftadjR = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left Adjusted Data"]
pub type LeftadjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPD` reader - Voltage Pump Disable"]
pub type VpdR = crate::BitReader;
#[doc = "Field `VPD` writer - Voltage Pump Disable"]
pub type VpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDWP` reader - Bypass DATABUF Write Protection"]
pub type BdwpR = crate::BitReader;
#[doc = "Field `BDWP` writer - Bypass DATABUF Write Protection"]
pub type BdwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refselselect {
    #[doc = "0: Internal 1.0V reference"]
    Int1v = 0,
    #[doc = "1: AVCC"]
    Avcc = 1,
    #[doc = "2: External reference"]
    Vrefp = 2,
}
impl From<Refselselect> for u8 {
    #[inline(always)]
    fn from(variant: Refselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refselselect {
    type Ux = u8;
}
impl crate::IsEnum for Refselselect {}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type RefselR = crate::FieldReader<Refselselect>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refselselect> {
        match self.bits {
            0 => Some(Refselselect::Int1v),
            1 => Some(Refselselect::Avcc),
            2 => Some(Refselselect::Vrefp),
            _ => None,
        }
    }
    #[doc = "Internal 1.0V reference"]
    #[inline(always)]
    pub fn is_int1v(&self) -> bool {
        *self == Refselselect::Int1v
    }
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        *self == Refselselect::Avcc
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == Refselselect::Vrefp
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refselselect>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 1.0V reference"]
    #[inline(always)]
    pub fn int1v(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Int1v)
    }
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Avcc)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Vrefp)
    }
}
impl R {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&self) -> EoenR {
        EoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IoenR {
        IoenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LeftadjR {
        LeftadjR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&self) -> VpdR {
        VpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    pub fn bdwp(&self) -> BdwpR {
        BdwpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&mut self) -> EoenW<CtrlbSpec> {
        EoenW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IoenW<CtrlbSpec> {
        IoenW::new(self, 1)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LeftadjW<CtrlbSpec> {
        LeftadjW::new(self, 2)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&mut self) -> VpdW<CtrlbSpec> {
        VpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    pub fn bdwp(&mut self) -> BdwpW<CtrlbSpec> {
        BdwpW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<CtrlbSpec> {
        RefselW::new(self, 6)
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
