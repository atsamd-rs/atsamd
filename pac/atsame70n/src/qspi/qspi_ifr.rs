#[doc = "Register `QSPI_IFR` reader"]
pub struct R(crate::R<QSPI_IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_IFR` writer"]
pub struct W(crate::W<QSPI_IFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_IFR_SPEC>;
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
impl From<crate::W<QSPI_IFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_IFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Width of Instruction Code, Address, Option Code and Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WIDTH_A {
    #[doc = "0: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SINGLE_BIT_SPI = 0,
    #[doc = "1: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DUAL_OUTPUT = 1,
    #[doc = "2: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QUAD_OUTPUT = 2,
    #[doc = "3: Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_IO = 3,
    #[doc = "4: Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_IO = 4,
    #[doc = "5: Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_CMD = 5,
    #[doc = "6: Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_CMD = 6,
}
impl From<WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WIDTH` reader - Width of Instruction Code, Address, Option Code and Data"]
pub struct WIDTH_R(crate::FieldReader<u8, WIDTH_A>);
impl WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WIDTH_A> {
        match self.bits {
            0 => Some(WIDTH_A::SINGLE_BIT_SPI),
            1 => Some(WIDTH_A::DUAL_OUTPUT),
            2 => Some(WIDTH_A::QUAD_OUTPUT),
            3 => Some(WIDTH_A::DUAL_IO),
            4 => Some(WIDTH_A::QUAD_IO),
            5 => Some(WIDTH_A::DUAL_CMD),
            6 => Some(WIDTH_A::QUAD_CMD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT_SPI`"]
    #[inline(always)]
    pub fn is_single_bit_spi(&self) -> bool {
        **self == WIDTH_A::SINGLE_BIT_SPI
    }
    #[doc = "Checks if the value of the field is `DUAL_OUTPUT`"]
    #[inline(always)]
    pub fn is_dual_output(&self) -> bool {
        **self == WIDTH_A::DUAL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `QUAD_OUTPUT`"]
    #[inline(always)]
    pub fn is_quad_output(&self) -> bool {
        **self == WIDTH_A::QUAD_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        **self == WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        **self == WIDTH_A::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `DUAL_CMD`"]
    #[inline(always)]
    pub fn is_dual_cmd(&self) -> bool {
        **self == WIDTH_A::DUAL_CMD
    }
    #[doc = "Checks if the value of the field is `QUAD_CMD`"]
    #[inline(always)]
    pub fn is_quad_cmd(&self) -> bool {
        **self == WIDTH_A::QUAD_CMD
    }
}
impl core::ops::Deref for WIDTH_R {
    type Target = crate::FieldReader<u8, WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIDTH` writer - Width of Instruction Code, Address, Option Code and Data"]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn single_bit_spi(self) -> &'a mut W {
        self.variant(WIDTH_A::SINGLE_BIT_SPI)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_output(self) -> &'a mut W {
        self.variant(WIDTH_A::DUAL_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_output(self) -> &'a mut W {
        self.variant(WIDTH_A::QUAD_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(WIDTH_A::DUAL_IO)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(WIDTH_A::QUAD_IO)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_cmd(self) -> &'a mut W {
        self.variant(WIDTH_A::DUAL_CMD)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_cmd(self) -> &'a mut W {
        self.variant(WIDTH_A::QUAD_CMD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `INSTEN` reader - Instruction Enable"]
pub struct INSTEN_R(crate::FieldReader<bool, bool>);
impl INSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTEN` writer - Instruction Enable"]
pub struct INSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ADDREN` reader - Address Enable"]
pub struct ADDREN_R(crate::FieldReader<bool, bool>);
impl ADDREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDREN` writer - Address Enable"]
pub struct ADDREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OPTEN` reader - Option Enable"]
pub struct OPTEN_R(crate::FieldReader<bool, bool>);
impl OPTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTEN` writer - Option Enable"]
pub struct OPTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DATAEN` reader - Data Enable"]
pub struct DATAEN_R(crate::FieldReader<bool, bool>);
impl DATAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAEN` writer - Data Enable"]
pub struct DATAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Option Code Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPTL_A {
    #[doc = "0: The option code is 1 bit long."]
    OPTION_1BIT = 0,
    #[doc = "1: The option code is 2 bits long."]
    OPTION_2BIT = 1,
    #[doc = "2: The option code is 4 bits long."]
    OPTION_4BIT = 2,
    #[doc = "3: The option code is 8 bits long."]
    OPTION_8BIT = 3,
}
impl From<OPTL_A> for u8 {
    #[inline(always)]
    fn from(variant: OPTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPTL` reader - Option Code Length"]
pub struct OPTL_R(crate::FieldReader<u8, OPTL_A>);
impl OPTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPTL_A {
        match self.bits {
            0 => OPTL_A::OPTION_1BIT,
            1 => OPTL_A::OPTION_2BIT,
            2 => OPTL_A::OPTION_4BIT,
            3 => OPTL_A::OPTION_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPTION_1BIT`"]
    #[inline(always)]
    pub fn is_option_1bit(&self) -> bool {
        **self == OPTL_A::OPTION_1BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_2BIT`"]
    #[inline(always)]
    pub fn is_option_2bit(&self) -> bool {
        **self == OPTL_A::OPTION_2BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_4BIT`"]
    #[inline(always)]
    pub fn is_option_4bit(&self) -> bool {
        **self == OPTL_A::OPTION_4BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_8BIT`"]
    #[inline(always)]
    pub fn is_option_8bit(&self) -> bool {
        **self == OPTL_A::OPTION_8BIT
    }
}
impl core::ops::Deref for OPTL_R {
    type Target = crate::FieldReader<u8, OPTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTL` writer - Option Code Length"]
pub struct OPTL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPTL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The option code is 1 bit long."]
    #[inline(always)]
    pub fn option_1bit(self) -> &'a mut W {
        self.variant(OPTL_A::OPTION_1BIT)
    }
    #[doc = "The option code is 2 bits long."]
    #[inline(always)]
    pub fn option_2bit(self) -> &'a mut W {
        self.variant(OPTL_A::OPTION_2BIT)
    }
    #[doc = "The option code is 4 bits long."]
    #[inline(always)]
    pub fn option_4bit(self) -> &'a mut W {
        self.variant(OPTL_A::OPTION_4BIT)
    }
    #[doc = "The option code is 8 bits long."]
    #[inline(always)]
    pub fn option_8bit(self) -> &'a mut W {
        self.variant(OPTL_A::OPTION_8BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Address Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRL_A {
    #[doc = "0: The address is 24 bits long."]
    _24_BIT = 0,
    #[doc = "1: The address is 32 bits long."]
    _32_BIT = 1,
}
impl From<ADDRL_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRL` reader - Address Length"]
pub struct ADDRL_R(crate::FieldReader<bool, ADDRL_A>);
impl ADDRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRL_A {
        match self.bits {
            false => ADDRL_A::_24_BIT,
            true => ADDRL_A::_32_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        **self == ADDRL_A::_24_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        **self == ADDRL_A::_32_BIT
    }
}
impl core::ops::Deref for ADDRL_R {
    type Target = crate::FieldReader<bool, ADDRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRL` writer - Address Length"]
pub struct ADDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The address is 24 bits long."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(ADDRL_A::_24_BIT)
    }
    #[doc = "The address is 32 bits long."]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(ADDRL_A::_32_BIT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Data Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TFRTYP_A {
    #[doc = "0: Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    TRSFR_READ = 0,
    #[doc = "1: Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    TRSFR_READ_MEMORY = 1,
    #[doc = "2: Write transfer into the serial memory.Scrambling is not performed."]
    TRSFR_WRITE = 2,
    #[doc = "3: Write data transfer into the serial memory.If enabled, scrambling is performed."]
    TRSFR_WRITE_MEMORY = 3,
}
impl From<TFRTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: TFRTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TFRTYP` reader - Data Transfer Type"]
pub struct TFRTYP_R(crate::FieldReader<u8, TFRTYP_A>);
impl TFRTYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFRTYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFRTYP_A {
        match self.bits {
            0 => TFRTYP_A::TRSFR_READ,
            1 => TFRTYP_A::TRSFR_READ_MEMORY,
            2 => TFRTYP_A::TRSFR_WRITE,
            3 => TFRTYP_A::TRSFR_WRITE_MEMORY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRSFR_READ`"]
    #[inline(always)]
    pub fn is_trsfr_read(&self) -> bool {
        **self == TFRTYP_A::TRSFR_READ
    }
    #[doc = "Checks if the value of the field is `TRSFR_READ_MEMORY`"]
    #[inline(always)]
    pub fn is_trsfr_read_memory(&self) -> bool {
        **self == TFRTYP_A::TRSFR_READ_MEMORY
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE`"]
    #[inline(always)]
    pub fn is_trsfr_write(&self) -> bool {
        **self == TFRTYP_A::TRSFR_WRITE
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE_MEMORY`"]
    #[inline(always)]
    pub fn is_trsfr_write_memory(&self) -> bool {
        **self == TFRTYP_A::TRSFR_WRITE_MEMORY
    }
}
impl core::ops::Deref for TFRTYP_R {
    type Target = crate::FieldReader<u8, TFRTYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFRTYP` writer - Data Transfer Type"]
pub struct TFRTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> TFRTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFRTYP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    #[inline(always)]
    pub fn trsfr_read(self) -> &'a mut W {
        self.variant(TFRTYP_A::TRSFR_READ)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    #[inline(always)]
    pub fn trsfr_read_memory(self) -> &'a mut W {
        self.variant(TFRTYP_A::TRSFR_READ_MEMORY)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn trsfr_write(self) -> &'a mut W {
        self.variant(TFRTYP_A::TRSFR_WRITE)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn trsfr_write_memory(self) -> &'a mut W {
        self.variant(TFRTYP_A::TRSFR_WRITE_MEMORY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Continuous Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRM_A {
    #[doc = "0: The Continuous Read mode is disabled."]
    DISABLED = 0,
    #[doc = "1: The Continuous Read mode is enabled."]
    ENABLED = 1,
}
impl From<CRM_A> for bool {
    #[inline(always)]
    fn from(variant: CRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRM` reader - Continuous Read Mode"]
pub struct CRM_R(crate::FieldReader<bool, CRM_A>);
impl CRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRM_A {
        match self.bits {
            false => CRM_A::DISABLED,
            true => CRM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CRM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CRM_A::ENABLED
    }
}
impl core::ops::Deref for CRM_R {
    type Target = crate::FieldReader<bool, CRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRM` writer - Continuous Read Mode"]
pub struct CRM_W<'a> {
    w: &'a mut W,
}
impl<'a> CRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Continuous Read mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRM_A::DISABLED)
    }
    #[doc = "The Continuous Read mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRM_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `NBDUM` reader - Number Of Dummy Cycles"]
pub struct NBDUM_R(crate::FieldReader<u8, u8>);
impl NBDUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBDUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBDUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBDUM` writer - Number Of Dummy Cycles"]
pub struct NBDUM_W<'a> {
    w: &'a mut W,
}
impl<'a> NBDUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn insten(&self) -> INSTEN_R {
        INSTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn opten(&self) -> OPTEN_R {
        OPTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&self) -> DATAEN_R {
        DATAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optl(&self) -> OPTL_R {
        OPTL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrl(&self) -> ADDRL_R {
        ADDRL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtyp(&self) -> TFRTYP_R {
        TFRTYP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crm(&self) -> CRM_R {
        CRM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    pub fn nbdum(&self) -> NBDUM_R {
        NBDUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn insten(&mut self) -> INSTEN_W {
        INSTEN_W { w: self }
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&mut self) -> ADDREN_W {
        ADDREN_W { w: self }
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn opten(&mut self) -> OPTEN_W {
        OPTEN_W { w: self }
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&mut self) -> DATAEN_W {
        DATAEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optl(&mut self) -> OPTL_W {
        OPTL_W { w: self }
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrl(&mut self) -> ADDRL_W {
        ADDRL_W { w: self }
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtyp(&mut self) -> TFRTYP_W {
        TFRTYP_W { w: self }
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crm(&mut self) -> CRM_W {
        CRM_W { w: self }
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    pub fn nbdum(&mut self) -> NBDUM_W {
        NBDUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_ifr](index.html) module"]
pub struct QSPI_IFR_SPEC;
impl crate::RegisterSpec for QSPI_IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_ifr::R](R) reader structure"]
impl crate::Readable for QSPI_IFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_ifr::W](W) writer structure"]
impl crate::Writable for QSPI_IFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_IFR to value 0"]
impl crate::Resettable for QSPI_IFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
