#[doc = "Register `HPMS` reader"]
pub type R = crate::R<HPMS_SPEC>;
#[doc = "Field `BIDX` reader - Buffer Index"]
pub type BIDX_R = crate::FieldReader;
#[doc = "Field `MSI` reader - Message Storage Indicator"]
pub type MSI_R = crate::FieldReader<MSISELECT_A>;
#[doc = "Message Storage Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISELECT_A {
    #[doc = "0: No FIFO selected"]
    NONE = 0,
    #[doc = "1: FIFO message lost"]
    LOST = 1,
    #[doc = "2: Message stored in FIFO 0"]
    FIFO0 = 2,
    #[doc = "3: Message stored in FIFO 1"]
    FIFO1 = 3,
}
impl From<MSISELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MSISELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSISELECT_A {
    type Ux = u8;
}
impl MSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSISELECT_A {
        match self.bits {
            0 => MSISELECT_A::NONE,
            1 => MSISELECT_A::LOST,
            2 => MSISELECT_A::FIFO0,
            3 => MSISELECT_A::FIFO1,
            _ => unreachable!(),
        }
    }
    #[doc = "No FIFO selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MSISELECT_A::NONE
    }
    #[doc = "FIFO message lost"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == MSISELECT_A::LOST
    }
    #[doc = "Message stored in FIFO 0"]
    #[inline(always)]
    pub fn is_fifo0(&self) -> bool {
        *self == MSISELECT_A::FIFO0
    }
    #[doc = "Message stored in FIFO 1"]
    #[inline(always)]
    pub fn is_fifo1(&self) -> bool {
        *self == MSISELECT_A::FIFO1
    }
}
#[doc = "Field `FIDX` reader - Filter Index"]
pub type FIDX_R = crate::FieldReader;
#[doc = "Field `FLST` reader - Filter List"]
pub type FLST_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "High Priority Message Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpms::R`](R) reader structure"]
impl crate::Readable for HPMS_SPEC {}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HPMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
