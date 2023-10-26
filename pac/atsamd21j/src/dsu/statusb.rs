#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<STATUSB_SPEC>;
#[doc = "Field `PROT` reader - Protected"]
pub type PROT_R = crate::BitReader;
#[doc = "Field `DBGPRES` reader - Debugger Present"]
pub type DBGPRES_R = crate::BitReader;
#[doc = "Field `DCCD0` reader - Debug Communication Channel 0 Dirty"]
pub type DCCD0_R = crate::BitReader;
#[doc = "Field `DCCD1` reader - Debug Communication Channel 1 Dirty"]
pub type DCCD1_R = crate::BitReader;
#[doc = "Field `HPE` reader - Hot-Plugging Enable"]
pub type HPE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Protected"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debugger Present"]
    #[inline(always)]
    pub fn dbgpres(&self) -> DBGPRES_R {
        DBGPRES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn dccd0(&self) -> DCCD0_R {
        DCCD0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn dccd1(&self) -> DCCD1_R {
        DCCD1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hot-Plugging Enable"]
    #[inline(always)]
    pub fn hpe(&self) -> HPE_R {
        HPE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {}
#[doc = "`reset()` method sets STATUSB to value 0x10"]
impl crate::Resettable for STATUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
