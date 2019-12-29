#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BOD33RDY`"]
pub type BOD33RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33DET`"]
pub type BOD33DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `B33SRDY`"]
pub type B33SRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD12RDY`"]
pub type BOD12RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD12DET`"]
pub type BOD12DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `B12SRDY`"]
pub type B12SRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREGRDY`"]
pub type VREGRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VCORERDY`"]
pub type VCORERDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BOD12 Ready"]
    #[inline(always)]
    pub fn bod12rdy(&self) -> BOD12RDY_R {
        BOD12RDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BOD12 Detection"]
    #[inline(always)]
    pub fn bod12det(&self) -> BOD12DET_R {
        BOD12DET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BOD12 Synchronization Ready"]
    #[inline(always)]
    pub fn b12srdy(&self) -> B12SRDY_R {
        B12SRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VREGRDY_R {
        VREGRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VCORERDY_R {
        VCORERDY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
