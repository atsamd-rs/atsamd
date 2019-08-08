#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLA {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    ER,
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    WP,
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    EAR,
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    WAP,
    #[doc = "Security Flow Command"]
    SF,
    #[doc = "Write lockbits"]
    WL,
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
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    SSB,
    #[doc = "Invalidates all cache lines."]
    INVALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::ER => 2,
            CMDR::WP => 4,
            CMDR::EAR => 5,
            CMDR::WAP => 6,
            CMDR::SF => 10,
            CMDR::WL => 15,
            CMDR::LR => 64,
            CMDR::UR => 65,
            CMDR::SPRM => 66,
            CMDR::CPRM => 67,
            CMDR::PBC => 68,
            CMDR::SSB => 69,
            CMDR::INVALL => 70,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            2 => CMDR::ER,
            4 => CMDR::WP,
            5 => CMDR::EAR,
            6 => CMDR::WAP,
            10 => CMDR::SF,
            15 => CMDR::WL,
            64 => CMDR::LR,
            65 => CMDR::UR,
            66 => CMDR::SPRM,
            67 => CMDR::CPRM,
            68 => CMDR::PBC,
            69 => CMDR::SSB,
            70 => CMDR::INVALL,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ER`"]
    #[inline]
    pub fn is_er(&self) -> bool {
        *self == CMDR::ER
    }
    #[doc = "Checks if the value of the field is `WP`"]
    #[inline]
    pub fn is_wp(&self) -> bool {
        *self == CMDR::WP
    }
    #[doc = "Checks if the value of the field is `EAR`"]
    #[inline]
    pub fn is_ear(&self) -> bool {
        *self == CMDR::EAR
    }
    #[doc = "Checks if the value of the field is `WAP`"]
    #[inline]
    pub fn is_wap(&self) -> bool {
        *self == CMDR::WAP
    }
    #[doc = "Checks if the value of the field is `SF`"]
    #[inline]
    pub fn is_sf(&self) -> bool {
        *self == CMDR::SF
    }
    #[doc = "Checks if the value of the field is `WL`"]
    #[inline]
    pub fn is_wl(&self) -> bool {
        *self == CMDR::WL
    }
    #[doc = "Checks if the value of the field is `LR`"]
    #[inline]
    pub fn is_lr(&self) -> bool {
        *self == CMDR::LR
    }
    #[doc = "Checks if the value of the field is `UR`"]
    #[inline]
    pub fn is_ur(&self) -> bool {
        *self == CMDR::UR
    }
    #[doc = "Checks if the value of the field is `SPRM`"]
    #[inline]
    pub fn is_sprm(&self) -> bool {
        *self == CMDR::SPRM
    }
    #[doc = "Checks if the value of the field is `CPRM`"]
    #[inline]
    pub fn is_cprm(&self) -> bool {
        *self == CMDR::CPRM
    }
    #[doc = "Checks if the value of the field is `PBC`"]
    #[inline]
    pub fn is_pbc(&self) -> bool {
        *self == CMDR::PBC
    }
    #[doc = "Checks if the value of the field is `SSB`"]
    #[inline]
    pub fn is_ssb(&self) -> bool {
        *self == CMDR::SSB
    }
    #[doc = "Checks if the value of the field is `INVALL`"]
    #[inline]
    pub fn is_invall(&self) -> bool {
        *self == CMDR::INVALL
    }
}
#[doc = "Possible values of the field `CMDEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDEXR {
    #[doc = "Execution Key"]
    KEY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDEXR::KEY => 165,
            CMDEXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDEXR {
        match value {
            165 => CMDEXR::KEY,
            i => CMDEXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline]
    pub fn is_key(&self) -> bool {
        *self == CMDEXR::KEY
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    ER,
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    WP,
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    EAR,
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    WAP,
    #[doc = "Security Flow Command"]
    SF,
    #[doc = "Write lockbits"]
    WL,
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
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    SSB,
    #[doc = "Invalidates all cache lines."]
    INVALL,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::ER => 2,
            CMDW::WP => 4,
            CMDW::EAR => 5,
            CMDW::WAP => 6,
            CMDW::SF => 10,
            CMDW::WL => 15,
            CMDW::LR => 64,
            CMDW::UR => 65,
            CMDW::SPRM => 66,
            CMDW::CPRM => 67,
            CMDW::PBC => 68,
            CMDW::SSB => 69,
            CMDW::INVALL => 70,
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
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline]
    pub fn er(self) -> &'a mut W {
        self.variant(CMDW::ER)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMDW::WP)
    }
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline]
    pub fn ear(self) -> &'a mut W {
        self.variant(CMDW::EAR)
    }
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline]
    pub fn wap(self) -> &'a mut W {
        self.variant(CMDW::WAP)
    }
    #[doc = "Security Flow Command"]
    #[inline]
    pub fn sf(self) -> &'a mut W {
        self.variant(CMDW::SF)
    }
    #[doc = "Write lockbits"]
    #[inline]
    pub fn wl(self) -> &'a mut W {
        self.variant(CMDW::WL)
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
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    #[inline]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMDW::SSB)
    }
    #[doc = "Invalidates all cache lines."]
    #[inline]
    pub fn invall(self) -> &'a mut W {
        self.variant(CMDW::INVALL)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:6 - Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline]
    pub fn cmdex(&self) -> CMDEXR {
        CMDEXR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
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
