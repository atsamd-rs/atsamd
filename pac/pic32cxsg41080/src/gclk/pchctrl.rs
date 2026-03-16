#[doc = "Register `PCHCTRL[%s]` reader"]
pub type R = crate::R<PchctrlSpec>;
#[doc = "Register `PCHCTRL[%s]` writer"]
pub type W = crate::W<PchctrlSpec>;
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Genselect {
    #[doc = "0: Generic clock generator 0"]
    Gclk0 = 0,
    #[doc = "1: Generic clock generator 1"]
    Gclk1 = 1,
    #[doc = "2: Generic clock generator 2"]
    Gclk2 = 2,
    #[doc = "3: Generic clock generator 3"]
    Gclk3 = 3,
    #[doc = "4: Generic clock generator 4"]
    Gclk4 = 4,
    #[doc = "5: Generic clock generator 5"]
    Gclk5 = 5,
    #[doc = "6: Generic clock generator 6"]
    Gclk6 = 6,
    #[doc = "7: Generic clock generator 7"]
    Gclk7 = 7,
    #[doc = "8: Generic clock generator 8"]
    Gclk8 = 8,
    #[doc = "9: Generic clock generator 9"]
    Gclk9 = 9,
    #[doc = "10: Generic clock generator 10"]
    Gclk10 = 10,
    #[doc = "11: Generic clock generator 11"]
    Gclk11 = 11,
}
impl From<Genselect> for u8 {
    #[inline(always)]
    fn from(variant: Genselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Genselect {
    type Ux = u8;
}
impl crate::IsEnum for Genselect {}
#[doc = "Field `GEN` reader - Generic Clock Generator"]
pub type GenR = crate::FieldReader<Genselect>;
impl GenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Genselect> {
        match self.bits {
            0 => Some(Genselect::Gclk0),
            1 => Some(Genselect::Gclk1),
            2 => Some(Genselect::Gclk2),
            3 => Some(Genselect::Gclk3),
            4 => Some(Genselect::Gclk4),
            5 => Some(Genselect::Gclk5),
            6 => Some(Genselect::Gclk6),
            7 => Some(Genselect::Gclk7),
            8 => Some(Genselect::Gclk8),
            9 => Some(Genselect::Gclk9),
            10 => Some(Genselect::Gclk10),
            11 => Some(Genselect::Gclk11),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == Genselect::Gclk0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == Genselect::Gclk1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == Genselect::Gclk2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == Genselect::Gclk3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == Genselect::Gclk4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == Genselect::Gclk5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == Genselect::Gclk6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == Genselect::Gclk7
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == Genselect::Gclk8
    }
    #[doc = "Generic clock generator 9"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == Genselect::Gclk9
    }
    #[doc = "Generic clock generator 10"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == Genselect::Gclk10
    }
    #[doc = "Generic clock generator 11"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == Genselect::Gclk11
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub type GenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Genselect>;
impl<'a, REG> GenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk8)
    }
    #[doc = "Generic clock generator 9"]
    #[inline(always)]
    pub fn gclk9(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk9)
    }
    #[doc = "Generic clock generator 10"]
    #[inline(always)]
    pub fn gclk10(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk10)
    }
    #[doc = "Generic clock generator 11"]
    #[inline(always)]
    pub fn gclk11(self) -> &'a mut crate::W<REG> {
        self.variant(Genselect::Gclk11)
    }
}
#[doc = "Field `CHEN` reader - Channel Enable"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel Enable"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GenR {
        GenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GenW<PchctrlSpec> {
        GenW::new(self, 0)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> ChenW<PchctrlSpec> {
        ChenW::new(self, 6)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WrtlockW<PchctrlSpec> {
        WrtlockW::new(self, 7)
    }
}
#[doc = "Peripheral Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pchctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pchctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PchctrlSpec;
impl crate::RegisterSpec for PchctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pchctrl::R`](R) reader structure"]
impl crate::Readable for PchctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pchctrl::W`](W) writer structure"]
impl crate::Writable for PchctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCHCTRL[%s]
to value 0"]
impl crate::Resettable for PchctrlSpec {
    const RESET_VALUE: u32 = 0;
}
