#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Perselect {
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
impl From<Perselect> for u8 {
    #[inline(always)]
    fn from(variant: Perselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Perselect {
    type Ux = u8;
}
impl crate::IsEnum for Perselect {}
#[doc = "Field `PER` reader - Time-Out Period"]
pub type PerR = crate::FieldReader<Perselect>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Perselect> {
        match self.bits {
            0 => Some(Perselect::_8),
            1 => Some(Perselect::_16),
            2 => Some(Perselect::_32),
            3 => Some(Perselect::_64),
            4 => Some(Perselect::_128),
            5 => Some(Perselect::_256),
            6 => Some(Perselect::_512),
            7 => Some(Perselect::_1k),
            8 => Some(Perselect::_2k),
            9 => Some(Perselect::_4k),
            10 => Some(Perselect::_8k),
            11 => Some(Perselect::_16k),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Perselect::_8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Perselect::_16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Perselect::_32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Perselect::_64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Perselect::_128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Perselect::_256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Perselect::_512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == Perselect::_1k
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == Perselect::_2k
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Perselect::_4k
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Perselect::_8k
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Perselect::_16k
    }
}
#[doc = "Field `PER` writer - Time-Out Period"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 4, Perselect>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_1k)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_2k)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_4k)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_8k)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::_16k)
    }
}
#[doc = "Window Mode Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Windowselect {
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
impl From<Windowselect> for u8 {
    #[inline(always)]
    fn from(variant: Windowselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Windowselect {
    type Ux = u8;
}
impl crate::IsEnum for Windowselect {}
#[doc = "Field `WINDOW` reader - Window Mode Time-Out Period"]
pub type WindowR = crate::FieldReader<Windowselect>;
impl WindowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Windowselect> {
        match self.bits {
            0 => Some(Windowselect::_8),
            1 => Some(Windowselect::_16),
            2 => Some(Windowselect::_32),
            3 => Some(Windowselect::_64),
            4 => Some(Windowselect::_128),
            5 => Some(Windowselect::_256),
            6 => Some(Windowselect::_512),
            7 => Some(Windowselect::_1k),
            8 => Some(Windowselect::_2k),
            9 => Some(Windowselect::_4k),
            10 => Some(Windowselect::_8k),
            11 => Some(Windowselect::_16k),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Windowselect::_8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Windowselect::_16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Windowselect::_32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Windowselect::_64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Windowselect::_128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Windowselect::_256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Windowselect::_512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == Windowselect::_1k
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == Windowselect::_2k
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Windowselect::_4k
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Windowselect::_8k
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Windowselect::_16k
    }
}
#[doc = "Field `WINDOW` writer - Window Mode Time-Out Period"]
pub type WindowW<'a, REG> = crate::FieldWriter<'a, REG, 4, Windowselect>;
impl<'a, REG> WindowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_1k)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_2k)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_4k)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn _8k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_8k)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn _16k(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::_16k)
    }
}
impl R {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    pub fn window(&self) -> WindowR {
        WindowR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-Out Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<ConfigSpec> {
        PerW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WindowW<ConfigSpec> {
        WindowW::new(self, 4)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0xbb"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u8 = 0xbb;
}
