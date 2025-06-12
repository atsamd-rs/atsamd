#[doc = "Register `EWCTRL` reader"]
pub type R = crate::R<EwctrlSpec>;
#[doc = "Register `EWCTRL` writer"]
pub type W = crate::W<EwctrlSpec>;
#[doc = "Early Warning Interrupt Time Offset\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ewoffsetselect {
    #[doc = "0: 8 clock cycles"]
    _8 = 0,
    #[doc = "1: 16 clock cycles"]
    _16 = 1,
    #[doc = "2: 32 clock cycles"]
    _32 = 2,
    #[doc = "3: 64 clock cycles"]
    _64 = 3,
    #[doc = "4: 128 clock cycles"]
    _128 = 4,
    #[doc = "5: 256 clock cycles"]
    _256 = 5,
    #[doc = "6: 512 clock cycles"]
    _512 = 6,
    #[doc = "7: 1024 clock cycles"]
    _1k = 7,
    #[doc = "8: 2048 clock cycles"]
    _2k = 8,
    #[doc = "9: 4096 clock cycles"]
    _4k = 9,
    #[doc = "10: 8192 clock cycles"]
    _8k = 10,
    #[doc = "11: 16384 clock cycles"]
    _16k = 11,
}
impl From<Ewoffsetselect> for u8 {
    #[inline(always)]
    fn from(variant: Ewoffsetselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ewoffsetselect {
    type Ux = u8;
}
impl crate::IsEnum for Ewoffsetselect {}
#[doc = "Field `EWOFFSET` reader - Early Warning Interrupt Time Offset"]
pub type EwoffsetR = crate::FieldReader<Ewoffsetselect>;
impl EwoffsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ewoffsetselect> {
        match self.bits {
            0 => Some(Ewoffsetselect::_8),
            1 => Some(Ewoffsetselect::_16),
            2 => Some(Ewoffsetselect::_32),
            3 => Some(Ewoffsetselect::_64),
            4 => Some(Ewoffsetselect::_128),
            5 => Some(Ewoffsetselect::_256),
            6 => Some(Ewoffsetselect::_512),
            7 => Some(Ewoffsetselect::_1k),
            8 => Some(Ewoffsetselect::_2k),
            9 => Some(Ewoffsetselect::_4k),
            10 => Some(Ewoffsetselect::_8k),
            11 => Some(Ewoffsetselect::_16k),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Ewoffsetselect::_8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Ewoffsetselect::_16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Ewoffsetselect::_32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Ewoffsetselect::_64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Ewoffsetselect::_128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Ewoffsetselect::_256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Ewoffsetselect::_512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == Ewoffsetselect::_1k
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == Ewoffsetselect::_2k
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Ewoffsetselect::_4k
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Ewoffsetselect::_8k
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Ewoffsetselect::_16k
    }
}
#[doc = "Field `EWOFFSET` writer - Early Warning Interrupt Time Offset"]
pub type EwoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ewoffsetselect>;
impl<'a, REG> EwoffsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_1k)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_2k)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_4k)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_8k)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::_16k)
    }
}
impl R {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    pub fn ewoffset(&self) -> EwoffsetR {
        EwoffsetR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
    #[inline(always)]
    pub fn ewoffset(&mut self) -> EwoffsetW<EwctrlSpec> {
        EwoffsetW::new(self, 0)
    }
}
#[doc = "Early Warning Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ewctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwctrlSpec;
impl crate::RegisterSpec for EwctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ewctrl::R`](R) reader structure"]
impl crate::Readable for EwctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ewctrl::W`](W) writer structure"]
impl crate::Writable for EwctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EWCTRL to value 0x0b"]
impl crate::Resettable for EwctrlSpec {
    const RESET_VALUE: u8 = 0x0b;
}
