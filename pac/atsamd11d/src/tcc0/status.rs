#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `STOP` reader - Stop"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `IDX` reader - Ramp"]
pub type IDX_R = crate::BitReader;
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault State"]
pub type DFS_R = crate::BitReader;
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault State"]
pub type DFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAVE` reader - Slave"]
pub type SLAVE_R = crate::BitReader;
#[doc = "Field `PATTBV` reader - Pattern Buffer Valid"]
pub type PATTBV_R = crate::BitReader;
#[doc = "Field `PATTBV` writer - Pattern Buffer Valid"]
pub type PATTBV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAVEBV` reader - Wave Buffer Valid"]
pub type WAVEBV_R = crate::BitReader;
#[doc = "Field `WAVEBV` writer - Wave Buffer Valid"]
pub type WAVEBV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERBV` reader - Period Buffer Valid"]
pub type PERBV_R = crate::BitReader;
#[doc = "Field `PERBV` writer - Period Buffer Valid"]
pub type PERBV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULTAIN` reader - Recoverable Fault A Input"]
pub type FAULTAIN_R = crate::BitReader;
#[doc = "Field `FAULTBIN` reader - Recoverable Fault B Input"]
pub type FAULTBIN_R = crate::BitReader;
#[doc = "Field `FAULT0IN` reader - Non-Recoverable Fault0 Input"]
pub type FAULT0IN_R = crate::BitReader;
#[doc = "Field `FAULT1IN` reader - Non-Recoverable Fault1 Input"]
pub type FAULT1IN_R = crate::BitReader;
#[doc = "Field `FAULTA` reader - Recoverable Fault A State"]
pub type FAULTA_R = crate::BitReader;
#[doc = "Field `FAULTA` writer - Recoverable Fault A State"]
pub type FAULTA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULTB` reader - Recoverable Fault B State"]
pub type FAULTB_R = crate::BitReader;
#[doc = "Field `FAULTB` writer - Recoverable Fault B State"]
pub type FAULTB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0 State"]
pub type FAULT0_R = crate::BitReader;
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0 State"]
pub type FAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1 State"]
pub type FAULT1_R = crate::BitReader;
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1 State"]
pub type FAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCBV0` reader - Compare Channel 0 Buffer Valid"]
pub type CCBV0_R = crate::BitReader;
#[doc = "Field `CCBV0` writer - Compare Channel 0 Buffer Valid"]
pub type CCBV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCBV1` reader - Compare Channel 1 Buffer Valid"]
pub type CCBV1_R = crate::BitReader;
#[doc = "Field `CCBV1` writer - Compare Channel 1 Buffer Valid"]
pub type CCBV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCBV2` reader - Compare Channel 2 Buffer Valid"]
pub type CCBV2_R = crate::BitReader;
#[doc = "Field `CCBV2` writer - Compare Channel 2 Buffer Valid"]
pub type CCBV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCBV3` reader - Compare Channel 3 Buffer Valid"]
pub type CCBV3_R = crate::BitReader;
#[doc = "Field `CCBV3` writer - Compare Channel 3 Buffer Valid"]
pub type CCBV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP0` reader - Compare Channel 0 Value"]
pub type CMP0_R = crate::BitReader;
#[doc = "Field `CMP1` reader - Compare Channel 1 Value"]
pub type CMP1_R = crate::BitReader;
#[doc = "Field `CMP2` reader - Compare Channel 2 Value"]
pub type CMP2_R = crate::BitReader;
#[doc = "Field `CMP3` reader - Compare Channel 3 Value"]
pub type CMP3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ramp"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbv(&self) -> PATTBV_R {
        PATTBV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    pub fn wavebv(&self) -> WAVEBV_R {
        WAVEBV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&self) -> PERBV_R {
        PERBV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Recoverable Fault A Input"]
    #[inline(always)]
    pub fn faultain(&self) -> FAULTAIN_R {
        FAULTAIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Recoverable Fault B Input"]
    #[inline(always)]
    pub fn faultbin(&self) -> FAULTBIN_R {
        FAULTBIN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
    #[inline(always)]
    pub fn fault0in(&self) -> FAULT0IN_R {
        FAULT0IN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
    #[inline(always)]
    pub fn fault1in(&self) -> FAULT1IN_R {
        FAULT1IN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    pub fn faulta(&self) -> FAULTA_R {
        FAULTA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    pub fn faultb(&self) -> FAULTB_R {
        FAULTB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv0(&self) -> CCBV0_R {
        CCBV0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv1(&self) -> CCBV1_R {
        CCBV1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv2(&self) -> CCBV2_R {
        CCBV2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv3(&self) -> CCBV3_R {
        CCBV3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Compare Channel 0 Value"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Compare Channel 1 Value"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Compare Channel 2 Value"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Compare Channel 3 Value"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DFS_W<STATUS_SPEC, 3> {
        DFS_W::new(self)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn pattbv(&mut self) -> PATTBV_W<STATUS_SPEC, 5> {
        PATTBV_W::new(self)
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn wavebv(&mut self) -> WAVEBV_W<STATUS_SPEC, 6> {
        WAVEBV_W::new(self)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn perbv(&mut self) -> PERBV_W<STATUS_SPEC, 7> {
        PERBV_W::new(self)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    #[must_use]
    pub fn faulta(&mut self) -> FAULTA_W<STATUS_SPEC, 12> {
        FAULTA_W::new(self)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    #[must_use]
    pub fn faultb(&mut self) -> FAULTB_W<STATUS_SPEC, 13> {
        FAULTB_W::new(self)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    #[must_use]
    pub fn fault0(&mut self) -> FAULT0_W<STATUS_SPEC, 14> {
        FAULT0_W::new(self)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<STATUS_SPEC, 15> {
        FAULT1_W::new(self)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbv0(&mut self) -> CCBV0_W<STATUS_SPEC, 16> {
        CCBV0_W::new(self)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbv1(&mut self) -> CCBV1_W<STATUS_SPEC, 17> {
        CCBV1_W::new(self)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbv2(&mut self) -> CCBV2_W<STATUS_SPEC, 18> {
        CCBV2_W::new(self)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbv3(&mut self) -> CCBV3_W<STATUS_SPEC, 19> {
        CCBV3_W::new(self)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
