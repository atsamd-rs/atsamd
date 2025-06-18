#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "0: Erase Page - Only supported in the USER and AUX pages."]
    Ep = 0,
    #[doc = "1: Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    Eb = 1,
    #[doc = "3: Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    Wp = 3,
    #[doc = "4: Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    Wqw = 4,
    #[doc = "16: Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    Swrst = 16,
    #[doc = "17: Lock Region - Locks the region containing the address location in the ADDR register."]
    Lr = 17,
    #[doc = "18: Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    Ur = 18,
    #[doc = "19: Sets the power reduction mode."]
    Sprm = 19,
    #[doc = "20: Clears the power reduction mode."]
    Cprm = 20,
    #[doc = "21: Page Buffer Clear - Clears the page buffer."]
    Pbc = 21,
    #[doc = "22: Set Security Bit"]
    Ssb = 22,
    #[doc = "23: Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    Bkswrst = 23,
    #[doc = "24: Chip Erase Lock - DSU.CE command is not available"]
    Celck = 24,
    #[doc = "25: Chip Erase Unlock - DSU.CE command is available"]
    Ceulck = 25,
    #[doc = "26: Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    Sbpdis = 26,
    #[doc = "27: Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    Cbpdis = 27,
    #[doc = "48: Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    Asees0 = 48,
    #[doc = "49: Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    Asees1 = 49,
    #[doc = "50: Starts SmartEEPROM sector reallocation algorithm"]
    Seeraloc = 50,
    #[doc = "51: Flush SMEE data when in buffered mode"]
    Seeflush = 51,
    #[doc = "52: Lock access to SmartEEPROM data from any mean"]
    Lsee = 52,
    #[doc = "53: Unlock access to SmartEEPROM data"]
    Usee = 53,
    #[doc = "54: Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    Lseer = 54,
    #[doc = "55: Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    Useer = 55,
}
impl From<Cmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 7, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase Page - Only supported in the USER and AUX pages."]
    #[inline(always)]
    pub fn ep(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ep)
    }
    #[doc = "Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    #[inline(always)]
    pub fn eb(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Eb)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wp)
    }
    #[doc = "Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    #[inline(always)]
    pub fn wqw(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wqw)
    }
    #[doc = "Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    #[inline(always)]
    pub fn swrst(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Swrst)
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn lr(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Lr)
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn ur(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ur)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn sprm(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Sprm)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn cprm(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Cprm)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Pbc)
    }
    #[doc = "Set Security Bit"]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ssb)
    }
    #[doc = "Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    #[inline(always)]
    pub fn bkswrst(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Bkswrst)
    }
    #[doc = "Chip Erase Lock - DSU.CE command is not available"]
    #[inline(always)]
    pub fn celck(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Celck)
    }
    #[doc = "Chip Erase Unlock - DSU.CE command is available"]
    #[inline(always)]
    pub fn ceulck(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ceulck)
    }
    #[doc = "Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    #[inline(always)]
    pub fn sbpdis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Sbpdis)
    }
    #[doc = "Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    #[inline(always)]
    pub fn cbpdis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Cbpdis)
    }
    #[doc = "Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    #[inline(always)]
    pub fn asees0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Asees0)
    }
    #[doc = "Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    #[inline(always)]
    pub fn asees1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Asees1)
    }
    #[doc = "Starts SmartEEPROM sector reallocation algorithm"]
    #[inline(always)]
    pub fn seeraloc(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Seeraloc)
    }
    #[doc = "Flush SMEE data when in buffered mode"]
    #[inline(always)]
    pub fn seeflush(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Seeflush)
    }
    #[doc = "Lock access to SmartEEPROM data from any mean"]
    #[inline(always)]
    pub fn lsee(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Lsee)
    }
    #[doc = "Unlock access to SmartEEPROM data"]
    #[inline(always)]
    pub fn usee(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Usee)
    }
    #[doc = "Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline(always)]
    pub fn lseer(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Lseer)
    }
    #[doc = "Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline(always)]
    pub fn useer(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Useer)
    }
}
#[doc = "Command Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdexselect {
    #[doc = "165: Execution Key"]
    Key = 165,
}
impl From<Cmdexselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdexselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdexselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdexselect {}
#[doc = "Field `CMDEX` writer - Command Execution"]
pub type CmdexW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cmdexselect>;
impl<'a, REG> CmdexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdexselect::Key)
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<CtrlbSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&mut self) -> CmdexW<CtrlbSpec> {
        CmdexW::new(self, 8)
    }
}
#[doc = "Control B\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
