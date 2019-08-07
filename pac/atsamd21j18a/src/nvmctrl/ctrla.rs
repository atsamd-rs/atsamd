#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u16, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u16, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_A {
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
impl crate::ToBits<u8> for CMD_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CMD_A::ER => 2,
            CMD_A::WP => 4,
            CMD_A::EAR => 5,
            CMD_A::WAP => 6,
            CMD_A::SF => 10,
            CMD_A::WL => 15,
            CMD_A::LR => 64,
            CMD_A::UR => 65,
            CMD_A::SPRM => 66,
            CMD_A::CPRM => 67,
            CMD_A::PBC => 68,
            CMD_A::SSB => 69,
            CMD_A::INVALL => 70,
        }
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, CMD_A>;
impl CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMD_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(CMD_A::ER),
            4 => Val(CMD_A::WP),
            5 => Val(CMD_A::EAR),
            6 => Val(CMD_A::WAP),
            10 => Val(CMD_A::SF),
            15 => Val(CMD_A::WL),
            64 => Val(CMD_A::LR),
            65 => Val(CMD_A::UR),
            66 => Val(CMD_A::SPRM),
            67 => Val(CMD_A::CPRM),
            68 => Val(CMD_A::PBC),
            69 => Val(CMD_A::SSB),
            70 => Val(CMD_A::INVALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ER`"]
    #[inline(always)]
    pub fn is_er(&self) -> bool {
        *self == CMD_A::ER
    }
    #[doc = "Checks if the value of the field is `WP`"]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        *self == CMD_A::WP
    }
    #[doc = "Checks if the value of the field is `EAR`"]
    #[inline(always)]
    pub fn is_ear(&self) -> bool {
        *self == CMD_A::EAR
    }
    #[doc = "Checks if the value of the field is `WAP`"]
    #[inline(always)]
    pub fn is_wap(&self) -> bool {
        *self == CMD_A::WAP
    }
    #[doc = "Checks if the value of the field is `SF`"]
    #[inline(always)]
    pub fn is_sf(&self) -> bool {
        *self == CMD_A::SF
    }
    #[doc = "Checks if the value of the field is `WL`"]
    #[inline(always)]
    pub fn is_wl(&self) -> bool {
        *self == CMD_A::WL
    }
    #[doc = "Checks if the value of the field is `LR`"]
    #[inline(always)]
    pub fn is_lr(&self) -> bool {
        *self == CMD_A::LR
    }
    #[doc = "Checks if the value of the field is `UR`"]
    #[inline(always)]
    pub fn is_ur(&self) -> bool {
        *self == CMD_A::UR
    }
    #[doc = "Checks if the value of the field is `SPRM`"]
    #[inline(always)]
    pub fn is_sprm(&self) -> bool {
        *self == CMD_A::SPRM
    }
    #[doc = "Checks if the value of the field is `CPRM`"]
    #[inline(always)]
    pub fn is_cprm(&self) -> bool {
        *self == CMD_A::CPRM
    }
    #[doc = "Checks if the value of the field is `PBC`"]
    #[inline(always)]
    pub fn is_pbc(&self) -> bool {
        *self == CMD_A::PBC
    }
    #[doc = "Checks if the value of the field is `SSB`"]
    #[inline(always)]
    pub fn is_ssb(&self) -> bool {
        *self == CMD_A::SSB
    }
    #[doc = "Checks if the value of the field is `INVALL`"]
    #[inline(always)]
    pub fn is_invall(&self) -> bool {
        *self == CMD_A::INVALL
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline(always)]
    pub fn er(self) -> &'a mut W {
        self.variant(CMD_A::ER)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMD_A::WP)
    }
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn ear(self) -> &'a mut W {
        self.variant(CMD_A::EAR)
    }
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn wap(self) -> &'a mut W {
        self.variant(CMD_A::WAP)
    }
    #[doc = "Security Flow Command"]
    #[inline(always)]
    pub fn sf(self) -> &'a mut W {
        self.variant(CMD_A::SF)
    }
    #[doc = "Write lockbits"]
    #[inline(always)]
    pub fn wl(self) -> &'a mut W {
        self.variant(CMD_A::WL)
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn lr(self) -> &'a mut W {
        self.variant(CMD_A::LR)
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn ur(self) -> &'a mut W {
        self.variant(CMD_A::UR)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn sprm(self) -> &'a mut W {
        self.variant(CMD_A::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn cprm(self) -> &'a mut W {
        self.variant(CMD_A::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut W {
        self.variant(CMD_A::PBC)
    }
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMD_A::SSB)
    }
    #[doc = "Invalidates all cache lines."]
    #[inline(always)]
    pub fn invall(self) -> &'a mut W {
        self.variant(CMD_A::INVALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Possible values of the field `CMDEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDEX_A {
    #[doc = "Execution Key"]
    KEY,
}
impl crate::ToBits<u8> for CMDEX_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CMDEX_A::KEY => 165,
        }
    }
}
#[doc = "Reader of field `CMDEX`"]
pub type CMDEX_R = crate::R<u8, CMDEX_A>;
impl CMDEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMDEX_A> {
        use crate::Variant::*;
        match self.bits {
            165 => Val(CMDEX_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == CMDEX_A::KEY
    }
}
#[doc = "Write proxy for field `CMDEX`"]
pub struct CMDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDEX_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEX_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&self) -> CMDEX_R {
        CMDEX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&mut self) -> CMDEX_W {
        CMDEX_W { w: self }
    }
}
