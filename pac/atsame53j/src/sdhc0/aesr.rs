#[doc = "Register `AESR` reader"]
pub type R = crate::R<AesrSpec>;
#[doc = "ADMA Error State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Errstselect {
    #[doc = "0: ST_STOP (Stop DMA)"]
    Stop = 0,
    #[doc = "1: ST_FDS (Fetch Descriptor)"]
    Fds = 1,
    #[doc = "3: ST_TFR (Transfer Data)"]
    Tfr = 3,
}
impl From<Errstselect> for u8 {
    #[inline(always)]
    fn from(variant: Errstselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Errstselect {
    type Ux = u8;
}
impl crate::IsEnum for Errstselect {}
#[doc = "Field `ERRST` reader - ADMA Error State"]
pub type ErrstR = crate::FieldReader<Errstselect>;
impl ErrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Errstselect> {
        match self.bits {
            0 => Some(Errstselect::Stop),
            1 => Some(Errstselect::Fds),
            3 => Some(Errstselect::Tfr),
            _ => None,
        }
    }
    #[doc = "ST_STOP (Stop DMA)"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Errstselect::Stop
    }
    #[doc = "ST_FDS (Fetch Descriptor)"]
    #[inline(always)]
    pub fn is_fds(&self) -> bool {
        *self == Errstselect::Fds
    }
    #[doc = "ST_TFR (Transfer Data)"]
    #[inline(always)]
    pub fn is_tfr(&self) -> bool {
        *self == Errstselect::Tfr
    }
}
#[doc = "ADMA Length Mismatch Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lmisselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Lmisselect> for bool {
    #[inline(always)]
    fn from(variant: Lmisselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMIS` reader - ADMA Length Mismatch Error"]
pub type LmisR = crate::BitReader<Lmisselect>;
impl LmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lmisselect {
        match self.bits {
            false => Lmisselect::No,
            true => Lmisselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Lmisselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Lmisselect::Yes
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline(always)]
    pub fn errst(&self) -> ErrstR {
        ErrstR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn lmis(&self) -> LmisR {
        LmisR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADMA Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesrSpec;
impl crate::RegisterSpec for AesrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aesr::R`](R) reader structure"]
impl crate::Readable for AesrSpec {}
#[doc = "`reset()` method sets AESR to value 0"]
impl crate::Resettable for AesrSpec {}
