#[doc = "Reader of register INTSTATUS%s"]
pub type R = crate::R<u8, super::INTSTATUS>;
#[doc = "Reader of field `IRQ0`"]
pub type IRQ0_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ1`"]
pub type IRQ1_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ2`"]
pub type IRQ2_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ3`"]
pub type IRQ3_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ4`"]
pub type IRQ4_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ5`"]
pub type IRQ5_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ6`"]
pub type IRQ6_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ7`"]
pub type IRQ7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Status for Interrupt Request 0 within Interrupt n"]
    #[inline(always)]
    pub fn irq0(&self) -> IRQ0_R {
        IRQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status for Interrupt Request 1 within Interrupt n"]
    #[inline(always)]
    pub fn irq1(&self) -> IRQ1_R {
        IRQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status for Interrupt Request 2 within Interrupt n"]
    #[inline(always)]
    pub fn irq2(&self) -> IRQ2_R {
        IRQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status for Interrupt Request 3 within Interrupt n"]
    #[inline(always)]
    pub fn irq3(&self) -> IRQ3_R {
        IRQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status for Interrupt Request 4 within Interrupt n"]
    #[inline(always)]
    pub fn irq4(&self) -> IRQ4_R {
        IRQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status for Interrupt Request 5 within Interrupt n"]
    #[inline(always)]
    pub fn irq5(&self) -> IRQ5_R {
        IRQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status for Interrupt Request 6 within Interrupt n"]
    #[inline(always)]
    pub fn irq6(&self) -> IRQ6_R {
        IRQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status for Interrupt Request 7 within Interrupt n"]
    #[inline(always)]
    pub fn irq7(&self) -> IRQ7_R {
        IRQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
