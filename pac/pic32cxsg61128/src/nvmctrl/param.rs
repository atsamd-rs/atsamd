#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Field `NVMP` reader - NVM Pages"]
pub type NvmpR = crate::FieldReader<u16>;
#[doc = "Page Size\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pszselect {
    #[doc = "0: 8 bytes"]
    Psz8 = 0,
    #[doc = "1: 16 bytes"]
    Psz16 = 1,
    #[doc = "2: 32 bytes"]
    Psz32 = 2,
    #[doc = "3: 64 bytes"]
    Psz64 = 3,
    #[doc = "4: 128 bytes"]
    Psz128 = 4,
    #[doc = "5: 256 bytes"]
    Psz256 = 5,
    #[doc = "6: 512 bytes"]
    Psz512 = 6,
    #[doc = "7: 1024 bytes"]
    Psz1024 = 7,
}
impl From<Pszselect> for u8 {
    #[inline(always)]
    fn from(variant: Pszselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pszselect {
    type Ux = u8;
}
impl crate::IsEnum for Pszselect {}
#[doc = "Field `PSZ` reader - Page Size"]
pub type PszR = crate::FieldReader<Pszselect>;
impl PszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pszselect {
        match self.bits {
            0 => Pszselect::Psz8,
            1 => Pszselect::Psz16,
            2 => Pszselect::Psz32,
            3 => Pszselect::Psz64,
            4 => Pszselect::Psz128,
            5 => Pszselect::Psz256,
            6 => Pszselect::Psz512,
            7 => Pszselect::Psz1024,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_psz_8(&self) -> bool {
        *self == Pszselect::Psz8
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_psz_16(&self) -> bool {
        *self == Pszselect::Psz16
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_psz_32(&self) -> bool {
        *self == Pszselect::Psz32
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_psz_64(&self) -> bool {
        *self == Pszselect::Psz64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_psz_128(&self) -> bool {
        *self == Pszselect::Psz128
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_psz_256(&self) -> bool {
        *self == Pszselect::Psz256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_psz_512(&self) -> bool {
        *self == Pszselect::Psz512
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_psz_1024(&self) -> bool {
        *self == Pszselect::Psz1024
    }
}
#[doc = "SmartEEPROM Supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seeselect {
    #[doc = "1: SmartEEPROM is supported"]
    Smarteeprom = 1,
    #[doc = "0: No SmartEEPROM support"]
    Nosmarteeprom = 0,
}
impl From<Seeselect> for bool {
    #[inline(always)]
    fn from(variant: Seeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEE` reader - SmartEEPROM Supported"]
pub type SeeR = crate::BitReader<Seeselect>;
impl SeeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seeselect {
        match self.bits {
            true => Seeselect::Smarteeprom,
            false => Seeselect::Nosmarteeprom,
        }
    }
    #[doc = "SmartEEPROM is supported"]
    #[inline(always)]
    pub fn is_smarteeprom(&self) -> bool {
        *self == Seeselect::Smarteeprom
    }
    #[doc = "No SmartEEPROM support"]
    #[inline(always)]
    pub fn is_nosmarteeprom(&self) -> bool {
        *self == Seeselect::Nosmarteeprom
    }
}
impl R {
    #[doc = "Bits 0:15 - NVM Pages"]
    #[inline(always)]
    pub fn nvmp(&self) -> NvmpR {
        NvmpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PszR {
        PszR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 31 - SmartEEPROM Supported"]
    #[inline(always)]
    pub fn see(&self) -> SeeR {
        SeeR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "NVM Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0x0006_0000"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
