#[doc = "Register `INSTRFRAME` reader"]
pub type R = crate::R<INSTRFRAME_SPEC>;
#[doc = "Register `INSTRFRAME` writer"]
pub type W = crate::W<INSTRFRAME_SPEC>;
#[doc = "Field `WIDTH` reader - Instruction Code, Address, Option Code and Data Width"]
pub type WIDTH_R = crate::FieldReader<WIDTHSELECT_A>;
#[doc = "Instruction Code, Address, Option Code and Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WIDTHSELECT_A {
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
impl From<WIDTHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WIDTHSELECT_A {
    type Ux = u8;
}
impl WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WIDTHSELECT_A> {
        match self.bits {
            0 => Some(WIDTHSELECT_A::SINGLE_BIT_SPI),
            1 => Some(WIDTHSELECT_A::DUAL_OUTPUT),
            2 => Some(WIDTHSELECT_A::QUAD_OUTPUT),
            3 => Some(WIDTHSELECT_A::DUAL_IO),
            4 => Some(WIDTHSELECT_A::QUAD_IO),
            5 => Some(WIDTHSELECT_A::DUAL_CMD),
            6 => Some(WIDTHSELECT_A::QUAD_CMD),
            _ => None,
        }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == WIDTHSELECT_A::SINGLE_BIT_SPI
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_output(&self) -> bool {
        *self == WIDTHSELECT_A::DUAL_OUTPUT
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_output(&self) -> bool {
        *self == WIDTHSELECT_A::QUAD_OUTPUT
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == WIDTHSELECT_A::DUAL_IO
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == WIDTHSELECT_A::QUAD_IO
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_cmd(&self) -> bool {
        *self == WIDTHSELECT_A::DUAL_CMD
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_cmd(&self) -> bool {
        *self == WIDTHSELECT_A::QUAD_CMD
    }
}
#[doc = "Field `WIDTH` writer - Instruction Code, Address, Option Code and Data Width"]
pub type WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WIDTHSELECT_A>;
impl<'a, REG, const O: u8> WIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn single_bit_spi(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTHSELECT_A::SINGLE_BIT_SPI)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_output(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTHSELECT_A::DUAL_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_output(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTHSELECT_A::QUAD_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTHSELECT_A::DUAL_IO)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTHSELECT_A::QUAD_IO)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTHSELECT_A::DUAL_CMD)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTHSELECT_A::QUAD_CMD)
    }
}
#[doc = "Field `INSTREN` reader - Instruction Enable"]
pub type INSTREN_R = crate::BitReader;
#[doc = "Field `INSTREN` writer - Instruction Enable"]
pub type INSTREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDREN` reader - Address Enable"]
pub type ADDREN_R = crate::BitReader;
#[doc = "Field `ADDREN` writer - Address Enable"]
pub type ADDREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPTCODEEN` reader - Option Enable"]
pub type OPTCODEEN_R = crate::BitReader;
#[doc = "Field `OPTCODEEN` writer - Option Enable"]
pub type OPTCODEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAEN` reader - Data Enable"]
pub type DATAEN_R = crate::BitReader;
#[doc = "Field `DATAEN` writer - Data Enable"]
pub type DATAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPTCODELEN` reader - Option Code Length"]
pub type OPTCODELEN_R = crate::FieldReader<OPTCODELENSELECT_A>;
#[doc = "Option Code Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPTCODELENSELECT_A {
    #[doc = "0: 1-bit length option code"]
    _1BIT = 0,
    #[doc = "1: 2-bits length option code"]
    _2BITS = 1,
    #[doc = "2: 4-bits length option code"]
    _4BITS = 2,
    #[doc = "3: 8-bits length option code"]
    _8BITS = 3,
}
impl From<OPTCODELENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OPTCODELENSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPTCODELENSELECT_A {
    type Ux = u8;
}
impl OPTCODELEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTCODELENSELECT_A {
        match self.bits {
            0 => OPTCODELENSELECT_A::_1BIT,
            1 => OPTCODELENSELECT_A::_2BITS,
            2 => OPTCODELENSELECT_A::_4BITS,
            3 => OPTCODELENSELECT_A::_8BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "1-bit length option code"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == OPTCODELENSELECT_A::_1BIT
    }
    #[doc = "2-bits length option code"]
    #[inline(always)]
    pub fn is_2bits(&self) -> bool {
        *self == OPTCODELENSELECT_A::_2BITS
    }
    #[doc = "4-bits length option code"]
    #[inline(always)]
    pub fn is_4bits(&self) -> bool {
        *self == OPTCODELENSELECT_A::_4BITS
    }
    #[doc = "8-bits length option code"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == OPTCODELENSELECT_A::_8BITS
    }
}
#[doc = "Field `OPTCODELEN` writer - Option Code Length"]
pub type OPTCODELEN_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, OPTCODELENSELECT_A>;
impl<'a, REG, const O: u8> OPTCODELEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-bit length option code"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut crate::W<REG> {
        self.variant(OPTCODELENSELECT_A::_1BIT)
    }
    #[doc = "2-bits length option code"]
    #[inline(always)]
    pub fn _2bits(self) -> &'a mut crate::W<REG> {
        self.variant(OPTCODELENSELECT_A::_2BITS)
    }
    #[doc = "4-bits length option code"]
    #[inline(always)]
    pub fn _4bits(self) -> &'a mut crate::W<REG> {
        self.variant(OPTCODELENSELECT_A::_4BITS)
    }
    #[doc = "8-bits length option code"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(OPTCODELENSELECT_A::_8BITS)
    }
}
#[doc = "Field `ADDRLEN` reader - Address Length"]
pub type ADDRLEN_R = crate::BitReader<ADDRLENSELECT_A>;
#[doc = "Address Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRLENSELECT_A {
    #[doc = "0: 24-bits address length"]
    _24BITS = 0,
    #[doc = "1: 32-bits address length"]
    _32BITS = 1,
}
impl From<ADDRLENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRLENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRLENSELECT_A {
        match self.bits {
            false => ADDRLENSELECT_A::_24BITS,
            true => ADDRLENSELECT_A::_32BITS,
        }
    }
    #[doc = "24-bits address length"]
    #[inline(always)]
    pub fn is_24bits(&self) -> bool {
        *self == ADDRLENSELECT_A::_24BITS
    }
    #[doc = "32-bits address length"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == ADDRLENSELECT_A::_32BITS
    }
}
#[doc = "Field `ADDRLEN` writer - Address Length"]
pub type ADDRLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADDRLENSELECT_A>;
impl<'a, REG, const O: u8> ADDRLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24-bits address length"]
    #[inline(always)]
    pub fn _24bits(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRLENSELECT_A::_24BITS)
    }
    #[doc = "32-bits address length"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRLENSELECT_A::_32BITS)
    }
}
#[doc = "Field `TFRTYPE` reader - Data Transfer Type"]
pub type TFRTYPE_R = crate::FieldReader<TFRTYPESELECT_A>;
#[doc = "Data Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFRTYPESELECT_A {
    #[doc = "0: Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    READ = 0,
    #[doc = "1: Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    READMEMORY = 1,
    #[doc = "2: Write transfer into the serial memory.Scrambling is not performed."]
    WRITE = 2,
    #[doc = "3: Write data transfer into the serial memory.If enabled, scrambling is performed."]
    WRITEMEMORY = 3,
}
impl From<TFRTYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TFRTYPESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TFRTYPESELECT_A {
    type Ux = u8;
}
impl TFRTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFRTYPESELECT_A {
        match self.bits {
            0 => TFRTYPESELECT_A::READ,
            1 => TFRTYPESELECT_A::READMEMORY,
            2 => TFRTYPESELECT_A::WRITE,
            3 => TFRTYPESELECT_A::WRITEMEMORY,
            _ => unreachable!(),
        }
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == TFRTYPESELECT_A::READ
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    #[inline(always)]
    pub fn is_readmemory(&self) -> bool {
        *self == TFRTYPESELECT_A::READMEMORY
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == TFRTYPESELECT_A::WRITE
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn is_writememory(&self) -> bool {
        *self == TFRTYPESELECT_A::WRITEMEMORY
    }
}
#[doc = "Field `TFRTYPE` writer - Data Transfer Type"]
pub type TFRTYPE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TFRTYPESELECT_A>;
impl<'a, REG, const O: u8> TFRTYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(TFRTYPESELECT_A::READ)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    #[inline(always)]
    pub fn readmemory(self) -> &'a mut crate::W<REG> {
        self.variant(TFRTYPESELECT_A::READMEMORY)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(TFRTYPESELECT_A::WRITE)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn writememory(self) -> &'a mut crate::W<REG> {
        self.variant(TFRTYPESELECT_A::WRITEMEMORY)
    }
}
#[doc = "Field `CRMODE` reader - Continuous Read Mode"]
pub type CRMODE_R = crate::BitReader;
#[doc = "Field `CRMODE` writer - Continuous Read Mode"]
pub type CRMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDREN` reader - Double Data Rate Enable"]
pub type DDREN_R = crate::BitReader;
#[doc = "Field `DDREN` writer - Double Data Rate Enable"]
pub type DDREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUMMYLEN` reader - Dummy Cycles Length"]
pub type DUMMYLEN_R = crate::FieldReader;
#[doc = "Field `DUMMYLEN` writer - Dummy Cycles Length"]
pub type DUMMYLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn instren(&self) -> INSTREN_R {
        INSTREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn optcodeen(&self) -> OPTCODEEN_R {
        OPTCODEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&self) -> DATAEN_R {
        DATAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optcodelen(&self) -> OPTCODELEN_R {
        OPTCODELEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrlen(&self) -> ADDRLEN_R {
        ADDRLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtype(&self) -> TFRTYPE_R {
        TFRTYPE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crmode(&self) -> CRMODE_R {
        CRMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline(always)]
    pub fn ddren(&self) -> DDREN_R {
        DDREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline(always)]
    pub fn dummylen(&self) -> DUMMYLEN_R {
        DUMMYLEN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<INSTRFRAME_SPEC, 0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    #[must_use]
    pub fn instren(&mut self) -> INSTREN_W<INSTRFRAME_SPEC, 4> {
        INSTREN_W::new(self)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addren(&mut self) -> ADDREN_W<INSTRFRAME_SPEC, 5> {
        ADDREN_W::new(self)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    #[must_use]
    pub fn optcodeen(&mut self) -> OPTCODEEN_W<INSTRFRAME_SPEC, 6> {
        OPTCODEEN_W::new(self)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataen(&mut self) -> DATAEN_W<INSTRFRAME_SPEC, 7> {
        DATAEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    #[must_use]
    pub fn optcodelen(&mut self) -> OPTCODELEN_W<INSTRFRAME_SPEC, 8> {
        OPTCODELEN_W::new(self)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    #[must_use]
    pub fn addrlen(&mut self) -> ADDRLEN_W<INSTRFRAME_SPEC, 10> {
        ADDRLEN_W::new(self)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn tfrtype(&mut self) -> TFRTYPE_W<INSTRFRAME_SPEC, 12> {
        TFRTYPE_W::new(self)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn crmode(&mut self) -> CRMODE_W<INSTRFRAME_SPEC, 14> {
        CRMODE_W::new(self)
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddren(&mut self) -> DDREN_W<INSTRFRAME_SPEC, 15> {
        DDREN_W::new(self)
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline(always)]
    #[must_use]
    pub fn dummylen(&mut self) -> DUMMYLEN_W<INSTRFRAME_SPEC, 16> {
        DUMMYLEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Instruction Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instrframe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instrframe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTRFRAME_SPEC;
impl crate::RegisterSpec for INSTRFRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instrframe::R`](R) reader structure"]
impl crate::Readable for INSTRFRAME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`instrframe::W`](W) writer structure"]
impl crate::Writable for INSTRFRAME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTRFRAME to value 0"]
impl crate::Resettable for INSTRFRAME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
