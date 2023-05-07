#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_AW {
    #[doc = "0: Erase Page - Only supported in the USER and AUX pages."]
    EP = 0,
    #[doc = "1: Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    EB = 1,
    #[doc = "3: Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    WP = 3,
    #[doc = "4: Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    WQW = 4,
    #[doc = "16: Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    SWRST = 16,
    #[doc = "17: Lock Region - Locks the region containing the address location in the ADDR register."]
    LR = 17,
    #[doc = "18: Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    UR = 18,
    #[doc = "19: Sets the power reduction mode."]
    SPRM = 19,
    #[doc = "20: Clears the power reduction mode."]
    CPRM = 20,
    #[doc = "21: Page Buffer Clear - Clears the page buffer."]
    PBC = 21,
    #[doc = "22: Set Security Bit"]
    SSB = 22,
    #[doc = "23: Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    BKSWRST = 23,
    #[doc = "24: Chip Erase Lock - DSU.CE command is not available"]
    CELCK = 24,
    #[doc = "25: Chip Erase Unlock - DSU.CE command is available"]
    CEULCK = 25,
    #[doc = "26: Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    SBPDIS = 26,
    #[doc = "27: Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    CBPDIS = 27,
    #[doc = "48: Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    ASEES0 = 48,
    #[doc = "49: Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    ASEES1 = 49,
    #[doc = "50: Starts SmartEEPROM sector reallocation algorithm"]
    SEERALOC = 50,
    #[doc = "51: Flush SMEE data when in buffered mode"]
    SEEFLUSH = 51,
    #[doc = "52: Lock access to SmartEEPROM data from any mean"]
    LSEE = 52,
    #[doc = "53: Unlock access to SmartEEPROM data"]
    USEE = 53,
    #[doc = "54: Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    LSEER = 54,
    #[doc = "55: Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    USEER = 55,
}
impl From<CMDSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLB_SPEC, u8, CMDSELECT_AW, 7, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Erase Page - Only supported in the USER and AUX pages."]
    #[inline(always)]
    pub fn ep(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::EP)
    }
    #[doc = "Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    #[inline(always)]
    pub fn eb(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::EB)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::WP)
    }
    #[doc = "Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    #[inline(always)]
    pub fn wqw(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::WQW)
    }
    #[doc = "Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    #[inline(always)]
    pub fn swrst(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SWRST)
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn lr(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::LR)
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn ur(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::UR)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn sprm(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn cprm(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::PBC)
    }
    #[doc = "Set Security Bit"]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SSB)
    }
    #[doc = "Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    #[inline(always)]
    pub fn bkswrst(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::BKSWRST)
    }
    #[doc = "Chip Erase Lock - DSU.CE command is not available"]
    #[inline(always)]
    pub fn celck(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::CELCK)
    }
    #[doc = "Chip Erase Unlock - DSU.CE command is available"]
    #[inline(always)]
    pub fn ceulck(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::CEULCK)
    }
    #[doc = "Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    #[inline(always)]
    pub fn sbpdis(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SBPDIS)
    }
    #[doc = "Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    #[inline(always)]
    pub fn cbpdis(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::CBPDIS)
    }
    #[doc = "Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    #[inline(always)]
    pub fn asees0(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::ASEES0)
    }
    #[doc = "Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    #[inline(always)]
    pub fn asees1(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::ASEES1)
    }
    #[doc = "Starts SmartEEPROM sector reallocation algorithm"]
    #[inline(always)]
    pub fn seeraloc(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SEERALOC)
    }
    #[doc = "Flush SMEE data when in buffered mode"]
    #[inline(always)]
    pub fn seeflush(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SEEFLUSH)
    }
    #[doc = "Lock access to SmartEEPROM data from any mean"]
    #[inline(always)]
    pub fn lsee(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::LSEE)
    }
    #[doc = "Unlock access to SmartEEPROM data"]
    #[inline(always)]
    pub fn usee(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::USEE)
    }
    #[doc = "Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline(always)]
    pub fn lseer(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::LSEER)
    }
    #[doc = "Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline(always)]
    pub fn useer(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::USEER)
    }
}
#[doc = "Command Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDEXSELECT_AW {
    #[doc = "165: Execution Key"]
    KEY = 165,
}
impl From<CMDEXSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMDEXSELECT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CMDEX` writer - Command Execution"]
pub type CMDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CTRLB_SPEC, u8, CMDEXSELECT_AW, 8, O>;
impl<'a, const O: u8> CMDEX_W<'a, O> {
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEXSELECT_AW::KEY)
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    #[must_use]
    pub fn cmdex(&mut self) -> CMDEX_W<8> {
        CMDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
