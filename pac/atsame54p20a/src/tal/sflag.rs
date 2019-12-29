#[doc = "Reader of register SFLAG%s"]
pub type R = crate::R<u32, super::SFLAG>;
#[doc = "Reader of field `IPS0`"]
pub type IPS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS1`"]
pub type IPS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS2`"]
pub type IPS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS3`"]
pub type IPS3_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS4`"]
pub type IPS4_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS5`"]
pub type IPS5_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS6`"]
pub type IPS6_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS7`"]
pub type IPS7_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS8`"]
pub type IPS8_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS9`"]
pub type IPS9_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS10`"]
pub type IPS10_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS11`"]
pub type IPS11_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS12`"]
pub type IPS12_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS13`"]
pub type IPS13_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS14`"]
pub type IPS14_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS15`"]
pub type IPS15_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS16`"]
pub type IPS16_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS17`"]
pub type IPS17_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS18`"]
pub type IPS18_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS19`"]
pub type IPS19_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS20`"]
pub type IPS20_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS21`"]
pub type IPS21_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS22`"]
pub type IPS22_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS23`"]
pub type IPS23_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS24`"]
pub type IPS24_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS25`"]
pub type IPS25_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS26`"]
pub type IPS26_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS27`"]
pub type IPS27_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS28`"]
pub type IPS28_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS29`"]
pub type IPS29_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS30`"]
pub type IPS30_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS31`"]
pub type IPS31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Inter-Process Signal 0"]
    #[inline(always)]
    pub fn ips0(&self) -> IPS0_R {
        IPS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Inter-Process Signal 1"]
    #[inline(always)]
    pub fn ips1(&self) -> IPS1_R {
        IPS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Inter-Process Signal 2"]
    #[inline(always)]
    pub fn ips2(&self) -> IPS2_R {
        IPS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Inter-Process Signal 3"]
    #[inline(always)]
    pub fn ips3(&self) -> IPS3_R {
        IPS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Inter-Process Signal 4"]
    #[inline(always)]
    pub fn ips4(&self) -> IPS4_R {
        IPS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Inter-Process Signal 5"]
    #[inline(always)]
    pub fn ips5(&self) -> IPS5_R {
        IPS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Inter-Process Signal 6"]
    #[inline(always)]
    pub fn ips6(&self) -> IPS6_R {
        IPS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Inter-Process Signal 7"]
    #[inline(always)]
    pub fn ips7(&self) -> IPS7_R {
        IPS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Inter-Process Signal 8"]
    #[inline(always)]
    pub fn ips8(&self) -> IPS8_R {
        IPS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Inter-Process Signal 9"]
    #[inline(always)]
    pub fn ips9(&self) -> IPS9_R {
        IPS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Inter-Process Signal 10"]
    #[inline(always)]
    pub fn ips10(&self) -> IPS10_R {
        IPS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Inter-Process Signal 11"]
    #[inline(always)]
    pub fn ips11(&self) -> IPS11_R {
        IPS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Inter-Process Signal 12"]
    #[inline(always)]
    pub fn ips12(&self) -> IPS12_R {
        IPS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Inter-Process Signal 13"]
    #[inline(always)]
    pub fn ips13(&self) -> IPS13_R {
        IPS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Inter-Process Signal 14"]
    #[inline(always)]
    pub fn ips14(&self) -> IPS14_R {
        IPS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Inter-Process Signal 15"]
    #[inline(always)]
    pub fn ips15(&self) -> IPS15_R {
        IPS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Inter-Process Signal 16"]
    #[inline(always)]
    pub fn ips16(&self) -> IPS16_R {
        IPS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Inter-Process Signal 17"]
    #[inline(always)]
    pub fn ips17(&self) -> IPS17_R {
        IPS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Inter-Process Signal 18"]
    #[inline(always)]
    pub fn ips18(&self) -> IPS18_R {
        IPS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Inter-Process Signal 19"]
    #[inline(always)]
    pub fn ips19(&self) -> IPS19_R {
        IPS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Inter-Process Signal 20"]
    #[inline(always)]
    pub fn ips20(&self) -> IPS20_R {
        IPS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Inter-Process Signal 21"]
    #[inline(always)]
    pub fn ips21(&self) -> IPS21_R {
        IPS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Inter-Process Signal 22"]
    #[inline(always)]
    pub fn ips22(&self) -> IPS22_R {
        IPS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Inter-Process Signal 23"]
    #[inline(always)]
    pub fn ips23(&self) -> IPS23_R {
        IPS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Inter-Process Signal 24"]
    #[inline(always)]
    pub fn ips24(&self) -> IPS24_R {
        IPS24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Inter-Process Signal 25"]
    #[inline(always)]
    pub fn ips25(&self) -> IPS25_R {
        IPS25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Inter-Process Signal 26"]
    #[inline(always)]
    pub fn ips26(&self) -> IPS26_R {
        IPS26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Inter-Process Signal 27"]
    #[inline(always)]
    pub fn ips27(&self) -> IPS27_R {
        IPS27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Inter-Process Signal 28"]
    #[inline(always)]
    pub fn ips28(&self) -> IPS28_R {
        IPS28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Inter-Process Signal 29"]
    #[inline(always)]
    pub fn ips29(&self) -> IPS29_R {
        IPS29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Inter-Process Signal 30"]
    #[inline(always)]
    pub fn ips30(&self) -> IPS30_R {
        IPS30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Inter-Process Signal 31"]
    #[inline(always)]
    pub fn ips31(&self) -> IPS31_R {
        IPS31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
