use super::hal;

use cortex_m::asm::delay as cycle_delay;
use gpio::{Floating, Input, Output, PushPull};
use hal::gpio::{self, *};
use hal::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Keys {
    SelectDown,
    SelectUp,
    StartDown,
    StartUp,
    BDown,
    BUp,
    ADown,
    AUp,

    UpUp,
    UpDown,
    DownUp,
    DownDown,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

pub struct ButtonIter {
    pub pressed: u8,
    pub released: u8,
    pub bit_index: u8,
}

//should be impossible for released and pressed, but gives released preference
fn mask_to_event(mask: u8, released: u8, pressed: u8) -> Option<Keys> {
    let pressed_bool = mask & pressed == mask;
    let released_bool = mask & released == mask;

    match mask {
        0x80 => {
            if released_bool {
                Some(Keys::BUp)
            } else if pressed_bool {
                Some(Keys::BDown)
            } else {
                None
            }
        }
        0x40 => {
            if released_bool {
                Some(Keys::AUp)
            } else if pressed_bool {
                Some(Keys::ADown)
            } else {
                None
            }
        }
        0x20 => {
            if released_bool {
                Some(Keys::StartUp)
            } else if pressed_bool {
                Some(Keys::StartDown)
            } else {
                None
            }
        }
        0x10 => {
            if released_bool {
                Some(Keys::SelectUp)
            } else if pressed_bool {
                Some(Keys::SelectDown)
            } else {
                None
            }
        }

        0x08 => {
            if released_bool {
                Some(Keys::RightUp)
            } else if pressed_bool {
                Some(Keys::RightDown)
            } else {
                None
            }
        }
        0x04 => {
            if released_bool {
                Some(Keys::DownUp)
            } else if pressed_bool {
                Some(Keys::DownDown)
            } else {
                None
            }
        }
        0x02 => {
            if released_bool {
                Some(Keys::UpUp)
            } else if pressed_bool {
                Some(Keys::UpDown)
            } else {
                None
            }
        }
        0x01 => {
            if released_bool {
                Some(Keys::LeftUp)
            } else if pressed_bool {
                Some(Keys::LeftDown)
            } else {
                None
            }
        }
        _ => None,
    }
}

impl Iterator for ButtonIter {
    type Item = Keys;

    fn next(&mut self) -> Option<Keys> {
        //really want a while post increment but doesnt exist
        if self.bit_index >= 8 {
            return None;
        }

        //funky do while
        while {
            let mask = 0x01 << self.bit_index;
            self.bit_index += 1;

            let event = mask_to_event(mask, self.released, self.pressed);
            if event.is_some() {
                return event;
            }

            self.bit_index < 8
        } {}

        None
    }
}

/// Button pins
pub struct ButtonReader {
    /// Button Latch
    pub latch: Pb0<Output<PushPull>>,
    /// Button Out
    pub data_in: Pb30<Input<Floating>>,
    /// Button Clock
    pub clock: Pb31<Output<PushPull>>,
    pub last: u8,
}

//120mhz, 1 cycle = 0.000000008333333 = 8.333333ns
//https://www.onsemi.com/pub/Collateral/MC74HC165A-D.PDF
//3v <=125c
//tsu min setup time 55ns = 7 cycles
//th min hold time 5ns = 1 cycles
//tw min pulse width 36ns = 5 cycles
//trec min recovery time 55ns, how long before you should attempt to read
// again?
impl ButtonReader {
    // 48*8.333ns total blocking read
    /// Returns a ButtonIter of button changes as Keys enums
    pub fn events(&mut self) -> ButtonIter {
        self.latch.set_low().ok();
        cycle_delay(7); //tsu?
        self.latch.set_high().ok();
        cycle_delay(1); //th?

        let mut current: u8 = 0;

        for _i in 0..8 {
            current <<= 1;

            self.clock.set_low().ok();
            cycle_delay(5); //tw

            if self.data_in.is_high().unwrap() {
                current |= 1;
            }
            self.clock.set_high().ok();
        }

        let iter = ButtonIter {
            pressed: (self.last ^ current) & current,
            released: (self.last ^ current) & self.last,
            bit_index: 0,
        };

        self.last = current;

        iter
    }
}
