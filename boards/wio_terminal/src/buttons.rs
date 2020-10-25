use atsamd_hal::clock::GenericClockController;
use atsamd_hal::common::eic;
use atsamd_hal::common::eic::pin::*;
use atsamd_hal::gpio::*;
use atsamd_hal::target_device::{interrupt, EIC, MCLK};

use cortex_m::peripheral::NVIC;

/// pushbuttons and joystick
pub struct ButtonPins {
    /// button1 pin
    pub button1: Pc26<Input<Floating>>,
    /// button2 pin
    pub button2: Pc27<Input<Floating>>,
    /// button3 pin
    pub button3: Pc28<Input<Floating>>,

    /// Joystick X
    pub switch_x: Pd8<Input<Floating>>,
    /// Joystick Y
    pub switch_y: Pd9<Input<Floating>>,
    /// Joystick Z
    pub switch_z: Pd10<Input<Floating>>,
    /// Joystick U
    pub switch_u: Pd20<Input<Floating>>,
    /// Joystick B
    pub switch_b: Pd12<Input<Floating>>,
}

impl ButtonPins {
    pub fn init(
        self,
        eic: EIC,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> ButtonController {
        let clk = clocks.gclk1();
        let mut eic = eic::init_with_ulp32k(mclk, clocks.eic(&clk).unwrap(), eic);
        eic.button_debounce_pins(&[
            self.button1.id(),
            self.button2.id(),
            self.button3.id(),
            self.switch_x.id(),
            self.switch_y.id(),
            self.switch_z.id(),
            self.switch_u.id(),
            self.switch_b.id(),
        ]);

        // Unfortunately, the pin assigned to B1 shares the same
        // ExtInt line as up on the joystick. As such, we don't
        // support B1.

        // let mut b1 = self.button1.into_ei(port);
        let mut b2 = self.button2.into_ei(port);
        let mut b3 = self.button3.into_ei(port);
        let mut x = self.switch_x.into_ei(port);
        let mut y = self.switch_y.into_ei(port);
        let mut z = self.switch_z.into_ei(port);
        let mut u = self.switch_u.into_ei(port);
        let mut b = self.switch_b.into_ei(port);

        // b1.sense(&mut eic, Sense::BOTH);
        b2.sense(&mut eic, Sense::BOTH);
        b3.sense(&mut eic, Sense::BOTH);
        x.sense(&mut eic, Sense::BOTH);
        y.sense(&mut eic, Sense::BOTH);
        z.sense(&mut eic, Sense::BOTH);
        u.sense(&mut eic, Sense::BOTH);
        b.sense(&mut eic, Sense::BOTH);

        // b1.enable_interrupt(&mut eic);
        b2.enable_interrupt(&mut eic);
        b3.enable_interrupt(&mut eic);
        x.enable_interrupt(&mut eic);
        y.enable_interrupt(&mut eic);
        z.enable_interrupt(&mut eic);
        u.enable_interrupt(&mut eic);
        b.enable_interrupt(&mut eic);

        ButtonController {
            _eic: eic.finalize(),
            // b1,
            b2,
            b3,
            x,
            y,
            z,
            u,
            b,
        }
    }
}

pub enum Button {
    TopLeft,
    TopMiddle,
    // TopRight,
    Down,
    Up,
    Left,
    Right,
    Click,
}

pub struct ButtonEvent {
    pub button: Button,
    pub down: bool,
}

pub struct ButtonController {
    _eic: eic::EIC,
    // b1: ExtInt10<Pc26<PfA>>,
    b2: ExtInt11<Pc27<PfA>>,
    b3: ExtInt12<Pc28<PfA>>,

    x: ExtInt3<Pd8<PfA>>,
    y: ExtInt4<Pd9<PfA>>,
    z: ExtInt5<Pd10<PfA>>,
    u: ExtInt10<Pd20<PfA>>,
    b: ExtInt7<Pd12<PfA>>,
}

macro_rules! isr {
    ($Handler:ident, $($Event:expr, $Button:ident),+) => {
        pub fn $Handler(&mut self) -> Option<ButtonEvent> {
            $(
                {
                    let b = &mut self.$Button;
                    if b.is_interrupt() {
                        b.clear_interrupt();
                        return Some(ButtonEvent {
                            button: $Event,
                            down: !b.state(),
                        })
                    }
                }
            )+

            None
        }
    };
}

impl ButtonController {
    pub fn enable(&self, nvic: &mut NVIC) {
        unsafe {
            nvic.set_priority(interrupt::EIC_EXTINT_10, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_10);
            nvic.set_priority(interrupt::EIC_EXTINT_11, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_11);
            nvic.set_priority(interrupt::EIC_EXTINT_12, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_12);
            nvic.set_priority(interrupt::EIC_EXTINT_3, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_3);
            nvic.set_priority(interrupt::EIC_EXTINT_4, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_4);
            nvic.set_priority(interrupt::EIC_EXTINT_5, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_5);
            nvic.set_priority(interrupt::EIC_EXTINT_7, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_7);
        }
    }

    isr!(interrupt_extint3, Button::Down, x);
    isr!(interrupt_extint4, Button::Right, y);
    isr!(interrupt_extint5, Button::Click, z);
    isr!(interrupt_extint7, Button::Left, b);
    isr!(interrupt_extint10, Button::Up, u);
    // isr!(interrupt_extint10, Button::TopRight, b1);
    isr!(interrupt_extint11, Button::TopMiddle, b2);
    isr!(interrupt_extint12, Button::TopLeft, b3);
}

#[macro_export]
macro_rules! button_interrupt {
    ($controller:ident, unsafe fn $func_name:ident ($cs:ident: $cstype:ty, $event:ident: ButtonEvent ) $code:block) => {
        unsafe fn $func_name($cs: $cstype, $event: ButtonEvent) {
            $code
        }

        macro_rules! _button_interrupt_handler {
            ($Interrupt:ident, $Handler:ident) => {
                #[interrupt]
                fn $Interrupt() {
                    disable_interrupts(|cs| unsafe {
                        $controller.as_mut().map(|ctrlr| {
                            if let Some(event) = ctrlr.$Handler() {
                                $func_name(cs, event);
                            }
                        });
                    });
                }
            };
        }

        _button_interrupt_handler!(EIC_EXTINT_3, interrupt_extint3);
        _button_interrupt_handler!(EIC_EXTINT_4, interrupt_extint4);
        _button_interrupt_handler!(EIC_EXTINT_5, interrupt_extint5);
        _button_interrupt_handler!(EIC_EXTINT_7, interrupt_extint7);
        _button_interrupt_handler!(EIC_EXTINT_10, interrupt_extint10);
        _button_interrupt_handler!(EIC_EXTINT_11, interrupt_extint11);
        _button_interrupt_handler!(EIC_EXTINT_12, interrupt_extint12);
    };
}
