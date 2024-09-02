#[doc = "Register `INSTRFRAME` reader"]
pub type R = crate::R<InstrframeSpec>;
#[doc = "Register `INSTRFRAME` writer"]
pub type W = crate::W<InstrframeSpec>;
#[doc = "Instruction Code, Address, Option Code and Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Widthselect {
    #[doc = "0: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SingleBitSpi = 0,
    #[doc = "1: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DualOutput = 1,
    #[doc = "2: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QuadOutput = 2,
    #[doc = "3: Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DualIo = 3,
    #[doc = "4: Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QuadIo = 4,
    #[doc = "5: Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DualCmd = 5,
    #[doc = "6: Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QuadCmd = 6,
}
impl From<Widthselect> for u8 {
    #[inline(always)]
    fn from(variant: Widthselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Widthselect {
    type Ux = u8;
}
impl crate::IsEnum for Widthselect {}
#[doc = "Field `WIDTH` reader - Instruction Code, Address, Option Code and Data Width"]
pub type WidthR = crate::FieldReader<Widthselect>;
impl WidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Widthselect> {
        match self.bits {
            0 => Some(Widthselect::SingleBitSpi),
            1 => Some(Widthselect::DualOutput),
            2 => Some(Widthselect::QuadOutput),
            3 => Some(Widthselect::DualIo),
            4 => Some(Widthselect::QuadIo),
            5 => Some(Widthselect::DualCmd),
            6 => Some(Widthselect::QuadCmd),
            _ => None,
        }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == Widthselect::SingleBitSpi
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_output(&self) -> bool {
        *self == Widthselect::DualOutput
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_output(&self) -> bool {
        *self == Widthselect::QuadOutput
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == Widthselect::DualIo
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == Widthselect::QuadIo
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_cmd(&self) -> bool {
        *self == Widthselect::DualCmd
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_cmd(&self) -> bool {
        *self == Widthselect::QuadCmd
    }
}
#[doc = "Field `WIDTH` writer - Instruction Code, Address, Option Code and Data Width"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Widthselect>;
impl<'a, REG> WidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn single_bit_spi(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::SingleBitSpi)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_output(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::DualOutput)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_output(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::QuadOutput)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::DualIo)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::QuadIo)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::DualCmd)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::QuadCmd)
    }
}
#[doc = "Field `INSTREN` reader - Instruction Enable"]
pub type InstrenR = crate::BitReader;
#[doc = "Field `INSTREN` writer - Instruction Enable"]
pub type InstrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDREN` reader - Address Enable"]
pub type AddrenR = crate::BitReader;
#[doc = "Field `ADDREN` writer - Address Enable"]
pub type AddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTCODEEN` reader - Option Enable"]
pub type OptcodeenR = crate::BitReader;
#[doc = "Field `OPTCODEEN` writer - Option Enable"]
pub type OptcodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAEN` reader - Data Enable"]
pub type DataenR = crate::BitReader;
#[doc = "Field `DATAEN` writer - Data Enable"]
pub type DataenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Option Code Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Optcodelenselect {
    #[doc = "0: 1-bit length option code"]
    _1bit = 0,
    #[doc = "1: 2-bits length option code"]
    _2bits = 1,
    #[doc = "2: 4-bits length option code"]
    _4bits = 2,
    #[doc = "3: 8-bits length option code"]
    _8bits = 3,
}
impl From<Optcodelenselect> for u8 {
    #[inline(always)]
    fn from(variant: Optcodelenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Optcodelenselect {
    type Ux = u8;
}
impl crate::IsEnum for Optcodelenselect {}
#[doc = "Field `OPTCODELEN` reader - Option Code Length"]
pub type OptcodelenR = crate::FieldReader<Optcodelenselect>;
impl OptcodelenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Optcodelenselect {
        match self.bits {
            0 => Optcodelenselect::_1bit,
            1 => Optcodelenselect::_2bits,
            2 => Optcodelenselect::_4bits,
            3 => Optcodelenselect::_8bits,
            _ => unreachable!(),
        }
    }
    #[doc = "1-bit length option code"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == Optcodelenselect::_1bit
    }
    #[doc = "2-bits length option code"]
    #[inline(always)]
    pub fn is_2bits(&self) -> bool {
        *self == Optcodelenselect::_2bits
    }
    #[doc = "4-bits length option code"]
    #[inline(always)]
    pub fn is_4bits(&self) -> bool {
        *self == Optcodelenselect::_4bits
    }
    #[doc = "8-bits length option code"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Optcodelenselect::_8bits
    }
}
#[doc = "Field `OPTCODELEN` writer - Option Code Length"]
pub type OptcodelenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Optcodelenselect, crate::Safe>;
impl<'a, REG> OptcodelenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-bit length option code"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut crate::W<REG> {
        self.variant(Optcodelenselect::_1bit)
    }
    #[doc = "2-bits length option code"]
    #[inline(always)]
    pub fn _2bits(self) -> &'a mut crate::W<REG> {
        self.variant(Optcodelenselect::_2bits)
    }
    #[doc = "4-bits length option code"]
    #[inline(always)]
    pub fn _4bits(self) -> &'a mut crate::W<REG> {
        self.variant(Optcodelenselect::_4bits)
    }
    #[doc = "8-bits length option code"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Optcodelenselect::_8bits)
    }
}
#[doc = "Address Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrlenselect {
    #[doc = "0: 24-bits address length"]
    _24bits = 0,
    #[doc = "1: 32-bits address length"]
    _32bits = 1,
}
impl From<Addrlenselect> for bool {
    #[inline(always)]
    fn from(variant: Addrlenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRLEN` reader - Address Length"]
pub type AddrlenR = crate::BitReader<Addrlenselect>;
impl AddrlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrlenselect {
        match self.bits {
            false => Addrlenselect::_24bits,
            true => Addrlenselect::_32bits,
        }
    }
    #[doc = "24-bits address length"]
    #[inline(always)]
    pub fn is_24bits(&self) -> bool {
        *self == Addrlenselect::_24bits
    }
    #[doc = "32-bits address length"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == Addrlenselect::_32bits
    }
}
#[doc = "Field `ADDRLEN` writer - Address Length"]
pub type AddrlenW<'a, REG> = crate::BitWriter<'a, REG, Addrlenselect>;
impl<'a, REG> AddrlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24-bits address length"]
    #[inline(always)]
    pub fn _24bits(self) -> &'a mut crate::W<REG> {
        self.variant(Addrlenselect::_24bits)
    }
    #[doc = "32-bits address length"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut crate::W<REG> {
        self.variant(Addrlenselect::_32bits)
    }
}
#[doc = "Data Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tfrtypeselect {
    #[doc = "0: Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    Read = 0,
    #[doc = "1: Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    Readmemory = 1,
    #[doc = "2: Write transfer into the serial memory.Scrambling is not performed."]
    Write = 2,
    #[doc = "3: Write data transfer into the serial memory.If enabled, scrambling is performed."]
    Writememory = 3,
}
impl From<Tfrtypeselect> for u8 {
    #[inline(always)]
    fn from(variant: Tfrtypeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tfrtypeselect {
    type Ux = u8;
}
impl crate::IsEnum for Tfrtypeselect {}
#[doc = "Field `TFRTYPE` reader - Data Transfer Type"]
pub type TfrtypeR = crate::FieldReader<Tfrtypeselect>;
impl TfrtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfrtypeselect {
        match self.bits {
            0 => Tfrtypeselect::Read,
            1 => Tfrtypeselect::Readmemory,
            2 => Tfrtypeselect::Write,
            3 => Tfrtypeselect::Writememory,
            _ => unreachable!(),
        }
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Tfrtypeselect::Read
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    #[inline(always)]
    pub fn is_readmemory(&self) -> bool {
        *self == Tfrtypeselect::Readmemory
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Tfrtypeselect::Write
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn is_writememory(&self) -> bool {
        *self == Tfrtypeselect::Writememory
    }
}
#[doc = "Field `TFRTYPE` writer - Data Transfer Type"]
pub type TfrtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tfrtypeselect, crate::Safe>;
impl<'a, REG> TfrtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial flash memory is not possible."]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypeselect::Read)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial flash memory is possible."]
    #[inline(always)]
    pub fn readmemory(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypeselect::Readmemory)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypeselect::Write)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn writememory(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypeselect::Writememory)
    }
}
#[doc = "Field `CRMODE` reader - Continuous Read Mode"]
pub type CrmodeR = crate::BitReader;
#[doc = "Field `CRMODE` writer - Continuous Read Mode"]
pub type CrmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDREN` reader - Double Data Rate Enable"]
pub type DdrenR = crate::BitReader;
#[doc = "Field `DDREN` writer - Double Data Rate Enable"]
pub type DdrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMYLEN` reader - Dummy Cycles Length"]
pub type DummylenR = crate::FieldReader;
#[doc = "Field `DUMMYLEN` writer - Dummy Cycles Length"]
pub type DummylenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn instren(&self) -> InstrenR {
        InstrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> AddrenR {
        AddrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn optcodeen(&self) -> OptcodeenR {
        OptcodeenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&self) -> DataenR {
        DataenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optcodelen(&self) -> OptcodelenR {
        OptcodelenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrlen(&self) -> AddrlenR {
        AddrlenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtype(&self) -> TfrtypeR {
        TfrtypeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crmode(&self) -> CrmodeR {
        CrmodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline(always)]
    pub fn ddren(&self) -> DdrenR {
        DdrenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline(always)]
    pub fn dummylen(&self) -> DummylenR {
        DummylenR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Instruction Code, Address, Option Code and Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WidthW<InstrframeSpec> {
        WidthW::new(self, 0)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    #[must_use]
    pub fn instren(&mut self) -> InstrenW<InstrframeSpec> {
        InstrenW::new(self, 4)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addren(&mut self) -> AddrenW<InstrframeSpec> {
        AddrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    #[must_use]
    pub fn optcodeen(&mut self) -> OptcodeenW<InstrframeSpec> {
        OptcodeenW::new(self, 6)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataen(&mut self) -> DataenW<InstrframeSpec> {
        DataenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    #[must_use]
    pub fn optcodelen(&mut self) -> OptcodelenW<InstrframeSpec> {
        OptcodelenW::new(self, 8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    #[must_use]
    pub fn addrlen(&mut self) -> AddrlenW<InstrframeSpec> {
        AddrlenW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn tfrtype(&mut self) -> TfrtypeW<InstrframeSpec> {
        TfrtypeW::new(self, 12)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn crmode(&mut self) -> CrmodeW<InstrframeSpec> {
        CrmodeW::new(self, 14)
    }
    #[doc = "Bit 15 - Double Data Rate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddren(&mut self) -> DdrenW<InstrframeSpec> {
        DdrenW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Dummy Cycles Length"]
    #[inline(always)]
    #[must_use]
    pub fn dummylen(&mut self) -> DummylenW<InstrframeSpec> {
        DummylenW::new(self, 16)
    }
}
#[doc = "Instruction Frame\n\nYou can [`read`](crate::Reg::read) this register and get [`instrframe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instrframe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstrframeSpec;
impl crate::RegisterSpec for InstrframeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instrframe::R`](R) reader structure"]
impl crate::Readable for InstrframeSpec {}
#[doc = "`write(|w| ..)` method takes [`instrframe::W`](W) writer structure"]
impl crate::Writable for InstrframeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INSTRFRAME to value 0"]
impl crate::Resettable for InstrframeSpec {
    const RESET_VALUE: u32 = 0;
}
