#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<Rxf1sSpec>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 Fill Level"]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1GI` reader - Rx FIFO 1 Get Index"]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 Put Index"]
pub type F1piR = crate::FieldReader;
#[doc = "Field `F1F` reader - Rx FIFO 1 Full"]
pub type F1fR = crate::BitReader;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Debug Message Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmsselect {
    #[doc = "0: Idle state"]
    Idle = 0,
    #[doc = "1: Debug message A received"]
    Dbga = 1,
    #[doc = "2: Debug message A/B received"]
    Dbgb = 2,
    #[doc = "3: Debug message A/B/C received, DMA request set"]
    Dbgc = 3,
}
impl From<Dmsselect> for u8 {
    #[inline(always)]
    fn from(variant: Dmsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmsselect {
    type Ux = u8;
}
impl crate::IsEnum for Dmsselect {}
#[doc = "Field `DMS` reader - Debug Message Status"]
pub type DmsR = crate::FieldReader<Dmsselect>;
impl DmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmsselect {
        match self.bits {
            0 => Dmsselect::Idle,
            1 => Dmsselect::Dbga,
            2 => Dmsselect::Dbgb,
            3 => Dmsselect::Dbgc,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle state"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Dmsselect::Idle
    }
    #[doc = "Debug message A received"]
    #[inline(always)]
    pub fn is_dbga(&self) -> bool {
        *self == Dmsselect::Dbga
    }
    #[doc = "Debug message A/B received"]
    #[inline(always)]
    pub fn is_dbgb(&self) -> bool {
        *self == Dmsselect::Dbgb
    }
    #[doc = "Debug message A/B/C received, DMA request set"]
    #[inline(always)]
    pub fn is_dbgc(&self) -> bool {
        *self == Dmsselect::Dbgc
    }
}
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Rx FIFO 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1sSpec;
impl crate::RegisterSpec for Rxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for Rxf1sSpec {}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for Rxf1sSpec {
    const RESET_VALUE: u32 = 0;
}
