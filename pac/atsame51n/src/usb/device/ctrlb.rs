#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `DETACH` reader - Detach"]
pub type DETACH_R = crate::BitReader;
#[doc = "Field `DETACH` writer - Detach"]
pub type DETACH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPRSM` reader - Upstream Resume"]
pub type UPRSM_R = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume"]
pub type UPRSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPDCONF` reader - Speed Configuration"]
pub type SPDCONF_R = crate::FieldReader<SPDCONFSELECT_A>;
#[doc = "Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDCONFSELECT_A {
    #[doc = "0: FS : Full Speed"]
    FS = 0,
    #[doc = "1: LS : Low Speed"]
    LS = 1,
    #[doc = "2: HS : High Speed capable"]
    HS = 2,
    #[doc = "3: HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    HSTM = 3,
}
impl From<SPDCONFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONFSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPDCONFSELECT_A {
    type Ux = u8;
}
impl SPDCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPDCONFSELECT_A {
        match self.bits {
            0 => SPDCONFSELECT_A::FS,
            1 => SPDCONFSELECT_A::LS,
            2 => SPDCONFSELECT_A::HS,
            3 => SPDCONFSELECT_A::HSTM,
            _ => unreachable!(),
        }
    }
    #[doc = "FS : Full Speed"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == SPDCONFSELECT_A::FS
    }
    #[doc = "LS : Low Speed"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == SPDCONFSELECT_A::LS
    }
    #[doc = "HS : High Speed capable"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == SPDCONFSELECT_A::HS
    }
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    #[inline(always)]
    pub fn is_hstm(&self) -> bool {
        *self == SPDCONFSELECT_A::HSTM
    }
}
#[doc = "Field `SPDCONF` writer - Speed Configuration"]
pub type SPDCONF_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SPDCONFSELECT_A>;
impl<'a, REG, const O: u8> SPDCONF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FS : Full Speed"]
    #[inline(always)]
    pub fn fs(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::FS)
    }
    #[doc = "LS : Low Speed"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::LS)
    }
    #[doc = "HS : High Speed capable"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::HS)
    }
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    #[inline(always)]
    pub fn hstm(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::HSTM)
    }
}
#[doc = "Field `NREPLY` reader - No Reply"]
pub type NREPLY_R = crate::BitReader;
#[doc = "Field `NREPLY` writer - No Reply"]
pub type NREPLY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TSTJ_R = crate::BitReader;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TSTJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TSTK_R = crate::BitReader;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TSTK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSTPCKT` reader - Test packet mode"]
pub type TSTPCKT_R = crate::BitReader;
#[doc = "Field `TSTPCKT` writer - Test packet mode"]
pub type TSTPCKT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPMODE2` reader - Specific Operational Mode"]
pub type OPMODE2_R = crate::BitReader;
#[doc = "Field `OPMODE2` writer - Specific Operational Mode"]
pub type OPMODE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GNAK` reader - Global NAK"]
pub type GNAK_R = crate::BitReader;
#[doc = "Field `GNAK` writer - Global NAK"]
pub type GNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMHDSK` reader - Link Power Management Handshake"]
pub type LPMHDSK_R = crate::FieldReader<LPMHDSKSELECT_A>;
#[doc = "Link Power Management Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMHDSKSELECT_A {
    #[doc = "0: No handshake. LPM is not supported"]
    NO = 0,
    #[doc = "1: ACK"]
    ACK = 1,
    #[doc = "2: NYET"]
    NYET = 2,
    #[doc = "3: STALL"]
    STALL = 3,
}
impl From<LPMHDSKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMHDSKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPMHDSKSELECT_A {
    type Ux = u8;
}
impl LPMHDSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPMHDSKSELECT_A {
        match self.bits {
            0 => LPMHDSKSELECT_A::NO,
            1 => LPMHDSKSELECT_A::ACK,
            2 => LPMHDSKSELECT_A::NYET,
            3 => LPMHDSKSELECT_A::STALL,
            _ => unreachable!(),
        }
    }
    #[doc = "No handshake. LPM is not supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPMHDSKSELECT_A::NO
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LPMHDSKSELECT_A::ACK
    }
    #[doc = "NYET"]
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == LPMHDSKSELECT_A::NYET
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == LPMHDSKSELECT_A::STALL
    }
}
#[doc = "Field `LPMHDSK` writer - Link Power Management Handshake"]
pub type LPMHDSK_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LPMHDSKSELECT_A>;
impl<'a, REG, const O: u8> LPMHDSK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No handshake. LPM is not supported"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(LPMHDSKSELECT_A::NO)
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(LPMHDSKSELECT_A::ACK)
    }
    #[doc = "NYET"]
    #[inline(always)]
    pub fn nyet(self) -> &'a mut crate::W<REG> {
        self.variant(LPMHDSKSELECT_A::NYET)
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(LPMHDSKSELECT_A::STALL)
    }
}
impl R {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    pub fn nreply(&self) -> NREPLY_R {
        NREPLY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TSTPCKT_R {
        TSTPCKT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> OPMODE2_R {
        OPMODE2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    pub fn gnak(&self) -> GNAK_R {
        GNAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    pub fn lpmhdsk(&self) -> LPMHDSK_R {
        LPMHDSK_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    #[must_use]
    pub fn detach(&mut self) -> DETACH_W<CTRLB_SPEC, 0> {
        DETACH_W::new(self)
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UPRSM_W<CTRLB_SPEC, 1> {
        UPRSM_W::new(self)
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SPDCONF_W<CTRLB_SPEC, 2> {
        SPDCONF_W::new(self)
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    #[must_use]
    pub fn nreply(&mut self) -> NREPLY_W<CTRLB_SPEC, 4> {
        NREPLY_W::new(self)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    #[must_use]
    pub fn tstj(&mut self) -> TSTJ_W<CTRLB_SPEC, 5> {
        TSTJ_W::new(self)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    #[must_use]
    pub fn tstk(&mut self) -> TSTK_W<CTRLB_SPEC, 6> {
        TSTK_W::new(self)
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    #[must_use]
    pub fn tstpckt(&mut self) -> TSTPCKT_W<CTRLB_SPEC, 7> {
        TSTPCKT_W::new(self)
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode2(&mut self) -> OPMODE2_W<CTRLB_SPEC, 8> {
        OPMODE2_W::new(self)
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    #[must_use]
    pub fn gnak(&mut self) -> GNAK_W<CTRLB_SPEC, 9> {
        GNAK_W::new(self)
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn lpmhdsk(&mut self) -> LPMHDSK_W<CTRLB_SPEC, 10> {
        LPMHDSK_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DEVICE Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0x01"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
