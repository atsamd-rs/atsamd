#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYNCBUSY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `GENCTRL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL0R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL0R::GCLK0 => true,
            GENCTRL0R::GCLK1 => true,
            GENCTRL0R::GCLK2 => true,
            GENCTRL0R::GCLK3 => true,
            GENCTRL0R::GCLK4 => true,
            GENCTRL0R::GCLK5 => true,
            GENCTRL0R::GCLK6 => true,
            GENCTRL0R::GCLK7 => true,
            GENCTRL0R::GCLK8 => true,
            GENCTRL0R::GCLK9 => true,
            GENCTRL0R::GCLK10 => true,
            GENCTRL0R::GCLK11 => true,
            GENCTRL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL0R {
        match value {
            true => GENCTRL0R::GCLK0,
            true => GENCTRL0R::GCLK1,
            true => GENCTRL0R::GCLK2,
            true => GENCTRL0R::GCLK3,
            true => GENCTRL0R::GCLK4,
            true => GENCTRL0R::GCLK5,
            true => GENCTRL0R::GCLK6,
            true => GENCTRL0R::GCLK7,
            true => GENCTRL0R::GCLK8,
            true => GENCTRL0R::GCLK9,
            true => GENCTRL0R::GCLK10,
            true => GENCTRL0R::GCLK11,
            i => GENCTRL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL0R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL0R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL0R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL0R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL0R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL0R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL0R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL0R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL0R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL0R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL0R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL0R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL1R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL1R::GCLK0 => true,
            GENCTRL1R::GCLK1 => true,
            GENCTRL1R::GCLK2 => true,
            GENCTRL1R::GCLK3 => true,
            GENCTRL1R::GCLK4 => true,
            GENCTRL1R::GCLK5 => true,
            GENCTRL1R::GCLK6 => true,
            GENCTRL1R::GCLK7 => true,
            GENCTRL1R::GCLK8 => true,
            GENCTRL1R::GCLK9 => true,
            GENCTRL1R::GCLK10 => true,
            GENCTRL1R::GCLK11 => true,
            GENCTRL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL1R {
        match value {
            true => GENCTRL1R::GCLK0,
            true => GENCTRL1R::GCLK1,
            true => GENCTRL1R::GCLK2,
            true => GENCTRL1R::GCLK3,
            true => GENCTRL1R::GCLK4,
            true => GENCTRL1R::GCLK5,
            true => GENCTRL1R::GCLK6,
            true => GENCTRL1R::GCLK7,
            true => GENCTRL1R::GCLK8,
            true => GENCTRL1R::GCLK9,
            true => GENCTRL1R::GCLK10,
            true => GENCTRL1R::GCLK11,
            i => GENCTRL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL1R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL1R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL1R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL1R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL1R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL1R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL1R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL1R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL1R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL1R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL1R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL1R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL2R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL2R::GCLK0 => true,
            GENCTRL2R::GCLK1 => true,
            GENCTRL2R::GCLK2 => true,
            GENCTRL2R::GCLK3 => true,
            GENCTRL2R::GCLK4 => true,
            GENCTRL2R::GCLK5 => true,
            GENCTRL2R::GCLK6 => true,
            GENCTRL2R::GCLK7 => true,
            GENCTRL2R::GCLK8 => true,
            GENCTRL2R::GCLK9 => true,
            GENCTRL2R::GCLK10 => true,
            GENCTRL2R::GCLK11 => true,
            GENCTRL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL2R {
        match value {
            true => GENCTRL2R::GCLK0,
            true => GENCTRL2R::GCLK1,
            true => GENCTRL2R::GCLK2,
            true => GENCTRL2R::GCLK3,
            true => GENCTRL2R::GCLK4,
            true => GENCTRL2R::GCLK5,
            true => GENCTRL2R::GCLK6,
            true => GENCTRL2R::GCLK7,
            true => GENCTRL2R::GCLK8,
            true => GENCTRL2R::GCLK9,
            true => GENCTRL2R::GCLK10,
            true => GENCTRL2R::GCLK11,
            i => GENCTRL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL2R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL2R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL2R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL2R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL2R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL2R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL2R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL2R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL2R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL2R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL2R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL2R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL3R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL3R::GCLK0 => true,
            GENCTRL3R::GCLK1 => true,
            GENCTRL3R::GCLK2 => true,
            GENCTRL3R::GCLK3 => true,
            GENCTRL3R::GCLK4 => true,
            GENCTRL3R::GCLK5 => true,
            GENCTRL3R::GCLK6 => true,
            GENCTRL3R::GCLK7 => true,
            GENCTRL3R::GCLK8 => true,
            GENCTRL3R::GCLK9 => true,
            GENCTRL3R::GCLK10 => true,
            GENCTRL3R::GCLK11 => true,
            GENCTRL3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL3R {
        match value {
            true => GENCTRL3R::GCLK0,
            true => GENCTRL3R::GCLK1,
            true => GENCTRL3R::GCLK2,
            true => GENCTRL3R::GCLK3,
            true => GENCTRL3R::GCLK4,
            true => GENCTRL3R::GCLK5,
            true => GENCTRL3R::GCLK6,
            true => GENCTRL3R::GCLK7,
            true => GENCTRL3R::GCLK8,
            true => GENCTRL3R::GCLK9,
            true => GENCTRL3R::GCLK10,
            true => GENCTRL3R::GCLK11,
            i => GENCTRL3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL3R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL3R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL3R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL3R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL3R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL3R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL3R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL3R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL3R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL3R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL3R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL3R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL4R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL4R::GCLK0 => true,
            GENCTRL4R::GCLK1 => true,
            GENCTRL4R::GCLK2 => true,
            GENCTRL4R::GCLK3 => true,
            GENCTRL4R::GCLK4 => true,
            GENCTRL4R::GCLK5 => true,
            GENCTRL4R::GCLK6 => true,
            GENCTRL4R::GCLK7 => true,
            GENCTRL4R::GCLK8 => true,
            GENCTRL4R::GCLK9 => true,
            GENCTRL4R::GCLK10 => true,
            GENCTRL4R::GCLK11 => true,
            GENCTRL4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL4R {
        match value {
            true => GENCTRL4R::GCLK0,
            true => GENCTRL4R::GCLK1,
            true => GENCTRL4R::GCLK2,
            true => GENCTRL4R::GCLK3,
            true => GENCTRL4R::GCLK4,
            true => GENCTRL4R::GCLK5,
            true => GENCTRL4R::GCLK6,
            true => GENCTRL4R::GCLK7,
            true => GENCTRL4R::GCLK8,
            true => GENCTRL4R::GCLK9,
            true => GENCTRL4R::GCLK10,
            true => GENCTRL4R::GCLK11,
            i => GENCTRL4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL4R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL4R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL4R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL4R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL4R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL4R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL4R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL4R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL4R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL4R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL4R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL4R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL5R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL5R::GCLK0 => true,
            GENCTRL5R::GCLK1 => true,
            GENCTRL5R::GCLK2 => true,
            GENCTRL5R::GCLK3 => true,
            GENCTRL5R::GCLK4 => true,
            GENCTRL5R::GCLK5 => true,
            GENCTRL5R::GCLK6 => true,
            GENCTRL5R::GCLK7 => true,
            GENCTRL5R::GCLK8 => true,
            GENCTRL5R::GCLK9 => true,
            GENCTRL5R::GCLK10 => true,
            GENCTRL5R::GCLK11 => true,
            GENCTRL5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL5R {
        match value {
            true => GENCTRL5R::GCLK0,
            true => GENCTRL5R::GCLK1,
            true => GENCTRL5R::GCLK2,
            true => GENCTRL5R::GCLK3,
            true => GENCTRL5R::GCLK4,
            true => GENCTRL5R::GCLK5,
            true => GENCTRL5R::GCLK6,
            true => GENCTRL5R::GCLK7,
            true => GENCTRL5R::GCLK8,
            true => GENCTRL5R::GCLK9,
            true => GENCTRL5R::GCLK10,
            true => GENCTRL5R::GCLK11,
            i => GENCTRL5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL5R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL5R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL5R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL5R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL5R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL5R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL5R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL5R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL5R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL5R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL5R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL5R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL6R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL6R::GCLK0 => true,
            GENCTRL6R::GCLK1 => true,
            GENCTRL6R::GCLK2 => true,
            GENCTRL6R::GCLK3 => true,
            GENCTRL6R::GCLK4 => true,
            GENCTRL6R::GCLK5 => true,
            GENCTRL6R::GCLK6 => true,
            GENCTRL6R::GCLK7 => true,
            GENCTRL6R::GCLK8 => true,
            GENCTRL6R::GCLK9 => true,
            GENCTRL6R::GCLK10 => true,
            GENCTRL6R::GCLK11 => true,
            GENCTRL6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL6R {
        match value {
            true => GENCTRL6R::GCLK0,
            true => GENCTRL6R::GCLK1,
            true => GENCTRL6R::GCLK2,
            true => GENCTRL6R::GCLK3,
            true => GENCTRL6R::GCLK4,
            true => GENCTRL6R::GCLK5,
            true => GENCTRL6R::GCLK6,
            true => GENCTRL6R::GCLK7,
            true => GENCTRL6R::GCLK8,
            true => GENCTRL6R::GCLK9,
            true => GENCTRL6R::GCLK10,
            true => GENCTRL6R::GCLK11,
            i => GENCTRL6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL6R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL6R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL6R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL6R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL6R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL6R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL6R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL6R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL6R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL6R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL6R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL6R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL7R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL7R::GCLK0 => true,
            GENCTRL7R::GCLK1 => true,
            GENCTRL7R::GCLK2 => true,
            GENCTRL7R::GCLK3 => true,
            GENCTRL7R::GCLK4 => true,
            GENCTRL7R::GCLK5 => true,
            GENCTRL7R::GCLK6 => true,
            GENCTRL7R::GCLK7 => true,
            GENCTRL7R::GCLK8 => true,
            GENCTRL7R::GCLK9 => true,
            GENCTRL7R::GCLK10 => true,
            GENCTRL7R::GCLK11 => true,
            GENCTRL7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL7R {
        match value {
            true => GENCTRL7R::GCLK0,
            true => GENCTRL7R::GCLK1,
            true => GENCTRL7R::GCLK2,
            true => GENCTRL7R::GCLK3,
            true => GENCTRL7R::GCLK4,
            true => GENCTRL7R::GCLK5,
            true => GENCTRL7R::GCLK6,
            true => GENCTRL7R::GCLK7,
            true => GENCTRL7R::GCLK8,
            true => GENCTRL7R::GCLK9,
            true => GENCTRL7R::GCLK10,
            true => GENCTRL7R::GCLK11,
            i => GENCTRL7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL7R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL7R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL7R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL7R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL7R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL7R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL7R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL7R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL7R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL7R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL7R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL7R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL8R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL8R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL8R::GCLK0 => true,
            GENCTRL8R::GCLK1 => true,
            GENCTRL8R::GCLK2 => true,
            GENCTRL8R::GCLK3 => true,
            GENCTRL8R::GCLK4 => true,
            GENCTRL8R::GCLK5 => true,
            GENCTRL8R::GCLK6 => true,
            GENCTRL8R::GCLK7 => true,
            GENCTRL8R::GCLK8 => true,
            GENCTRL8R::GCLK9 => true,
            GENCTRL8R::GCLK10 => true,
            GENCTRL8R::GCLK11 => true,
            GENCTRL8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL8R {
        match value {
            true => GENCTRL8R::GCLK0,
            true => GENCTRL8R::GCLK1,
            true => GENCTRL8R::GCLK2,
            true => GENCTRL8R::GCLK3,
            true => GENCTRL8R::GCLK4,
            true => GENCTRL8R::GCLK5,
            true => GENCTRL8R::GCLK6,
            true => GENCTRL8R::GCLK7,
            true => GENCTRL8R::GCLK8,
            true => GENCTRL8R::GCLK9,
            true => GENCTRL8R::GCLK10,
            true => GENCTRL8R::GCLK11,
            i => GENCTRL8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL8R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL8R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL8R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL8R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL8R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL8R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL8R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL8R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL8R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL8R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL8R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL8R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL9R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL9R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL9R::GCLK0 => true,
            GENCTRL9R::GCLK1 => true,
            GENCTRL9R::GCLK2 => true,
            GENCTRL9R::GCLK3 => true,
            GENCTRL9R::GCLK4 => true,
            GENCTRL9R::GCLK5 => true,
            GENCTRL9R::GCLK6 => true,
            GENCTRL9R::GCLK7 => true,
            GENCTRL9R::GCLK8 => true,
            GENCTRL9R::GCLK9 => true,
            GENCTRL9R::GCLK10 => true,
            GENCTRL9R::GCLK11 => true,
            GENCTRL9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL9R {
        match value {
            true => GENCTRL9R::GCLK0,
            true => GENCTRL9R::GCLK1,
            true => GENCTRL9R::GCLK2,
            true => GENCTRL9R::GCLK3,
            true => GENCTRL9R::GCLK4,
            true => GENCTRL9R::GCLK5,
            true => GENCTRL9R::GCLK6,
            true => GENCTRL9R::GCLK7,
            true => GENCTRL9R::GCLK8,
            true => GENCTRL9R::GCLK9,
            true => GENCTRL9R::GCLK10,
            true => GENCTRL9R::GCLK11,
            i => GENCTRL9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL9R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL9R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL9R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL9R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL9R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL9R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL9R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL9R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL9R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL9R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL9R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL9R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL10R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL10R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL10R::GCLK0 => true,
            GENCTRL10R::GCLK1 => true,
            GENCTRL10R::GCLK2 => true,
            GENCTRL10R::GCLK3 => true,
            GENCTRL10R::GCLK4 => true,
            GENCTRL10R::GCLK5 => true,
            GENCTRL10R::GCLK6 => true,
            GENCTRL10R::GCLK7 => true,
            GENCTRL10R::GCLK8 => true,
            GENCTRL10R::GCLK9 => true,
            GENCTRL10R::GCLK10 => true,
            GENCTRL10R::GCLK11 => true,
            GENCTRL10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL10R {
        match value {
            true => GENCTRL10R::GCLK0,
            true => GENCTRL10R::GCLK1,
            true => GENCTRL10R::GCLK2,
            true => GENCTRL10R::GCLK3,
            true => GENCTRL10R::GCLK4,
            true => GENCTRL10R::GCLK5,
            true => GENCTRL10R::GCLK6,
            true => GENCTRL10R::GCLK7,
            true => GENCTRL10R::GCLK8,
            true => GENCTRL10R::GCLK9,
            true => GENCTRL10R::GCLK10,
            true => GENCTRL10R::GCLK11,
            i => GENCTRL10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL10R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL10R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL10R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL10R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL10R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL10R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL10R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL10R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL10R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL10R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL10R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL10R::GCLK11
    }
}
#[doc = "Possible values of the field `GENCTRL11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRL11R {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GENCTRL11R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GENCTRL11R::GCLK0 => true,
            GENCTRL11R::GCLK1 => true,
            GENCTRL11R::GCLK2 => true,
            GENCTRL11R::GCLK3 => true,
            GENCTRL11R::GCLK4 => true,
            GENCTRL11R::GCLK5 => true,
            GENCTRL11R::GCLK6 => true,
            GENCTRL11R::GCLK7 => true,
            GENCTRL11R::GCLK8 => true,
            GENCTRL11R::GCLK9 => true,
            GENCTRL11R::GCLK10 => true,
            GENCTRL11R::GCLK11 => true,
            GENCTRL11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENCTRL11R {
        match value {
            true => GENCTRL11R::GCLK0,
            true => GENCTRL11R::GCLK1,
            true => GENCTRL11R::GCLK2,
            true => GENCTRL11R::GCLK3,
            true => GENCTRL11R::GCLK4,
            true => GENCTRL11R::GCLK5,
            true => GENCTRL11R::GCLK6,
            true => GENCTRL11R::GCLK7,
            true => GENCTRL11R::GCLK8,
            true => GENCTRL11R::GCLK9,
            true => GENCTRL11R::GCLK10,
            true => GENCTRL11R::GCLK11,
            i => GENCTRL11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRL11R::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRL11R::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRL11R::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRL11R::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRL11R::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRL11R::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRL11R::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRL11R::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRL11R::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRL11R::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRL11R::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRL11R::GCLK11
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRSTR { bits }
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl0(&self) -> GENCTRL0R {
        GENCTRL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl1(&self) -> GENCTRL1R {
        GENCTRL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl2(&self) -> GENCTRL2R {
        GENCTRL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl3(&self) -> GENCTRL3R {
        GENCTRL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl4(&self) -> GENCTRL4R {
        GENCTRL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl5(&self) -> GENCTRL5R {
        GENCTRL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl6(&self) -> GENCTRL6R {
        GENCTRL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl7(&self) -> GENCTRL7R {
        GENCTRL7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl8(&self) -> GENCTRL8R {
        GENCTRL8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Generic Clock Generator Control 9 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl9(&self) -> GENCTRL9R {
        GENCTRL9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Generic Clock Generator Control 10 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl10(&self) -> GENCTRL10R {
        GENCTRL10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Generic Clock Generator Control 11 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl11(&self) -> GENCTRL11R {
        GENCTRL11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
