#[doc = "Register `MCCAR` reader"]
pub type R = crate::R<MccarSpec>;
#[doc = "Maximum Current for 3.3V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxcur33vselect {
    #[doc = "0: Get information via another method"]
    Other = 0,
    #[doc = "1: 4mA"]
    _4ma = 1,
    #[doc = "2: 8mA"]
    _8ma = 2,
    #[doc = "3: 12mA"]
    _12ma = 3,
}
impl From<Maxcur33vselect> for u8 {
    #[inline(always)]
    fn from(variant: Maxcur33vselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxcur33vselect {
    type Ux = u8;
}
impl crate::IsEnum for Maxcur33vselect {}
#[doc = "Field `MAXCUR33V` reader - Maximum Current for 3.3V"]
pub type Maxcur33vR = crate::FieldReader<Maxcur33vselect>;
impl Maxcur33vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxcur33vselect> {
        match self.bits {
            0 => Some(Maxcur33vselect::Other),
            1 => Some(Maxcur33vselect::_4ma),
            2 => Some(Maxcur33vselect::_8ma),
            3 => Some(Maxcur33vselect::_12ma),
            _ => None,
        }
    }
    #[doc = "Get information via another method"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Maxcur33vselect::Other
    }
    #[doc = "4mA"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == Maxcur33vselect::_4ma
    }
    #[doc = "8mA"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == Maxcur33vselect::_8ma
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == Maxcur33vselect::_12ma
    }
}
#[doc = "Maximum Current for 3.0V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxcur30vselect {
    #[doc = "0: Get information via another method"]
    Other = 0,
    #[doc = "1: 4mA"]
    _4ma = 1,
    #[doc = "2: 8mA"]
    _8ma = 2,
    #[doc = "3: 12mA"]
    _12ma = 3,
}
impl From<Maxcur30vselect> for u8 {
    #[inline(always)]
    fn from(variant: Maxcur30vselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxcur30vselect {
    type Ux = u8;
}
impl crate::IsEnum for Maxcur30vselect {}
#[doc = "Field `MAXCUR30V` reader - Maximum Current for 3.0V"]
pub type Maxcur30vR = crate::FieldReader<Maxcur30vselect>;
impl Maxcur30vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxcur30vselect> {
        match self.bits {
            0 => Some(Maxcur30vselect::Other),
            1 => Some(Maxcur30vselect::_4ma),
            2 => Some(Maxcur30vselect::_8ma),
            3 => Some(Maxcur30vselect::_12ma),
            _ => None,
        }
    }
    #[doc = "Get information via another method"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Maxcur30vselect::Other
    }
    #[doc = "4mA"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == Maxcur30vselect::_4ma
    }
    #[doc = "8mA"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == Maxcur30vselect::_8ma
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == Maxcur30vselect::_12ma
    }
}
#[doc = "Maximum Current for 1.8V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxcur18vselect {
    #[doc = "0: Get information via another method"]
    Other = 0,
    #[doc = "1: 4mA"]
    _4ma = 1,
    #[doc = "2: 8mA"]
    _8ma = 2,
    #[doc = "3: 12mA"]
    _12ma = 3,
}
impl From<Maxcur18vselect> for u8 {
    #[inline(always)]
    fn from(variant: Maxcur18vselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxcur18vselect {
    type Ux = u8;
}
impl crate::IsEnum for Maxcur18vselect {}
#[doc = "Field `MAXCUR18V` reader - Maximum Current for 1.8V"]
pub type Maxcur18vR = crate::FieldReader<Maxcur18vselect>;
impl Maxcur18vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxcur18vselect> {
        match self.bits {
            0 => Some(Maxcur18vselect::Other),
            1 => Some(Maxcur18vselect::_4ma),
            2 => Some(Maxcur18vselect::_8ma),
            3 => Some(Maxcur18vselect::_12ma),
            _ => None,
        }
    }
    #[doc = "Get information via another method"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Maxcur18vselect::Other
    }
    #[doc = "4mA"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == Maxcur18vselect::_4ma
    }
    #[doc = "8mA"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == Maxcur18vselect::_8ma
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == Maxcur18vselect::_12ma
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn maxcur33v(&self) -> Maxcur33vR {
        Maxcur33vR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline(always)]
    pub fn maxcur30v(&self) -> Maxcur30vR {
        Maxcur30vR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline(always)]
    pub fn maxcur18v(&self) -> Maxcur18vR {
        Maxcur18vR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`mccar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MccarSpec;
impl crate::RegisterSpec for MccarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mccar::R`](R) reader structure"]
impl crate::Readable for MccarSpec {}
#[doc = "`reset()` method sets MCCAR to value 0"]
impl crate::Resettable for MccarSpec {}
