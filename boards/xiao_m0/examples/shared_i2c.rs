#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;

use hal::{clock::GenericClockController, delay::Delay, prelude::*, time::KiloHertz};
use mpu6050::Mpu6050;
use pac::{CorePeripherals, Peripherals};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use bsp::{entry, hal, pac};
use xiao_m0 as bsp;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let i2c = bsp::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.a4,
        pins.a5,
    );
    let i2c_bus = shared_bus::BusManagerSimple::new(i2c);

    let interface = I2CDisplayInterface::new(i2c_bus.acquire_i2c());
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180).into_terminal_mode();
    display.init().unwrap();

    let mut mpu = Mpu6050::new(i2c_bus.acquire_i2c());
    mpu.init(&mut delay).unwrap();

    loop {
        display.clear().unwrap();

        let acc = mpu.get_acc().unwrap();
        display
            .write_fmt(format_args!("ax={}\nay={}\naz={}\n", acc.x, acc.y, acc.z))
            .unwrap();

        delay.delay_ms(1_000u32);
    }
}
