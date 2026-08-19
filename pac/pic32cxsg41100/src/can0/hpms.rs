#[doc = "Register `HPMS` reader"]
pub type R = crate::R<HpmsSpec>;
#[doc = "Field `BIDX` reader - Buffer Index"]
pub type BidxR = crate::FieldReader;
#[doc = "Message Storage Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msiselect {
    #[doc = "0: No FIFO selected"]
    None = 0,
    #[doc = "1: FIFO message lost"]
    Lost = 1,
    #[doc = "2: Message stored in FIFO 0"]
    Fifo0 = 2,
    #[doc = "3: Message stored in FIFO 1"]
    Fifo1 = 3,
}
impl From<Msiselect> for u8 {
    #[inline(always)]
    fn from(variant: Msiselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msiselect {
    type Ux = u8;
}
impl crate::IsEnum for Msiselect {}
#[doc = "Field `MSI` reader - Message Storage Indicator"]
pub type MsiR = crate::FieldReader<Msiselect>;
impl MsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msiselect {
        match self.bits {
            0 => Msiselect::None,
            1 => Msiselect::Lost,
            2 => Msiselect::Fifo0,
            3 => Msiselect::Fifo1,
            _ => unreachable!(),
        }
    }
    #[doc = "No FIFO selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Msiselect::None
    }
    #[doc = "FIFO message lost"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == Msiselect::Lost
    }
    #[doc = "Message stored in FIFO 0"]
    #[inline(always)]
    pub fn is_fifo0(&self) -> bool {
        *self == Msiselect::Fifo0
    }
    #[doc = "Message stored in FIFO 1"]
    #[inline(always)]
    pub fn is_fifo1(&self) -> bool {
        *self == Msiselect::Fifo1
    }
}
#[doc = "Field `FIDX` reader - Filter Index"]
pub type FidxR = crate::FieldReader;
#[doc = "Field `FLST` reader - Filter List"]
pub type FlstR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FlstR {
        FlstR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "High Priority Message Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpmsSpec;
impl crate::RegisterSpec for HpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpms::R`](R) reader structure"]
impl crate::Readable for HpmsSpec {}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HpmsSpec {
    const RESET_VALUE: u32 = 0;
}
