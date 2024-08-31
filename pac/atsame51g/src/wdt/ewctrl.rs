#[doc = "Register `EWCTRL` reader"]
pub type R = crate::R<EwctrlSpec>;
#[doc = "Register `EWCTRL` writer"]
pub type W = crate::W<EwctrlSpec>;
#[doc = "Early Warning Interrupt Time Offset\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ewoffsetselect {
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
            0 => Some(Ewoffsetselect::Cyc8),
            1 => Some(Ewoffsetselect::Cyc16),
            2 => Some(Ewoffsetselect::Cyc32),
            3 => Some(Ewoffsetselect::Cyc64),
            4 => Some(Ewoffsetselect::Cyc128),
            5 => Some(Ewoffsetselect::Cyc256),
            6 => Some(Ewoffsetselect::Cyc512),
            7 => Some(Ewoffsetselect::Cyc1024),
            8 => Some(Ewoffsetselect::Cyc2048),
            9 => Some(Ewoffsetselect::Cyc4096),
            10 => Some(Ewoffsetselect::Cyc8192),
            11 => Some(Ewoffsetselect::Cyc16384),
            _ => None,
        }
    }
    #[doc = "8 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == Ewoffsetselect::Cyc8
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == Ewoffsetselect::Cyc16
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == Ewoffsetselect::Cyc32
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == Ewoffsetselect::Cyc64
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == Ewoffsetselect::Cyc128
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == Ewoffsetselect::Cyc256
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == Ewoffsetselect::Cyc512
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == Ewoffsetselect::Cyc1024
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == Ewoffsetselect::Cyc2048
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == Ewoffsetselect::Cyc4096
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == Ewoffsetselect::Cyc8192
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == Ewoffsetselect::Cyc16384
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
    pub fn cyc8(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc8)
    }
    #[doc = "16 clock cycles"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc16)
    }
    #[doc = "32 clock cycles"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc32)
    }
    #[doc = "64 clock cycles"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc64)
    }
    #[doc = "128 clock cycles"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc128)
    }
    #[doc = "256 clock cycles"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc256)
    }
    #[doc = "512 clock cycles"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc512)
    }
    #[doc = "1024 clock cycles"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc1024)
    }
    #[doc = "2048 clock cycles"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc2048)
    }
    #[doc = "4096 clock cycles"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc4096)
    }
    #[doc = "8192 clock cycles"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc8192)
    }
    #[doc = "16384 clock cycles"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut crate::W<REG> {
        self.variant(Ewoffsetselect::Cyc16384)
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
    #[must_use]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EWCTRL to value 0x0b"]
impl crate::Resettable for EwctrlSpec {
    const RESET_VALUE: u8 = 0x0b;
}
