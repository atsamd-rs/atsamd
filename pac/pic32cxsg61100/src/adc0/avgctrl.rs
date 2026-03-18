#[doc = "Register `AVGCTRL` reader"]
pub type R = crate::R<AvgctrlSpec>;
#[doc = "Register `AVGCTRL` writer"]
pub type W = crate::W<AvgctrlSpec>;
#[doc = "Number of Samples to be Collected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Samplenumselect {
    #[doc = "0: 1 sample"]
    _1 = 0,
    #[doc = "1: 2 samples"]
    _2 = 1,
    #[doc = "2: 4 samples"]
    _4 = 2,
    #[doc = "3: 8 samples"]
    _8 = 3,
    #[doc = "4: 16 samples"]
    _16 = 4,
    #[doc = "5: 32 samples"]
    _32 = 5,
    #[doc = "6: 64 samples"]
    _64 = 6,
    #[doc = "7: 128 samples"]
    _128 = 7,
    #[doc = "8: 256 samples"]
    _256 = 8,
    #[doc = "9: 512 samples"]
    _512 = 9,
    #[doc = "10: 1024 samples"]
    _1024 = 10,
}
impl From<Samplenumselect> for u8 {
    #[inline(always)]
    fn from(variant: Samplenumselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Samplenumselect {
    type Ux = u8;
}
impl crate::IsEnum for Samplenumselect {}
#[doc = "Field `SAMPLENUM` reader - Number of Samples to be Collected"]
pub type SamplenumR = crate::FieldReader<Samplenumselect>;
impl SamplenumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Samplenumselect> {
        match self.bits {
            0 => Some(Samplenumselect::_1),
            1 => Some(Samplenumselect::_2),
            2 => Some(Samplenumselect::_4),
            3 => Some(Samplenumselect::_8),
            4 => Some(Samplenumselect::_16),
            5 => Some(Samplenumselect::_32),
            6 => Some(Samplenumselect::_64),
            7 => Some(Samplenumselect::_128),
            8 => Some(Samplenumselect::_256),
            9 => Some(Samplenumselect::_512),
            10 => Some(Samplenumselect::_1024),
            _ => None,
        }
    }
    #[doc = "1 sample"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Samplenumselect::_1
    }
    #[doc = "2 samples"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Samplenumselect::_2
    }
    #[doc = "4 samples"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Samplenumselect::_4
    }
    #[doc = "8 samples"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Samplenumselect::_8
    }
    #[doc = "16 samples"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Samplenumselect::_16
    }
    #[doc = "32 samples"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Samplenumselect::_32
    }
    #[doc = "64 samples"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Samplenumselect::_64
    }
    #[doc = "128 samples"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Samplenumselect::_128
    }
    #[doc = "256 samples"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Samplenumselect::_256
    }
    #[doc = "512 samples"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Samplenumselect::_512
    }
    #[doc = "1024 samples"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Samplenumselect::_1024
    }
}
#[doc = "Field `SAMPLENUM` writer - Number of Samples to be Collected"]
pub type SamplenumW<'a, REG> = crate::FieldWriter<'a, REG, 4, Samplenumselect>;
impl<'a, REG> SamplenumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 sample"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_1)
    }
    #[doc = "2 samples"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_2)
    }
    #[doc = "4 samples"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_4)
    }
    #[doc = "8 samples"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_8)
    }
    #[doc = "16 samples"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_16)
    }
    #[doc = "32 samples"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_32)
    }
    #[doc = "64 samples"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_64)
    }
    #[doc = "128 samples"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_128)
    }
    #[doc = "256 samples"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_256)
    }
    #[doc = "512 samples"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_512)
    }
    #[doc = "1024 samples"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Samplenumselect::_1024)
    }
}
#[doc = "Field `ADJRES` reader - Adjusting Result / Division Coefficient"]
pub type AdjresR = crate::FieldReader;
#[doc = "Field `ADJRES` writer - Adjusting Result / Division Coefficient"]
pub type AdjresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline(always)]
    pub fn samplenum(&self) -> SamplenumR {
        SamplenumR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline(always)]
    pub fn adjres(&self) -> AdjresR {
        AdjresR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of Samples to be Collected"]
    #[inline(always)]
    #[must_use]
    pub fn samplenum(&mut self) -> SamplenumW<AvgctrlSpec> {
        SamplenumW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn adjres(&mut self) -> AdjresW<AvgctrlSpec> {
        AdjresW::new(self, 4)
    }
}
#[doc = "Average Control\n\nYou can [`read`](crate::Reg::read) this register and get [`avgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvgctrlSpec;
impl crate::RegisterSpec for AvgctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`avgctrl::R`](R) reader structure"]
impl crate::Readable for AvgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`avgctrl::W`](W) writer structure"]
impl crate::Writable for AvgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AVGCTRL to value 0"]
impl crate::Resettable for AvgctrlSpec {
    const RESET_VALUE: u8 = 0;
}
