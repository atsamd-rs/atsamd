#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `DETACH` reader - Detach"]
pub type DetachR = crate::BitReader;
#[doc = "Field `DETACH` writer - Detach"]
pub type DetachW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSM` reader - Upstream Resume"]
pub type UprsmR = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume"]
pub type UprsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spdconfselect {
    #[doc = "0: FS : Full Speed"]
    Fs = 0,
    #[doc = "1: LS : Low Speed"]
    Ls = 1,
    #[doc = "2: HS : High Speed capable"]
    Hs = 2,
    #[doc = "3: HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    Hstm = 3,
}
impl From<Spdconfselect> for u8 {
    #[inline(always)]
    fn from(variant: Spdconfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spdconfselect {
    type Ux = u8;
}
impl crate::IsEnum for Spdconfselect {}
#[doc = "Field `SPDCONF` reader - Speed Configuration"]
pub type SpdconfR = crate::FieldReader<Spdconfselect>;
impl SpdconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spdconfselect {
        match self.bits {
            0 => Spdconfselect::Fs,
            1 => Spdconfselect::Ls,
            2 => Spdconfselect::Hs,
            3 => Spdconfselect::Hstm,
            _ => unreachable!(),
        }
    }
    #[doc = "FS : Full Speed"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == Spdconfselect::Fs
    }
    #[doc = "LS : Low Speed"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == Spdconfselect::Ls
    }
    #[doc = "HS : High Speed capable"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == Spdconfselect::Hs
    }
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    #[inline(always)]
    pub fn is_hstm(&self) -> bool {
        *self == Spdconfselect::Hstm
    }
}
#[doc = "Field `SPDCONF` writer - Speed Configuration"]
pub type SpdconfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spdconfselect, crate::Safe>;
impl<'a, REG> SpdconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FS : Full Speed"]
    #[inline(always)]
    pub fn fs(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::Fs)
    }
    #[doc = "LS : Low Speed"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::Ls)
    }
    #[doc = "HS : High Speed capable"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::Hs)
    }
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    #[inline(always)]
    pub fn hstm(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::Hstm)
    }
}
#[doc = "Field `NREPLY` reader - No Reply"]
pub type NreplyR = crate::BitReader;
#[doc = "Field `NREPLY` writer - No Reply"]
pub type NreplyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TstjR = crate::BitReader;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TstjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TstkR = crate::BitReader;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TstkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTPCKT` reader - Test packet mode"]
pub type TstpcktR = crate::BitReader;
#[doc = "Field `TSTPCKT` writer - Test packet mode"]
pub type TstpcktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPMODE2` reader - Specific Operational Mode"]
pub type Opmode2R = crate::BitReader;
#[doc = "Field `OPMODE2` writer - Specific Operational Mode"]
pub type Opmode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNAK` reader - Global NAK"]
pub type GnakR = crate::BitReader;
#[doc = "Field `GNAK` writer - Global NAK"]
pub type GnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Link Power Management Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpmhdskselect {
    #[doc = "0: No handshake. LPM is not supported"]
    No = 0,
    #[doc = "1: ACK"]
    Ack = 1,
    #[doc = "2: NYET"]
    Nyet = 2,
    #[doc = "3: STALL"]
    Stall = 3,
}
impl From<Lpmhdskselect> for u8 {
    #[inline(always)]
    fn from(variant: Lpmhdskselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpmhdskselect {
    type Ux = u8;
}
impl crate::IsEnum for Lpmhdskselect {}
#[doc = "Field `LPMHDSK` reader - Link Power Management Handshake"]
pub type LpmhdskR = crate::FieldReader<Lpmhdskselect>;
impl LpmhdskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpmhdskselect {
        match self.bits {
            0 => Lpmhdskselect::No,
            1 => Lpmhdskselect::Ack,
            2 => Lpmhdskselect::Nyet,
            3 => Lpmhdskselect::Stall,
            _ => unreachable!(),
        }
    }
    #[doc = "No handshake. LPM is not supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Lpmhdskselect::No
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Lpmhdskselect::Ack
    }
    #[doc = "NYET"]
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == Lpmhdskselect::Nyet
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == Lpmhdskselect::Stall
    }
}
#[doc = "Field `LPMHDSK` writer - Link Power Management Handshake"]
pub type LpmhdskW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpmhdskselect, crate::Safe>;
impl<'a, REG> LpmhdskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No handshake. LPM is not supported"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmhdskselect::No)
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmhdskselect::Ack)
    }
    #[doc = "NYET"]
    #[inline(always)]
    pub fn nyet(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmhdskselect::Nyet)
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmhdskselect::Stall)
    }
}
impl R {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DetachR {
        DetachR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&self) -> UprsmR {
        UprsmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SpdconfR {
        SpdconfR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    pub fn nreply(&self) -> NreplyR {
        NreplyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TstjR {
        TstjR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TstkR {
        TstkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TstpcktR {
        TstpcktR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> Opmode2R {
        Opmode2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    pub fn gnak(&self) -> GnakR {
        GnakR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    pub fn lpmhdsk(&self) -> LpmhdskR {
        LpmhdskR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    #[must_use]
    pub fn detach(&mut self) -> DetachW<CtrlbSpec> {
        DetachW::new(self, 0)
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UprsmW<CtrlbSpec> {
        UprsmW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SpdconfW<CtrlbSpec> {
        SpdconfW::new(self, 2)
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    #[must_use]
    pub fn nreply(&mut self) -> NreplyW<CtrlbSpec> {
        NreplyW::new(self, 4)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    #[must_use]
    pub fn tstj(&mut self) -> TstjW<CtrlbSpec> {
        TstjW::new(self, 5)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    #[must_use]
    pub fn tstk(&mut self) -> TstkW<CtrlbSpec> {
        TstkW::new(self, 6)
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    #[must_use]
    pub fn tstpckt(&mut self) -> TstpcktW<CtrlbSpec> {
        TstpcktW::new(self, 7)
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode2(&mut self) -> Opmode2W<CtrlbSpec> {
        Opmode2W::new(self, 8)
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    #[must_use]
    pub fn gnak(&mut self) -> GnakW<CtrlbSpec> {
        GnakW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn lpmhdsk(&mut self) -> LpmhdskW<CtrlbSpec> {
        LpmhdskW::new(self, 10)
    }
}
#[doc = "DEVICE Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0x01"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u16 = 0x01;
}
