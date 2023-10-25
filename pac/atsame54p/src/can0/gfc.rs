#[doc = "Register `GFC` reader"]
pub type R = crate::R<GFC_SPEC>;
#[doc = "Register `GFC` writer"]
pub type W = crate::W<GFC_SPEC>;
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended"]
pub type RRFE_R = crate::BitReader;
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended"]
pub type RRFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard"]
pub type RRFS_R = crate::BitReader;
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard"]
pub type RRFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended"]
pub type ANFE_R = crate::FieldReader<ANFESELECT_A>;
#[doc = "Accept Non-matching Frames Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFESELECT_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    RXF0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RXF1 = 1,
    #[doc = "2: Reject"]
    REJECT = 2,
}
impl From<ANFESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFESELECT_A {
    type Ux = u8;
}
impl ANFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ANFESELECT_A> {
        match self.bits {
            0 => Some(ANFESELECT_A::RXF0),
            1 => Some(ANFESELECT_A::RXF1),
            2 => Some(ANFESELECT_A::REJECT),
            _ => None,
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == ANFESELECT_A::RXF0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == ANFESELECT_A::RXF1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == ANFESELECT_A::REJECT
    }
}
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended"]
pub type ANFE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ANFESELECT_A>;
impl<'a, REG, const O: u8> ANFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFESELECT_A::RXF0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFESELECT_A::RXF1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut crate::W<REG> {
        self.variant(ANFESELECT_A::REJECT)
    }
}
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard"]
pub type ANFS_R = crate::FieldReader<ANFSSELECT_A>;
#[doc = "Accept Non-matching Frames Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFSSELECT_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    RXF0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RXF1 = 1,
    #[doc = "2: Reject"]
    REJECT = 2,
}
impl From<ANFSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFSSELECT_A {
    type Ux = u8;
}
impl ANFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ANFSSELECT_A> {
        match self.bits {
            0 => Some(ANFSSELECT_A::RXF0),
            1 => Some(ANFSSELECT_A::RXF1),
            2 => Some(ANFSSELECT_A::REJECT),
            _ => None,
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == ANFSSELECT_A::RXF0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == ANFSSELECT_A::RXF1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == ANFSSELECT_A::REJECT
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard"]
pub type ANFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ANFSSELECT_A>;
impl<'a, REG, const O: u8> ANFS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFSSELECT_A::RXF0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFSSELECT_A::RXF1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut crate::W<REG> {
        self.variant(ANFSSELECT_A::REJECT)
    }
}
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<GFC_SPEC, 0> {
        RRFE_W::new(self)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<GFC_SPEC, 1> {
        RRFS_W::new(self)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<GFC_SPEC, 2> {
        ANFE_W::new(self)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<GFC_SPEC, 4> {
        ANFS_W::new(self)
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
#[doc = "Global Filter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GFC_SPEC;
impl crate::RegisterSpec for GFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfc::R`](R) reader structure"]
impl crate::Readable for GFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gfc::W`](W) writer structure"]
impl crate::Writable for GFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
