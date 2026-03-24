#[doc = "Register `ECCERR` reader"]
pub type R = crate::R<EccerrSpec>;
#[doc = "Field `ADDR` reader - Error Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Low Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Typelselect {
    #[doc = "0: No Error Detected Since Last Read"]
    None = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    Single = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    Dual = 2,
}
impl From<Typelselect> for u8 {
    #[inline(always)]
    fn from(variant: Typelselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Typelselect {
    type Ux = u8;
}
impl crate::IsEnum for Typelselect {}
#[doc = "Field `TYPEL` reader - Low Double-Word Error Type"]
pub type TypelR = crate::FieldReader<Typelselect>;
impl TypelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Typelselect> {
        match self.bits {
            0 => Some(Typelselect::None),
            1 => Some(Typelselect::Single),
            2 => Some(Typelselect::Dual),
            _ => None,
        }
    }
    #[doc = "No Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Typelselect::None
    }
    #[doc = "At Least One Single Error Detected Since last Read"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Typelselect::Single
    }
    #[doc = "At Least One Dual Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Typelselect::Dual
    }
}
#[doc = "High Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Typehselect {
    #[doc = "0: No Error Detected Since Last Read"]
    None = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    Single = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    Dual = 2,
}
impl From<Typehselect> for u8 {
    #[inline(always)]
    fn from(variant: Typehselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Typehselect {
    type Ux = u8;
}
impl crate::IsEnum for Typehselect {}
#[doc = "Field `TYPEH` reader - High Double-Word Error Type"]
pub type TypehR = crate::FieldReader<Typehselect>;
impl TypehR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Typehselect> {
        match self.bits {
            0 => Some(Typehselect::None),
            1 => Some(Typehselect::Single),
            2 => Some(Typehselect::Dual),
            _ => None,
        }
    }
    #[doc = "No Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Typehselect::None
    }
    #[doc = "At Least One Single Error Detected Since last Read"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Typehselect::Single
    }
    #[doc = "At Least One Dual Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Typehselect::Dual
    }
}
impl R {
    #[doc = "Bits 0:23 - Error Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 28:29 - Low Double-Word Error Type"]
    #[inline(always)]
    pub fn typel(&self) -> TypelR {
        TypelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - High Double-Word Error Type"]
    #[inline(always)]
    pub fn typeh(&self) -> TypehR {
        TypehR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "ECC Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccerr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccerrSpec;
impl crate::RegisterSpec for EccerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccerr::R`](R) reader structure"]
impl crate::Readable for EccerrSpec {}
#[doc = "`reset()` method sets ECCERR to value 0"]
impl crate::Resettable for EccerrSpec {
    const RESET_VALUE: u32 = 0;
}
