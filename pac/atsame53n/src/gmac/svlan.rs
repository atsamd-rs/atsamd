#[doc = "Register `SVLAN` reader"]
pub type R = crate::R<SVLAN_SPEC>;
#[doc = "Register `SVLAN` writer"]
pub type W = crate::W<SVLAN_SPEC>;
#[doc = "Field `VLAN_TYPE` reader - User Defined VLAN_TYPE Field"]
pub type VLAN_TYPE_R = crate::FieldReader<u16>;
#[doc = "Field `VLAN_TYPE` writer - User Defined VLAN_TYPE Field"]
pub type VLAN_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ESVLAN` reader - Enable Stacked VLAN Processing Mode"]
pub type ESVLAN_R = crate::BitReader;
#[doc = "Field `ESVLAN` writer - Enable Stacked VLAN Processing Mode"]
pub type ESVLAN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    pub fn vlan_type(&self) -> VLAN_TYPE_R {
        VLAN_TYPE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    pub fn esvlan(&self) -> ESVLAN_R {
        ESVLAN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_type(&mut self) -> VLAN_TYPE_W<SVLAN_SPEC, 0> {
        VLAN_TYPE_W::new(self)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn esvlan(&mut self) -> ESVLAN_W<SVLAN_SPEC, 31> {
        ESVLAN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Stacked VLAN Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svlan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svlan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SVLAN_SPEC;
impl crate::RegisterSpec for SVLAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`svlan::R`](R) reader structure"]
impl crate::Readable for SVLAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`svlan::W`](W) writer structure"]
impl crate::Writable for SVLAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVLAN to value 0"]
impl crate::Resettable for SVLAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
