#[doc = "Reader of register BRKSTATUS"]
pub type R = crate::R<u16, super::BRKSTATUS>;
#[doc = "Reader of field `CPU0`"]
pub type CPU0_R = crate::R<u8, u8>;
#[doc = "Reader of field `CPU1`"]
pub type CPU1_R = crate::R<u8, u8>;
#[doc = "Reader of field `EVBRK`"]
pub type EVBRK_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXTBRK`"]
pub type EXTBRK_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - CPU 0 Break Request"]
    #[inline(always)]
    pub fn cpu0(&self) -> CPU0_R {
        CPU0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CPU 1 Break Request"]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Event Break Request"]
    #[inline(always)]
    pub fn evbrk(&self) -> EVBRK_R {
        EVBRK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - External Break Request"]
    #[inline(always)]
    pub fn extbrk(&self) -> EXTBRK_R {
        EXTBRK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
