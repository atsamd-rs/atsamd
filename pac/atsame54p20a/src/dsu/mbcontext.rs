#[doc = "Reader of register MBCONTEXT"]
pub type R = crate::R<u32, super::MBCONTEXT>;
#[doc = "Algorithm Sub-step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBSTEP_A {
    #[doc = "1: `1`"]
    R0_1 = 1,
    #[doc = "3: `11`"]
    R1_1 = 3,
    #[doc = "5: `101`"]
    R0_2 = 5,
    #[doc = "7: `111`"]
    R1_2 = 7,
    #[doc = "9: `1001`"]
    R0_3 = 9,
    #[doc = "11: `1011`"]
    R1_3 = 11,
}
impl From<SUBSTEP_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBSTEP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBSTEP`"]
pub type SUBSTEP_R = crate::R<u8, SUBSTEP_A>;
impl SUBSTEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBSTEP_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SUBSTEP_A::R0_1),
            3 => Val(SUBSTEP_A::R1_1),
            5 => Val(SUBSTEP_A::R0_2),
            7 => Val(SUBSTEP_A::R1_2),
            9 => Val(SUBSTEP_A::R0_3),
            11 => Val(SUBSTEP_A::R1_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `R0_1`"]
    #[inline(always)]
    pub fn is_r0_1(&self) -> bool {
        *self == SUBSTEP_A::R0_1
    }
    #[doc = "Checks if the value of the field is `R1_1`"]
    #[inline(always)]
    pub fn is_r1_1(&self) -> bool {
        *self == SUBSTEP_A::R1_1
    }
    #[doc = "Checks if the value of the field is `R0_2`"]
    #[inline(always)]
    pub fn is_r0_2(&self) -> bool {
        *self == SUBSTEP_A::R0_2
    }
    #[doc = "Checks if the value of the field is `R1_2`"]
    #[inline(always)]
    pub fn is_r1_2(&self) -> bool {
        *self == SUBSTEP_A::R1_2
    }
    #[doc = "Checks if the value of the field is `R0_3`"]
    #[inline(always)]
    pub fn is_r0_3(&self) -> bool {
        *self == SUBSTEP_A::R0_3
    }
    #[doc = "Checks if the value of the field is `R1_3`"]
    #[inline(always)]
    pub fn is_r1_3(&self) -> bool {
        *self == SUBSTEP_A::R1_3
    }
}
#[doc = "Algorithm Step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STEP_A {
    #[doc = "2: `10`"]
    DOWN_R0W1 = 2,
    #[doc = "3: `11`"]
    UP_R1W0R0W1 = 3,
    #[doc = "4: `100`"]
    UP_R1W0 = 4,
    #[doc = "5: `101`"]
    UP_R0W1R1W0 = 5,
    #[doc = "6: `110`"]
    UP_R0 = 6,
    #[doc = "7: `111`"]
    UP_R0R0W0R0W1 = 7,
    #[doc = "8: `1000`"]
    UP_R1R1W1R1W0 = 8,
    #[doc = "9: `1001`"]
    DOWN_R0R0W0R0W1 = 9,
    #[doc = "10: `1010`"]
    DOWN_R1R1W1R1W0 = 10,
    #[doc = "12: `1100`"]
    UP_R0R0 = 12,
    #[doc = "14: `1110`"]
    DOWN_R1W0R0W1 = 14,
    #[doc = "15: `1111`"]
    DOWN_R1R1 = 15,
}
impl From<STEP_A> for u8 {
    #[inline(always)]
    fn from(variant: STEP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STEP`"]
pub type STEP_R = crate::R<u8, STEP_A>;
impl STEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STEP_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(STEP_A::DOWN_R0W1),
            3 => Val(STEP_A::UP_R1W0R0W1),
            4 => Val(STEP_A::UP_R1W0),
            5 => Val(STEP_A::UP_R0W1R1W0),
            6 => Val(STEP_A::UP_R0),
            7 => Val(STEP_A::UP_R0R0W0R0W1),
            8 => Val(STEP_A::UP_R1R1W1R1W0),
            9 => Val(STEP_A::DOWN_R0R0W0R0W1),
            10 => Val(STEP_A::DOWN_R1R1W1R1W0),
            12 => Val(STEP_A::UP_R0R0),
            14 => Val(STEP_A::DOWN_R1W0R0W1),
            15 => Val(STEP_A::DOWN_R1R1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DOWN_R0W1`"]
    #[inline(always)]
    pub fn is_down_r0w1(&self) -> bool {
        *self == STEP_A::DOWN_R0W1
    }
    #[doc = "Checks if the value of the field is `UP_R1W0R0W1`"]
    #[inline(always)]
    pub fn is_up_r1w0r0w1(&self) -> bool {
        *self == STEP_A::UP_R1W0R0W1
    }
    #[doc = "Checks if the value of the field is `UP_R1W0`"]
    #[inline(always)]
    pub fn is_up_r1w0(&self) -> bool {
        *self == STEP_A::UP_R1W0
    }
    #[doc = "Checks if the value of the field is `UP_R0W1R1W0`"]
    #[inline(always)]
    pub fn is_up_r0w1r1w0(&self) -> bool {
        *self == STEP_A::UP_R0W1R1W0
    }
    #[doc = "Checks if the value of the field is `UP_R0`"]
    #[inline(always)]
    pub fn is_up_r0(&self) -> bool {
        *self == STEP_A::UP_R0
    }
    #[doc = "Checks if the value of the field is `UP_R0R0W0R0W1`"]
    #[inline(always)]
    pub fn is_up_r0r0w0r0w1(&self) -> bool {
        *self == STEP_A::UP_R0R0W0R0W1
    }
    #[doc = "Checks if the value of the field is `UP_R1R1W1R1W0`"]
    #[inline(always)]
    pub fn is_up_r1r1w1r1w0(&self) -> bool {
        *self == STEP_A::UP_R1R1W1R1W0
    }
    #[doc = "Checks if the value of the field is `DOWN_R0R0W0R0W1`"]
    #[inline(always)]
    pub fn is_down_r0r0w0r0w1(&self) -> bool {
        *self == STEP_A::DOWN_R0R0W0R0W1
    }
    #[doc = "Checks if the value of the field is `DOWN_R1R1W1R1W0`"]
    #[inline(always)]
    pub fn is_down_r1r1w1r1w0(&self) -> bool {
        *self == STEP_A::DOWN_R1R1W1R1W0
    }
    #[doc = "Checks if the value of the field is `UP_R0R0`"]
    #[inline(always)]
    pub fn is_up_r0r0(&self) -> bool {
        *self == STEP_A::UP_R0R0
    }
    #[doc = "Checks if the value of the field is `DOWN_R1W0R0W1`"]
    #[inline(always)]
    pub fn is_down_r1w0r0w1(&self) -> bool {
        *self == STEP_A::DOWN_R1W0R0W1
    }
    #[doc = "Checks if the value of the field is `DOWN_R1R1`"]
    #[inline(always)]
    pub fn is_down_r1r1(&self) -> bool {
        *self == STEP_A::DOWN_R1R1
    }
}
#[doc = "Reader of field `PORT`"]
pub type PORT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Algorithm Sub-step"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Algorithm Step"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - DPRAM Port Index"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
