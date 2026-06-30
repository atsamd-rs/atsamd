#[doc = "Register `SVLAN` reader"]
pub type R = crate::R<SvlanSpec>;
#[doc = "Register `SVLAN` writer"]
pub type W = crate::W<SvlanSpec>;
#[doc = "Field `VLAN_TYPE` reader - User Defined VLAN_TYPE Field"]
pub type VlanTypeR = crate::FieldReader<u16>;
#[doc = "Field `VLAN_TYPE` writer - User Defined VLAN_TYPE Field"]
pub type VlanTypeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ESVLAN` reader - Enable Stacked VLAN Processing Mode"]
pub type EsvlanR = crate::BitReader;
#[doc = "Field `ESVLAN` writer - Enable Stacked VLAN Processing Mode"]
pub type EsvlanW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    pub fn vlan_type(&self) -> VlanTypeR {
        VlanTypeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    pub fn esvlan(&self) -> EsvlanR {
        EsvlanR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_type(&mut self) -> VlanTypeW<SvlanSpec> {
        VlanTypeW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn esvlan(&mut self) -> EsvlanW<SvlanSpec> {
        EsvlanW::new(self, 31)
    }
}
#[doc = "Stacked VLAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`svlan::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svlan::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SvlanSpec;
impl crate::RegisterSpec for SvlanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`svlan::R`](R) reader structure"]
impl crate::Readable for SvlanSpec {}
#[doc = "`write(|w| ..)` method takes [`svlan::W`](W) writer structure"]
impl crate::Writable for SvlanSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SVLAN to value 0"]
impl crate::Resettable for SvlanSpec {
    const RESET_VALUE: u32 = 0;
}
