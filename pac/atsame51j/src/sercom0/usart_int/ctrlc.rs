#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CTRLC_SPEC>;
#[doc = "Field `GTIME` reader - Guard Time"]
pub type GTIME_R = crate::FieldReader;
#[doc = "Field `GTIME` writer - Guard Time"]
pub type GTIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BRKLEN` reader - LIN Master Break Length"]
pub type BRKLEN_R = crate::FieldReader;
#[doc = "Field `BRKLEN` writer - LIN Master Break Length"]
pub type BRKLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HDRDLY` reader - LIN Master Header Delay"]
pub type HDRDLY_R = crate::FieldReader;
#[doc = "Field `HDRDLY` writer - LIN Master Header Delay"]
pub type HDRDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `INACK` reader - Inhibit Not Acknowledge"]
pub type INACK_R = crate::BitReader;
#[doc = "Field `INACK` writer - Inhibit Not Acknowledge"]
pub type INACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DSNACK_R = crate::BitReader;
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DSNACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAXITER` reader - Maximum Iterations"]
pub type MAXITER_R = crate::FieldReader;
#[doc = "Field `MAXITER` writer - Maximum Iterations"]
pub type MAXITER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub type DATA32B_R = crate::FieldReader<DATA32BSELECT_A>;
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATA32BSELECT_A {
    #[doc = "0: Data reads and writes according CTRLB.CHSIZE"]
    DATA_READ_WRITE_CHSIZE = 0,
    #[doc = "1: Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    DATA_READ_CHSIZE_WRITE_32BIT = 1,
    #[doc = "2: Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    DATA_READ_32BIT_WRITE_CHSIZE = 2,
    #[doc = "3: Data reads and writes according 32-bit extension"]
    DATA_READ_WRITE_32BIT = 3,
}
impl From<DATA32BSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA32BSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATA32BSELECT_A {
    type Ux = u8;
}
impl DATA32B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA32BSELECT_A {
        match self.bits {
            0 => DATA32BSELECT_A::DATA_READ_WRITE_CHSIZE,
            1 => DATA32BSELECT_A::DATA_READ_CHSIZE_WRITE_32BIT,
            2 => DATA32BSELECT_A::DATA_READ_32BIT_WRITE_CHSIZE,
            3 => DATA32BSELECT_A::DATA_READ_WRITE_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Data reads and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn is_data_read_write_chsize(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_READ_WRITE_CHSIZE
    }
    #[doc = "Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    #[inline(always)]
    pub fn is_data_read_chsize_write_32bit(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_READ_CHSIZE_WRITE_32BIT
    }
    #[doc = "Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn is_data_read_32bit_write_chsize(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_READ_32BIT_WRITE_CHSIZE
    }
    #[doc = "Data reads and writes according 32-bit extension"]
    #[inline(always)]
    pub fn is_data_read_write_32bit(&self) -> bool {
        *self == DATA32BSELECT_A::DATA_READ_WRITE_32BIT
    }
}
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub type DATA32B_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DATA32BSELECT_A>;
impl<'a, REG, const O: u8> DATA32B_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data reads and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_write_chsize(self) -> &'a mut crate::W<REG> {
        self.variant(DATA32BSELECT_A::DATA_READ_WRITE_CHSIZE)
    }
    #[doc = "Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_chsize_write_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATA32BSELECT_A::DATA_READ_CHSIZE_WRITE_32BIT)
    }
    #[doc = "Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_32bit_write_chsize(self) -> &'a mut crate::W<REG> {
        self.variant(DATA32BSELECT_A::DATA_READ_32BIT_WRITE_CHSIZE)
    }
    #[doc = "Data reads and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_write_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATA32BSELECT_A::DATA_READ_WRITE_32BIT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GTIME_R {
        GTIME_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BRKLEN_R {
        BRKLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HDRDLY_R {
        HDRDLY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&self) -> MAXITER_R {
        MAXITER_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> DATA32B_R {
        DATA32B_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    #[must_use]
    pub fn gtime(&mut self) -> GTIME_W<CTRLC_SPEC, 0> {
        GTIME_W::new(self)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    #[must_use]
    pub fn brklen(&mut self) -> BRKLEN_W<CTRLC_SPEC, 8> {
        BRKLEN_W::new(self)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    #[must_use]
    pub fn hdrdly(&mut self) -> HDRDLY_W<CTRLC_SPEC, 10> {
        HDRDLY_W::new(self)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn inack(&mut self) -> INACK_W<CTRLC_SPEC, 16> {
        INACK_W::new(self)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    #[must_use]
    pub fn dsnack(&mut self) -> DSNACK_W<CTRLC_SPEC, 17> {
        DSNACK_W::new(self)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    #[must_use]
    pub fn maxiter(&mut self) -> MAXITER_W<CTRLC_SPEC, 20> {
        MAXITER_W::new(self)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn data32b(&mut self) -> DATA32B_W<CTRLC_SPEC, 24> {
        DATA32B_W::new(self)
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
#[doc = "USART_INT Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
