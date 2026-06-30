#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resptypselect {
    #[doc = "0: No response"]
    None = 0,
    #[doc = "1: 136-bit response"]
    _136Bit = 1,
    #[doc = "2: 48-bit response"]
    _48Bit = 2,
    #[doc = "3: 48-bit response check busy after response"]
    _48BitBusy = 3,
}
impl From<Resptypselect> for u8 {
    #[inline(always)]
    fn from(variant: Resptypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resptypselect {
    type Ux = u8;
}
impl crate::IsEnum for Resptypselect {}
#[doc = "Field `RESPTYP` reader - Response Type"]
pub type ResptypR = crate::FieldReader<Resptypselect>;
impl ResptypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resptypselect {
        match self.bits {
            0 => Resptypselect::None,
            1 => Resptypselect::_136Bit,
            2 => Resptypselect::_48Bit,
            3 => Resptypselect::_48BitBusy,
            _ => unreachable!(),
        }
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Resptypselect::None
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn is_136_bit(&self) -> bool {
        *self == Resptypselect::_136Bit
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn is_48_bit(&self) -> bool {
        *self == Resptypselect::_48Bit
    }
    #[doc = "48-bit response check busy after response"]
    #[inline(always)]
    pub fn is_48_bit_busy(&self) -> bool {
        *self == Resptypselect::_48BitBusy
    }
}
#[doc = "Field `RESPTYP` writer - Response Type"]
pub type ResptypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resptypselect, crate::Safe>;
impl<'a, REG> ResptypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No response"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypselect::None)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypselect::_136Bit)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypselect::_48Bit)
    }
    #[doc = "48-bit response check busy after response"]
    #[inline(always)]
    pub fn _48_bit_busy(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypselect::_48BitBusy)
    }
}
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdccenselect {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cmdccenselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdccenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCCEN` reader - Command CRC Check Enable"]
pub type CmdccenR = crate::BitReader<Cmdccenselect>;
impl CmdccenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdccenselect {
        match self.bits {
            false => Cmdccenselect::Disable,
            true => Cmdccenselect::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmdccenselect::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmdccenselect::Enable
    }
}
#[doc = "Field `CMDCCEN` writer - Command CRC Check Enable"]
pub type CmdccenW<'a, REG> = crate::BitWriter<'a, REG, Cmdccenselect>;
impl<'a, REG> CmdccenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdccenselect::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdccenselect::Enable)
    }
}
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdicenselect {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cmdicenselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdicenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDICEN` reader - Command Index Check Enable"]
pub type CmdicenR = crate::BitReader<Cmdicenselect>;
impl CmdicenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdicenselect {
        match self.bits {
            false => Cmdicenselect::Disable,
            true => Cmdicenselect::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmdicenselect::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmdicenselect::Enable
    }
}
#[doc = "Field `CMDICEN` writer - Command Index Check Enable"]
pub type CmdicenW<'a, REG> = crate::BitWriter<'a, REG, Cmdicenselect>;
impl<'a, REG> CmdicenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdicenselect::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdicenselect::Enable)
    }
}
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpselselect {
    #[doc = "0: No Data Present"]
    NoData = 0,
    #[doc = "1: Data Present"]
    Data = 1,
}
impl From<Dpselselect> for bool {
    #[inline(always)]
    fn from(variant: Dpselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPSEL` reader - Data Present Select"]
pub type DpselR = crate::BitReader<Dpselselect>;
impl DpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpselselect {
        match self.bits {
            false => Dpselselect::NoData,
            true => Dpselselect::Data,
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Dpselselect::NoData
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Dpselselect::Data
    }
}
#[doc = "Field `DPSEL` writer - Data Present Select"]
pub type DpselW<'a, REG> = crate::BitWriter<'a, REG, Dpselselect>;
impl<'a, REG> DpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Dpselselect::NoData)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Dpselselect::Data)
    }
}
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtypselect {
    #[doc = "0: Other commands"]
    Normal = 0,
    #[doc = "1: CMD52 for writing Bus Suspend in CCCR"]
    Suspend = 1,
    #[doc = "2: CMD52 for writing Function Select in CCCR"]
    Resume = 2,
    #[doc = "3: CMD12, CMD52 for writing I/O Abort in CCCR"]
    Abort = 3,
}
impl From<Cmdtypselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtypselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdtypselect {}
#[doc = "Field `CMDTYP` reader - Command Type"]
pub type CmdtypR = crate::FieldReader<Cmdtypselect>;
impl CmdtypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtypselect {
        match self.bits {
            0 => Cmdtypselect::Normal,
            1 => Cmdtypselect::Suspend,
            2 => Cmdtypselect::Resume,
            3 => Cmdtypselect::Abort,
            _ => unreachable!(),
        }
    }
    #[doc = "Other commands"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Cmdtypselect::Normal
    }
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Cmdtypselect::Suspend
    }
    #[doc = "CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Cmdtypselect::Resume
    }
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == Cmdtypselect::Abort
    }
}
#[doc = "Field `CMDTYP` writer - Command Type"]
pub type CmdtypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdtypselect, crate::Safe>;
impl<'a, REG> CmdtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Other commands"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Normal)
    }
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Suspend)
    }
    #[doc = "CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Resume)
    }
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Abort)
    }
}
#[doc = "Field `CMDIDX` reader - Command Index"]
pub type CmdidxR = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - Command Index"]
pub type CmdidxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Response Type"]
    #[inline(always)]
    pub fn resptyp(&self) -> ResptypR {
        ResptypR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdccen(&self) -> CmdccenR {
        CmdccenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdicen(&self) -> CmdicenR {
        CmdicenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DpselR {
        DpselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CmdtypR {
        CmdtypR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CmdidxR {
        CmdidxR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type"]
    #[inline(always)]
    #[must_use]
    pub fn resptyp(&mut self) -> ResptypW<CrSpec> {
        ResptypW::new(self, 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdccen(&mut self) -> CmdccenW<CrSpec> {
        CmdccenW::new(self, 3)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdicen(&mut self) -> CmdicenW<CrSpec> {
        CmdicenW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    #[must_use]
    pub fn dpsel(&mut self) -> DpselW<CrSpec> {
        DpselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtyp(&mut self) -> CmdtypW<CrSpec> {
        CmdtypW::new(self, 6)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CmdidxW<CrSpec> {
        CmdidxW::new(self, 8)
    }
}
#[doc = "Command\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u16 = 0;
}
