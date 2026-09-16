#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Field `GCLK` reader - dynamic Clock Gating supported"]
pub type GclkR = crate::BitReader;
#[doc = "Field `RRP` reader - Round Robin Policy supported"]
pub type RrpR = crate::BitReader;
#[doc = "Number of Way\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Waynumselect {
    #[doc = "2: 4-WAY set associative"]
    Arch4way = 2,
}
impl From<Waynumselect> for u8 {
    #[inline(always)]
    fn from(variant: Waynumselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Waynumselect {
    type Ux = u8;
}
impl crate::IsEnum for Waynumselect {}
#[doc = "Field `WAYNUM` reader - Number of Way"]
pub type WaynumR = crate::FieldReader<Waynumselect>;
impl WaynumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Waynumselect> {
        match self.bits {
            2 => Some(Waynumselect::Arch4way),
            _ => None,
        }
    }
    #[doc = "4-WAY set associative"]
    #[inline(always)]
    pub fn is_arch4way(&self) -> bool {
        *self == Waynumselect::Arch4way
    }
}
#[doc = "Field `LCKDOWN` reader - Lock Down supported"]
pub type LckdownR = crate::BitReader;
#[doc = "Cache Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csizeselect {
    #[doc = "0: Cache Size is 1 KB"]
    Csize1kb = 0,
    #[doc = "1: Cache Size is 2 KB"]
    Csize2kb = 1,
    #[doc = "2: Cache Size is 4 KB"]
    Csize4kb = 2,
}
impl From<Csizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Csizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Csizeselect {}
#[doc = "Field `CSIZE` reader - Cache Size"]
pub type CsizeR = crate::FieldReader<Csizeselect>;
impl CsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csizeselect> {
        match self.bits {
            0 => Some(Csizeselect::Csize1kb),
            1 => Some(Csizeselect::Csize2kb),
            2 => Some(Csizeselect::Csize4kb),
            _ => None,
        }
    }
    #[doc = "Cache Size is 1 KB"]
    #[inline(always)]
    pub fn is_csize_1kb(&self) -> bool {
        *self == Csizeselect::Csize1kb
    }
    #[doc = "Cache Size is 2 KB"]
    #[inline(always)]
    pub fn is_csize_2kb(&self) -> bool {
        *self == Csizeselect::Csize2kb
    }
    #[doc = "Cache Size is 4 KB"]
    #[inline(always)]
    pub fn is_csize_4kb(&self) -> bool {
        *self == Csizeselect::Csize4kb
    }
}
#[doc = "Cache Line Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clsizeselect {
    #[doc = "2: Cache Line Size is 16 bytes"]
    Clsize16b = 2,
}
impl From<Clsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Clsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Clsizeselect {}
#[doc = "Field `CLSIZE` reader - Cache Line Size"]
pub type ClsizeR = crate::FieldReader<Clsizeselect>;
impl ClsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clsizeselect> {
        match self.bits {
            2 => Some(Clsizeselect::Clsize16b),
            _ => None,
        }
    }
    #[doc = "Cache Line Size is 16 bytes"]
    #[inline(always)]
    pub fn is_clsize_16b(&self) -> bool {
        *self == Clsizeselect::Clsize16b
    }
}
impl R {
    #[doc = "Bit 1 - dynamic Clock Gating supported"]
    #[inline(always)]
    pub fn gclk(&self) -> GclkR {
        GclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Round Robin Policy supported"]
    #[inline(always)]
    pub fn rrp(&self) -> RrpR {
        RrpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Number of Way"]
    #[inline(always)]
    pub fn waynum(&self) -> WaynumR {
        WaynumR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Lock Down supported"]
    #[inline(always)]
    pub fn lckdown(&self) -> LckdownR {
        LckdownR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Cache Size"]
    #[inline(always)]
    pub fn csize(&self) -> CsizeR {
        CsizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Cache Line Size"]
    #[inline(always)]
    pub fn clsize(&self) -> ClsizeR {
        ClsizeR::new(((self.bits >> 11) & 7) as u8)
    }
}
#[doc = "Cache Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TypeSpec;
impl crate::RegisterSpec for TypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TypeSpec {}
#[doc = "`reset()` method sets TYPE to value 0x12d2"]
impl crate::Resettable for TypeSpec {
    const RESET_VALUE: u32 = 0x12d2;
}
