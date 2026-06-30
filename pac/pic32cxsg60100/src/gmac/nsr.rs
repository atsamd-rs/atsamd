#[doc = "Register `NSR` reader"]
pub type R = crate::R<NsrSpec>;
#[doc = "Field `MDIO` reader - MDIO Input Status"]
pub type MdioR = crate::BitReader;
#[doc = "Field `IDLE` reader - PHY Management Logic Idle"]
pub type IdleR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - MDIO Input Status"]
    #[inline(always)]
    pub fn mdio(&self) -> MdioR {
        MdioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY Management Logic Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Network Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsrSpec;
impl crate::RegisterSpec for NsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsr::R`](R) reader structure"]
impl crate::Readable for NsrSpec {}
#[doc = "`reset()` method sets NSR to value 0x04"]
impl crate::Resettable for NsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
