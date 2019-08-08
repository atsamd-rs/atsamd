#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLB {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Erase Page - Only supported in the USER and AUX pages."]
    EP,
    #[doc = "Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    EB,
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    WP,
    #[doc = "Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    WQW,
    #[doc = "Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    SWRST,
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    LR,
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    UR,
    #[doc = "Sets the power reduction mode."]
    SPRM,
    #[doc = "Clears the power reduction mode."]
    CPRM,
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    PBC,
    #[doc = "Set Security Bit"]
    SSB,
    #[doc = "Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    BKSWRST,
    #[doc = "Chip Erase Lock - DSU.CE command is not available"]
    CELCK,
    #[doc = "Chip Erase Unlock - DSU.CE command is available"]
    CEULCK,
    #[doc = "Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    SBPDIS,
    #[doc = "Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    CBPDIS,
    #[doc = "Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    ASEES0,
    #[doc = "Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    ASEES1,
    #[doc = "Starts SmartEEPROM sector reallocation algorithm"]
    SEERALOC,
    #[doc = "Flush SMEE data when in buffered mode"]
    SEEFLUSH,
    #[doc = "Lock access to SmartEEPROM data from any mean"]
    LSEE,
    #[doc = "Unlock access to SmartEEPROM data"]
    USEE,
    #[doc = "Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    LSEER,
    #[doc = "Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    USEER,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::EP => 0,
            CMDW::EB => 1,
            CMDW::WP => 3,
            CMDW::WQW => 4,
            CMDW::SWRST => 16,
            CMDW::LR => 17,
            CMDW::UR => 18,
            CMDW::SPRM => 19,
            CMDW::CPRM => 20,
            CMDW::PBC => 21,
            CMDW::SSB => 22,
            CMDW::BKSWRST => 23,
            CMDW::CELCK => 24,
            CMDW::CEULCK => 25,
            CMDW::SBPDIS => 26,
            CMDW::CBPDIS => 27,
            CMDW::ASEES0 => 48,
            CMDW::ASEES1 => 49,
            CMDW::SEERALOC => 50,
            CMDW::SEEFLUSH => 51,
            CMDW::LSEE => 52,
            CMDW::USEE => 53,
            CMDW::LSEER => 54,
            CMDW::USEER => 55,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Erase Page - Only supported in the USER and AUX pages."]
    #[inline]
    pub fn ep(self) -> &'a mut W {
        self.variant(CMDW::EP)
    }
    #[doc = "Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    #[inline]
    pub fn eb(self) -> &'a mut W {
        self.variant(CMDW::EB)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    #[inline]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMDW::WP)
    }
    #[doc = "Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    #[inline]
    pub fn wqw(self) -> &'a mut W {
        self.variant(CMDW::WQW)
    }
    #[doc = "Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    #[inline]
    pub fn swrst(self) -> &'a mut W {
        self.variant(CMDW::SWRST)
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline]
    pub fn lr(self) -> &'a mut W {
        self.variant(CMDW::LR)
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline]
    pub fn ur(self) -> &'a mut W {
        self.variant(CMDW::UR)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline]
    pub fn sprm(self) -> &'a mut W {
        self.variant(CMDW::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline]
    pub fn cprm(self) -> &'a mut W {
        self.variant(CMDW::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline]
    pub fn pbc(self) -> &'a mut W {
        self.variant(CMDW::PBC)
    }
    #[doc = "Set Security Bit"]
    #[inline]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMDW::SSB)
    }
    #[doc = "Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    #[inline]
    pub fn bkswrst(self) -> &'a mut W {
        self.variant(CMDW::BKSWRST)
    }
    #[doc = "Chip Erase Lock - DSU.CE command is not available"]
    #[inline]
    pub fn celck(self) -> &'a mut W {
        self.variant(CMDW::CELCK)
    }
    #[doc = "Chip Erase Unlock - DSU.CE command is available"]
    #[inline]
    pub fn ceulck(self) -> &'a mut W {
        self.variant(CMDW::CEULCK)
    }
    #[doc = "Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    #[inline]
    pub fn sbpdis(self) -> &'a mut W {
        self.variant(CMDW::SBPDIS)
    }
    #[doc = "Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    #[inline]
    pub fn cbpdis(self) -> &'a mut W {
        self.variant(CMDW::CBPDIS)
    }
    #[doc = "Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    #[inline]
    pub fn asees0(self) -> &'a mut W {
        self.variant(CMDW::ASEES0)
    }
    #[doc = "Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    #[inline]
    pub fn asees1(self) -> &'a mut W {
        self.variant(CMDW::ASEES1)
    }
    #[doc = "Starts SmartEEPROM sector reallocation algorithm"]
    #[inline]
    pub fn seeraloc(self) -> &'a mut W {
        self.variant(CMDW::SEERALOC)
    }
    #[doc = "Flush SMEE data when in buffered mode"]
    #[inline]
    pub fn seeflush(self) -> &'a mut W {
        self.variant(CMDW::SEEFLUSH)
    }
    #[doc = "Lock access to SmartEEPROM data from any mean"]
    #[inline]
    pub fn lsee(self) -> &'a mut W {
        self.variant(CMDW::LSEE)
    }
    #[doc = "Unlock access to SmartEEPROM data"]
    #[inline]
    pub fn usee(self) -> &'a mut W {
        self.variant(CMDW::USEE)
    }
    #[doc = "Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline]
    pub fn lseer(self) -> &'a mut W {
        self.variant(CMDW::LSEER)
    }
    #[doc = "Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline]
    pub fn useer(self) -> &'a mut W {
        self.variant(CMDW::USEER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDEX`"]
pub enum CMDEXW {
    #[doc = "Execution Key"]
    KEY,
}
impl CMDEXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDEXW::KEY => 165,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDEXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDEXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Execution Key"]
    #[inline]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEXW::KEY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline]
    pub fn cmdex(&mut self) -> _CMDEXW {
        _CMDEXW { w: self }
    }
}
