#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "2: Erase Row - Erases the row addressed by the ADDR register."]
    Er = 2,
    #[doc = "4: Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    Wp = 4,
    #[doc = "5: Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    Ear = 5,
    #[doc = "6: Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    Wap = 6,
    #[doc = "10: Security Flow Command"]
    Sf = 10,
    #[doc = "15: Write lockbits"]
    Wl = 15,
    #[doc = "64: Lock Region - Locks the region containing the address location in the ADDR register."]
    Lr = 64,
    #[doc = "65: Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    Ur = 65,
    #[doc = "66: Sets the power reduction mode."]
    Sprm = 66,
    #[doc = "67: Clears the power reduction mode."]
    Cprm = 67,
    #[doc = "68: Page Buffer Clear - Clears the page buffer."]
    Pbc = 68,
    #[doc = "69: Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    Ssb = 69,
    #[doc = "70: Invalidates all cache lines."]
    Invall = 70,
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
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::FieldReader<Cmdselect>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdselect> {
        match self.bits {
            2 => Some(Cmdselect::Er),
            4 => Some(Cmdselect::Wp),
            5 => Some(Cmdselect::Ear),
            6 => Some(Cmdselect::Wap),
            10 => Some(Cmdselect::Sf),
            15 => Some(Cmdselect::Wl),
            64 => Some(Cmdselect::Lr),
            65 => Some(Cmdselect::Ur),
            66 => Some(Cmdselect::Sprm),
            67 => Some(Cmdselect::Cprm),
            68 => Some(Cmdselect::Pbc),
            69 => Some(Cmdselect::Ssb),
            70 => Some(Cmdselect::Invall),
            _ => None,
        }
    }
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline(always)]
    pub fn is_er(&self) -> bool {
        *self == Cmdselect::Er
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        *self == Cmdselect::Wp
    }
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn is_ear(&self) -> bool {
        *self == Cmdselect::Ear
    }
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn is_wap(&self) -> bool {
        *self == Cmdselect::Wap
    }
    #[doc = "Security Flow Command"]
    #[inline(always)]
    pub fn is_sf(&self) -> bool {
        *self == Cmdselect::Sf
    }
    #[doc = "Write lockbits"]
    #[inline(always)]
    pub fn is_wl(&self) -> bool {
        *self == Cmdselect::Wl
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn is_lr(&self) -> bool {
        *self == Cmdselect::Lr
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn is_ur(&self) -> bool {
        *self == Cmdselect::Ur
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn is_sprm(&self) -> bool {
        *self == Cmdselect::Sprm
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn is_cprm(&self) -> bool {
        *self == Cmdselect::Cprm
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn is_pbc(&self) -> bool {
        *self == Cmdselect::Pbc
    }
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    #[inline(always)]
    pub fn is_ssb(&self) -> bool {
        *self == Cmdselect::Ssb
    }
    #[doc = "Invalidates all cache lines."]
    #[inline(always)]
    pub fn is_invall(&self) -> bool {
        *self == Cmdselect::Invall
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 7, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline(always)]
    pub fn er(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Er)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline(always)]
    pub fn wp(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wp)
    }
    #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn ear(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ear)
    }
    #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
    #[inline(always)]
    pub fn wap(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wap)
    }
    #[doc = "Security Flow Command"]
    #[inline(always)]
    pub fn sf(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Sf)
    }
    #[doc = "Write lockbits"]
    #[inline(always)]
    pub fn wl(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wl)
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
    #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Ssb)
    }
    #[doc = "Invalidates all cache lines."]
    #[inline(always)]
    pub fn invall(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Invall)
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
#[doc = "Field `CMDEX` reader - Command Execution"]
pub type CmdexR = crate::FieldReader<Cmdexselect>;
impl CmdexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdexselect> {
        match self.bits {
            165 => Some(Cmdexselect::Key),
            _ => None,
        }
    }
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Cmdexselect::Key
    }
}
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
impl R {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&self) -> CmdexR {
        CmdexR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CtrlaSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    #[must_use]
    pub fn cmdex(&mut self) -> CmdexW<CtrlaSpec> {
        CmdexW::new(self, 8)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u16 = 0;
}
