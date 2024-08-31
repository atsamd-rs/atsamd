#[doc = "Register `HCVR` reader"]
pub type R = crate::R<HcvrSpec>;
#[doc = "Field `SVER` reader - Spec Version"]
pub type SverR = crate::FieldReader;
#[doc = "Field `VVER` reader - Vendor Version"]
pub type VverR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Spec Version"]
    #[inline(always)]
    pub fn sver(&self) -> SverR {
        SverR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version"]
    #[inline(always)]
    pub fn vver(&self) -> VverR {
        VverR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Version\n\nYou can [`read`](crate::Reg::read) this register and get [`hcvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcvrSpec;
impl crate::RegisterSpec for HcvrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hcvr::R`](R) reader structure"]
impl crate::Readable for HcvrSpec {}
#[doc = "`reset()` method sets HCVR to value 0x1802"]
impl crate::Resettable for HcvrSpec {
    const RESET_VALUE: u16 = 0x1802;
}
