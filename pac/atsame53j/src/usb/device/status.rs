#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speedselect {
    #[doc = "0: Full-speed mode"]
    Fs = 0,
    #[doc = "1: Low-speed mode"]
    Ls = 1,
    #[doc = "2: High-speed mode"]
    Hs = 2,
}
impl From<Speedselect> for u8 {
    #[inline(always)]
    fn from(variant: Speedselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speedselect {
    type Ux = u8;
}
impl crate::IsEnum for Speedselect {}
#[doc = "Field `SPEED` reader - Speed Status"]
pub type SpeedR = crate::FieldReader<Speedselect>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speedselect> {
        match self.bits {
            0 => Some(Speedselect::Fs),
            1 => Some(Speedselect::Ls),
            2 => Some(Speedselect::Hs),
            _ => None,
        }
    }
    #[doc = "Full-speed mode"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == Speedselect::Fs
    }
    #[doc = "Low-speed mode"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == Speedselect::Ls
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == Speedselect::Hs
    }
}
#[doc = "USB Line State Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Linestateselect {
    #[doc = "0: SE0/RESET"]
    _0 = 0,
    #[doc = "1: FS-J or LS-K State"]
    _1 = 1,
    #[doc = "2: FS-K or LS-J State"]
    _2 = 2,
}
impl From<Linestateselect> for u8 {
    #[inline(always)]
    fn from(variant: Linestateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Linestateselect {
    type Ux = u8;
}
impl crate::IsEnum for Linestateselect {}
#[doc = "Field `LINESTATE` reader - USB Line State Status"]
pub type LinestateR = crate::FieldReader<Linestateselect>;
impl LinestateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Linestateselect> {
        match self.bits {
            0 => Some(Linestateselect::_0),
            1 => Some(Linestateselect::_1),
            2 => Some(Linestateselect::_2),
            _ => None,
        }
    }
    #[doc = "SE0/RESET"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Linestateselect::_0
    }
    #[doc = "FS-J or LS-K State"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Linestateselect::_1
    }
    #[doc = "FS-K or LS-J State"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Linestateselect::_2
    }
}
impl R {
    #[doc = "Bits 2:3 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 6:7 - USB Line State Status"]
    #[inline(always)]
    pub fn linestate(&self) -> LinestateR {
        LinestateR::new((self.bits >> 6) & 3)
    }
}
#[doc = "DEVICE Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0x40;
}
