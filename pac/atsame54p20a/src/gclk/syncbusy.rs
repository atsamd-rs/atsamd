#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Generic Clock Generator Control 0 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL0_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL0_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL0`"]
pub type GENCTRL0_R = crate::R<bool, GENCTRL0_A>;
impl GENCTRL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL0_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL0_A::GCLK0),
            true => Val(GENCTRL0_A::GCLK1),
            true => Val(GENCTRL0_A::GCLK2),
            true => Val(GENCTRL0_A::GCLK3),
            true => Val(GENCTRL0_A::GCLK4),
            true => Val(GENCTRL0_A::GCLK5),
            true => Val(GENCTRL0_A::GCLK6),
            true => Val(GENCTRL0_A::GCLK7),
            true => Val(GENCTRL0_A::GCLK8),
            true => Val(GENCTRL0_A::GCLK9),
            true => Val(GENCTRL0_A::GCLK10),
            true => Val(GENCTRL0_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL0_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL0_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL0_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL0_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL0_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL0_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL0_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL0_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL0_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL0_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL0_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL0_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 1 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL1_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL1_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL1`"]
pub type GENCTRL1_R = crate::R<bool, GENCTRL1_A>;
impl GENCTRL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL1_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL1_A::GCLK0),
            true => Val(GENCTRL1_A::GCLK1),
            true => Val(GENCTRL1_A::GCLK2),
            true => Val(GENCTRL1_A::GCLK3),
            true => Val(GENCTRL1_A::GCLK4),
            true => Val(GENCTRL1_A::GCLK5),
            true => Val(GENCTRL1_A::GCLK6),
            true => Val(GENCTRL1_A::GCLK7),
            true => Val(GENCTRL1_A::GCLK8),
            true => Val(GENCTRL1_A::GCLK9),
            true => Val(GENCTRL1_A::GCLK10),
            true => Val(GENCTRL1_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL1_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL1_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL1_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL1_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL1_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL1_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL1_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL1_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL1_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL1_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL1_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL1_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 2 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL2_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL2_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL2`"]
pub type GENCTRL2_R = crate::R<bool, GENCTRL2_A>;
impl GENCTRL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL2_A::GCLK0),
            true => Val(GENCTRL2_A::GCLK1),
            true => Val(GENCTRL2_A::GCLK2),
            true => Val(GENCTRL2_A::GCLK3),
            true => Val(GENCTRL2_A::GCLK4),
            true => Val(GENCTRL2_A::GCLK5),
            true => Val(GENCTRL2_A::GCLK6),
            true => Val(GENCTRL2_A::GCLK7),
            true => Val(GENCTRL2_A::GCLK8),
            true => Val(GENCTRL2_A::GCLK9),
            true => Val(GENCTRL2_A::GCLK10),
            true => Val(GENCTRL2_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL2_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL2_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL2_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL2_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL2_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL2_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL2_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL2_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL2_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL2_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL2_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL2_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 3 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL3_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL3_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL3`"]
pub type GENCTRL3_R = crate::R<bool, GENCTRL3_A>;
impl GENCTRL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL3_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL3_A::GCLK0),
            true => Val(GENCTRL3_A::GCLK1),
            true => Val(GENCTRL3_A::GCLK2),
            true => Val(GENCTRL3_A::GCLK3),
            true => Val(GENCTRL3_A::GCLK4),
            true => Val(GENCTRL3_A::GCLK5),
            true => Val(GENCTRL3_A::GCLK6),
            true => Val(GENCTRL3_A::GCLK7),
            true => Val(GENCTRL3_A::GCLK8),
            true => Val(GENCTRL3_A::GCLK9),
            true => Val(GENCTRL3_A::GCLK10),
            true => Val(GENCTRL3_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL3_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL3_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL3_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL3_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL3_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL3_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL3_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL3_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL3_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL3_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL3_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL3_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 4 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL4_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL4_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL4`"]
pub type GENCTRL4_R = crate::R<bool, GENCTRL4_A>;
impl GENCTRL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL4_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL4_A::GCLK0),
            true => Val(GENCTRL4_A::GCLK1),
            true => Val(GENCTRL4_A::GCLK2),
            true => Val(GENCTRL4_A::GCLK3),
            true => Val(GENCTRL4_A::GCLK4),
            true => Val(GENCTRL4_A::GCLK5),
            true => Val(GENCTRL4_A::GCLK6),
            true => Val(GENCTRL4_A::GCLK7),
            true => Val(GENCTRL4_A::GCLK8),
            true => Val(GENCTRL4_A::GCLK9),
            true => Val(GENCTRL4_A::GCLK10),
            true => Val(GENCTRL4_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL4_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL4_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL4_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL4_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL4_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL4_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL4_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL4_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL4_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL4_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL4_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL4_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 5 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL5_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL5_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL5`"]
pub type GENCTRL5_R = crate::R<bool, GENCTRL5_A>;
impl GENCTRL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL5_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL5_A::GCLK0),
            true => Val(GENCTRL5_A::GCLK1),
            true => Val(GENCTRL5_A::GCLK2),
            true => Val(GENCTRL5_A::GCLK3),
            true => Val(GENCTRL5_A::GCLK4),
            true => Val(GENCTRL5_A::GCLK5),
            true => Val(GENCTRL5_A::GCLK6),
            true => Val(GENCTRL5_A::GCLK7),
            true => Val(GENCTRL5_A::GCLK8),
            true => Val(GENCTRL5_A::GCLK9),
            true => Val(GENCTRL5_A::GCLK10),
            true => Val(GENCTRL5_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL5_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL5_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL5_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL5_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL5_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL5_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL5_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL5_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL5_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL5_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL5_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL5_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 6 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL6_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL6_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL6`"]
pub type GENCTRL6_R = crate::R<bool, GENCTRL6_A>;
impl GENCTRL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL6_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL6_A::GCLK0),
            true => Val(GENCTRL6_A::GCLK1),
            true => Val(GENCTRL6_A::GCLK2),
            true => Val(GENCTRL6_A::GCLK3),
            true => Val(GENCTRL6_A::GCLK4),
            true => Val(GENCTRL6_A::GCLK5),
            true => Val(GENCTRL6_A::GCLK6),
            true => Val(GENCTRL6_A::GCLK7),
            true => Val(GENCTRL6_A::GCLK8),
            true => Val(GENCTRL6_A::GCLK9),
            true => Val(GENCTRL6_A::GCLK10),
            true => Val(GENCTRL6_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL6_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL6_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL6_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL6_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL6_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL6_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL6_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL6_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL6_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL6_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL6_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL6_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 7 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL7_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL7_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL7`"]
pub type GENCTRL7_R = crate::R<bool, GENCTRL7_A>;
impl GENCTRL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL7_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL7_A::GCLK0),
            true => Val(GENCTRL7_A::GCLK1),
            true => Val(GENCTRL7_A::GCLK2),
            true => Val(GENCTRL7_A::GCLK3),
            true => Val(GENCTRL7_A::GCLK4),
            true => Val(GENCTRL7_A::GCLK5),
            true => Val(GENCTRL7_A::GCLK6),
            true => Val(GENCTRL7_A::GCLK7),
            true => Val(GENCTRL7_A::GCLK8),
            true => Val(GENCTRL7_A::GCLK9),
            true => Val(GENCTRL7_A::GCLK10),
            true => Val(GENCTRL7_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL7_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL7_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL7_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL7_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL7_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL7_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL7_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL7_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL7_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL7_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL7_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL7_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 8 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL8_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL8_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL8`"]
pub type GENCTRL8_R = crate::R<bool, GENCTRL8_A>;
impl GENCTRL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL8_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL8_A::GCLK0),
            true => Val(GENCTRL8_A::GCLK1),
            true => Val(GENCTRL8_A::GCLK2),
            true => Val(GENCTRL8_A::GCLK3),
            true => Val(GENCTRL8_A::GCLK4),
            true => Val(GENCTRL8_A::GCLK5),
            true => Val(GENCTRL8_A::GCLK6),
            true => Val(GENCTRL8_A::GCLK7),
            true => Val(GENCTRL8_A::GCLK8),
            true => Val(GENCTRL8_A::GCLK9),
            true => Val(GENCTRL8_A::GCLK10),
            true => Val(GENCTRL8_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL8_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL8_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL8_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL8_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL8_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL8_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL8_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL8_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL8_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL8_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL8_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL8_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 9 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL9_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL9_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL9`"]
pub type GENCTRL9_R = crate::R<bool, GENCTRL9_A>;
impl GENCTRL9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL9_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL9_A::GCLK0),
            true => Val(GENCTRL9_A::GCLK1),
            true => Val(GENCTRL9_A::GCLK2),
            true => Val(GENCTRL9_A::GCLK3),
            true => Val(GENCTRL9_A::GCLK4),
            true => Val(GENCTRL9_A::GCLK5),
            true => Val(GENCTRL9_A::GCLK6),
            true => Val(GENCTRL9_A::GCLK7),
            true => Val(GENCTRL9_A::GCLK8),
            true => Val(GENCTRL9_A::GCLK9),
            true => Val(GENCTRL9_A::GCLK10),
            true => Val(GENCTRL9_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL9_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL9_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL9_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL9_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL9_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL9_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL9_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL9_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL9_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL9_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL9_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL9_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 10 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL10_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL10_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL10`"]
pub type GENCTRL10_R = crate::R<bool, GENCTRL10_A>;
impl GENCTRL10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL10_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL10_A::GCLK0),
            true => Val(GENCTRL10_A::GCLK1),
            true => Val(GENCTRL10_A::GCLK2),
            true => Val(GENCTRL10_A::GCLK3),
            true => Val(GENCTRL10_A::GCLK4),
            true => Val(GENCTRL10_A::GCLK5),
            true => Val(GENCTRL10_A::GCLK6),
            true => Val(GENCTRL10_A::GCLK7),
            true => Val(GENCTRL10_A::GCLK8),
            true => Val(GENCTRL10_A::GCLK9),
            true => Val(GENCTRL10_A::GCLK10),
            true => Val(GENCTRL10_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL10_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL10_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL10_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL10_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL10_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL10_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL10_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL10_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL10_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL10_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL10_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL10_A::GCLK11
    }
}
#[doc = "Generic Clock Generator Control 11 Synchronization Busy bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL11_A {
    #[doc = "1: Generic clock generator 0"]
    GCLK0 = 1,
    #[doc = "2: Generic clock generator 1"]
    GCLK1 = 2,
    #[doc = "4: Generic clock generator 2"]
    GCLK2 = 4,
    #[doc = "8: Generic clock generator 3"]
    GCLK3 = 8,
    #[doc = "16: Generic clock generator 4"]
    GCLK4 = 16,
    #[doc = "32: Generic clock generator 5"]
    GCLK5 = 32,
    #[doc = "64: Generic clock generator 6"]
    GCLK6 = 64,
    #[doc = "128: Generic clock generator 7"]
    GCLK7 = 128,
    #[doc = "256: Generic clock generator 8"]
    GCLK8 = 256,
    #[doc = "512: Generic clock generator 9"]
    GCLK9 = 512,
    #[doc = "1024: Generic clock generator 10"]
    GCLK10 = 1024,
    #[doc = "2048: Generic clock generator 11"]
    GCLK11 = 2048,
}
impl From<GENCTRL11_A> for bool {
    #[inline(always)]
    fn from(variant: GENCTRL11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GENCTRL11`"]
pub type GENCTRL11_R = crate::R<bool, GENCTRL11_A>;
impl GENCTRL11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, GENCTRL11_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(GENCTRL11_A::GCLK0),
            true => Val(GENCTRL11_A::GCLK1),
            true => Val(GENCTRL11_A::GCLK2),
            true => Val(GENCTRL11_A::GCLK3),
            true => Val(GENCTRL11_A::GCLK4),
            true => Val(GENCTRL11_A::GCLK5),
            true => Val(GENCTRL11_A::GCLK6),
            true => Val(GENCTRL11_A::GCLK7),
            true => Val(GENCTRL11_A::GCLK8),
            true => Val(GENCTRL11_A::GCLK9),
            true => Val(GENCTRL11_A::GCLK10),
            true => Val(GENCTRL11_A::GCLK11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL11_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL11_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL11_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL11_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL11_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL11_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL11_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL11_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL11_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL11_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL11_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL11_A::GCLK11
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl0(&self) -> GENCTRL0_R {
        GENCTRL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl1(&self) -> GENCTRL1_R {
        GENCTRL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl2(&self) -> GENCTRL2_R {
        GENCTRL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl3(&self) -> GENCTRL3_R {
        GENCTRL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl4(&self) -> GENCTRL4_R {
        GENCTRL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl5(&self) -> GENCTRL5_R {
        GENCTRL5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl6(&self) -> GENCTRL6_R {
        GENCTRL6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl7(&self) -> GENCTRL7_R {
        GENCTRL7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl8(&self) -> GENCTRL8_R {
        GENCTRL8_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic Clock Generator Control 9 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl9(&self) -> GENCTRL9_R {
        GENCTRL9_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Generic Clock Generator Control 10 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl10(&self) -> GENCTRL10_R {
        GENCTRL10_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Generic Clock Generator Control 11 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl11(&self) -> GENCTRL11_R {
        GENCTRL11_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
