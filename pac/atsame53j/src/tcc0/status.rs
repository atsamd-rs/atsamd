#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - Stop"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Stop"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `IDX` reader - Ramp"]
pub type IDX_R = crate::BitReader<bool>;
#[doc = "Field `IDX` writer - Ramp"]
pub type IDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `UFS` reader - Non-recoverable Update Fault State"]
pub type UFS_R = crate::BitReader<bool>;
#[doc = "Field `UFS` writer - Non-recoverable Update Fault State"]
pub type UFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault State"]
pub type DFS_R = crate::BitReader<bool>;
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault State"]
pub type DFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `SLAVE` reader - Slave"]
pub type SLAVE_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE` writer - Slave"]
pub type SLAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `PATTBUFV` reader - Pattern Buffer Valid"]
pub type PATTBUFV_R = crate::BitReader<bool>;
#[doc = "Field `PATTBUFV` writer - Pattern Buffer Valid"]
pub type PATTBUFV_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `PERBUFV` reader - Period Buffer Valid"]
pub type PERBUFV_R = crate::BitReader<bool>;
#[doc = "Field `PERBUFV` writer - Period Buffer Valid"]
pub type PERBUFV_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULTAIN` reader - Recoverable Fault A Input"]
pub type FAULTAIN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTAIN` writer - Recoverable Fault A Input"]
pub type FAULTAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULTBIN` reader - Recoverable Fault B Input"]
pub type FAULTBIN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTBIN` writer - Recoverable Fault B Input"]
pub type FAULTBIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULT0IN` reader - Non-Recoverable Fault0 Input"]
pub type FAULT0IN_R = crate::BitReader<bool>;
#[doc = "Field `FAULT0IN` writer - Non-Recoverable Fault0 Input"]
pub type FAULT0IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULT1IN` reader - Non-Recoverable Fault1 Input"]
pub type FAULT1IN_R = crate::BitReader<bool>;
#[doc = "Field `FAULT1IN` writer - Non-Recoverable Fault1 Input"]
pub type FAULT1IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULTA` reader - Recoverable Fault A State"]
pub type FAULTA_R = crate::BitReader<bool>;
#[doc = "Field `FAULTA` writer - Recoverable Fault A State"]
pub type FAULTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULTB` reader - Recoverable Fault B State"]
pub type FAULTB_R = crate::BitReader<bool>;
#[doc = "Field `FAULTB` writer - Recoverable Fault B State"]
pub type FAULTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0 State"]
pub type FAULT0_R = crate::BitReader<bool>;
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0 State"]
pub type FAULT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1 State"]
pub type FAULT1_R = crate::BitReader<bool>;
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1 State"]
pub type FAULT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV0` reader - Compare Channel 0 Buffer Valid"]
pub type CCBUFV0_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV0` writer - Compare Channel 0 Buffer Valid"]
pub type CCBUFV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV1` reader - Compare Channel 1 Buffer Valid"]
pub type CCBUFV1_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV1` writer - Compare Channel 1 Buffer Valid"]
pub type CCBUFV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV2` reader - Compare Channel 2 Buffer Valid"]
pub type CCBUFV2_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV2` writer - Compare Channel 2 Buffer Valid"]
pub type CCBUFV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV3` reader - Compare Channel 3 Buffer Valid"]
pub type CCBUFV3_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV3` writer - Compare Channel 3 Buffer Valid"]
pub type CCBUFV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV4` reader - Compare Channel 4 Buffer Valid"]
pub type CCBUFV4_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV4` writer - Compare Channel 4 Buffer Valid"]
pub type CCBUFV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV5` reader - Compare Channel 5 Buffer Valid"]
pub type CCBUFV5_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV5` writer - Compare Channel 5 Buffer Valid"]
pub type CCBUFV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMP0` reader - Compare Channel 0 Value"]
pub type CMP0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0` writer - Compare Channel 0 Value"]
pub type CMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMP1` reader - Compare Channel 1 Value"]
pub type CMP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1` writer - Compare Channel 1 Value"]
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMP2` reader - Compare Channel 2 Value"]
pub type CMP2_R = crate::BitReader<bool>;
#[doc = "Field `CMP2` writer - Compare Channel 2 Value"]
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMP3` reader - Compare Channel 3 Value"]
pub type CMP3_R = crate::BitReader<bool>;
#[doc = "Field `CMP3` writer - Compare Channel 3 Value"]
pub type CMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMP4` reader - Compare Channel 4 Value"]
pub type CMP4_R = crate::BitReader<bool>;
#[doc = "Field `CMP4` writer - Compare Channel 4 Value"]
pub type CMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMP5` reader - Compare Channel 5 Value"]
pub type CMP5_R = crate::BitReader<bool>;
#[doc = "Field `CMP5` writer - Compare Channel 5 Value"]
pub type CMP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
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
    #[doc = "Bit 2 - Non-recoverable Update Fault State"]
    #[inline(always)]
    pub fn ufs(&self) -> UFS_R {
        UFS_R::new(((self.bits >> 2) & 1) != 0)
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
    pub fn pattbufv(&self) -> PATTBUFV_R {
        PATTBUFV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbufv(&self) -> PERBUFV_R {
        PERBUFV_R::new(((self.bits >> 7) & 1) != 0)
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
    pub fn ccbufv0(&self) -> CCBUFV0_R {
        CCBUFV0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> CCBUFV1_R {
        CCBUFV1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv2(&self) -> CCBUFV2_R {
        CCBUFV2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv3(&self) -> CCBUFV3_R {
        CCBUFV3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compare Channel 4 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv4(&self) -> CCBUFV4_R {
        CCBUFV4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Compare Channel 5 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv5(&self) -> CCBUFV5_R {
        CCBUFV5_R::new(((self.bits >> 21) & 1) != 0)
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
    #[doc = "Bit 28 - Compare Channel 4 Value"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Compare Channel 5 Value"]
    #[inline(always)]
    pub fn cmp5(&self) -> CMP5_R {
        CMP5_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<0> {
        STOP_W::new(self)
    }
    #[doc = "Bit 1 - Ramp"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IDX_W<1> {
        IDX_W::new(self)
    }
    #[doc = "Bit 2 - Non-recoverable Update Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn ufs(&mut self) -> UFS_W<2> {
        UFS_W::new(self)
    }
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DFS_W<3> {
        DFS_W::new(self)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SLAVE_W<4> {
        SLAVE_W::new(self)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn pattbufv(&mut self) -> PATTBUFV_W<5> {
        PATTBUFV_W::new(self)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn perbufv(&mut self) -> PERBUFV_W<7> {
        PERBUFV_W::new(self)
    }
    #[doc = "Bit 8 - Recoverable Fault A Input"]
    #[inline(always)]
    #[must_use]
    pub fn faultain(&mut self) -> FAULTAIN_W<8> {
        FAULTAIN_W::new(self)
    }
    #[doc = "Bit 9 - Recoverable Fault B Input"]
    #[inline(always)]
    #[must_use]
    pub fn faultbin(&mut self) -> FAULTBIN_W<9> {
        FAULTBIN_W::new(self)
    }
    #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
    #[inline(always)]
    #[must_use]
    pub fn fault0in(&mut self) -> FAULT0IN_W<10> {
        FAULT0IN_W::new(self)
    }
    #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
    #[inline(always)]
    #[must_use]
    pub fn fault1in(&mut self) -> FAULT1IN_W<11> {
        FAULT1IN_W::new(self)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    #[must_use]
    pub fn faulta(&mut self) -> FAULTA_W<12> {
        FAULTA_W::new(self)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    #[must_use]
    pub fn faultb(&mut self) -> FAULTB_W<13> {
        FAULTB_W::new(self)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    #[must_use]
    pub fn fault0(&mut self) -> FAULT0_W<14> {
        FAULT0_W::new(self)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<15> {
        FAULT1_W::new(self)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv0(&mut self) -> CCBUFV0_W<16> {
        CCBUFV0_W::new(self)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W<17> {
        CCBUFV1_W::new(self)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv2(&mut self) -> CCBUFV2_W<18> {
        CCBUFV2_W::new(self)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv3(&mut self) -> CCBUFV3_W<19> {
        CCBUFV3_W::new(self)
    }
    #[doc = "Bit 20 - Compare Channel 4 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv4(&mut self) -> CCBUFV4_W<20> {
        CCBUFV4_W::new(self)
    }
    #[doc = "Bit 21 - Compare Channel 5 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv5(&mut self) -> CCBUFV5_W<21> {
        CCBUFV5_W::new(self)
    }
    #[doc = "Bit 24 - Compare Channel 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> CMP0_W<24> {
        CMP0_W::new(self)
    }
    #[doc = "Bit 25 - Compare Channel 1 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<25> {
        CMP1_W::new(self)
    }
    #[doc = "Bit 26 - Compare Channel 2 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<26> {
        CMP2_W::new(self)
    }
    #[doc = "Bit 27 - Compare Channel 3 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> CMP3_W<27> {
        CMP3_W::new(self)
    }
    #[doc = "Bit 28 - Compare Channel 4 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<28> {
        CMP4_W::new(self)
    }
    #[doc = "Bit 29 - Compare Channel 5 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5(&mut self) -> CMP5_W<29> {
        CMP5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
