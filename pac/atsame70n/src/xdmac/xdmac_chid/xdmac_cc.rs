#[doc = "Register `XDMAC_CC` reader"]
pub struct R(crate::R<XDMAC_CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDMAC_CC` writer"]
pub struct W(crate::W<XDMAC_CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_CC_SPEC>;
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
impl From<crate::W<XDMAC_CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel x Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE_A {
    #[doc = "0: Self-triggered mode (memory-to-memory transfer)."]
    MEM_TRAN = 0,
    #[doc = "1: Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    PER_TRAN = 1,
}
impl From<TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE` reader - Channel x Transfer Type"]
pub struct TYPE_R(crate::FieldReader<bool, TYPE_A>);
impl TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            false => TYPE_A::MEM_TRAN,
            true => TYPE_A::PER_TRAN,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_TRAN`"]
    #[inline(always)]
    pub fn is_mem_tran(&self) -> bool {
        **self == TYPE_A::MEM_TRAN
    }
    #[doc = "Checks if the value of the field is `PER_TRAN`"]
    #[inline(always)]
    pub fn is_per_tran(&self) -> bool {
        **self == TYPE_A::PER_TRAN
    }
}
impl core::ops::Deref for TYPE_R {
    type Target = crate::FieldReader<bool, TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE` writer - Channel x Transfer Type"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn mem_tran(self) -> &'a mut W {
        self.variant(TYPE_A::MEM_TRAN)
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn per_tran(self) -> &'a mut W {
        self.variant(TYPE_A::PER_TRAN)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Channel x Memory Burst Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MBSIZE_A {
    #[doc = "0: The memory burst size is set to one."]
    SINGLE = 0,
    #[doc = "1: The memory burst size is set to four."]
    FOUR = 1,
    #[doc = "2: The memory burst size is set to eight."]
    EIGHT = 2,
    #[doc = "3: The memory burst size is set to sixteen."]
    SIXTEEN = 3,
}
impl From<MBSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: MBSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MBSIZE` reader - Channel x Memory Burst Size"]
pub struct MBSIZE_R(crate::FieldReader<u8, MBSIZE_A>);
impl MBSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MBSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBSIZE_A {
        match self.bits {
            0 => MBSIZE_A::SINGLE,
            1 => MBSIZE_A::FOUR,
            2 => MBSIZE_A::EIGHT,
            3 => MBSIZE_A::SIXTEEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == MBSIZE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        **self == MBSIZE_A::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        **self == MBSIZE_A::EIGHT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        **self == MBSIZE_A::SIXTEEN
    }
}
impl core::ops::Deref for MBSIZE_R {
    type Target = crate::FieldReader<u8, MBSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBSIZE` writer - Channel x Memory Burst Size"]
pub struct MBSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MBSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MBSIZE_A::SINGLE)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(MBSIZE_A::FOUR)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(MBSIZE_A::EIGHT)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(MBSIZE_A::SIXTEEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Channel x Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSYNC_A {
    #[doc = "0: Peripheral-to-memory transfer."]
    PER2MEM = 0,
    #[doc = "1: Memory-to-peripheral transfer."]
    MEM2PER = 1,
}
impl From<DSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: DSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSYNC` reader - Channel x Synchronization"]
pub struct DSYNC_R(crate::FieldReader<bool, DSYNC_A>);
impl DSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSYNC_A {
        match self.bits {
            false => DSYNC_A::PER2MEM,
            true => DSYNC_A::MEM2PER,
        }
    }
    #[doc = "Checks if the value of the field is `PER2MEM`"]
    #[inline(always)]
    pub fn is_per2mem(&self) -> bool {
        **self == DSYNC_A::PER2MEM
    }
    #[doc = "Checks if the value of the field is `MEM2PER`"]
    #[inline(always)]
    pub fn is_mem2per(&self) -> bool {
        **self == DSYNC_A::MEM2PER
    }
}
impl core::ops::Deref for DSYNC_R {
    type Target = crate::FieldReader<bool, DSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSYNC` writer - Channel x Synchronization"]
pub struct DSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn per2mem(self) -> &'a mut W {
        self.variant(DSYNC_A::PER2MEM)
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn mem2per(self) -> &'a mut W {
        self.variant(DSYNC_A::MEM2PER)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Channel x Software Request Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ_A {
    #[doc = "0: Hardware request line is connected to the peripheral request line."]
    HWR_CONNECTED = 0,
    #[doc = "1: Software request is connected to the peripheral request line."]
    SWR_CONNECTED = 1,
}
impl From<SWREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SWREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ` reader - Channel x Software Request Trigger"]
pub struct SWREQ_R(crate::FieldReader<bool, SWREQ_A>);
impl SWREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWREQ_A {
        match self.bits {
            false => SWREQ_A::HWR_CONNECTED,
            true => SWREQ_A::SWR_CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `HWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_hwr_connected(&self) -> bool {
        **self == SWREQ_A::HWR_CONNECTED
    }
    #[doc = "Checks if the value of the field is `SWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_swr_connected(&self) -> bool {
        **self == SWREQ_A::SWR_CONNECTED
    }
}
impl core::ops::Deref for SWREQ_R {
    type Target = crate::FieldReader<bool, SWREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWREQ` writer - Channel x Software Request Trigger"]
pub struct SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn hwr_connected(self) -> &'a mut W {
        self.variant(SWREQ_A::HWR_CONNECTED)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn swr_connected(self) -> &'a mut W {
        self.variant(SWREQ_A::SWR_CONNECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Channel x Fill Block of memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMSET_A {
    #[doc = "0: Memset is not activated."]
    NORMAL_MODE = 0,
    #[doc = "1: Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HW_MODE = 1,
}
impl From<MEMSET_A> for bool {
    #[inline(always)]
    fn from(variant: MEMSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMSET` reader - Channel x Fill Block of memory"]
pub struct MEMSET_R(crate::FieldReader<bool, MEMSET_A>);
impl MEMSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMSET_A {
        match self.bits {
            false => MEMSET_A::NORMAL_MODE,
            true => MEMSET_A::HW_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        **self == MEMSET_A::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `HW_MODE`"]
    #[inline(always)]
    pub fn is_hw_mode(&self) -> bool {
        **self == MEMSET_A::HW_MODE
    }
}
impl core::ops::Deref for MEMSET_R {
    type Target = crate::FieldReader<bool, MEMSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMSET` writer - Channel x Fill Block of memory"]
pub struct MEMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(MEMSET_A::NORMAL_MODE)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn hw_mode(self) -> &'a mut W {
        self.variant(MEMSET_A::HW_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Channel x Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSIZE_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 2 data transferred"]
    CHK_2 = 1,
    #[doc = "2: 4 data transferred"]
    CHK_4 = 2,
    #[doc = "3: 8 data transferred"]
    CHK_8 = 3,
    #[doc = "4: 16 data transferred"]
    CHK_16 = 4,
}
impl From<CSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSIZE` reader - Channel x Chunk Size"]
pub struct CSIZE_R(crate::FieldReader<u8, CSIZE_A>);
impl CSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZE_A> {
        match self.bits {
            0 => Some(CSIZE_A::CHK_1),
            1 => Some(CSIZE_A::CHK_2),
            2 => Some(CSIZE_A::CHK_4),
            3 => Some(CSIZE_A::CHK_8),
            4 => Some(CSIZE_A::CHK_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        **self == CSIZE_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_2`"]
    #[inline(always)]
    pub fn is_chk_2(&self) -> bool {
        **self == CSIZE_A::CHK_2
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        **self == CSIZE_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        **self == CSIZE_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        **self == CSIZE_A::CHK_16
    }
}
impl core::ops::Deref for CSIZE_R {
    type Target = crate::FieldReader<u8, CSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSIZE` writer - Channel x Chunk Size"]
pub struct CSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(CSIZE_A::CHK_1)
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn chk_2(self) -> &'a mut W {
        self.variant(CSIZE_A::CHK_2)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(CSIZE_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(CSIZE_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(CSIZE_A::CHK_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Channel x Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DWIDTH_A {
    #[doc = "0: The data size is set to 8 bits"]
    BYTE = 0,
    #[doc = "1: The data size is set to 16 bits"]
    HALFWORD = 1,
    #[doc = "2: The data size is set to 32 bits"]
    WORD = 2,
}
impl From<DWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DWIDTH` reader - Channel x Data Width"]
pub struct DWIDTH_R(crate::FieldReader<u8, DWIDTH_A>);
impl DWIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DWIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DWIDTH_A> {
        match self.bits {
            0 => Some(DWIDTH_A::BYTE),
            1 => Some(DWIDTH_A::HALFWORD),
            2 => Some(DWIDTH_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == DWIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        **self == DWIDTH_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == DWIDTH_A::WORD
    }
}
impl core::ops::Deref for DWIDTH_R {
    type Target = crate::FieldReader<u8, DWIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DWIDTH` writer - Channel x Data Width"]
pub struct DWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DWIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DWIDTH_A::BYTE)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DWIDTH_A::HALFWORD)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DWIDTH_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Channel x Source Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIF_A {
    #[doc = "0: The data is read through the system bus interface 0."]
    AHB_IF0 = 0,
    #[doc = "1: The data is read through the system bus interface 1."]
    AHB_IF1 = 1,
}
impl From<SIF_A> for bool {
    #[inline(always)]
    fn from(variant: SIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIF` reader - Channel x Source Interface Identifier"]
pub struct SIF_R(crate::FieldReader<bool, SIF_A>);
impl SIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIF_A {
        match self.bits {
            false => SIF_A::AHB_IF0,
            true => SIF_A::AHB_IF1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        **self == SIF_A::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        **self == SIF_A::AHB_IF1
    }
}
impl core::ops::Deref for SIF_R {
    type Target = crate::FieldReader<bool, SIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIF` writer - Channel x Source Interface Identifier"]
pub struct SIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(SIF_A::AHB_IF0)
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(SIF_A::AHB_IF1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Channel x Destination Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIF_A {
    #[doc = "0: The data is written through the system bus interface 0."]
    AHB_IF0 = 0,
    #[doc = "1: The data is written though the system bus interface 1."]
    AHB_IF1 = 1,
}
impl From<DIF_A> for bool {
    #[inline(always)]
    fn from(variant: DIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIF` reader - Channel x Destination Interface Identifier"]
pub struct DIF_R(crate::FieldReader<bool, DIF_A>);
impl DIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIF_A {
        match self.bits {
            false => DIF_A::AHB_IF0,
            true => DIF_A::AHB_IF1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        **self == DIF_A::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        **self == DIF_A::AHB_IF1
    }
}
impl core::ops::Deref for DIF_R {
    type Target = crate::FieldReader<bool, DIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIF` writer - Channel x Destination Interface Identifier"]
pub struct DIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(DIF_A::AHB_IF0)
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(DIF_A::AHB_IF1)
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
#[doc = "Channel x Source Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAM_A {
    #[doc = "0: The address remains unchanged."]
    FIXED_AM = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UBS_AM = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM = 3,
}
impl From<SAM_A> for u8 {
    #[inline(always)]
    fn from(variant: SAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SAM` reader - Channel x Source Addressing Mode"]
pub struct SAM_R(crate::FieldReader<u8, SAM_A>);
impl SAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAM_A {
        match self.bits {
            0 => SAM_A::FIXED_AM,
            1 => SAM_A::INCREMENTED_AM,
            2 => SAM_A::UBS_AM,
            3 => SAM_A::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        **self == SAM_A::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        **self == SAM_A::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        **self == SAM_A::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        **self == SAM_A::UBS_DS_AM
    }
}
impl core::ops::Deref for SAM_R {
    type Target = crate::FieldReader<u8, SAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAM` writer - Channel x Source Addressing Mode"]
pub struct SAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(SAM_A::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(SAM_A::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(SAM_A::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(SAM_A::UBS_DS_AM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Channel x Destination Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAM_A {
    #[doc = "0: The address remains unchanged."]
    FIXED_AM = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UBS_AM = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    UBS_DS_AM = 3,
}
impl From<DAM_A> for u8 {
    #[inline(always)]
    fn from(variant: DAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAM` reader - Channel x Destination Addressing Mode"]
pub struct DAM_R(crate::FieldReader<u8, DAM_A>);
impl DAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAM_A {
        match self.bits {
            0 => DAM_A::FIXED_AM,
            1 => DAM_A::INCREMENTED_AM,
            2 => DAM_A::UBS_AM,
            3 => DAM_A::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        **self == DAM_A::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        **self == DAM_A::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        **self == DAM_A::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        **self == DAM_A::UBS_DS_AM
    }
}
impl core::ops::Deref for DAM_R {
    type Target = crate::FieldReader<u8, DAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAM` writer - Channel x Destination Addressing Mode"]
pub struct DAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(DAM_A::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(DAM_A::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(DAM_A::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(DAM_A::UBS_DS_AM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Channel Initialization Terminated (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITD_A {
    #[doc = "0: Channel initialization is in progress."]
    IN_PROGRESS = 0,
    #[doc = "1: Channel initialization is completed."]
    TERMINATED = 1,
}
impl From<INITD_A> for bool {
    #[inline(always)]
    fn from(variant: INITD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITD` reader - Channel Initialization Terminated (this bit is read-only)"]
pub struct INITD_R(crate::FieldReader<bool, INITD_A>);
impl INITD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INITD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITD_A {
        match self.bits {
            false => INITD_A::IN_PROGRESS,
            true => INITD_A::TERMINATED,
        }
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        **self == INITD_A::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `TERMINATED`"]
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        **self == INITD_A::TERMINATED
    }
}
impl core::ops::Deref for INITD_R {
    type Target = crate::FieldReader<bool, INITD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITD` writer - Channel Initialization Terminated (this bit is read-only)"]
pub struct INITD_W<'a> {
    w: &'a mut W,
}
impl<'a> INITD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(INITD_A::IN_PROGRESS)
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn terminated(self) -> &'a mut W {
        self.variant(INITD_A::TERMINATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Read in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIP_A {
    #[doc = "0: No active read transaction on the bus."]
    DONE = 0,
    #[doc = "1: A read transaction is in progress."]
    IN_PROGRESS = 1,
}
impl From<RDIP_A> for bool {
    #[inline(always)]
    fn from(variant: RDIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIP` reader - Read in Progress (this bit is read-only)"]
pub struct RDIP_R(crate::FieldReader<bool, RDIP_A>);
impl RDIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIP_A {
        match self.bits {
            false => RDIP_A::DONE,
            true => RDIP_A::IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        **self == RDIP_A::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        **self == RDIP_A::IN_PROGRESS
    }
}
impl core::ops::Deref for RDIP_R {
    type Target = crate::FieldReader<bool, RDIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDIP` writer - Read in Progress (this bit is read-only)"]
pub struct RDIP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDIP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(RDIP_A::DONE)
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(RDIP_A::IN_PROGRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Write in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRIP_A {
    #[doc = "0: No active write transaction on the bus."]
    DONE = 0,
    #[doc = "1: A write transaction is in progress."]
    IN_PROGRESS = 1,
}
impl From<WRIP_A> for bool {
    #[inline(always)]
    fn from(variant: WRIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRIP` reader - Write in Progress (this bit is read-only)"]
pub struct WRIP_R(crate::FieldReader<bool, WRIP_A>);
impl WRIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRIP_A {
        match self.bits {
            false => WRIP_A::DONE,
            true => WRIP_A::IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        **self == WRIP_A::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        **self == WRIP_A::IN_PROGRESS
    }
}
impl core::ops::Deref for WRIP_R {
    type Target = crate::FieldReader<bool, WRIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRIP` writer - Write in Progress (this bit is read-only)"]
pub struct WRIP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRIP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(WRIP_A::DONE)
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(WRIP_A::IN_PROGRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Channel x Peripheral Hardware Request Line Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERID_A {
    #[doc = "0: HSMCI"]
    HSMCI = 0,
    #[doc = "1: SPI0_TX"]
    SPI0_TX = 1,
    #[doc = "2: SPI0_RX"]
    SPI0_RX = 2,
    #[doc = "5: QSPI_TX"]
    QSPI_TX = 5,
    #[doc = "6: QSPI_RX"]
    QSPI_RX = 6,
    #[doc = "7: USART0_TX"]
    USART0_TX = 7,
    #[doc = "8: USART0_RX"]
    USART0_RX = 8,
    #[doc = "9: USART1_TX"]
    USART1_TX = 9,
    #[doc = "10: USART1_RX"]
    USART1_RX = 10,
    #[doc = "11: USART2_TX"]
    USART2_TX = 11,
    #[doc = "12: USART2_RX"]
    USART2_RX = 12,
    #[doc = "13: PWM0"]
    PWM0 = 13,
    #[doc = "14: TWIHS0_TX"]
    TWIHS0_TX = 14,
    #[doc = "15: TWIHS0_RX"]
    TWIHS0_RX = 15,
    #[doc = "16: TWIHS1_TX"]
    TWIHS1_TX = 16,
    #[doc = "17: TWIHS1_RX"]
    TWIHS1_RX = 17,
    #[doc = "18: TWIHS2_TX"]
    TWIHS2_TX = 18,
    #[doc = "19: TWIHS2_RX"]
    TWIHS2_RX = 19,
    #[doc = "20: UART0_TX"]
    UART0_TX = 20,
    #[doc = "21: UART0_RX"]
    UART0_RX = 21,
    #[doc = "22: UART1_TX"]
    UART1_TX = 22,
    #[doc = "23: UART1_RX"]
    UART1_RX = 23,
    #[doc = "24: UART2_TX"]
    UART2_TX = 24,
    #[doc = "25: UART2_RX"]
    UART2_RX = 25,
    #[doc = "26: UART3_TX"]
    UART3_TX = 26,
    #[doc = "27: UART3_RX"]
    UART3_RX = 27,
    #[doc = "28: UART4_TX"]
    UART4_TX = 28,
    #[doc = "29: UART4_RX"]
    UART4_RX = 29,
    #[doc = "30: DACC0"]
    DACC0 = 30,
    #[doc = "31: DACC1"]
    DACC1 = 31,
    #[doc = "32: SSC_TX"]
    SSC_TX = 32,
    #[doc = "33: SSC_RX"]
    SSC_RX = 33,
    #[doc = "34: PIOA"]
    PIOA = 34,
    #[doc = "35: AFEC0"]
    AFEC0 = 35,
    #[doc = "36: AFEC1"]
    AFEC1 = 36,
    #[doc = "37: AES_TX"]
    AES_TX = 37,
    #[doc = "38: AES_RX"]
    AES_RX = 38,
    #[doc = "39: PWM1"]
    PWM1 = 39,
    #[doc = "40: TC0"]
    TC0 = 40,
    #[doc = "41: TC3"]
    TC3 = 41,
    #[doc = "42: TC6"]
    TC6 = 42,
    #[doc = "43: TC9"]
    TC9 = 43,
    #[doc = "44: I2SC0_TX_LEFT"]
    I2SC0_TX_LEFT = 44,
    #[doc = "45: I2SC0_RX_LEFT"]
    I2SC0_RX_LEFT = 45,
    #[doc = "48: I2SC0_TX_RIGHT"]
    I2SC0_TX_RIGHT = 48,
    #[doc = "49: I2SC0_RX_RIGHT"]
    I2SC0_RX_RIGHT = 49,
}
impl From<PERID_A> for u8 {
    #[inline(always)]
    fn from(variant: PERID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PERID` reader - Channel x Peripheral Hardware Request Line Identifier"]
pub struct PERID_R(crate::FieldReader<u8, PERID_A>);
impl PERID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERID_A> {
        match self.bits {
            0 => Some(PERID_A::HSMCI),
            1 => Some(PERID_A::SPI0_TX),
            2 => Some(PERID_A::SPI0_RX),
            5 => Some(PERID_A::QSPI_TX),
            6 => Some(PERID_A::QSPI_RX),
            7 => Some(PERID_A::USART0_TX),
            8 => Some(PERID_A::USART0_RX),
            9 => Some(PERID_A::USART1_TX),
            10 => Some(PERID_A::USART1_RX),
            11 => Some(PERID_A::USART2_TX),
            12 => Some(PERID_A::USART2_RX),
            13 => Some(PERID_A::PWM0),
            14 => Some(PERID_A::TWIHS0_TX),
            15 => Some(PERID_A::TWIHS0_RX),
            16 => Some(PERID_A::TWIHS1_TX),
            17 => Some(PERID_A::TWIHS1_RX),
            18 => Some(PERID_A::TWIHS2_TX),
            19 => Some(PERID_A::TWIHS2_RX),
            20 => Some(PERID_A::UART0_TX),
            21 => Some(PERID_A::UART0_RX),
            22 => Some(PERID_A::UART1_TX),
            23 => Some(PERID_A::UART1_RX),
            24 => Some(PERID_A::UART2_TX),
            25 => Some(PERID_A::UART2_RX),
            26 => Some(PERID_A::UART3_TX),
            27 => Some(PERID_A::UART3_RX),
            28 => Some(PERID_A::UART4_TX),
            29 => Some(PERID_A::UART4_RX),
            30 => Some(PERID_A::DACC0),
            31 => Some(PERID_A::DACC1),
            32 => Some(PERID_A::SSC_TX),
            33 => Some(PERID_A::SSC_RX),
            34 => Some(PERID_A::PIOA),
            35 => Some(PERID_A::AFEC0),
            36 => Some(PERID_A::AFEC1),
            37 => Some(PERID_A::AES_TX),
            38 => Some(PERID_A::AES_RX),
            39 => Some(PERID_A::PWM1),
            40 => Some(PERID_A::TC0),
            41 => Some(PERID_A::TC3),
            42 => Some(PERID_A::TC6),
            43 => Some(PERID_A::TC9),
            44 => Some(PERID_A::I2SC0_TX_LEFT),
            45 => Some(PERID_A::I2SC0_RX_LEFT),
            48 => Some(PERID_A::I2SC0_TX_RIGHT),
            49 => Some(PERID_A::I2SC0_RX_RIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSMCI`"]
    #[inline(always)]
    pub fn is_hsmci(&self) -> bool {
        **self == PERID_A::HSMCI
    }
    #[doc = "Checks if the value of the field is `SPI0_TX`"]
    #[inline(always)]
    pub fn is_spi0_tx(&self) -> bool {
        **self == PERID_A::SPI0_TX
    }
    #[doc = "Checks if the value of the field is `SPI0_RX`"]
    #[inline(always)]
    pub fn is_spi0_rx(&self) -> bool {
        **self == PERID_A::SPI0_RX
    }
    #[doc = "Checks if the value of the field is `QSPI_TX`"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        **self == PERID_A::QSPI_TX
    }
    #[doc = "Checks if the value of the field is `QSPI_RX`"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        **self == PERID_A::QSPI_RX
    }
    #[doc = "Checks if the value of the field is `USART0_TX`"]
    #[inline(always)]
    pub fn is_usart0_tx(&self) -> bool {
        **self == PERID_A::USART0_TX
    }
    #[doc = "Checks if the value of the field is `USART0_RX`"]
    #[inline(always)]
    pub fn is_usart0_rx(&self) -> bool {
        **self == PERID_A::USART0_RX
    }
    #[doc = "Checks if the value of the field is `USART1_TX`"]
    #[inline(always)]
    pub fn is_usart1_tx(&self) -> bool {
        **self == PERID_A::USART1_TX
    }
    #[doc = "Checks if the value of the field is `USART1_RX`"]
    #[inline(always)]
    pub fn is_usart1_rx(&self) -> bool {
        **self == PERID_A::USART1_RX
    }
    #[doc = "Checks if the value of the field is `USART2_TX`"]
    #[inline(always)]
    pub fn is_usart2_tx(&self) -> bool {
        **self == PERID_A::USART2_TX
    }
    #[doc = "Checks if the value of the field is `USART2_RX`"]
    #[inline(always)]
    pub fn is_usart2_rx(&self) -> bool {
        **self == PERID_A::USART2_RX
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        **self == PERID_A::PWM0
    }
    #[doc = "Checks if the value of the field is `TWIHS0_TX`"]
    #[inline(always)]
    pub fn is_twihs0_tx(&self) -> bool {
        **self == PERID_A::TWIHS0_TX
    }
    #[doc = "Checks if the value of the field is `TWIHS0_RX`"]
    #[inline(always)]
    pub fn is_twihs0_rx(&self) -> bool {
        **self == PERID_A::TWIHS0_RX
    }
    #[doc = "Checks if the value of the field is `TWIHS1_TX`"]
    #[inline(always)]
    pub fn is_twihs1_tx(&self) -> bool {
        **self == PERID_A::TWIHS1_TX
    }
    #[doc = "Checks if the value of the field is `TWIHS1_RX`"]
    #[inline(always)]
    pub fn is_twihs1_rx(&self) -> bool {
        **self == PERID_A::TWIHS1_RX
    }
    #[doc = "Checks if the value of the field is `TWIHS2_TX`"]
    #[inline(always)]
    pub fn is_twihs2_tx(&self) -> bool {
        **self == PERID_A::TWIHS2_TX
    }
    #[doc = "Checks if the value of the field is `TWIHS2_RX`"]
    #[inline(always)]
    pub fn is_twihs2_rx(&self) -> bool {
        **self == PERID_A::TWIHS2_RX
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        **self == PERID_A::UART0_TX
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        **self == PERID_A::UART0_RX
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        **self == PERID_A::UART1_TX
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        **self == PERID_A::UART1_RX
    }
    #[doc = "Checks if the value of the field is `UART2_TX`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        **self == PERID_A::UART2_TX
    }
    #[doc = "Checks if the value of the field is `UART2_RX`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        **self == PERID_A::UART2_RX
    }
    #[doc = "Checks if the value of the field is `UART3_TX`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        **self == PERID_A::UART3_TX
    }
    #[doc = "Checks if the value of the field is `UART3_RX`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        **self == PERID_A::UART3_RX
    }
    #[doc = "Checks if the value of the field is `UART4_TX`"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        **self == PERID_A::UART4_TX
    }
    #[doc = "Checks if the value of the field is `UART4_RX`"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        **self == PERID_A::UART4_RX
    }
    #[doc = "Checks if the value of the field is `DACC0`"]
    #[inline(always)]
    pub fn is_dacc0(&self) -> bool {
        **self == PERID_A::DACC0
    }
    #[doc = "Checks if the value of the field is `DACC1`"]
    #[inline(always)]
    pub fn is_dacc1(&self) -> bool {
        **self == PERID_A::DACC1
    }
    #[doc = "Checks if the value of the field is `SSC_TX`"]
    #[inline(always)]
    pub fn is_ssc_tx(&self) -> bool {
        **self == PERID_A::SSC_TX
    }
    #[doc = "Checks if the value of the field is `SSC_RX`"]
    #[inline(always)]
    pub fn is_ssc_rx(&self) -> bool {
        **self == PERID_A::SSC_RX
    }
    #[doc = "Checks if the value of the field is `PIOA`"]
    #[inline(always)]
    pub fn is_pioa(&self) -> bool {
        **self == PERID_A::PIOA
    }
    #[doc = "Checks if the value of the field is `AFEC0`"]
    #[inline(always)]
    pub fn is_afec0(&self) -> bool {
        **self == PERID_A::AFEC0
    }
    #[doc = "Checks if the value of the field is `AFEC1`"]
    #[inline(always)]
    pub fn is_afec1(&self) -> bool {
        **self == PERID_A::AFEC1
    }
    #[doc = "Checks if the value of the field is `AES_TX`"]
    #[inline(always)]
    pub fn is_aes_tx(&self) -> bool {
        **self == PERID_A::AES_TX
    }
    #[doc = "Checks if the value of the field is `AES_RX`"]
    #[inline(always)]
    pub fn is_aes_rx(&self) -> bool {
        **self == PERID_A::AES_RX
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        **self == PERID_A::PWM1
    }
    #[doc = "Checks if the value of the field is `TC0`"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        **self == PERID_A::TC0
    }
    #[doc = "Checks if the value of the field is `TC3`"]
    #[inline(always)]
    pub fn is_tc3(&self) -> bool {
        **self == PERID_A::TC3
    }
    #[doc = "Checks if the value of the field is `TC6`"]
    #[inline(always)]
    pub fn is_tc6(&self) -> bool {
        **self == PERID_A::TC6
    }
    #[doc = "Checks if the value of the field is `TC9`"]
    #[inline(always)]
    pub fn is_tc9(&self) -> bool {
        **self == PERID_A::TC9
    }
    #[doc = "Checks if the value of the field is `I2SC0_TX_LEFT`"]
    #[inline(always)]
    pub fn is_i2sc0_tx_left(&self) -> bool {
        **self == PERID_A::I2SC0_TX_LEFT
    }
    #[doc = "Checks if the value of the field is `I2SC0_RX_LEFT`"]
    #[inline(always)]
    pub fn is_i2sc0_rx_left(&self) -> bool {
        **self == PERID_A::I2SC0_RX_LEFT
    }
    #[doc = "Checks if the value of the field is `I2SC0_TX_RIGHT`"]
    #[inline(always)]
    pub fn is_i2sc0_tx_right(&self) -> bool {
        **self == PERID_A::I2SC0_TX_RIGHT
    }
    #[doc = "Checks if the value of the field is `I2SC0_RX_RIGHT`"]
    #[inline(always)]
    pub fn is_i2sc0_rx_right(&self) -> bool {
        **self == PERID_A::I2SC0_RX_RIGHT
    }
}
impl core::ops::Deref for PERID_R {
    type Target = crate::FieldReader<u8, PERID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERID` writer - Channel x Peripheral Hardware Request Line Identifier"]
pub struct PERID_W<'a> {
    w: &'a mut W,
}
impl<'a> PERID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSMCI"]
    #[inline(always)]
    pub fn hsmci(self) -> &'a mut W {
        self.variant(PERID_A::HSMCI)
    }
    #[doc = "SPI0_TX"]
    #[inline(always)]
    pub fn spi0_tx(self) -> &'a mut W {
        self.variant(PERID_A::SPI0_TX)
    }
    #[doc = "SPI0_RX"]
    #[inline(always)]
    pub fn spi0_rx(self) -> &'a mut W {
        self.variant(PERID_A::SPI0_RX)
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut W {
        self.variant(PERID_A::QSPI_TX)
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut W {
        self.variant(PERID_A::QSPI_RX)
    }
    #[doc = "USART0_TX"]
    #[inline(always)]
    pub fn usart0_tx(self) -> &'a mut W {
        self.variant(PERID_A::USART0_TX)
    }
    #[doc = "USART0_RX"]
    #[inline(always)]
    pub fn usart0_rx(self) -> &'a mut W {
        self.variant(PERID_A::USART0_RX)
    }
    #[doc = "USART1_TX"]
    #[inline(always)]
    pub fn usart1_tx(self) -> &'a mut W {
        self.variant(PERID_A::USART1_TX)
    }
    #[doc = "USART1_RX"]
    #[inline(always)]
    pub fn usart1_rx(self) -> &'a mut W {
        self.variant(PERID_A::USART1_RX)
    }
    #[doc = "USART2_TX"]
    #[inline(always)]
    pub fn usart2_tx(self) -> &'a mut W {
        self.variant(PERID_A::USART2_TX)
    }
    #[doc = "USART2_RX"]
    #[inline(always)]
    pub fn usart2_rx(self) -> &'a mut W {
        self.variant(PERID_A::USART2_RX)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(PERID_A::PWM0)
    }
    #[doc = "TWIHS0_TX"]
    #[inline(always)]
    pub fn twihs0_tx(self) -> &'a mut W {
        self.variant(PERID_A::TWIHS0_TX)
    }
    #[doc = "TWIHS0_RX"]
    #[inline(always)]
    pub fn twihs0_rx(self) -> &'a mut W {
        self.variant(PERID_A::TWIHS0_RX)
    }
    #[doc = "TWIHS1_TX"]
    #[inline(always)]
    pub fn twihs1_tx(self) -> &'a mut W {
        self.variant(PERID_A::TWIHS1_TX)
    }
    #[doc = "TWIHS1_RX"]
    #[inline(always)]
    pub fn twihs1_rx(self) -> &'a mut W {
        self.variant(PERID_A::TWIHS1_RX)
    }
    #[doc = "TWIHS2_TX"]
    #[inline(always)]
    pub fn twihs2_tx(self) -> &'a mut W {
        self.variant(PERID_A::TWIHS2_TX)
    }
    #[doc = "TWIHS2_RX"]
    #[inline(always)]
    pub fn twihs2_rx(self) -> &'a mut W {
        self.variant(PERID_A::TWIHS2_RX)
    }
    #[doc = "UART0_TX"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PERID_A::UART0_TX)
    }
    #[doc = "UART0_RX"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PERID_A::UART0_RX)
    }
    #[doc = "UART1_TX"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(PERID_A::UART1_TX)
    }
    #[doc = "UART1_RX"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(PERID_A::UART1_RX)
    }
    #[doc = "UART2_TX"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut W {
        self.variant(PERID_A::UART2_TX)
    }
    #[doc = "UART2_RX"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut W {
        self.variant(PERID_A::UART2_RX)
    }
    #[doc = "UART3_TX"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut W {
        self.variant(PERID_A::UART3_TX)
    }
    #[doc = "UART3_RX"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut W {
        self.variant(PERID_A::UART3_RX)
    }
    #[doc = "UART4_TX"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut W {
        self.variant(PERID_A::UART4_TX)
    }
    #[doc = "UART4_RX"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut W {
        self.variant(PERID_A::UART4_RX)
    }
    #[doc = "DACC0"]
    #[inline(always)]
    pub fn dacc0(self) -> &'a mut W {
        self.variant(PERID_A::DACC0)
    }
    #[doc = "DACC1"]
    #[inline(always)]
    pub fn dacc1(self) -> &'a mut W {
        self.variant(PERID_A::DACC1)
    }
    #[doc = "SSC_TX"]
    #[inline(always)]
    pub fn ssc_tx(self) -> &'a mut W {
        self.variant(PERID_A::SSC_TX)
    }
    #[doc = "SSC_RX"]
    #[inline(always)]
    pub fn ssc_rx(self) -> &'a mut W {
        self.variant(PERID_A::SSC_RX)
    }
    #[doc = "PIOA"]
    #[inline(always)]
    pub fn pioa(self) -> &'a mut W {
        self.variant(PERID_A::PIOA)
    }
    #[doc = "AFEC0"]
    #[inline(always)]
    pub fn afec0(self) -> &'a mut W {
        self.variant(PERID_A::AFEC0)
    }
    #[doc = "AFEC1"]
    #[inline(always)]
    pub fn afec1(self) -> &'a mut W {
        self.variant(PERID_A::AFEC1)
    }
    #[doc = "AES_TX"]
    #[inline(always)]
    pub fn aes_tx(self) -> &'a mut W {
        self.variant(PERID_A::AES_TX)
    }
    #[doc = "AES_RX"]
    #[inline(always)]
    pub fn aes_rx(self) -> &'a mut W {
        self.variant(PERID_A::AES_RX)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(PERID_A::PWM1)
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut W {
        self.variant(PERID_A::TC0)
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn tc3(self) -> &'a mut W {
        self.variant(PERID_A::TC3)
    }
    #[doc = "TC6"]
    #[inline(always)]
    pub fn tc6(self) -> &'a mut W {
        self.variant(PERID_A::TC6)
    }
    #[doc = "TC9"]
    #[inline(always)]
    pub fn tc9(self) -> &'a mut W {
        self.variant(PERID_A::TC9)
    }
    #[doc = "I2SC0_TX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_tx_left(self) -> &'a mut W {
        self.variant(PERID_A::I2SC0_TX_LEFT)
    }
    #[doc = "I2SC0_RX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_rx_left(self) -> &'a mut W {
        self.variant(PERID_A::I2SC0_RX_LEFT)
    }
    #[doc = "I2SC0_TX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_tx_right(self) -> &'a mut W {
        self.variant(PERID_A::I2SC0_TX_RIGHT)
    }
    #[doc = "I2SC0_RX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_rx_right(self) -> &'a mut W {
        self.variant(PERID_A::I2SC0_RX_RIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&self) -> MBSIZE_R {
        MBSIZE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&self) -> DSYNC_R {
        DSYNC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&self) -> SIF_R {
        SIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&self) -> DAM_R {
        DAM_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&self) -> INITD_R {
        INITD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&self) -> RDIP_R {
        RDIP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&self) -> WRIP_R {
        WRIP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&mut self) -> MBSIZE_W {
        MBSIZE_W { w: self }
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&mut self) -> DSYNC_W {
        DSYNC_W { w: self }
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W {
        SWREQ_W { w: self }
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W {
        MEMSET_W { w: self }
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&mut self) -> CSIZE_W {
        CSIZE_W { w: self }
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DWIDTH_W {
        DWIDTH_W { w: self }
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&mut self) -> SIF_W {
        SIF_W { w: self }
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&mut self) -> DIF_W {
        DIF_W { w: self }
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&mut self) -> SAM_W {
        SAM_W { w: self }
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&mut self) -> DAM_W {
        DAM_W { w: self }
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&mut self) -> INITD_W {
        INITD_W { w: self }
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&mut self) -> RDIP_W {
        RDIP_W { w: self }
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&mut self) -> WRIP_W {
        WRIP_W { w: self }
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> PERID_W {
        PERID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cc](index.html) module"]
pub struct XDMAC_CC_SPEC;
impl crate::RegisterSpec for XDMAC_CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_cc::R](R) reader structure"]
impl crate::Readable for XDMAC_CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdmac_cc::W](W) writer structure"]
impl crate::Writable for XDMAC_CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_CC to value 0"]
impl crate::Resettable for XDMAC_CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
