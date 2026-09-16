#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TxefsSpec>;
#[doc = "Field `EFFL` reader - Event FIFO Fill Level"]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFGI` reader - Event FIFO Get Index"]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFPI` reader - Event FIFO Put Index"]
pub type EfpiR = crate::FieldReader;
#[doc = "Field `EFF` reader - Event FIFO Full"]
pub type EffR = crate::BitReader;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TeflR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index"]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Event FIFO Put Index"]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Tx Event FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefsSpec;
impl crate::RegisterSpec for TxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TxefsSpec {}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TxefsSpec {
    const RESET_VALUE: u32 = 0;
}
