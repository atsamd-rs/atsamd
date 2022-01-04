#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
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
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - Command"]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            2 => Some(CMD_A::ER),
            4 => Some(CMD_A::WP),
            5 => Some(CMD_A::EAR),
            6 => Some(CMD_A::WAP),
            10 => Some(CMD_A::SF),
            15 => Some(CMD_A::WL),
            64 => Some(CMD_A::LR),
            65 => Some(CMD_A::UR),
            66 => Some(CMD_A::SPRM),
            67 => Some(CMD_A::CPRM),
            68 => Some(CMD_A::PBC),
            69 => Some(CMD_A::SSB),
            70 => Some(CMD_A::INVALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ER`"]
    #[inline(always)]
    pub fn is_er(&self) -> bool {
        **self == CMD_A::ER
    }
    #[doc = "Checks if the value of the field is `WP`"]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        **self == CMD_A::WP
    }
    #[doc = "Checks if the value of the field is `EAR`"]
    #[inline(always)]
    pub fn is_ear(&self) -> bool {
        **self == CMD_A::EAR
    }
    #[doc = "Checks if the value of the field is `WAP`"]
    #[inline(always)]
    pub fn is_wap(&self) -> bool {
        **self == CMD_A::WAP
    }
    #[doc = "Checks if the value of the field is `SF`"]
    #[inline(always)]
    pub fn is_sf(&self) -> bool {
        **self == CMD_A::SF
    }
    #[doc = "Checks if the value of the field is `WL`"]
    #[inline(always)]
    pub fn is_wl(&self) -> bool {
        **self == CMD_A::WL
    }
    #[doc = "Checks if the value of the field is `LR`"]
    #[inline(always)]
    pub fn is_lr(&self) -> bool {
        **self == CMD_A::LR
    }
    #[doc = "Checks if the value of the field is `UR`"]
    #[inline(always)]
    pub fn is_ur(&self) -> bool {
        **self == CMD_A::UR
    }
    #[doc = "Checks if the value of the field is `SPRM`"]
    #[inline(always)]
    pub fn is_sprm(&self) -> bool {
        **self == CMD_A::SPRM
    }
    #[doc = "Checks if the value of the field is `CPRM`"]
    #[inline(always)]
    pub fn is_cprm(&self) -> bool {
        **self == CMD_A::CPRM
    }
    #[doc = "Checks if the value of the field is `PBC`"]
    #[inline(always)]
    pub fn is_pbc(&self) -> bool {
        **self == CMD_A::PBC
    }
    #[doc = "Checks if the value of the field is `SSB`"]
    #[inline(always)]
    pub fn is_ssb(&self) -> bool {
        **self == CMD_A::SSB
    }
    #[doc = "Checks if the value of the field is `INVALL`"]
    #[inline(always)]
    pub fn is_invall(&self) -> bool {
        **self == CMD_A::INVALL
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Command"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
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
        self.w.bits = (self.w.bits & !0x7f) | (value as u16 & 0x7f);
        self.w
    }
}
#[doc = "Command Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDEX_A {
    #[doc = "165: Execution Key"]
    KEY = 165,
}
impl From<CMDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDEX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMDEX` reader - Command Execution"]
pub struct CMDEX_R(crate::FieldReader<u8, CMDEX_A>);
impl CMDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMDEX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDEX_A> {
        match self.bits {
            165 => Some(CMDEX_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        **self == CMDEX_A::KEY
    }
}
impl core::ops::Deref for CMDEX_R {
    type Target = crate::FieldReader<u8, CMDEX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDEX` writer - Command Execution"]
pub struct CMDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDEX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEX_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
