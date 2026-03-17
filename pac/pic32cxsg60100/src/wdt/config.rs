#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Perselect {
    #[doc = "0: 8 clock cycles"]
    Cyc8 = 0,
    #[doc = "1: 16 clock cycles"]
    Cyc16 = 1,
    #[doc = "2: 32 clock cycles"]
    Cyc32 = 2,
    #[doc = "3: 64 clock cycles"]
    Cyc64 = 3,
    #[doc = "4: 128 clock cycles"]
    Cyc128 = 4,
    #[doc = "5: 256 clock cycles"]
    Cyc256 = 5,
    #[doc = "6: 512 clock cycles"]
    Cyc512 = 6,
    #[doc = "7: 1024 clock cycles"]
    Cyc1024 = 7,
    #[doc = "8: 2048 clock cycles"]
    Cyc2048 = 8,
    #[doc = "9: 4096 clock cycles"]
    Cyc4096 = 9,
    #[doc = "10: 8192 clock cycles"]
    Cyc8192 = 10,
    #[doc = "11: 16384 clock cycles"]
    Cyc16384 = 11,
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
            0 => Some(Perselect::Cyc8),
            1 => Some(Perselect::Cyc16),
            2 => Some(Perselect::Cyc32),
            3 => Some(Perselect::Cyc64),
            4 => Some(Perselect::Cyc128),
            5 => Some(Perselect::Cyc256),
            6 => Some(Perselect::Cyc512),
            7 => Some(Perselect::Cyc1024),
            8 => Some(Perselect::Cyc2048),
            9 => Some(Perselect::Cyc4096),
            10 => Some(Perselect::Cyc8192),
            11 => Some(Perselect::Cyc16384),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == Perselect::Cyc8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == Perselect::Cyc16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == Perselect::Cyc32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == Perselect::Cyc64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == Perselect::Cyc128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == Perselect::Cyc256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == Perselect::Cyc512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == Perselect::Cyc1024
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == Perselect::Cyc2048
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == Perselect::Cyc4096
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == Perselect::Cyc8192
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == Perselect::Cyc16384
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
    pub fn cyc8(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut crate::W<REG> {
        self.variant(Perselect::Cyc16384)
    }
}
#[doc = "Window Mode Time-Out Period\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Windowselect {
    #[doc = "0: 8 clock cycles"]
    Cyc8 = 0,
    #[doc = "1: 16 clock cycles"]
    Cyc16 = 1,
    #[doc = "2: 32 clock cycles"]
    Cyc32 = 2,
    #[doc = "3: 64 clock cycles"]
    Cyc64 = 3,
    #[doc = "4: 128 clock cycles"]
    Cyc128 = 4,
    #[doc = "5: 256 clock cycles"]
    Cyc256 = 5,
    #[doc = "6: 512 clock cycles"]
    Cyc512 = 6,
    #[doc = "7: 1024 clock cycles"]
    Cyc1024 = 7,
    #[doc = "8: 2048 clock cycles"]
    Cyc2048 = 8,
    #[doc = "9: 4096 clock cycles"]
    Cyc4096 = 9,
    #[doc = "10: 8192 clock cycles"]
    Cyc8192 = 10,
    #[doc = "11: 16384 clock cycles"]
    Cyc16384 = 11,
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
            0 => Some(Windowselect::Cyc8),
            1 => Some(Windowselect::Cyc16),
            2 => Some(Windowselect::Cyc32),
            3 => Some(Windowselect::Cyc64),
            4 => Some(Windowselect::Cyc128),
            5 => Some(Windowselect::Cyc256),
            6 => Some(Windowselect::Cyc512),
            7 => Some(Windowselect::Cyc1024),
            8 => Some(Windowselect::Cyc2048),
            9 => Some(Windowselect::Cyc4096),
            10 => Some(Windowselect::Cyc8192),
            11 => Some(Windowselect::Cyc16384),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == Windowselect::Cyc8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == Windowselect::Cyc16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == Windowselect::Cyc32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == Windowselect::Cyc64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == Windowselect::Cyc128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == Windowselect::Cyc256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == Windowselect::Cyc512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == Windowselect::Cyc1024
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == Windowselect::Cyc2048
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == Windowselect::Cyc4096
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == Windowselect::Cyc8192
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == Windowselect::Cyc16384
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
    pub fn cyc8(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut crate::W<REG> {
        self.variant(Windowselect::Cyc16384)
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
