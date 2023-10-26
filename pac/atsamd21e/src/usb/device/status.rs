#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `SPEED` reader - Speed Status"]
pub type SPEED_R = crate::FieldReader<SPEEDSELECT_A>;
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEEDSELECT_A {
    #[doc = "0: Full-speed mode"]
    FS = 0,
    #[doc = "1: High-speed mode"]
    HS = 1,
    #[doc = "2: Low-speed mode"]
    LS = 2,
}
impl From<SPEEDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEEDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEEDSELECT_A {
    type Ux = u8;
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPEEDSELECT_A> {
        match self.bits {
            0 => Some(SPEEDSELECT_A::FS),
            1 => Some(SPEEDSELECT_A::HS),
            2 => Some(SPEEDSELECT_A::LS),
            _ => None,
        }
    }
    #[doc = "Full-speed mode"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == SPEEDSELECT_A::FS
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == SPEEDSELECT_A::HS
    }
    #[doc = "Low-speed mode"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == SPEEDSELECT_A::LS
    }
}
#[doc = "Field `LINESTATE` reader - USB Line State Status"]
pub type LINESTATE_R = crate::FieldReader<LINESTATESELECT_A>;
#[doc = "USB Line State Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LINESTATESELECT_A {
    #[doc = "0: SE0/RESET"]
    _0 = 0,
    #[doc = "1: FS-J or LS-K State"]
    _1 = 1,
    #[doc = "2: FS-K or LS-J State"]
    _2 = 2,
}
impl From<LINESTATESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LINESTATESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LINESTATESELECT_A {
    type Ux = u8;
}
impl LINESTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LINESTATESELECT_A> {
        match self.bits {
            0 => Some(LINESTATESELECT_A::_0),
            1 => Some(LINESTATESELECT_A::_1),
            2 => Some(LINESTATESELECT_A::_2),
            _ => None,
        }
    }
    #[doc = "SE0/RESET"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATESELECT_A::_0
    }
    #[doc = "FS-J or LS-K State"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATESELECT_A::_1
    }
    #[doc = "FS-K or LS-J State"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == LINESTATESELECT_A::_2
    }
}
impl R {
    #[doc = "Bits 2:3 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 6:7 - USB Line State Status"]
    #[inline(always)]
    pub fn linestate(&self) -> LINESTATE_R {
        LINESTATE_R::new((self.bits >> 6) & 3)
    }
}
#[doc = "DEVICE Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
