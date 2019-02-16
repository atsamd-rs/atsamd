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
    _Reserved(u16),
}
impl GENCTRL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL0R::GCLK0 => 1,
            GENCTRL0R::GCLK1 => 2,
            GENCTRL0R::GCLK2 => 4,
            GENCTRL0R::GCLK3 => 8,
            GENCTRL0R::GCLK4 => 16,
            GENCTRL0R::GCLK5 => 32,
            GENCTRL0R::GCLK6 => 64,
            GENCTRL0R::GCLK7 => 128,
            GENCTRL0R::GCLK8 => 256,
            GENCTRL0R::GCLK9 => 512,
            GENCTRL0R::GCLK10 => 1024,
            GENCTRL0R::GCLK11 => 2048,
            GENCTRL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL0R {
        match value {
            1 => GENCTRL0R::GCLK0,
            2 => GENCTRL0R::GCLK1,
            4 => GENCTRL0R::GCLK2,
            8 => GENCTRL0R::GCLK3,
            16 => GENCTRL0R::GCLK4,
            32 => GENCTRL0R::GCLK5,
            64 => GENCTRL0R::GCLK6,
            128 => GENCTRL0R::GCLK7,
            256 => GENCTRL0R::GCLK8,
            512 => GENCTRL0R::GCLK9,
            1024 => GENCTRL0R::GCLK10,
            2048 => GENCTRL0R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL1R::GCLK0 => 1,
            GENCTRL1R::GCLK1 => 2,
            GENCTRL1R::GCLK2 => 4,
            GENCTRL1R::GCLK3 => 8,
            GENCTRL1R::GCLK4 => 16,
            GENCTRL1R::GCLK5 => 32,
            GENCTRL1R::GCLK6 => 64,
            GENCTRL1R::GCLK7 => 128,
            GENCTRL1R::GCLK8 => 256,
            GENCTRL1R::GCLK9 => 512,
            GENCTRL1R::GCLK10 => 1024,
            GENCTRL1R::GCLK11 => 2048,
            GENCTRL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL1R {
        match value {
            1 => GENCTRL1R::GCLK0,
            2 => GENCTRL1R::GCLK1,
            4 => GENCTRL1R::GCLK2,
            8 => GENCTRL1R::GCLK3,
            16 => GENCTRL1R::GCLK4,
            32 => GENCTRL1R::GCLK5,
            64 => GENCTRL1R::GCLK6,
            128 => GENCTRL1R::GCLK7,
            256 => GENCTRL1R::GCLK8,
            512 => GENCTRL1R::GCLK9,
            1024 => GENCTRL1R::GCLK10,
            2048 => GENCTRL1R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL2R::GCLK0 => 1,
            GENCTRL2R::GCLK1 => 2,
            GENCTRL2R::GCLK2 => 4,
            GENCTRL2R::GCLK3 => 8,
            GENCTRL2R::GCLK4 => 16,
            GENCTRL2R::GCLK5 => 32,
            GENCTRL2R::GCLK6 => 64,
            GENCTRL2R::GCLK7 => 128,
            GENCTRL2R::GCLK8 => 256,
            GENCTRL2R::GCLK9 => 512,
            GENCTRL2R::GCLK10 => 1024,
            GENCTRL2R::GCLK11 => 2048,
            GENCTRL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL2R {
        match value {
            1 => GENCTRL2R::GCLK0,
            2 => GENCTRL2R::GCLK1,
            4 => GENCTRL2R::GCLK2,
            8 => GENCTRL2R::GCLK3,
            16 => GENCTRL2R::GCLK4,
            32 => GENCTRL2R::GCLK5,
            64 => GENCTRL2R::GCLK6,
            128 => GENCTRL2R::GCLK7,
            256 => GENCTRL2R::GCLK8,
            512 => GENCTRL2R::GCLK9,
            1024 => GENCTRL2R::GCLK10,
            2048 => GENCTRL2R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL3R::GCLK0 => 1,
            GENCTRL3R::GCLK1 => 2,
            GENCTRL3R::GCLK2 => 4,
            GENCTRL3R::GCLK3 => 8,
            GENCTRL3R::GCLK4 => 16,
            GENCTRL3R::GCLK5 => 32,
            GENCTRL3R::GCLK6 => 64,
            GENCTRL3R::GCLK7 => 128,
            GENCTRL3R::GCLK8 => 256,
            GENCTRL3R::GCLK9 => 512,
            GENCTRL3R::GCLK10 => 1024,
            GENCTRL3R::GCLK11 => 2048,
            GENCTRL3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL3R {
        match value {
            1 => GENCTRL3R::GCLK0,
            2 => GENCTRL3R::GCLK1,
            4 => GENCTRL3R::GCLK2,
            8 => GENCTRL3R::GCLK3,
            16 => GENCTRL3R::GCLK4,
            32 => GENCTRL3R::GCLK5,
            64 => GENCTRL3R::GCLK6,
            128 => GENCTRL3R::GCLK7,
            256 => GENCTRL3R::GCLK8,
            512 => GENCTRL3R::GCLK9,
            1024 => GENCTRL3R::GCLK10,
            2048 => GENCTRL3R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL4R::GCLK0 => 1,
            GENCTRL4R::GCLK1 => 2,
            GENCTRL4R::GCLK2 => 4,
            GENCTRL4R::GCLK3 => 8,
            GENCTRL4R::GCLK4 => 16,
            GENCTRL4R::GCLK5 => 32,
            GENCTRL4R::GCLK6 => 64,
            GENCTRL4R::GCLK7 => 128,
            GENCTRL4R::GCLK8 => 256,
            GENCTRL4R::GCLK9 => 512,
            GENCTRL4R::GCLK10 => 1024,
            GENCTRL4R::GCLK11 => 2048,
            GENCTRL4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL4R {
        match value {
            1 => GENCTRL4R::GCLK0,
            2 => GENCTRL4R::GCLK1,
            4 => GENCTRL4R::GCLK2,
            8 => GENCTRL4R::GCLK3,
            16 => GENCTRL4R::GCLK4,
            32 => GENCTRL4R::GCLK5,
            64 => GENCTRL4R::GCLK6,
            128 => GENCTRL4R::GCLK7,
            256 => GENCTRL4R::GCLK8,
            512 => GENCTRL4R::GCLK9,
            1024 => GENCTRL4R::GCLK10,
            2048 => GENCTRL4R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL5R::GCLK0 => 1,
            GENCTRL5R::GCLK1 => 2,
            GENCTRL5R::GCLK2 => 4,
            GENCTRL5R::GCLK3 => 8,
            GENCTRL5R::GCLK4 => 16,
            GENCTRL5R::GCLK5 => 32,
            GENCTRL5R::GCLK6 => 64,
            GENCTRL5R::GCLK7 => 128,
            GENCTRL5R::GCLK8 => 256,
            GENCTRL5R::GCLK9 => 512,
            GENCTRL5R::GCLK10 => 1024,
            GENCTRL5R::GCLK11 => 2048,
            GENCTRL5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL5R {
        match value {
            1 => GENCTRL5R::GCLK0,
            2 => GENCTRL5R::GCLK1,
            4 => GENCTRL5R::GCLK2,
            8 => GENCTRL5R::GCLK3,
            16 => GENCTRL5R::GCLK4,
            32 => GENCTRL5R::GCLK5,
            64 => GENCTRL5R::GCLK6,
            128 => GENCTRL5R::GCLK7,
            256 => GENCTRL5R::GCLK8,
            512 => GENCTRL5R::GCLK9,
            1024 => GENCTRL5R::GCLK10,
            2048 => GENCTRL5R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL6R::GCLK0 => 1,
            GENCTRL6R::GCLK1 => 2,
            GENCTRL6R::GCLK2 => 4,
            GENCTRL6R::GCLK3 => 8,
            GENCTRL6R::GCLK4 => 16,
            GENCTRL6R::GCLK5 => 32,
            GENCTRL6R::GCLK6 => 64,
            GENCTRL6R::GCLK7 => 128,
            GENCTRL6R::GCLK8 => 256,
            GENCTRL6R::GCLK9 => 512,
            GENCTRL6R::GCLK10 => 1024,
            GENCTRL6R::GCLK11 => 2048,
            GENCTRL6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL6R {
        match value {
            1 => GENCTRL6R::GCLK0,
            2 => GENCTRL6R::GCLK1,
            4 => GENCTRL6R::GCLK2,
            8 => GENCTRL6R::GCLK3,
            16 => GENCTRL6R::GCLK4,
            32 => GENCTRL6R::GCLK5,
            64 => GENCTRL6R::GCLK6,
            128 => GENCTRL6R::GCLK7,
            256 => GENCTRL6R::GCLK8,
            512 => GENCTRL6R::GCLK9,
            1024 => GENCTRL6R::GCLK10,
            2048 => GENCTRL6R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL7R::GCLK0 => 1,
            GENCTRL7R::GCLK1 => 2,
            GENCTRL7R::GCLK2 => 4,
            GENCTRL7R::GCLK3 => 8,
            GENCTRL7R::GCLK4 => 16,
            GENCTRL7R::GCLK5 => 32,
            GENCTRL7R::GCLK6 => 64,
            GENCTRL7R::GCLK7 => 128,
            GENCTRL7R::GCLK8 => 256,
            GENCTRL7R::GCLK9 => 512,
            GENCTRL7R::GCLK10 => 1024,
            GENCTRL7R::GCLK11 => 2048,
            GENCTRL7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL7R {
        match value {
            1 => GENCTRL7R::GCLK0,
            2 => GENCTRL7R::GCLK1,
            4 => GENCTRL7R::GCLK2,
            8 => GENCTRL7R::GCLK3,
            16 => GENCTRL7R::GCLK4,
            32 => GENCTRL7R::GCLK5,
            64 => GENCTRL7R::GCLK6,
            128 => GENCTRL7R::GCLK7,
            256 => GENCTRL7R::GCLK8,
            512 => GENCTRL7R::GCLK9,
            1024 => GENCTRL7R::GCLK10,
            2048 => GENCTRL7R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL8R::GCLK0 => 1,
            GENCTRL8R::GCLK1 => 2,
            GENCTRL8R::GCLK2 => 4,
            GENCTRL8R::GCLK3 => 8,
            GENCTRL8R::GCLK4 => 16,
            GENCTRL8R::GCLK5 => 32,
            GENCTRL8R::GCLK6 => 64,
            GENCTRL8R::GCLK7 => 128,
            GENCTRL8R::GCLK8 => 256,
            GENCTRL8R::GCLK9 => 512,
            GENCTRL8R::GCLK10 => 1024,
            GENCTRL8R::GCLK11 => 2048,
            GENCTRL8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL8R {
        match value {
            1 => GENCTRL8R::GCLK0,
            2 => GENCTRL8R::GCLK1,
            4 => GENCTRL8R::GCLK2,
            8 => GENCTRL8R::GCLK3,
            16 => GENCTRL8R::GCLK4,
            32 => GENCTRL8R::GCLK5,
            64 => GENCTRL8R::GCLK6,
            128 => GENCTRL8R::GCLK7,
            256 => GENCTRL8R::GCLK8,
            512 => GENCTRL8R::GCLK9,
            1024 => GENCTRL8R::GCLK10,
            2048 => GENCTRL8R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL9R::GCLK0 => 1,
            GENCTRL9R::GCLK1 => 2,
            GENCTRL9R::GCLK2 => 4,
            GENCTRL9R::GCLK3 => 8,
            GENCTRL9R::GCLK4 => 16,
            GENCTRL9R::GCLK5 => 32,
            GENCTRL9R::GCLK6 => 64,
            GENCTRL9R::GCLK7 => 128,
            GENCTRL9R::GCLK8 => 256,
            GENCTRL9R::GCLK9 => 512,
            GENCTRL9R::GCLK10 => 1024,
            GENCTRL9R::GCLK11 => 2048,
            GENCTRL9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL9R {
        match value {
            1 => GENCTRL9R::GCLK0,
            2 => GENCTRL9R::GCLK1,
            4 => GENCTRL9R::GCLK2,
            8 => GENCTRL9R::GCLK3,
            16 => GENCTRL9R::GCLK4,
            32 => GENCTRL9R::GCLK5,
            64 => GENCTRL9R::GCLK6,
            128 => GENCTRL9R::GCLK7,
            256 => GENCTRL9R::GCLK8,
            512 => GENCTRL9R::GCLK9,
            1024 => GENCTRL9R::GCLK10,
            2048 => GENCTRL9R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL10R::GCLK0 => 1,
            GENCTRL10R::GCLK1 => 2,
            GENCTRL10R::GCLK2 => 4,
            GENCTRL10R::GCLK3 => 8,
            GENCTRL10R::GCLK4 => 16,
            GENCTRL10R::GCLK5 => 32,
            GENCTRL10R::GCLK6 => 64,
            GENCTRL10R::GCLK7 => 128,
            GENCTRL10R::GCLK8 => 256,
            GENCTRL10R::GCLK9 => 512,
            GENCTRL10R::GCLK10 => 1024,
            GENCTRL10R::GCLK11 => 2048,
            GENCTRL10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL10R {
        match value {
            1 => GENCTRL10R::GCLK0,
            2 => GENCTRL10R::GCLK1,
            4 => GENCTRL10R::GCLK2,
            8 => GENCTRL10R::GCLK3,
            16 => GENCTRL10R::GCLK4,
            32 => GENCTRL10R::GCLK5,
            64 => GENCTRL10R::GCLK6,
            128 => GENCTRL10R::GCLK7,
            256 => GENCTRL10R::GCLK8,
            512 => GENCTRL10R::GCLK9,
            1024 => GENCTRL10R::GCLK10,
            2048 => GENCTRL10R::GCLK11,
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
    _Reserved(u16),
}
impl GENCTRL11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRL11R::GCLK0 => 1,
            GENCTRL11R::GCLK1 => 2,
            GENCTRL11R::GCLK2 => 4,
            GENCTRL11R::GCLK3 => 8,
            GENCTRL11R::GCLK4 => 16,
            GENCTRL11R::GCLK5 => 32,
            GENCTRL11R::GCLK6 => 64,
            GENCTRL11R::GCLK7 => 128,
            GENCTRL11R::GCLK8 => 256,
            GENCTRL11R::GCLK9 => 512,
            GENCTRL11R::GCLK10 => 1024,
            GENCTRL11R::GCLK11 => 2048,
            GENCTRL11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRL11R {
        match value {
            1 => GENCTRL11R::GCLK0,
            2 => GENCTRL11R::GCLK1,
            4 => GENCTRL11R::GCLK2,
            8 => GENCTRL11R::GCLK3,
            16 => GENCTRL11R::GCLK4,
            32 => GENCTRL11R::GCLK5,
            64 => GENCTRL11R::GCLK6,
            128 => GENCTRL11R::GCLK7,
            256 => GENCTRL11R::GCLK8,
            512 => GENCTRL11R::GCLK9,
            1024 => GENCTRL11R::GCLK10,
            2048 => GENCTRL11R::GCLK11,
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
    #[doc = "Bits 2:13 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl0(&self) -> GENCTRL0R {
        GENCTRL0R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 3:14 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl1(&self) -> GENCTRL1R {
        GENCTRL1R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 4:15 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl2(&self) -> GENCTRL2R {
        GENCTRL2R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 5:16 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl3(&self) -> GENCTRL3R {
        GENCTRL3R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 6:17 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl4(&self) -> GENCTRL4R {
        GENCTRL4R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 7:18 - Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl5(&self) -> GENCTRL5R {
        GENCTRL5R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 8:19 - Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl6(&self) -> GENCTRL6R {
        GENCTRL6R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 9:20 - Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl7(&self) -> GENCTRL7R {
        GENCTRL7R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 10:21 - Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl8(&self) -> GENCTRL8R {
        GENCTRL8R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 11:22 - Generic Clock Generator Control 9 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl9(&self) -> GENCTRL9R {
        GENCTRL9R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 12:23 - Generic Clock Generator Control 10 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl10(&self) -> GENCTRL10R {
        GENCTRL10R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 13:24 - Generic Clock Generator Control 11 Synchronization Busy bits"]
    #[inline]
    pub fn genctrl11(&self) -> GENCTRL11R {
        GENCTRL11R::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
