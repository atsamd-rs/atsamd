#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Synchroniation Busy bit"]
pub type SwrstR = crate::BitReader;
#[doc = "Generic Clock Generator Control n Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Genctrlselect {
    #[doc = "1: Generic clock generator 0"]
    Gclk0 = 1,
    #[doc = "2: Generic clock generator 1"]
    Gclk1 = 2,
    #[doc = "4: Generic clock generator 2"]
    Gclk2 = 4,
    #[doc = "8: Generic clock generator 3"]
    Gclk3 = 8,
    #[doc = "16: Generic clock generator 4"]
    Gclk4 = 16,
    #[doc = "32: Generic clock generator 5"]
    Gclk5 = 32,
    #[doc = "64: Generic clock generator 6"]
    Gclk6 = 64,
    #[doc = "128: Generic clock generator 7"]
    Gclk7 = 128,
    #[doc = "256: Generic clock generator 8"]
    Gclk8 = 256,
    #[doc = "512: Generic clock generator 9"]
    Gclk9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    Gclk10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    Gclk11 = 2048,
}
impl From<Genctrlselect> for u16 {
    #[inline(always)]
    fn from(variant: Genctrlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Genctrlselect {
    type Ux = u16;
}
impl crate::IsEnum for Genctrlselect {}
#[doc = "Field `GENCTRL` reader - Generic Clock Generator Control n Synchronization Busy bits"]
pub type GenctrlR = crate::FieldReader<Genctrlselect>;
impl GenctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Genctrlselect> {
        match self.bits {
            1 => Some(Genctrlselect::Gclk0),
            2 => Some(Genctrlselect::Gclk1),
            4 => Some(Genctrlselect::Gclk2),
            8 => Some(Genctrlselect::Gclk3),
            16 => Some(Genctrlselect::Gclk4),
            32 => Some(Genctrlselect::Gclk5),
            64 => Some(Genctrlselect::Gclk6),
            128 => Some(Genctrlselect::Gclk7),
            256 => Some(Genctrlselect::Gclk8),
            512 => Some(Genctrlselect::Gclk9),
            1024 => Some(Genctrlselect::Gclk10),
            2048 => Some(Genctrlselect::Gclk11),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == Genctrlselect::Gclk0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == Genctrlselect::Gclk1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == Genctrlselect::Gclk2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == Genctrlselect::Gclk3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == Genctrlselect::Gclk4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == Genctrlselect::Gclk5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == Genctrlselect::Gclk6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == Genctrlselect::Gclk7
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == Genctrlselect::Gclk8
    }
    #[doc = "Generic clock generator 9"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == Genctrlselect::Gclk9
    }
    #[doc = "Generic clock generator 10"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == Genctrlselect::Gclk10
    }
    #[doc = "Generic clock generator 11"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == Genctrlselect::Gclk11
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:13 - Generic Clock Generator Control n Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl(&self) -> GenctrlR {
        GenctrlR::new(((self.bits >> 2) & 0x0fff) as u16)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {
    const RESET_VALUE: u32 = 0;
}
