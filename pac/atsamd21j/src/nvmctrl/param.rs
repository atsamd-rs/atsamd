#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Register `PARAM` writer"]
pub type W = crate::W<ParamSpec>;
#[doc = "Field `NVMP` reader - NVM Pages"]
pub type NvmpR = crate::FieldReader<u16>;
#[doc = "Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pszselect {
    #[doc = "0: 8 bytes"]
    _8 = 0,
    #[doc = "1: 16 bytes"]
    _16 = 1,
    #[doc = "2: 32 bytes"]
    _32 = 2,
    #[doc = "3: 64 bytes"]
    _64 = 3,
    #[doc = "4: 128 bytes"]
    _128 = 4,
    #[doc = "5: 256 bytes"]
    _256 = 5,
    #[doc = "6: 512 bytes"]
    _512 = 6,
    #[doc = "7: 1024 bytes"]
    _1024 = 7,
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
            0 => Pszselect::_8,
            1 => Pszselect::_16,
            2 => Pszselect::_32,
            3 => Pszselect::_64,
            4 => Pszselect::_128,
            5 => Pszselect::_256,
            6 => Pszselect::_512,
            7 => Pszselect::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Pszselect::_8
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Pszselect::_16
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Pszselect::_32
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Pszselect::_64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Pszselect::_128
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Pszselect::_256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Pszselect::_512
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Pszselect::_1024
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
}
impl W {}
#[doc = "NVM Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`write(|w| ..)` method takes [`param::W`](W) writer structure"]
impl crate::Writable for ParamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0;
}
