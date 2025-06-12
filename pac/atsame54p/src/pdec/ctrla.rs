#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: QDEC operating mode"]
    Qdec = 0,
    #[doc = "1: HALL operating mode"]
    Hall = 1,
    #[doc = "2: COUNTER operating mode"]
    Counter = 2,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Operation Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::Qdec),
            1 => Some(Modeselect::Hall),
            2 => Some(Modeselect::Counter),
            _ => None,
        }
    }
    #[doc = "QDEC operating mode"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == Modeselect::Qdec
    }
    #[doc = "HALL operating mode"]
    #[inline(always)]
    pub fn is_hall(&self) -> bool {
        *self == Modeselect::Hall
    }
    #[doc = "COUNTER operating mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == Modeselect::Counter
    }
}
#[doc = "Field `MODE` writer - Operation Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QDEC operating mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Qdec)
    }
    #[doc = "HALL operating mode"]
    #[inline(always)]
    pub fn hall(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Hall)
    }
    #[doc = "COUNTER operating mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Counter)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PDEC Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Confselect {
    #[doc = "0: Quadrature decoder direction"]
    X4 = 0,
    #[doc = "1: Secure Quadrature decoder direction"]
    X4s = 1,
    #[doc = "2: Decoder direction"]
    X2 = 2,
    #[doc = "3: Secure decoder direction"]
    X2s = 3,
    #[doc = "4: Auto correction mode"]
    Autoc = 4,
}
impl From<Confselect> for u8 {
    #[inline(always)]
    fn from(variant: Confselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Confselect {
    type Ux = u8;
}
impl crate::IsEnum for Confselect {}
#[doc = "Field `CONF` reader - PDEC Configuration"]
pub type ConfR = crate::FieldReader<Confselect>;
impl ConfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Confselect> {
        match self.bits {
            0 => Some(Confselect::X4),
            1 => Some(Confselect::X4s),
            2 => Some(Confselect::X2),
            3 => Some(Confselect::X2s),
            4 => Some(Confselect::Autoc),
            _ => None,
        }
    }
    #[doc = "Quadrature decoder direction"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == Confselect::X4
    }
    #[doc = "Secure Quadrature decoder direction"]
    #[inline(always)]
    pub fn is_x4s(&self) -> bool {
        *self == Confselect::X4s
    }
    #[doc = "Decoder direction"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == Confselect::X2
    }
    #[doc = "Secure decoder direction"]
    #[inline(always)]
    pub fn is_x2s(&self) -> bool {
        *self == Confselect::X2s
    }
    #[doc = "Auto correction mode"]
    #[inline(always)]
    pub fn is_autoc(&self) -> bool {
        *self == Confselect::Autoc
    }
}
#[doc = "Field `CONF` writer - PDEC Configuration"]
pub type ConfW<'a, REG> = crate::FieldWriter<'a, REG, 3, Confselect>;
impl<'a, REG> ConfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(Confselect::X4)
    }
    #[doc = "Secure Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4s(self) -> &'a mut crate::W<REG> {
        self.variant(Confselect::X4s)
    }
    #[doc = "Decoder direction"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(Confselect::X2)
    }
    #[doc = "Secure decoder direction"]
    #[inline(always)]
    pub fn x2s(self) -> &'a mut crate::W<REG> {
        self.variant(Confselect::X2s)
    }
    #[doc = "Auto correction mode"]
    #[inline(always)]
    pub fn autoc(self) -> &'a mut crate::W<REG> {
        self.variant(Confselect::Autoc)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type AlockR = crate::BitReader;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type AlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - PDEC Phase A and B Swap"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - PDEC Phase A and B Swap"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREN` reader - Period Enable"]
pub type PerenR = crate::BitReader;
#[doc = "Field `PEREN` writer - Period Enable"]
pub type PerenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINEN0` reader - PDEC Input From Pin 0 Enable"]
pub type Pinen0R = crate::BitReader;
#[doc = "Field `PINEN0` writer - PDEC Input From Pin 0 Enable"]
pub type Pinen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINEN1` reader - PDEC Input From Pin 1 Enable"]
pub type Pinen1R = crate::BitReader;
#[doc = "Field `PINEN1` writer - PDEC Input From Pin 1 Enable"]
pub type Pinen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINEN2` reader - PDEC Input From Pin 2 Enable"]
pub type Pinen2R = crate::BitReader;
#[doc = "Field `PINEN2` writer - PDEC Input From Pin 2 Enable"]
pub type Pinen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINVEN0` reader - IO Pin 0 Invert Enable"]
pub type Pinven0R = crate::BitReader;
#[doc = "Field `PINVEN0` writer - IO Pin 0 Invert Enable"]
pub type Pinven0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINVEN1` reader - IO Pin 1 Invert Enable"]
pub type Pinven1R = crate::BitReader;
#[doc = "Field `PINVEN1` writer - IO Pin 1 Invert Enable"]
pub type Pinven1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINVEN2` reader - IO Pin 2 Invert Enable"]
pub type Pinven2R = crate::BitReader;
#[doc = "Field `PINVEN2` writer - IO Pin 2 Invert Enable"]
pub type Pinven2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANGULAR` reader - Angular Counter Length"]
pub type AngularR = crate::FieldReader;
#[doc = "Field `ANGULAR` writer - Angular Counter Length"]
pub type AngularW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAXCMP` reader - Maximum Consecutive Missing Pulses"]
pub type MaxcmpR = crate::FieldReader;
#[doc = "Field `MAXCMP` writer - Maximum Consecutive Missing Pulses"]
pub type MaxcmpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> ConfR {
        ConfR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> AlockR {
        AlockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    pub fn peren(&self) -> PerenR {
        PerenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    pub fn pinen0(&self) -> Pinen0R {
        Pinen0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    pub fn pinen1(&self) -> Pinen1R {
        Pinen1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    pub fn pinen2(&self) -> Pinen2R {
        Pinen2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    pub fn pinven0(&self) -> Pinven0R {
        Pinven0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    pub fn pinven1(&self) -> Pinven1R {
        Pinven1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    pub fn pinven2(&self) -> Pinven2R {
        Pinven2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    pub fn angular(&self) -> AngularR {
        AngularR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&self) -> MaxcmpR {
        MaxcmpR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<CtrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    pub fn conf(&mut self) -> ConfW<CtrlaSpec> {
        ConfW::new(self, 8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&mut self) -> AlockW<CtrlaSpec> {
        AlockW::new(self, 11)
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<CtrlaSpec> {
        SwapW::new(self, 14)
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    pub fn peren(&mut self) -> PerenW<CtrlaSpec> {
        PerenW::new(self, 15)
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    pub fn pinen0(&mut self) -> Pinen0W<CtrlaSpec> {
        Pinen0W::new(self, 16)
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    pub fn pinen1(&mut self) -> Pinen1W<CtrlaSpec> {
        Pinen1W::new(self, 17)
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    pub fn pinen2(&mut self) -> Pinen2W<CtrlaSpec> {
        Pinen2W::new(self, 18)
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    pub fn pinven0(&mut self) -> Pinven0W<CtrlaSpec> {
        Pinven0W::new(self, 20)
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    pub fn pinven1(&mut self) -> Pinven1W<CtrlaSpec> {
        Pinven1W::new(self, 21)
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    pub fn pinven2(&mut self) -> Pinven2W<CtrlaSpec> {
        Pinven2W::new(self, 22)
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    pub fn angular(&mut self) -> AngularW<CtrlaSpec> {
        AngularW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&mut self) -> MaxcmpW<CtrlaSpec> {
        MaxcmpW::new(self, 28)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
