#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<CMDSELECT_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
    #[doc = "2: Erase Row - Erases the row addressed by the ADDR register."]
    ER = 2,
    #[doc = "4: Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    WP = 4,
    #[doc = "5: Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    EAR = 5,
    #[doc = "6: Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    WAP = 6,
    #[doc = "10: Security Flow Command"]
    SF = 10,
    #[doc = "15: Write lockbits"]
    WL = 15,
    #[doc = "64: Lock Region - Locks the region containing the address location in the ADDR register."]
    LR = 64,
    #[doc = "65: Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    UR = 65,
    #[doc = "66: Sets the power reduction mode."]
    SPRM = 66,
    #[doc = "67: Clears the power reduction mode."]
    CPRM = 67,
    #[doc = "68: Page Buffer Clear - Clears the page buffer."]
    PBC = 68,
    #[doc = "69: Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    SSB = 69,
    #[doc = "70: Invalidates all cache lines."]
    INVALL = 70,
}
impl From<CMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDSELECT_A {
    type Ux = u8;
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            2 => Some(CMDSELECT_A::ER),
            4 => Some(CMDSELECT_A::WP),
            5 => Some(CMDSELECT_A::EAR),
            6 => Some(CMDSELECT_A::WAP),
            10 => Some(CMDSELECT_A::SF),
            15 => Some(CMDSELECT_A::WL),
            64 => Some(CMDSELECT_A::LR),
            65 => Some(CMDSELECT_A::UR),
            66 => Some(CMDSELECT_A::SPRM),
            67 => Some(CMDSELECT_A::CPRM),
            68 => Some(CMDSELECT_A::PBC),
            69 => Some(CMDSELECT_A::SSB),
            70 => Some(CMDSELECT_A::INVALL),
            _ => None,
        }
    }
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline(always)]
    pub fn is_er(&self) -> bool {
        *self == CMDSELECT_A::ER
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        *self == CMDSELECT_A::WP
    }
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn is_ear(&self) -> bool {
        *self == CMDSELECT_A::EAR
    }
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn is_wap(&self) -> bool {
        *self == CMDSELECT_A::WAP
    }
    #[doc = "Security Flow Command"]
    #[inline(always)]
    pub fn is_sf(&self) -> bool {
        *self == CMDSELECT_A::SF
    }
    #[doc = "Write lockbits"]
    #[inline(always)]
    pub fn is_wl(&self) -> bool {
        *self == CMDSELECT_A::WL
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn is_lr(&self) -> bool {
        *self == CMDSELECT_A::LR
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn is_ur(&self) -> bool {
        *self == CMDSELECT_A::UR
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn is_sprm(&self) -> bool {
        *self == CMDSELECT_A::SPRM
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn is_cprm(&self) -> bool {
        *self == CMDSELECT_A::CPRM
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn is_pbc(&self) -> bool {
        *self == CMDSELECT_A::PBC
    }
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    #[inline(always)]
    pub fn is_ssb(&self) -> bool {
        *self == CMDSELECT_A::SSB
    }
    #[doc = "Invalidates all cache lines."]
    #[inline(always)]
    pub fn is_invall(&self) -> bool {
        *self == CMDSELECT_A::INVALL
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, CMDSELECT_A>;
impl<'a, REG, const O: u8> CMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline(always)]
    pub fn er(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::ER)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline(always)]
    pub fn wp(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::WP)
    }
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn ear(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::EAR)
    }
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn wap(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::WAP)
    }
    #[doc = "Security Flow Command"]
    #[inline(always)]
    pub fn sf(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::SF)
    }
    #[doc = "Write lockbits"]
    #[inline(always)]
    pub fn wl(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::WL)
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn lr(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::LR)
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn ur(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::UR)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn sprm(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn cprm(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::PBC)
    }
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::SSB)
    }
    #[doc = "Invalidates all cache lines."]
    #[inline(always)]
    pub fn invall(self) -> &'a mut crate::W<REG> {
        self.variant(CMDSELECT_A::INVALL)
    }
}
#[doc = "Field `CMDEX` reader - Command Execution"]
pub type CMDEX_R = crate::FieldReader<CMDEXSELECT_A>;
#[doc = "Command Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDEXSELECT_A {
    #[doc = "165: Execution Key"]
    KEY = 165,
}
impl From<CMDEXSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDEXSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDEXSELECT_A {
    type Ux = u8;
}
impl CMDEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDEXSELECT_A> {
        match self.bits {
            165 => Some(CMDEXSELECT_A::KEY),
            _ => None,
        }
    }
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == CMDEXSELECT_A::KEY
    }
}
#[doc = "Field `CMDEX` writer - Command Execution"]
pub type CMDEX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CMDEXSELECT_A>;
impl<'a, REG, const O: u8> CMDEX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(CMDEXSELECT_A::KEY)
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
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CTRLA_SPEC, 0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    #[must_use]
    pub fn cmdex(&mut self) -> CMDEX_W<CTRLA_SPEC, 8> {
        CMDEX_W::new(self)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
